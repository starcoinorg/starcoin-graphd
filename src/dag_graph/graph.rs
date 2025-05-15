use crate::chain_reader::ChainReader;
use crate::dag_graph::{DagEdge, DagGraphProvider, DagNode};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct DagGraph {
    pub nodes: Vec<DagNode>,
    pub edges: Vec<DagEdge>,
}
impl DagGraph {
    fn from_provider<P: DagGraphProvider + ?Sized>(provider: &P) -> Self {
        Self {
            nodes: provider.dag_nodes(),
            edges: provider.dag_edges(),
        }
    }

    pub async fn build<R>(reader: &R, anchor: Option<u64>, count: u64) -> anyhow::Result<Self>
    where
        R: ChainReader + Sync,
    {
        let boxed = reader.dag_view(anchor, count).await?;
        Ok(Self::from_provider(boxed.as_ref()))
    }
}
