use in_scraper::{LinkedInClient, Result};
use scraper::Selector;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    print!("LinkedIn Email: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    let email = email.trim();

    print!("LinkedIn Password: ");
    io::stdout().flush().unwrap();
    let password = rpassword::read_password().unwrap();

    println!("Logging in...");
    let client = LinkedInClient::login(&email, &password).await?;
    
    println!("Testing feed access first...");
    match client.get_html("https://www.linkedin.com/feed/").await {
        Ok(document) => {
            let title_selector = Selector::parse("title").unwrap();
            if let Some(title) = document.select(&title_selector).next() {
                println!("Feed page title: {}", title.text().collect::<String>());
            }
        }
        Err(e) => println!("Feed access error: {}", e),
    }
    
    println!("Getting profile page...");
    let test_url = "https://www.linkedin.com/in/williamhgates";
    
    match client.get_html(test_url).await {
        Ok(document) => {
            println!("Successfully retrieved page");
            
            let title_selector = Selector::parse("title").unwrap();
            if let Some(title) = document.select(&title_selector).next() {
                let title_text = title.text().collect::<String>();
                println!("Page title: '{}'", title_text);
                
                if title_text.contains("LinkedIn") && !title_text.contains("Bill Gates") {
                    println!("Warning: This may not be the actual profile page");
                }
            }
            
            println!("\nAll text content (first 500 chars):");
            let all_text: String = document.root_element().text().collect::<Vec<_>>().join(" ");
            let preview = if all_text.len() > 500 { &all_text[..500] } else { &all_text };
            println!("{}", preview);
            
            println!("\nLooking for any name-like text in h1 tags:");
            let h1_selector = Selector::parse("h1").unwrap();
            for (i, h1) in document.select(&h1_selector).enumerate().take(10) {
                let text = h1.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    println!("  H1 #{}: '{}'", i+1, text);
                }
            }
            
            println!("\nLooking for span elements with potential names:");
            let span_selector = Selector::parse("span").unwrap();
            let mut span_count = 0;
            for span in document.select(&span_selector) {
                let text = span.text().collect::<String>().trim().to_string();
                if text.len() > 5 && text.len() < 50 && text.contains(' ') && 
                   text.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
                    println!("  Potential name: '{}'", text);
                    span_count += 1;
                    if span_count > 5 { break; }
                }
            }
        }
        Err(e) => {
            println!("Error getting page: {}", e);
        }
    }
    
    Ok(())
}
