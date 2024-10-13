use chrono::Utc;
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    domain::{
        models::lego::{LegoMinifig, LegoSet},
        queries::lego_queries::{commit_lego_minifig, commit_lego_set, find_lego_last_updated_at},
        services::CacheService,
        state::State,
    },
    infrastructure::{
        services::cdn_service_bunny::make_cdn_url,
        utils::networking::{copy_file_from_url_to_cdn, download_json},
    },
    ONE_HOUR_PERIOD,
};

use crate::prelude::*;

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

fn make_login_url() -> String {
    format!(
        "{}?apiKey={}&username={}&password={}",
        LOGIN_URL,
        dotenv!("BRICKSET_API_KEY"),
        dotenv!("BRICKSET_USERNAME"),
        dotenv!("BRICKSET_PASSWORD")
    )
}

fn make_get_set_url(hash: &str) -> String {
    format!(
        "{}?apiKey={}&userHash={}&params={{\"owned\":1, \"pageSize\": 500}}",
        GET_SET_URL,
        dotenv!("BRICKSET_API_KEY"),
        hash,
    )
}

fn make_get_minifig_url(hash: &str) -> String {
    format!(
        "{}?apiKey={}&userHash={}&params={{\"owned\":1, \"pageSize\": 500}}",
        GET_MINIFIG_URL,
        dotenv!("BRICKSET_API_KEY"),
        hash,
    )
}

pub async fn update_lego_command(state: &impl State) -> Result<()> {
    if let Some(last_updated_at) = find_lego_last_updated_at(state).await? {
        if last_updated_at + ONE_HOUR_PERIOD > Utc::now() {
            return Ok(());
        }
    }

    info!("Updating lego sets and minifigs");

    let client = reqwest::Client::new();

    let login_reponse = download_json::<BricksetLoginResponse>(&client, &make_login_url()).await?;

    let sets_response =
        download_json::<GetSetResponse>(&client, &make_get_set_url(&login_reponse.hash)).await?;

    let minifigs_response =
        download_json::<GetMinifigsResponse>(&client, &make_get_minifig_url(&login_reponse.hash))
            .await?;

    if sets_response.status == "success" {
        for set in sets_response.sets {
            let cdn_url = format!("lego/{}.jpg", set.set_id);
            let thumbnail_cdn_url = format!("lego/{}-thumbnail.jpg", set.set_id);

            copy_file_from_url_to_cdn(state, &set.image.image_url, &cdn_url).await?;
            copy_file_from_url_to_cdn(state, &set.image.thumbnail_url, &thumbnail_cdn_url).await?;

            let set = LegoSet::new(
                set.set_id,
                set.name,
                set.number,
                set.category,
                set.pieces.unwrap_or(1),
                make_cdn_url(&cdn_url),
                make_cdn_url(&thumbnail_cdn_url),
                set.brickset_url,
                set.collection.qty_owned,
            );

            commit_lego_set(state, &set).await?;
        }
    }

    if minifigs_response.status == "success" {
        for minifig in minifigs_response.minifigs {
            let image_url = format!(
                "https://img.bricklink.com/ItemImage/MN/0/{}.png",
                minifig.minifig_number
            );

            let cdn_url = format!("lego/{}.png", minifig.minifig_number);

            copy_file_from_url_to_cdn(state, &image_url, &cdn_url).await?;

            let minifig = LegoMinifig::new(
                minifig.minifig_number,
                minifig.name,
                minifig.category,
                minifig.owned_in_sets,
                minifig.owned_loose,
                minifig.owned_in_sets + minifig.owned_loose,
                make_cdn_url(&cdn_url),
            );

            commit_lego_minifig(state, &minifig).await?;
        }
    }

    Ok(())
}
