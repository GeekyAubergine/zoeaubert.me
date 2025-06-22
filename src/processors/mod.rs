use tokio::try_join;

use crate::{
    domain::{models::data::Data, state::State},
    processors::{
        about_text::process_about_text, blog_posts::process_blog_posts, faq::process_faq, games::process_games, lego::proces_lego, micro_blog_archive::process_micro_blog_archive, micro_posts::process_micro_posts, now_text::process_now_text, projects::process_projects, referrals::process_referrals, silly_names::process_silly_names
    },
    services::{file_service::FileService, network_service::NetworkService2, ServiceContext},
};

use crate::prelude::*;

pub mod about_text;
pub mod blog_posts;
pub mod faq;
pub mod games;
pub mod lego;
pub mod micro_blog_archive;
pub mod micro_posts;
pub mod now_text;
pub mod projects;
pub mod referrals;
pub mod silly_names;

pub async fn process_data(legacy_state: &impl State, ctx: &ServiceContext) -> Result<Data> {
    let (
        games,
        now_text,
        about_text,
        faq,
        projects,
        referrals,
        silly_names,
        blog_posts,
        micro_posts,
        micro_blog_archive,
        lego,
    ) = try_join!(
        process_games(ctx),
        process_now_text(ctx),
        process_about_text(ctx),
        process_faq(ctx),
        process_projects(ctx),
        process_referrals(ctx),
        process_silly_names(ctx),
        process_blog_posts(ctx),
        process_micro_posts(ctx),
        process_micro_blog_archive(ctx),
        proces_lego(ctx),
    )?;

    let mut data = Data::from_state(legacy_state).await?;

    data.games = games;
    data.now_text = now_text;
    data.about_text = about_text;
    data.faq = faq;
    data.projects = projects;
    data.referrals = referrals;
    data.silly_names = silly_names;
    data.lego = lego;

    // Posts?

    Ok(data)
}
