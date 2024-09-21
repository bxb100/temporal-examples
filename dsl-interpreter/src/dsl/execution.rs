use crate::dsl::dsl::Statement;
use helper::payload_ext::PayloadExt;
use helper::wf_context_ext::ProxyActivityFn;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

pub type Acts = Arc<HashMap<String, ProxyActivityFn<'static>>>;
pub type Bindings = Arc<Mutex<HashMap<String, serde_json::Value>>>;

pub async fn execution(statement: Statement, bindings: Bindings, acts: Acts) {
    match statement {
        Statement::Activity(activity) => {
            let args = activity.arguments.unwrap_or(vec![]);
            let args = args
                .iter()
                .map(|arg| {
                    let bindings = bindings.lock().unwrap();
                    if let Some(value) = bindings.get(arg) {
                        value.clone()
                    } else {
                        serde_json::Value::String(arg.clone())
                    }
                })
                .collect::<Vec<_>>();
            let func = acts[&activity.name](args.as_json_payload().unwrap()).await;
            if let Some(result) = activity.result {
                let mut bindings = bindings.lock().unwrap();
                bindings.insert(
                    result,
                    serde_json::Value::String(
                        func.unwrap_ok_payload().deserialize::<String>().unwrap(),
                    ),
                );
            }
        }
        Statement::Sequence(seq) => {
            for element in seq.elements {
                Box::pin(execution(element, bindings.clone(), acts.clone())).await;
            }
        }
        Statement::Parallel(parallel) => {
            let count = Arc::new(AtomicUsize::new(parallel.branches.len()));
            for branch in parallel.branches {
                spawn_parallel(branch, bindings.clone(), acts.clone(), count.clone());
            }
            while count.load(std::sync::atomic::Ordering::SeqCst) > 0 {
                tokio::task::yield_now().await;
            }
        }
    }
}

fn spawn_parallel(statement: Statement, bindings: Bindings, acts: Acts, count: Arc<AtomicUsize>) {
    tokio::spawn(async move {
        execution(statement, bindings, acts).await;
        count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    });
}
