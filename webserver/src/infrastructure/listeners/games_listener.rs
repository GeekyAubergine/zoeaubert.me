use crate::application::jobs::games::fetch_game_data_from_steam_job::FetchGameAchievementsFromSteamJob;
use crate::application::jobs::games::fetch_games_data_from_steam_job::FetchGamesDataJob;
use crate::infrastructure::bus::event_queue::EventListener;
use crate::infrastructure::bus::job_runner::JobPriority;
use crate::{
    prelude::*, GAMES_ARCHIVE_FILENAME, LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

pub struct GamesListener;

impl GamesListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for GamesListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        match event {
            Event::ServerBooted => {
                app_state
                    .dispatch_job(FetchGamesDataJob::new(), JobPriority::Normal)
                    .await?;
            }
            Event::GameUpdated { game_id } => {
                app_state
                    .dispatch_job(
                        FetchGameAchievementsFromSteamJob::new(*game_id),
                        JobPriority::Normal,
                    )
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
