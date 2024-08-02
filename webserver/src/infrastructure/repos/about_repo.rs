use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::models::about::About, infrastructure::config::Config, prelude::*
};

const FILE_NAME_SHORT: &str = "about_short.md";
const FILE_NAME_LONG: &str = "about_long.md";

#[derive(Debug, Clone, Default)]
pub struct AboutRepo {
    about: Arc<RwLock<About>>,
}

impl AboutRepo {
    pub async fn commit(&self, about: About) {
        let mut about_ref = self.about.write().await;
        *about_ref = about;
    }

    pub async fn get(&self) -> About {
        self.about.read().await.clone()
    }
}
