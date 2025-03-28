use chrono::Utc;
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;
use url::Url;

use crate::domain::{
    models::lego::{LegoMinifig, LegoSet},
    repositories::LegoRepo,
    services::{CacheService, CdnService, ImageService, NetworkService, QueryLimitingService},
    state::State,
};

use crate::prelude::*;

const QUERY_KEY: &str = "lego";

const LOGIN_URL: &str = "https://brickset.com/api/v3.asmx/login";
const GET_SET_URL: &str = "https://brickset.com/api/v3.asmx/getSets";
const GET_MINIFIG_URL: &str = "https://brickset.com/api/v3.asmx/getMinifigCollection";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BricksetLoginResponse {
    hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BricksetSetImages {
    #[serde(rename = "imageURL")]
    image_url: Url,
    #[serde(rename = "thumbnailURL")]
    thumbnail_url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BricksetSetCollection {
    #[serde(rename = "qtyOwned")]
    qty_owned: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BricksetSet {
    #[serde(rename = "setID")]
    set_id: u32,
    name: String,
    number: String,
    category: String,
    pieces: Option<u32>,
    image: BricksetSetImages,
    #[serde(rename = "bricksetURL")]
    brickset_url: Url,
    collection: BricksetSetCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSetResponse {
    status: String,
    sets: Vec<BricksetSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinifigsResponse {
    status: String,
    minifigs: Vec<BricksetMinifig>,
}

fn make_login_url() -> Url {
    format!(
        "{}?apiKey={}&username={}&password={}",
        LOGIN_URL,
        dotenv!("BRICKSET_API_KEY"),
        dotenv!("BRICKSET_USERNAME"),
        dotenv!("BRICKSET_PASSWORD")
    )
    .parse()
    .unwrap()
}

fn make_get_set_url(hash: &str) -> Url {
    format!(
        "{}?apiKey={}&userHash={}&params={{\"owned\":1, \"pageSize\": 500}}",
        GET_SET_URL,
        dotenv!("BRICKSET_API_KEY"),
        hash,
    )
    .parse()
    .unwrap()
}

fn make_get_minifig_url(hash: &str) -> Url {
    format!(
        "{}?apiKey={}&userHash={}&params={{\"owned\":1, \"pageSize\": 500}}",
        GET_MINIFIG_URL,
        dotenv!("BRICKSET_API_KEY"),
        hash,
    )
    .parse()
    .unwrap()
}

pub async fn update_lego_command(state: &impl State) -> Result<()> {
    if !state
        .query_limiting_service()
        .can_query_within_day(QUERY_KEY)
        .await?
    {
        return Ok(());
    }

    info!("Updating lego sets and minifigs");

    let login_reponse = state
        .network_service()
        .download_json::<BricksetLoginResponse>(&make_login_url())
        .await?;

    let sets_response = state
        .network_service()
        .download_json::<GetSetResponse>(&make_get_set_url(&login_reponse.hash))
        .await?;

    let minifigs_response = state
        .network_service()
        .download_json::<GetMinifigsResponse>(&make_get_minifig_url(&login_reponse.hash))
        .await?;

    if sets_response.status == "success" {
        for set in sets_response.sets {
            let cdn_path = format!("lego/{}.jpg", set.set_id);
            let cdn_path = Path::new(&cdn_path);

            let thumbnail_cdn_url = format!("lego/{}-thumbnail.jpg", set.set_id);
            let thumbnail_cdn_path = Path::new(&thumbnail_cdn_url);

            let image = state
                .image_service()
                .copy_image_from_url(state, &set.image.image_url, &cdn_path, &set.name)
                .await?;
            let thumbnail = state
                .image_service()
                .copy_image_from_url(
                    state,
                    &set.image.thumbnail_url,
                    &thumbnail_cdn_path,
                    &set.name,
                )
                .await?;

            let set = LegoSet::new(
                set.set_id,
                set.name,
                set.number,
                set.category,
                set.pieces.unwrap_or(1),
                image,
                thumbnail,
                set.brickset_url,
                set.collection.qty_owned,
            );

            state.lego_repo().commit_set(&set).await?;
        }
    }

    if minifigs_response.status == "success" {
        for minifig in minifigs_response.minifigs {
            let image_url: Url = format!(
                "https://img.bricklink.com/ItemImage/MN/0/{}.png",
                minifig.minifig_number
            )
            .parse()
            .unwrap();

            let cdn_path = format!("lego/{}.png", minifig.minifig_number);
            let cdn_path = Path::new(&cdn_path);

            let image = state
                .image_service()
                .copy_image_from_url(state, &image_url, &cdn_path, &minifig.name)
                .await?;

            let minifig = LegoMinifig::new(
                minifig.minifig_number,
                minifig.name,
                minifig.category,
                minifig.owned_in_sets,
                minifig.owned_loose,
                minifig.owned_in_sets + minifig.owned_loose,
                image,
            );

            state.lego_repo().commit_minifig(&minifig).await?;
        }
    }

    Ok(())
}
