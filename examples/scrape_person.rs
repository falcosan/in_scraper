use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let email = std::env::var("LINKEDIN_EMAIL")
        .expect("LINKEDIN_EMAIL environment variable is required");
    let password = std::env::var("LINKEDIN_PASSWORD")
        .expect("LINKEDIN_PASSWORD environment variable is required");

    let client = LinkedInClient::login(&email, &password).await?;
    
    let profile_url = "https://www.linkedin.com/in/example-profile";
    let person = client.scrape_person(profile_url).await?;

    println!("Name: {:?}", person.name);
    println!("Headline: {:?}", person.headline);
    println!("Location: {:?}", person.location);
    println!("About: {:?}", person.about);
    println!("Open to work: {}", person.open_to_work);
    
    println!("\nExperiences:");
    for experience in &person.experiences {
        println!("  - {} at {} ({})", 
            experience.title.as_deref().unwrap_or("Unknown title"),
            experience.company.as_deref().unwrap_or("Unknown company"),
            experience.duration.as_deref().unwrap_or("Unknown duration")
        );
    }

    println!("\nEducations:");
    for education in &person.educations {
        println!("  - {} at {}", 
            education.degree.as_deref().unwrap_or("Unknown degree"),
            education.school.as_deref().unwrap_or("Unknown school")
        );
    }

    let json = serde_json::to_string_pretty(&person)?;
    println!("\nFull profile as JSON:\n{}", json);

    Ok(())
} 