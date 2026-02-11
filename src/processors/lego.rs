use dotenvy_macro::dotenv;
use serde::Deserialize;
use tracing::info;
use url::Url;

use crate::{
    config::{CONFIG, Config},
    domain::models::lego::{Lego, LegoMinifig, LegoSet},
    prelude::*,
    processors::tasks::{Task, run_tasks},
    services::{
        ServiceContext,
        cdn_service::CdnFile,
        file_service::{FileService, ReadableFile, WritableFile},
        media_service::MediaService,
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
        LOGIN_URL, CONFIG.brickset.api_key, CONFIG.brickset.username, CONFIG.brickset.password
    )
    .parse()
    .unwrap()
}

fn make_get_set_url(hash: &str) -> Url {
    format!(
        "{}?apiKey={}&userHash={}&params={{\"owned\":1, \"pageSize\": 500}}",
        GET_SET_URL, CONFIG.brickset.api_key, hash,
    )
    .parse()
    .unwrap()
}

fn make_get_minifig_url(hash: &str) -> Url {
    format!(
        "{}?apiKey={}&userHash={}&params={{\"owned\":1, \"pageSize\": 500}}",
        GET_MINIFIG_URL, CONFIG.brickset.api_key, hash,
    )
    .parse()
    .unwrap()
}

pub fn load_lego(ctx: &ServiceContext) -> Result<Lego> {
    let file = FileService::archive(STORE.into());

    let mut lego = file.read_json_or_default()?;

    if !ctx.query_limiter.can_query_within_day(QUERY_KEY)? {
        return Ok(lego);
    }

    info!("Processing lego sets and minifigs");

    let login_reponse = ctx
        .network
        .download_json::<BricksetLoginResponse>(&make_login_url())?;

    let sets_response = ctx
        .network
        .download_json::<GetSetResponse>(&make_get_set_url(&login_reponse.hash))?;

    let minifigs_response = ctx
        .network
        .download_json::<GetMinifigsResponse>(&make_get_minifig_url(&login_reponse.hash))?;

    if sets_response.status == "success" {
        let set_tasks = sets_response
            .sets
            .into_iter()
            .map(|set| ProcessSet { set })
            .collect();

        let sets = run_tasks(set_tasks, ctx)?;

        for set in sets {
            lego.add_set(set);
        }
    }

    if minifigs_response.status == "success" {
        let minifig_tasks = minifigs_response
            .minifigs
            .into_iter()
            .map(|minifig| ProcessMinifig { minifig })
            .collect();

        let minifigs = run_tasks(minifig_tasks, ctx)?;

        for minifig in minifigs {
            lego.add_minifig(minifig);
        }
    }

    file.write_json(&lego.clone())?;

    Ok(lego)
}

struct ProcessSet {
    set: BricksetSet,
}

impl Task for ProcessSet {
    type Output = LegoSet;

    fn run(self, ctx: &ServiceContext) -> Result<Self::Output> {
        let cdn_file = CdnFile::from_str(&format!("lego/{}.png", self.set.set_id));

        let image = MediaService::image_from_url(
            ctx,
            &self.set.image.image_url,
            &cdn_file,
            &self.set.name,
            None,
            None,
        )?;

        Ok(LegoSet::new(
            self.set.set_id,
            self.set.name,
            self.set.number,
            self.set.category,
            self.set.pieces.unwrap_or(1),
            image,
            self.set.brickset_url,
            self.set.collection.qty_owned,
        ))
    }
}

struct ProcessMinifig {
    minifig: BricksetMinifig,
}

impl Task for ProcessMinifig {
    type Output = LegoMinifig;

    fn run(self, ctx: &ServiceContext) -> Result<Self::Output> {
        let image_url: Url = format!(
            "https://img.bricklink.com/ItemImage/MN/0/{}.png",
            self.minifig.minifig_number
        )
        .parse()
        .unwrap();

        let cdn_file = CdnFile::from_str(&format!("lego/{}.png", self.minifig.minifig_number));

        let image = MediaService::image_from_url(
            ctx,
            &image_url,
            &cdn_file,
            &self.minifig.name,
            None,
            None,
        )?;

        Ok(LegoMinifig::new(
            self.minifig.minifig_number,
            self.minifig.name,
            self.minifig.category,
            self.minifig.owned_in_sets,
            self.minifig.owned_loose,
            self.minifig.owned_in_sets + self.minifig.owned_loose,
            image,
        ))
    }
}
