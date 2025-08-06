use dotenvy_macro::dotenv;
use serde::Deserialize;
use tracing::info;
use url::Url;

use crate::{
    domain::models::lego::{Lego, LegoMinifig, LegoSet},
    prelude::*,
    services::{
        cdn_service::CdnFile,
        file_service::{FileService, ReadableFile},
        media_service::MediaService,
        ServiceContext,
    },
};

const STORE: &str = "lego.json";

const QUERY_KEY: &str = "lego";

const LOGIN_URL: &str = "https://brickset.com/api/v3.asmx/login";
const GET_SET_URL: &str = "https://brickset.com/api/v3.asmx/getSets";
const GET_MINIFIG_URL: &str = "https://brickset.com/api/v3.asmx/getMinifigCollection";

#[derive(Debug, Clone, Deserialize)]
pub struct BricksetLoginResponse {
    hash: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BricksetSetImages {
    #[serde(rename = "imageURL")]
    image_url: Url,
    #[serde(rename = "thumbnailURL")]
    thumbnail_url: Url,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BricksetSetCollection {
    #[serde(rename = "qtyOwned")]
    qty_owned: u32,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct GetSetResponse {
    status: String,
    sets: Vec<BricksetSet>,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
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

pub async fn proces_lego(ctx: &ServiceContext) -> Result<Lego> {
    if !ctx.query_limiter.can_query_within_day(QUERY_KEY).await? {
        let existing: Lego = FileService::archive(STORE.into()).read_json_or_default()?;
        return Ok(existing);
    }

    let mut lego = Lego::new();

    info!("Processing lego sets and minifigs");

    let login_reponse = ctx
        .network
        .download_json::<BricksetLoginResponse>(&make_login_url())
        .await?;

    let sets_response = ctx
        .network
        .download_json::<GetSetResponse>(&make_get_set_url(&login_reponse.hash))
        .await?;

    let minifigs_response = ctx
        .network
        .download_json::<GetMinifigsResponse>(&make_get_minifig_url(&login_reponse.hash))
        .await?;

    if sets_response.status == "success" {
        for set in sets_response.sets {
            let cdn_file = CdnFile::from_str(&format!("lego/{}.png", set.set_id));

            let image =
                MediaService::image_from_url(ctx, &set.image.image_url, &cdn_file, &set.name, None)
                    .await?;

            let set = LegoSet::new(
                set.set_id,
                set.name,
                set.number,
                set.category,
                set.pieces.unwrap_or(1),
                image,
                set.brickset_url,
                set.collection.qty_owned,
            );

            lego.add_set(&set);
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

            let cdn_file = CdnFile::from_str(&format!("lego/{}.png", minifig.minifig_number));

            let image =
                MediaService::image_from_url(ctx, &image_url, &cdn_file, &minifig.name, None)
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

            lego.add_minifig(&minifig);
        }
    }

    Ok(lego)
}
