use crate::domain::models::media::{Media, MediaDimensions};
use crate::prelude::*;
use hypertext::{prelude::*};

use crate::domain::models::image::{Image, SizedImage};
use crate::renderer::{render_template, TemplateRenderResult};

pub struct RenderableImage<'l> {
    image: &'l SizedImage,
    description: &'l str,
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

pub struct MediaGripOptions {
    pub compress: bool,
    pub link_to_original: bool,
}

impl MediaGripOptions {
    pub fn for_list() -> Self {
        Self {
            compress: true,
            link_to_original: false,
        }
    }

    pub fn for_post() -> Self {
        Self {
            compress: false,
            link_to_original: true,
        }
    }
}

enum MediaRenderableSize {
    Tiny,
    Small,
    Large,
    Original,
}

fn render_media_at_size<'l>(
    media: &'l Media,
    size: &'l MediaRenderableSize,
) -> impl Renderable + 'l {
    maud! {
        @match size {
            MediaRenderableSize::Tiny => (media.render_tiny()),
            MediaRenderableSize::Small => (media.render_small()),
            MediaRenderableSize::Large => (media.render_large()),
            MediaRenderableSize::Original => (media.render_original()),
        }
    }
}

fn render_media<'l>(
    media: &'l Media,
    options: &'l MediaGripOptions,
    size: &'l MediaRenderableSize,
) -> impl Renderable + 'l {
    maud! {
        @if options.link_to_original {
            a href=(media.original_cdn_url().as_str()) {
                (render_media_at_size(media, size))
            }
        }
        @else {
            (render_media_at_size(media, size))
        }
    }
}

pub fn render_media_grid<'l>(
    media: &'l [Media],
    options: &'l MediaGripOptions,
) -> impl Renderable + 'l {
    maud! {
        @match media.len() {
            0 => {}
            1 => {
                div class="media" {
                    @if let Some(media) = media.first() {
                        (render_media(media, options, if options.compress { &MediaRenderableSize::Large } else { &MediaRenderableSize::Original }))
                    }
                }
            }
            _ => {
                div class="media" {
                    div class="media-grid" {
                        @for element in media.iter().take(4) {
                            (render_media(element, options, if options.compress { &MediaRenderableSize::Small } else { &MediaRenderableSize::Large }))
                        }
                    }
                }
            }
        }
    }
}
