use anyhow::Result;
use hello_world::worker;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    worker::start_worker().await
}
