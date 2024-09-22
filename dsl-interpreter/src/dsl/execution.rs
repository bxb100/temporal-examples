use crate::dsl::types::{ParallelStruct, Statement};
use async_scoped::TokioScope;
use helper::payload_ext::PayloadExt;
use helper::wf_context_ext::ProxyActivityFn;
use log::{error, info};
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

pub type Acts<'a> = Arc<HashMap<String, ProxyActivityFn<'a>>>;
pub type Bindings = Arc<Mutex<HashMap<String, String>>>;

pub async fn execution(
    statement: Statement,
    bindings: Bindings,
    acts: Acts<'_>,
) -> anyhow::Result<()> {
    info!("executing: {:?}", statement);
    match statement {
        Statement::Activity(activity) => {
            let args = activity.arguments.unwrap_or(vec![]);
            let args = args
                .iter()
                .map(|arg| {
                    let bindings = bindings.lock().unwrap();
                    bindings.get(arg).unwrap_or(arg).clone()
                })
                .collect::<Vec<_>>();
            info!("args: {:?}", args);
            let func = acts[&activity.name](args.as_json_payload()?).await;
            if let Some(result) = activity.result {
                let mut bindings = bindings.lock().unwrap();
                bindings.insert(result, func.unwrap_ok_payload().deserialize::<String>()?);
            }
        }
        Statement::Sequence(seq) => {
            for element in seq.elements {
                if let Err(err) = Box::pin(execution(element, bindings.clone(), acts.clone())).await
                {
                    error!("error in sequence: {:?}", err);
                    return Err(err);
                }
            }
        }
        Statement::Parallel(parallel) => {
            let count = Arc::new(AtomicUsize::new(parallel.branches.len()));
            // spawn_parallel(parallel, bindings.clone(), acts.clone(), count.clone());
            while count.load(std::sync::atomic::Ordering::Relaxed) > 0 {
                tokio::task::yield_now().await;
            }
        }
    }

    Ok(())
}

// fn spawn_parallel(
//     parallel: ParallelStruct,
//     bindings: Bindings,
//     acts: Acts<'_>,
//     count: Arc<AtomicUsize>,
// ) {
//     unsafe {
//         // !! you know what, I'm not sure if this is the right way to do this
//         TokioScope::scope(|s| {
//             for branch in parallel.branches {
//                 let bindings = bindings.clone();
//                 let acts = acts.clone();
//                 let count = count.clone();
//                 s.spawn(async move {
//                     match execution(branch, bindings, acts).await {
//                         Ok(_) => {
//                             count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
//                         }
//                         Err(e) => {
//                             log::error!("error in parallel branch: {:#?}", e);
//                         }
//                     }
//                 });
//             }
//         });
//     }
// }
