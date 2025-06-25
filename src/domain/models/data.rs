use std::collections::HashMap;

use bitflags::bitflags;
use chrono::Datelike;

use crate::domain::models::albums::Albums;
use crate::domain::models::projects::Projects;
use crate::prelude::*;

use super::about_text::AboutText;
use super::blog_post::BlogPost;
use super::faq::Faq;
use super::games::Games;
use super::lego::Lego;
use super::now_text::NowText;
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
    pub lego: Lego,
    pub games: Games,
    pub posts: Posts,
    pub albums: Albums,
    pub projects: Projects,
}
