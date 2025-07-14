use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyProfile {
    pub name: String,
    pub summary: String,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub founded: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobListing {
    pub job_title: String,
    pub job_detail_url: String,
    pub job_listed: String,
    pub company_name: String,
    pub company_link: String,
    pub company_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonProfile {
    pub profile: String,
    pub url: String,
    pub name: String,
    pub description: String,
    pub location: Option<String>,
    pub followers: Option<String>,
    pub connections: Option<String>,
    pub about: Option<String>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub organisation_profile: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub duration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub organisation: String,
    pub organisation_profile: Option<String>,
    pub course_details: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}
