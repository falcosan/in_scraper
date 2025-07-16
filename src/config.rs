use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bot_name: String,
    pub concurrent_requests: usize,
    pub robotstxt_obey: bool,
    pub output_dir: String,
    pub user_agent: String,
    pub request_timeout: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_retries: 3,
            request_timeout: 30,
            retry_delay_ms: 1000,
            robotstxt_obey: false,
            concurrent_requests: 1,
            output_dir: "data".to_string(),
            bot_name: "linkedin".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(concurrent) = std::env::var("CONCURRENT_REQUESTS") {
            if let Ok(num) = concurrent.parse() {
                config.concurrent_requests = num;
            }
        }

        if let Ok(timeout) = std::env::var("REQUEST_TIMEOUT") {
            if let Ok(num) = timeout.parse() {
                config.request_timeout = num;
            }
        }

        if let Ok(retries) = std::env::var("MAX_RETRIES") {
            if let Ok(num) = retries.parse() {
                config.max_retries = num;
            }
        }

        if let Ok(retry_delay) = std::env::var("RETRY_DELAY_MS") {
            if let Ok(num) = retry_delay.parse() {
                config.retry_delay_ms = num;
            }
        }

        if let Ok(user_agent) = std::env::var("USER_AGENT") {
            config.user_agent = user_agent;
        }

        config
    }
}
