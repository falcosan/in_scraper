pub mod base;
pub mod jobs;
pub mod people;
pub mod company;

pub use jobs::JobsSpider;
pub use base::{ Spider, Request };
pub use people::PeopleProfileSpider;
pub use company::CompanyProfileSpider;
