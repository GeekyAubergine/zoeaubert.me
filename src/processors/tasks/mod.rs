use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{prelude::*, services::ServiceContext};

pub trait Task: Send {
    type Output: Send;

    fn run(self, ctx: &ServiceContext) -> Result<Self::Output>;
}

// Some tasks are IO blocking, mostly through networking
// I considered splitting them out the but most of the tasks IO bound tasks are limited by query limiters anyway
// so in practice so few occur it's not work splitting out at this time
pub fn run_tasks<T: Task>(tasks: Vec<T>, ctx: &ServiceContext) -> Result<Vec<T::Output>> {
    tasks
        .into_iter()
        .par_bridge()
        .map(|task| task.run(ctx))
        .collect()
}
