use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let email = env::var("LINKEDIN_EMAIL").expect("LINKEDIN_EMAIL must be set");
    let password = env::var("LINKEDIN_PASSWORD").expect("LINKEDIN_PASSWORD must be set");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
    headers.insert("DNT", "1".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());

    let client = Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::limited(10))
        .default_headers(headers)
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;

    println!("1. Fetching LinkedIn login page...");
    let login_url = "https://www.linkedin.com/login";
    let response = client.get(login_url).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    println!("   HTML length: {} characters", html.len());
    println!("   First 500 chars: {}", &html[..std::cmp::min(500, html.len())]);

    let title_selector = Selector::parse("title").unwrap();
    if let Some(title) = document.select(&title_selector).next() {
        println!("   Page title: {}", title.text().collect::<String>().trim());
    }

    let csrf_selector = Selector::parse("input[name='loginCsrfParam']").unwrap();
    let csrf_token = document.select(&csrf_selector)
        .next()
        .and_then(|element| element.value().attr("value"));
    
    println!("2. CSRF token search result: {:?}", csrf_token);
    
    if csrf_token.is_none() {
        println!("All form inputs:");
        let all_inputs = Selector::parse("input").unwrap();
        for input in document.select(&all_inputs) {
            if let Some(name) = input.value().attr("name") {
                println!("  Input: {} = {:?}", name, input.value().attr("value"));
            }
        }
        return Err("CSRF token not found".into());
    }
    
    let csrf_token = csrf_token.unwrap();

    println!("2. Found CSRF token: {}", csrf_token);

    let mut login_data = HashMap::new();
    login_data.insert("session_key", email.as_str());
    login_data.insert("session_password", password.as_str());
    login_data.insert("loginCsrfParam", csrf_token);

    println!("3. Submitting login form...");
    let login_submit_url = "https://www.linkedin.com/checkpoint/lg/login-submit";
    let response = client
        .post(login_submit_url)
        .form(&login_data)
        .send()
        .await?;

    let final_url = response.url().to_string();
    let status = response.status();
    
    println!("4. Login response:");
    println!("   Status: {}", status);
    println!("   Final URL: {}", final_url);
    println!("   Headers: {:#?}", response.headers());

    let response_text = response.text().await?;
    println!("5. Response length: {} bytes", response_text.len());

    if final_url.contains("/challenge") || final_url.contains("/uas") {
        println!("ERROR: LinkedIn requires additional verification");
        return Ok(());
    }
    
    if final_url.contains("/checkpoint") {
        println!("ERROR: Still at checkpoint - authentication likely failed");
        println!("Response preview: {}", &response_text[..std::cmp::min(500, response_text.len())]);
        return Ok(());
    }

    if final_url.contains("/feed") || final_url.contains("/in/") {
        println!("SUCCESS: Redirected to feed or profile page");
        return Ok(());
    }

    if response_text.contains("global-nav") || response_text.contains("linkedin.com/feed") {
        println!("SUCCESS: Found navigation elements indicating successful login");
        return Ok(());
    }

    println!("UNKNOWN: Login status unclear");
    println!("Response preview: {}", &response_text[..std::cmp::min(500, response_text.len())]);

    let doc = Html::parse_document(&response_text);
    let title_selector = Selector::parse("title").unwrap();
    if let Some(title) = doc.select(&title_selector).next() {
        println!("Page title: {}", title.text().collect::<String>());
    }

    let error_selector = Selector::parse(".error, .alert, .form-global-error").unwrap();
    for error in doc.select(&error_selector) {
        println!("Error found: {}", error.text().collect::<String>().trim());
    }

    Ok(())
}
