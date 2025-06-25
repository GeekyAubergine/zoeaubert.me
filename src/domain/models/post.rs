use std::collections::HashMap;

use bitflags::bitflags;
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::models::albums::album::Album;
use crate::domain::models::albums::album_photo::AlbumPhoto;
use crate::domain::models::book::BookReview;
use crate::domain::models::games::steam::{SteamGame, SteamGameAchievementUnlocked};
use crate::domain::models::image::Image;
use crate::domain::models::mastodon_post::MastodonPost;
use crate::domain::models::media::Media;
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::movie::MovieReview;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::source_post::SourcePost;
use crate::domain::models::tv_show::TvShowReview;
use crate::prelude::*;

use super::about_text::AboutText;
use super::blog_post::BlogPost;
use super::faq::Faq;
use super::now_text::NowText;
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Post {
    BlogPost(BlogPost),
    MicroPost(MicroPost),
    MastodonPost(MastodonPost),
    AlbumPhoto {
        album: Album,
        photo: AlbumPhoto,
    },
    Album(Album),
    SteamAcheivementUnlocked {
        game: SteamGame,
        achievement: SteamGameAchievementUnlocked,
    },
    MovieReview(MovieReview),
    TvShowReview(TvShowReview),
    BookReview(BookReview),
}

impl Post {
    pub fn key(&self) -> String {
        match self {
            Self::BlogPost(blog_post) => blog_post.slug.to_string(),
            Self::MicroPost(micro_post) => micro_post.slug.to_string(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug().to_string(),
            Self::AlbumPhoto { photo, .. } => photo.slug.to_string(),
            Self::Album(album) => album.slug.to_string(),
            Self::SteamAcheivementUnlocked { game, achievement } => {
                format!("{}-{}", game.id, achievement.id)
            }
            Self::MovieReview(review) => review.source_content.slug().to_string(),
            Self::TvShowReview(review) => review.source_content.slug().to_string(),
            Self::BookReview(review) => review.source_content.slug().to_string(),
        }
    }

    pub fn slug(&self) -> Slug {
        match self {
            Self::BlogPost(blog_post) => blog_post.slug.clone(),
            Self::MicroPost(micro_post) => micro_post.slug.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug(),
            Self::AlbumPhoto { photo, .. } => photo.slug.clone(),
            Self::Album(album) => album.slug.clone(),
            Self::SteamAcheivementUnlocked { game, .. } => game.slug(),
            Self::MovieReview(review) => review.source_content.slug(),
            Self::TvShowReview(review) => review.source_content.slug(),
            Self::BookReview(review) => review.source_content.slug(),
        }
    }

    pub fn link(&self) -> String {
        match self {
            Self::BlogPost(blog_post) => blog_post.slug.relative_link(),
            Self::MicroPost(micro_post) => micro_post.slug.relative_link(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug().relative_link(),
            Self::AlbumPhoto { photo, .. } => photo.slug.relative_link(),
            Self::Album(album) => album.slug.relative_link(),
            Self::SteamAcheivementUnlocked { game, .. } => game.slug().relative_link(),
            Self::MovieReview(review) => review.source_content.slug().relative_link(),
            Self::TvShowReview(review) => review.source_content.slug().relative_link(),
            Self::BookReview(review) => review.source_content.slug().relative_link(),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            Self::BlogPost(blog_post) => &blog_post.date,
            Self::MicroPost(micro_post) => &micro_post.date,
            Self::MastodonPost(mastodon_post) => mastodon_post.created_at(),
            Self::AlbumPhoto { photo, .. } => &photo.date,
            Self::Album(album) => &album.date,
            Self::SteamAcheivementUnlocked { achievement, .. } => &achievement.unlocked_date,
            Self::MovieReview(review) => review.source_content.date(),
            Self::TvShowReview(review) => review.source_content.date(),
            Self::BookReview(review) => review.source_content.date(),
        }
    }

    pub fn media(&self) -> Vec<Media> {
        match self {
            Self::BlogPost(blog_post) => blog_post.media.clone(),
            Self::MicroPost(micro_post) => micro_post.media.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.media(),
            Self::AlbumPhoto { photo, .. } => {
                vec![photo.small_image.clone().into()]
            }
            Self::Album(_) => vec![], // It does it's own thing
            Self::SteamAcheivementUnlocked { .. } => vec![], // Don't want this showing up in photos
            Self::MovieReview(_) => vec![], // Don't want this showing up in photos
            Self::TvShowReview(_) => vec![], // Don't want this showing up in photos
            Self::BookReview(review) => vec![],
        }
    }

    pub fn optimised_media(&self) -> Vec<Media> {
        match self {
            Self::BlogPost(blog_post) => blog_post.media.clone(),
            Self::MicroPost(micro_post) => micro_post.media.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.optimised_media(),
            Self::AlbumPhoto { photo, .. } => {
                vec![photo.small_image.clone().into()]
            }
            Self::Album(_) => vec![], // It does it's own thing
            Self::SteamAcheivementUnlocked { .. } => vec![], // Don't want this showing up in photos
            Self::MovieReview(_) => vec![], // Don't want this showing up in photos
            Self::TvShowReview(_) => vec![], // Don't want this showing up in photos
            Self::BookReview(review) => vec![],
        }
    }

    pub fn tags(&self) -> Vec<Tag> {
        match self {
            Self::BlogPost(blog_post) => blog_post.tags.clone(),
            Self::MicroPost(micro_post) => micro_post.tags.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.tags().clone(),
            Self::AlbumPhoto { photo, .. } => photo.tags.clone(),
            Self::Album(_) => vec![], // Don't want it in search
            Self::SteamAcheivementUnlocked { .. } => vec![], // Doesn't have tags
            Self::MovieReview(review) => review.source_content.tags(),
            Self::TvShowReview(review) => review.source_content.tags(),
            Self::BookReview(review) => review.source_content.tags(),
        }
    }

    pub fn side_image(&self) -> Option<Image> {
        match self {
            Self::BlogPost(blog_post) => None,
            Self::MicroPost(micro_post) => None,
            Self::MastodonPost(mastodon_post) => None,
            Self::AlbumPhoto { .. } => None,
            Self::Album(_) => None,
            Self::SteamAcheivementUnlocked { game, .. } => Some(game.header_image.clone()),
            Self::MovieReview(review) => Some(review.movie.poster.clone()),
            Self::TvShowReview(review) => Some(review.tv_show.poster.clone()),
            Self::BookReview(review) => Some(review.book.cover.clone()),
        }
    }

    pub fn page(&self) -> Option<Page> {
        match self {
            Self::BlogPost(blog_post) => Some(blog_post.page()),
            Self::MicroPost(micro_post) => Some(micro_post.page()),
            Self::MastodonPost(mastodon_post) => Some(mastodon_post.page()),
            Self::AlbumPhoto { photo, .. } => Some(photo.page()),
            Self::Album(album) => Some(album.page()),
            Self::SteamAcheivementUnlocked { game, .. } => None,
            Self::MovieReview(review) => Some(review.source_content.page()),
            Self::TvShowReview(review) => Some(review.source_content.page()),
            Self::BookReview(review) => Some(review.source_content.page()),
        }
    }
}

impl From<SourcePost> for Post {
    fn from(content: SourcePost) -> Self {
        match content {
            SourcePost::BlogPost(blog_post) => Self::BlogPost(blog_post),
            SourcePost::MicroPost(micro_post) => Self::MicroPost(micro_post),
            SourcePost::MastodonPost(mastodon_post) => Self::MastodonPost(mastodon_post),
        }
    }
}

impl From<BlogPost> for Post {
    fn from(blog_post: BlogPost) -> Self {
        Self::BlogPost(blog_post)
    }
}

impl From<MicroPost> for Post {
    fn from(micro_post: MicroPost) -> Self {
        Self::MicroPost(micro_post)
    }
}

impl From<MastodonPost> for Post {
    fn from(mastodon_post: MastodonPost) -> Self {
        Self::MastodonPost(mastodon_post)
    }
}

// impl From<AlbumPhoto> for OmniPost {
//     fn from(album_photo: AlbumPhoto) -> Self {
//         Self::AlbumPhoto(album_photo)
//     }
// }

impl From<Album> for Post {
    fn from(album: Album) -> Self {
        Self::Album(album)
    }
}

impl From<(SteamGame, SteamGameAchievementUnlocked)> for Post {
    fn from((game, achievement): (SteamGame, SteamGameAchievementUnlocked)) -> Self {
        Self::SteamAcheivementUnlocked { game, achievement }
    }
}

impl From<MovieReview> for Post {
    fn from(movie: MovieReview) -> Self {
        Self::MovieReview(movie)
    }
}

impl From<TvShowReview> for Post {
    fn from(tv_show: TvShowReview) -> Self {
        Self::TvShowReview(tv_show)
    }
}

impl From<BookReview> for Post {
    fn from(book: BookReview) -> Self {
        Self::BookReview(book)
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
    posts: HashMap<String, Post>,
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

    pub fn from_posts(posts: Vec<Post>) -> Self {
        let mut p = Self::new();
        p.add_posts(posts);
        p
    }

    pub fn update_internal_state(&mut self) {
        let mut posts = self.posts.values().cloned().collect::<Vec<Post>>();

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

    pub fn find_all_by_date(&self) -> Vec<&Post> {
        self.post_date_order
            .iter()
            .filter_map(|slug| self.posts.get(slug))
            .collect::<Vec<&Post>>()
    }

    pub fn find_all_by_tag(&self, tag: &Tag) -> Vec<&Post> {
        self.posts_by_tag
            .get(tag)
            .map(|slugs| {
                slugs
                    .iter()
                    .filter_map(|slug| self.posts.get(slug))
                    .collect::<Vec<&Post>>()
            })
            .unwrap_or_default()
    }

    pub fn find_all_by_filter(&self, filter_flags: PostFilter) -> Vec<&Post> {
        self.find_all_by_date()
            .into_iter()
            .filter(|p| match p {
                Post::BlogPost(_) => filter_flags.contains(PostFilter::BLOG_POST),
                Post::MicroPost(_) => filter_flags.contains(PostFilter::MICRO_POST),
                Post::MastodonPost(_) => filter_flags.contains(PostFilter::MASTODON_POST),
                Post::Album(_) => filter_flags.contains(PostFilter::ALBUM),
                Post::AlbumPhoto { .. } => filter_flags.contains(PostFilter::ALBUM_PHOTO),
                Post::SteamAcheivementUnlocked { .. } => {
                    filter_flags.contains(PostFilter::UNLOCKED_STEAM_ACHIEVEMENT)
                }
                Post::MovieReview(_) => filter_flags.contains(PostFilter::MOVIE_REVIEW),
                Post::TvShowReview(_) => filter_flags.contains(PostFilter::TV_SHOW_REVIEW),
                Post::BookReview(_) => filter_flags.contains(PostFilter::BOOK_REVIEW),
            })
            .collect::<Vec<&Post>>()
    }

    pub fn find_all_by_year_and_grouped_by_year(
        &self,
        filter: PostFilter,
    ) -> HashMap<u16, Vec<&Post>> {
        let posts = self.find_all_by_filter(filter);

        let years: HashMap<u16, Vec<&Post>> =
            posts.into_iter().fold(HashMap::new(), |mut acc, post| {
                acc.entry(post.date().year() as u16)
                    .or_insert_with(Vec::new)
                    .push(post);
                acc
            });

        years
    }

    pub fn add_posts(&mut self, posts: Vec<Post>) {
        for post in posts {
            self.posts.insert(post.key(), post);
        }

        self.update_internal_state();
    }
}
