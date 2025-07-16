use std::sync::Arc;
use tokio::time::sleep;
use std::time::Duration;
use crate::config::Config;
use crate::utils::ProxyRotator;
use anyhow::{ Result, Context };
use tracing::{ error, warn, info, debug };
use reqwest::{ Client, Response, StatusCode, Proxy };

pub struct HttpClient {
    config: Arc<Config>,
    proxy_rotator: Option<Arc<ProxyRotator>>,
}

impl HttpClient {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let proxy_rotator = if !config.proxies.is_empty() {
            let rotator = Arc::new(ProxyRotator::new(config.proxies.clone()));
            info!("HTTP client initialized with {} proxies", rotator.proxy_count());
            Some(rotator)
        } else {
            info!("HTTP client initialized without proxies");
            None
        };

        Ok(Self {
            config,
            proxy_rotator,
        })
    }

    fn create_client_with_proxy(&self, proxy_url: Option<&str>) -> Result<Client> {
        let mut builder = Client::builder()
            .timeout(Duration::from_secs(self.config.request_timeout))
            .user_agent(&self.config.user_agent)
            .gzip(true)
            .pool_max_idle_per_host(0)
            .tcp_keepalive(Duration::from_secs(60))
            .connection_verbose(false);

        if let Some(proxy_url) = proxy_url {
            let proxy = Proxy::all(proxy_url).context("Failed to create proxy")?;
            builder = builder.proxy(proxy);
            debug!("HTTP client configured with proxy: {}", proxy_url);
        }

        builder.build().context("Failed to build HTTP client")
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        self.execute_with_retry(|| async {
            let proxy_url = self.get_next_proxy();
            let client = self.create_client_with_proxy(proxy_url.as_deref())?;

            if let Some(ref proxy) = proxy_url {
                debug!("Making request to {} using proxy: {}", url, proxy);
            } else {
                debug!("Making request to {} without proxy", url);
            }

            client
                .get(url)
                .send().await
                .map_err(|e| anyhow::anyhow!(e))
        }).await
    }

    pub async fn get_text(&self, url: &str) -> Result<String> {
        let response = self.get(url).await?;
        let text = response.text().await.context("Failed to get response text")?;
        Ok(text)
    }

    fn get_next_proxy(&self) -> Option<String> {
        self.proxy_rotator.as_ref().and_then(|r| r.get_next_proxy())
    }

    pub fn has_proxies(&self) -> bool {
        self.proxy_rotator.as_ref().is_some_and(|r| r.has_proxies())
    }

    pub fn proxy_count(&self) -> usize {
        self.proxy_rotator.as_ref().map_or(0, |r| r.proxy_count())
    }

    async fn execute_with_retry<F, Fut>(&self, request_fn: F) -> Result<Response>
        where
            F: Fn() -> Fut + Send + Sync,
            Fut: std::future::Future<Output = Result<Response>> + Send
    {
        let mut retries = 0;
        let max_retries = self.config.max_retries;
        let retry_delay = Duration::from_millis(self.config.retry_delay_ms);

        loop {
            match request_fn().await {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        return Ok(response);
                    }

                    if status == StatusCode::TOO_MANY_REQUESTS {
                        warn!("Rate limited (429), will retry with different proxy after delay");
                        if retries < max_retries {
                            retries += 1;
                            let delay = retry_delay * retries;
                            debug!(
                                "Waiting {}ms before retry {}/{}",
                                delay.as_millis(),
                                retries,
                                max_retries + 1
                            );
                            sleep(delay).await;
                            continue;
                        }
                    }

                    if status.is_server_error() {
                        error!("Server error: {}, will retry with different proxy", status);
                        if retries < max_retries {
                            retries += 1;
                            let delay = retry_delay * retries;
                            debug!(
                                "Waiting {}ms before retry {}/{}",
                                delay.as_millis(),
                                retries,
                                max_retries + 1
                            );
                            sleep(delay).await;
                            continue;
                        }
                    }

                    if status.is_client_error() {
                        error!("Client error: {}", status);
                        return Err(anyhow::anyhow!("HTTP client error: {}", status));
                    }

                    return Ok(response);
                }
                Err(e) => {
                    error!("Request failed: {}", e);

                    if retries < max_retries {
                        retries += 1;
                        let delay = retry_delay * retries;
                        warn!(
                            "Retrying request with different proxy (attempt {}/{}) after {}ms",
                            retries,
                            max_retries + 1,
                            delay.as_millis()
                        );
                        sleep(delay).await;
                        continue;
                    }

                    return Err(
                        anyhow::anyhow!("Request failed after {} retries: {}", max_retries, e)
                    );
                }
            }
        }
    }
}

impl Clone for HttpClient {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            proxy_rotator: self.proxy_rotator.clone(),
        }
    }
}
