# LinkedIn Scraper

A fast and efficient LinkedIn scraper written in Rust with a comprehensive command-line interface.

## Features

- **Person Profiles**: Extract detailed information from LinkedIn profiles
- **People Search**: Search for people by name, job title, or criteria
- **Company Data**: Get company information and employee lists
- **Job Search**: Search and scrape job postings
- **CLI Tool**: Complete command-line interface
- **Cookie Authentication**: Secure authentication using LinkedIn session cookies
- **Multiple Formats**: JSON, pretty JSON, table, and summary outputs
- **Async/Await**: High-performance asynchronous requests

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
in_scraper = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::new("your-li-at-cookie-value")?;

    let person = client.scrape_person("https://www.linkedin.com/in/example").await?;
    println!("Name: {:?}", person.name);

    let people = client.search_people("Software Engineer", Some("Rome")).await?;
    println!("Found {} people", people.len());

    let company = client.scrape_company("https://www.linkedin.com/company/example").await?;
    println!("Company: {:?}", company.name);

    let jobs = client.search_jobs("Software Engineer", Some("Rome")).await?;
    println!("Found {} jobs", jobs.len());

    Ok(())
}
```

### Cookie Authentication

To use the LinkedIn scraper, you need to provide your LinkedIn session cookie (`li_at`):

1. **Get your li_at cookie**:

   - Log into LinkedIn in your browser
   - Open browser developer tools (F12)
   - Go to Application/Storage tab > Cookies > linkedin.com
   - Copy the value of the `li_at` cookie

2. **Use with environment variable**:

   ```bash
   LINKEDIN_LI_AT=your-li-at-cookie-value
   ```

3. **Use with command line**:
   ```bash
   in_scraper --li-at "your-li-at-cookie-value" <command>
   ```

## Command Line Interface

### Installation

Build the project:

```bash
cargo build --release
```

The binary is available at `./target/release/in_scraper`

### Authentication

You must provide your LinkedIn li_at cookie in one of two ways:

1. **Environment Variable** (Recommended):
   Create a `.env` file in your project root:

   ```bash
   LINKEDIN_LI_AT=your-li-at-cookie-value
   ```

2. **Command Line Argument**:
   ```bash
   in_scraper --li-at "your-li-at-cookie-value" <command>
   ```

### Global Options

Use these options with any command:

- `--format <format>`: Output format (json, pretty, summary, table)
- `--output <file>`: Save results to file
- `--verbose`: Show detailed progress

### Commands

#### Person Profile Scraping

```bash
# Basic usage
in_scraper person "https://www.linkedin.com/in/someone"

# Pretty output
in_scraper person "https://www.linkedin.com/in/someone" --format pretty

# Save to file
in_scraper person "https://www.linkedin.com/in/someone" --output person.json
```

#### People Search

```bash
# Basic search
in_scraper people "Software Engineer"

# With location
in_scraper people "Data Scientist" --location "Rosario, AR"

# Get detailed info for first 3 results
in_scraper people "Product Manager" --details 3

# Table format
in_scraper people "Machine Learning" --format table
```

#### Company Scraping

```bash
# Basic company info
in_scraper company "https://www.linkedin.com/company/example-company"

# Include employees
in_scraper company "https://www.linkedin.com/company/example-company" --employees

# Summary format
in_scraper company "https://www.linkedin.com/company/example-company" --format summary
```

#### Job Search

```bash
# Basic job search
in_scraper jobs "Software Engineer"

# With location
in_scraper jobs "Data Scientist" --location "Rosario, AR"

# Get detailed info for first 3 jobs
in_scraper jobs "Product Manager" --details 3

# Table format
in_scraper jobs "Machine Learning" --format table
```

#### Specific Job Scraping

```bash
# Scrape job details
in_scraper job "https://www.linkedin.com/jobs/view/1234567890"

# Summary format
in_scraper job "https://www.linkedin.com/jobs/view/1234567890" --format summary
```

### Output Formats

- **json**: Compact JSON for programmatic use
- **pretty**: Human-readable, indented JSON
- **summary**: Concise summary of key information
- **table**: Tabular format for multiple items

### Examples

#### Research Workflow

```bash
# Get detailed profile
in_scraper person "https://linkedin.com/in/john-doe" --format summary

# Find professionals
in_scraper people "Senior Developer" --location "Rome" --format table

# Company analysis
in_scraper company "https://linkedin.com/company/tech-startup" --employees --output company_data.json

# Job market research
in_scraper jobs "Senior Developer" --location "Rome" --details 5 --output opportunities.json
```

#### Automation

```bash
# Count total jobs found
in_scraper jobs "Python Developer" --format json | jq '. | length'

# Extract company names
in_scraper jobs "React Developer" --format json | jq '.[].company'

# Filter remote jobs
in_scraper jobs "Machine Learning" --format json | jq '.[] | select(.location | contains("Remote"))'
```

## Rate Limiting

LinkedIn has rate limiting. To avoid being blocked:

- Add delays between requests
- Use residential proxies for large-scale scraping
- Respect robots.txt
- Don't scrape more than necessary

## Security Notes

- Never commit credentials to version control
- Use environment variables for credential storage
- Be mindful of LinkedIn's Terms of Service
- Respect rate limits to avoid being blocked
- Use for legitimate research purposes only

## Legal Notice

This tool is for educational and research purposes. Always check LinkedIn's Terms of Service and respect rate limits. The authors are not responsible for any misuse of this software.

## License

MIT License - see LICENSE file for details.
