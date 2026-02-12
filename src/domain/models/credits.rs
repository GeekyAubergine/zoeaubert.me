use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize)]
pub struct Credit {
    pub name: String,
    pub url: Url,
    pub text: String,
}

pub type Credits = Vec<Credit>;
