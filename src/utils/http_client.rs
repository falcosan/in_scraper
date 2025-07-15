// use std::env;
// use dotenv::dotenv;
use std::sync::Arc;
use tokio::time::sleep;
use std::time::Duration;
use crate::config::Config;
use tracing::{ error, warn };
use anyhow::{ Result, Context };
use reqwest::{ Client, Response, StatusCode };

pub struct HttpClient {
    client: Client,
    config: Arc<Config>,
    // li_at_cookie: String,
    // jsession_id_cookie: String,
}

impl HttpClient {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        // dotenv().ok();
        // let li_at_cookie = env
        //     ::var("LINKEDIN_COOKIE_LI_AT")
        //     .context("LINKEDIN_COOKIE_LI_AT variable not set in .env file")?;

        // let jsession_id_cookie = env
        //     ::var("LINKEDIN_COOKIE_JSESSIONID")
        //     .context("LINKEDIN_COOKIE_JSESSIONID variable not set in .env file")?;

        let client = Client::builder()
            .timeout(Duration::from_secs(config.request_timeout))
            .user_agent(&config.user_agent)
            .gzip(true)
            .pool_max_idle_per_host(10)
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            config,
            // li_at_cookie,
            // jsession_id_cookie
        })
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        // let cookie_header = format!(
        //     "li_at={}; jsessionid={}",
        //     self.li_at_cookie,
        //     self.jsession_id_cookie
        // );
        self.execute_with_retry(||
            self.client
                .get(
                    format!("https://proxy.scrapeops.io/v1/?api_key=1a816100-42cc-4945-870a-6a82f2a88674&url={url}")
                )
                // .header("cookie", &cookie_header)
                // .header("referer", "https://www.linkedin.com/feed/")
                // .header("accept", "application/vnd.linkedin.normalized+json+2.1")
                // .header("csrf-token", self.jsession_id_cookie.replace("\"", ""))
                .send()
        ).await
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
                        return Ok(response);
                    }

                    if status == StatusCode::TOO_MANY_REQUESTS {
                        warn!("Rate limited (429), retrying after delay");
                        if retries < max_retries {
                            retries += 1;
                            sleep(retry_delay * retries).await;
                            continue;
                        }
                    }

                    if status.is_server_error() {
                        error!("Server error: {}, retrying", status);
                        if retries < max_retries {
                            retries += 1;
                            sleep(retry_delay * retries).await;
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
                        sleep(retry_delay * retries).await;
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
        Self::new(self.config.clone()).expect("Failed to clone HttpClient")
    }
}
