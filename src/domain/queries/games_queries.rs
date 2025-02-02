use crate::domain::models::game::{Game, GameAchievmentLocked, GameAchievmentUnlocked};
use crate::domain::repositories::{SteamAchievementsRepo, SteamGamesRepo};
use crate::domain::state::State;
use crate::prelude::*;

pub struct GameWithAchievements {
    pub game: Game,
    pub unlocked: Vec<GameAchievmentUnlocked>,
    pub locked: Vec<GameAchievmentLocked>,
}

impl GameWithAchievements {
    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn unlocked(&self) -> &[GameAchievmentUnlocked] {
        &self.unlocked
    }

    pub fn locked(&self) -> &[GameAchievmentLocked] {
        &self.locked
    }

    pub fn total_achievements(&self) -> usize {
        self.unlocked.len() + self.locked.len()
    }

    pub fn achievement_unlocked_percentage(&self) -> f32 {
        let unlocked_count = self.unlocked.len() as f32;
        let total_count = (self.unlocked.len() + self.locked.len()) as f32;

        unlocked_count / total_count.max(1.0)
    }
}

async fn find_all_games_and_achievements(state: &impl State) -> Result<Vec<GameWithAchievements>> {
    let steam_games = state.steam_games_repo().find_all().await?;

    let mut out: Vec<GameWithAchievements> = vec![];

    for game in steam_games {
        let unlocked = state
            .steam_achievements_repo()
            .find_all_unlocked_by_unlocked_date(game.id)
            .await?;
        let locked = state
            .steam_achievements_repo()
            .find_all_locked_by_name(game.id)
            .await?;

        out.push(GameWithAchievements {
            game: Game::from(game),
            unlocked: unlocked
                .into_iter()
                .map(GameAchievmentUnlocked::from)
                .collect::<Vec<GameAchievmentUnlocked>>(),
            locked: locked
                .into_iter()
                .map(GameAchievmentLocked::from)
                .collect::<Vec<GameAchievmentLocked>>(),
        });
    }

    Ok(out)
}

pub async fn find_all_games_by_achievment_unlocked_percentage(
    state: &impl State,
) -> Result<Vec<GameWithAchievements>> {
    let steam_games = state.steam_games_repo().find_all().await?;

    let mut games = find_all_games_and_achievements(state).await?;

    games.sort_by(|a, b| {
        let a_percentage = a.achievement_unlocked_percentage();
        let b_percentage = b.achievement_unlocked_percentage();

        b_percentage.partial_cmp(&a_percentage).unwrap()
    });

    Ok(games)
}

pub async fn find_all_games_by_most_recently_played(state: &impl State) -> Result<Vec<Game>> {
    let steam_games = state.steam_games_repo().find_all().await?;

    let mut games = steam_games.into_iter().map(Game::from).collect::<Vec<_>>();

    games.sort_by(|a, b| b.last_played().cmp(&a.last_played()));

    Ok(games)
}
