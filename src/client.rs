use reqwest::Client;
use scraper::{ Html, Selector };
use crate::error::{ LinkedInError, Result };

pub struct LinkedInClient {
    client: Client,
}

impl LinkedInClient {
    pub fn new_with_cookie(li_at_cookie: &str) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
                .parse()
                .unwrap()
        );
        headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
        headers.insert("DNT", "1".parse().unwrap());
        headers.insert("Connection", "keep-alive".parse().unwrap());
        headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
        headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
        headers.insert("Sec-Fetch-Site", "none".parse().unwrap());
        headers.insert("Sec-Fetch-User", "?1".parse().unwrap());

        let cookie_value = format!("li_at={li_at_cookie}");
        headers.insert(
            "Cookie",
            cookie_value
                .parse()
                .map_err(|_| LinkedInError::Unknown("Invalid li_at cookie format".to_string()))?
        );

        let client = Client::builder()
            .cookie_store(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .default_headers(headers)
            .user_agent(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
            )
            .build()
            .map_err(|e| LinkedInError::Unknown(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self {
            client,
        })
    }

    pub async fn get_html(&self, url: &str) -> Result<Html> {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let response = self.client.get(url).send().await?;

        if response.status() == 429 {
            return Err(LinkedInError::RateLimited);
        }

        if !response.status().is_success() {
            return Err(
                LinkedInError::Unknown(format!("HTTP {} for URL: {}", response.status(), url))
            );
        }

        let html = response.text().await?;

        if html.len() < 1000 {
            return Err(
                LinkedInError::Unknown("Page content too short, possible bot detection".to_string())
            );
        }

        Ok(Html::parse_document(&html))
    }

    pub fn extract_text_by_selector(
        &self,
        document: &Html,
        selector_strs: &[&str]
    ) -> Option<String> {
        let selector = match self.select_working_selector(document, selector_strs) {
            Ok(s) => s,
            Err(_) => {
                return None;
            }
        };
        document
            .select(&selector)
            .next()
            .map(|element| element.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .filter(|text| !text.is_empty())
    }

    pub fn extract_multiple_text_by_selector(
        &self,
        document: &Html,
        selector_strs: &[&str]
    ) -> Vec<String> {
        let selector = self
            .select_working_selector(document, selector_strs)
            .unwrap_or_else(|_| panic!("Invalid selector: {selector_strs:?}"));
        document
            .select(&selector)
            .map(|element| element.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .filter(|text| !text.is_empty())
            .collect()
    }

    pub fn extract_attribute_by_selector(
        &self,
        document: &Html,
        selector_strs: &[&str],
        attribute: &str
    ) -> Option<String> {
        let selector = match self.select_working_selector(document, selector_strs) {
            Ok(s) => s,
            Err(_) => {
                return None;
            }
        };
        document
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr(attribute))
            .map(|attr| attr.to_string())
    }

    pub fn make_selector(&self, sel_str: &str) -> Result<Selector> {
        Selector::parse(sel_str).map_err(|e| LinkedInError::ParseError(e.to_string()))
    }

    pub fn select_working_selector(
        &self,
        document: &Html,
        candidates: &[&str]
    ) -> Result<Selector> {
        for &sel_str in candidates {
            if let Ok(sel) = Selector::parse(sel_str) {
                if document.select(&sel).next().is_some() {
                    return Ok(sel);
                }
            }
        }
        Err(
            LinkedInError::ElementNotFound(
                format!("No working selector found among {candidates:?}")
            )
        )
    }

    pub fn select_working_selector_for_element(
        &self,
        element: &scraper::ElementRef,
        candidates: &[&str]
    ) -> Result<Selector> {
        for &sel_str in candidates {
            if let Ok(sel) = Selector::parse(sel_str) {
                if element.select(&sel).next().is_some() {
                    return Ok(sel);
                }
            }
        }
        Err(
            LinkedInError::ElementNotFound(
                format!("No working selector found among {candidates:?} for element")
            )
        )
    }
}
