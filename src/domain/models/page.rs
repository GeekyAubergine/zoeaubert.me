use chrono::format;

use crate::{
    build_data::BUILD_DATE,
    infrastructure::config::{SiteConfig, SiteConfigNavLink, SiteConfigSocialNetworkLink},
};

use super::image::Image;

pub struct NavigationLink {
    name: String,
    url: String,
    target: String,
    rel: String,
}

impl NavigationLink {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn rel(&self) -> &str {
        &self.rel
    }
}

impl From<SiteConfigNavLink> for NavigationLink {
    fn from(val: SiteConfigNavLink) -> Self {
        NavigationLink {
            name: val.name().to_owned(),
            url: val.url().to_owned(),
            target: val.target().to_owned(),
            rel: val.rel().to_owned(),
        }
    }
}

pub struct SocialNetworkLink {
    name: String,
    url: String,
    show_in_top_nav: bool,
    show_in_footer: bool,
}

impl SocialNetworkLink {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn show_in_top_nav(&self) -> bool {
        self.show_in_top_nav
    }

    pub fn show_in_footer(&self) -> bool {
        self.show_in_footer
    }
}

impl From<SiteConfigSocialNetworkLink> for SocialNetworkLink {
    fn from(val: SiteConfigSocialNetworkLink) -> Self {
        SocialNetworkLink {
            name: val.name().to_owned(),
            url: val.url().to_owned(),
            show_in_top_nav: val.show_in_top_nav(),
            show_in_footer: val.show_in_footer(),
        }
    }
}

pub struct Page {
    url: String,
    title: String,
    description: String,
    author: String,
    image: Image,
    language: String,
    build_date: String,
    no_index: bool,
    disable_search: bool,
    navigation_links: Vec<NavigationLink>,
    social_links: Vec<SocialNetworkLink>,
}

impl Page {
    pub fn new(
        site: &SiteConfig,
        slug: &str,
        title: Option<&str>,
        description: Option<&str>,
        image: Option<Image>,
    ) -> Self {
        let title = match title {
            Some(t) => format!("{} | {}", t, site.title()),
            None => site.title().to_owned(),
        };

        let description = match description {
            Some(d) => d.to_string(),
            None => site.description().to_owned(),
        };

        let image = match image {
            Some(image) => image,
            None => site.image().clone(),
        };

        let url = match slug {
            "" | "/" => site.url().to_owned(),
            _ => format!("{}{}", site.url(), slug),
        };

        Self {
            url,
            title,
            description,
            author: site.author().to_owned(),
            image: image.clone(),
            language: site.language().to_owned(),
            build_date: BUILD_DATE.to_string(),
            no_index: false,
            disable_search: false,
            navigation_links: site
                .nav_links()
                .iter()
                .map(|link| NavigationLink::from(link.clone()))
                .collect(),
            social_links: site
                .social_links()
                .iter()
                .map(|link| SocialNetworkLink::from(link.clone()))
                .collect(),
        }
    }

    pub fn set_no_index(&mut self) {
        self.no_index = false;
    }

    pub fn set_no_search(&mut self) {
        self.disable_search = false;
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn build_date(&self) -> &str {
        &self.build_date
    }

    pub fn no_index(&self) -> bool {
        self.no_index
    }

    pub fn image_url(&self) -> String {
        if self.image.url().starts_with("http") {
            self.image.url().to_owned()
        } else {
            format!("{}/{}", self.url(), self.image.url())
        }
    }

    pub fn image_alt(&self) -> &str {
        self.image.alt()
    }

    pub fn disable_search(&self) -> bool {
        self.disable_search
    }

    pub fn navigation_links(&self) -> &[NavigationLink] {
        &self.navigation_links
    }

    pub fn social_links(&self) -> &[SocialNetworkLink] {
        &self.social_links
    }
}
