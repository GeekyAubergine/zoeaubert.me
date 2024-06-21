use async_trait::async_trait;
use tracing::{debug, info};

use crate::{
    application::events::Event,
    infrastructure::{app_state::AppState, bus::event_queue::EventListener},
    prelude::*,
};

pub struct LoggerListener;

impl LoggerListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for LoggerListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        debug!("Processing event: {:?}", event.name());

        Ok(())
    }
}
