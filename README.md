# LinkedIn Scraper (Rust)

A fast and efficient LinkedIn scraper written in Rust.

## Features

- **Person Profile Scraping**: Extract detailed information from LinkedIn profiles including experience, education, and contact details
- **Company Scraping**: Get company information, employee lists, and company details  
- **Job Search**: Search for jobs and scrape individual job postings
- **Authentication**: Secure login with cookie persistence
- **Async/Await**: High-performance asynchronous HTTP requests
- **Error Handling**: Robust error handling with detailed error types
- **JSON Export**: Serialize all data to JSON format

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
in_scraper = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Login to LinkedIn
    let client = LinkedInClient::login("email@example.com", "password").await?;
    
    // Scrape a person's profile
    let person = client.scrape_person("https://www.linkedin.com/in/example").await?;
    println!("Name: {:?}", person.name);
    
    // Scrape a company
    let company = client.scrape_company("https://www.linkedin.com/company/example").await?;
    println!("Company: {:?}", company.name);
    
    // Search for jobs
    let jobs = client.search_jobs("Software Engineer", Some("San Francisco")).await?;
    println!("Found {} jobs", jobs.len());
    
    Ok(())
}
```

### Environment Variables

For security, use environment variables for credentials:

```bash
export LINKEDIN_EMAIL="your-email@example.com"
export LINKEDIN_PASSWORD="your-password"
```

### Person Scraping

```rust
use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::login(&email, &password).await?;
    let person = client.scrape_person("https://www.linkedin.com/in/example").await?;
    
    // Access person data
    println!("Name: {:?}", person.name);
    println!("Headline: {:?}", person.headline);
    println!("Location: {:?}", person.location);
    println!("Open to work: {}", person.open_to_work);
    
    // Experiences
    for exp in &person.experiences {
        println!("Role: {} at {}", 
            exp.title.as_deref().unwrap_or("Unknown"),
            exp.company.as_deref().unwrap_or("Unknown")
        );
    }
    
    // Export to JSON
    let json = serde_json::to_string_pretty(&person)?;
    std::fs::write("person.json", json)?;
    
    Ok(())
}
```

### Company Scraping

```rust
use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::login(&email, &password).await?;
    let company = client.scrape_company("https://www.linkedin.com/company/example").await?;
    
    println!("Company: {:?}", company.name);
    println!("Industry: {:?}", company.industry);
    println!("Size: {:?}", company.company_size);
    println!("Founded: {:?}", company.founded);
    
    // Get employees separately for better performance
    let employees = client.scrape_company_employees(&company.linkedin_url).await?;
    println!("Found {} employees", employees.len());
    
    Ok(())
}
```

### Job Search

```rust
use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::login(&email, &password).await?;
    
    // Search jobs
    let jobs = client.search_jobs("Machine Learning Engineer", Some("Remote")).await?;
    
    for job in &jobs {
        println!("{} at {}", 
            job.title.as_deref().unwrap_or("Unknown"),
            job.company.as_deref().unwrap_or("Unknown")
        );
    }
    
    // Get detailed job information
    if !jobs.is_empty() {
        let detailed = client.scrape_job(&jobs[0].linkedin_url).await?;
        println!("Description: {:?}", detailed.description);
    }
    
    Ok(())
}
```

## Data Models

The scraper returns structured data using these main types:

### Person
```rust
pub struct Person {
    pub linkedin_url: String,
    pub name: Option<String>,
    pub headline: Option<String>,
    pub location: Option<String>,
    pub about: Option<String>,
    pub experiences: Vec<Experience>,
    pub educations: Vec<Education>,
    pub interests: Vec<Interest>,
    pub accomplishments: Vec<Accomplishment>,
    pub contacts: Vec<Contact>,
    pub open_to_work: bool,
}
```

### Company
```rust
pub struct Company {
    pub linkedin_url: String,
    pub name: Option<String>,
    pub about: Option<String>,
    pub website: Option<String>,
    pub headquarters: Option<String>,
    pub founded: Option<i32>,
    pub industry: Option<String>,
    pub company_size: Option<String>,
    pub specialties: Vec<String>,
    pub employees: Vec<Employee>,
    pub follower_count: Option<i32>,
}
```

### Job
```rust
pub struct Job {
    pub linkedin_url: String,
    pub title: Option<String>,
    pub company: Option<String>,
    pub company_linkedin_url: Option<String>,
    pub location: Option<String>,
    pub posted_date: Option<String>,
    pub applicant_count: Option<i32>,
    pub description: Option<String>,
    pub employment_type: Option<String>,
    pub seniority_level: Option<String>,
}
```

## Examples

Run the provided examples:

```bash
# Set credentials
export LINKEDIN_EMAIL="your-email@example.com"
export LINKEDIN_PASSWORD="your-password"

# Scrape a person profile
cargo run --example scrape_person

# Scrape a company
cargo run --example scrape_company  

# Search jobs
cargo run --example search_jobs
```

## Error Handling

The library uses a comprehensive error system:

```rust
use in_scraper::{LinkedInError, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::login("email", "password").await?;
    
    match client.scrape_person("invalid-url").await {
        Ok(person) => println!("Success: {:?}", person.name),
        Err(LinkedInError::ProfileNotFound(url)) => {
            println!("Profile not found: {}", url);
        },
        Err(LinkedInError::RateLimited) => {
            println!("Rate limited - wait before retrying");
        },
        Err(e) => println!("Other error: {}", e),
    }
    
    Ok(())
}
```

## Rate Limiting

LinkedIn has rate limiting. To avoid being blocked:

- Add delays between requests
- Use residential proxies if scraping at scale  
- Respect robots.txt
- Don't scrape more than you need

## Legal Notice

This tool is for educational and research purposes. Always check LinkedIn's Terms of Service and respect rate limits. The authors are not responsible for any misuse of this software.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details.
