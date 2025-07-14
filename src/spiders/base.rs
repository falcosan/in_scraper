use anyhow::Result;
use std::sync::Arc;
use serde::Serialize;
use crate::config::Config;
use async_trait::async_trait;
use crate::utils::HttpClient;

#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub meta: std::collections::HashMap<String, String>,
}

impl Request {
    pub fn new(url: String) -> Self {
        Self {
            url,
            meta: std::collections::HashMap::new(),
        }
    }

    pub fn with_meta(mut self, key: String, value: String) -> Self {
        self.meta.insert(key, value);
        self
    }
}

#[async_trait]
pub trait Spider: Send + Sync + Clone {
    type Item: Serialize + Send;

    fn name(&self) -> &str;

    fn get_config(&self) -> &Arc<Config>;

    async fn start_requests(&self) -> Vec<Request>;

    async fn parse(
        &self,
        response: String,
        request: &Request
    ) -> Result<(Vec<Self::Item>, Vec<Request>)>;

    async fn execute_request(&self, request: Request) -> Result<(Vec<Self::Item>, Vec<Request>)> {
        let http_client = self.get_http_client();
        let response = http_client.get_text(&request.url).await?;
        self.parse(response, &request).await
    }

    fn get_http_client(&self) -> &HttpClient;
}
