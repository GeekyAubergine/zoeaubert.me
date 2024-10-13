use std::{collections::VecDeque, future::Future};

use async_trait::async_trait;
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::{sleep, Duration},
};

use tracing::{debug, error, info};

use crate::{application::events::Event, prelude::*};

use super::app_state::{self, AppState};

pub enum JobPriority {
    High,
    Normal,
    Low,
}

#[async_trait]
pub trait Job: Send + Sync {
    fn name(&self) -> &str;

    async fn run(&self, state: &AppState) -> Result<()>;
}

pub type BoxedJob = Box<dyn Job>;

pub fn make_job_channel() -> (Sender<BoxedJob>, Receiver<BoxedJob>) {
    channel(1000)
}

pub struct JobRunner {
    state: AppState,
    job_receiver: Receiver<BoxedJob>,
}

impl JobRunner {
    pub fn new(state: AppState, job_receiver: Receiver<BoxedJob>) -> Self {
        Self {
            state,
            job_receiver,
        }
    }

    pub async fn run(&mut self) {
        loop {
            while let Some(job) = self.job_receiver.recv().await {
                let app_state = self.state.clone();
                tokio::spawn(async move {
                    debug!("Running job: {}", job.name());
                    let result = job.run(&app_state).await;
                    if let Err(err) = result {
                        error!("Job {} failed: {}", job.name(), err);
                    }
                });
            }
        }
    }
}
