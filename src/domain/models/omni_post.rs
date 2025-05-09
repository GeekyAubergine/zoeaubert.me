use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    album::{Album, AlbumPhoto},
    blog_post::BlogPost,
    book::BookReview,
    raw_content::RawContent,
    image::Image,
    mastodon_post::MastodonPost,
    media::Media,
    micro_post::MicroPost,
    movie::{Movie, MovieReview},
    page::Page,
    slug::Slug,
    steam::{SteamGame, SteamGameAchievementUnlocked},
    tag::Tag,
    tv_show::TvShowReview,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OmniPost {
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

impl OmniPost {
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

    pub fn last_updated_at(&self) -> Option<&DateTime<Utc>> {
        match self {
            Self::BlogPost(blog_post) => Some(&blog_post.updated_at),
            Self::MicroPost(micro_post) => micro_post.updated_at.as_ref(),
            Self::MastodonPost(mastodon_post) => Some(mastodon_post.updated_at()),
            Self::AlbumPhoto { photo, .. } => Some(&photo.updated_at),
            Self::Album(album) => Some(&album.updated_at),
            Self::SteamAcheivementUnlocked { achievement, .. } => Some(&achievement.unlocked_date),
            Self::MovieReview(review) => review.source_content.last_updated_at(),
            Self::TvShowReview(review) => review.source_content.last_updated_at(),
            Self::BookReview(review) => review.source_content.last_updated_at(),
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

impl From<RawContent> for OmniPost {
    fn from(content: RawContent) -> Self {
        match content {
            RawContent::BlogPost(blog_post) => Self::BlogPost(blog_post),
            RawContent::MicroPost(micro_post) => Self::MicroPost(micro_post),
            RawContent::MastodonPost(mastodon_post) => Self::MastodonPost(mastodon_post),
            RawContent::AlbumPhoto { photo, album } => Self::AlbumPhoto { album, photo },
            RawContent::Album(album) => Self::Album(album),
        }
    }
}

impl From<BlogPost> for OmniPost {
    fn from(blog_post: BlogPost) -> Self {
        Self::BlogPost(blog_post)
    }
}

impl From<MicroPost> for OmniPost {
    fn from(micro_post: MicroPost) -> Self {
        Self::MicroPost(micro_post)
    }
}

impl From<MastodonPost> for OmniPost {
    fn from(mastodon_post: MastodonPost) -> Self {
        Self::MastodonPost(mastodon_post)
    }
}

// impl From<AlbumPhoto> for OmniPost {
//     fn from(album_photo: AlbumPhoto) -> Self {
//         Self::AlbumPhoto(album_photo)
//     }
// }

impl From<Album> for OmniPost {
    fn from(album: Album) -> Self {
        Self::Album(album)
    }
}

impl From<(SteamGame, SteamGameAchievementUnlocked)> for OmniPost {
    fn from((game, achievement): (SteamGame, SteamGameAchievementUnlocked)) -> Self {
        Self::SteamAcheivementUnlocked { game, achievement }
    }
}

impl From<MovieReview> for OmniPost {
    fn from(movie: MovieReview) -> Self {
        Self::MovieReview(movie)
    }
}

impl From<TvShowReview> for OmniPost {
    fn from(tv_show: TvShowReview) -> Self {
        Self::TvShowReview(tv_show)
    }
}

impl From<BookReview> for OmniPost {
    fn from(book: BookReview) -> Self {
        Self::BookReview(book)
    }
}
