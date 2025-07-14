pub mod base;
pub mod jobs;
pub mod people_profile;
pub mod company_profile;

pub use jobs::JobsSpider;
pub use base::{ Spider, Request };
pub use people_profile::PeopleProfileSpider;
pub use company_profile::CompanyProfileSpider;
