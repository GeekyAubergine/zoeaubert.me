use crate::prelude::*;

use crate::infrastructure::app_state::{self, AppState};

use super::omni_post_models::OmniPost;

pub struct OmniPostRepo;

impl OmniPostRepo {
    pub async fn get_posts_ordered_by_date(app_state: &AppState) -> Result<Vec<OmniPost>> {
        let mut posts = vec![];

        let all_games = app_state.games_repo().get_all_games().await;

        for game in all_games.values() {
            let unlocked_achievements = app_state
                .games_repo()
                .get_all_unlocked_acheivements_for_game(game.id())
                .await;

            for achievement in unlocked_achievements {
                let post = OmniPost::UnlockedGameAchievement {
                    game: game.clone(),
                    achievement: achievement.clone(),
                };
                posts.push(post);
            }
        }

        posts.sort_by(|a, b| b.date().cmp(&a.date()));

        Ok(posts)
    }
}
