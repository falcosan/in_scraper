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
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8".parse().unwrap());
        headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
        headers.insert("DNT", "1".parse().unwrap());
        headers.insert("Connection", "keep-alive".parse().unwrap());
        headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
        headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
        headers.insert("Sec-Fetch-Site", "none".parse().unwrap());
        headers.insert("Sec-Fetch-User", "?1".parse().unwrap());

        let client = Client::builder()
            .cookie_store(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .default_headers(headers)
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://www.linkedin.com".to_string(),
        }
    }

    pub async fn login(email: &str, password: &str) -> Result<Self> {
        let client = Self::new();
        
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
        
        let login_url = format!("{}/login", client.base_url);
        let response = client.client.get(&login_url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let email_selector = Selector::parse(selectors::auth::EMAIL_INPUT)
            .map_err(|e| LinkedInError::ParseError(e.to_string()))?;
        let csrf_selector = Selector::parse(selectors::auth::CSRF_TOKEN)
            .map_err(|e| LinkedInError::ParseError(e.to_string()))?;

        if document.select(&email_selector).next().is_none() {
            return Err(LinkedInError::ElementNotFound("Email input field not found. LinkedIn may have changed their login page structure.".to_string()));
        }

        let csrf_token = document.select(&csrf_selector)
            .next()
            .and_then(|element| element.value().attr("value"))
            .ok_or_else(|| LinkedInError::ElementNotFound("CSRF token not found on login page.".to_string()))?;

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let mut login_data = HashMap::new();
        login_data.insert("session_key", email);
        login_data.insert("session_password", password);
        login_data.insert("loginCsrfParam", csrf_token);

        let login_submit_url = format!("{}/checkpoint/lg/login-submit", client.base_url);
        let response = client
            .client
            .post(&login_submit_url)
            .header("Referer", &login_url)
            .header("Origin", &client.base_url)
            .form(&login_data)
            .send()
            .await?;

        let final_url = response.url().to_string();
        
        if final_url.contains("/challenge") {
            return Err(LinkedInError::Unknown("LinkedIn security challenge detected. This usually happens when:\n1. Logging in from a new location or device\n2. Unusual activity is detected\n3. Account needs verification\n\nSolutions:\n- Log in through a web browser first to complete any challenges\n- Wait some time before trying again\n- Use a residential IP address\n- Try from the same network/device you normally use".to_string()));
        }
        
        if final_url.contains("/uas") {
            return Err(LinkedInError::Unknown("LinkedIn account verification required. Please log in through a web browser first to verify your account.".to_string()));
        }
        
        if final_url.contains("/checkpoint") && !final_url.contains("/feed") {
            let response_text = response.text().await?;
            let doc = Html::parse_document(&response_text);
            
            let error_selector = Selector::parse(".form__label--error, .alert, .msg--error, .error").unwrap();
            let mut error_messages = Vec::new();
            for error in doc.select(&error_selector) {
                let text = error.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    error_messages.push(text);
                }
            }
            
            if !error_messages.is_empty() {
                return Err(LinkedInError::AuthenticationFailed);
            }
            
            return Err(LinkedInError::Unknown("Login failed. You may be stuck at a checkpoint. Try logging in manually first.".to_string()));
        }

        if response.status().is_success() && (final_url.contains("/feed") || final_url.contains("/in/")) {
            return Ok(client);
        }

        let response_text = response.text().await?;
        
        if response_text.contains("global-nav") || response_text.contains("linkedin.com/feed") || response_text.contains("li-header") {
            return Ok(client);
        }

        Err(LinkedInError::AuthenticationFailed)
    }

    pub async fn get_html(&self, url: &str) -> Result<Html> {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        
        let response = self.client.get(url).send().await?;
        
        if response.status() == 429 {
            return Err(LinkedInError::RateLimited);
        }
        
        if !response.status().is_success() {
            return Err(LinkedInError::Unknown(format!("HTTP {} for URL: {}", response.status(), url)));
        }

        let html = response.text().await?;
        
        if html.len() < 1000 {
            return Err(LinkedInError::Unknown("Page content too short, possible bot detection".to_string()));
        }
        
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

    pub async fn is_logged_in(&self) -> bool {
        match self.client.get(&format!("{}/feed", self.base_url)).send().await {
            Ok(response) => {
                let url = response.url().to_string();
                !url.contains("/login") && !url.contains("/checkpoint") && (
                    url.contains("/feed") || 
                    url.contains("/in/") ||
                    response.status().is_success()
                )
            }
            Err(_) => false,
        }
    }

    pub async fn login_with_session_check(email: &str, password: &str) -> Result<Self> {
        let client = Self::new();
        
        if client.is_logged_in().await {
            return Ok(client);
        }
        
        Self::login(email, password).await
    }

    pub async fn login_with_retry(email: &str, password: &str, max_retries: u32) -> Result<Self> {
        let mut last_error = LinkedInError::Unknown("No attempts made".to_string());
        
        for attempt in 1..=max_retries {
            if attempt > 1 {
                let delay = std::cmp::min(5000 * attempt as u64, 30000);
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            }
            
            match Self::login(email, password).await {
                Ok(client) => return Ok(client),
                Err(e) => {
                    last_error = e;
                    match &last_error {
                        LinkedInError::Unknown(msg) if msg.contains("challenge") => {
                            break;
                        }
                        LinkedInError::RateLimited => {
                            if attempt < max_retries {
                                continue;
                            }
                        }
                        LinkedInError::AuthenticationFailed => {
                            break;
                        }
                        _ => {
                            if attempt < max_retries {
                                continue;
                            }
                        }
                    }
                }
            }
        }
        
        Err(last_error)
    }
}

impl Default for LinkedInClient {
    fn default() -> Self {
        Self::new()
    }
}