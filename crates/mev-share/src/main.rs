use mev_share::cli;

#[tokio::main]
async fn main() {
    cli::run().await.expect("failed to start mev-share");
}
