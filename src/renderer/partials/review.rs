use crate::{domain::models::slug::Link, prelude::*, renderer::partials::utils::LinkComponent};
use hypertext::prelude::*;
use url::Url;

use crate::domain::models::{image::Image, post::Post};

pub struct RenderableReviewPageHeader<'l> {
    title: &'l str,
    external_link: Option<&'l Url>,
    image: &'l Image,
    average_score: f32,
    reveiws: &'l [&'l Post],
}

impl<'l> RenderableReviewPageHeader<'l> {
    pub fn new(
        title: &'l str,
        external_link: Option<&'l Url>,
        image: &'l Image,
        average_score: f32,
        reveiws: &'l [&'l Post],
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
                        LinkComponent
                            link=(&Link::External(&external_link))
                            children=(&self.image.render_large())
                        ;
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
