use temporal_sdk::{ActContext, ActExitValue, ActivityError};

pub async fn activity5(
    _ctx: ActContext,
    input: Vec<String>,
) -> Result<ActExitValue<String>, ActivityError> {
    if let [ref arg_arg3, ref arg_result4, ..] = input[..] {
        return Ok(ActExitValue::Normal(format!(
            r#"activity5 received:
  arg3: {arg_arg3}:
  result4: {arg_result4}:"#
        )));
    }

    Err(anyhow::anyhow!("required 2 args, but only received {:#?} ", input).into())
}
