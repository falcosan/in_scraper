use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use scraper::{ Html, Selector, ElementRef };
use crate::{
    config::Config,
    utils::HttpClient,
    spiders::{ Spider, Request },
    items::{ PersonProfile, Experience, Education },
};

#[derive(Clone)]
pub struct PeopleProfileSpider {
    config: Arc<Config>,
    http_client: HttpClient,
    profiles: Vec<String>,
    summary_selector: Selector,
    name_selector: Selector,
    description_selector: Selector,
    subline_item_selector: Selector,
    about_selector: Selector,
    exp_item_selector: Selector,
    exp_title_selector: Selector,
    exp_location_selector: Selector,
    exp_desc_more_selector: Selector,
    exp_desc_less_selector: Selector,
    exp_date_time_selector: Selector,
    exp_duration_selector: Selector,
    edu_item_selector: Selector,
    edu_org_selector: Selector,
    edu_link_selector: Selector,
    edu_details_selector: Selector,
    edu_desc_selector: Selector,
    edu_date_time_selector: Selector,
}

impl PeopleProfileSpider {
    pub fn new(config: Arc<Config>, profiles: Vec<String>) -> Self {
        let http_client = HttpClient::new(config.clone()).expect("Failed to create HTTP client");
        Self {
            config,
            http_client,
            profiles,
            summary_selector: Selector::parse("section.top-card-layout").unwrap(),
            name_selector: Selector::parse(crate::selectors::PeopleSelectors::NAME).unwrap(),
            description_selector: Selector::parse(
                crate::selectors::PeopleSelectors::DESCRIPTION
            ).unwrap(),
            subline_item_selector: Selector::parse(
                crate::selectors::PeopleSelectors::SUBLINE_ITEM
            ).unwrap(),
            about_selector: Selector::parse(crate::selectors::PeopleSelectors::ABOUT).unwrap(),
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
        let (mut location, mut followers, mut connections) = (None, None, None);

        if let Some(box_el) = summary_box {
            let subline_items: Vec<_> = box_el
                .select(&self.subline_item_selector)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .collect();

            for item in subline_items {
                if item.contains("followers") {
                    followers = Some(item.replace(" followers", ""));
                } else if item.contains("connections") {
                    connections = Some(item.replace(" connections", ""));
                } else {
                    location.get_or_insert(item);
                }
            }
        }

        let person = PersonProfile {
            profile,
            url,
            name: summary_box
                .and_then(|el| Self::extract_text(el, &self.name_selector))
                .unwrap_or_default(),
            description: summary_box
                .and_then(|el| Self::extract_text(el, &self.description_selector))
                .unwrap_or_default(),
            about: document
                .select(&self.about_selector)
                .next()
                .map(|el| el.text().collect()),
            experience: self.parse_experience(&document),
            education: self.parse_education(&document),
            location,
            followers,
            connections,
        };

        Ok((vec![person], vec![]))
    }
}
