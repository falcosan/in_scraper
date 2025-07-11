use crate::client::LinkedInClient;
use crate::error::Result;
use crate::models::{Company, Employee};
use crate::selectors;
use scraper::{Html, Selector};
use regex::Regex;

impl LinkedInClient {
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
        company.employees = self.extract_employees(&document);

        Ok(company)
    }

    pub async fn scrape_company_employees(&self, company_url: &str) -> Result<Vec<Employee>> {
        let employees_url = format!("{}/people", company_url);
        let document = self.get_html(&employees_url).await?;
        Ok(self.extract_employees(&document))
    }

    fn extract_specialties(&self, document: &Html) -> Vec<String> {
        let specialties_selector = ".org-about-company-module__specialties .org-about-company-module__specialties-item";
        self.extract_multiple_text_by_selector(document, specialties_selector)
    }

    fn extract_employees(&self, document: &Html) -> Vec<Employee> {
        let mut employees = Vec::new();
        
        let employee_selector = Selector::parse(".org-people-profile-card").unwrap_or_else(|_| {
            Selector::parse(".list-style-none li").unwrap()
        });
        
        for employee_element in document.select(&employee_selector) {
            if let Some(employee) = self.parse_employee_item(&employee_element) {
                employees.push(employee);
            }
        }

        employees
    }

    fn parse_employee_item(&self, item: &scraper::ElementRef) -> Option<Employee> {
        let name_selector = Selector::parse(".org-people-profile-card__profile-title").ok()
            .or_else(|| Selector::parse(".t-16 .t-black .t-bold").ok())?;
        
        let title_selector = Selector::parse(".org-people-profile-card__profile-info").ok()
            .or_else(|| Selector::parse(".t-14 .t-black--light").ok())?;
        
        let link_selector = Selector::parse("a[href*='/in/']").ok()?;

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
                    format!("https://linkedin.com{}", href)
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