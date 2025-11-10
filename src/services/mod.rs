use crate::{
    prelude::*,
    services::{
        book_service::BookService, cdn_service::CdnService, data_hash_cache_service::ContentHashService, file_service::FileService, network_service::NetworkService2, query_limiter_service::QueryLimitingService2
    },
};

pub mod cdn_service;
pub mod data_hash_cache_service;
pub mod file_service;
pub mod media_service;
pub mod network_service;
pub mod page_renderer;
pub mod query_limiter_service;
pub mod book_service;

#[derive(Debug)]
pub struct ServiceContext {
    pub network: NetworkService2,
    pub cdn: CdnService,
    pub query_limiter: QueryLimitingService2,
    pub content_hash_service: ContentHashService,
    pub books: BookService,
}

impl ServiceContext {
    pub fn new() -> Result<Self> {
        Ok(Self {
            network: NetworkService2::new(),
            cdn: CdnService::new(),
            query_limiter: QueryLimitingService2::new()?,
            content_hash_service: ContentHashService::new()?,
            books: BookService::new()?,
        })
    }
}
