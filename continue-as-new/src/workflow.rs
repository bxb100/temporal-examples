use helper::payload_ext::PayloadExt;
use log::info;
use std::time::Duration;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core_protos::coresdk::workflow_commands::ContinueAsNewWorkflowExecution;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

pub async fn looping_workflow(ctx: WfContext) -> WorkflowResult<()> {
    let iteration = ctx.get_args()[0].deserialize::<u32>()?;
    if iteration >= 10 {
        return Ok(WfExitValue::Normal(()));
    }

    let timer_res = ctx.timer(Duration::from_millis(1000)).await;

    info!("timer result {:?}", timer_res);

    Ok(WfExitValue::ContinueAsNew(Box::new(
        ContinueAsNewWorkflowExecution {
            workflow_type: "looping_workflow".to_string(),
            task_queue: ctx.task_queue().to_string(),
            arguments: vec![(iteration + 1).as_json_payload()?],
            ..Default::default()
        },
    )))
}
