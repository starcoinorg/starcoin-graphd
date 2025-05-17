use serde::Serialize;
use starcoin_crypto::HashValue;

#[derive(Debug, Serialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum NodeColor {
    Blue,
    Red,
    Unknown,
}

pub trait DagGraphProvider {
    fn dag_nodes(&self) -> Vec<DagNode>;
    fn dag_edges(&self) -> Vec<DagEdge>;
}

mod graph;
pub use graph::DagGraphBuilder;
