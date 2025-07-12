// =====================================================================
//  TESTED: 06/2025
// =====================================================================

pub mod person {
    pub const NAME: &[&str] = &[
        "h1.top-card-layout__title",
        "h1.text-heading-xlarge",
        "h1.inline.t-24",
    ];
    pub const HEADLINE: &[&str] = &[".top-card-layout__headline", ".text-body-medium.break-words"];
    pub const LOCATION: &[&str] = &[
        "span.top-card__subline-item--bullet",
        ".pv-text-details__left-panel .t-black--light",
    ];
    pub const ABOUT: &[&str] = &[
        "section[id='about'] div.pv-shared-text",
        "#about ~ div .pv-shared-text",
    ];
    pub const EXPERIENCE_SECTION: &[&str] = &[
        "section[id='experience'] ul.pvs-list",
        "#experience + section ul.pvs-list",
    ];
    pub const EXPERIENCE_ITEMS: &[&str] = &[
        "li.pvs-list__paged-list-item",
        "li.artdeco-list__item",
    ];
    pub const EXPERIENCE_TITLES: &[&str] = &["li span.t-bold span"];
    pub const EXPERIENCE_COMPANIES: &[&str] = &["li span.t-normal span"];
    pub const EXPERIENCE_INFO: &[&str] = &["li span.t-black--light span"];
    pub const EXPERIENCE_COMPANY_LINKS: &[&str] = &["a[href*='/company/']"];
    pub const EDUCATION_SECTION: &[&str] = &["section[id='education'] ul.pvs-list"];
    pub const EDUCATION_ITEMS: &[&str] = &["li.pvs-list__paged-list-item"];
    pub const EDUCATION_SCHOOLS: &[&str] = &["li .t-bold span"];
    pub const EDUCATION_DEGREES: &[&str] = &["li .t-normal span"];
    pub const EDUCATION_DURATIONS: &[&str] = &["li .t-black--light span"];
    pub const EDUCATION_SCHOOL_LINKS: &[&str] = &["a[href*='/school/']"];

    pub const SEARCH_CARDS: &[&str] = &[
        ".reusable-search__result-container",
        ".search-results-container .entity-result",
    ];
    pub const SEARCH_TITLES: &[&str] = &["a.app-aware-link[href*='/in/']"];
    pub const SEARCH_HEADLINES: &[&str] = &[".entity-result__primary-subtitle"];
    pub const SEARCH_LOCATIONS: &[&str] = &[".entity-result__secondary-subtitle"];
    pub const OPEN_TO_WORK: &[&str] = &[".profile-photo-edit__preview", "img[alt*='Open to work']"];
}

pub mod company {
    pub const NAME: &[&str] = &["h1.org-top-card-summary__title", "h1.top-card-layout__title"];
    pub const ABOUT: &[&str] = &[
        "div.org-details__description",
        ".org-about-company-module__company-description",
    ];
    pub const WEBSITE: &[&str] = &[
        "a[data-control-name='page_details_module_website_external_link']",
        ".org-about-us__card-spacing a[href^='http']",
    ];
    pub const HEADQUARTERS: &[&str] = &[
        "dd[data-test-data-tracking-control-name='headquarters']",
        ".org-location-card__content",
    ];
    pub const FOUNDED: &[&str] = &[
        "dd[data-test-data-tracking-control-name='founded']",
        ".org-founded-card__content",
    ];
    pub const COMPANY_SIZE: &[&str] = &[
        ".org-page-details__employees-on-linkedin-count",
        ".org-about-company-module__company-size-definition-text",
    ];
    pub const INDUSTRY: &[&str] = &[
        "dd[data-test-data-tracking-control-name='industry']",
        ".org-top-card-summary__industry",
    ];
    pub const EMPLOYEES_SECTION: &[&str] = &[
        "section[data-test-id='employees-section']",
        ".org-people-bar-graph-module",
    ];
    pub const SPECIALTIES_ITEMS: &[&str] = &[".org-about-company-module__specialties-item"];
    pub const EMPLOYEE_CARDS: &[&str] = &[".org-people-profile-card", ".list-style-none li"];
    pub const EMPLOYEE_NAMES: &[&str] = &[".org-people-profile-card__profile-title", ".t-bold"];
    pub const EMPLOYEE_TITLES: &[&str] = &[
        ".org-people-profile-card__profile-info",
        ".t-black--light",
    ];
    pub const EMPLOYEE_LINKS: &[&str] = &["a[href*='/in/']"];
}

pub mod job {
    pub const TITLE: &[&str] = &[
        "h1.job-details-jobs-unified-top-card__job-title",
        "h1.jobs-unified-top-card__job-title",
    ];
    pub const COMPANY: &[&str] = &[
        ".job-details-jobs-unified-top-card__company-name",
        ".jobs-unified-top-card__company-name",
        ".top-card-layout__second-subline span",
    ];
    pub const LOCATION: &[&str] = &[
        ".job-details-jobs-unified-top-card__bullet",
        ".jobs-unified-top-card__bullet",
    ];
    pub const DESCRIPTION: &[&str] = &[
        "div.jobs-description__container",
        "div.jobs-box__html-content",
        "#job-details",
    ];
    pub const POSTED_DATE: &[&str] = &[
        ".job-details-jobs-unified-top-card__posted-date",
        ".jobs-unified-top-card__posted-date",
        ".posted-time-ago__text",
    ];
    pub const APPLICANT_COUNT: &[&str] = &[
        ".job-details-jobs-unified-top-card__applicant-count",
        ".jobs-unified-top-card__applicant-count",
        ".num-applicants__caption",
    ];
    pub const COMPANY_LINK: &[&str] = &[
        ".job-details-jobs-unified-top-card__company-name a",
        "a.topcard__org-name-link",
    ];
    pub const JOB_INSIGHTS: &[&str] = &[
        "ul.job-details-jobs-unified-top-card__job-insight-value-list li",
        "ul.jobs-unified-top-card__job-insight li",
    ];

    pub const SEARCH_CARDS: &[&str] = &[".job-search-card", ".jobs-search-results__list-item"];
    pub const SEARCH_TITLES: &[&str] = &[
        "h3 a.job-search-card__title",
        "a.app-aware-link[href*='/jobs/view']",
    ];
    pub const SEARCH_COMPANIES: &[&str] = &[
        ".job-search-card__subtitle",
        ".job-search-card__subtitle-link",
    ];
    pub const SEARCH_LOCATIONS: &[&str] = &[".job-search-card__location"];
    pub const SEARCH_POSTED_DATES: &[&str] = &[
        "time.job-search-card__listdate",
        "time.job-posted-date",
    ];
    pub const SEARCH_COMPANY_LINKS: &[&str] = &["a[href*='/company/']"];
}
