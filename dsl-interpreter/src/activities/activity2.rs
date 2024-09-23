use log::info;
use temporal_sdk::{ActContext, ActExitValue, ActivityError};

pub async fn activity2(
    _ctx: ActContext,
    input: Vec<String>,
) -> Result<ActExitValue<String>, ActivityError> {
    let input = &input[0];
    info!("Executing activity2 {}", input);
    Ok(ActExitValue::Normal(format!(
        "[result from activity2: {input}]"
    )))
}
