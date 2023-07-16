use super::ImageOrientation;

pub struct PostImage {
    src: String,
    alt: String,
    width: u32,
    height: u32,
    oreintation: ImageOrientation,
}

impl PostImage {
    pub fn new(src: String, alt: String, width: u32, height: u32) -> Self {
        let oreintation = ImageOrientation::from(width, height);

        Self {
            src,
            alt,
            width,
            height,
            oreintation,
        }
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn alt(&self) -> &str {
        &self.alt
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
