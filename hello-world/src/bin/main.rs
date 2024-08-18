use anyhow::Result;
use hello_world::worker;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    worker::start_worker().await
}
