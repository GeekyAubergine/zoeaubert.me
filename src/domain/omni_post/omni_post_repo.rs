use std::collections::HashMap;

use crate::domain::models::tag::Tag;
use crate::prelude::*;

use crate::infrastructure::app_state::{self, AppState};

use super::omni_post_models::OmniPost;

pub struct OmniPostRepo;

impl OmniPostRepo {
    pub async fn get_posts_ordered_by_date(app_state: &AppState) -> Result<Vec<OmniPost>> {
        let mut posts = vec![];

        // Games
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

        // StatusLol
        posts.extend(
            app_state
                .status_lol_repo()
                .get_all()
                .await
                .into_iter()
                .map(OmniPost::StatusLol)
                .collect::<Vec<_>>(),
        );

        // Blog Posts
        posts.extend(
            app_state
                .blog_posts_repo()
                .get_all()
                .await
                .into_values()
                .map(OmniPost::BlogPost)
                .collect::<Vec<_>>(),
        );

        posts.sort_by(|a, b| b.date().cmp(a.date()));

        Ok(posts)
    }

    pub async fn get_posts_tags_and_counts(app_state: &AppState) -> Result<HashMap<Tag, usize>> {
        let mut tags = HashMap::new();

        let posts = Self::get_posts_ordered_by_date(app_state).await?;

        for post in posts {
            for tag in post.tags() {
                let count = tags.entry(tag).or_insert(0);
                *count += 1;
            }
        }

        Ok(tags)
    }

    pub async fn get_posts_by_tag_ordered_by_date(
        app_state: &AppState,
        tag: &Tag,
    ) -> Result<Vec<OmniPost>> {
        let posts = Self::get_posts_ordered_by_date(app_state).await?;

        let mut posts = posts
            .into_iter()
            .filter(|post| post.tags().iter().any(|t| t == tag))
            .collect::<Vec<_>>();

        posts.sort_by(|a, b| b.date().cmp(a.date()));

        Ok(posts)
    }
}
