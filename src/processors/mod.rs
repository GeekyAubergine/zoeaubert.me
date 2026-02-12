use std::process::exit;

use chrono::Utc;
use tokio::try_join;
use tracing::{info, instrument};

use crate::{
    domain::models::data::Data,
    processors::{
        about_text::load_about_text, albums::load_albums, blog_posts::load_blog_posts,
        credits::load_credits, faq::load_faq, games::load_games, lego::load_lego,
        mastodon::load_mastodon_posts, micro_blog_archive::load_micro_blog_archive,
        micro_posts::load_micro_posts, now_text::load_now_text, projects::load_projects,
        referrals::load_referrals, silly_names::load_silly_names,
        timeline_events::process_timeline_events,
    },
    services::{ServiceContext, file_service::FileService, network_service::NetworkService},
};

use crate::prelude::*;

pub mod about_text;
pub mod albums;
pub mod blog_posts;
pub mod credits;
pub mod faq;
pub mod games;
pub mod lego;
pub mod mastodon;
pub mod micro_blog_archive;
pub mod micro_posts;
pub mod now_text;
pub mod projects;
pub mod referrals;
pub mod silly_names;
pub mod timeline_events;

pub mod tasks;

const MOVIE_REVIEW_POST_TAG: &str = "Movies";
const TV_SHOW_REVIEW_POST_TAG: &str = "TV";
const BOOK_REVIEW_POST_TAG: &str = "Books";

#[instrument(skip_all)]
pub fn process_data(ctx: &ServiceContext) -> Result<Data> {
    info!("Processing data | Start");

    let start = Utc::now();

    let mastodon = load_mastodon_posts(ctx)?;

    let games = load_games(ctx)?;

    let now_text = load_now_text(ctx)?;
    let about_text = load_about_text(ctx)?;
    let faq = load_faq(ctx)?;
    let projects = load_projects(ctx)?;
    let referrals = load_referrals(ctx)?;
    let silly_names = load_silly_names(ctx)?;
    let blog_posts = load_blog_posts(ctx)?;
    let micro_posts = load_micro_posts(ctx)?;
    let micro_blog_archive = load_micro_blog_archive(ctx)?;
    let lego = load_lego(ctx)?;
    let albums = load_albums(ctx)?;
    let credits = load_credits(ctx)?;

    let mut micro_posts = micro_posts;
    micro_posts.extend(micro_blog_archive);

    info!(
        "Processing data | Load | Done [{}ms]",
        (Utc::now() - start).num_milliseconds()
    );

    let start = Utc::now();

    let timeline_events =
        process_timeline_events(ctx, blog_posts, micro_posts, mastodon, &games, &albums);

    info!(
        "Processing data | Process Timeline | Events: {} [{}ms]",
        timeline_events.all_by_date().len(),
        (Utc::now() - start).num_milliseconds(),
    );

    Ok(Data {
        about_text,
        silly_names,
        faq,
        referrals,
        now_text,
        lego,
        games,
        albums,
        projects,
        timeline_events,
        credits,
    })
}
