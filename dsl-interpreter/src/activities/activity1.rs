use log::info;
use temporal_sdk::{ActContext, ActExitValue, ActivityError};

pub async fn activity1(
    _ctx: ActContext,
    input: Vec<String>,
) -> Result<ActExitValue<String>, ActivityError> {
    let input = &input[0];
    info!("Executing activity1 {}", input);
    Ok(ActExitValue::Normal(format!(
        "[result from activity1: {input}]"
    )))
}
