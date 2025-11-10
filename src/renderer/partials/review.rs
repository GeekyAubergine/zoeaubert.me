use crate::{domain::models::{slug::Link, timeline_event::TimelineEvent}, prelude::*, renderer::partials::utils::link};
use hypertext::prelude::*;
use url::Url;

use crate::domain::models::image::Image;

pub struct RenderableReviewPageHeader<'l> {
    title: &'l str,
    external_link: Option<&'l Url>,
    image: &'l Image,
    average_score: f32,
    reveiws: &'l [&'l TimelineEvent],
}

impl<'l> RenderableReviewPageHeader<'l> {
    pub fn new(
        title: &'l str,
        external_link: Option<&'l Url>,
        image: &'l Image,
        average_score: f32,
        reveiws: &'l [&'l TimelineEvent],
    ) -> Self {
        Self {
            title,
            external_link,
            image,
            average_score,
            reveiws,
        }
    }
}

impl<'l> Renderable for RenderableReviewPageHeader<'l> {
    fn render_to(&self, output: &mut String) {
        maud! {
            div class="page-header mb-12" {
                h1 { (self.title) }
                div class="items-center my-4" {
                    @if let Some(external_link) = self.external_link {
                        (link(&Link::External(&external_link), &self.image.render_large()))
                    } @else {
                        (self.image.render_large())
                    }
                }
                p { (self.average_score.round()) "/5"}
            }
        }
        .render_to(output);
    }
}
