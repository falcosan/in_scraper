use tracing::info;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::{ Request, Response };

#[async_trait]
pub trait DownloaderMiddleware: Send + Sync {
    async fn process_request(
        &self,
        request: Request,
        _spider_name: &str
    ) -> Result<Option<Request>> {
        Ok(Some(request))
    }

    async fn process_response(&self, response: Response, _spider_name: &str) -> Result<Response> {
        Ok(response)
    }

    async fn process_exception(&self, _error: &anyhow::Error, _spider_name: &str) -> Result<()> {
        Ok(())
    }
}

pub struct LinkedinDownloaderMiddleware;

impl Default for LinkedinDownloaderMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl LinkedinDownloaderMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DownloaderMiddleware for LinkedinDownloaderMiddleware {
    async fn process_request(
        &self,
        request: Request,
        _spider_name: &str
    ) -> Result<Option<Request>> {
        Ok(Some(request))
    }

    async fn process_response(&self, response: Response, _spider_name: &str) -> Result<Response> {
        Ok(response)
    }

    async fn process_exception(&self, error: &anyhow::Error, spider_name: &str) -> Result<()> {
        info!("Downloader for spider {} encountered error: {}", spider_name, error);
        Ok(())
    }
}
