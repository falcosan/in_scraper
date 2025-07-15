use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use scraper::{ Html, Selector };
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
}

impl PeopleProfileSpider {
    pub fn new(config: Arc<Config>, profiles: Vec<String>) -> Self {
        let http_client = HttpClient::new(config.clone()).expect("Failed to create HTTP client");
        Self {
            config,
            http_client,
            profiles,
        }
    }

    fn build_url(&self, profile: &str) -> String {
        format!("https://www.linkedin.com/in/{}/", profile)
    }

    fn parse_experience(
        &self,
        document: &Html,
        exp_selector: &Selector,
        title_selector: &Selector,
        location_selector: &Selector,
        desc_more_selector: &Selector,
        desc_less_selector: &Selector,
        date_time_selector: &Selector,
        duration_selector: &Selector
    ) -> Vec<Experience> {
        let mut experiences = Vec::new();
        for block in document.select(exp_selector) {
            let organisation_profile = block
                .select(title_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| href.split('?').next().unwrap_or(href).to_string());

            let location = block
                .select(location_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string());

            let description = block
                .select(desc_more_selector)
                .next()
                .or_else(|| block.select(desc_less_selector).next())
                .map(|el| el.text().collect::<String>().trim().to_string());

            let date_ranges: Vec<_> = block
                .select(date_time_selector)
                .map(|el| el.text().collect::<String>())
                .collect();

            let (start_time, end_time, duration) = match date_ranges.len() {
                2 =>
                    (
                        Some(date_ranges[0].clone()),
                        Some(date_ranges[1].clone()),
                        block
                            .select(duration_selector)
                            .next()
                            .map(|el| el.text().collect::<String>()),
                    ),
                1 =>
                    (
                        Some(date_ranges[0].clone()),
                        Some("present".to_string()),
                        block
                            .select(duration_selector)
                            .next()
                            .map(|el| el.text().collect::<String>()),
                    ),
                _ => (None, None, None),
            };

            experiences.push(Experience {
                organisation_profile,
                location,
                description,
                start_time,
                end_time,
                duration,
            });
        }
        experiences
    }

    fn parse_education(
        &self,
        document: &Html,
        edu_selector: &Selector,
        org_selector: &Selector,
        link_selector: &Selector,
        details_selector: &Selector,
        desc_selector: &Selector,
        date_time_selector: &Selector
    ) -> Vec<Education> {
        let mut educations = Vec::new();
        for block in document.select(edu_selector) {
            let organisation = block
                .select(org_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let organisation_profile = block
                .select(link_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| href.split('?').next().unwrap_or(href).to_string());

            let course_details = block
                .select(details_selector)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            let description = block
                .select(desc_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string());

            let date_ranges: Vec<_> = block
                .select(date_time_selector)
                .map(|el| el.text().collect::<String>())
                .collect();

            let (start_time, end_time) = match date_ranges.len() {
                2 => (Some(date_ranges[0].clone()), Some(date_ranges[1].clone())),
                1 => (Some(date_ranges[0].clone()), Some("present".to_string())),
                _ => (None, None),
            };

            educations.push(Education {
                organisation,
                organisation_profile,
                course_details: if course_details.is_empty() {
                    None
                } else {
                    Some(course_details)
                },
                description,
                start_time,
                end_time,
            });
        }
        educations
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
        let mut items = Vec::new();

        let summary_selector = Selector::parse("section.top-card-layout").unwrap();
        let summary_box = document.select(&summary_selector).next();

        let name_selector = Selector::parse(crate::selectors::PeopleSelectors::NAME).unwrap();
        let description_selector = Selector::parse(
            crate::selectors::PeopleSelectors::DESCRIPTION
        ).unwrap();
        let subline_item_selector = Selector::parse(
            crate::selectors::PeopleSelectors::SUBLINE_ITEM
        ).unwrap();
        let about_selector = Selector::parse(crate::selectors::PeopleSelectors::ABOUT).unwrap();

        let name = summary_box
            .and_then(|el| el.select(&name_selector).next())
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let description = summary_box
            .and_then(|el| el.select(&description_selector).next())
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let location = summary_box
            .and_then(|el| {
                el.select(&Selector::parse("div.top-card__subline-item").unwrap())
                    .next()
                    .or_else(|| el.select(&subline_item_selector).next())
            })
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|loc| !loc.contains("followers") && !loc.contains("connections"));

        let mut followers = None;
        let mut connections = None;

        if let Some(box_el) = summary_box {
            for span in box_el.select(&subline_item_selector) {
                let text = span.text().collect::<String>();
                if text.contains("followers") {
                    followers = Some(text.replace(" followers", "").trim().to_string());
                } else if text.contains("connections") {
                    connections = Some(text.replace(" connections", "").trim().to_string());
                }
            }
        }

        let about = document
            .select(&about_selector)
            .next()
            .map(|el| el.text().collect::<String>());

        let exp_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EXPERIENCE_ITEM
        ).unwrap();
        let exp_title_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EXPERIENCE_TITLE
        ).unwrap();
        let exp_location_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EXPERIENCE_LOCATION
        ).unwrap();
        let exp_desc_more_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EXPERIENCE_DESCRIPTION_MORE
        ).unwrap();
        let exp_desc_less_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EXPERIENCE_DESCRIPTION_LESS
        ).unwrap();
        let exp_date_time_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EXPERIENCE_DATE_TIME
        ).unwrap();
        let exp_duration_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EXPERIENCE_DURATION
        ).unwrap();

        let experience = self.parse_experience(
            &document,
            &exp_selector,
            &exp_title_selector,
            &exp_location_selector,
            &exp_desc_more_selector,
            &exp_desc_less_selector,
            &exp_date_time_selector,
            &exp_duration_selector
        );

        let edu_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EDUCATION_ITEM
        ).unwrap();
        let edu_org_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EDUCATION_ORGANIZATION
        ).unwrap();
        let edu_link_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EDUCATION_LINK
        ).unwrap();
        let edu_details_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EDUCATION_DETAILS
        ).unwrap();
        let edu_desc_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EDUCATION_DESCRIPTION
        ).unwrap();
        let edu_date_time_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EDUCATION_DATE_TIME
        ).unwrap();

        let education = self.parse_education(
            &document,
            &edu_selector,
            &edu_org_selector,
            &edu_link_selector,
            &edu_details_selector,
            &edu_desc_selector,
            &edu_date_time_selector
        );

        let person = PersonProfile {
            profile,
            url,
            name,
            description,
            location,
            followers,
            connections,
            about,
            experience,
            education,
        };

        items.push(person);
        Ok((items, vec![]))
    }
}
