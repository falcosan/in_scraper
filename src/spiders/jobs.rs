use tracing::info;
use anyhow::Result;
use std::sync::Arc;
use urlencoding::encode;
use async_trait::async_trait;
use std::collections::HashSet;
use scraper::{ Html, Selector, ElementRef };
use htmlentity::entity::{ decode, ICodedDataTrait };
use crate::{
    config::Config,
    items::JobListing,
    spiders::{ Spider, Request },
    utils::{ selector_utils::parse_selector, HttpClient },
};

#[derive(Clone)]
pub struct JobsSpider {
    config: Arc<Config>,
    http_client: HttpClient,
    keywords: String,
    location: String,
}

impl JobsSpider {
    pub fn new(config: Arc<Config>, keywords: String, location: String) -> Self {
        let http_client = HttpClient::new(config.clone()).expect("Failed to create HTTP client");
        Self {
            config,
            http_client,
            keywords,
            location,
        }
    }

    fn build_url(&self, start: usize) -> String {
        format!(
            "https://www.linkedin.com/jobs-guest/jobs/api/seeMoreJobPostings/search?keywords={}&location={}&start={}",
            encode(&self.keywords),
            encode(&self.location),
            start
        )
    }

    fn truncate_url_params<'a>(&self, url: &'a str) -> &'a str {
        if let Some(pos) = url.find('?') { &url[..pos] } else { url }
    }

    fn extract_text(element: ElementRef, selector: &Selector) -> String {
        element
            .select(selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "not-found".to_string())
    }

    fn extract_href(element: ElementRef, selector: &Selector) -> String {
        element
            .select(selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .unwrap_or("not-found")
            .to_string()
    }
}

#[async_trait]
impl Spider for JobsSpider {
    type Item = JobListing;

    fn name(&self) -> &str {
        "linkedin_jobs"
    }

    fn get_config(&self) -> &Arc<Config> {
        &self.config
    }

    fn get_http_client(&self) -> &HttpClient {
        &self.http_client
    }

    async fn start_requests(&self) -> Vec<Request> {
        vec![Request::new(self.build_url(0)).with_meta("start".to_string(), "0".to_string())]
    }

    async fn parse(
        &self,
        response: String,
        request: &Request
    ) -> Result<(Vec<Self::Item>, Vec<Request>)> {
        let start_offset = request.meta
            .get("start")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        let decoded = decode(response.as_bytes());
        let document = Html::parse_document(&decoded.to_string().unwrap());

        let job_selector = parse_selector(crate::selectors::JobSelectors::ITEM);
        let title_selector = parse_selector(crate::selectors::JobSelectors::TITLE);
        let url_selector = parse_selector(crate::selectors::JobSelectors::URL);
        let time_selector = parse_selector(crate::selectors::JobSelectors::TIME);
        let company_name_selector = parse_selector(crate::selectors::JobSelectors::COMPANY_NAME);
        let location_selector = parse_selector(crate::selectors::JobSelectors::LOCATION);

        let mut items = Vec::new();
        let mut seen_urls = HashSet::new();

        let jobs: Vec<_> = document.select(&job_selector).collect();
        info!("Jobs found on page: {}", jobs.len());

        for job in jobs.iter() {
            let raw_url = Self::extract_href(*job, &url_selector);
            let truncated_url = self.truncate_url_params(&raw_url).to_string();

            if truncated_url == "not-found" || !seen_urls.insert(truncated_url.clone()) {
                continue;
            }

            items.push(JobListing {
                job_detail_url: truncated_url,
                job_title: Self::extract_text(*job, &title_selector),
                job_listed: Self::extract_text(*job, &time_selector),
                company_name: Self::extract_text(*job, &company_name_selector),
                company_link: Self::extract_href(*job, &company_name_selector),
                company_location: Self::extract_text(*job, &location_selector),
            });
        }

        info!("Unique jobs collected: {}", items.len());

        let mut next_requests = Vec::new();
        if !jobs.is_empty() {
            let next_start = start_offset + jobs.len();
            info!("Requesting next page with start offset: {}", next_start);
            next_requests.push(
                Request::new(self.build_url(next_start)).with_meta(
                    "start".to_string(),
                    next_start.to_string()
                )
            );
        }

        Ok((items, next_requests))
    }
}
