pub mod person {
    pub const NAME: &str = "h1.text-heading-xlarge";
    pub const HEADLINE: &str = ".text-body-medium.break-words";
    pub const LOCATION: &str = ".text-body-small.inline.t-black--light.break-words";
    pub const ABOUT: &str = "#about ~ .pv-shared-text-with-see-more .full-width";
    pub const OPEN_TO_WORK: &str = ".pv-top-card-profile-picture img[title*='OPEN_TO_WORK']";
    pub const EXPERIENCE_SECTION: &str = "#experience ~ .pvs-list__container";
    pub const EXPERIENCE_ITEMS: &str = ".pvs-list__paged-list-item";
    pub const EXPERIENCE_TITLES: &str = ".t-bold span";
    pub const EXPERIENCE_COMPANIES: &str = ".t-normal span";
    pub const EXPERIENCE_INFO: &str = ".t-black--light span";
    pub const EXPERIENCE_COMPANY_LINKS: &str = "a[href*='/company/']";
    pub const EDUCATION_SECTION: &str = "#education ~ .pvs-list__container";
    pub const EDUCATION_ITEMS: &str = ".pvs-list__paged-list-item";
    pub const EDUCATION_SCHOOLS: &str = ".t-bold span";
    pub const EDUCATION_DEGREES: &str = ".t-normal span";
    pub const EDUCATION_DURATIONS: &str = ".t-black--light span";
    pub const EDUCATION_SCHOOL_LINKS: &str = "a[href*='/school/']";
    pub const SEARCH_CARDS: &[&str] = &[
        ".search-result__wrapper",
        ".search-result",
        ".reusable-search__result-container",
    ];
    pub const SEARCH_TITLES: &[&str] = &[
        ".search-result__result-link",
        ".app-aware-link",
        "a[href*='/in/']",
    ];
    pub const SEARCH_HEADLINES: &[&str] = &[".subline-level-1", ".entity-result__primary-subtitle"];
    pub const SEARCH_LOCATIONS: &[&str] = &[
        ".subline-level-2",
        ".entity-result__secondary-subtitle",
    ];
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
    pub const SPECIALTIES_ITEMS: &str =
        ".org-about-company-module__specialties .org-about-company-module__specialties-item";
    pub const EMPLOYEE_CARDS: &[&str] = &[".org-people-profile-card", ".list-style-none li"];
    pub const EMPLOYEE_NAMES: &[&str] = &[
        ".org-people-profile-card__profile-title",
        ".t-16 .t-black .t-bold",
    ];
    pub const EMPLOYEE_TITLES: &[&str] = &[
        ".org-people-profile-card__profile-info",
        ".t-14 .t-black--light",
    ];
    pub const EMPLOYEE_LINKS: &str = "a[href*='/in/']";
}

pub mod job {
    pub const TITLE: &str = "h1.job-details-jobs-unified-top-card__job-title";
    pub const COMPANY: &str = ".job-details-jobs-unified-top-card__company-name";
    pub const LOCATION: &str = ".job-details-jobs-unified-top-card__bullet";
    pub const DESCRIPTION: &str = ".jobs-description";
    pub const POSTED_DATE: &str = ".jobs-unified-top-card__posted-date";
    pub const APPLICANT_COUNT: &str = ".jobs-unified-top-card__applicant-count";
    pub const COMPANY_LINK: &str = ".job-details-jobs-unified-top-card__company-name a";
    pub const JOB_INSIGHTS: &str =
        ".job-details-jobs-unified-top-card__job-insight .job-details-jobs-unified-top-card__job-insight-value-list li";
    pub const SEARCH_CARDS: &[&str] = &[".job-search-card", ".jobs-search-results__list-item"];
    pub const SEARCH_TITLES: &[&str] = &[".job-search-card__title a", "h3 a"];
    pub const SEARCH_COMPANIES: &[&str] = &[
        ".job-search-card__subtitle",
        ".job-search-card__subtitle-link",
    ];
    pub const SEARCH_LOCATIONS: &str = ".job-search-card__location";
    pub const SEARCH_POSTED_DATES: &[&str] = &[".job-search-card__listdate", ".job-posted-date"];
    pub const SEARCH_COMPANY_LINKS: &str = "a[href*='/company/']";
}

pub mod auth {
    pub const EMAIL_INPUT: &str = "#username";
    pub const PASSWORD_INPUT: &str = "#password";
    pub const LOGIN_BUTTON: &str = ".btn__primary--large";
    pub const CSRF_TOKEN: &str = "input[name='loginCsrfParam']";
    pub const VERIFICATION_ELEMENT: &str = ".global-nav__primary-link";
}
