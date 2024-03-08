pub struct Tag {
    tag: String,
}

impl Tag {
    pub fn new(tag: &str) -> Self {
        Self { tag: tag.to_string() }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn slug(&self) -> String {
        /**
         *     eleventyConfig.addFilter('slug', function (slug) {
        if (slug === 'F1') {
            return 'f1'
        }

        if (slug === 'F2') {
            return 'f2'
        }

        if (slug === 'F3') {
            return 'f3'
        }

        if (slug === 'WIPWednesday') {
            return 'wip-wednesday'
        }

        if (slug.endsWith('GP')) {
            return slug.toLowerCase().replace('gp', '-gp')
        }

        if (slug.toLowerCase() === 'tv') {
            return 'tv'
        }

        return slug
            .replace(
                /([A-Z][a-z]+)|(\d+)/g,
                (letter) => `-${letter.toLowerCase()}`,
            )
            .replace(/^-/, '')
    })
         */
        self.tag.to_lowercase().replace(" ", "-")
    }
}