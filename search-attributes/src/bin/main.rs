use anyhow::Result;
use search_attributes::worker;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    worker::start_worker().await
}
