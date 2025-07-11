pub mod person {
    pub const NAME: &str = "h1.text-heading-xlarge";
    pub const HEADLINE: &str = ".text-body-medium.break-words";
    pub const LOCATION: &str = ".text-body-small.inline.t-black--light.break-words";
    pub const ABOUT: &str = "#about ~ .pv-shared-text-with-see-more .full-width";
    pub const EXPERIENCE_SECTION: &str = "#experience ~ .pvs-list__container";
    pub const EDUCATION_SECTION: &str = "#education ~ .pvs-list__container";
    pub const OPEN_TO_WORK: &str = ".pv-top-card-profile-picture img[title*='OPEN_TO_WORK']";
}

pub mod company {
    pub const NAME: &str = "h1.org-top-card-summary__title";
    pub const ABOUT: &str = ".org-about-company-module__company-description";
    pub const WEBSITE: &str = ".org-about-us__card-spacing a[href*='http']";
    pub const HEADQUARTERS: &str = ".org-location-card__content";
    pub const FOUNDED: &str = ".org-founded-card__content";
    pub const COMPANY_SIZE: &str = ".org-people-bar-graph-module__company-size";
    pub const INDUSTRY: &str = ".org-top-card-summary__industry";
    pub const EMPLOYEES_SECTION: &str = ".org-people-bar-graph-module";
}

pub mod job {
    pub const TITLE: &str = "h1.job-details-jobs-unified-top-card__job-title";
    pub const COMPANY: &str = ".job-details-jobs-unified-top-card__company-name";
    pub const LOCATION: &str = ".job-details-jobs-unified-top-card__bullet";
    pub const DESCRIPTION: &str = ".jobs-description";
    pub const POSTED_DATE: &str = ".jobs-unified-top-card__posted-date";
    pub const APPLICANT_COUNT: &str = ".jobs-unified-top-card__applicant-count";
}

pub mod auth {
    pub const EMAIL_INPUT: &str = "#username";
    pub const PASSWORD_INPUT: &str = "#password";
    pub const LOGIN_BUTTON: &str = ".btn__primary--large";
    pub const VERIFICATION_ELEMENT: &str = ".global-nav__primary-link";
} 