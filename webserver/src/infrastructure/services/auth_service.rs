use axum::{body::Body, extract::Request, http::Response, middleware::Next};
use dotenvy_macro::dotenv;

use crate::{error::AuthError, prelude::Result, ResponseResult};

fn validate_token(token: Option<&str>) -> ResponseResult<()> {
    let token = match token {
        Some(token) => token,
        None => return Err(AuthError::invalid_token().into()),
    };

    if token != dotenv!("API_TOKEN") {
        return Err(AuthError::unauthorized().into());
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

    validate_token(token)?;

    Ok(next.run(req).await)
}
