use std::collections::HashMap;

use bitflags::bitflags;
use chrono::Datelike;

use crate::domain::repositories::{AboutTextRepo, OmniPostRepo};
use crate::prelude::*;

use crate::domain::state::State;

use super::about_text::AboutText;
use super::blog_post::BlogPost;
use super::faq::Faq;
use super::now_text::NowText;
use super::omni_post::OmniPost;
use super::referral::Referrals;
use super::silly_names::SillyNames;
use super::tag::Tag;

bitflags! {
    #[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
    pub struct PostFilter: u64 {
        const BLOG_POST = 0x1 << 0;
        const MICRO_POST = 0x1 << 1;
        const MASTODON_POST = 0x1 << 2;
        const ALBUM = 0x1 << 3;
        const ALBUM_PHOTO = 0x1 << 4;
        const UNLOCKED_STEAM_ACHIEVEMENT = 0x1 << 5;
        const MOVIE_REVIEW = 0x1 << 6;
        const TV_SHOW_REVIEW = 0x1 << 7;
        const BOOK_REVIEW = 0x1 << 8;
    }
}

impl PostFilter {
    pub fn filter_all() -> PostFilter {
        PostFilter::all()
    }

    pub fn filter_main_timeline() -> PostFilter {
        PostFilter::BLOG_POST
            | PostFilter::MICRO_POST
            | PostFilter::MASTODON_POST
            | PostFilter::ALBUM
            | PostFilter::MOVIE_REVIEW
            | PostFilter::TV_SHOW_REVIEW
            | PostFilter::BOOK_REVIEW
    }

    pub fn filter_photos_page() -> PostFilter {
        PostFilter::MICRO_POST | PostFilter::MASTODON_POST | PostFilter::ALBUM_PHOTO
    }

    pub fn filter_tags_page() -> PostFilter {
        PostFilter::BLOG_POST
            | PostFilter::MICRO_POST
            | PostFilter::MASTODON_POST
            | PostFilter::ALBUM_PHOTO
            | PostFilter::MOVIE_REVIEW
            | PostFilter::TV_SHOW_REVIEW
            | PostFilter::BOOK_REVIEW
    }

    pub fn filter_firehose() -> PostFilter {
        Self::filter_all() - PostFilter::ALBUM_PHOTO
    }

    pub fn filter_game_activity() -> PostFilter {
        PostFilter::UNLOCKED_STEAM_ACHIEVEMENT
    }

    pub fn filter_home_page() -> PostFilter {
        PostFilter::MICRO_POST
            | PostFilter::MASTODON_POST
            | PostFilter::ALBUM
            | PostFilter::MOVIE_REVIEW
            | PostFilter::TV_SHOW_REVIEW
            | PostFilter::BOOK_REVIEW
    }
}

pub struct Posts {
    posts: HashMap<String, OmniPost>,
    post_date_order: Vec<String>,
    posts_by_tag: HashMap<Tag, Vec<String>>,
}

impl Posts {
    pub fn new() -> Self {
        Self {
            posts: HashMap::new(),
            post_date_order: Vec::new(),
            posts_by_tag: HashMap::new(),
        }
    }

    pub fn update_internal_state(&mut self) {
        let mut posts = self.posts.values().cloned().collect::<Vec<OmniPost>>();

        posts.sort_by(|a, b| b.date().cmp(&a.date()));

        self.post_date_order = posts.iter().map(|p| p.key()).collect();

        for post in posts {
            for tag in post.tags() {
                self.posts_by_tag
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(post.key());
            }
        }
    }

    pub fn find_all_by_date(&self) -> Vec<&OmniPost> {
        self.post_date_order
            .iter()
            .filter_map(|slug| self.posts.get(slug))
            .collect::<Vec<&OmniPost>>()
    }

    pub fn find_all_by_tag(&self, tag: &Tag) -> Vec<&OmniPost> {
        self.posts_by_tag
            .get(tag)
            .map(|slugs| {
                slugs
                    .iter()
                    .filter_map(|slug| self.posts.get(slug))
                    .collect::<Vec<&OmniPost>>()
            })
            .unwrap_or_default()
    }

    pub fn find_all_by_filter(&self, filter_flags: PostFilter) -> Vec<&OmniPost> {
        self.find_all_by_date()
            .into_iter()
            .filter(|p| match p {
                OmniPost::BlogPost(_) => filter_flags.contains(PostFilter::BLOG_POST),
                OmniPost::MicroPost(_) => filter_flags.contains(PostFilter::MICRO_POST),
                OmniPost::MastodonPost(_) => filter_flags.contains(PostFilter::MASTODON_POST),
                OmniPost::Album(_) => filter_flags.contains(PostFilter::ALBUM),
                OmniPost::AlbumPhoto { .. } => filter_flags.contains(PostFilter::ALBUM_PHOTO),
                OmniPost::SteamAcheivementUnlocked { .. } => {
                    filter_flags.contains(PostFilter::UNLOCKED_STEAM_ACHIEVEMENT)
                }
                OmniPost::MovieReview(_) => filter_flags.contains(PostFilter::MOVIE_REVIEW),
                OmniPost::TvShowReview(_) => filter_flags.contains(PostFilter::TV_SHOW_REVIEW),
                OmniPost::BookReview(_) => filter_flags.contains(PostFilter::BOOK_REVIEW),
            })
            .collect::<Vec<&OmniPost>>()
    }

    pub fn find_all_by_year_and_grouped_by_year(
        &self,
        filter: PostFilter,
    ) -> HashMap<u16, Vec<&OmniPost>> {
        let posts = self.find_all_by_filter(filter);

        let years: HashMap<u16, Vec<&OmniPost>> =
            posts.into_iter().fold(HashMap::new(), |mut acc, post| {
                acc.entry(post.date().year() as u16)
                    .or_insert_with(Vec::new)
                    .push(post);
                acc
            });

        years
    }

    pub fn add_posts(&mut self, posts: Vec<OmniPost>) {
        for post in posts {
            self.posts.insert(post.key(), post);
        }

        self.update_internal_state();
    }
}

pub struct Data {
    pub about_text: AboutText,
    pub silly_names: SillyNames,
    pub faq: Faq,
    pub referrals: Referrals,
    pub now_text: NowText,
    pub posts: Posts,
}

impl Data {
    pub async fn from_state(state: &impl State) -> Result<Data> {
        let mut posts = Posts::new();

        posts.add_posts(state.omni_post_repo().find_all_by_date().await?);

        Ok(Data {
            about_text: AboutText::from_state(state).await?,
            silly_names: SillyNames::from_state(state).await?,
            faq: Faq::from_state(state).await?,
            referrals: Referrals::from_state(state).await?,
            now_text: NowText::from_state(state).await?,
            posts,
        })
    }
}
