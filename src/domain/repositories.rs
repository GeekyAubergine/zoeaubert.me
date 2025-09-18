use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::Duration;

use crate::{
    domain::models::{albums::{album::Album, album_photo::AlbumPhoto}, projects::Project},
    prelude::*,
};

use super::{
    models::{
        blog_post::BlogPost,
        league::LeagueChampNote,
        lego::{LegoMinifig, LegoSet},
        mastodon_post::MastodonPost,
        micro_post::MicroPost,
        movie::{MovieId, MovieReview},
        omni_post::Post,
        raw_content::SourcePost,
        referral::Referral,
        slug::Slug,
        steam::{
            SteamGame, SteamGameAchievement, SteamGameAchievementLocked,
            SteamGameAchievementUnlocked,
        },
        tag::Tag,
        tv_show::{TvShowId, TvShowReview},
    },
    services::FileService,
    state::State,
};

#[async_trait::async_trait]
pub trait Profiler {
    async fn entity_processing_started(&self) -> Result<()>;

    async fn entity_processed(&self) -> Result<()>;

    async fn entity_processing_finished(&self) -> Result<()>;

    async fn set_page_generation_duration(&self, duration: Duration) -> Result<()>;

    async fn set_page_rendering_duration(&self, duration: Duration) -> Result<()>;

    async fn set_page_write_duration(&self, duration: Duration) -> Result<()>;

    async fn set_number_of_pages_written(&self, number_of_pages: u32) -> Result<()>;

    async fn print_results(&self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait SillyNamesRepo {
    async fn find_all(&self) -> Result<Vec<String>>;

    async fn commit(&self, names: Vec<String>) -> Result<()>;
}

#[async_trait::async_trait]
pub trait AboutTextRepo {
    async fn find_short(&self) -> Result<String>;

    async fn find_long(&self) -> Result<String>;

    async fn commit(&self, short: String, long: String) -> Result<()>;
}

#[async_trait::async_trait]
pub trait BlogPostsRepo {
    async fn find_all_by_date(&self) -> Result<Vec<BlogPost>>;

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<BlogPost>>;

    async fn commit(&self, blog_post: &BlogPost) -> Result<()>;
}

#[async_trait::async_trait]
pub trait MicroPostsRepo {
    async fn find_all(&self) -> Result<Vec<MicroPost>>;

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<MicroPost>>;

    async fn commit(&self, micro_post: &MicroPost) -> Result<()>;
}

#[async_trait::async_trait]
pub trait MastodonPostsRepo {
    async fn find_all_by_date(&self) -> Result<Vec<MastodonPost>>;

    async fn find_last_updated_at(&self) -> Result<Option<DateTime<Utc>>>;

    async fn commit(&self, micro_post: &MastodonPost) -> Result<()>;
}

#[async_trait::async_trait]
pub trait LegoRepo {
    async fn find_all_sets(&self) -> Result<Vec<LegoSet>>;

    async fn find_all_minifigs(&self) -> Result<Vec<LegoMinifig>>;

    async fn find_total_pieces(&self) -> Result<u32>;

    async fn find_total_sets(&self) -> Result<u32>;

    async fn find_total_minifigs(&self) -> Result<u32>;

    async fn find_last_updated_at(&self) -> Result<Option<DateTime<Utc>>>;

    async fn commit_set(&self, set: &LegoSet) -> Result<()>;

    async fn commit_minifig(&self, minifig: &LegoMinifig) -> Result<()>;
}

#[async_trait::async_trait]
pub trait SteamGamesRepo {
    async fn find_by_id(&self, game_id: u32) -> Result<Option<SteamGame>>;

    async fn find_all(&self) -> Result<Vec<SteamGame>>;

    async fn find_total_playtime(&self) -> Result<u32>;

    async fn find_total_games(&self) -> Result<u32>;

    async fn find_most_recently_updated_at(&self) -> Result<Option<DateTime<Utc>>>;

    async fn commit(&self, game: &SteamGame) -> Result<()>;
}

#[async_trait::async_trait]
pub trait SteamAchievementsRepo {
    async fn find_all_unlocked_by_unlocked_date(
        &self,
        game_id: u32,
    ) -> Result<Vec<SteamGameAchievementUnlocked>>;

    async fn find_all_locked_by_name(
        &self,
        game_id: u32,
    ) -> Result<Vec<SteamGameAchievementLocked>>;

    async fn commit(&self, game: &SteamGame, achievement: &SteamGameAchievement) -> Result<()>;
}

#[async_trait::async_trait]
pub trait MovieReviewsRepo {
    async fn find_by_movie_id(&self, movie_id: &MovieId) -> Result<Vec<MovieReview>>;

    async fn find_all_grouped_by_movie_id(&self) -> Result<HashMap<MovieId, Vec<MovieReview>>>;

    async fn commit(&self, movie_review: &MovieReview) -> Result<()>;
}

#[async_trait::async_trait]
pub trait TvShowReviewsRepo {
    async fn find_by_tv_show_id(&self, tv_show_id: &TvShowId) -> Result<Vec<TvShowReview>>;

    async fn find_all_grouped_by_tv_show_id(&self) -> Result<HashMap<TvShowId, Vec<TvShowReview>>>;

    async fn commit(&self, tv_show_review: &TvShowReview) -> Result<()>;
}

#[async_trait::async_trait]
pub trait AlbumsRepo {
    async fn find_all_by_date(&self) -> Result<Vec<Album>>;

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Album>>;

    async fn find_grouped_by_year(&self) -> Result<Vec<(u16, Vec<Album>)>>;

    async fn find_all_album_photos(&self) -> Result<Vec<AlbumPhoto>>;

    async fn commit(&self, album: &Album) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ReferralsRepo {
    async fn find_all(&self) -> Result<Vec<Referral>>;
    async fn commit(&self, referrals: Vec<Referral>) -> Result<()>;
}

#[async_trait::async_trait]
pub trait FaqRepo {
    async fn find(&self) -> Result<String>;

    async fn commit(&self, faq: String) -> Result<()>;
}

#[async_trait::async_trait]
pub trait NowTextRepo {
    async fn find(&self) -> Result<String>;

    async fn commit(&self, now_text: String) -> Result<()>;
}

#[async_trait::async_trait]
pub trait LeagueRepo {
    async fn find_all_champ_notes_by_name(&self) -> Result<Vec<LeagueChampNote>>;

    async fn commit_champ_notes(&self, notes: Vec<LeagueChampNote>) -> Result<()>;
}

#[async_trait::async_trait]
pub trait OmniPostRepo {
    async fn find_all_by_date(&self) -> Result<Vec<Post>>;

    async fn find_all_by_tag(&self, tag: &Tag) -> Result<Vec<Post>>;

    async fn commit(&self, state: &impl State, posts: Vec<Post>) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ProjectsRepo {
    async fn find_all_by_rank_and_name(&self) -> Result<Vec<Project>>;

    async fn find_by_name(&self, name: &String) -> Result<Option<Project>>;

    async fn commit(&self, project: &Project) -> Result<()>;
}
