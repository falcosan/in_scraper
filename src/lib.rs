pub mod client;
pub mod models;
pub mod person;
pub mod company;
pub mod job;
pub mod error;
pub mod selectors;

pub use client::LinkedInClient;
pub use models::{Person, Company, Job, Experience, Education, Contact, Employee};
pub use error::{Result, LinkedInError};

pub async fn login(email: &str, password: &str) -> Result<LinkedInClient> {
    LinkedInClient::login(email, password).await
} 