use serde::Serialize;
use starcoin_crypto::HashValue;
use starcoin_types::block::BlockNumber;
use crate::chain_reader::ChainReader;
use std::collections::{HashMap, HashSet};
#[derive(Debug, Serialize)]
pub struct DagGraph {
    pub nodes: Vec<DagNode>,
    pub edges: Vec<DagEdge>,
}

#[derive(Debug, Serialize)]
pub struct DagNode {
    pub id: HashValue,
    pub score: u64,
    pub color: NodeColor,
}

#[derive(Debug, Serialize)]
pub struct DagEdge {
    pub from: HashValue,
    pub to: HashValue,
    pub is_selected: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeColor {
    Blue,
    Red,
    SelectedChain,
}



impl DagGraph {
    pub async fn build<R: ChainReader>(
        reader: &R,
        anchor_number: Option<BlockNumber>,
        count: u64,
    ) -> anyhow::Result<Self> {
        // Fetch the selected chain blocks (main chain).
        let selected_chain = reader
            .get_selected_chain(anchor_number, count)
            .await?;

        let selected_ids: HashSet<HashValue> =
            selected_chain.iter().map(|h| h.id()).collect();

        // Fetch GHOSTDAG data for each block in the selected chain.
        let mut ghostdag_map = HashMap::new();
        for header in &selected_chain {
            let hash = header.id();
            if let Some(gd) = reader.get_ghostdag_data(hash).await? {
                ghostdag_map.insert(hash, gd);
            }
        }

        // Collect all involved block IDs: selected + merge set blocks.
        let mut all_ids = HashSet::new();
        for header in &selected_chain {
            let hash = header.id();
            all_ids.insert(hash);

            if let Some(gd) = ghostdag_map.get(&hash) {
                all_ids.insert(gd.selected_parent);
                all_ids.extend(&gd.mergeset_blues);
                all_ids.extend(&gd.mergeset_reds);
            }
        }

        // Fetch headers for all required blocks (to get .parents()).
        let all_headers = reader
            .get_headers(all_ids.iter().cloned().collect())
            .await?;

        let header_map: HashMap<_, _> =
            all_headers.into_iter().map(|h| (h.id(), h)).collect();

        // Build DAG nodes.
        let mut nodes = Vec::new();
        for id in &all_ids {
            let color = if selected_ids.contains(id) {
                NodeColor::Blue
            } else if ghostdag_map.values().any(|g| g.mergeset_blues.contains(id)) {
                NodeColor::Blue
            } else if ghostdag_map.values().any(|g| g.mergeset_reds.contains(id)) {
                NodeColor::Red
            } else {
                continue; // Skip uncolored node (should not happen)
            };

            let score = header_map.get(id).map_or(0, |h| h.number());

            nodes.push(DagNode {
                id: *id,
                score,
                color,
            });
        }

        // Step 6: Build DAG edges.
        let mut edges = Vec::new();
        for (id, header) in &header_map {
            for parent in header.parents_hash() {
                let is_selected = ghostdag_map
                    .get(id)
                    .map(|gd| gd.selected_parent == parent)
                    .unwrap_or(false);

                edges.push(DagEdge {
                    from: parent,
                    to: *id,
                    is_selected,
                });
            }
        }

        Ok(DagGraph { nodes, edges })
    }
}
