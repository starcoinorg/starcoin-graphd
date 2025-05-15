use crate::dag_graph::DagGraphProvider;
use async_trait::async_trait;

#[async_trait]
pub trait ChainReader {
    async fn dag_view(
        &self,
        anchor: Option<u64>,
        count: u64,
    ) -> anyhow::Result<Box<dyn DagGraphProvider + Send + Sync>>;
}

pub mod ext;
pub mod mock;
