use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let email = std::env::var("LINKEDIN_EMAIL")
        .expect("LINKEDIN_EMAIL environment variable is required");
    let password = std::env::var("LINKEDIN_PASSWORD")
        .expect("LINKEDIN_PASSWORD environment variable is required");

    let client = LinkedInClient::login(&email, &password).await?;
    
    let company_url = "https://www.linkedin.com/company/example-company";
    let company = client.scrape_company(company_url).await?;

    println!("Company Name: {:?}", company.name);
    println!("Industry: {:?}", company.industry);
    println!("Headquarters: {:?}", company.headquarters);
    println!("Founded: {:?}", company.founded);
    println!("Company Size: {:?}", company.company_size);
    println!("Website: {:?}", company.website);
    
    if !company.about.as_ref().unwrap_or(&String::new()).is_empty() {
        println!("\nAbout:\n{}", company.about.as_ref().unwrap());
    }

    if !company.specialties.is_empty() {
        println!("\nSpecialties:");
        for specialty in &company.specialties {
            println!("  - {}", specialty);
        }
    }

    if !company.employees.is_empty() {
        println!("\nSample Employees:");
        for (i, employee) in company.employees.iter().take(5).enumerate() {
            println!("  {}. {} - {}", 
                i + 1,
                employee.name,
                employee.title.as_deref().unwrap_or("Unknown title")
            );
        }
        if company.employees.len() > 5 {
            println!("  ... and {} more employees", company.employees.len() - 5);
        }
    }

    let json = serde_json::to_string_pretty(&company)?;
    println!("\nFull company data as JSON:\n{}", json);

    Ok(())
} 