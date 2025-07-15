pub struct PeopleSelectors;

impl PeopleSelectors {
    pub const TOP_CARD_LAYOUT: &'static str = "section.top-card-layout";
    pub const NAME: &'static str = "h1.top-card-layout__title";
    pub const DESCRIPTION: &'static str = "span.top-card-link__description";
    pub const LOCATION: &'static str =
        "h3.top-card-layout__first-subline .profile-info-subheader span:first-child";
    pub const FOLLOWERS: &'static str =
        "h3.top-card-layout__first-subline .not-first-middot span:first-child";
    pub const CONNECTIONS: &'static str =
        "h3.top-card-layout__first-subline .not-first-middot span:last-child";
    pub const EXPERIENCE_EDUCATION_SECTION: &'static str = "section#experience-education";
    pub const EXPERIENCE_EDUCATION_ITEMS: &'static str =
        "section#experience-education ul.visible-list > li.profile-section-card";
    pub const EXPERIENCE_EDUCATION_COMPANY_LOGO: &'static str =
        "li.profile-section-card img.profile-section-card__image";
    pub const EXPERIENCE_EDUCATION_COMPANY_NAME: &'static str = "li.profile-section-card h3";
    pub const EXPERIENCE_EDUCATION_TITLE: &'static str =
        "li.profile-section-card h4 > p:first-child";
    pub const EXPERIENCE_EDUCATION_DETAILS: &'static str =
        "li.profile-section-card div.text-color-text-low-emphasis";
    pub const CORE_SECTION_CONTAINER: &'static str = "section.core-section-container";
    pub const CORE_SECTION_TITLE: &'static str = "h2.core-section-container__title";
    pub const CORE_SECTION_CONTENT: &'static str = "div.core-section-container__content";
    pub const PROJECTS_SECTION: &'static str = "section#projects";
    pub const PROJECTS_ITEMS: &'static str = "section#projects li";
    pub const LANGUAGES_SECTION: &'static str = "section#languages";
    pub const LANGUAGES_ITEMS: &'static str = "section#languages li";
    pub const ACTIVITIES_SECTION: &'static str = "section#activity";
    pub const ACTIVITIES_ITEMS: &'static str = "section#activity li";
    pub const SUBLINE_ITEM: &'static str = "span.top-card__subline-item";
    pub const EXPERIENCE_ITEM: &'static str = "li.profile-section-card";
    pub const EXPERIENCE_TITLE: &'static str = "li.profile-section-card h4 > p:first-child";
    pub const EXPERIENCE_LOCATION: &'static str =
        "li.profile-section-card div.text-color-text-low-emphasis";
    pub const EXPERIENCE_DESCRIPTION_MORE: &'static str = "p.show-more-less-text__text--more";
    pub const EXPERIENCE_DESCRIPTION_LESS: &'static str = "p.show-more-less-text__text--less";
    pub const EXPERIENCE_DATE_TIME: &'static str = "span.date-range time";
    pub const EXPERIENCE_DURATION: &'static str = "span.date-range__duration";
    pub const EDUCATION_ITEM: &'static str = "li.profile-section-card";
    pub const EDUCATION_ORGANIZATION: &'static str = "li.profile-section-card h3";
    pub const EDUCATION_LINK: &'static str = "a";
    pub const EDUCATION_DETAILS: &'static str = "li.profile-section-card h4 > p:first-child";
    pub const EDUCATION_DESCRIPTION: &'static str =
        "li.profile-section-card div.text-color-text-low-emphasis";
    pub const EDUCATION_DATE_TIME: &'static str = "span.date-range time";
}
