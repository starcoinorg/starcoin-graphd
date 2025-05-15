use async_trait::async_trait;
use crate::dag_graph::DagGraphProvider;

#[async_trait]
pub trait ChainReader {
    async fn dag_view(
        &self,
        anchor: Option<u64>,
        count: u64,
    ) -> anyhow::Result<Box<dyn DagGraphProvider + Send + Sync>>;
}

pub mod mock;
pub mod ext;
