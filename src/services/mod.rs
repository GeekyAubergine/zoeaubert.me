use std::sync::Arc;

use crate::{
    prelude::*,
    services::{
        book_service::BookService, cdn_service::CdnService,
        data_hash_cache_service::ContentHashService, file_service::FileService,
        movie_service::MovieService, network_service::NetworkService,
        query_limiter_service::QueryLimitingService, ts_show_service::TvShowService,
    },
};

pub mod book_service;
pub mod cdn_service;
pub mod data_hash_cache_service;
pub mod file_service;
pub mod media_service;
pub mod movie_service;
pub mod network_service;
pub mod page_renderer;
pub mod query_limiter_service;
pub mod ts_show_service;

#[derive(Debug)]
pub struct ServiceContext {
    pub network: Arc<NetworkService>,
    pub cdn: Arc<CdnService>,
    pub query_limiter: Arc<QueryLimitingService>,
    pub content_hash_service: Arc<ContentHashService>,
    pub books: Arc<BookService>,
    pub movies: Arc<MovieService>,
    pub tv_shows: Arc<TvShowService>,
}

impl ServiceContext {
    pub fn new() -> Result<Self> {
        Ok(Self {
            network: Arc::new(NetworkService::new()),
            cdn: Arc::new(CdnService::new()),
            query_limiter: Arc::new(QueryLimitingService::new()?),
            content_hash_service: Arc::new(ContentHashService::new()?),
            books: Arc::new(BookService::new()?),
            movies: Arc::new(MovieService::new()?),
            tv_shows: Arc::new(TvShowService::new()?),
        })
    }
}
