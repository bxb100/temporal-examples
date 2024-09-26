use anyhow::Result;
use encryption::worker;
use encryption::worker::init_codec;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    // init codec
    unsafe {
        init_codec().await;
    }

    worker::start_worker().await
}
