use std::path::Path;
use std::sync::Arc;

use rayon::prelude::*;

use askama::{DynTemplate, Template};
use futures::lock::Mutex;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::domain::models::slug::Slug;
use crate::domain::repositories::Profiler;
use crate::domain::services::{FileService, PageRenderingService};
use crate::domain::{models::page::Page, state::State};

use crate::error::TemplateError;
use crate::prelude::*;

#[derive(Clone)]
struct RenderingJob {
    slug: Slug,
    render_fn: Arc<dyn Fn() -> Result<String> + Send + Sync>,
}

struct FileWriteJob {
    slug: Slug,
    content: String,
}

pub struct PageRenderingServiceImpl {
    rendering_jobs: Arc<RwLock<Vec<RenderingJob>>>,
}

impl PageRenderingServiceImpl {
    pub fn new() -> Self {
        Self {
            rendering_jobs: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl<'l> PageRenderingService for PageRenderingServiceImpl {
    async fn add_page<T>(&self, state: &impl State, slug: Slug, template: T) -> Result<()>
    where
        T: Template + Send + Sync + 'static,
    {
        let job = RenderingJob {
            slug,
            render_fn: Arc::new(move || template.render().map_err(TemplateError::render_error)),
        };

        self.rendering_jobs.write().await.push(job);

        Ok(())
    }

    async fn render_pages(&self, state: &impl State) -> Result<()> {
        let jobs = self.rendering_jobs.read().await;

        println!("Building {} pages", jobs.len());

        let render_start = std::time::Instant::now();

        let mut file_writes: Vec<FileWriteJob> = jobs
            .iter()
            .cloned()
            .par_bridge()
            .map(|job| {
                let rendered = (job.render_fn)()?;

                Ok(FileWriteJob {
                    slug: job.slug.clone(),
                    content: rendered,
                })
            })
            .collect::<Result<Vec<FileWriteJob>>>()?;

        let render_elapsed = render_start.elapsed();

        let write_start = std::time::Instant::now();

        for job in file_writes.iter() {
            debug!("Writing page: {}", job.slug.relative_link());

            let path = format!("{}index.html", job.slug.relative_link());

            let path = state
                .file_service()
                .make_output_file_path(&Path::new(&path));

            state
                .file_service()
                .write_text_file_blocking(&path, &job.content)
                .await?;
        }

        let write_elapsed = write_start.elapsed();

        state
            .profiler()
            .set_page_write_duration(write_elapsed)
            .await?;
        state
            .profiler()
            .set_page_rendering_duration(render_elapsed)
            .await?;
        state
            .profiler()
            .set_number_of_pages_written(file_writes.len() as u32)
            .await?;

        Ok(())
    }
}
