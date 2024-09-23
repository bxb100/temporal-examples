use anyhow::anyhow;
use temporal_sdk::{ActContext, ActExitValue, ActivityError};

pub async fn activity3(
    _ctx: ActContext,
    input: Vec<String>,
) -> Result<ActExitValue<String>, ActivityError> {
    if let [ref arg_arg2, ref arg_result2, ..] = input[..] {
        return Ok(ActExitValue::Normal(format!(
            r#"activity3 received arg2: {arg_arg2}:

  And received: {arg_result2}"#
        )));
    }

    Err(anyhow!("required 2 args, but only received {:#?} ", input).into())
}
