use std::sync::Arc;
use std::time::Duration;
use crate::config::Config;
use tracing::{ debug, error, warn };
use anyhow::{ Result, Context };
use reqwest::{ Client, Response, StatusCode };
use tokio::time::sleep;

pub struct HttpClient {
    client: Client,
    config: Arc<Config>,
}

impl HttpClient {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.request_timeout))
            .user_agent(&config.user_agent)
            .gzip(true)
            .pool_max_idle_per_host(10)
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self { client, config })
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        self.execute_with_retry(|| self.client.get(url).send()).await
    }

    pub async fn get_text(&self, url: &str) -> Result<String> {
        let response = self.get(url).await?;
        let text = response.text().await.context("Failed to get response text")?;
        Ok(text)
    }

    async fn execute_with_retry<F, Fut>(&self, request_fn: F) -> Result<Response>
        where F: Fn() -> Fut, Fut: std::future::Future<Output = Result<Response, reqwest::Error>>
    {
        let mut retries = 0;
        let max_retries = self.config.max_retries;
        let retry_delay = Duration::from_millis(self.config.retry_delay_ms);

        loop {
            match request_fn().await {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        debug!("Successfully fetched URL");
                        return Ok(response);
                    }

                    if status == StatusCode::TOO_MANY_REQUESTS {
                        warn!("Rate limited (429), retrying after delay");
                        if retries < max_retries {
                            retries += 1;
                            sleep(retry_delay * (retries as u32)).await;
                            continue;
                        }
                    }

                    if status.is_server_error() {
                        error!("Server error: {}, retrying", status);
                        if retries < max_retries {
                            retries += 1;
                            sleep(retry_delay * (retries as u32)).await;
                            continue;
                        }
                    }

                    if status.is_client_error() {
                        error!("Client error: {} - this might be a permanent issue", status);
                        return Err(anyhow::anyhow!("HTTP client error: {}", status));
                    }

                    return Ok(response);
                }
                Err(e) => {
                    error!("Request failed: {}", e);

                    if retries < max_retries {
                        retries += 1;
                        warn!("Retrying request (attempt {}/{})", retries, max_retries + 1);
                        sleep(retry_delay * (retries as u32)).await;
                        continue;
                    }

                    return Err(
                        anyhow::anyhow!("Request failed after {} retries: {}", max_retries, e)
                    );
                }
            }
        }
    }

    pub async fn get_with_headers(
        &self,
        url: &str,
        headers: Vec<(&str, &str)>
    ) -> Result<Response> {
        let mut request = self.client.get(url);

        for (key, value) in headers {
            request = request.header(key, value);
        }

        self.execute_with_retry(|| request.try_clone().unwrap().send()).await
    }
}

impl Clone for HttpClient {
    fn clone(&self) -> Self {
        Self::new(self.config.clone()).expect("Failed to clone HttpClient")
    }
}
