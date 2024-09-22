use temporal_sdk::{ActContext, ActExitValue, ActivityError};
use log::info;

pub async fn activity5(
    _ctx: ActContext,
    input: Vec<String>,
) -> Result<ActExitValue<String>, ActivityError> {
    let input = &input[0];
    info!("Executing activity5 {}", input);
    Ok(ActExitValue::Normal(format!(
        "[result from activity5: {input}]"
    )))
}
