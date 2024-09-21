use helper::util::activity_input::ActivityInput;
use serde::Deserialize;
use temporal_sdk::{ActContext, ActivityError};
use tracing::info;

#[derive(Deserialize, Debug)]
struct Response {
    args: Args,
}

#[derive(Deserialize, Debug)]
struct Args {
    answer: String,
}

pub async fn make_http_request(
    _ctx: ActContext,
    _input: ActivityInput<String>,
) -> Result<String, ActivityError> {
    info!("{:?}", _input);

    let res = reqwest::get("https://httpbin.org/get?answer=42")
        .await?
        .json::<Response>()
        .await?;

    info!("Got response: {:?}", res);

    Ok(res.args.answer)
}
