use crate::chain_reader::{ChainReader,BlockWindow};
use crate::dag_graph::{DagEdge, DagNode};
use serde::Serialize;
use std::sync::Arc;
#[derive(Debug, Serialize)]
pub struct DagGraph {
    pub nodes: Vec<DagNode>,
    pub edges: Vec<DagEdge>,
}

#[derive(Clone)]
pub struct DagGraphBuilder {
    reader: Arc<dyn ChainReader>,
    window: BlockWindow,
}
impl DagGraphBuilder {
    pub fn new(reader: Arc<dyn ChainReader>, window: BlockWindow) -> Self {
        Self { reader, window }
    }

    pub async fn generate(&self) -> anyhow::Result<DagGraph> {
        let provider = self.reader.dag_view(self.window).await?;
        let nodes = provider.dag_nodes();
        let edges = provider.dag_edges();
        Ok(DagGraph { nodes, edges })
    }    
}
