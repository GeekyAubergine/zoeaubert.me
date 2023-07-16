pub mod post_image;
pub mod mastodon_post;

pub enum ImageOrientation {
    Landscape,
    Portrait,
    Square,
}

impl ImageOrientation {
    pub fn from(width: u32, height: u32) -> Self {
        match width.cmp(&height) {
            std::cmp::Ordering::Less => Self::Portrait,
            std::cmp::Ordering::Equal => Self::Square,
            std::cmp::Ordering::Greater => Self::Landscape,
        }
    }
}
