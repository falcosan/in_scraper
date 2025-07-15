use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use scraper::{ Html, Selector, ElementRef };
use crate::{
    config::Config,
    utils::HttpClient,
    spiders::{ Spider, Request },
    items::{ PersonProfile, Experience, Education, Project, Language, Activity },
};

#[derive(Clone)]
pub struct PeopleProfileSpider {
    config: Arc<Config>,
    http_client: HttpClient,
    profiles: Vec<String>,
    summary_selector: Selector,
    name_selector: Selector,
    description_selector: Selector,
    location_selector: Selector,
    followers_selector: Selector,
    connections_selector: Selector,
    subline_item_selector: Selector,
    about_selector: Selector,
    about_section_selector: Selector,
    about_text_selector: Selector,
    exp_item_selector: Selector,
    exp_title_selector: Selector,
    exp_location_selector: Selector,
    exp_desc_more_selector: Selector,
    exp_desc_less_selector: Selector,
    exp_date_time_selector: Selector,
    exp_duration_selector: Selector,
    exp_company_logo_selector: Selector,
    exp_company_name_selector: Selector,
    edu_item_selector: Selector,
    edu_org_selector: Selector,
    edu_link_selector: Selector,
    edu_details_selector: Selector,
    edu_desc_selector: Selector,
    edu_date_time_selector: Selector,
    projects_section_selector: Selector,
    projects_items_selector: Selector,
    languages_section_selector: Selector,
    languages_items_selector: Selector,
    activities_section_selector: Selector,
    activities_items_selector: Selector,
    exp_edu_section_selector: Selector,
    exp_edu_items_selector: Selector,
    exp_edu_title_selector: Selector,
    exp_edu_details_selector: Selector,
    core_section_container_selector: Selector,
    core_section_title_selector: Selector,
    core_section_content_selector: Selector,
}

impl PeopleProfileSpider {
    pub fn new(config: Arc<Config>, profiles: Vec<String>) -> Self {
        let http_client = HttpClient::new(config.clone()).expect("Failed to create HTTP client");
        Self {
            config,
            http_client,
            profiles,
            summary_selector: Selector::parse(
                crate::selectors::PeopleSelectors::TOP_CARD_LAYOUT
            ).unwrap(),
            name_selector: Selector::parse(crate::selectors::PeopleSelectors::NAME).unwrap(),
            description_selector: Selector::parse(
                crate::selectors::PeopleSelectors::DESCRIPTION
            ).unwrap(),
            location_selector: Selector::parse(
                crate::selectors::PeopleSelectors::LOCATION
            ).unwrap(),
            followers_selector: Selector::parse(
                crate::selectors::PeopleSelectors::FOLLOWERS
            ).unwrap(),
            connections_selector: Selector::parse(
                crate::selectors::PeopleSelectors::CONNECTIONS
            ).unwrap(),
            subline_item_selector: Selector::parse(
                crate::selectors::PeopleSelectors::SUBLINE_ITEM
            ).unwrap(),
            about_selector: Selector::parse(crate::selectors::PeopleSelectors::ABOUT).unwrap(),
            about_section_selector: Selector::parse(
                crate::selectors::PeopleSelectors::ABOUT_SECTION
            ).unwrap(),
            about_text_selector: Selector::parse(
                crate::selectors::PeopleSelectors::ABOUT_TEXT
            ).unwrap(),
            exp_item_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_ITEM
            ).unwrap(),
            exp_title_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_TITLE
            ).unwrap(),
            exp_location_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_LOCATION
            ).unwrap(),
            exp_desc_more_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_DESCRIPTION_MORE
            ).unwrap(),
            exp_desc_less_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_DESCRIPTION_LESS
            ).unwrap(),
            exp_date_time_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_DATE_TIME
            ).unwrap(),
            exp_duration_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_DURATION
            ).unwrap(),
            exp_company_logo_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_EDUCATION_COMPANY_LOGO
            ).unwrap(),
            exp_company_name_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_EDUCATION_COMPANY_NAME
            ).unwrap(),
            edu_item_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EDUCATION_ITEM
            ).unwrap(),
            edu_org_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EDUCATION_ORGANIZATION
            ).unwrap(),
            edu_link_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EDUCATION_LINK
            ).unwrap(),
            edu_details_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EDUCATION_DETAILS
            ).unwrap(),
            edu_desc_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EDUCATION_DESCRIPTION
            ).unwrap(),
            edu_date_time_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EDUCATION_DATE_TIME
            ).unwrap(),
            projects_section_selector: Selector::parse(
                crate::selectors::PeopleSelectors::PROJECTS_SECTION
            ).unwrap(),
            projects_items_selector: Selector::parse(
                crate::selectors::PeopleSelectors::PROJECTS_ITEMS
            ).unwrap(),
            languages_section_selector: Selector::parse(
                crate::selectors::PeopleSelectors::LANGUAGES_SECTION
            ).unwrap(),
            languages_items_selector: Selector::parse(
                crate::selectors::PeopleSelectors::LANGUAGES_ITEMS
            ).unwrap(),
            activities_section_selector: Selector::parse(
                crate::selectors::PeopleSelectors::ACTIVITIES_SECTION
            ).unwrap(),
            activities_items_selector: Selector::parse(
                crate::selectors::PeopleSelectors::ACTIVITIES_ITEMS
            ).unwrap(),
            exp_edu_section_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_EDUCATION_SECTION
            ).unwrap(),
            exp_edu_items_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_EDUCATION_ITEMS
            ).unwrap(),
            exp_edu_title_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_EDUCATION_TITLE
            ).unwrap(),
            exp_edu_details_selector: Selector::parse(
                crate::selectors::PeopleSelectors::EXPERIENCE_EDUCATION_DETAILS
            ).unwrap(),
            core_section_container_selector: Selector::parse(
                crate::selectors::PeopleSelectors::CORE_SECTION_CONTAINER
            ).unwrap(),
            core_section_title_selector: Selector::parse(
                crate::selectors::PeopleSelectors::CORE_SECTION_TITLE
            ).unwrap(),
            core_section_content_selector: Selector::parse(
                crate::selectors::PeopleSelectors::CORE_SECTION_CONTENT
            ).unwrap(),
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
    }

    fn truncate_url(url: &str) -> String {
        url.split('?').next().unwrap_or(url).to_string()
    }

    fn parse_experience(&self, document: &Html) -> Vec<Experience> {
        document
            .select(&self.exp_item_selector)
            .map(|block| {
                let date_ranges: Vec<_> = block
                    .select(&self.exp_date_time_selector)
                    .map(|el| el.text().collect::<String>())
                    .collect();

                let (start_time, end_time) = match date_ranges.len() {
                    2 => (Some(date_ranges[0].clone()), Some(date_ranges[1].clone())),
                    1 => (Some(date_ranges[0].clone()), Some("present".to_string())),
                    _ => (None, None),
                };

                Experience {
                    organization_profile: block
                        .select(&self.exp_title_selector)
                        .next()
                        .and_then(|el| el.value().attr("href"))
                        .map(Self::truncate_url),
                    location: Self::extract_text(block, &self.exp_location_selector),
                    description: Self::extract_text(block, &self.exp_desc_more_selector).or_else(||
                        Self::extract_text(block, &self.exp_desc_less_selector)
                    ),
                    duration: Self::extract_text(block, &self.exp_duration_selector),
                    start_time,
                    end_time,
                    logo: block
                        .select(&self.exp_company_logo_selector)
                        .next()
                        .and_then(|el| el.value().attr("src"))
                        .map(|s| s.to_string()),
                    title: Self::extract_text(block, &self.exp_title_selector),
                }
            })
            .collect()
    }

    fn parse_education(&self, document: &Html) -> Vec<Education> {
        document
            .select(&self.edu_item_selector)
            .map(|block| {
                let course_details = block
                    .select(&self.edu_details_selector)
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .collect::<Vec<_>>()
                    .join(" ");

                let date_ranges: Vec<_> = block
                    .select(&self.edu_date_time_selector)
                    .map(|el| el.text().collect::<String>())
                    .collect();

                let (start_time, end_time) = match date_ranges.len() {
                    2 => (Some(date_ranges[0].clone()), Some(date_ranges[1].clone())),
                    1 => (Some(date_ranges[0].clone()), Some("present".to_string())),
                    _ => (None, None),
                };

                Education {
                    organization: Self::extract_text(
                        block,
                        &self.edu_org_selector
                    ).unwrap_or_default(),
                    organization_profile: block
                        .select(&self.edu_link_selector)
                        .next()
                        .and_then(|el| el.value().attr("href"))
                        .map(Self::truncate_url),
                    course_details: if course_details.is_empty() {
                        None
                    } else {
                        Some(course_details)
                    },
                    description: Self::extract_text(block, &self.edu_desc_selector),
                    start_time,
                    end_time,
                }
            })
            .collect()
    }

    fn parse_projects(&self, document: &Html) -> Vec<Project> {
        document
            .select(&self.projects_items_selector)
            .map(|block| {
                Project {
                    name: Self::extract_text(block, &self.exp_title_selector),
                    description: Self::extract_text(block, &self.exp_desc_less_selector),
                    url: block
                        .select(&self.edu_link_selector)
                        .next()
                        .and_then(|el| el.value().attr("href"))
                        .map(Self::truncate_url),
                }
            })
            .collect()
    }

    fn parse_languages(&self, document: &Html) -> Vec<Language> {
        document
            .select(&self.languages_items_selector)
            .map(|block| {
                Language {
                    name: Self::extract_text(block, &self.exp_company_name_selector),
                    proficiency: Self::extract_text(block, &self.exp_desc_less_selector),
                }
            })
            .collect()
    }

    fn parse_activities(&self, document: &Html) -> Vec<Activity> {
        document
            .select(&self.activities_items_selector)
            .map(|block| {
                Activity {
                    title: Self::extract_text(block, &self.exp_title_selector),
                    url: block
                        .select(&self.edu_link_selector)
                        .next()
                        .and_then(|el| el.value().attr("href"))
                        .map(Self::truncate_url),
                }
            })
            .collect()
    }

    fn parse_about(&self, document: &Html) -> Option<String> {
        if let Some(about_section) = document.select(&self.about_section_selector).next() {
            if let Some(about_text) = Self::extract_text(about_section, &self.about_text_selector) {
                return Some(about_text);
            }
        }

        document
            .select(&self.about_selector)
            .next()
            .map(|el| el.text().collect())
    }

    fn extract_location_followers_connections(
        &self,
        summary_box: ElementRef
    ) -> (Option<String>, Option<String>, Option<String>) {
        let mut location = Self::extract_text(summary_box, &self.location_selector);
        let mut followers = Self::extract_text(summary_box, &self.followers_selector);
        let mut connections = Self::extract_text(summary_box, &self.connections_selector);

        if location.is_none() || followers.is_none() || connections.is_none() {
            let subline_items: Vec<_> = summary_box
                .select(&self.subline_item_selector)
                .map(|el| el.text().collect::<String>().trim().to_string())
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
        let profile = request.meta.get("profile").cloned().unwrap_or_default();
        let url = request.meta.get("linkedin_url").cloned().unwrap_or_default();
        let document = Html::parse_document(&response);

        let summary_box = document.select(&self.summary_selector).next();
        let (location, followers, connections) = if let Some(box_el) = summary_box {
            self.extract_location_followers_connections(box_el)
        } else {
            (None, None, None)
        };

        let person = PersonProfile {
            profile,
            url,
            name: summary_box
                .and_then(|el| Self::extract_text(el, &self.name_selector))
                .unwrap_or_default(),
            description: summary_box
                .and_then(|el| Self::extract_text(el, &self.description_selector))
                .unwrap_or_default(),
            about: self.parse_about(&document),
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
