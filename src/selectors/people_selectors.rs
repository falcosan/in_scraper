pub struct PeopleSelectors;

impl PeopleSelectors {
    pub const NAME: &'static str = "h1";
    pub const DESCRIPTION: &'static str = "h2";
    pub const SUBLINE_ITEM: &'static str = "span.top-card__subline-item";
    pub const ABOUT: &'static str = "section.summary div.core-section-container__content p";
    pub const EXPERIENCE_ITEM: &'static str = "li.experience-item";
    pub const EXPERIENCE_TITLE: &'static str = "h4 a";
    pub const EXPERIENCE_LOCATION: &'static str = "p.experience-item__location";
    pub const EXPERIENCE_DESCRIPTION_MORE: &'static str = "p.show-more-less-text__text--more";
    pub const EXPERIENCE_DESCRIPTION_LESS: &'static str = "p.show-more-less-text__text--less";
    pub const EXPERIENCE_DATE_TIME: &'static str = "span.date-range time";
    pub const EXPERIENCE_DURATION: &'static str = "span.date-range__duration";
    pub const EDUCATION_ITEM: &'static str = "li.education__list-item";
    pub const EDUCATION_ORGANIZATION: &'static str = "h3";
    pub const EDUCATION_LINK: &'static str = "a";
    pub const EDUCATION_DETAILS: &'static str = "h4 span";
    pub const EDUCATION_DESCRIPTION: &'static str = "div.education__item--details p";
    pub const EDUCATION_DATE_TIME: &'static str = "span.date-range time";
}
