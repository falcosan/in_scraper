use tracing::info;
use anyhow::Result;
use std::sync::Arc;
use async_trait::async_trait;
use scraper::{ Html, Selector };
use crate::{
    config::Config,
    utils::HttpClient,
    items::CompanyProfile,
    spiders::{ Spider, Request },
};

#[derive(Clone)]
pub struct CompanyProfileSpider {
    config: Arc<Config>,
    http_client: HttpClient,
    company_pages: Vec<String>,
}

impl CompanyProfileSpider {
    pub fn new(config: Arc<Config>, company_pages: Vec<String>) -> Self {
        let http_client = HttpClient::new(config.clone()).expect("Failed to create HTTP client");
        Self {
            config,
            http_client,
            company_pages,
        }
    }

    fn extract_detail_text(
        &self,
        details: &[scraper::ElementRef],
        index: usize,
        text_selector: &Selector
    ) -> Option<String> {
        details.get(index).and_then(|detail| {
            let texts: Vec<String> = detail
                .select(text_selector)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .collect();
            texts.get(1).cloned()
        })
    }
}

#[async_trait]
impl Spider for CompanyProfileSpider {
    type Item = CompanyProfile;

    fn name(&self) -> &str {
        "linkedin_company_profile"
    }

    fn get_config(&self) -> &Arc<Config> {
        &self.config
    }

    fn get_http_client(&self) -> &HttpClient {
        &self.http_client
    }

    async fn start_requests(&self) -> Vec<Request> {
        self.company_pages
            .iter()
            .enumerate()
            .map(|(index, url)| {
                Request::new(url.clone()).with_meta("company_index".to_string(), index.to_string())
            })
            .collect()
    }

    async fn parse(
        &self,
        response: String,
        request: &Request
    ) -> Result<(Vec<Self::Item>, Vec<Request>)> {
        let company_index = request.meta
            .get("company_index")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        info!("Scraping page {} of {}", company_index + 1, self.company_pages.len());

        let document = Html::parse_document(&response);
        let name_selector = Selector::parse(crate::selectors::CompanySelectors::NAME).unwrap();
        let summary_selector = Selector::parse(
            crate::selectors::CompanySelectors::SUMMARY
        ).unwrap();
        let details_selector = Selector::parse(
            crate::selectors::CompanySelectors::DETAILS
        ).unwrap();
        let text_selector = Selector::parse(crate::selectors::CompanySelectors::TEXT_MD).unwrap();

        let name = document
            .select(&name_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "not-found".to_string());

        let summary = document
            .select(&summary_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "not-found".to_string());

        let details: Vec<_> = document.select(&details_selector).collect();

        let company = CompanyProfile {
            name,
            summary,
            industry: self.extract_detail_text(&details, 1, &text_selector),
            size: self.extract_detail_text(&details, 2, &text_selector),
            founded: self.extract_detail_text(&details, 5, &text_selector),
        };

        Ok((vec![company], vec![]))
    }
}
