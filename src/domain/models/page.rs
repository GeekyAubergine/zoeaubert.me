use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use crate::{
    build_data::BUILD_DATE, renderer::formatters::format_date::FormatDate,
    utils::paginator::PaginatorPage,
};

use super::{
    mastodon_post::MastodonPost,
    media::Media,
    site_config::{HeaderLink, PageImage, PageLinkGroup, SocialNetworkLink, SITE_CONFIG},
    slug::Slug,
    tag::Tag,
};

#[derive(Debug, Clone)]
pub struct PagePaginationDataLink {
    pub index: usize,
    pub slug: Slug,
}

impl PagePaginationDataLink {
    pub fn new(index: usize, slug: Slug) -> Self {
        Self {
            index,
            slug: slug.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PagePaginationData {
    pub current_index: usize,
    pub first: PagePaginationDataLink,
    pub previous: PagePaginationDataLink,
    pub next: PagePaginationDataLink,
    pub last: PagePaginationDataLink,
}

impl PagePaginationData {
    pub fn new(
        index: usize,
        first: PagePaginationDataLink,
        previous: PagePaginationDataLink,
        next: PagePaginationDataLink,
        last: PagePaginationDataLink,
    ) -> Self {
        Self {
            current_index: index,
            first,
            previous,
            next,
            last,
        }
    }

    pub fn from_slug_and_pagniator_page<'d, D>(
        slug: &Slug,
        page: &PaginatorPage<'d, D>,
        entity_name: &str,
    ) -> Self {
        // let next = match page.has_next() {
        //     true => Some(PagePaginationDataLink::new(
        //         page.page_number + 1,
        //         slug.append(&format!("page-{}", page.page_number + 1)),
        //     )),
        //     false => None,
        // };

        let prev = match page.page_number {
            2 => PagePaginationDataLink::new(page.page_number - 1, slug.clone()),
            _ => PagePaginationDataLink::new(
                page.page_number - 1,
                slug.append(&format!("page-{}", page.page_number - 1)),
            ),
        };

        Self::new(
            page.page_number,
            PagePaginationDataLink::new(1, slug.clone()),
            prev,
            PagePaginationDataLink::new(
                page.page_number + 1,
                slug.append(&format!("page-{}", page.page_number + 1)),
            ),
            PagePaginationDataLink::new(
                page.total_pages,
                slug.append(&format!("page-{}", page.total_pages)),
            ),
        )
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
    pub navigation_links: Vec<HeaderLink>,
    pub social_links: Vec<SocialNetworkLink>,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub slug: Slug,
    pub title: Option<String>,
    pub description: String,
    pub author: String,
    pub image: PageImage,
    pub language: String,
    pub build_date: String,
    pub header_links: Vec<HeaderLink>,
    pub page_links: Vec<PageLinkGroup>,
    pub social_links: Vec<SocialNetworkLink>,
    pub date: Option<DateTime<Utc>>,
    pub read_time: Option<String>,
    pub tags: Vec<Tag>,
    pub page_pagination: Option<PagePaginationData>,
}

impl Page {
    pub fn new(slug: Slug, title: Option<String>, description: Option<String>) -> Self {
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
            header_links: SITE_CONFIG.header_links.clone(),
            page_links: SITE_CONFIG.page_links.clone(),
            social_links: SITE_CONFIG.social_links.clone(),
            date: None,
            read_time: None,
            tags: vec![],
            page_pagination: None,
        }
    }

    pub fn from_page_and_pagination_page<'d, D>(
        page: &Self,
        paginator_page: &PaginatorPage<'d, D>,
        entity_name: &str,
    ) -> Self {
        let mut page = page.clone();

        let pagination = PagePaginationData::from_slug_and_pagniator_page(
            &page.slug,
            paginator_page,
            entity_name,
        );

        page.page_pagination = Some(pagination);

        if paginator_page.page_number > 1 {
            page.slug = page
                .slug
                .append(&format!("page-{}", paginator_page.page_number));
        }

        page
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

    pub fn with_pagination(mut self, pagination: PagePaginationData) -> Self {
        self.page_pagination = Some(pagination);
        self
    }

    // pub fn with_pagination_from_paginator<'d, D>(
    //     mut self,
    //     paginator_page: &PaginatorPage<'d, D>,
    //     entity_name: &str,
    // ) -> Self {
    //     let pagination =
    //         PagePagination::from_slug_and_pagniator_page(&self.slug, paginator_page, entity_name);

    //     self.page_pagination = Some(pagination);

    //     if paginator_page.page_number > 1 {
    //         self.slug = self
    //             .slug
    //             .append(&format!("page-{}", paginator_page.page_number));
    //     }

    //     self
    // }

    pub fn permalink(&self) -> String {
        self.slug.permalink_string()
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

    pub fn header_links(&self) -> &[HeaderLink] {
        &self.header_links
    }

    pub fn social_links(&self) -> &[SocialNetworkLink] {
        &self.social_links
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

    pub fn page_pagination(&self) -> Option<&PagePaginationData> {
        self.page_pagination.as_ref()
    }

    pub fn first_half_of_header_links(&self) -> &[HeaderLink] {
        &self.header_links[0..self.header_links.len() / 2]
    }

    pub fn second_half_of_header_links(&self) -> &[HeaderLink] {
        &self.header_links[self.header_links.len() / 2..]
    }
}
