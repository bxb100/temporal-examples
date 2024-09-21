use schedules::worker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    worker::start_worker().await
}
