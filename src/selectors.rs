pub mod person {
    pub const NAME: &[&str] = &[
        "h1[class*='text-heading']",
        "h1[class*='pv-text-details__']",
        "h1.text-heading-xlarge",
        ".pv-top-card-profile-picture__name h1",
    ];
    pub const HEADLINE: &[&str] = &[
        "[class*='text-body-medium'][class*='break-words']",
        "[class*='pv-text-details__'][class*='headline']",
        ".text-body-medium.break-words",
        ".pv-top-card-profile-picture__headline",
    ];
    pub const LOCATION: &[&str] = &[
        "[class*='text-body-small'][class*='t-black--light']",
        "[class*='pv-text-details__'][class*='location']",
        ".text-body-small.inline.t-black--light.break-words",
        ".pv-top-card-profile-picture__location",
    ];
    pub const ABOUT: &[&str] = &[
        "#about ~ div [class*='pv-shared-text']",
        "[class*='pv-oc'] [class*='full-width']",
        "#about ~ .pv-shared-text-with-see-more .full-width",
        ".pv-about-section [class*='pv-shared-text']",
    ];
    pub const EXPERIENCE_SECTION: &[&str] = &[
        "#experience + * [class*='pvs-list']",
        "section[id*='experience'] [class*='pvs-list']",
        "#experience ~ .pvs-list__container",
        ".pv-profile-section[id*='experience'] [class*='pvs-list']",
    ];
    pub const EXPERIENCE_ITEMS: &[&str] = &[
        "[class*='pvs-list'][class*='item']",
        "[class*='paged-list-item']",
        ".pvs-list__paged-list-item",
        ".pv-profile-section__list-item",
    ];
    pub const EXPERIENCE_TITLES: &[&str] = &[
        "[class*='pvs-list'] .t-bold span",
        "[class*='experience'] .t-bold span",
        ".pv-entity__summary-info .t-bold span",
        ".pv-entity__summary-info-v2 .t-bold span",
    ];
    pub const EXPERIENCE_COMPANIES: &[&str] = &[
        "[class*='pvs-list'] .t-normal span",
        "[class*='experience'] .t-normal span",
        ".pv-entity__secondary-title span",
        ".pv-entity__company-summary-info span",
    ];
    pub const EXPERIENCE_INFO: &[&str] = &[".t-black--light span"];
    pub const EXPERIENCE_COMPANY_LINKS: &[&str] = &["a[href*='/company/']"];
    pub const EDUCATION_SECTION: &[&str] = &["#education ~ .pvs-list__container"];
    pub const EDUCATION_ITEMS: &[&str] = &[".pvs-list__paged-list-item"];
    pub const EDUCATION_SCHOOLS: &[&str] = &[".t-bold span"];
    pub const EDUCATION_DEGREES: &[&str] = &[".t-normal span"];
    pub const EDUCATION_DURATIONS: &[&str] = &[".t-black--light span"];
    pub const EDUCATION_SCHOOL_LINKS: &[&str] = &["a[href*='/school/']"];
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
    pub const OPEN_TO_WORK: &[&str] = &[
        ".pv-top-card-profile-picture__ring",
        "img[alt*='Open to work']",
        "[class*='open-to-work']",
    ];
}

pub mod company {
    pub const NAME: &[&str] = &["h1.org-top-card-summary__title"];
    pub const ABOUT: &[&str] = &[".org-about-company-module__company-description"];
    pub const WEBSITE: &[&str] = &[".org-about-us__card-spacing a[href*='http']"];
    pub const HEADQUARTERS: &[&str] = &[".org-location-card__content"];
    pub const FOUNDED: &[&str] = &[".org-founded-card__content"];
    pub const COMPANY_SIZE: &[&str] = &[".org-people-bar-graph-module__company-size"];
    pub const INDUSTRY: &[&str] = &[".org-top-card-summary__industry"];
    pub const EMPLOYEES_SECTION: &[&str] = &[".org-people-bar-graph-module"];
    pub const SPECIALTIES_ITEMS: &[&str] = &[
        ".org-about-company-module__specialties .org-about-company-module__specialties-item",
    ];
    pub const EMPLOYEE_CARDS: &[&str] = &[".org-people-profile-card", ".list-style-none li"];
    pub const EMPLOYEE_NAMES: &[&str] = &[
        ".org-people-profile-card__profile-title",
        ".t-16 .t-black .t-bold",
    ];
    pub const EMPLOYEE_TITLES: &[&str] = &[
        ".org-people-profile-card__profile-info",
        ".t-14 .t-black--light",
    ];
    pub const EMPLOYEE_LINKS: &[&str] = &["a[href*='/in/']"];
}

pub mod job {
    pub const TITLE: &[&str] = &["h1.job-details-jobs-unified-top-card__job-title"];
    pub const COMPANY: &[&str] = &[".job-details-jobs-unified-top-card__company-name"];
    pub const LOCATION: &[&str] = &[".job-details-jobs-unified-top-card__bullet"];
    pub const DESCRIPTION: &[&str] = &[".jobs-description"];
    pub const POSTED_DATE: &[&str] = &[".jobs-unified-top-card__posted-date"];
    pub const APPLICANT_COUNT: &[&str] = &[".jobs-unified-top-card__applicant-count"];
    pub const COMPANY_LINK: &[&str] = &[".job-details-jobs-unified-top-card__company-name a"];
    pub const JOB_INSIGHTS: &[&str] = &[
        ".job-details-jobs-unified-top-card__job-insight .job-details-jobs-unified-top-card__job-insight-value-list li",
    ];
    pub const SEARCH_CARDS: &[&str] = &[".job-search-card", ".jobs-search-results__list-item"];
    pub const SEARCH_TITLES: &[&str] = &[".job-search-card__title a", "h3 a"];
    pub const SEARCH_COMPANIES: &[&str] = &[
        ".job-search-card__subtitle",
        ".job-search-card__subtitle-link",
    ];
    pub const SEARCH_LOCATIONS: &[&str] = &[".job-search-card__location"];
    pub const SEARCH_POSTED_DATES: &[&str] = &[".job-search-card__listdate", ".job-posted-date"];
    pub const SEARCH_COMPANY_LINKS: &[&str] = &["a[href*='/company/']"];
}

pub mod auth {
    pub const EMAIL_INPUT: &[&str] = &["#username"];
    pub const PASSWORD_INPUT: &[&str] = &["#password"];
    pub const LOGIN_BUTTON: &[&str] = &[".btn__primary--large"];
    pub const CSRF_TOKEN: &[&str] = &["input[name='loginCsrfParam']"];
    pub const VERIFICATION_ELEMENT: &[&str] = &[".global-nav__primary-link"];
}
