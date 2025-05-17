use anyhow::Result;
use clap::Parser;
use starcoin_graphd::prelude::*;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(
    name = "starcoin-graphd",
    author,
    version,
    about = "Serve DAG view and API"
)]
struct Cli {
    /// HTTP listen endpoint, e.g. 127.0.0.1:8080
    #[arg(long, default_value = "127.0.0.1:8080")]
    listen: String,

    /// Network to connect to: halley, vega, or custom
    #[arg(long, default_value = "halley", value_parser = ["halley", "vega", "custom"])]
    network: String,

    /// Custom RPC URL (only used if --network=custom)
    #[arg(long)]
    rpc_url: Option<String>,
}

fn resolve_rpc_url(cli: &Cli) -> Result<String> {
    match cli.network.as_str() {
        "halley" => Ok("http://halley.seed.starcoin.org".to_string()),
        "vega" => Ok("http://vega.seed.starcoin.org".to_string()),
        "custom" => cli
            .rpc_url
            .clone()
            .ok_or_else(|| anyhow::anyhow!("--rpc-url must be set when --network=custom")),
        other => Err(anyhow::anyhow!("Unsupported network: {}", other)),
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let rpc_url = resolve_rpc_url(&cli)?;
    let reader: Arc<dyn ChainReader> = Arc::new(RpcChainReader::new(&rpc_url));
    let builder = DagGraphBuilder::new(reader, BlockWindow::Latest(20));
    start_server(builder, &cli.listen).await
}
