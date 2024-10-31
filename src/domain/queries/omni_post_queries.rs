use bitflags::bitflags;

use crate::domain::repositories::{AlbumsRepo, BlogPostsRepo, MastodonPostsRepo, MicroPostsRepo};
use crate::prelude::*;

use crate::domain::{models::omni_post::OmniPost, models::tag::Tag, state::State};

bitflags! {
    #[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
    pub struct OmniPostFilterFlags: u64 {
        const STATUS_LOL = 0x1 << 0;
        const BLOG_POST = 0x1 << 1;
        const MICRO_POST = 0x1 << 2;
        const MASTODON_POST = 0x1 << 4;
        const UNLOCKED_GAME_ACHIEVEMENT = 0x1 << 5;
        const ALBUM = 0x1 << 6;
        const ALBUM_PHOTO = 0x1 << 7;
    }
}

impl OmniPostFilterFlags {
    pub fn filter_all() -> OmniPostFilterFlags {
        OmniPostFilterFlags::all()
    }

    pub fn filter_main_timeline() -> OmniPostFilterFlags {
        Self::filter_all()
            - OmniPostFilterFlags::ALBUM_PHOTO
            - OmniPostFilterFlags::UNLOCKED_GAME_ACHIEVEMENT
    }

    pub fn filter_photos_page() -> OmniPostFilterFlags {
        Self::filter_all() - OmniPostFilterFlags::ALBUM
    }

    pub fn filter_tags_page() -> OmniPostFilterFlags {
        Self::filter_all()
            - OmniPostFilterFlags::ALBUM
            - OmniPostFilterFlags::UNLOCKED_GAME_ACHIEVEMENT
    }
}

pub async fn find_all_omni_posts(
    state: &impl State,
    filter_flags: OmniPostFilterFlags,
) -> Result<Vec<OmniPost>> {
    let mut omni_posts = Vec::new();

    if filter_flags.contains(OmniPostFilterFlags::BLOG_POST) {
        let blog_posts = state
            .blog_posts_repo()
            .find_all_by_date()
            .await?
            .iter()
            .map(|p| p.into())
            .collect::<Vec<OmniPost>>();

        omni_posts.extend(blog_posts);
    }

    if filter_flags.contains(OmniPostFilterFlags::MICRO_POST) {
        let micro_posts = state
            .micro_posts_repo()
            .find_all()
            .await?
            .iter()
            .map(|p| p.into())
            .collect::<Vec<OmniPost>>();

        omni_posts.extend(micro_posts);
    }

    if filter_flags.contains(OmniPostFilterFlags::MASTODON_POST) {
        let mastodon_posts = state
            .mastodon_posts_repo()
            .find_all_by_date()
            .await?
            .iter()
            .map(|p| p.into())
            .collect::<Vec<OmniPost>>();

        omni_posts.extend(mastodon_posts);
    }

    if filter_flags.contains(OmniPostFilterFlags::ALBUM_PHOTO) {
        let album_photos = state
            .albums_repo()
            .find_all_album_photos()
            .await?
            .iter()
            .map(|p| p.into())
            .collect::<Vec<OmniPost>>();

        omni_posts.extend(album_photos);
    }

    if filter_flags.contains(OmniPostFilterFlags::ALBUM) {
        let albums = state
            .albums_repo()
            .find_all_by_date()
            .await?
            .iter()
            .map(|p| p.into())
            .collect::<Vec<OmniPost>>();

        omni_posts.extend(albums);
    }

    omni_posts.sort_by(|a, b| b.date().cmp(&a.date()));

    Ok(omni_posts)
}

pub async fn find_all_omni_posts_by_tag(state: &impl State, tag: &Tag) -> Result<Vec<OmniPost>> {
    let posts = find_all_omni_posts(state, OmniPostFilterFlags::filter_all()).await?;

    let filtered_posts = posts
        .iter()
        .filter(|p| p.tags().contains(tag))
        .cloned()
        .collect::<Vec<OmniPost>>();

    Ok(filtered_posts)
}
