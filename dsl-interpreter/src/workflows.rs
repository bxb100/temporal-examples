use crate::activities::*;
use helper::util::get_mod_simple_name;
use helper::wf_context_ext::{ProxyActivityFn, ProxyActivityOptions, WfContextExt};
use log::info;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::FromJsonPayloadExt;

use crate::dsl::{execution::execution, types::Dsl};

macro_rules! build_acts {
    ($ctx:expr, $($func:ident),+) => {
        {
            let mut map = HashMap::new();
            $(
                let a = _proxy($ctx, $func);
                map.insert(a.0, a.1);
            )+
            map
        }
    };
}

fn _proxy<T>(ctx: &WfContext, func: T) -> (String, ProxyActivityFn) {
    let name = get_mod_simple_name::<T>();
    let func = ctx.proxy_activity(
        func,
        ProxyActivityOptions {
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        },
    );
    (name.to_string(), func)
}

pub async fn example(ctx: WfContext) -> WorkflowResult<()> {
    let args = ctx.get_args();
    let input = Dsl::from_json_payload(args.first().unwrap())?;

    info!("Starting workflow with input: {:?}", input);
    let acts = Arc::new(build_acts!(
        &ctx, activity1, activity2, activity3, activity4, activity5
    ));
    let bindings = Arc::new(Mutex::new(input.variables));

    execution(input.root, bindings, acts.clone()).await?;

    Ok(WfExitValue::Normal(()))
}
