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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PersonProfile {
    pub profile: String,
    pub url: String,
    pub name: String,
    pub description: String,
    pub location: Option<String>,
    pub followers: Option<String>,
    pub connections: Option<String>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub projects: Vec<Project>,
    pub languages: Vec<Language>,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Experience {
    pub organization_profile: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub duration: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub logo: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Education {
    pub organization: String,
    pub organization_profile: Option<String>,
    pub course_details: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Project {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Language {
    pub name: Option<String>,
    pub proficiency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Activity {
    pub title: Option<String>,
    pub url: Option<String>,
}
