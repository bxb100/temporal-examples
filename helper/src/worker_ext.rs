use crate::get_type_name;
use temporal_sdk::{IntoActivityFunc, Worker, WorkflowFunction};

pub trait WorkerExt {
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
    fn register_act<T, A, R, O>(&mut self, t: T) -> &mut Self
    where
        T: IntoActivityFunc<A, R, O>,
    {
        let (l, r) = get_type_name(t);
        self.register_activity(l, r);
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
