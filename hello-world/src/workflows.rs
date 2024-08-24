use helper::parse_activity_result::parse_activity_result;
use log::info;
use std::time::Duration;
use temporal_sdk::{ActivityOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};

/// A workflow that simply calls an activity
pub async fn example(ctx: WfContext) -> WorkflowResult<String> {
    let args = ctx.get_args();

    let input = String::from_json_payload(args.first().unwrap())?;
    info!("Starting workflow with input: {}", input);

    let resolution = ctx
        .activity(ActivityOptions {
            activity_type: "activities".into(),
            input: input.as_json_payload()?,
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        })
        .await;

    let v = parse_activity_result::<String>(&resolution)?;
    info!("Activity completed with: {}", v);
    Ok(WfExitValue::Normal(v))
}
