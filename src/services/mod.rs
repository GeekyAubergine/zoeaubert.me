use crate::{
    prelude::*,
    services::{
        cdn_service::CdnService, file_service::FileService, network_service::NetworkService2,
        query_limiter_service::QueryLimitingService2,
    },
};

pub mod cdn_service;
pub mod file_service;
pub mod media_service;
pub mod network_service;
pub mod page_renderer;
pub mod query_limiter_service;

pub struct ServiceContext {
    pub network: NetworkService2,
    pub cdn: CdnService,
    pub query_limiter: QueryLimitingService2,
}

impl ServiceContext {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            network: NetworkService2::new(),
            cdn: CdnService::new(),
            query_limiter: QueryLimitingService2::new().await?,
        })
    }
}
