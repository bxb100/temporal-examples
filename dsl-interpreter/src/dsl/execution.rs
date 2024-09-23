use crate::dsl::types::Statement;
use helper::payload_ext::PayloadExt;
use helper::wf_context_ext::ProxyActivityOptions;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use temporal_sdk::{ActivityOptions, WfContext};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

/// key is the activity name, value is the activity options
pub type Acts = Arc<HashMap<String, ProxyActivityOptions>>;
pub type Bindings = Arc<Mutex<HashMap<String, String>>>;

pub async fn execution(statements: Statement, bindings: Bindings, ctx: &WfContext, acts: Acts) {
    match statements {
        Statement::Parallel(parallel) => {
            let mut futures = vec![];
            for statement in parallel.branches {
                let bindings = bindings.clone();
                let ctx = ctx.clone();
                let acts = acts.clone();
                futures.push(async move {
                    Box::pin(execution(statement, bindings, &ctx, acts)).await;
                });
            }
            let _ = futures::future::join_all(futures).await;
        }
        Statement::Sequence(sequence) => {
            for statement in sequence.elements {
                Box::pin(execution(statement, bindings.clone(), ctx, acts.clone())).await;
            }
        }
        Statement::Activity(activity) => {
            let activity_name = activity.name.clone();
            let args = activity.arguments.unwrap_or_default();
            let args = args
                .iter()
                .map(|arg| {
                    let bindings = bindings.lock().unwrap();
                    bindings.get(arg).unwrap_or(arg).clone()
                })
                .collect::<Vec<_>>();
            let options = acts.get(&activity_name).unwrap();
            // execution activity
            let res = ctx
                .activity(ActivityOptions {
                    activity_type: activity_name,
                    input: args.as_json_payload().unwrap(),
                    schedule_to_start_timeout: options.schedule_to_start_timeout,
                    start_to_close_timeout: options.start_to_close_timeout,
                    schedule_to_close_timeout: options.schedule_to_close_timeout,
                    heartbeat_timeout: options.heartbeat_timeout,
                    cancellation_type: options.cancellation_type,
                    task_queue: options.task_queue.clone(),
                    activity_id: options.activity_id.clone(),
                    retry_policy: options.retry_policy.clone(),
                })
                .await;
            if activity.result.is_some() {
                let mut bindings = bindings.lock().unwrap();
                bindings.insert(
                    activity.result.unwrap(),
                    res.unwrap_ok_payload().deserialize::<String>().unwrap(),
                );
            }
        }
    }
}
