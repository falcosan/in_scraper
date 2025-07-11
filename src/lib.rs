pub mod error;
pub mod client;
pub mod models;
pub mod scraping;
pub mod selectors;
pub use client::LinkedInClient;
pub use error::{ Result, LinkedInError };
pub use models::{ Person, Company, Job, Experience, Education, Contact, Employee };

pub async fn login(email: &str, password: &str) -> Result<LinkedInClient> {
    LinkedInClient::login(email, password).await
}
