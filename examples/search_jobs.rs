use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let email = std::env::var("LINKEDIN_EMAIL")
        .expect("LINKEDIN_EMAIL environment variable is required");
    let password = std::env::var("LINKEDIN_PASSWORD")
        .expect("LINKEDIN_PASSWORD environment variable is required");

    let client = LinkedInClient::login(&email, &password).await?;
    
    let search_query = "Software Engineer";
    let location = Some("San Francisco, CA");
    
    println!("Searching for '{}' jobs in {:?}...\n", search_query, location);
    
    let jobs = client.search_jobs(search_query, location).await?;
    
    println!("Found {} job listings:\n", jobs.len());
    
    for (i, job) in jobs.iter().enumerate() {
        println!("{}. {}", i + 1, job.title.as_deref().unwrap_or("Unknown title"));
        println!("   Company: {}", job.company.as_deref().unwrap_or("Unknown company"));
        if let Some(location) = &job.location {
            println!("   Location: {}", location);
        }
        if let Some(posted_date) = &job.posted_date {
            println!("   Posted: {}", posted_date);
        }
        println!("   URL: {}", job.linkedin_url);
        println!();
    }

    if !jobs.is_empty() {
        println!("Scraping details for the first job...\n");
        let detailed_job = client.scrape_job(&jobs[0].linkedin_url).await?;
        
        println!("Job Details:");
        println!("Title: {:?}", detailed_job.title);
        println!("Company: {:?}", detailed_job.company);
        println!("Location: {:?}", detailed_job.location);
        println!("Employment Type: {:?}", detailed_job.employment_type);
        println!("Seniority Level: {:?}", detailed_job.seniority_level);
        
        if let Some(description) = &detailed_job.description {
            println!("\nDescription:\n{}", description);
        }

        let json = serde_json::to_string_pretty(&detailed_job)?;
        println!("\nFull job details as JSON:\n{}", json);
    }

    Ok(())
} 