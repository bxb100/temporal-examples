use std::time::Duration;
use temporal_sdk::{ActivityOptions, CancellableFuture, WfContext};
use temporal_sdk_core::protos::coresdk::activity_result::ActivityResolution;
use temporal_sdk_core::protos::coresdk::workflow_commands::ActivityCancellationType;
use temporal_sdk_core::protos::temporal::api::common::v1::{Payload, RetryPolicy};

#[derive(Debug, Default)]
pub struct ProxyActivityOptions {
    pub activity_id: Option<String>,
    pub task_queue: Option<String>,
    pub schedule_to_start_timeout: Option<Duration>,
    pub start_to_close_timeout: Option<Duration>,
    pub schedule_to_close_timeout: Option<Duration>,
    pub heartbeat_timeout: Option<Duration>,
    pub cancellation_type: ActivityCancellationType,
    pub retry_policy: Option<RetryPolicy>,
}

pub trait WfContextExt {
    fn proxy_activity<'a, T>(
        self: &'a Self,
        _: T,
        options: ProxyActivityOptions,
    ) -> Box<dyn FnOnce(Payload) -> Box<dyn CancellableFuture<ActivityResolution> + Send> + 'a>;
}

impl WfContextExt for WfContext {
    fn proxy_activity<'a, T>(
        self: &'a Self,
        _: T,
        options: ProxyActivityOptions,
    ) -> Box<dyn FnOnce(Payload) -> Box<dyn CancellableFuture<ActivityResolution> + Send> + 'a>
    {
        let name = std::any::type_name::<T>();
        Box::new(move |input: Payload| {
            Box::new(self.activity(ActivityOptions {
                activity_type: name.to_string(),
                input,
                activity_id: options.activity_id.clone(),
                task_queue: options.task_queue.clone(),
                schedule_to_start_timeout: options.schedule_to_start_timeout,
                start_to_close_timeout: options.start_to_close_timeout,
                schedule_to_close_timeout: options.schedule_to_close_timeout,
                heartbeat_timeout: options.heartbeat_timeout,
                cancellation_type: options.cancellation_type,
                retry_policy: options.retry_policy.clone(),
            }))
        })
    }
}
