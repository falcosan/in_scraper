# Rust LinkedIn Scraper

A command-line tool that searches LinkedIn for professionals matching specific keywords and generates a visual HTML report of matching profiles.

## Features

- Search LinkedIn profiles based on keywords
- Filter for professionals who are "Open to Work"
- Generate a clean, responsive HTML gallery of profiles

## Prerequisites

- Rust and Cargo installed
- A valid LinkedIn session token

## Installation

1. Clone this repository
2. Build the project with:

```bash
cargo build --release
```

## Configuration

Before running the scraper, you need to set your LinkedIn session token:

```bash
export SEARCH_SESSION_TOKEN="your_linkedin_session_token"
```

You can find your LinkedIn session token by:

1. Logging into LinkedIn in your browser
2. Opening Developer Tools (F12)
3. Going to the Application/Storage tab
4. Finding the "li_at" cookie value under Cookies

## Usage

### Using the provided script

```bash
./search.sh "search keywords"
```

To filter for only "Open to Work" profiles:

```bash
./search.sh --open-to-work "search keywords"
```

### Using cargo directly

```bash
cargo run --release "search keywords"
# Or for Open to Work profiles:
cargo run --release "search keywords" true
```

## Output

The script generates an HTML file in the `output` directory with the naming convention:
`search_query_count_filter-type_date_hour.html`

For example: `software_engineer_125_open-to-work_30-02-2025_18-29-34.html`

## Limitations

This tool is for educational purposes. Use responsibly and respect LinkedIn's terms of service and rate limits.
