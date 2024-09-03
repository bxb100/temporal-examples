use anyhow::anyhow;
use log::info;
use serde::Deserialize;
use std::collections::HashMap;
use temporal_sdk::{ActContext, ActivityError};

#[derive(Deserialize, Debug)]
struct Response {
    args: HashMap<String, String>,
}

pub async fn make_http_request(_ctx: ActContext, _input: String) -> Result<String, ActivityError> {
    let res = reqwest::get("https://httpbin.org/get?answer=42")
        .await?
        .json::<Response>()
        .await?;

    info!("Got response: {:?}", res);

    match res.args.get("answer") {
        None => Err(anyhow!("No answer found").into()),
        Some(answer) => Ok(answer.to_owned()),
    }
}
