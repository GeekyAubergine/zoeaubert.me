use std::sync::Arc;
use std::time::Duration;

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
    entities: ProfilerBlock,
    page_generation_duration: Option<Duration>,
    render_duration: Option<Duration>,
    write_duration: Option<Duration>,
    number_of_pages_written: Option<u32>,
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
    async fn entity_processing_started(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.entities.started_at = Some(std::time::Instant::now());
        Ok(())
    }

    async fn entity_processed(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.entities.entities_processed += 1;
        Ok(())
    }

    async fn entity_processing_finished(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.entities.finished_at = Some(std::time::Instant::now());
        Ok(())
    }

    // async fn print_entity_processing_results(&self) -> Result<()> {
    //     let data = self.data.read().await;

    //     if let Some(started_at) = data.entities.started_at {
    //         if let Some(finished_at) = data.entities.finished_at {
    //             let duration = finished_at.duration_since(started_at);
    //             println!("Entities processed: {}", data.entities.entities_processed);
    //             println!("Entity processing duration: {:?}", duration);
    //         }
    //     }

    //     Ok(())
    // }

    // async fn page_generation_started(&self) -> Result<()> {
    //     let mut data = self.data.write().await;
    //     data.pages.started_at = Some(std::time::Instant::now());
    //     Ok(())
    // }

    // async fn page_generated(&self) -> Result<()> {
    //     let mut data = self.data.write().await;
    //     data.pages.entities_processed += 1;
    //     Ok(())
    // }

    // async fn page_generation_finished(&self) -> Result<()> {
    //     let mut data = self.data.write().await;
    //     data.pages.finished_at = Some(std::time::Instant::now());
    //     Ok(())
    // }

    // async fn queue_processing_started(&self) -> Result<()> {
    //     let mut data = self.data.write().await;
    //     data.queue.started_at = Some(std::time::Instant::now());
    //     Ok(())
    // }

    // async fn queue_processed(&self) -> Result<()> {
    //     let mut data = self.data.write().await;
    //     data.queue.entities_processed += 1;
    //     Ok(())
    // }

    // async fn queue_processing_finished(&self) -> Result<()> {
    //     let mut data = self.data.write().await;
    //     data.queue.finished_at = Some(std::time::Instant::now());
    //     Ok(())
    // }

    async fn set_page_generation_duration(&self, duration: Duration) -> Result<()> {
        let mut data = self.data.write().await;
        data.page_generation_duration = Some(duration);
        Ok(())
    }

    async fn set_page_rendering_duration(&self, duration: Duration) -> Result<()> {
        let mut data = self.data.write().await;
        data.render_duration = Some(duration);
        Ok(())
    }

    async fn set_page_write_duration(&self, duration: Duration) -> Result<()> {
        let mut data = self.data.write().await;
        data.write_duration = Some(duration);
        Ok(())
    }

    async fn set_number_of_pages_written(&self, number_of_pages: u32) -> Result<()> {
        let mut data = self.data.write().await;
        data.number_of_pages_written = Some(number_of_pages);
        Ok(())
    }

    async fn print_results(&self) -> Result<()> {
        let data = self.data.read().await;

        let mut total_duration = Duration::new(0, 0);

        if let Some(started_at) = data.entities.started_at {
            if let Some(finished_at) = data.entities.finished_at {
                let posts_duration = finished_at.duration_since(started_at);
                println!("Entities processed: {}", data.entities.entities_processed);
                println!("Entity processing duration: {:?}", posts_duration);
                total_duration += posts_duration;
            }
        }

        if let Some(page_generation_duration) = data.page_generation_duration {
            println!("Page generation duration: {:?}", page_generation_duration);
            total_duration += page_generation_duration;
        }

        if let Some(render_duration) = data.render_duration {
            println!("Page rendering duration: {:?}", render_duration);
            total_duration += render_duration;
        }

        if let Some(write_duration) = data.write_duration {
            println!("Page write duration: {:?}", write_duration);
            total_duration += write_duration;
        }

        if let Some(number_of_pages_written) = data.number_of_pages_written {
            println!("Number of pages written: {}", number_of_pages_written);
        }

        println!("Overall duration: {:?}", total_duration);

        Ok(())
    }
}
