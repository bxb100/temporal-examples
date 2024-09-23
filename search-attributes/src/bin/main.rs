use anyhow::Result;
use search_attributes::worker;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    worker::start_worker().await
}
