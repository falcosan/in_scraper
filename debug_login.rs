use in_scraper::client::LinkedInClient;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .build()?;

    println!("Fetching LinkedIn login page...");
    let login_url = "https://www.linkedin.com/login";
    let response = client.get(login_url).send().await?;
    
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    
    let html = response.text().await?;
    let document = Html::parse_document(&html);
    
    println!("Page title: {:?}", document.select(&Selector::parse("title").unwrap()).next().map(|e| e.text().collect::<String>()));
    
    let email_selector = Selector::parse("#username").unwrap();
    let password_selector = Selector::parse("#password").unwrap();
    let csrf_selector = Selector::parse("input[name='loginCsrfParam']").unwrap();
    
    println!("Email field found: {}", document.select(&email_selector).next().is_some());
    println!("Password field found: {}", document.select(&password_selector).next().is_some());
    
    if let Some(csrf_element) = document.select(&csrf_selector).next() {
        if let Some(csrf_token) = csrf_element.value().attr("value") {
            println!("CSRF token found: {}", csrf_token);
        }
    } else {
        println!("No CSRF token found");
    }
    
    println!("\nForm elements:");
    let form_inputs = Selector::parse("form input").unwrap();
    for input in document.select(&form_inputs) {
        if let Some(name) = input.value().attr("name") {
            if let Some(input_type) = input.value().attr("type") {
                println!("  {} ({})", name, input_type);
            }
        }
    }
    
    Ok(())
}
