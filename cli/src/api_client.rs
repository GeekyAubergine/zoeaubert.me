use std::{
    net::{SocketAddr, SocketAddrV4},
    sync::Arc,
};

use dotenvy_macro::dotenv;
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use shared::zoeaubert_proto::webserver::silly_names_client::SillyNamesClient;
use tonic::{
    metadata::MetadataValue,
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
};
use tracing::debug;

use crate::{
    error::{ApiError, Error, TonicError},
    prelude::Result,
};

fn make_api_url(path: &str) -> String {
    let mut url = dotenv!("WEBSERVER_URL").to_string();

    if !url.ends_with('/') {
        url.push('/');
    }

    if !path.starts_with("api/") {
        url.push_str("api/");
    }

    if !path.starts_with('/') {
        url.push('/');
    }

    url.push_str(path);

    let url = url.replace("//", "/");

    url
}

pub struct ApiResponse<B> {
    code: StatusCode,
    body: B,
}

// type ClientChannel = InterceptedService<
//     Channel,
//     impl FnOnce(tonic::Request<()>) -> std::result::Result<tonic::Request<()>, tonic::Status>,
// >;

fn add_authorization_header(
    mut req: tonic::Request<()>,
) -> std::result::Result<tonic::Request<()>, tonic::Status> {
    let token: MetadataValue<_> = format!("Bearer {}", dotenv!("API_TOKEN")).parse().unwrap();

    req.metadata_mut().insert("authorization", token);
    Ok(req)
}

#[derive(Clone)]
pub struct Inceptor;

impl Interceptor for Inceptor {
    fn call(
        &mut self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        add_authorization_header(request)
    }
}

type InterceptedChannel = InterceptedService<Channel, Inceptor>;

pub struct ApiClientBase {
    reqwest_client: reqwest::Client,
    silly_names_client: SillyNamesClient<InterceptedChannel>,
}

impl ApiClientBase {
    pub async fn new() -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();

        let api_token = format!("Bearer {}", dotenv!("API_TOKEN"))
            .parse()
            .map_err(|_| ApiError::invalid_header_config("API_TOKEN is invalid".to_string()))?;

        headers.insert(AUTHORIZATION, api_token);

        let content_type: HeaderValue = "application/json"
            .parse()
            .map_err(|_| ApiError::invalid_header_config("content type is invalid".to_string()))?;

        headers.insert(CONTENT_TYPE, content_type);

        let reqwest_client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(Error::HttpReqwest)?;

        let channel = Channel::from_shared(dotenv!("GRPC_URL"))
            .map_err(TonicError::invalid_uri)?
            .connect()
            .await
            .map_err(TonicError::unable_to_connect)?;

        let silly_names_client = SillyNamesClient::with_interceptor(channel, Inceptor);

        Ok(Self {
            reqwest_client,
            silly_names_client,
        })
    }

    pub async fn put<B>(&self, path: &str, body: &B) -> Result<()>
    where
        B: serde::Serialize,
    {
        let url = make_api_url(path);

        debug!("PUT {}", url);

        let response = self
            .reqwest_client
            .put(&url)
            .json(&body)
            .send()
            .await
            .map_err(Error::HttpReqwest)?;

        let code = response.status();

        if !code.is_success() {
            return Err(ApiError::bad_response(code, url));
        }

        Ok(())
    }

    pub fn silly_names_client(&self) -> SillyNamesClient<InterceptedChannel> {
        self.silly_names_client.clone()
    }
}

pub type ApiClient = Arc<ApiClientBase>;

pub async fn make_api_client() -> Result<ApiClient> {
    Ok(Arc::new(ApiClientBase::new().await?))
}
