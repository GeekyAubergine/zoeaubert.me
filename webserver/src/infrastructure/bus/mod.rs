pub mod event_queue;
pub mod job_runner;
pub mod logger_listener;

use crate::application::events::Event;

use self::{
    event_queue::{EventListener, EventQueue},
    job_runner::{BoxedJob, JobRunner},
};

use super::app_state::{self, AppState};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task,
    time::{sleep, Duration},
};

pub struct Bus {
    app_state: AppState,
    event_queue: EventQueue,
    high_priority_job_runner: JobRunner,
    normal_priority_job_runner: JobRunner,
    low_priority_job_runner: JobRunner,
}

impl Bus {
    pub fn new(
        app_state: AppState,
        event_receiver: Receiver<Event>,
        high_priority_job_receiver: Receiver<BoxedJob>,
        normal_priority_job_receiver: Receiver<BoxedJob>,
        low_priority_job_receiver: Receiver<BoxedJob>,
    ) -> Self {
        let event_queue = EventQueue::new(app_state.clone(), event_receiver);

        let high_priority_job_runner = JobRunner::new(app_state.clone(), high_priority_job_receiver);
        let normal_priority_job_runner = JobRunner::new(app_state.clone(), normal_priority_job_receiver);
        let low_priority_job_runner = JobRunner::new(app_state.clone(), low_priority_job_receiver);

        Self {
            app_state,
            event_queue,
            high_priority_job_runner,
            normal_priority_job_runner,
            low_priority_job_runner,
        }
    }

    pub fn add_event_listener(&mut self, listener: Box<dyn EventListener>) {
        self.event_queue.add_event_listener(listener);
    }

    pub async fn start(mut self) {
        task::spawn(async move {
            self.event_queue.run().await;
        });

        task::spawn(async move {
            self.high_priority_job_runner.run().await;
        });

        task::spawn(async move {
            self.normal_priority_job_runner.run().await;
        });

        task::spawn(async move {
            self.low_priority_job_runner.run().await;
        });
    }
}
