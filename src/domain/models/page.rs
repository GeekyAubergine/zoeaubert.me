use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use crate::{build_data::BUILD_DATE, infrastructure::utils::paginator::PaginatorPage};

use super::{
    site_config::{NavigationLink, PageImage, SocialNetworkLink, SITE_CONFIG},
    slug::Slug,
    tag::Tag,
};

#[derive(Debug, Clone)]
pub struct PagePaginationLabel {
    pub url: Slug,
    pub label: String,
}

impl PagePaginationLabel {
    pub fn new(url: &Slug, title: &str) -> Self {
        Self {
            url: url.to_owned(),
            label: title.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PagePagination {
    pub next: Option<PagePaginationLabel>,
    pub previous: Option<PagePaginationLabel>,
}

impl PagePagination {
    pub fn new(next: Option<PagePaginationLabel>, prev: Option<PagePaginationLabel>) -> Self {
        Self {
            next,
            previous: prev,
        }
    }

    pub fn from_slug_and_pagniator_page<'d, D>(
        slug: &Slug,
        page: &PaginatorPage<'d, D>,
        entity_name: &str,
    ) -> Self {
        let next = match page.has_next() {
            true => Some(PagePaginationLabel::new(
                &slug.append(&format!("page-{}", page.page_number + 1)),
                &format!("Newer {}", entity_name),
            )),
            false => None,
        };

        let prev = match page.has_previous() {
            true => Some(PagePaginationLabel::new(
                &slug.append(&format!("page-{}", page.page_number - 1)),
                &format!("Older {}", entity_name),
            )),
            false => None,
        };

        Self::new(next, prev)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageConfig {
    pub url: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub image: PageImage,
    pub language: String,
    pub navigation_links: Vec<NavigationLink>,
    pub social_links: Vec<SocialNetworkLink>,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub slug: Slug,
    pub title: String,
    pub description: String,
    pub author: String,
    pub image: PageImage,
    pub language: String,
    pub build_date: String,
    pub navigation_links: Vec<NavigationLink>,
    pub social_links: Vec<SocialNetworkLink>,
    pub heading: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub read_time: Option<String>,
    pub tags: Vec<Tag>,
    pub page_pagination: Option<PagePagination>,
}

impl Page {
    pub fn new(slug: Slug, title: Option<&str>, description: Option<&str>) -> Self {
        let heading = title.map(|t| t.to_owned());

        let title = match title {
            Some(t) => format!("{} | {}", t, SITE_CONFIG.title),
            None => SITE_CONFIG.title.clone(),
        };

        let description = match description {
            Some(d) => d.to_owned(),
            None => SITE_CONFIG.description.clone(),
        };

        Self {
            slug,
            title,
            description,
            author: SITE_CONFIG.author.to_string(),
            image: SITE_CONFIG.image.clone(),
            language: SITE_CONFIG.language.to_string(),
            build_date: BUILD_DATE.to_string(),
            navigation_links: SITE_CONFIG
                .navigation_links
                .iter()
                .map(|link| NavigationLink::from(link.clone()))
                .collect(),
            social_links: SITE_CONFIG
                .social_links
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

    pub fn with_image(mut self, image: PageImage) -> Self {
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

    pub fn with_pagination_from_paginator<'d, D>(
        mut self,
        paginator_page: &PaginatorPage<'d, D>,
        entity_name: &str,
    ) -> Self {
        let pagination =
            PagePagination::from_slug_and_pagniator_page(&self.slug, paginator_page, entity_name);

        self.page_pagination = Some(pagination);

        if paginator_page.page_number > 1 {
            self.slug = self.slug.append(&format!("page-{}", paginator_page.page_number));
        }

        self
    }

    pub fn hide_heading(&mut self) {
        self.heading = None;
    }

    pub fn permalink(&self) -> String {
        self.slug.permalink()
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

    pub fn image(&self) -> &PageImage {
        &self.image
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn build_date(&self) -> &str {
        &self.build_date
    }

    pub fn image_url(&self) -> &str {
        &self.image.url
    }

    pub fn image_alt(&self) -> &str {
        &self.image.alt
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
