use askama::Error;
use serde::de::DeserializeOwned;

pub struct NetworkResponseBodyJson<J>(pub J)
where
    J: DeserializeOwned;

pub struct NetworkResponseBytes(pub Vec<u8>);

pub enum NetworkResponseBody<J>
where
    J: DeserializeOwned,
{
    Json(NetworkResponseBodyJson<J>),
    Bytes(NetworkResponseBytes),
}

impl<J> NetworkResponseBody<J>
where
    J: DeserializeOwned,
{
    pub fn json(body: J) -> NetworkResponseBody<J> {
        NetworkResponseBody::Json(NetworkResponseBodyJson(body))
    }

    pub fn bytes(body: Vec<u8>) -> NetworkResponseBody<J> {
        NetworkResponseBody::Bytes(NetworkResponseBytes(body))
    }
}

pub struct NetworkResponseOk<J>
where
    J: DeserializeOwned,
{
    pub status: u16,
    pub body: NetworkResponseBody<J>,
}

pub struct NetworkResponseBad {
    pub status: u16,
    pub error: Error,
}

pub enum NetworkResponse<J>
where
    J: DeserializeOwned,
{
    Ok(NetworkResponseOk<J>),
    Bad(NetworkResponseBad),
}

impl<J> NetworkResponse<J>
where
    J: DeserializeOwned,
{
    pub fn ok(status: u16, body: NetworkResponseBody<J>) -> NetworkResponse<J> {
        NetworkResponse::Ok(NetworkResponseOk { status, body })
    }

    pub fn bad(status: u16, error: Error) -> NetworkResponse<J> {
        NetworkResponse::Bad(NetworkResponseBad { status, error })
    }
}
