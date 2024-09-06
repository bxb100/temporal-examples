use helper::payload_ext::PayloadExt;
use temporal_sdk::{ChildWorkflowOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core_protos::coresdk::child_workflow::{child_workflow_result::Status, Success};

pub async fn parent_workflow(ctx: WfContext) -> WorkflowResult<String> {
    let names = ctx.get_args();
    let mut handlers = Vec::with_capacity(names.len());

    for p in names {
        handlers.push(tokio::spawn(
            ctx.child_workflow(ChildWorkflowOptions {
                // workflow_id // must be fixed in current scope
                workflow_id: format!("child-workflow-{}", p.deserialize::<String>()?),
                workflow_type: "child_workflow".to_string(),
                input: vec![p.clone()],
                // // regular workflow options apply here, with two additions (defaults shown):
                // cancel_type: ChildWorkflowCancellationType::WaitCancellationCompleted,
                // parent_close_policy: ParentClosePolicy::Terminate,
                ..Default::default()
            })
            .start(&ctx),
        ));
    }

    let mut pending_futures = Vec::with_capacity(handlers.len());
    for handler in handlers {
        pending_futures.push(handler.await);
    }

    let mut s = Vec::with_capacity(pending_futures.len());
    for pending in pending_futures {
        if let Some(Status::Completed(Success {
            result: Some(ref payload),
        })) = pending?.into_started().unwrap().result().await.status
        {
            s.push(payload.deserialize::<String>()?);
        }
    }

    Ok(WfExitValue::Normal(s.join("\n")))
}

pub async fn child_workflow(ctx: WfContext) -> WorkflowResult<String> {
    let name = ctx.get_args()[0].deserialize::<String>()?;
    Ok(WfExitValue::Normal(format!("I am a child named {}", name)))
}
