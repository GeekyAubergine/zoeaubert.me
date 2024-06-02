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
    job_runner: JobRunner,
    event_queue: EventQueue,
}

impl Bus {
    pub fn new(
        app_state: AppState,
        job_receiver: Receiver<BoxedJob>,
        event_receiver: Receiver<Event>,
    ) -> Self {
        let job_runner = JobRunner::new(app_state.clone(), job_receiver);
        let event_queue = EventQueue::new(app_state.clone(), event_receiver);

        Self {
            app_state,
            job_runner,
            event_queue,
        }
    }

    pub fn add_event_listener(&mut self, listener: Box<dyn EventListener>) {
        self.event_queue.add_event_listener(listener);
    }

    pub async fn start(mut self) {
        task::spawn(async move {
            self.job_runner.run().await;
        });

        task::spawn(async move {
            self.event_queue.run().await;
        });
    }
}
