use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use scraper::{ Html, Selector, ElementRef };
use htmlentity::entity::{ decode, ICodedDataTrait };
use crate::{
    config::Config,
    utils::HttpClient,
    spiders::{ Spider, Request },
    items::{ PersonProfile, Experience, Education, Project, Language, Activity },
};

#[derive(Clone)]
struct CompiledSelectors {
    summary: Selector,
    name: Selector,
    description: Selector,
    location: Selector,
    followers: Selector,
    connections: Selector,
    subline_item: Selector,
    exp_item: Selector,
    exp_title: Selector,
    exp_location: Selector,
    exp_desc_more: Selector,
    exp_desc_less: Selector,
    exp_date_time: Selector,
    exp_duration: Selector,
    exp_company_logo: Selector,
    edu_item: Selector,
    edu_org: Selector,
    edu_link: Selector,
    edu_details: Selector,
    edu_desc: Selector,
    edu_date_time: Selector,
    projects_items: Selector,
    project_title: Selector,
    project_description: Selector,
    project_link: Selector,
    languages_items: Selector,
    language_name: Selector,
    language_proficiency: Selector,
    activities_items: Selector,
    activity_title: Selector,
    activity_link: Selector,
}

impl CompiledSelectors {
    fn new() -> Self {
        Self {
            summary: Self::parse_selector(crate::selectors::PeopleSelectors::TOP_CARD_LAYOUT),
            name: Self::parse_selector(crate::selectors::PeopleSelectors::NAME),
            description: Self::parse_selector(crate::selectors::PeopleSelectors::DESCRIPTION),
            location: Self::parse_selector(crate::selectors::PeopleSelectors::LOCATION),
            followers: Self::parse_selector(crate::selectors::PeopleSelectors::FOLLOWERS),
            connections: Self::parse_selector(crate::selectors::PeopleSelectors::CONNECTIONS),
            subline_item: Self::parse_selector(crate::selectors::PeopleSelectors::SUBLINE_ITEM),
            exp_item: Self::parse_selector(crate::selectors::PeopleSelectors::EXPERIENCE_ITEM),
            exp_title: Self::parse_selector(crate::selectors::PeopleSelectors::EXPERIENCE_TITLE),
            exp_location: Self::parse_selector(
                crate::selectors::PeopleSelectors::EXPERIENCE_LOCATION
            ),
            exp_desc_more: Self::parse_selector(
                crate::selectors::PeopleSelectors::EXPERIENCE_DESCRIPTION_MORE
            ),
            exp_desc_less: Self::parse_selector(
                crate::selectors::PeopleSelectors::EXPERIENCE_DESCRIPTION_LESS
            ),
            exp_date_time: Self::parse_selector(
                crate::selectors::PeopleSelectors::EXPERIENCE_DATE_TIME
            ),
            exp_duration: Self::parse_selector(
                crate::selectors::PeopleSelectors::EXPERIENCE_DURATION
            ),
            exp_company_logo: Self::parse_selector(
                crate::selectors::PeopleSelectors::EXPERIENCE_EDUCATION_COMPANY_LOGO
            ),
            edu_item: Self::parse_selector(crate::selectors::PeopleSelectors::EDUCATION_ITEM),
            edu_org: Self::parse_selector(
                crate::selectors::PeopleSelectors::EDUCATION_ORGANIZATION
            ),
            edu_link: Self::parse_selector(crate::selectors::PeopleSelectors::EDUCATION_LINK),
            edu_details: Self::parse_selector(crate::selectors::PeopleSelectors::EDUCATION_DETAILS),
            edu_desc: Self::parse_selector(
                crate::selectors::PeopleSelectors::EDUCATION_DESCRIPTION
            ),
            edu_date_time: Self::parse_selector(
                crate::selectors::PeopleSelectors::EDUCATION_DATE_TIME
            ),
            projects_items: Self::parse_selector(crate::selectors::PeopleSelectors::PROJECTS_ITEMS),
            project_title: Self::parse_selector(crate::selectors::PeopleSelectors::PROJECT_TITLE),
            project_description: Self::parse_selector(
                crate::selectors::PeopleSelectors::PROJECT_DESCRIPTION
            ),
            project_link: Self::parse_selector(crate::selectors::PeopleSelectors::PROJECT_LINK),
            languages_items: Self::parse_selector(
                crate::selectors::PeopleSelectors::LANGUAGES_ITEMS
            ),
            language_name: Self::parse_selector(crate::selectors::PeopleSelectors::LANGUAGE_NAME),
            language_proficiency: Self::parse_selector(
                crate::selectors::PeopleSelectors::LANGUAGE_PROFICIENCY
            ),
            activities_items: Self::parse_selector(
                crate::selectors::PeopleSelectors::ACTIVITIES_ITEMS
            ),
            activity_title: Self::parse_selector(crate::selectors::PeopleSelectors::ACTIVITY_TITLE),
            activity_link: Self::parse_selector(crate::selectors::PeopleSelectors::ACTIVITY_LINK),
        }
    }

    fn parse_selector(selector_str: &str) -> Selector {
        Selector::parse(selector_str).unwrap_or_else(|_|
            panic!("Invalid CSS selector: {selector_str}")
        )
    }
}

#[derive(Clone)]
pub struct PeopleProfileSpider {
    config: Arc<Config>,
    http_client: HttpClient,
    profiles: Vec<String>,
    selectors: CompiledSelectors,
}

impl PeopleProfileSpider {
    pub fn new(config: Arc<Config>, profiles: Vec<String>) -> Self {
        let http_client = HttpClient::new(config.clone()).expect("Failed to create HTTP client");
        Self {
            config,
            http_client,
            profiles,
            selectors: CompiledSelectors::new(),
        }
    }

    fn build_url(&self, profile: &str) -> String {
        format!("https://www.linkedin.com/in/{profile}/")
    }

    fn extract_text(element: ElementRef, selector: &Selector) -> Option<String> {
        element
            .select(selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|text| !text.is_empty())
    }

    fn truncate_url(url: &str) -> String {
        url.split('?').next().unwrap_or(url).to_string()
    }

    fn parse_date_range(date_ranges: &[String]) -> (Option<String>, Option<String>) {
        match date_ranges.len() {
            2 => (Some(date_ranges[0].clone()), Some(date_ranges[1].clone())),
            1 => (Some(date_ranges[0].clone()), Some("present".to_string())),
            _ => (None, None),
        }
    }

    fn extract_date_ranges(element: ElementRef, selector: &Selector) -> Vec<String> {
        element
            .select(selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|text| !text.is_empty())
            .collect()
    }

    fn parse_experience(&self, document: &Html) -> Vec<Experience> {
        document
            .select(&self.selectors.exp_item)
            .map(|block| {
                let date_ranges = Self::extract_date_ranges(block, &self.selectors.exp_date_time);
                let (start_time, end_time) = Self::parse_date_range(&date_ranges);

                Experience {
                    organization_profile: block
                        .select(&self.selectors.exp_title)
                        .next()
                        .and_then(|el| el.value().attr("href"))
                        .map(Self::truncate_url),
                    location: Self::extract_text(block, &self.selectors.exp_location),
                    description: Self::extract_text(block, &self.selectors.exp_desc_more).or_else(||
                        Self::extract_text(block, &self.selectors.exp_desc_less)
                    ),
                    duration: Self::extract_text(block, &self.selectors.exp_duration),
                    start_time,
                    end_time,
                    logo: block
                        .select(&self.selectors.exp_company_logo)
                        .next()
                        .and_then(|el| el.value().attr("src"))
                        .map(String::from),
                    title: Self::extract_text(block, &self.selectors.exp_title),
                }
            })
            .collect()
    }

    fn parse_education(&self, document: &Html) -> Vec<Education> {
        document
            .select(&self.selectors.edu_item)
            .map(|block| {
                let course_details = block
                    .select(&self.selectors.edu_details)
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .filter(|text| !text.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ");

                let date_ranges = Self::extract_date_ranges(block, &self.selectors.edu_date_time);
                let (start_time, end_time) = Self::parse_date_range(&date_ranges);

                Education {
                    organization: Self::extract_text(
                        block,
                        &self.selectors.edu_org
                    ).unwrap_or_default(),
                    organization_profile: block
                        .select(&self.selectors.edu_link)
                        .next()
                        .and_then(|el| el.value().attr("href"))
                        .map(Self::truncate_url),
                    course_details: if course_details.is_empty() {
                        None
                    } else {
                        Some(course_details)
                    },
                    description: Self::extract_text(block, &self.selectors.edu_desc),
                    start_time,
                    end_time,
                }
            })
            .collect()
    }

    fn parse_projects(&self, document: &Html) -> Vec<Project> {
        document
            .select(&self.selectors.projects_items)
            .map(|block| Project {
                name: Self::extract_text(block, &self.selectors.project_title),
                description: Self::extract_text(block, &self.selectors.project_description),
                url: block
                    .select(&self.selectors.project_link)
                    .next()
                    .and_then(|el| el.value().attr("href"))
                    .map(Self::truncate_url),
            })
            .collect()
    }

    fn parse_languages(&self, document: &Html) -> Vec<Language> {
        document
            .select(&self.selectors.languages_items)
            .map(|block| Language {
                name: Self::extract_text(block, &self.selectors.language_name),
                proficiency: Self::extract_text(block, &self.selectors.language_proficiency),
            })
            .collect()
    }

    fn parse_activities(&self, document: &Html) -> Vec<Activity> {
        document
            .select(&self.selectors.activities_items)
            .map(|block| Activity {
                title: Self::extract_text(block, &self.selectors.activity_title),
                url: block
                    .select(&self.selectors.activity_link)
                    .next()
                    .and_then(|el| el.value().attr("href"))
                    .map(Self::truncate_url),
            })
            .collect()
    }

    fn extract_location_followers_connections(
        &self,
        summary_box: ElementRef
    ) -> (Option<String>, Option<String>, Option<String>) {
        let mut location = Self::extract_text(summary_box, &self.selectors.location);
        let mut followers = Self::extract_text(summary_box, &self.selectors.followers);
        let mut connections = Self::extract_text(summary_box, &self.selectors.connections);

        if location.is_none() || followers.is_none() || connections.is_none() {
            let subline_items: Vec<String> = summary_box
                .select(&self.selectors.subline_item)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .filter(|text| !text.is_empty())
                .collect();

            for item in subline_items {
                if item.contains("followers") && followers.is_none() {
                    followers = Some(item.replace(" followers", ""));
                } else if item.contains("connections") && connections.is_none() {
                    connections = Some(item.replace(" connections", ""));
                } else if location.is_none() {
                    location = Some(item);
                }
            }
        }

        (location, followers, connections)
    }
}

#[async_trait]
impl Spider for PeopleProfileSpider {
    type Item = PersonProfile;

    fn name(&self) -> &str {
        "linkedin_people_profile"
    }

    fn get_config(&self) -> &Arc<Config> {
        &self.config
    }

    fn get_http_client(&self) -> &HttpClient {
        &self.http_client
    }

    async fn start_requests(&self) -> Vec<Request> {
        self.profiles
            .iter()
            .map(|profile| {
                let url = self.build_url(profile);
                Request::new(url.clone())
                    .with_meta("profile".to_string(), profile.clone())
                    .with_meta("linkedin_url".to_string(), url)
            })
            .collect()
    }

    async fn parse(
        &self,
        response: String,
        request: &Request
    ) -> Result<(Vec<Self::Item>, Vec<Request>)> {
        let profile = request.meta
            .get("profile")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        let url = request.meta
            .get("linkedin_url")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());

        let decoded = decode(response.as_bytes());
        let document = Html::parse_document(&decoded.to_string().unwrap());

        let summary_box = document.select(&self.selectors.summary).next();
        let (location, followers, connections) = if let Some(box_el) = summary_box {
            self.extract_location_followers_connections(box_el)
        } else {
            (None, None, None)
        };

        let person = PersonProfile {
            profile,
            url,
            name: summary_box
                .and_then(|el| Self::extract_text(el, &self.selectors.name))
                .unwrap_or_default(),
            description: summary_box
                .and_then(|el| Self::extract_text(el, &self.selectors.description))
                .unwrap_or_default(),
            experience: self.parse_experience(&document),
            education: self.parse_education(&document),
            location,
            followers,
            connections,
            projects: self.parse_projects(&document),
            languages: self.parse_languages(&document),
            activities: self.parse_activities(&document),
        };

        Ok((vec![person], vec![]))
    }
}
