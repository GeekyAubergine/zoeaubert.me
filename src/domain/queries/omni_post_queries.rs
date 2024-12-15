use std::collections::HashMap;

use bitflags::bitflags;
use chrono::Datelike;
use tracing::warn;

use crate::domain::models::content::Content;
use crate::domain::repositories::{
    AlbumsRepo, BlogPostsRepo, MastodonPostsRepo, MicroPostsRepo, OmniPostRepo,
    SteamAchievementsRepo, SteamGamesRepo,
};
use crate::domain::services::{MovieService, TvShowsService};
use crate::prelude::*;

use crate::domain::{models::omni_post::OmniPost, models::tag::Tag, state::State};

bitflags! {
    #[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
    pub struct OmniPostFilterFlags: u64 {
        const BLOG_POST = 0x1 << 0;
        const MICRO_POST = 0x1 << 1;
        const MASTODON_POST = 0x1 << 2;
        const ALBUM = 0x1 << 3;
        const ALBUM_PHOTO = 0x1 << 4;
        const UNLOCKED_STEAM_ACHIEVEMENT = 0x1 << 5;
        const MOVIE_REVIEW = 0x1 << 6;
        const TV_SHOW_REVIEW = 0x1 << 7;
    }
}

impl OmniPostFilterFlags {
    pub fn filter_all() -> OmniPostFilterFlags {
        OmniPostFilterFlags::all()
    }

    pub fn filter_main_timeline() -> OmniPostFilterFlags {
        OmniPostFilterFlags::BLOG_POST
            | OmniPostFilterFlags::MICRO_POST
            | OmniPostFilterFlags::MASTODON_POST
            | OmniPostFilterFlags::ALBUM
            | OmniPostFilterFlags::MOVIE_REVIEW
            | OmniPostFilterFlags::TV_SHOW_REVIEW
    }

    pub fn filter_photos_page() -> OmniPostFilterFlags {
        OmniPostFilterFlags::BLOG_POST
            | OmniPostFilterFlags::MICRO_POST
            | OmniPostFilterFlags::MASTODON_POST
            | OmniPostFilterFlags::ALBUM_PHOTO
    }

    pub fn filter_tags_page() -> OmniPostFilterFlags {
        OmniPostFilterFlags::BLOG_POST
            | OmniPostFilterFlags::MICRO_POST
            | OmniPostFilterFlags::MASTODON_POST
            | OmniPostFilterFlags::ALBUM_PHOTO
    }

    pub fn filter_firehose() -> OmniPostFilterFlags {
        Self::filter_all() - OmniPostFilterFlags::ALBUM_PHOTO
    }

    pub fn filter_game_activity() -> OmniPostFilterFlags {
        OmniPostFilterFlags::UNLOCKED_STEAM_ACHIEVEMENT
    }
}

pub async fn find_all_omni_posts(
    state: &impl State,
    filter_flags: OmniPostFilterFlags,
) -> Result<Vec<OmniPost>> {
    let posts = state.omni_post_repo().find_all_by_date().await?;

    let posts = posts
        .into_iter()
        .filter(|p| match p {
            OmniPost::BlogPost(_) => filter_flags.contains(OmniPostFilterFlags::BLOG_POST),
            OmniPost::MicroPost(_) => filter_flags.contains(OmniPostFilterFlags::MICRO_POST),
            OmniPost::MastodonPost(_) => filter_flags.contains(OmniPostFilterFlags::MASTODON_POST),
            OmniPost::Album(_) => filter_flags.contains(OmniPostFilterFlags::ALBUM),
            OmniPost::AlbumPhoto(_) => filter_flags.contains(OmniPostFilterFlags::ALBUM_PHOTO),
            OmniPost::SteamAcheivementUnlocked { .. } => {
                filter_flags.contains(OmniPostFilterFlags::UNLOCKED_STEAM_ACHIEVEMENT)
            }
            OmniPost::MovieReview(_) => filter_flags.contains(OmniPostFilterFlags::MOVIE_REVIEW),
            OmniPost::TvShowReview(_) => filter_flags.contains(OmniPostFilterFlags::TV_SHOW_REVIEW),
        })
        .collect::<Vec<OmniPost>>();

    // posts.sort_by(|a, b| b.date().cmp(&a.date()));

    Ok(posts)
}

pub async fn find_all_omni_posts_by_tag(state: &impl State, tag: &Tag) -> Result<Vec<OmniPost>> {
    state.omni_post_repo().find_all_by_tag(tag).await
}

pub async fn find_omni_posts_grouped_by_year(
    state: &impl State,
    filter_flags: OmniPostFilterFlags,
) -> Result<HashMap<u16, Vec<OmniPost>>> {
    let posts = find_all_omni_posts(state, filter_flags).await?;

    let years: HashMap<u16, Vec<OmniPost>> =
        posts.into_iter().fold(HashMap::new(), |mut acc, post| {
            acc.entry(post.date().year() as u16)
                .or_insert_with(Vec::new)
                .push(post);
            acc
        });

    Ok(years)
}
