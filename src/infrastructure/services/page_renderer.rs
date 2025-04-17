use crate::{
    domain::{
        models::{page::Page, slug::Slug},
        repositories::Profiler,
        services::FileService,
        state::State,
    },
    error::TemplateError,
    prelude::*,
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

use crate::infrastructure::renderers::formatters::format_date::FormatDate;

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
        state: &impl State,
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

        self.save_file(state, &path, &rendered).await?;

        self.site_map_pages.write().await.push(SiteMapPage {
            url: slug.permalink(),
            last_modified,
        });

        Ok(())
    }

    pub async fn render_file<'t, T>(&self, state: &impl State, path: PathBuf, template: &'t T) -> Result<()>
    where
        T: Template,
    {
        let path = path.to_string_lossy().to_string();

        let rendered = template.render().map_err(TemplateError::render_error)?;

        self.save_file(state, &path, &rendered).await?;

        Ok(())
    }

    async fn save_file(&self, state: &impl State, path: &str, rendered: &str) -> Result<()> {
        let path = state
            .file_service()
            .make_output_file_path(&Path::new(&path));

        state
            .file_service()
            .write_text_file_blocking(&path, rendered)
            .await
    }

    pub async fn build_sitemap(
        &self,
        state: &impl State,
        disallowed_routes: &[String],
    ) -> Result<()> {
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

        let path = state
            .file_service()
            .make_output_file_path(&Path::new("sitemap.xml"));

        state
            .file_service()
            .write_text_file_blocking(&path, &rendered)
            .await?;

        Ok(())
    }
}
