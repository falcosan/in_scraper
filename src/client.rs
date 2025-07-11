use crate::error::{LinkedInError, Result};
use crate::selectors;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;


pub struct LinkedInClient {
    client: Client,
    base_url: String,
}

impl LinkedInClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .cookie_store(true)
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://www.linkedin.com".to_string(),
        }
    }

    pub async fn login(email: &str, password: &str) -> Result<Self> {
        let client = Self::new();
        
        let login_url = format!("{}/login", client.base_url);
        let response = client.client.get(&login_url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let email_selector = Selector::parse(selectors::auth::EMAIL_INPUT)
            .map_err(|e| LinkedInError::ParseError(e.to_string()))?;
        let _password_selector = Selector::parse(selectors::auth::PASSWORD_INPUT)
            .map_err(|e| LinkedInError::ParseError(e.to_string()))?;

        if document.select(&email_selector).next().is_none() {
            return Err(LinkedInError::ElementNotFound("Email input field".to_string()));
        }

        let mut login_data = HashMap::new();
        login_data.insert("session_key", email);
        login_data.insert("session_password", password);

        let login_submit_url = format!("{}/checkpoint/lg/login-submit", client.base_url);
        let response = client
            .client
            .post(&login_submit_url)
            .form(&login_data)
            .send()
            .await?;

        if response.status().is_success() {
            let html = response.text().await?;
            let document = Html::parse_document(&html);
            
            let verification_selector = Selector::parse(selectors::auth::VERIFICATION_ELEMENT)
                .map_err(|e| LinkedInError::ParseError(e.to_string()))?;
            
            if document.select(&verification_selector).next().is_some() {
                return Ok(client);
            }
        }

        Err(LinkedInError::AuthenticationFailed)
    }

    pub async fn get_html(&self, url: &str) -> Result<Html> {
        let response = self.client.get(url).send().await?;
        
        if response.status() == 429 {
            return Err(LinkedInError::RateLimited);
        }
        
        if !response.status().is_success() {
            return Err(LinkedInError::Unknown(format!("HTTP {}", response.status())));
        }

        let html = response.text().await?;
        Ok(Html::parse_document(&html))
    }



    pub fn extract_text_by_selector(&self, document: &Html, selector_str: &str) -> Option<String> {
        let selector = Selector::parse(selector_str).ok()?;
        document
            .select(&selector)
            .next()
            .map(|element| element.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .filter(|text| !text.is_empty())
    }

    pub fn extract_multiple_text_by_selector(&self, document: &Html, selector_str: &str) -> Vec<String> {
        let selector = Selector::parse(selector_str).unwrap_or_else(|_| {
            panic!("Invalid selector: {}", selector_str)
        });
        
        document
            .select(&selector)
            .map(|element| element.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .filter(|text| !text.is_empty())
            .collect()
    }

    pub fn extract_attribute_by_selector(&self, document: &Html, selector_str: &str, attribute: &str) -> Option<String> {
        let selector = Selector::parse(selector_str).ok()?;
        document
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr(attribute))
            .map(|attr| attr.to_string())
    }
}

impl Default for LinkedInClient {
    fn default() -> Self {
        Self::new()
    }
} 