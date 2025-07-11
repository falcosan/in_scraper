use crate::client::LinkedInClient;
use crate::error::{LinkedInError, Result};
use crate::models::{Person, Experience, Education};
use crate::selectors;
use scraper::{Html, Selector};
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

    fn extract_experiences(&self, document: &Html) -> Result<Vec<Experience>> {
        let mut experiences = Vec::new();
        
        let experience_selector = Selector::parse(selectors::person::EXPERIENCE_SECTION)
            .map_err(|e| LinkedInError::ParseError(e.to_string()))?;
        
        if let Some(experience_section) = document.select(&experience_selector).next() {
            let item_selector = Selector::parse(".pvs-list__paged-list-item")
                .map_err(|e| LinkedInError::ParseError(e.to_string()))?;
            
            for item in experience_section.select(&item_selector) {
                if let Some(experience) = self.parse_experience_item(&item) {
                    experiences.push(experience);
                }
            }
        }

        Ok(experiences)
    }

    fn parse_experience_item(&self, item: &scraper::ElementRef) -> Option<Experience> {
        let title_selector = Selector::parse(".t-bold span").ok()?;
        let company_selector = Selector::parse(".t-normal span").ok()?;
        let duration_selector = Selector::parse(".t-black--light span").ok()?;
        let location_selector = Selector::parse(".t-black--light span").ok()?;

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

        let company_link_selector = Selector::parse("a[href*='/company/']").ok()?;
        let company_linkedin_url = item.select(&company_link_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://linkedin.com{}", href)
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
        
        let education_selector = Selector::parse(selectors::person::EDUCATION_SECTION)
            .map_err(|e| LinkedInError::ParseError(e.to_string()))?;
        
        if let Some(education_section) = document.select(&education_selector).next() {
            let item_selector = Selector::parse(".pvs-list__paged-list-item")
                .map_err(|e| LinkedInError::ParseError(e.to_string()))?;
            
            for item in education_section.select(&item_selector) {
                if let Some(education) = self.parse_education_item(&item) {
                    educations.push(education);
                }
            }
        }

        Ok(educations)
    }

    fn parse_education_item(&self, item: &scraper::ElementRef) -> Option<Education> {
        let school_selector = Selector::parse(".t-bold span").ok()?;
        let degree_selector = Selector::parse(".t-normal span").ok()?;
        let duration_selector = Selector::parse(".t-black--light span").ok()?;

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

        let school_link_selector = Selector::parse("a[href*='/school/']").ok()?;
        let school_linkedin_url = item.select(&school_link_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|href| {
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://linkedin.com{}", href)
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
} 