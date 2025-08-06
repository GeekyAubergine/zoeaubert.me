use std::process::exit;

use chrono::Utc;
use tokio::try_join;
use tracing::info;

use crate::{
    domain::models::{data::Data, post::Posts},
    processors::{
        about_text::process_about_text, albums::process_albums, blog_posts::process_blog_posts,
        extract_posts::extract_posts, faq::process_faq, games::process_games, lego::proces_lego,
        mastodon::process_mastodon, micro_blog_archive::process_micro_blog_archive,
        micro_posts::process_micro_posts, now_text::process_now_text, projects::process_projects,
        referrals::process_referrals, silly_names::process_silly_names,
    },
    services::{file_service::FileService, network_service::NetworkService2, ServiceContext},
};

use crate::prelude::*;

pub mod about_text;
pub mod albums;
pub mod blog_posts;
pub mod extract_posts;
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

pub async fn process_data(ctx: &ServiceContext) -> Result<Data> {
    info!("Processing data");

    let start = Utc::now();

    let games = process_games(ctx).await?;

    let now_text = process_now_text(ctx)?;
    let about_text = process_about_text(ctx)?;
    let faq = process_faq(ctx)?;
    let projects = process_projects(ctx).await?;
    let referrals = process_referrals(ctx)?;
    let silly_names = process_silly_names(ctx)?;
    let blog_posts = process_blog_posts(ctx).await?;
    let micro_posts = process_micro_posts(ctx).await?;
    let micro_blog_archive = process_micro_blog_archive(ctx).await?;
    let lego = proces_lego(ctx).await?;
    let albums = process_albums(ctx).await?;
    let mastodon = process_mastodon(ctx).await?;
    // let (
    //     games,
    //     now_text,
    //     about_text,
    //     faq,
    //     projects,
    //     referrals,
    //     silly_names,
    //     blog_posts,
    //     micro_posts,
    //     micro_blog_archive,
    //     lego,
    //     albums,
    //     mastodon,
    // ) = try_join!(
    //     process_games(ctx),
    //     process_now_text(ctx),
    //     process_about_text(ctx),
    //     process_faq(ctx),
    //     process_projects(ctx),
    //     process_referrals(ctx),
    //     process_silly_names(ctx),
    //     process_blog_posts(ctx),
    //     process_micro_posts(ctx),
    //     process_micro_blog_archive(ctx),
    //     proces_lego(ctx),
    //     process_albums(ctx),
    //     process_mastodon(ctx)
    // )?;

    let mut micro_posts = micro_posts;
    micro_posts.extend(micro_blog_archive);

    let posts = extract_posts(ctx, blog_posts, micro_posts, &mastodon, &albums, &games).await?;

    let posts = Posts::from_posts(posts);

    info!(
        "Processing data - done [{}ms]",
        (Utc::now() - start).num_milliseconds()
    );

    Ok(Data {
        about_text,
        silly_names,
        faq,
        referrals,
        now_text,
        lego,
        games,
        posts,
        albums,
        projects,
    })
}
