use crate::{
    domain::models::{page::Page, slug::Slug},
    error::TemplateError,
    prelude::*,
    services::{file_service::{File, FilePath}, ServiceContext},
};

use askama::Template;
use chrono::{DateTime, Utc};
use rayon::iter::ParallelBridge;
use tracing::debug;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::renderers::formatters::format_date::FormatDate;

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

    pub async fn render_page<'t, T>(
        &self,
        slug: &Slug,
        template: &'t T,
        last_modified: Option<DateTime<Utc>>,
    ) -> Result<()>
    where
        T: Template,
    {
        debug!("Rendering page: {}", slug);

        let path = format!("{}index.html", slug.relative_link());

        let rendered = template.render().map_err(TemplateError::render_error)?;

        self.save_file(&path, &rendered).await?;

        self.site_map_pages.write().await.push(SiteMapPage {
            url: slug.permalink(),
            last_modified,
        });

        Ok(())
    }

    pub async fn render_file<'t, T>(&self, path: PathBuf, template: &'t T) -> Result<()>
    where
        T: Template,
    {
        let path = path.to_string_lossy().to_string();

        let rendered = template.render().map_err(TemplateError::render_error)?;

        self.save_file(&path, &rendered).await?;

        Ok(())
    }

    pub async fn build_sitemap(&self, disallowed_routes: &[String]) -> Result<()> {
        let pages = self.site_map_pages.read().await;

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

        self.save_file("sitemap.xml", &rendered).await
    }

    async fn save_file(&self, path: &str, rendered: &str) -> Result<()> {
        File::from_path(FilePath::output(path)).write_text(&rendered).await
    }
}
