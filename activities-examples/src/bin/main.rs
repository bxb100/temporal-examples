use activities_examples::worker;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    worker::start_worker().await?;
    Ok(())
}
