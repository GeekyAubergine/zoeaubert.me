use std::{collections::VecDeque, future::Future};

use async_trait::async_trait;
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task,
    time::{sleep, Duration},
};

use super::app_state::{self, AppState};

use tracing::error;

use crate::{application::events::Event, prelude::*};

pub fn make_event_channel() -> (Sender<Event>, Receiver<Event>) {
    channel(1000)
}

#[async_trait]
pub trait EventListener: Send + Sync {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()>;
}

pub struct EventQueue {
    app_state: AppState,
    event_receiver: Receiver<Event>,
    event_listeners: Vec<Box<dyn EventListener>>,
}

impl EventQueue {
    pub fn new(app_state: AppState, event_receiver: Receiver<Event>) -> Self {
        Self {
            app_state,
            event_receiver,
            event_listeners: Vec::new(),
        }
    }

    pub fn add_event_listener(&mut self, listener: Box<dyn EventListener>) {
        self.event_listeners.push(listener);
    }

    pub async fn run(&mut self) {
        loop {
            while let Some(event) = self.event_receiver.recv().await {
                for listener in &self.event_listeners {
                    let result = listener.on_event(&event, &self.app_state).await;
                    if let Err(err) = result {
                        error!("Event listener failed: {}", err);
                    }
                }
            }
        }
    }
}
