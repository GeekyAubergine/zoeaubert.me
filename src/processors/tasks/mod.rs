use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{prelude::*, services::ServiceContext};

pub trait Task: Send {
    type Output: Send;

    fn run(self, ctx: &ServiceContext) -> Result<Self::Output>;
}

pub fn run_tasks<T: Task>(tasks: Vec<T>, ctx: &ServiceContext) -> Result<Vec<T::Output>> {
    tasks
        .into_iter()
        .par_bridge()
        .map(|task| task.run(ctx))
        .collect()
}
