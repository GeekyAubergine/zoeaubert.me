use std::{collections::HashMap, hash::Hash, time::Duration};

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::{
    application::events::Event,
    domain::models::lego::{LegoMinifig, LegoSet},
    get_json,
    infrastructure::{app_state::AppState, bus::job_runner::Job, config::Config},
    prelude::Result,
    LEGO_ARCHIVE_FILENAME, ONE_DAY_CACHE_PERIOD, ONE_HOUR_CACHE_PERIOD,
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
            Utc::now(),
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
            Utc::now(),
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
    async fn run(&self, state: &AppState) -> Result<()> {
        if let Some(last_updated) = state
            .lego_set_repo()
            .find_most_recently_updated_at()
            .await?
        {
            if last_updated + ONE_DAY_CACHE_PERIOD > Utc::now() {
                info!("Skipping fetching lego data - cache is still valid");
                return Ok(());
            }
        }
        info!("Fetching lego data");

        let login_response =
            get_json::<BricksetLoginResponse>(&make_login_url(state.config())).await?;

        let get_set_url = make_get_set_url(state.config(), &login_response.hash);

        let get_set_response = get_json::<GetSetResponse>(&get_set_url).await?;

        if get_set_response.status == "success" {
            for lego_set in get_set_response.sets.iter() {
                let lego_set = lego_set.clone().into();

                state.lego_set_repo().commit(&lego_set).await?;
            }
        }

        let get_minifig_url = make_get_minifig_url(state.config(), &login_response.hash);

        let get_minifig_response = get_json::<GetMinifigsResponse>(&get_minifig_url).await?;

        if get_minifig_response.status == "success" {
            for minifig in get_minifig_response.minifigs.iter() {
                let minifig = minifig.clone().into();

                state.lego_minifigs_repo().commit(&minifig).await?;
            }
        }

        Ok(())
    }
}
