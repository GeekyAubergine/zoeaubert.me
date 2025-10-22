use crate::domain::models::media::{Media, MediaDimensions};
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

fn render_image<'l>(image: &'l SizedImage, description: &'l str) -> impl Renderable + 'l {
    maud! {
        img
            src={(image.file.as_cdn_url().as_str())}
            alt={(description)}
            width={(image.dimensions.width)}
            height={(image.dimensions.height)};
    }
}

impl Image {
    pub fn render_original<'l>(&'l self) -> impl Renderable + 'l {
        render_image(&self.original, &self.description)
    }

    pub fn render_large<'l>(&'l self) -> impl Renderable + 'l {
        render_image(&self.large, &self.description)
    }

    pub fn render_small<'l>(&'l self) -> impl Renderable + 'l {
        render_image(&self.small, &self.description)
    }
    pub fn render_tiny<'l>(&'l self) -> impl Renderable + 'l {
        render_image(&self.tiny, &self.description)
    }
}

impl Media {
    pub fn render_original<'l>(&'l self) -> impl Renderable + 'l {
        match self {
            Media::Image(image) => image.render_original(),
        }
    }

    pub fn render_large<'l>(&'l self) -> impl Renderable + 'l {
        match self {
            Media::Image(image) => image.render_large(),
        }
    }

    pub fn render_small<'l>(&'l self) -> impl Renderable + 'l {
        match self {
            Media::Image(image) => image.render_small(),
        }
    }
    pub fn render_tiny<'l>(&'l self) -> impl Renderable + 'l {
        match self {
            Media::Image(image) => image.render_tiny(),
        }
    }
}
