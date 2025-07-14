use tracing::info;
use anyhow::Result;
use serde::Serialize;
use async_trait::async_trait;

#[async_trait]
pub trait SpiderMiddleware: Send + Sync {
    async fn process_spider_input(&self, _response: &str, _spider_name: &str) -> Result<()> {
        Ok(())
    }

    async fn process_spider_output<T: Serialize + Send>(
        &self,
        items: Vec<T>,
        _spider_name: &str
    ) -> Result<Vec<T>> {
        Ok(items)
    }

    async fn process_spider_exception(
        &self,
        _error: &anyhow::Error,
        _spider_name: &str
    ) -> Result<()> {
        Ok(())
    }
}

pub struct LinkedinSpiderMiddleware;

impl Default for LinkedinSpiderMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl LinkedinSpiderMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SpiderMiddleware for LinkedinSpiderMiddleware {
    async fn process_spider_input(&self, _response: &str, _spider_name: &str) -> Result<()> {
        Ok(())
    }

    async fn process_spider_output<T: Serialize + Send>(
        &self,
        items: Vec<T>,
        _spider_name: &str
    ) -> Result<Vec<T>> {
        Ok(items)
    }

    async fn process_spider_exception(
        &self,
        error: &anyhow::Error,
        spider_name: &str
    ) -> Result<()> {
        info!("Spider {} encountered error: {}", spider_name, error);
        Ok(())
    }
}
