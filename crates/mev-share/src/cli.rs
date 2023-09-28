use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

mod args;
pub use args::*;

pub async fn run() -> anyhow::Result<()> {
    // init tracing
    let filter = filter::Targets::new().with_target("mev_share", Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    // cli arguments
    let args = Args::parse();

    info!("running mev-share with arguments \n{:#?}", args);

    Ok(())
}
