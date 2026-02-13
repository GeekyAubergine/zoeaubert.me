use std::{num::NonZeroU32, sync::Arc, time::Duration};

use dashmap::DashMap;
use governor::{
    Jitter, Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};
use reqwest::blocking::{Client, Response};
use serde::de::DeserializeOwned;
use tracing::instrument;
use url::Url;

use crate::{ config::CONFIG, error::NetworkError, prelude::*};

const ALLOW_LIST: [&str; 1] = [CONFIG.cdn_url];

fn is_on_allow_list(domain: &str) -> bool {
    ALLOW_LIST
        .iter()
        .any(|allowed| allowed.eq_ignore_ascii_case(domain))
}

type DomainLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

#[derive(Debug)]
struct DomainRateLimiter {
    quota: Quota,
    limiters: DashMap<String, Arc<DomainLimiter>>,
}

impl DomainRateLimiter {
    pub fn new() -> Self {
        let quota = Quota::per_second(NonZeroU32::new(5).unwrap());

        Self {
            quota,
            limiters: DashMap::new(),
        }
    }

    fn limiter_for_domain(&self, domain: &str) -> Arc<DomainLimiter> {
        if let Some(limiter) = self.limiters.get(domain) {
            return limiter.clone();
        }

        let limiter = Arc::new(RateLimiter::direct(self.quota));
        self.limiters.insert(domain.to_string(), limiter.clone());
        limiter
    }

    pub fn limit(&self, url: &Url) {
        let domain = match url.domain() {
            Some(d) => d,
            None => return,
        };

        if is_on_allow_list(domain) {
            return;
        }

        let limiter = self.limiter_for_domain(domain);

        limiter.until_ready_with_jitter(Jitter::up_to(Duration::from_millis(100)));
    }
}

#[instrument(
     skip_all,
     fields(method = "GET", url = %url),
     err
 )]
fn get(client: &Client, limiter: &DomainRateLimiter, url: &Url) -> Result<Response> {
    limiter.limit(url);

    client
        .get(url.as_str())
        .send()
        .map_err(NetworkError::fetch_error)
}

#[derive(Debug)]
pub struct NetworkService {
    client: reqwest::blocking::Client,
    limiter: DomainRateLimiter,
}

impl NetworkService {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            limiter: DomainRateLimiter::new(),
        }
    }

    #[instrument(
        skip_all,
        fields(method = "GET", url = %url),
        err
    )]
    pub fn download_json<J>(&self, url: &Url) -> Result<J>
    where
        J: DeserializeOwned,
    {
        let resp = get(&self.client, &self.limiter, url)?;

        let json = resp.json::<J>().map_err(NetworkError::fetch_error)?;

        Ok(json)
    }

    #[instrument(
        skip_all,
        fields(method = "GET", url = %url),
        err
    )]
    pub fn download_bytes(&self, url: &Url) -> Result<Vec<u8>> {
        let resp = get(&self.client, &self.limiter, url)?;

        let bytes = resp.bytes().map_err(NetworkError::fetch_error)?;

        Ok(bytes.to_vec())
    }
}
