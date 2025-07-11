use thiserror::Error;

pub type Result<T> = std::result::Result<T, LinkedInError>;

#[derive(Error, Debug)]
pub enum LinkedInError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
    
    #[error("Element not found: {0}")]
    ElementNotFound(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Rate limited")]
    RateLimited,
    
    #[error("Unknown error: {0}")]
    Unknown(String),
} 