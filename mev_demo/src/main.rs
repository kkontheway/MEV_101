//Set up engine
//Set up Collector
//Set up Strategy
//Set up Executor
//Start engine
use std::sync::Arc;

use anyhow::{Ok, Result};
use artemis_core::{
    collectors::mevshare_collector::MevShareCollector,
    engine::Engine,
    executors::mev_share_executor::MevshareExecutor,
    types::{CollectorMap, ExecutorMap},
};
use clap::Parser;
use ethers::{
    prelude::MiddlewareBuilder,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer},
    types::Address,
};
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub wss: String,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    //  Set up providers and signers.
    let ws = Ws::connect(args.wss).await?;
    let provider = Provider::new(ws);
    // Set up engine.
    let mut engine: Engine<(), ()> = Engine::default();

    // Set up collector.
    let mevshare_collector = Box::new(MevShareCollector::new(String::from(
        "https://mev-share.flashbots.net",
    )));
    let mevshare_collector = CollectorMap::new(mevshare_collector, Event::MEVShareEvent);
    engine.add_collector(Box::new(mevshare_collector));
    Ok(())
}
