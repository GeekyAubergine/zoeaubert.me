use chrono::{format, DateTime, Utc};

use crate::{
    build_data::BUILD_DATE,
    infrastructure::config::{SiteConfig, SiteConfigNavLink, SiteConfigSocialNetworkLink},
};

use super::{media::image::Image, tag::Tag};

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

pub struct PagePaginationLabel {
    url: String,
    label: String,
}

impl PagePaginationLabel {
    pub fn new(url: &str, title: &str) -> Self {
        Self {
            url: url.to_owned(),
            label: title.to_owned(),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

pub struct PagePagination {
    next: Option<PagePaginationLabel>,
    previous: Option<PagePaginationLabel>,
}

impl PagePagination {
    pub fn new(next: Option<PagePaginationLabel>, prev: Option<PagePaginationLabel>) -> Self {
        Self {
            next,
            previous: prev,
        }
    }

    pub fn next(&self) -> Option<&PagePaginationLabel> {
        self.next.as_ref()
    }

    pub fn previous(&self) -> Option<&PagePaginationLabel> {
        self.previous.as_ref()
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
    navigation_links: Vec<NavigationLink>,
    social_links: Vec<SocialNetworkLink>,
    heading: Option<String>,
    date: Option<DateTime<Utc>>,
    read_time: Option<String>,
    tags: Vec<Tag>,
    page_pagination: Option<PagePagination>,
}

impl Page {
    pub fn new(
        site: &SiteConfig,
        slug: &str,
        title: Option<&str>,
        description: Option<&str>,
        // image: Option<Image>,
        // date: Option<DateTime<Utc>>,
        // read_time: Option<String>,
        // tags: Vec<Tag>,
    ) -> Self {
        let heading = title.map(|t| t.to_owned());

        let title = match title {
            Some(t) => format!("{} | {}", t, site.title()),
            None => site.title().to_owned(),
        };

        let description = match description {
            Some(d) => d.to_owned(),
            None => site.description().to_owned(),
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
            image: site.image().clone(),
            language: site.language().to_owned(),
            build_date: BUILD_DATE.to_string(),
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
            heading,
            date: None,
            read_time: None,
            tags: vec![],
            page_pagination: None,
        }
    }

    pub fn with_image(mut self, image: Image) -> Self {
        self.image = image;
        self
    }

    pub fn with_date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }

    pub fn with_read_time(mut self, read_time: &str) -> Self {
        self.read_time = Some(read_time.to_owned());
        self
    }

    pub fn with_tags(mut self, tags: Vec<Tag>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_pagination(mut self, pagination: PagePagination) -> Self {
        self.page_pagination = Some(pagination);
        self
    }

    pub fn hide_heading(&mut self) {
        self.heading = None;
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

    pub fn navigation_links(&self) -> &[NavigationLink] {
        &self.navigation_links
    }

    pub fn social_links(&self) -> &[SocialNetworkLink] {
        &self.social_links
    }

    pub fn hide_header(&self) -> bool {
        self.heading.is_none()
    }

    pub fn heading(&self) -> Option<&str> {
        self.heading.as_deref()
    }

    pub fn date(&self) -> Option<&DateTime<Utc>> {
        self.date.as_ref()
    }

    pub fn read_time(&self) -> Option<&str> {
        self.read_time.as_deref()
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn page_pagination(&self) -> Option<&PagePagination> {
        self.page_pagination.as_ref()
    }
}
