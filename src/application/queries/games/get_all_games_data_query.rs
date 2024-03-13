// use std::collections::HashMap;

// use axum::{
//     extract::{FromRef, State},
//     http::StatusCode,
//     response::{IntoResponse, Response},
//     Json,
// };
// use axum_extra::protobuf::Protobuf;
// use chrono::{DateTime, Utc};
// use serde::Serialize;

// use crate::{
//     domain::models::game::{Game, GameAchievement},
//     infrastructure::app_state::AppState,
//     prelude::*,
// };

// const RECENT_GAMES_COUNT: usize = 3;

// #[derive(Debug, Clone, Serialize)]
// struct ResponseGameAchievement {
//     id: String,
//     display_name: String,
//     description: String,
//     image_unlocked_url: String,
//     image_locked_url: String,
//     unlocked_data: Option<DateTime<Utc>>,
// }

// impl From<GameAchievement> for ResponseGameAchievement {
//     fn from(achievement: GameAchievement) -> Self {
//         Self {
//             id: achievement.id().to_string(),
//             display_name: achievement.display_name().to_string(),
//             description: achievement.description().to_string(),
//             image_unlocked_url: achievement.image_unlocked_url().to_string(),
//             image_locked_url: achievement.image_locked_url().to_string(),
//             unlocked_data: achievement.unlocked_date().map(|date| *date),
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize)]
// struct ResponseGame {
//     id: u32,
//     name: String,
//     header_image_url: String,
//     playtime: u32,
//     last_played: String,
//     link_url: String,
//     achievements_unlocked: u32,
//     achievements_total: u32,
//     achievements: HashMap<String, ResponseGameAchievement>,
//     achievment_ids: Vec<String>,
//     achievements_unlocked_ids: Vec<String>,
//     achievements_locked_ids: Vec<String>,
//     achievements_by_unlocked_date: Vec<String>,
// }

// impl From<Game> for ResponseGame {
//     fn from(game: Game) -> Self {
//         let achievements = game
//             .achievements()
//             .iter()
//             .map(|(key, achievement)| (key.to_string(), achievement.clone().into()))
//             .collect::<HashMap<String, ResponseGameAchievement>>();

//         Self {
//             id: game.id(),
//             name: game.name().to_string(),
//             header_image_url: game.header_image_url().to_string(),
//             playtime: game.playtime(),
//             last_played: game.last_played().to_string(),
//             link_url: game.link_url().to_string(),
//             achievements_unlocked: game.achievements_unlocked_count() as u32,
//             achievements_total: game.achievements_count() as u32,
//             achievements,
//             achievment_ids: game.achievements().keys().cloned().collect(),
//             achievements_unlocked_ids: game.achievements_unlocked_ids(),
//             achievements_locked_ids: game.achievements_locked_ids(),
//             achievements_by_unlocked_date: game.achievements_by_unlocked_date(),
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize)]
// pub struct ResponseGameData {
//     games: HashMap<u32, ResponseGame>,
//     game_ids: Vec<u32>,
//     games_by_last_played: Vec<u32>,
//     games_by_playtime: Vec<u32>,
//     games_by_percentage_achievements: Vec<u32>,
//     total_playtime: u32,
// }

// pub async fn get_all_games_data_query(State(state): State<AppState>) -> Json<ResponseGameData> {
//     let games = state
//         .games_repo()
//         .get_all_games()
//         .await
//         .iter()
//         .map(|(key, game)| (*key, game.clone().into()))
//         .collect::<HashMap<u32, ResponseGame>>();

//     let games_keys = games.keys().cloned().collect::<Vec<u32>>();

//     let games_by_last_played = state
//         .games_repo()
//         .get_games_by_most_recently_played()
//         .await
//         .iter()
//         .map(|game| game.id())
//         .collect::<Vec<u32>>();

//     let games_by_playtime = state
//         .games_repo()
//         .get_games_by_most_played()
//         .await
//         .iter()
//         .map(|game| game.id())
//         .collect::<Vec<u32>>();

//     let total_playtime = state.games_repo().get_total_play_time().await;

//     let games_by_percentage_achievements = state
//         .games_repo()
//         .get_games_by_most_completed_achievements()
//         .await
//         .iter()
//         .map(|game| game.id())
//         .collect::<Vec<u32>>();

//     Json(ResponseGameData {
//         games,
//         game_ids: games_keys,
//         games_by_last_played,
//         games_by_playtime,
//         games_by_percentage_achievements,
//         total_playtime,
//     })
// }
