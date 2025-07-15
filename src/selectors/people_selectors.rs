pub struct PeopleSelectors;

impl PeopleSelectors {
    pub const TOP_CARD_LAYOUT: &'static str = "section.top-card-layout";
    pub const NAME: &'static str = "h1.top-card-layout__title";
    pub const DESCRIPTION: &'static str = "span.top-card-link__description";
    pub const LOCATION: &'static str = ".profile-info-subheader span:first-child";
    pub const FOLLOWERS: &'static str = ".not-first-middot span:first-child";
    pub const CONNECTIONS: &'static str = ".not-first-middot span:last-child";
    pub const SUBLINE_ITEM: &'static str = "span.top-card__subline-item";
    pub const EXPERIENCE_EDUCATION_COMPANY_LOGO: &'static str =
        "li.profile-section-card img.profile-section-card__image";
    pub const PROJECTS_ITEMS: &'static str =
        "section[data-section='projects'] ul > li.personal-project";
    pub const PROJECT_TITLE: &'static str = "h3";
    pub const PROJECT_DESCRIPTION: &'static str = "p.show-more-less-text__text--less";
    pub const PROJECT_LINK: &'static str = "h3 a";
    pub const LANGUAGES_ITEMS: &'static str =
        "section[data-section='languages'] ul > li.profile-section-card";
    pub const LANGUAGE_NAME: &'static str = "h3";
    pub const LANGUAGE_PROFICIENCY: &'static str = "h4";
    pub const ACTIVITIES_ITEMS: &'static str =
        "section[data-section='posts'] ul[data-test-id='activities__list'] > li";
    pub const ACTIVITY_TITLE: &'static str = "h3.base-main-card__title";
    pub const ACTIVITY_LINK: &'static str = "a.base-card__full-link";
    pub const EXPERIENCE_ITEM: &'static str =
        "section.experience-education li.profile-section-card";
    pub const EXPERIENCE_TITLE: &'static str = "h4 > p:first-child";
    pub const EXPERIENCE_LOCATION: &'static str = "div.text-color-text-low-emphasis";
    pub const EXPERIENCE_DESCRIPTION_MORE: &'static str = "p.show-more-less-text__text--more";
    pub const EXPERIENCE_DESCRIPTION_LESS: &'static str = "p.show-more-less-text__text--less";
    pub const EXPERIENCE_DATE_TIME: &'static str = "span.date-range time";
    pub const EXPERIENCE_DURATION: &'static str = "span.date-range__duration";
    pub const EDUCATION_ITEM: &'static str = "section.experience-education li.profile-section-card";
    pub const EDUCATION_ORGANIZATION: &'static str = "h3";
    pub const EDUCATION_LINK: &'static str = "a";
    pub const EDUCATION_DETAILS: &'static str = "h4 > p:first-child";
    pub const EDUCATION_DESCRIPTION: &'static str = "div.text-color-text-low-emphasis";
    pub const EDUCATION_DATE_TIME: &'static str = "span.date-range time";
}
