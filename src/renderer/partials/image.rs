use crate::prelude::*;
use hypertext::prelude::*;

use crate::domain::models::image::{Image, SizedImage};
use crate::renderer::{render_template, TemplateRenderResult};

fn render_sized(sized_image: &SizedImage, description: &str) -> Rendered<String> {
    maud! {
        img
            src={(sized_image.file.as_cdn_url().as_str())}
            alt={(description)}
            width={(sized_image.dimensions.width)}
            height={(sized_image.dimensions.height)};
    }.render()
}

impl Image {
    pub fn render_small(&self) -> Rendered<String> {
        render_sized(&self.small, &self.description)
    }

    pub fn render_large(&self) -> Rendered<String> {
        render_sized(&self.large, &self.description)

    }

    pub fn redner_original(&self) -> Rendered<String> {
        render_sized(&self.original, &self.description)
    }
}
