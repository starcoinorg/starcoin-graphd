use crate::chain_reader::ChainReader;
use crate::dag_graph::{DagEdge, DagGraphProvider, DagNode, NodeColor};
use async_trait::async_trait;
use starcoin_crypto::HashValue;
use std::collections::{HashMap, HashSet, VecDeque};

use super::BlockWindow;

#[async_trait]
pub trait ChainReaderExt: Send + Sync {
    // get selected chain by blue score number-count to number
    async fn get_selected_chain(
        &self,
        number: Option<u64>,
        count: u64,
    ) -> anyhow::Result<Vec<BlockHeader>>;

    async fn get_ghostdag_data(
        &self,
        ids: &[HashValue],
    ) -> anyhow::Result<Vec<Option<GhostdagData>>>;

    async fn get_headers(&self, ids: &[HashValue]) -> anyhow::Result<Vec<BlockHeader>>;
}

#[derive(Clone, Debug)]
pub struct GhostdagData {
    pub blue_score: u64,
    pub selected_parent: HashValue,
    pub mergeset_blues: Vec<HashValue>,
    pub mergeset_reds: Vec<HashValue>,
}

#[derive(Clone, Debug)]
pub struct BlockHeader {
    pub id: HashValue,
    pub number: u64,
    pub parents_hash: Vec<HashValue>,
}

impl BlockHeader {
    pub fn id(&self) -> HashValue {
        self.id
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn parents_hash(&self) -> Vec<HashValue> {
        self.parents_hash.clone()
    }
}

pub struct DagBuildContext {
    pub header_map: HashMap<HashValue, BlockHeader>,
    pub ghostdag_map: HashMap<HashValue, GhostdagData>,
}

impl DagGraphProvider for DagBuildContext {
    fn dag_nodes(&self) -> Vec<DagNode> {
        let mut color_map = HashMap::new();
        for gd in self.ghostdag_map.values() {
            for &b in &gd.mergeset_blues {
                color_map.insert(b, NodeColor::Blue);
            }
            for &r in &gd.mergeset_reds {
                color_map.insert(r, NodeColor::Red);
            }
        }

        self.header_map
            .iter()
            .map(|(id, header)| {
                let color = color_map.get(id).copied().unwrap_or(NodeColor::Unknown);
                DagNode {
                    id: *id,
                    score: header.number(),
                    color,
                }
            })
            .collect()
    }
    fn dag_edges(&self) -> Vec<DagEdge> {
        let mut edges = Vec::new();

        for (id, header) in &self.header_map {
            let selected_parent = self.ghostdag_map.get(id).map(|gd| gd.selected_parent);

            for parent in header.parents_hash() {
                edges.push(DagEdge {
                    from: parent,
                    to: *id,
                    is_selected: Some(parent) == selected_parent,
                });
            }
        }

        edges
    }
}

#[async_trait]
impl<T> ChainReader for T
where
    T: ChainReaderExt + Sync,
{
    async fn dag_view(
        &self,
        window: BlockWindow,
    ) -> anyhow::Result<Box<dyn DagGraphProvider + Send + Sync>> {
        // Only ghostdag data for blocks on the selected chain are required.
        //
        // According to the GHOSTDAG protocol, each selected chain block contains
        // authoritative information about its mergeset (i.e., which blocks are blue or red).
        //
        // These selected blocks form a topological path from genesis to tip, and their
        // ghostdag data are constructed recursively via their selected parents.
        //
        // Mergeset blocks (blue or red) are referenced by selected blocks, but do not
        // require their own ghostdag data for DAG visualization or color inference.
        //
        // Therefore, to build a complete and colored DAG view, we only need:
        //   1. The headers of all selected + mergeset blocks
        //   2. The ghostdag data of the selected chain blocks only

        let selected_chain = {
            match window {
                BlockWindow::Latest(n) => self.get_selected_chain(None, n).await?,
                _ => unimplemented!(),
            }
        };
        let mut header_map: HashMap<_, _> =
            selected_chain.iter().map(|h| (h.id(), h.clone())).collect();
        let selected_ids: Vec<_> = header_map.keys().copied().collect();

        let ghostdag_vec = self.get_ghostdag_data(&selected_ids).await?;
        let mut ghostdag_map = HashMap::new();
        let mut seen: HashSet<_> = header_map.keys().copied().collect();
        let mut to_fetch = VecDeque::new();

        for (i, opt) in ghostdag_vec.into_iter().enumerate() {
            if let Some(gd) = opt {
                let id = selected_ids[i];
                ghostdag_map.insert(id, gd.clone());
                for &h in gd.mergeset_blues.iter().chain(&gd.mergeset_reds) {
                    if seen.insert(h) {
                        to_fetch.push_back(h);
                    }
                }
            }
        }

        // Collect all needed block hashes
        let mut missing = Vec::new();
        while let Some(hash) = to_fetch.pop_front() {
            if !header_map.contains_key(&hash) {
                missing.push(hash);
            }
        }

        // Fetch all missing headers in batch
        let mut pending: HashSet<HashValue> = missing.iter().copied().collect();
        while !pending.is_empty() {
            let batch: Vec<HashValue> = pending.iter().copied().collect();
            let new_headers = self.get_headers(&batch).await?;
            for header in new_headers {
                let id = header.id();
                pending.remove(&id);
                for p in header.parents_hash() {
                    if seen.insert(p) {
                        pending.insert(p);
                    }
                }
                header_map.insert(id, header);
            }
        }

        Ok(Box::new(DagBuildContext {
            header_map,
            ghostdag_map,
        }))
    }
}
