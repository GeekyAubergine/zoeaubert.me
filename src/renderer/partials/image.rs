use crate::domain::models::media::MediaDimensions;
use crate::prelude::*;
use hypertext::prelude::*;

use crate::domain::models::image::{Image, SizedImage};
use crate::renderer::{render_template, TemplateRenderResult};

pub struct RenderableImage<'l> {
    image: &'l SizedImage,
    description: &'l str,
}

impl<'l> Renderable for RenderableImage<'l> {
    fn render_to(&self, output: &mut String) {
        maud! {
            img
                src={(self.image.file.as_cdn_url().as_str())}
                alt={(self.description)}
                width={(self.image.dimensions.width)}
                height={(self.image.dimensions.height)};
        }
        .render_to(output);
    }

    fn render(&self) -> Rendered<String> {
        let mut output = String::new();
        self.render_to(&mut output);
        Rendered(output)
    }
}

impl Image {
    pub fn render_original(&self) -> RenderableImage {
        RenderableImage {
            image: &self.original,
            description: &self.description,
        }
    }

    pub fn render_large(&self) -> RenderableImage {
        RenderableImage {
            image: &self.large,
            description: &self.description,
        }
    }

    pub fn render_small(&self) -> RenderableImage {
        RenderableImage {
            image: &self.small,
            description: &self.description,
        }
    }

    pub fn render_tiny(&self) -> RenderableImage {
        RenderableImage {
            image: &self.tiny,
            description: &self.description,
        }
    }
}
