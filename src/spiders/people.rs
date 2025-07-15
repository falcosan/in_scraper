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
                let url = format!("https://www.linkedin.com/in/{profile}/");
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

        let name = summary_box
            .and_then(|el|
                el.select(&Selector::parse(crate::selectors::PeopleSelectors::NAME).unwrap()).next()
            )
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let description = summary_box
            .and_then(|el|
                el
                    .select(
                        &Selector::parse(crate::selectors::PeopleSelectors::DESCRIPTION).unwrap()
                    )
                    .next()
            )
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let location = summary_box
            .and_then(|el| {
                el.select(&Selector::parse("div.top-card__subline-item").unwrap())
                    .next()
                    .or_else(||
                        el
                            .select(
                                &Selector::parse(
                                    crate::selectors::PeopleSelectors::SUBLINE_ITEM
                                ).unwrap()
                            )
                            .next()
                    )
            })
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|loc| !loc.contains("followers") && !loc.contains("connections"));

        let mut followers = None;
        let mut connections = None;

        if let Some(box_el) = summary_box {
            for span in box_el.select(
                &Selector::parse(crate::selectors::PeopleSelectors::SUBLINE_ITEM).unwrap()
            ) {
                let text = span.text().collect::<String>();
                if text.contains("followers") {
                    followers = Some(text.replace(" followers", "").trim().to_string());
                } else if text.contains("connections") {
                    connections = Some(text.replace(" connections", "").trim().to_string());
                }
            }
        }

        let about = document
            .select(&Selector::parse(crate::selectors::PeopleSelectors::ABOUT).unwrap())
            .next()
            .map(|el| el.text().collect::<String>());

        let experience = self.parse_experience(&document);
        let education = self.parse_education(&document);

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

impl PeopleProfileSpider {
    fn parse_experience(&self, document: &Html) -> Vec<Experience> {
        let mut experiences = Vec::new();
        let exp_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EXPERIENCE_ITEM
        ).unwrap();

        for block in document.select(&exp_selector) {
            let organisation_profile = block
                .select(
                    &Selector::parse(crate::selectors::PeopleSelectors::EXPERIENCE_TITLE).unwrap()
                )
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| href.split('?').next().unwrap_or(href).to_string());

            let location = block
                .select(
                    &Selector::parse(
                        crate::selectors::PeopleSelectors::EXPERIENCE_LOCATION
                    ).unwrap()
                )
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string());

            let description = block
                .select(
                    &Selector::parse(
                        crate::selectors::PeopleSelectors::EXPERIENCE_DESCRIPTION_MORE
                    ).unwrap()
                )
                .next()
                .or_else(||
                    block
                        .select(
                            &Selector::parse(
                                crate::selectors::PeopleSelectors::EXPERIENCE_DESCRIPTION_LESS
                            ).unwrap()
                        )
                        .next()
                )
                .map(|el| el.text().collect::<String>().trim().to_string());

            let date_ranges: Vec<_> = block
                .select(
                    &Selector::parse(
                        crate::selectors::PeopleSelectors::EXPERIENCE_DATE_TIME
                    ).unwrap()
                )
                .map(|el| el.text().collect::<String>())
                .collect();

            let (start_time, end_time, duration) = match date_ranges.len() {
                2 =>
                    (
                        Some(date_ranges[0].clone()),
                        Some(date_ranges[1].clone()),
                        block
                            .select(
                                &Selector::parse(
                                    crate::selectors::PeopleSelectors::EXPERIENCE_DURATION
                                ).unwrap()
                            )
                            .next()
                            .map(|el| el.text().collect::<String>()),
                    ),
                1 =>
                    (
                        Some(date_ranges[0].clone()),
                        Some("present".to_string()),
                        block
                            .select(
                                &Selector::parse(
                                    crate::selectors::PeopleSelectors::EXPERIENCE_DURATION
                                ).unwrap()
                            )
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

    fn parse_education(&self, document: &Html) -> Vec<Education> {
        let mut educations = Vec::new();
        let edu_selector = Selector::parse(
            crate::selectors::PeopleSelectors::EDUCATION_ITEM
        ).unwrap();

        for block in document.select(&edu_selector) {
            let organisation = block
                .select(
                    &Selector::parse(
                        crate::selectors::PeopleSelectors::EDUCATION_ORGANIZATION
                    ).unwrap()
                )
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let organisation_profile = block
                .select(
                    &Selector::parse(crate::selectors::PeopleSelectors::EDUCATION_LINK).unwrap()
                )
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| href.split('?').next().unwrap_or(href).to_string());

            let course_details = block
                .select(
                    &Selector::parse(crate::selectors::PeopleSelectors::EDUCATION_DETAILS).unwrap()
                )
                .map(|el| el.text().collect::<String>().trim().to_string())
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            let description = block
                .select(
                    &Selector::parse(
                        crate::selectors::PeopleSelectors::EDUCATION_DESCRIPTION
                    ).unwrap()
                )
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string());

            let date_ranges: Vec<_> = block
                .select(
                    &Selector::parse(
                        crate::selectors::PeopleSelectors::EDUCATION_DATE_TIME
                    ).unwrap()
                )
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
