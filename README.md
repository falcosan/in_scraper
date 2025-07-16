# LinkedIn Scraper - Rust Implementation

A Rust implementation of a LinkedIn scraper that can extract company profiles, job listings, and people profiles.

## Features

- **Company Profile Spider**: Scrapes company information including name, summary, industry, size, and founding date
- **Jobs Spider**: Scrapes job listings with pagination support
- **People Profile Spider**: Scrapes people profiles including experience and education
- **Concurrent Processing**: Configurable concurrent request handling
- **HTTP Client**: Built-in retry mechanisms and rate limiting handling
- **JSON Output**: Saves data in JSON format with timestamps
- **Configurable Timeouts**: Customizable request timeouts and retry settings

## Installation

1. Make sure you have Rust installed (https://rustup.rs/)
2. Clone this repository
3. Build the project:

```bash
cargo build --release
```

## Usage

The scraper provides three main commands:

### Company Profile Scraper

```bash
# Scrape specific company profiles
cargo run -- company-profile --urls "https://www.linkedin.com/company/microsoft" --urls "https://www.linkedin.com/company/google"

# With custom settings
cargo run -- company-profile --urls "https://www.linkedin.com/company/microsoft" --concurrent 3 --timeout 60 --retries 5
```

### Jobs Scraper

```bash
# Scrape job listings
cargo run -- jobs --keywords "rust developer" --location "San Francisco"

# With concurrency and custom settings
cargo run -- jobs --keywords "data scientist" --concurrent 5 --timeout 45 --retries 3
```

### People Profile Scraper

```bash
# Scrape people profiles
cargo run -- people-profile --profiles "danielefalchetti"

# With custom output directory and settings
cargo run -- people-profile --profiles "danielefalchetti" --output custom_data --timeout 30
```

## Command Line Options

### Global Options

- `-c, --concurrent <N>`: Number of concurrent requests (default: 1)
- `-o, --output <PATH>`: Output directory for JSON files (default: "data")
- `--timeout <SECONDS>`: Request timeout in seconds (default: 30)
- `--retries <N>`: Maximum number of retries for failed requests (default: 3)

### Jobs Command Options

- `--keywords <KEYWORDS>`: Search keywords
- `--location <LOCATION>`: Job location

### Company Profile Command Options

- `--urls <URL>`: Company profile URLs (can be specified multiple times)

### People Profile Command Options

- `--profiles <PROFILE>`: LinkedIn profile usernames (can be specified multiple times)

## Environment Variables

You can set configuration via environment variables:

- `CONCURRENT_REQUESTS`: Number of concurrent requests
- `REQUEST_TIMEOUT`: Request timeout in seconds
- `MAX_RETRIES`: Maximum number of retries for failed requests

## Architecture

The scraper follows a modular architecture:

- **Spiders**: Define scraping logic for each data type
- **HTTP Client**: Handles requests with retry mechanisms and rate limiting
- **Pipeline**: Processes and saves scraped items
- **Middleware**: Extensible request/response processing

## HTTP Client Features

The built-in HTTP client includes:

- **Automatic retries** with exponential backoff
- **Rate limiting detection** and handling
- **Configurable timeouts**
- **Connection pooling** for better performance
- **User-agent rotation** support
- **Comprehensive error handling**

## Development

To run in development mode with debug logging:

```bash
RUST_LOG=debug cargo run -- jobs
```

## Performance Considerations

- The scraper respects rate limits by default (1 concurrent request)
- Increase concurrency carefully to avoid being blocked
- Use appropriate timeout and retry settings for your use case
- Consider implementing delays between requests for production use

## Rate Limiting and Best Practices

The scraper includes several mechanisms to handle rate limiting:

1. **Exponential backoff** on retries
2. **429 status code detection** with automatic retry
3. **Configurable delays** between requests
4. **Connection pooling** to reduce overhead

## License

This project is for educational purposes only. Please respect LinkedIn's Terms of Service and robots.txt when using this scraper.
