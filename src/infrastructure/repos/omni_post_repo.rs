use std::collections::HashMap;

use crate::domain::models::omni_post::OmniPost;
use crate::domain::models::tag::Tag;
use crate::prelude::*;

use bitflags::bitflags;
use tracing::error;

use crate::infrastructure::app_state::{self, AppState};

bitflags! {
    #[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
    pub struct OmniPostFilterFlags: u64 {
        const STATUS_LOL = 0x1 << 0;
        const BLOG_POST = 0x1 << 1;
        const MICRO_POST = 0x1 << 2;
        const MICROBLOG_ARCHIVE_POST = 0x1 << 3;
        const MASTODON_POST = 0x1 << 4;
        const UNLOCKED_GAME_ACHIEVEMENT = 0x1 << 5;
        const ALBUM = 0x1 << 6;
        const ALBUM_PHOTO = 0x1 << 7;
    }
}

pub struct OmniPostRepo;

impl OmniPostRepo {
    pub fn filter_all() -> OmniPostFilterFlags {
        OmniPostFilterFlags::all()
    }

    pub fn filter_non_album() -> OmniPostFilterFlags {
        Self::filter_all() - OmniPostFilterFlags::ALBUM
    }

    pub fn filter_non_album_photo() -> OmniPostFilterFlags {
        Self::filter_all() - OmniPostFilterFlags::ALBUM_PHOTO
    }

    pub fn filter_non_album_photo_and_game_achievement() -> OmniPostFilterFlags {
        Self::filter_all()
            - OmniPostFilterFlags::ALBUM_PHOTO
            - OmniPostFilterFlags::UNLOCKED_GAME_ACHIEVEMENT
    }

    pub async fn get_posts_ordered_by_date(
        app_state: &AppState,
        filter: OmniPostFilterFlags,
    ) -> Result<Vec<OmniPost>> {
        let mut posts = vec![];

        if OmniPostFilterFlags::UNLOCKED_GAME_ACHIEVEMENT.intersects(filter) {
            let all_games = app_state.games_repo().find_all_games().await?;

            for game in all_games.into_iter() {
                let unlocked_achievements = app_state
                    .game_achievements_repo()
                    .find_all_unlocked_for_game_id(game.id())
                    .await?;

                for achievement in unlocked_achievements.into_iter() {
                    let post = OmniPost::UnlockedGameAchievement {
                        game: game.clone(),
                        achievement: achievement.clone(),
                    };
                    posts.push(post);
                }
            }
        }

        if OmniPostFilterFlags::STATUS_LOL.intersects(filter) {
            let status_lol_posts = app_state.status_lol_repo().find_all().await?;

            posts.extend(status_lol_posts.into_iter().map(OmniPost::StatusLol));
        }

        if OmniPostFilterFlags::BLOG_POST.intersects(filter) {
            posts.extend(
                app_state
                    .blog_posts_repo()
                    .get_all()
                    .await
                    .into_values()
                    .map(OmniPost::BlogPost)
                    .collect::<Vec<_>>(),
            );
        }

        if OmniPostFilterFlags::MICRO_POST.intersects(filter) {
            posts.extend(
                app_state
                    .micro_posts_repo()
                    .get_all()
                    .await
                    .into_values()
                    .map(OmniPost::MicroPost)
                    .collect::<Vec<_>>(),
            );
        }

        if OmniPostFilterFlags::MICROBLOG_ARCHIVE_POST.intersects(filter) {
            posts.extend(
                app_state
                    .microblog_archive_repo()
                    .get_all()
                    .await
                    .into_values()
                    .map(OmniPost::MicroblogArchivePost)
                    .collect::<Vec<_>>(),
            );
        }

        if OmniPostFilterFlags::MASTODON_POST.intersects(filter) {
            posts.extend(
                app_state
                    .mastodon_posts_repo()
                    .get_all()
                    .await
                    .into_values()
                    .map(OmniPost::MastodonPost)
                    .collect::<Vec<_>>(),
            );
        }

        posts.sort_by(|a, b| b.date().cmp(a.date()));

        Ok(posts)
    }

    pub async fn get_posts_tags_and_counts(
        app_state: &AppState,
        filter: OmniPostFilterFlags,
    ) -> Result<HashMap<Tag, usize>> {
        let mut tags = HashMap::new();

        let posts = Self::get_posts_ordered_by_date(app_state, filter).await?;

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
        filter: OmniPostFilterFlags,
    ) -> Result<Vec<OmniPost>> {
        let posts = Self::get_posts_ordered_by_date(app_state, filter).await?;

        let mut posts = posts
            .into_iter()
            .filter(|post| post.tags().iter().any(|t| t == tag))
            .collect::<Vec<_>>();

        posts.sort_by(|a, b| b.date().cmp(a.date()));

        Ok(posts)
    }
}
