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

fn tuple_type_with_name<T>(t: T) -> (&'static str, T) {
    (std::any::type_name::<T>(), t)
}

impl WorkerExt for Worker {
    fn register_act<T, A, R, O>(&mut self, t: T) -> &mut Self
    where
        T: IntoActivityFunc<A, R, O>,
    {
        let (l, r) = tuple_type_with_name(t);
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
