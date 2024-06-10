use std::{collections::VecDeque, future::Future};

use async_trait::async_trait;
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task,
    time::{sleep, Duration},
};

use tracing::{error, info};

use crate::{application::events::Event, prelude::*};

use super::app_state::{self, AppState};

#[async_trait]
pub trait Job: Send + Sync {
    fn name(&self) -> &str;

    async fn run(&self, app_state: &AppState) -> Result<()>;
}

pub type BoxedJob = Box<dyn Job>;

pub fn make_job_channel() -> (Sender<BoxedJob>, Receiver<BoxedJob>) {
    channel(1000)
}

pub struct JobRunner {
    app_state: AppState,
    job_receiver: Receiver<BoxedJob>,
}

impl JobRunner {
    pub fn new(app_state: AppState, job_receiver: Receiver<BoxedJob>) -> Self {
        Self {
            app_state,
            job_receiver,
        }
    }

    pub async fn run(&mut self) {
        loop {
            while let Some(job) = self.job_receiver.recv().await {
                info!("Running job: {}", job.name());
                let result = job.run(&self.app_state).await;
                if let Err(err) = result {
                    error!("Job {} failed: {}", job.name(), err);
                }
            }
        }
    }
}
