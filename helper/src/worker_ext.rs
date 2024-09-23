use crate::util::get_mod_simple_name;
use std::future::Future;
use std::sync::Arc;
use temporal_sdk::{IntoActivityFunc, Worker, WorkflowFunction};
use temporal_sdk_core::init_worker;
use temporal_sdk_core_api::worker::WorkerConfig;

pub trait WorkerExt {
    fn single(worker_config: WorkerConfig) -> impl Future<Output = anyhow::Result<Worker>> + Send;

    fn register_act<T, A, R, O>(&mut self, t: T) -> &mut Self
    where
        T: IntoActivityFunc<A, R, O>;

    fn register_workflow<F: Into<WorkflowFunction>>(
        &mut self,
        name: impl Into<String>,
        f: F,
    ) -> &mut Self;
}

impl WorkerExt for Worker {
    async fn single(worker_config: WorkerConfig) -> anyhow::Result<Worker> {
        let task_queue = worker_config.task_queue.clone();

        let client = super::get_client().await?;
        let core_worker = init_worker(super::core_runtime(), worker_config, client)?;
        Ok(Worker::new_from_core(Arc::new(core_worker), task_queue))
    }

    fn register_act<F, A, R, O>(&mut self, f: F) -> &mut Self
    where
        F: IntoActivityFunc<A, R, O>,
    {
        let name = get_mod_simple_name::<F>();
        self.register_activity(name, f);
        self
    }

    fn register_workflow<F: Into<WorkflowFunction>>(
        &mut self,
        name: impl Into<String>,
        f: F,
    ) -> &mut Self {
        self.register_wf(name, f);
        self
    }
}
