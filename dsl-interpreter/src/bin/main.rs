use anyhow::Result;
use dsl_interpreter::worker;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    worker::start_worker().await
}
