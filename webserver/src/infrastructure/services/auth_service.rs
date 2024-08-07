use axum::{body::Body, extract::Request, http::Response, middleware::Next};
use dotenvy_macro::dotenv;
use tonic::metadata::MetadataValue;

use crate::{error::AuthError, prelude::Result, ResponseResult};

fn validate_token(token: Option<&str>) -> Result<()> {
    let token = match token {
        Some(token) => token,
        None => return Err(AuthError::invalid_token()),
    };

    if token != dotenv!("API_TOKEN") {
        return Err(AuthError::unauthorized());
    }

    Ok(())
}

pub async fn auth_middleware(req: Request, next: Next) -> ResponseResult<Response<Body>> {
    let auth_header = req.headers().get(axum::http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(auth_header) => auth_header,
        None => return Err(AuthError::no_authorization_header().into()),
    };

    let auth_header = auth_header
        .to_str()
        .map_err(|_| AuthError::invalid_header())?;

    let mut header = auth_header.split_whitespace();

    let (bearer, token) = (header.next(), header.next());

    validate_token(token).map_err(|e| e.into_response())?;

    Ok(next.run(req).await)
}

pub fn authenticate_grpc<R>(request: &tonic::Request<R>) -> std::result::Result<(), tonic::Status> {
    let expected_token: MetadataValue<_> = format!("Bearer {}", dotenv!("API_TOKEN"))
        .parse()
        .map_err(|_| AuthError::invalid_token().into_tonic_status())?;

    let auth_header = request.metadata().get("Authorization");

    match auth_header {
        Some(t) => {
            if t != expected_token {
                return Err(AuthError::unauthorized().into_tonic_status());
            }
            Ok(())
        }
        None => Err(AuthError::no_authorization_header().into_tonic_status()),
    }
}
