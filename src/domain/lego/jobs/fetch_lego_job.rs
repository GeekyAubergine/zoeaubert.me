use std::{collections::HashMap, hash::Hash, time::Duration};

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    application::events::Event,
    domain::lego::lego_models::{LegoMinifig, LegoSet},
    get_json,
    infrastructure::{app_state::AppState, bus::job_runner::Job, config::Config},
    load_archive_file,
    prelude::Result,
    save_archive_file, LEGO_ARCHIVE_FILENAME, ONE_HOUR_CACHE_PERIOD,
};

const NO_REFETCH_DURATION: Duration = ONE_HOUR_CACHE_PERIOD;

const LOGIN_URL: &str = "https://brickset.com/api/v3.asmx/login";
const GET_SET_URL: &str = "https://brickset.com/api/v3.asmx/getSets";
const GET_MINIFIG_URL: &str = "https://brickset.com/api/v3.asmx/getMinifigCollection";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BricksetLoginResponse {
    hash: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BricksetSetImages {
    #[serde(rename = "imageURL")]
    image_url: String,
    #[serde(rename = "thumbnailURL")]
    thumbnail_url: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BricksetSetCollection {
    #[serde(rename = "qtyOwned")]
    qty_owned: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BricksetSet {
    #[serde(rename = "setID")]
    set_id: u32,
    name: String,
    number: String,
    category: String,
    pieces: Option<u32>,
    image: BricksetSetImages,
    #[serde(rename = "bricksetURL")]
    brickset_url: String,
    collection: BricksetSetCollection,
}

impl From<BricksetSet> for LegoSet {
    fn from(set: BricksetSet) -> Self {
        LegoSet::new(
            set.set_id,
            set.name,
            set.number,
            set.category,
            set.pieces.unwrap_or(1),
            set.image.image_url,
            set.image.thumbnail_url,
            set.brickset_url,
            set.collection.qty_owned,
        )
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSetResponse {
    status: String,
    sets: Vec<BricksetSet>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BricksetMinifig {
    #[serde(rename = "minifigNumber")]
    minifig_number: String,
    name: String,
    category: String,
    #[serde(rename = "ownedInSets")]
    owned_in_sets: u32,
    #[serde(rename = "ownedLoose")]
    owned_loose: u32,
}

impl From<BricksetMinifig> for LegoMinifig {
    fn from(set: BricksetMinifig) -> Self {
        LegoMinifig::new(
            set.minifig_number.clone(),
            set.name,
            set.category,
            set.owned_in_sets,
            set.owned_loose,
            set.owned_in_sets + set.owned_loose,
            format!(
                "https://img.bricklink.com/ItemImage/MN/0/{}.png",
                set.minifig_number
            ),
        )
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMinifigsResponse {
    status: String,
    minifigs: Vec<BricksetMinifig>,
}

fn make_login_url(config: &Config) -> String {
    format!(
        "{}?apiKey={}&username={}&password={}",
        LOGIN_URL,
        config.brickset().api_key(),
        config.brickset().username(),
        config.brickset().password()
    )
}

fn make_get_set_url(config: &Config, hash: &str) -> String {
    format!(
        "{}?apiKey={}&userHash={}&params={{\"owned\":1, \"pageSize\": 500}}",
        GET_SET_URL,
        config.brickset().api_key(),
        hash,
    )
}

fn make_get_minifig_url(config: &Config, hash: &str) -> String {
    format!(
        "{}?apiKey={}&userHash={}&params={{\"owned\":1, \"pageSize\": 500}}",
        GET_MINIFIG_URL,
        config.brickset().api_key(),
        hash,
    )
}

#[derive(Debug)]
pub struct FetchLegoJob;
impl FetchLegoJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for FetchLegoJob {
    fn name(&self) -> &str {
        "FetchLegoJob"
    }
    async fn run(&self, app_state: &AppState) -> Result<()> {
        let last_updated = app_state.lego_repo().get_last_updated().await;

        if last_updated + NO_REFETCH_DURATION > Utc::now() {
            return Ok(());
        }

        let login_response =
            get_json::<BricksetLoginResponse>(&make_login_url(app_state.config())).await?;

        let get_set_url = make_get_set_url(app_state.config(), &login_response.hash);

        let get_set_response = get_json::<GetSetResponse>(&get_set_url).await?;

        let mut lego_sets: HashMap<String, LegoSet> = HashMap::new();

        if get_set_response.status == "success" {
            lego_sets = get_set_response
                .sets
                .iter()
                .map(|set| (set.number.clone(), set.clone().into()))
                .collect();
        }

        let get_minifig_url = make_get_minifig_url(app_state.config(), &login_response.hash);

        let get_minifig_response = get_json::<GetMinifigsResponse>(&get_minifig_url).await?;

        let mut lego_minifigs: HashMap<String, LegoMinifig> = HashMap::new();

        if get_minifig_response.status == "success" {
            lego_minifigs = get_minifig_response
                .minifigs
                .iter()
                .map(|minifig| (minifig.minifig_number.clone(), minifig.clone().into()))
                .collect();
        }

        app_state
            .lego_repo()
            .commit(lego_sets, lego_minifigs)
            .await;

        app_state.dispatch_event(Event::LegoRepoUpdated).await?;

        Ok(())
    }
}
