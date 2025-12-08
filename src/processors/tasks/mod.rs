use std::sync::Arc;

use rayon::iter::{ParallelBridge, ParallelIterator};
use reqwest::blocking::Client;

use crate::{prelude::*, services::{ServiceContext, cdn_service::CdnService, network_service::NetworkService}};

pub trait Task: Send {
    type Output: Send;

    fn run(self, ctx: &ServiceContext) -> Result<Self::Output>;
}

struct TestTask;

impl Task for TestTask {
    type Output = u32;

    fn run(self, ctx: &ServiceContext) -> Result<u32> {
        Ok(0)
    }
}

pub fn run_tasks<T: Task>(tasks: Vec<T>, ctx: &ServiceContext) -> Result<Vec<T::Output>> {
    tasks.into_iter().par_bridge().map(|task| task.run(ctx)).collect()
}
