use crate::client::LinkedInClient;
use crate::error::{LinkedInError, Result};
use crate::models::Job;
use crate::selectors;
use scraper::{Html, Selector};
use regex::Regex;

impl LinkedInClient {
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
            ".job-details-jobs-unified-top-card__company-name a",
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
        
        let job_card_selector = Selector::parse(".job-search-card")
            .or_else(|_| Selector::parse(".jobs-search-results__list-item"))
            .map_err(|e| LinkedInError::ParseError(e.to_string()))?;
        
        for job_card in document.select(&job_card_selector) {
            if let Some(job) = self.parse_job_card(&job_card) {
                jobs.push(job);
            }
        }

        Ok(jobs)
    }

    fn parse_job_card(&self, card: &scraper::ElementRef) -> Option<Job> {
        let title_selector = Selector::parse(".job-search-card__title a").ok()
            .or_else(|| Selector::parse("h3 a").ok())?;
        
        let company_selector = Selector::parse(".job-search-card__subtitle").ok()
            .or_else(|| Selector::parse(".job-search-card__subtitle-link").ok())?;
        
        let location_selector = Selector::parse(".job-search-card__location").ok()?;
        
        let posted_date_selector = Selector::parse(".job-search-card__listdate").ok()
            .or_else(|| Selector::parse(".job-posted-date").ok())?;

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

        let company_link_selector = Selector::parse("a[href*='/company/']").ok();
        let company_linkedin_url = company_link_selector
            .and_then(|selector| card.select(&selector).next())
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
        let employment_selector = ".job-details-jobs-unified-top-card__job-insight .job-details-jobs-unified-top-card__job-insight-value-list li";
        self.extract_multiple_text_by_selector(document, employment_selector)
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
        let seniority_selector = ".job-details-jobs-unified-top-card__job-insight .job-details-jobs-unified-top-card__job-insight-value-list li";
        self.extract_multiple_text_by_selector(document, seniority_selector)
            .into_iter()
            .find(|text| {
                text.contains("Entry level") || 
                text.contains("Associate") || 
                text.contains("Mid-Senior level") || 
                text.contains("Director") ||
                text.contains("Executive")
            })
    }
} 