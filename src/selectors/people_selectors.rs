pub struct PeopleSelectors;

impl PeopleSelectors {
    pub const TOP_CARD_LAYOUT: &str = "section.top-card-layout";
    pub const NAME: &str = "h1.top-card-layout__title";
    pub const DESCRIPTION: &str = "span.top-card-link__description";
    pub const LOCATION: &str = ".profile-info-subheader span:first-child";
    pub const FOLLOWERS: &str = ".not-first-middot span:first-child";
    pub const CONNECTIONS: &str = ".not-first-middot span:last-child";
    pub const SUBLINE_ITEM: &str = "span.top-card__subline-item";
    pub const PROJECTS_ITEMS: &str = "section[data-section='projects'] ul > li.personal-project";
    pub const PROJECT_TITLE: &str = "h3";
    pub const PROJECT_DESCRIPTION: &str = "p.show-more-less-text__text--less";
    pub const PROJECT_LINK: &str = "h3 a";
    pub const LANGUAGES_ITEMS: &str =
        "section[data-section='languages'] ul > li.profile-section-card";
    pub const LANGUAGE_NAME: &str = "h3";
    pub const LANGUAGE_PROFICIENCY: &str = "h4";
    pub const ACTIVITIES_ITEMS: &str =
        "section[data-section='posts'] ul[data-test-id='activities__list'] > li";
    pub const ACTIVITY_TITLE: &str = "h3.base-main-card__title";
    pub const ACTIVITY_LINK: &str = "a.base-card__full-link";
    pub const EXPERIENCE_ITEM: &str = "li.profile-section-card";
    pub const EXPERIENCE_EDUCATION_COMPANY_LOGO: &str =
        "li.profile-section-card img.profile-section-card__image";
    pub const EXPERIENCE_TITLE: &str = "h4 > p:first-child";
    pub const EXPERIENCE_LOCATION: &str = "div.text-color-text-low-emphasis";
    pub const EXPERIENCE_DESCRIPTION_MORE: &str = "p.show-more-less-text__text--more";
    pub const EXPERIENCE_DESCRIPTION_LESS: &str = "p.show-more-less-text__text--less";
    pub const EXPERIENCE_DATE_TIME: &str = "span.date-range time";
    pub const EXPERIENCE_DURATION: &str = "span.date-range__duration";
    pub const EDUCATION_ITEM: &str = "li.profile-section-card";
    pub const EDUCATION_ORGANIZATION: &str = "h3";
    pub const EDUCATION_LINK: &str = "a";
    pub const EDUCATION_DETAILS: &str = "h4 > p:first-child";
    pub const EDUCATION_DESCRIPTION: &str = "div.text-color-text-low-emphasis";
    pub const EDUCATION_DATE_TIME: &str = "span.date-range time";
}
