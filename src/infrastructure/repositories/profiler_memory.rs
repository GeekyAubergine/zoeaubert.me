use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::repositories::Profiler;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct ProfilerBlock {
    started_at: Option<std::time::Instant>,
    finished_at: Option<std::time::Instant>,
    entities_processed: u64,
}

#[derive(Debug, Default)]
pub struct ProfierData {
    posts: ProfilerBlock,
    pages: ProfilerBlock,
    queue: ProfilerBlock,
}

pub struct ProfilerMemory {
    data: Arc<RwLock<ProfierData>>,
}

impl ProfilerMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(ProfierData::default())),
        }
    }
}

#[async_trait::async_trait]
impl Profiler for ProfilerMemory {
    async fn post_processing_started(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.posts.started_at = Some(std::time::Instant::now());
        Ok(())
    }

    async fn post_processed(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.posts.entities_processed += 1;
        Ok(())
    }

    async fn post_processing_finished(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.posts.finished_at = Some(std::time::Instant::now());
        Ok(())
    }

    async fn page_generation_started(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.pages.started_at = Some(std::time::Instant::now());
        Ok(())
    }

    async fn page_generated(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.pages.entities_processed += 1;
        Ok(())
    }

    async fn page_generation_finished(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.pages.finished_at = Some(std::time::Instant::now());
        Ok(())
    }

    async fn queue_processing_started(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.queue.started_at = Some(std::time::Instant::now());
        Ok(())
    }

    async fn queue_processed(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.queue.entities_processed += 1;
        Ok(())
    }

    async fn queue_processing_finished(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.queue.finished_at = Some(std::time::Instant::now());
        Ok(())
    }

    async fn print_results(&self) -> Result<()> {
        let data = self.data.read().await;

        let mut total_duration = std::time::Duration::new(0, 0);

        if let Some(started_at) = data.posts.started_at {
            if let Some(finished_at) = data.posts.finished_at {
                let posts_duration = finished_at.duration_since(started_at);
                println!("Posts processed: {}", data.posts.entities_processed);
                println!("Post processing duration: {:?}", posts_duration);
                total_duration += posts_duration;
            }
        }

        if let Some(started_at) = data.pages.started_at {
            if let Some(finished_at) = data.pages.finished_at {
                let page_gen_duration = finished_at.duration_since(started_at);
                println!("Pages generated: {}", data.pages.entities_processed);
                println!("Page generation duration: {:?}", page_gen_duration);
                total_duration += page_gen_duration;
            }
        }

        if let Some(started_at) = data.queue.started_at {
            if let Some(finished_at) = data.queue.finished_at {
                let queue_duration = finished_at.duration_since(started_at);
                println!("Queue processed: {}", data.queue.entities_processed);
                println!("Queue processing duration: {:?}", queue_duration);
                total_duration += finished_at.duration_since(started_at);
            }
        }

        println!("Overall duration: {:?}", total_duration);

        Ok(())
    }
}
