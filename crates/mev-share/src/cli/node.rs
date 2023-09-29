use super::NodeCommand;
use tracing::info;

pub async fn run(args: NodeCommand) -> anyhow::Result<()> {
    // connect to redis

    let redis_client = redis::Client::open(args.redis_endpoint)?;
    info!("connecting to redis");
    Ok(())
}
