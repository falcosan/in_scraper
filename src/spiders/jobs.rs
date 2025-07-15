use tracing::info;
use anyhow::Result;
use std::sync::Arc;
use urlencoding::encode;
use async_trait::async_trait;
use scraper::{ Html, Selector };
use crate::{ config::Config, items::JobListing, spiders::{ Spider, Request }, utils::HttpClient };

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
        vec![Request::new(self.build_url(0)).with_meta("page".to_string(), "0".to_string())]
    }

    async fn parse(
        &self,
        response: String,
        request: &Request
    ) -> Result<(Vec<Self::Item>, Vec<Request>)> {
        let page = request.meta
            .get("page")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        let document = Html::parse_document(&response);

        let job_selector = Selector::parse(crate::selectors::JobSelectors::ITEM).unwrap();
        let title_selector = Selector::parse(crate::selectors::JobSelectors::TITLE).unwrap();
        let url_selector = Selector::parse(crate::selectors::JobSelectors::URL).unwrap();
        let time_selector = Selector::parse(crate::selectors::JobSelectors::TIME).unwrap();
        let company_name_selector = Selector::parse(
            crate::selectors::JobSelectors::COMPANY_NAME
        ).unwrap();
        let location_selector = Selector::parse(crate::selectors::JobSelectors::LOCATION).unwrap();

        let jobs: Vec<_> = document.select(&job_selector).collect();
        info!("Number of jobs returned: {}", jobs.len());

        let mut items = Vec::new();
        for job in &jobs {
            let job_title = job
                .select(&title_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "not-found".to_string());

            let job_detail_url = job
                .select(&url_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .unwrap_or("not-found")
                .to_string();

            let job_listed = job
                .select(&time_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "not-found".to_string());

            let company_element = job.select(&company_name_selector).next();
            let company_name = company_element
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "not-found".to_string());

            let company_link = company_element
                .and_then(|el| el.value().attr("href"))
                .unwrap_or("not-found")
                .to_string();

            let company_location = job
                .select(&location_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "not-found".to_string());

            items.push(JobListing {
                job_title,
                job_detail_url,
                job_listed,
                company_name,
                company_link,
                company_location,
            });
        }

        let mut next_requests = Vec::new();
        if !jobs.is_empty() {
            let next_page = page + 1;
            next_requests.push(
                Request::new(self.build_url(next_page)).with_meta(
                    "page".to_string(),
                    next_page.to_string()
                )
            );
        }

        Ok((items, next_requests))
    }
}
