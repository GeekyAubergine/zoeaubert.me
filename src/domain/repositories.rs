use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::prelude::*;

use super::{
    models::{
        album::Album,
        blog_post::BlogPost,
        games::{Game, GameAchievement, GameAchievementLocked, GameAchievementUnlocked},
        lego::{LegoMinifig, LegoSet},
        mastodon_post::MastodonPost,
        micro_post::MicroPost,
        movie::{MovieId, MovieReview},
        slug::Slug,
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

    async fn page_generation_started(&self) -> Result<()>;

    async fn page_generated(&self) -> Result<()>;

    async fn page_generation_finished(&self) -> Result<()>;

    async fn queue_processing_started(&self) -> Result<()>;

    async fn queue_processed(&self) -> Result<()>;

    async fn queue_processing_finished(&self) -> Result<()>;

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
pub trait GamesRepo {
    async fn find_by_game_id(&self, game_id: u32) -> Result<Option<Game>>;

    async fn find_all_games(&self) -> Result<Vec<Game>>;

    async fn find_total_playtime(&self) -> Result<u32>;

    async fn find_total_games(&self) -> Result<u32>;

    async fn find_most_recently_updated_at(&self) -> Result<Option<DateTime<Utc>>>;

    async fn commit(&self, game: &Game) -> Result<()>;
}

#[async_trait::async_trait]
pub trait GameAchievementsRepo {
    async fn find_all_unlocked_by_unlocked_date(
        &self,
        game_id: u32,
    ) -> Result<Vec<GameAchievementUnlocked>>;

    async fn find_all_locked_by_name(&self, game_id: u32) -> Result<Vec<GameAchievementLocked>>;

    async fn commit(&self, game: &Game, achievement: &GameAchievement) -> Result<()>;
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

    async fn commit(&self, album: &Album) -> Result<()>;
}
