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
    api_url: String,
    keywords: String,
    location: String,
}

impl JobsSpider {
    pub fn new(config: Arc<Config>, keywords: String, location: String) -> Self {
        let http_client = HttpClient::new(config.clone()).expect("Failed to create HTTP client");

        let api_url = format!(
            "https://www.linkedin.com/jobs-guest/jobs/api/seeMoreJobPostings/search?keywords={}&location={}",
            encode(&keywords),
            encode(&location)
        );

        Self {
            config,
            http_client,
            api_url,
            keywords,
            location,
        }
    }

    pub fn get_keywords(&self) -> &str {
        &self.keywords
    }

    pub fn get_location(&self) -> &str {
        &self.location
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
        vec![
            Request::new(format!("{}{}", self.api_url, 0)).with_meta(
                "first_job_on_page".to_string(),
                "0".to_string()
            )
        ]
    }

    async fn parse(
        &self,
        response: String,
        request: &Request
    ) -> Result<(Vec<Self::Item>, Vec<Request>)> {
        let first_job_on_page = request.meta
            .get("first_job_on_page")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        let document = Html::parse_document(&response);
        let mut items = Vec::new();
        let mut next_requests = Vec::new();

        let job_selector = Selector::parse(crate::selectors::JOB_ITEM).unwrap();
        let jobs: Vec<_> = document.select(&job_selector).collect();

        info!("Number of jobs returned: {}", jobs.len());

        for job in &jobs {
            let title_selector = Selector::parse(crate::selectors::JOB_TITLE).unwrap();
            let url_selector = Selector::parse(crate::selectors::JOB_URL).unwrap();
            let time_selector = Selector::parse(crate::selectors::JOB_TIME).unwrap();
            let company_name_selector = Selector::parse(
                crate::selectors::JOB_COMPANY_NAME
            ).unwrap();
            let location_selector = Selector::parse(crate::selectors::JOB_LOCATION).unwrap();

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

        if !jobs.is_empty() {
            let next_page = first_job_on_page + 25;
            let next_url = format!("{}{}", self.api_url, next_page);
            let next_request = Request::new(next_url).with_meta(
                "first_job_on_page".to_string(),
                next_page.to_string()
            );
            next_requests.push(next_request);
        }

        Ok((items, next_requests))
    }
}
