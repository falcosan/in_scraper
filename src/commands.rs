use crate::cli::{Commands, OutputFormat};
use crate::{LinkedInClient, Result};
use in_scraper::models::{Person, Company, Job};
use std::fs;
use std::io::{self, Write};
use tabled::{Table, Tabled};

pub async fn execute_command(
    client: &LinkedInClient,
    command: Commands,
    format: OutputFormat,
    output: Option<String>,
    verbose: bool,
) -> Result<()> {
    let result = match command {
        Commands::Person { url, .. } => {
            if verbose { eprintln!("Scraping person profile: {url}"); }
            let person = client.scrape_person(&url).await?;
            format_person_output(person, format)
        }
        Commands::People { query, location, details, .. } => {
            if verbose { 
                eprintln!("Searching people for: {query} in {location:?}"); 
            }
            let people = client.search_people(&query, location.as_deref()).await?;
            
            let mut detailed_people = Vec::new();
            if details > 0 {
                if verbose { eprintln!("Fetching details for {} people...", details.min(people.len())); }
                for person in people.iter().take(details) {
                    match client.scrape_person(&person.linkedin_url).await {
                        Ok(detailed_person) => detailed_people.push(detailed_person),
                        Err(e) => {
                            if verbose { eprintln!("Failed to get details for person: {e}"); }
                            detailed_people.push(person.clone());
                        }
                    }
                }
                format_people_output(detailed_people, format)
            } else {
                format_people_output(people, format)
            }
        }
        Commands::Company { url, employees, .. } => {
            if verbose { eprintln!("Scraping company page: {url}"); }
            let mut company = client.scrape_company(&url).await?;
            
            if employees {
                if verbose { eprintln!("Fetching employee list..."); }
                company.employees = client.scrape_company_employees(&url).await?;
            }
            
            format_company_output(company, format)
        }
        Commands::Jobs { query, location, details, .. } => {
            if verbose { 
                eprintln!("Searching jobs for: {query} in {location:?}"); 
            }
            let jobs = client.search_jobs(&query, location.as_deref()).await?;
            
            let mut detailed_jobs = Vec::new();
            if details > 0 {
                if verbose { eprintln!("Fetching details for {} jobs...", details.min(jobs.len())); }
                for job in jobs.iter().take(details) {
                    match client.scrape_job(&job.linkedin_url).await {
                        Ok(detailed_job) => detailed_jobs.push(detailed_job),
                        Err(e) => {
                            if verbose { eprintln!("Failed to get details for job: {e}"); }
                            detailed_jobs.push(job.clone());
                        }
                    }
                }
                format_jobs_output(detailed_jobs, format)
            } else {
                format_jobs_output(jobs, format)
            }
        }
        Commands::Job { url, .. } => {
            if verbose { eprintln!("Scraping job posting: {url}"); }
            let job = client.scrape_job(&url).await?;
            format_job_output(job, format)
        }
    };

    write_output(result, output)?;
    Ok(())
}

fn format_person_output(person: Person, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string(&person).unwrap_or_default(),
        OutputFormat::Pretty => serde_json::to_string_pretty(&person).unwrap_or_default(),
        OutputFormat::Summary => format_person_summary(person),
        OutputFormat::Table => format_person_table(person),
    }
}

fn format_company_output(company: Company, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string(&company).unwrap_or_default(),
        OutputFormat::Pretty => serde_json::to_string_pretty(&company).unwrap_or_default(),
        OutputFormat::Summary => format_company_summary(company),
        OutputFormat::Table => format_company_table(company),
    }
}

fn format_jobs_output(jobs: Vec<Job>, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string(&jobs).unwrap_or_default(),
        OutputFormat::Pretty => serde_json::to_string_pretty(&jobs).unwrap_or_default(),
        OutputFormat::Summary => format_jobs_summary(&jobs),
        OutputFormat::Table => format_jobs_table(jobs),
    }
}

fn format_job_output(job: Job, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string(&job).unwrap_or_default(),
        OutputFormat::Pretty => serde_json::to_string_pretty(&job).unwrap_or_default(),
        OutputFormat::Summary => format_job_summary(job),
        OutputFormat::Table => format_job_table(job),
    }
}

fn format_people_output(people: Vec<Person>, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string(&people).unwrap_or_default(),
        OutputFormat::Pretty => serde_json::to_string_pretty(&people).unwrap_or_default(),
        OutputFormat::Summary => format_people_summary(&people),
        OutputFormat::Table => format_people_table(people),
    }
}

fn format_person_summary(person: Person) -> String {
    let mut output = String::new();
    
    if let Some(name) = &person.name {
        output.push_str(&format!("Name: {name}\n"));
    }
    
    if let Some(headline) = &person.headline {
        output.push_str(&format!("Headline: {headline}\n"));
    }
    
    if let Some(location) = &person.location {
        output.push_str(&format!("Location: {location}\n"));
    }
    
    if person.open_to_work {
        output.push_str("Status: Open to work\n");
    }
    
    if !person.experiences.is_empty() {
        output.push_str(&format!("\nExperiences ({}):\n", person.experiences.len()));
        for (i, exp) in person.experiences.iter().take(3).enumerate() {
            output.push_str(&format!(
                "  {}. {} at {} ({})\n",
                i + 1,
                exp.title.as_deref().unwrap_or("Unknown title"),
                exp.company.as_deref().unwrap_or("Unknown company"),
                exp.duration.as_deref().unwrap_or("Unknown duration")
            ));
        }
        if person.experiences.len() > 3 {
            output.push_str(&format!("  ... and {} more\n", person.experiences.len() - 3));
        }
    }
    
    if !person.educations.is_empty() {
        output.push_str(&format!("\nEducations ({}):\n", person.educations.len()));
        for (i, edu) in person.educations.iter().take(2).enumerate() {
            output.push_str(&format!(
                "  {}. {} at {}\n",
                i + 1,
                edu.degree.as_deref().unwrap_or("Unknown degree"),
                edu.school.as_deref().unwrap_or("Unknown school")
            ));
        }
        if person.educations.len() > 2 {
            output.push_str(&format!("  ... and {} more\n", person.educations.len() - 2));
        }
    }
    
    output
}

fn format_company_summary(company: Company) -> String {
    let mut output = String::new();
    
    if let Some(name) = &company.name {
        output.push_str(&format!("Company: {name}\n"));
    }
    
    if let Some(industry) = &company.industry {
        output.push_str(&format!("Industry: {industry}\n"));
    }
    
    if let Some(size) = &company.company_size {
        output.push_str(&format!("Size: {size}\n"));
    }
    
    if let Some(founded) = company.founded {
        output.push_str(&format!("Founded: {founded}\n"));
    }
    
    if let Some(headquarters) = &company.headquarters {
        output.push_str(&format!("Headquarters: {headquarters}\n"));
    }
    
    if !company.employees.is_empty() {
        output.push_str(&format!("\nEmployees ({}):\n", company.employees.len()));
        for (i, emp) in company.employees.iter().take(5).enumerate() {
            output.push_str(&format!(
                "  {}. {} - {}\n",
                i + 1,
                emp.name,
                emp.title.as_deref().unwrap_or("Unknown title")
            ));
        }
        if company.employees.len() > 5 {
            output.push_str(&format!("  ... and {} more\n", company.employees.len() - 5));
        }
    }
    
    output
}

fn format_jobs_summary(jobs: &[Job]) -> String {
    let mut output = format!("Found {} jobs:\n\n", jobs.len());
    
    for (i, job) in jobs.iter().take(10).enumerate() {
        output.push_str(&format!(
            "{}. {} at {}\n   Location: {}\n   URL: {}\n\n",
            i + 1,
            job.title.as_deref().unwrap_or("Unknown title"),
            job.company.as_deref().unwrap_or("Unknown company"),
            job.location.as_deref().unwrap_or("Unknown location"),
            job.linkedin_url
        ));
    }
    
    if jobs.len() > 10 {
        output.push_str(&format!("... and {} more jobs\n", jobs.len() - 10));
    }
    
    output
}

fn format_job_summary(job: Job) -> String {
    let mut output = String::new();
    
    if let Some(title) = &job.title {
        output.push_str(&format!("Title: {title}\n"));
    }
    
    if let Some(company) = &job.company {
        output.push_str(&format!("Company: {company}\n"));
    }
    
    if let Some(location) = &job.location {
        output.push_str(&format!("Location: {location}\n"));
    }
    
    if let Some(employment_type) = &job.employment_type {
        output.push_str(&format!("Type: {employment_type}\n"));
    }
    
    if let Some(seniority_level) = &job.seniority_level {
        output.push_str(&format!("Level: {seniority_level}\n"));
    }
    
    if let Some(posted_date) = &job.posted_date {
        output.push_str(&format!("Posted: {posted_date}\n"));
    }
    
    if let Some(applicant_count) = job.applicant_count {
        output.push_str(&format!("Applicants: {applicant_count}\n"));
    }
    
    if let Some(description) = &job.description {
        output.push_str(&format!("\nDescription:\n{description}\n"));
    }
    
    output.push_str(&format!("\nURL: {}\n", job.linkedin_url));
    
    output
}

fn format_people_summary(people: &[Person]) -> String {
    let mut output = format!("Found {} people:\n\n", people.len());
    
    for (i, person) in people.iter().take(10).enumerate() {
        output.push_str(&format!(
            "{}. {}\n   {}\n   {}\n   URL: {}\n\n",
            i + 1,
            person.name.as_deref().unwrap_or("Unknown name"),
            person.headline.as_deref().unwrap_or("No headline"),
            person.location.as_deref().unwrap_or("Unknown location"),
            person.linkedin_url
        ));
    }
    
    if people.len() > 10 {
        output.push_str(&format!("... and {} more people\n", people.len() - 10));
    }
    
    output
}

#[derive(Tabled)]
struct PersonTableRow {
    name: String,
    headline: String,
    location: String,
    open_to_work: String,
    experiences: String,
    educations: String,
}

fn format_person_table(person: Person) -> String {
    let row = PersonTableRow {
        name: person.name.unwrap_or_default(),
        headline: person.headline.unwrap_or_default(),
        location: person.location.unwrap_or_default(),
        open_to_work: if person.open_to_work { "Yes".to_string() } else { "No".to_string() },
        experiences: person.experiences.len().to_string(),
        educations: person.educations.len().to_string(),
    };
    
    Table::new([row]).to_string()
}

#[derive(Tabled)]
struct CompanyTableRow {
    name: String,
    industry: String,
    size: String,
    founded: String,
    headquarters: String,
    employees: String,
}

fn format_company_table(company: Company) -> String {
    let row = CompanyTableRow {
        name: company.name.unwrap_or_default(),
        industry: company.industry.unwrap_or_default(),
        size: company.company_size.unwrap_or_default(),
        founded: company.founded.map_or_default(|f| f.to_string()),
        headquarters: company.headquarters.unwrap_or_default(),
        employees: company.employees.len().to_string(),
    };
    
    Table::new([row]).to_string()
}

#[derive(Tabled)]
struct JobTableRow {
    title: String,
    company: String,
    location: String,
    posted: String,
    applicants: String,
}

fn format_jobs_table(jobs: Vec<Job>) -> String {
    let rows: Vec<JobTableRow> = jobs
        .into_iter()
        .map(|job| JobTableRow {
            title: job.title.unwrap_or_default(),
            company: job.company.unwrap_or_default(),
            location: job.location.unwrap_or_default(),
            posted: job.posted_date.unwrap_or_default(),
            applicants: job.applicant_count.map_or_default(|c| c.to_string()),
        })
        .collect();
    
    Table::new(rows).to_string()
}

fn format_job_table(job: Job) -> String {
    let row = JobTableRow {
        title: job.title.unwrap_or_default(),
        company: job.company.unwrap_or_default(),
        location: job.location.unwrap_or_default(),
        posted: job.posted_date.unwrap_or_default(),
        applicants: job.applicant_count.map_or_default(|c| c.to_string()),
    };
    
    Table::new([row]).to_string()
}

#[derive(Tabled)]
struct PeopleTableRow {
    name: String,
    headline: String,
    location: String,
    linkedin_url: String,
}

fn format_people_table(people: Vec<Person>) -> String {
    let rows: Vec<PeopleTableRow> = people
        .into_iter()
        .map(|person| PeopleTableRow {
            name: person.name.unwrap_or_default(),
            headline: person.headline.unwrap_or_default(),
            location: person.location.unwrap_or_default(),
            linkedin_url: person.linkedin_url,
        })
        .collect();
    
    Table::new(rows).to_string()
}

fn write_output(content: String, output_file: Option<String>) -> io::Result<()> {
    match output_file {
        Some(filename) => {
            fs::write(filename, content)?;
        }
        None => {
            print!("{content}");
            io::stdout().flush()?;
        }
    }
    Ok(())
}

trait MapOrDefault<T> {
    fn map_or_default<F>(self, f: F) -> String
    where
        F: FnOnce(T) -> String;
}

impl<T> MapOrDefault<T> for Option<T> {
    fn map_or_default<F>(self, f: F) -> String
    where
        F: FnOnce(T) -> String,
    {
        self.map(f).unwrap_or_default()
    }
} 