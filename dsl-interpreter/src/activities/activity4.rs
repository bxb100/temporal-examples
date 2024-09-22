use temporal_sdk::{ActContext, ActExitValue, ActivityError};
use log::info;

pub async fn activity4(
    _ctx: ActContext,
    input: Vec<String>,
) -> Result<ActExitValue<String>, ActivityError> {
    let input = &input[0];
    info!("Executing activity4 {}", input);
    Ok(ActExitValue::Normal(format!(
        "[result from activity4: {input}]"
    )))
}
