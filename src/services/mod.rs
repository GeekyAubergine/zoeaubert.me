use crate::{
    prelude::*,
    services::{
        cdn_service::CdnService, file_service::FileService, image_service::ImageService2,
        network_service::NetworkService2, query_limiter_service::QueryLimitingService2,
    },
};

pub mod cdn_service;
pub mod file_service;
pub mod image_service;
pub mod network_service;
pub mod page_renderer;
pub mod query_limiter_service;

pub struct ServiceContext {
    pub network: NetworkService2,
    pub cdn: CdnService,
    pub image: ImageService2,
    pub query_limiter: QueryLimitingService2,
}

impl ServiceContext {
    pub async fn new() -> Result<Self> {
        Self {
            network: NetworkService2::new(),
            cdn: CdnService::new(),
            image: ImageService2::new(),
            query_limiter: QueryLimitingService2::new().await?
        }
    }
}
