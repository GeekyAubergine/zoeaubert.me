use crate::{
    domain::models::{page::Page, slug::Slug},
    error::{Error, FileSystemError, TemplateError},
    prelude::*,
    services::{
        file_service::{WritableFile},
        ServiceContext,
    },
};

use askama::Template;
use chrono::{DateTime, Utc};
use hypertext::Rendered;
use rayon::iter::ParallelBridge;
use tracing::debug;

use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use crate::renderer::formatters::format_date::FormatDate;

use super::file_service::FileService;

#[derive(Debug, Clone)]
struct SiteMapPage {
    url: String,
    last_modified: Option<DateTime<Utc>>,
}

#[derive(Template)]
#[template(path = "sitemap.xml")]
pub struct SitemapTemplate {
    pages: Vec<SiteMapPage>,
}

pub struct PageRenderer {
    site_map_pages: Arc<RwLock<Vec<SiteMapPage>>>,
}

impl PageRenderer {
    pub fn new() -> Self {
        Self {
            site_map_pages: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn render_page(
        &self,
        slug: &Slug,
        rendered: Rendered<String>,
        last_modified: Option<DateTime<Utc>>,
    ) -> Result<()> {
        debug!("Rendering page: {}", slug);

        let path = format!("{}index.html", slug.relative_link());

        let rendered = rendered.as_str();

        self.save_file(&path, &rendered)?;

        self.site_map_pages
            .write()
            .map_err(|_| Error::Unknown())?
            .push(SiteMapPage {
                url: slug.permalink(),
                last_modified,
            });

        Ok(())
    }

    pub fn render_file<'t, T>(&self, path: PathBuf, rendered: Rendered<String>) -> Result<()> {
        let path = path.to_string_lossy().to_string();

        self.save_file(&path, &rendered.as_str())?;

        Ok(())
    }

    pub fn build_sitemap(&self, disallowed_routes: &[String]) -> Result<()> {
        let pages = self
            .site_map_pages
            .read()
            .map_err(|_| Error::Unknown())?
            .clone();

        let pages: Vec<SiteMapPage> = pages
            .iter()
            .filter(|page| {
                !disallowed_routes
                    .iter()
                    .any(|disallowed| page.url.contains(disallowed))
            })
            .cloned()
            .collect();

        let template = SitemapTemplate {
            pages: pages.clone(),
        };

        let rendered = template.render().map_err(TemplateError::render_error)?;

        self.save_file("sitemap.xml", &rendered)
    }

    fn save_file(&self, path: &str, rendered: &str) -> Result<()> {
        FileService::output(PathBuf::from(path)).write_text(&rendered)
    }
}
