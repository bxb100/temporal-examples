use anyhow::Result;
use dsl_interpreter::worker;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    worker::start_worker().await
}
