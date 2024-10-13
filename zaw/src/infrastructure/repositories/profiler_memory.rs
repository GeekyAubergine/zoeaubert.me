use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::repositories::Profiler;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct ProfierData {
    posts_processed: u64,
    pages_generated: u64,
    start_time: Option<std::time::Instant>,
    end_time: Option<std::time::Instant>,
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
    async fn add_post_processed(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.posts_processed += 1;
        Ok(())
    }

    async fn add_page_generated(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.pages_generated += 1;
        Ok(())
    }

    async fn start_timer(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.start_time = Some(std::time::Instant::now());
        Ok(())
    }

    async fn stop_timer(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.end_time = Some(std::time::Instant::now());
        Ok(())
    }

    async fn print_results(&self) -> Result<()> {
        let data = self.data.read().await;
        let start_time = data.start_time.unwrap();
        let end_time = data.end_time.unwrap();
        let duration = end_time - start_time;

        println!("Posts processed: {}", data.posts_processed);
        println!("Pages generated: {}", data.pages_generated);
        println!("Duration: {:?}", duration);

        Ok(())
    }
}
