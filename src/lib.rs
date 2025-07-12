pub mod error;
pub mod client;
pub mod models;
pub mod scraping;
pub mod selectors;
pub use client::LinkedInClient;
pub use error::{ LinkedInError, Result };
pub use models::{ Company, Contact, Education, Employee, Experience, Job, Person };
