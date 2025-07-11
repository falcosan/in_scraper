use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: Option<String>,
    pub occupation: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub name: Option<String>,
    pub linkedin_url: Option<String>,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub company_type: Option<String>,
    pub headquarters: Option<String>,
    pub company_size: Option<String>,
    pub founded: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub title: Option<String>,
    pub company: Option<String>,
    pub company_linkedin_url: Option<String>,
    pub location: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub duration: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub school: Option<String>,
    pub school_linkedin_url: Option<String>,
    pub degree: Option<String>,
    pub field_of_study: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interest {
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Accomplishment {
    pub category: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Person {
    pub fn new(linkedin_url: String) -> Self {
        Self {
            linkedin_url,
            name: None,
            headline: None,
            location: None,
            about: None,
            experiences: Vec::new(),
            educations: Vec::new(),
            interests: Vec::new(),
            accomplishments: Vec::new(),
            contacts: Vec::new(),
            open_to_work: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub linkedin_url: String,
    pub name: Option<String>,
    pub about: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub headquarters: Option<String>,
    pub founded: Option<i32>,
    pub industry: Option<String>,
    pub company_type: Option<String>,
    pub company_size: Option<String>,
    pub specialties: Vec<String>,
    pub employees: Vec<Employee>,
    pub follower_count: Option<i32>,
}

impl Company {
    pub fn new(linkedin_url: String) -> Self {
        Self {
            linkedin_url,
            name: None,
            about: None,
            website: None,
            phone: None,
            headquarters: None,
            founded: None,
            industry: None,
            company_type: None,
            company_size: None,
            specialties: Vec::new(),
            employees: Vec::new(),
            follower_count: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub title: Option<String>,
    pub linkedin_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub linkedin_url: String,
    pub title: Option<String>,
    pub company: Option<String>,
    pub company_linkedin_url: Option<String>,
    pub location: Option<String>,
    pub posted_date: Option<String>,
    pub applicant_count: Option<i32>,
    pub description: Option<String>,
    pub benefits: Option<String>,
    pub employment_type: Option<String>,
    pub seniority_level: Option<String>,
}

impl Job {
    pub fn new(linkedin_url: String) -> Self {
        Self {
            linkedin_url,
            title: None,
            company: None,
            company_linkedin_url: None,
            location: None,
            posted_date: None,
            applicant_count: None,
            description: None,
            benefits: None,
            employment_type: None,
            seniority_level: None,
        }
    }
} 