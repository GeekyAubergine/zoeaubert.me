use crate::{
    domain::models::{game::Game},
    infrastructure::app_state::AppState,
    prelude::Result,
};

pub struct GameWithAchievementsQueryService;

impl GameWithAchievementsQueryService {
    // pub async fn commit(
    //     state: &AppState,
    //     game_with_achievements: &GameWithAchievements,
    // ) -> Result<()> {
    //     state
    //         .games_repo()
    //         .commit(game_with_achievements.game())
    //         .await?;

    //     for achievement in game_with_achievements.achievements().values() {
    //         state.game_achievements_repo().commit(achievement).await?;
    //     }

    //     Ok(())
    // }

    // pub async fn find_by_game_id(
    //     state: &AppState,
    //     game_id: u32,
    // ) -> Result<Option<GameWithAchievements>> {
    //     let game = state.games_repo().find_by_id(game_id).await?;

    //     if let Some(game) = game {
    //         let achievements = state
    //             .game_achievements_repo()
    //             .find_by_game_id(game.id())
    //             .await?;

    //         let mut game_with_achievements = GameWithAchievements::from_game(game);

    //         for achievement in achievements {
    //             game_with_achievements.add_achievement(achievement);
    //         }

    //         Ok(Some(game_with_achievements))
    //     } else {
    //         Ok(None)
    //     }
    // }

    // pub async fn get_games_by_most_completed_achievements(&self) -> Vec<GameWithAchievments> {
    //     return vec![];

    //     // let games = self.games.read().await;

    //     // let mut games_array = games.values().cloned().collect::<Vec<Game>>();

    //     // games_array.sort_by(|a, b| {
    //     //     b.achievements_unlocked_count()
    //     //         .cmp(&a.achievements_unlocked_count())
    //     // });

    //     // games_array
    // }

    // pub async fn get_all_unlocked_acheivements_for_game(
    //     &self,
    //     id: u32,
    // ) -> Vec<GameAchievementUnlocked> {
    //     return vec![];

    //     // let games = self.games.read().await;

    //     // match games.get(&id) {
    //     //     Some(game) => game
    //     //         .achievements()
    //     //         .values()
    //     //         .filter_map(|achievement| match achievement {
    //     //             GameAchievement::Unlocked(unlocked) => Some(unlocked.clone()),
    //     //             _ => None,
    //     //         })
    //     //         .collect(),
    //     //     None => vec![],
    //     // }
    // }
}
