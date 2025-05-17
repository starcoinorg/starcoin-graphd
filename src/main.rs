use anyhow::Result;
use starcoin_graphd::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let reader: Arc<dyn ChainReader> = Arc::new(RpcChainReader::new("http://127.0.0.1:32824"));
    let builder = DagGraphBuilder::new(reader, BlockWindow::Latest(20));
    start_server(builder).await
}
