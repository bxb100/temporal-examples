use continue_as_new::worker;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    worker::start_worker().await?;
    Ok(())
}
