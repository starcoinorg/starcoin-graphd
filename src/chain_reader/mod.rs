use crate::dag_graph::DagGraphProvider;
use async_trait::async_trait;

#[derive(Clone,Copy)]
pub enum BlockWindow{
    Latest(u64),
    Between{from:u64, count:u64}
}

#[async_trait]
pub trait ChainReader:Send+Sync {
    async fn dag_view(
        &self,
	window: BlockWindow
    ) -> anyhow::Result<Box<dyn DagGraphProvider + Send + Sync>>;
}

pub mod ext;
pub mod mock;
pub mod rpc;
