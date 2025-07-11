use crate::client::LinkedInClient;
use crate::error::Result;
use crate::models::{Person, Experience, Education, Job, Company, Employee};
use crate::selectors;
use scraper::Html;
use regex::Regex;

impl LinkedInClient {
    pub async fn scrape_person(&self, linkedin_url: &str) -> Result<Person> {
        let mut person = Person::new(linkedin_url.to_string());
        let document = self.get_html(linkedin_url).await?;

        person.name = self.extract_text_by_selector(&document, selectors::person::NAME);
        person.headline = self.extract_text_by_selector(&document, selectors::person::HEADLINE);
        person.location = self.extract_text_by_selector(&document, selectors::person::LOCATION);
        person.about = self.extract_text_by_selector(&document, selectors::person::ABOUT);
        person.open_to_work = self.extract_attribute_by_selector(
            &document,
            selectors::person::OPEN_TO_WORK,
            "title"
        ).is_some();

        person.experiences = self.extract_experiences(&document)?;
        person.educations = self.extract_educations(&document)?;

        Ok(person)
    }

    pub async fn search_people(&self, query: &str, location: Option<&str>) -> Result<Vec<Person>> {
        let mut search_url = format!(
            "https://www.linkedin.com/search/results/people/?keywords={}",
            urlencoding::encode(query)
        );

        if let Some(loc) = location {
            search_url.push_str(&format!("&geoUrn={}", urlencoding::encode(loc)));
        }

        let document = self.get_html(&search_url).await?;
        self.extract_people_listings(&document)
    }

    fn extract_people_listings(&self, document: &Html) -> Result<Vec<Person>> {
        let mut people = Vec::new();
        
        let person_card_selector = self.select_working_selector(document, selectors::person::SEARCH_CARDS)?;
        
        for person_card in document.select(&person_card_selector) {
            if let Some(person) = self.parse_person_card(&person_card) {
                people.push(person);
            }
        }

        Ok(people)
    }

    fn parse_person_card(&self, card: &scraper::ElementRef) -> Option<Person> {
        let title_selector = self.select_working_selector_for_element(card, selectors::person::SEARCH_TITLES).ok()?;
        let headline_selector = self.select_working_selector_for_element(card, selectors::person::SEARCH_HEADLINES).ok()?;
        let location_selector = self.select_working_selector_for_element(card, selectors::person::SEARCH_LOCATIONS).ok()?;

        let title_element = card.select(&title_selector).next()?;
        let name = title_element.text().collect::<String>().trim().to_string();
        
        let linkedin_url = title_element.value().attr("href")
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://linkedin.com{href}")
                }
            })?;

        let headline = card.select(&headline_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let location = card.select(&location_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let mut person = Person::new(linkedin_url);
        person.name = Some(name);
        person.headline = headline;
        person.location = location;

        Some(person)
    }

    fn extract_experiences(&self, document: &Html) -> Result<Vec<Experience>> {
        let mut experiences = Vec::new();
        
        let experience_section_selector = self.make_selector(selectors::person::EXPERIENCE_SECTION)?;
        
        if let Some(experience_section) = document.select(&experience_section_selector).next() {
            let item_selector = self.make_selector(selectors::person::EXPERIENCE_ITEMS)?;
            
            for item in experience_section.select(&item_selector) {
                if let Some(experience) = self.parse_experience_item(&item) {
                    experiences.push(experience);
                }
            }
        }

        Ok(experiences)
    }

    fn parse_experience_item(&self, item: &scraper::ElementRef) -> Option<Experience> {
        let title_selector = self.make_selector(selectors::person::EXPERIENCE_TITLES).ok()?;
        let company_selector = self.make_selector(selectors::person::EXPERIENCE_COMPANIES).ok()?;
        let duration_selector = self.make_selector(selectors::person::EXPERIENCE_INFO).ok()?;
        let location_selector = self.make_selector(selectors::person::EXPERIENCE_INFO).ok()?;
        let company_link_selector = self.make_selector(selectors::person::EXPERIENCE_COMPANY_LINKS).ok()?;

        let title = item.select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let company = item.select(&company_selector)
            .nth(1)
            .map(|el| el.text().collect::<String>().trim().to_string());

        let duration_text = item.select(&duration_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let (from_date, to_date, duration) = self.parse_duration(&duration_text.unwrap_or_default());

        let location = item.select(&location_selector)
            .last()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let company_linkedin_url = item.select(&company_link_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://linkedin.com{href}")
                }
            });

        Some(Experience {
            title,
            company,
            company_linkedin_url,
            location,
            from_date,
            to_date,
            duration,
            description: None,
        })
    }

    fn extract_educations(&self, document: &Html) -> Result<Vec<Education>> {
        let mut educations = Vec::new();
        
        let education_section_selector = self.make_selector(selectors::person::EDUCATION_SECTION)? ;

        if let Some(education_section) = document.select(&education_section_selector).next() {
            let item_selector = self.make_selector(selectors::person::EDUCATION_ITEMS)?;
            
            for item in education_section.select(&item_selector) {
                if let Some(education) = self.parse_education_item(&item) {
                    educations.push(education);
                }
            }
        }

        Ok(educations)
    }

    fn parse_education_item(&self, item: &scraper::ElementRef) -> Option<Education> {
        let school_selector = self.make_selector(selectors::person::EDUCATION_SCHOOLS).ok()?;
        let degree_selector = self.make_selector(selectors::person::EDUCATION_DEGREES).ok()?;
        let duration_selector = self.make_selector(selectors::person::EDUCATION_DURATIONS).ok()?;
        let school_link_selector = self.make_selector(selectors::person::EDUCATION_SCHOOL_LINKS).ok()?;

        let school = item.select(&school_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let degree = item.select(&degree_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let duration_text = item.select(&duration_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let (from_date, to_date, _) = self.parse_duration(&duration_text.unwrap_or_default());

        let school_linkedin_url = item.select(&school_link_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://linkedin.com{href}")
                }
            });

        Some(Education {
            school,
            school_linkedin_url,
            degree,
            field_of_study: None,
            from_date,
            to_date,
            description: None,
        })
    }

    fn parse_duration(&self, duration_text: &str) -> (Option<String>, Option<String>, Option<String>) {
        let duration_regex = Regex::new(r"(\w+\s+\d{4})\s*[-–]\s*(\w+\s+\d{4}|Present)").unwrap();
        
        if let Some(captures) = duration_regex.captures(duration_text) {
            let from_date = captures.get(1).map(|m| m.as_str().to_string());
            let to_date = captures.get(2).map(|m| m.as_str().to_string());
            
            let duration = if duration_text.contains("·") {
                duration_text.split("·").nth(1).map(|s| s.trim().to_string())
            } else {
                None
            };

            (from_date, to_date, duration)
        } else {
            (None, None, Some(duration_text.to_string()))
        }
    }
    pub async fn scrape_job(&self, linkedin_url: &str) -> Result<Job> {
        let mut job = Job::new(linkedin_url.to_string());
        let document = self.get_html(linkedin_url).await?;

        job.title = self.extract_text_by_selector(&document, selectors::job::TITLE);
        job.company = self.extract_text_by_selector(&document, selectors::job::COMPANY);
        job.location = self.extract_text_by_selector(&document, selectors::job::LOCATION);
        job.description = self.extract_text_by_selector(&document, selectors::job::DESCRIPTION);
        job.posted_date = self.extract_text_by_selector(&document, selectors::job::POSTED_DATE);

        if let Some(applicant_text) = self.extract_text_by_selector(&document, selectors::job::APPLICANT_COUNT) {
            job.applicant_count = self.parse_applicant_count(&applicant_text);
        }

        job.company_linkedin_url = self.extract_attribute_by_selector(
            &document,
            selectors::job::COMPANY_LINK,
            "href"
        );

        job.employment_type = self.extract_employment_type(&document);
        job.seniority_level = self.extract_seniority_level(&document);

        Ok(job)
    }

    pub async fn search_jobs(&self, query: &str, location: Option<&str>) -> Result<Vec<Job>> {
        let mut search_url = format!(
            "https://www.linkedin.com/jobs/search/?keywords={}",
            urlencoding::encode(query)
        );

        if let Some(loc) = location {
            search_url.push_str(&format!("&location={}", urlencoding::encode(loc)));
        }

        let document = self.get_html(&search_url).await?;
        self.extract_job_listings(&document)
    }

    fn extract_job_listings(&self, document: &Html) -> Result<Vec<Job>> {
        let mut jobs = Vec::new();
        
        let job_card_selector = self.select_working_selector(document, selectors::job::SEARCH_CARDS)?;
        
        for job_card in document.select(&job_card_selector) {
            if let Some(job) = self.parse_job_card(&job_card) {
                jobs.push(job);
            }
        }

        Ok(jobs)
    }

    fn parse_job_card(&self, card: &scraper::ElementRef) -> Option<Job> {
        let title_selector = self.select_working_selector_for_element(card, selectors::job::SEARCH_TITLES).ok()?;
        
        let company_selector = self.select_working_selector_for_element(card, selectors::job::SEARCH_COMPANIES).ok()?;
        
        let location_selector = self.make_selector(selectors::job::SEARCH_LOCATIONS).ok()?;
        
        let posted_date_selector = self.select_working_selector_for_element(card, selectors::job::SEARCH_POSTED_DATES).ok()?;

        let title_element = card.select(&title_selector).next()?;
        let title = title_element.text().collect::<String>().trim().to_string();
        
        let linkedin_url = title_element.value().attr("href")
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://linkedin.com{href}")
                }
            })?;

        let company = card.select(&company_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let location = card.select(&location_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let posted_date = card.select(&posted_date_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let company_link_selector = self.make_selector(selectors::job::SEARCH_COMPANY_LINKS).ok()?;
        let company_linkedin_url = card.select(&company_link_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://linkedin.com{href}")
                }
            });

        Some(Job {
            linkedin_url,
            title: Some(title),
            company,
            company_linkedin_url,
            location,
            posted_date,
            applicant_count: None,
            description: None,
            benefits: None,
            employment_type: None,
            seniority_level: None,
        })
    }

    fn parse_applicant_count(&self, applicant_text: &str) -> Option<i32> {
        let count_regex = Regex::new(r"(\d+)").unwrap();
        count_regex.captures(applicant_text)
            .and_then(|captures| captures.get(1))
            .and_then(|count_match| count_match.as_str().parse().ok())
    }

    fn extract_employment_type(&self, document: &Html) -> Option<String> {
        self.extract_multiple_text_by_selector(document, selectors::job::JOB_INSIGHTS)
            .into_iter()
            .find(|text| {
                text.contains("Full-time") || 
                text.contains("Part-time") || 
                text.contains("Contract") || 
                text.contains("Temporary") ||
                text.contains("Internship")
            })
    }

    fn extract_seniority_level(&self, document: &Html) -> Option<String> {
        self.extract_multiple_text_by_selector(document, selectors::job::JOB_INSIGHTS)
            .into_iter()
            .find(|text| {
                text.contains("Entry level") || 
                text.contains("Associate") || 
                text.contains("Mid-Senior level") || 
                text.contains("Director") ||
                text.contains("Executive")
            })
    }
    pub async fn scrape_company(&self, linkedin_url: &str) -> Result<Company> {
        let mut company = Company::new(linkedin_url.to_string());
        let document = self.get_html(linkedin_url).await?;

        company.name = self.extract_text_by_selector(&document, selectors::company::NAME);
        company.about = self.extract_text_by_selector(&document, selectors::company::ABOUT);
        company.website = self.extract_attribute_by_selector(&document, selectors::company::WEBSITE, "href");
        company.headquarters = self.extract_text_by_selector(&document, selectors::company::HEADQUARTERS);
        company.industry = self.extract_text_by_selector(&document, selectors::company::INDUSTRY);
        company.company_size = self.extract_text_by_selector(&document, selectors::company::COMPANY_SIZE);

        if let Some(founded_text) = self.extract_text_by_selector(&document, selectors::company::FOUNDED) {
            company.founded = self.parse_founded_year(&founded_text);
        }

        company.specialties = self.extract_specialties(&document);
        company.employees = self.extract_employees(&document)?;

        Ok(company)
    }

    pub async fn scrape_company_employees(&self, company_url: &str) -> Result<Vec<Employee>> {
        let employees_url = format!("{company_url}/people");
        let document = self.get_html(&employees_url).await?;
        self.extract_employees(&document)
    }

    fn extract_specialties(&self, document: &Html) -> Vec<String> {
        self.extract_multiple_text_by_selector(document, selectors::company::SPECIALTIES_ITEMS)
    }

    fn extract_employees(&self, document: &Html) -> Result<Vec<Employee>> {
        let mut employees = Vec::new();
        
        let employee_selector = self.select_working_selector(document, selectors::company::EMPLOYEE_CARDS)?;
        
        for employee_element in document.select(&employee_selector) {
            if let Some(employee) = self.parse_employee_item(&employee_element) {
                employees.push(employee);
            }
        }

        Ok(employees)
    }

    fn parse_employee_item(&self, item: &scraper::ElementRef) -> Option<Employee> {
        let name_selector = self.select_working_selector_for_element(item, selectors::company::EMPLOYEE_NAMES).ok()?;
        
        let title_selector = self.select_working_selector_for_element(item, selectors::company::EMPLOYEE_TITLES).ok()?;
        
        let link_selector = self.make_selector(selectors::company::EMPLOYEE_LINKS).ok()?;

        let name = item.select(&name_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())?;

        let title = item.select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        let linkedin_url = item.select(&link_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://linkedin.com{href}")
                }
            });

        Some(Employee {
            name,
            title,
            linkedin_url,
        })
    }

    fn parse_founded_year(&self, founded_text: &str) -> Option<i32> {
        let year_regex = Regex::new(r"\b(\d{4})\b").unwrap();
        year_regex.captures(founded_text)
            .and_then(|captures| captures.get(1))
            .and_then(|year_match| year_match.as_str().parse().ok())
    }
} 