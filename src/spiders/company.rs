use tracing::info;
use anyhow::Result;
use std::sync::Arc;
use async_trait::async_trait;
use htmlentity::entity::{ decode, ICodedDataTrait };
use crate::{
    config::Config,
    utils::HttpClient,
    items::CompanyProfile,
    spiders::{ Spider, Request },
};
use scraper::{ Html, Selector, ElementRef };

#[derive(Clone)]
pub struct CompanyProfileSpider {
    config: Arc<Config>,
    http_client: HttpClient,
    company_pages: Vec<String>,
    name_selector: Selector,
    summary_selector: Selector,
    details_selector: Selector,
    text_selector: Selector,
}

impl CompanyProfileSpider {
    pub fn new(config: Arc<Config>, company_pages: Vec<String>) -> Self {
        let http_client = HttpClient::new(config.clone()).expect("Failed to create HTTP client");
        Self {
            config,
            http_client,
            company_pages,
            name_selector: Selector::parse(crate::selectors::CompanySelectors::NAME).expect(
                "Invalid name selector"
            ),
            summary_selector: Selector::parse(crate::selectors::CompanySelectors::SUMMARY).expect(
                "Invalid summary selector"
            ),
            details_selector: Selector::parse(crate::selectors::CompanySelectors::DETAILS).expect(
                "Invalid details selector"
            ),
            text_selector: Selector::parse(crate::selectors::CompanySelectors::TEXT_MD).expect(
                "Invalid text selector"
            ),
        }
    }

    fn extract_detail(&self, details: &[ElementRef], index: usize) -> Option<String> {
        details.get(index).and_then(|detail_element| {
            let texts: Vec<String> = detail_element
                .select(&self.text_selector)
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
        info!("Parsing company {} of {}", company_index + 1, self.company_pages.len());

        let decoded = decode(response.as_bytes());
        let document = Html::parse_document(&decoded.to_string().unwrap());

        let name = document
            .select(&self.name_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "not-found".to_string());

        let summary = document
            .select(&self.summary_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "not-found".to_string());

        let details: Vec<_> = document.select(&self.details_selector).collect();

        let company = CompanyProfile {
            name,
            summary,
            industry: self.extract_detail(&details, 1),
            size: self.extract_detail(&details, 2),
            founded: self.extract_detail(&details, 5),
        };

        Ok((vec![company], vec![]))
    }
}
