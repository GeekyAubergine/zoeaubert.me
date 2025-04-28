use std::collections::HashMap;

use bitflags::bitflags;
use chrono::Datelike;

use crate::domain::repositories::{AboutTextRepo, OmniPostRepo};
use crate::prelude::*;

use crate::domain::state::State;

use super::about_text::AboutText;
use super::blog_post::BlogPost;
use super::faq::Faq;
use super::games::Games;
use super::league::LeaugeData;
use super::lego::Lego;
use super::now_text::NowText;
use super::omni_post::OmniPost;
use super::post::Posts;
use super::referral::Referrals;
use super::silly_names::SillyNames;
use super::tag::Tag;

pub struct Data {
    pub about_text: AboutText,
    pub silly_names: SillyNames,
    pub faq: Faq,
    pub referrals: Referrals,
    pub now_text: NowText,
    pub league: LeaugeData,
    pub lego: Lego,
    pub games: Games,
    pub posts: Posts,
}

impl Data {
    pub async fn from_state(state: &impl State) -> Result<Data> {
        let mut posts = Posts::new();

        posts.add_posts(state.omni_post_repo().find_all_by_date().await?);

        Ok(Data {
            about_text: AboutText::from_state(state).await?,
            silly_names: SillyNames::from_state(state).await?,
            faq: Faq::from_state(state).await?,
            referrals: Referrals::from_state(state).await?,
            now_text: NowText::from_state(state).await?,
            league: LeaugeData::from_state(state).await?,
            lego: Lego::from_state(state).await?,
            games: Games::from_state(state).await?,
            posts,
        })
    }
}
