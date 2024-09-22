use anyhow::anyhow;
use temporal_sdk::{ActContext, ActExitValue, ActivityError};

pub async fn activity3(
    _ctx: ActContext,
    input: Vec<String>,
) -> Result<ActExitValue<String>, ActivityError> {
    let arg2 = input.get(0).ok_or(anyhow!("missing arg2"))?;
    let arg = input.get(1).ok_or(anyhow!("missing arg"))?;

    Ok(ActExitValue::Normal(format!(
        r#"activity3 received arg2: {arg2}:

  And received: {arg}"#
    )))
}
