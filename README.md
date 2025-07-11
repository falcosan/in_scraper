# LinkedIn Scraper (Rust)

A fast and efficient LinkedIn scraper written in Rust.

## Features

- **Person Profile Scraping**: Extract detailed information from LinkedIn profiles including experience, education, and contact details
- **People Search**: Search for people on LinkedIn by name, job title, or other criteria
- **Company Scraping**: Get company information, employee lists, and company details  
- **Job Search**: Search for jobs and scrape individual job postings
- **Command Line Interface**: Complete CLI tool for terminal usage
- **Authentication**: Secure login with cookie persistence
- **Async/Await**: High-performance asynchronous HTTP requests
- **Error Handling**: Robust error handling with detailed error types
- **Multiple Output Formats**: JSON, pretty JSON, table, and summary formats
- **JSON Export**: Serialize all data to JSON format

## Installation

Add this to your `Cargo.toml`:

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
    // Login to LinkedIn
    let client = LinkedInClient::login("email@example.com", "password").await?;
    
    // Scrape a person's profile
    let person = client.scrape_person("https://www.linkedin.com/in/example").await?;
    println!("Name: {:?}", person.name);
    
    // Search for people
    let people = client.search_people("Software Engineer", Some("San Francisco")).await?;
    println!("Found {} people", people.len());
    
    // Scrape a company
    let company = client.scrape_company("https://www.linkedin.com/company/example").await?;
    println!("Company: {:?}", company.name);
    
    // Search for jobs
    let jobs = client.search_jobs("Software Engineer", Some("San Francisco")).await?;
    println!("Found {} jobs", jobs.len());
    
    Ok(())
}
```

### Environment Variables

For security, use environment variables for credentials:

```bash
export LINKEDIN_EMAIL="your-email@example.com"
export LINKEDIN_PASSWORD="your-password"
```

### Person Scraping

```rust
use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::login(&email, &password).await?;
    let person = client.scrape_person("https://www.linkedin.com/in/example").await?;
    
    // Access person data
    println!("Name: {:?}", person.name);
    println!("Headline: {:?}", person.headline);
    println!("Location: {:?}", person.location);
    println!("Open to work: {}", person.open_to_work);
    
    // Experiences
    for exp in &person.experiences {
        println!("Role: {} at {}", 
            exp.title.as_deref().unwrap_or("Unknown"),
            exp.company.as_deref().unwrap_or("Unknown")
        );
    }
    
    // Export to JSON
    let json = serde_json::to_string_pretty(&person)?;
    std::fs::write("person.json", json)?;
    
    Ok(())
}
```

### People Search

```rust
use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::login(&email, &password).await?;
    
    // Search for people by job title
    let people = client.search_people("Data Scientist", Some("New York")).await?;
    
    for person in &people {
        println!("{}: {}", 
            person.name.as_deref().unwrap_or("Unknown"),
            person.headline.as_deref().unwrap_or("No headline")
        );
    }
    
    // Get detailed information for specific people
    if !people.is_empty() {
        let detailed_person = client.scrape_person(&people[0].linkedin_url).await?;
        println!("Detailed info: {:?}", detailed_person);
    }
    
    Ok(())
}
```

### Company Scraping

```rust
use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::login(&email, &password).await?;
    let company = client.scrape_company("https://www.linkedin.com/company/example").await?;
    
    println!("Company: {:?}", company.name);
    println!("Industry: {:?}", company.industry);
    println!("Size: {:?}", company.company_size);
    println!("Founded: {:?}", company.founded);
    
    // Get employees separately for better performance
    let employees = client.scrape_company_employees(&company.linkedin_url).await?;
    println!("Found {} employees", employees.len());
    
    Ok(())
}
```

### Job Search

```rust
use in_scraper::{LinkedInClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::login(&email, &password).await?;
    
    // Search jobs
    let jobs = client.search_jobs("Machine Learning Engineer", Some("Remote")).await?;
    
    for job in &jobs {
        println!("{} at {}", 
            job.title.as_deref().unwrap_or("Unknown"),
            job.company.as_deref().unwrap_or("Unknown")
        );
    }
    
    // Get detailed job information
    if !jobs.is_empty() {
        let detailed = client.scrape_job(&jobs[0].linkedin_url).await?;
        println!("Description: {:?}", detailed.description);
    }
    
    Ok(())
}
```

## Command Line Interface

The LinkedIn Scraper provides a powerful command-line interface for scraping LinkedIn profiles, companies, and jobs directly from your terminal.

### CLI Installation

After building the project, the CLI binary is available at:
```bash
./target/release/in_scraper
```

You can also install it globally:
```bash
cargo install --path .
```

### Authentication

The CLI requires LinkedIn credentials. You can provide them in three ways:

#### 1. Environment Variables (Recommended)
```bash
export LINKEDIN_EMAIL="your-email@example.com"
export LINKEDIN_PASSWORD="your-password"
```

#### 2. Command Line Arguments
```bash
in_scraper --email your-email@example.com --password your-password <command>
```

#### 3. Interactive Prompt
If no credentials are provided, you'll be prompted to enter them:
```bash
in_scraper <command>
# LinkedIn Email: your-email@example.com
# LinkedIn Password: ********
```

### Global Options

The CLI supports global options that can be used with any command:

```bash
# Global format option
in_scraper --format pretty person "https://linkedin.com/in/someone"

# Global output and verbose options
in_scraper --output results.json --verbose people "Software Engineer"

# Combine global and command-specific options
in_scraper --format json person "https://linkedin.com/in/someone" --output person.json
```

**Note**: You can also use these options directly with subcommands (as shown in the examples below), which is often more convenient.

### CLI Commands

#### 1. Person Profile Scraping

Scrape a LinkedIn person's profile:

```bash
# Basic usage
in_scraper person "https://www.linkedin.com/in/someone"

# Pretty JSON output
in_scraper person "https://www.linkedin.com/in/someone" --format pretty

# Summary format
in_scraper person "https://www.linkedin.com/in/someone" --format summary

# Save to file
in_scraper person "https://www.linkedin.com/in/someone" --output person.json

# Verbose output
in_scraper person "https://www.linkedin.com/in/someone" --verbose
```

#### 2. People Search

Search for people on LinkedIn:

```bash
# Basic people search
in_scraper people "Software Engineer"

# Search with location
in_scraper people "Data Scientist" --location "San Francisco, CA"

# Get detailed info for first 3 people
in_scraper people "Product Manager" --details 3

# Table format for people listings
in_scraper people "Machine Learning" --format table

# Save people search results
in_scraper people "DevOps Engineer" --location "Remote" --output people.json
```

#### 3. Company Scraping

Scrape a LinkedIn company page:

```bash
# Basic company info
in_scraper company "https://www.linkedin.com/company/example-company"

# Include employee list
in_scraper company "https://www.linkedin.com/company/example-company" --employees

# Table format
in_scraper company "https://www.linkedin.com/company/example-company" --format table

# Summary with employees
in_scraper company "https://www.linkedin.com/company/example-company" --employees --format summary
```

#### 4. Job Search

Search for jobs on LinkedIn:

```bash
# Basic job search
in_scraper jobs "Software Engineer"

# Search with location
in_scraper jobs "Data Scientist" --location "San Francisco, CA"

# Get detailed info for first 3 jobs
in_scraper jobs "Product Manager" --details 3

# Table format for job listings
in_scraper jobs "Machine Learning" --format table

# Save job search results
in_scraper jobs "DevOps Engineer" --location "Remote" --output jobs.json
```

#### 5. Specific Job Scraping

Scrape a specific job posting:

```bash
# Scrape job details
in_scraper job "https://www.linkedin.com/jobs/view/1234567890"

# Summary format
in_scraper job "https://www.linkedin.com/jobs/view/1234567890" --format summary
```

### Output Formats

The CLI supports multiple output formats:

#### JSON (Default)
```bash
in_scraper person "https://linkedin.com/in/someone" --format json
```
Compact JSON output suitable for programmatic use.

#### Pretty JSON
```bash
in_scraper person "https://linkedin.com/in/someone" --format pretty
```
Human-readable, indented JSON.

#### Summary
```bash
in_scraper person "https://linkedin.com/in/someone" --format summary
```
Concise, human-readable summary of key information.

#### Table
```bash
in_scraper people "Engineer" --format table
```
Tabular format, particularly useful for search results and multiple items.

### CLI Examples

#### Complete Workflow Examples

##### 1. Research a Person
```bash
# Get detailed profile
in_scraper person "https://linkedin.com/in/john-doe" --format summary --verbose

# Save full data for later analysis
in_scraper person "https://linkedin.com/in/john-doe" --output john_doe.json
```

##### 2. Find People in Your Field
```bash
# Search for professionals
in_scraper people "Senior Developer" --location "New York" --format table

# Get detailed info on top candidates
in_scraper people "Senior Developer" --location "New York" --details 5 --output candidates.json
```

##### 3. Company Analysis
```bash
# Get company overview
in_scraper company "https://linkedin.com/company/tech-startup" --format summary

# Deep dive with employees
in_scraper company "https://linkedin.com/company/tech-startup" --employees --output company_data.json
```

##### 4. Job Market Research
```bash
# Survey the market
in_scraper jobs "Senior Developer" --location "New York" --format table

# Get detailed info on top opportunities
in_scraper jobs "Senior Developer" --location "New York" --details 5 --output opportunities.json
```

##### 5. Specific Job Investigation
```bash
# Analyze a specific role
in_scraper job "https://linkedin.com/jobs/view/3456789012" --format summary --verbose
```

### Advanced CLI Usage

#### Chaining with Unix Tools

```bash
# Count total jobs found
in_scraper jobs "Python Developer" --format json | jq '. | length'

# Extract just company names
in_scraper jobs "React Developer" --format json | jq '.[].company'

# Filter jobs by specific criteria
in_scraper jobs "Machine Learning" --format json | jq '.[] | select(.location | contains("Remote"))'

# Count people found in search
in_scraper people "Data Scientist" --format json | jq '. | length'

# Extract people names and headlines
in_scraper people "Product Manager" --format json | jq '.[] | {name: .name, headline: .headline}'
```

#### Automation Scripts

```bash
#!/bin/bash
# research_companies.sh

companies=(
    "https://linkedin.com/company/google"
    "https://linkedin.com/company/microsoft"
    "https://linkedin.com/company/apple"
)

for company in "${companies[@]}"; do
    name=$(echo $company | sed 's/.*\///')
    echo "Researching $name..."
    in_scraper company "$company" --employees --output "${name}_data.json" --verbose
    sleep 5  # Rate limiting
done
```

```bash
#!/bin/bash
# find_candidates.sh

# Search for candidates and save results
in_scraper people "Senior Software Engineer" --location "San Francisco" --details 10 --output sf_engineers.json

# Search for data scientists
in_scraper people "Data Scientist" --location "New York" --details 10 --output ny_data_scientists.json

echo "Candidate research complete!"
```

#### Environment Setup Script

```bash
#!/bin/bash
# setup_linkedin_scraper.sh

echo "Setting up LinkedIn Scraper environment..."

read -p "LinkedIn Email: " email
read -s -p "LinkedIn Password: " password
echo

export LINKEDIN_EMAIL="$email"
export LINKEDIN_PASSWORD="$password"

echo "Environment configured! You can now use:"
echo "  in_scraper person <url>"
echo "  in_scraper people <query>"
echo "  in_scraper company <url>"
echo "  in_scraper jobs <query>"
```

### Error Handling

The CLI provides helpful error messages:

```bash
# Invalid URL
in_scraper person "not-a-url"
# Error: Invalid LinkedIn URL format

# Authentication failure
in_scraper person "https://linkedin.com/in/someone" --email invalid@email.com
# Error: Failed to login to LinkedIn: Authentication failed

# Rate limiting
in_scraper people "Engineer" --details 100
# Error: Rate limited - please wait before retrying
```

### Tips for Success

1. **Use Environment Variables**: Set `LINKEDIN_EMAIL` and `LINKEDIN_PASSWORD` to avoid typing credentials repeatedly.

2. **Rate Limiting**: Add delays between requests when scraping multiple profiles:
   ```bash
   for url in $(cat urls.txt); do
       in_scraper person "$url" --output "profile_$(basename $url).json"
       sleep 5
   done
   ```

3. **Verbose Mode**: Use `--verbose` to see what's happening during scraping:
   ```bash
   in_scraper company "https://linkedin.com/company/example" --employees --verbose
   ```

4. **Output Files**: Save results to files for later analysis:
   ```bash
   in_scraper people "Data Science" --location "Remote" --details 10 --output remote_ds_people.json
   ```

5. **Format Selection**: Choose the right format for your use case:
   - `json`: For programmatic processing
   - `pretty`: For manual review
   - `summary`: For quick overview
   - `table`: For comparing multiple items

### Security Notes

- Never commit credentials to version control
- Use environment variables or secure credential storage
- Be mindful of LinkedIn's Terms of Service
- Respect rate limits to avoid being blocked
- Consider using the tool for legitimate research purposes only

### Performance Tips

- Use summary format for faster processing when you don't need all details
- Avoid `--employees` flag for companies unless necessary (it's slower)
- Use `--details` sparingly as it requires additional requests per item
- Consider running during off-peak hours for better performance

## Data Models

The scraper returns structured data using these main types:

### Person
```rust
pub struct Person {
    pub linkedin_url: String,
    pub name: Option<String>,
    pub headline: Option<String>,
    pub location: Option<String>,
    pub about: Option<String>,
    pub experiences: Vec<Experience>,
    pub educations: Vec<Education>,
    pub interests: Vec<Interest>,
    pub accomplishments: Vec<Accomplishment>,
    pub contacts: Vec<Contact>,
    pub open_to_work: bool,
}
```

### Company
```rust
pub struct Company {
    pub linkedin_url: String,
    pub name: Option<String>,
    pub about: Option<String>,
    pub website: Option<String>,
    pub headquarters: Option<String>,
    pub founded: Option<i32>,
    pub industry: Option<String>,
    pub company_size: Option<String>,
    pub specialties: Vec<String>,
    pub employees: Vec<Employee>,
    pub follower_count: Option<i32>,
}
```

### Job
```rust
pub struct Job {
    pub linkedin_url: String,
    pub title: Option<String>,
    pub company: Option<String>,
    pub company_linkedin_url: Option<String>,
    pub location: Option<String>,
    pub posted_date: Option<String>,
    pub applicant_count: Option<i32>,
    pub description: Option<String>,
    pub employment_type: Option<String>,
    pub seniority_level: Option<String>,
}
```

## Library Examples

Run the provided library examples:

```bash
# Set credentials
export LINKEDIN_EMAIL="your-email@example.com"
export LINKEDIN_PASSWORD="your-password"

# Scrape a person profile
cargo run --example scrape_person

# Scrape a company
cargo run --example scrape_company  

# Search jobs
cargo run --example search_jobs
```

## Error Handling

The library uses a comprehensive error system:

```rust
use in_scraper::{LinkedInError, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = LinkedInClient::login("email", "password").await?;
    
    match client.scrape_person("invalid-url").await {
        Ok(person) => println!("Success: {:?}", person.name),
        Err(LinkedInError::ProfileNotFound(url)) => {
            println!("Profile not found: {}", url);
        },
        Err(LinkedInError::RateLimited) => {
            println!("Rate limited - wait before retrying");
        },
        Err(e) => println!("Other error: {}", e),
    }
    
    Ok(())
}
```

## Rate Limiting

LinkedIn has rate limiting. To avoid being blocked:

- Add delays between requests
- Use residential proxies if scraping at scale  
- Respect robots.txt
- Don't scrape more than you need

## Legal Notice

This tool is for educational and research purposes. Always check LinkedIn's Terms of Service and respect rate limits. The authors are not responsible for any misuse of this software.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details.
