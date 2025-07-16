use std::time::Duration;
use tracing::{ info, error };
use futures::future::join_all;
use reqwest::{ Client, Proxy };
use anyhow::{ Result, Context };

pub struct ProxyValidator {
    test_url: String,
    timeout: Duration,
}

impl Default for ProxyValidator {
    fn default() -> Self {
        Self {
            test_url: "https://httpbin.org/ip".to_string(),
            timeout: Duration::from_secs(10),
        }
    }
}

impl ProxyValidator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_test_url(mut self, url: String) -> Self {
        self.test_url = url;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn validate_proxies(&self, proxies: &[String]) -> Vec<String> {
        if proxies.is_empty() {
            return Vec::new();
        }

        info!("Validating {} proxies...", proxies.len());

        let validation_tasks: Vec<_> = proxies
            .iter()
            .map(|proxy| self.validate_single_proxy(proxy))
            .collect();

        let results = join_all(validation_tasks).await;

        let valid_proxies: Vec<String> = results
            .into_iter()
            .filter_map(|result| result.ok())
            .collect();

        info!("Validated {}/{} proxies successfully", valid_proxies.len(), proxies.len());
        valid_proxies
    }

    async fn validate_single_proxy(&self, proxy_url: &str) -> Result<String> {
        let proxy = Proxy::all(proxy_url).context("Failed to create proxy")?;

        let client = Client::builder()
            .proxy(proxy)
            .timeout(self.timeout)
            .build()
            .context("Failed to build client with proxy")?;

        let response = client
            .get(&self.test_url)
            .send().await
            .context("Request through proxy failed")?;

        if response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            info!(
                "Proxy {} validated successfully. Response: {}",
                proxy_url,
                body.chars().take(100).collect::<String>()
            );
            Ok(proxy_url.to_string())
        } else {
            error!("Proxy {} returned status: {}", proxy_url, response.status());
            Err(anyhow::anyhow!("Proxy validation failed with status: {}", response.status()))
        }
    }

    pub async fn get_proxy_ip(&self, proxy_url: &str) -> Result<String> {
        let proxy = Proxy::all(proxy_url).context("Failed to create proxy")?;

        let client = Client::builder()
            .proxy(proxy)
            .timeout(self.timeout)
            .build()
            .context("Failed to build client with proxy")?;

        let response = client
            .get(&self.test_url)
            .send().await
            .context("Request through proxy failed")?;

        if response.status().is_success() {
            let body = response.text().await.context("Failed to get response body")?;
            Ok(body)
        } else {
            Err(anyhow::anyhow!("Failed to get IP through proxy: {}", response.status()))
        }
    }
}
