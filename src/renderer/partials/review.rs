use crate::{
    prelude::*,
    renderer::partials::utils::{link, PageParts},
};
use hypertext::prelude::*;
use url::Url;

use crate::domain::models::{image::Image, post::Post};

pub fn review_page_header(
    title: &str,
    external_link: Option<&Url>,
    image: Image,
    average_score: f32,
    reveiws: &[&Post],
) -> Rendered<String> {
    maud! {
        div class="page-header mb-12" {
            h1 { (title) }
            div class="items-center my-4" {
                @if let Some(external_link) = external_link {
                    (link(&external_link, image.render_large()).as_str())
                } @else {
                    (image.render_large().as_str())
                }
            }
            p { (average_score.round()) "/5"}
        }
    }
    .render()
}
