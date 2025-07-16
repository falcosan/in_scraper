use anyhow::Result;
use std::time::Duration;
use async_trait::async_trait;
use reqwest::{ Request, Response };
use tracing::{ info, warn, debug };

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

pub struct LinkedinDownloaderMiddleware {
    delay_between_requests: Duration,
}

impl Default for LinkedinDownloaderMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl LinkedinDownloaderMiddleware {
    pub fn new() -> Self {
        Self {
            delay_between_requests: Duration::from_millis(1000),
        }
    }

    pub fn with_delay(delay: Duration) -> Self {
        Self {
            delay_between_requests: delay,
        }
    }
}

#[async_trait]
impl DownloaderMiddleware for LinkedinDownloaderMiddleware {
    async fn process_request(
        &self,
        mut request: Request,
        spider_name: &str
    ) -> Result<Option<Request>> {
        debug!("Processing request for spider {}: {}", spider_name, request.url());

        if self.delay_between_requests > Duration::ZERO {
            tokio::time::sleep(self.delay_between_requests).await;
        }

        let headers = request.headers_mut();
        headers.insert(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
                .parse()
                .unwrap()
        );
        headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip, deflate".parse().unwrap());
        headers.insert("DNT", "1".parse().unwrap());
        headers.insert("Connection", "keep-alive".parse().unwrap());
        headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());

        Ok(Some(request))
    }

    async fn process_response(&self, response: Response, spider_name: &str) -> Result<Response> {
        let status = response.status();
        let url = response.url().clone();

        debug!("Response for spider {}: {} - {}", spider_name, status, url);

        if status.is_success() {
            info!("Request successful: {} - {}", status, url);
        } else if status.as_u16() == 429 {
            warn!("Rate limited response: {} - {}", status, url);
        } else if status.is_client_error() {
            warn!("Client error response: {} - {}", status, url);
        } else if status.is_server_error() {
            warn!("Server error response: {} - {}", status, url);
        }

        Ok(response)
    }

    async fn process_exception(&self, error: &anyhow::Error, spider_name: &str) -> Result<()> {
        warn!("Downloader middleware for spider {} encountered error: {}", spider_name, error);
        Ok(())
    }
}
