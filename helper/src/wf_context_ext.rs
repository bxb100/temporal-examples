use crate::util::get_mod_simple_name;
use std::pin::Pin;
use std::time::Duration;
use temporal_sdk::{ActivityOptions, CancellableFuture, WfContext};
use temporal_sdk_core::protos::coresdk::activity_result::ActivityResolution;
use temporal_sdk_core::protos::coresdk::workflow_commands::ActivityCancellationType;
use temporal_sdk_core::protos::temporal::api::common::v1::{Payload, RetryPolicy};

#[derive(Debug, Default, Clone)]
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
impl ProxyActivityOptions {
    pub fn convert_to(&self, activity_type: impl Into<String>, input: Payload) -> ActivityOptions {
        ActivityOptions {
            activity_type: activity_type.into(),
            input,
            activity_id: self.activity_id.clone(),
            task_queue: self.task_queue.clone(),
            schedule_to_start_timeout: self.schedule_to_start_timeout,
            start_to_close_timeout: self.start_to_close_timeout,
            schedule_to_close_timeout: self.schedule_to_close_timeout,
            heartbeat_timeout: self.heartbeat_timeout,
            cancellation_type: self.cancellation_type,
            retry_policy: self.retry_policy.clone(),
        }
    }
}

/// see [BoxFuture](temporal_sdk::BoxFuture), the `Send` is required by [register_workflow](crate::worker_ext::WorkerExt::register_workflow)
type PinProxyActivityFuture = Pin<Box<dyn CancellableFuture<ActivityResolution> + Send + 'static>>;

pub trait WfContextExt {
    fn proxy_activity<T>(
        self: &Self,
        _: T,
        options: ProxyActivityOptions,
    ) -> impl Fn(Payload) -> PinProxyActivityFuture;
}

impl WfContextExt for WfContext {
    fn proxy_activity<T>(
        self: &Self,
        _activity_func: T,
        options: ProxyActivityOptions,
    ) -> impl Fn(Payload) -> PinProxyActivityFuture {
        let name = get_mod_simple_name::<T>();
        move |input: Payload| Box::pin(self.activity(options.convert_to(name, input)))
    }
}
