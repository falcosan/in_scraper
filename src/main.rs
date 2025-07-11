use anyhow::{Context, Result};
use chrono::Local;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::{from_str, Value};
use std::env;
use tokio::fs;

const MAX_PAGES: u32 = 100;
const FILTER_ALL: &str = "all";
const CONCURRENT_REQUESTS: usize = 20;
const FILTER_OPEN_TO_WORK: &str = "open-to-work";
const ENTITY_TYPE: &str = "com.linkedin.voyager.dash.search.EntityResultViewModel";
const LINKEDIN_SEARCH_URL: &str = "https://www.linkedin.com/search/results/people/?keywords={}&page={}";

type PersonData = (String, String, String, String);

static JSON_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("code[id*='bpr-guid']").unwrap());
static SEARCH_SESSION_TOKEN: Lazy<String> =
    Lazy::new(|| env::var("SEARCH_SESSION_TOKEN").expect("SEARCH_SESSION_TOKEN not found"));

async fn fetcher(client: &Client, search_query: &str, page: u32) -> Result<String> {
    let url = LINKEDIN_SEARCH_URL
        .replace("{}", search_query)
        .replacen("{}", &page.to_string(), 1);
    
    client
        .get(&url)
        .header("cookie", format!("li_at={}", *SEARCH_SESSION_TOKEN))
        .send()
        .await?
        .text()
        .await
        .context("Failed to get response text")
}

fn extract_people_data(json_value: &Value, filter_open_to_work: bool) -> Vec<PersonData> {
    json_value
        .get("included")
        .and_then(Value::as_array)
        .map(|included| {
            included
                .iter()
                .filter_map(|item| extract_person_from_item(item, filter_open_to_work))
                .collect()
        })
        .unwrap_or_default()
}

fn extract_person_from_item(item: &Value, filter_open_to_work: bool) -> Option<PersonData> {
    if item.get("$type").and_then(Value::as_str) != Some(ENTITY_TYPE) {
        return None;
    }

    let name = item.pointer("/title/text").and_then(Value::as_str).unwrap_or_default();
    let profile_link = item.get("navigationUrl").and_then(Value::as_str).unwrap_or_default();
    let position = item.pointer("/primarySubtitle/text").and_then(Value::as_str).unwrap_or_default();
    let image_url = item
        .pointer("/image/attributes/0/detailData/nonEntityProfilePicture/vectorImage/artifacts/0/fileIdentifyingUrlPathSegment")
        .and_then(Value::as_str)
        .unwrap_or_default();
    
    let is_open_to_work = item
        .pointer("/image/accessibilityText")
        .and_then(Value::as_str)
        .map_or(false, |text| text.to_lowercase().replace(' ', "_").contains("open_to_work"));

    if filter_open_to_work && !is_open_to_work {
        return None;
    }

    Some((name.to_string(), position.to_string(), image_url.to_string(), profile_link.to_string()))
}

async fn save_people_as_html(
    people: &[PersonData],
    search_query: &str,
    filter_open_to_work: bool,
) -> Result<()> {
    let sanitized_keyword = search_query
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .to_lowercase();

    let filter_type = if filter_open_to_work { FILTER_OPEN_TO_WORK } else { FILTER_ALL };
    let people_count = people.len();
    let formatted_keyword = sanitized_keyword.replace('_', " ").to_uppercase();
    let timestamp = Local::now().format("%d-%m-%Y_%H-%M-%S");
    let current_time = Local::now().format("%d/%m/%Y at %H:%M:%S").to_string();

    let filename = format!("{}_{}_{}people_{}.html", sanitized_keyword, people_count, filter_type, timestamp);
    let output_dir = env::current_dir()?.join("output");
    fs::create_dir_all(&output_dir).await?;

    let html_content = generate_html_content(people, &formatted_keyword, people_count, &current_time);
    let output_file = output_dir.join(&filename);
    
    fs::write(&output_file, html_content).await?;
    
    tokio::task::spawn_blocking(move || {
        if let Err(e) = open::that(&output_file) {
            eprintln!("Failed to open file: {}", e);
        }
    });

    Ok(())
}

fn generate_html_content(people: &[PersonData], formatted_keyword: &str, people_count: usize, current_time: &str) -> String {
    let mut html = format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{} | {} profiles</title><style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background-color: #f3f2ef;
            margin: 0;
            padding: 20px;
        }}
        ul {{
            list-style: none;
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 20px;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }}
        li {{
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            transition: transform 0.2s, box-shadow 0.2s;
        }}
        li:hover {{
            transform: translateY(-3px);
            box-shadow: 0 5px 15px rgba(0,0,0,0.15);
        }}
        img {{
            border-radius: 50%;
            display: block;
            margin: 15px auto;
            border: 3px solid #f3f2ef;
            object-fit: cover;
        }}
        a {{
            text-decoration: none;
            color: inherit;
            padding: 15px;
            display: block;
        }}
        h3 {{
            margin: 10px 0;
            color: #0a66c2;
            text-align: center;
        }}
        p {{
            color: #333333;
            text-align: center;
            margin: 10px 0;
        }}
        </style></head><body><h1>{} | {} profiles | {}</h1><ul>"#,
        formatted_keyword, people_count, formatted_keyword, people_count, current_time
    );

    for (name, position, image_url, profile_link) in people {
        html.push_str(&format!(
            r#"<li><a href="{}" target="_blank"><img src="{}" alt="{}" width="100" height="100"><h3>{}</h3><p>{}</p></a></li>"#,
            profile_link, image_url, name, name, position
        ));
    }

    html.push_str("</ul></body></html>");
    html
}

async fn process_page(
    client: &Client,
    search_query: &str,
    page: u32,
    filter_open_to_work: bool,
) -> Vec<PersonData> {
    match fetcher(client, search_query, page).await {
        Ok(body) => {
            let document = Html::parse_document(&body);
            document
                .select(&JSON_SELECTOR)
                .filter_map(|element| from_str::<Value>(&element.text().collect::<String>()).ok())
                .find_map(|json| {
                    json.pointer("/data/data/searchDashClustersByAll")
                        .map(|_| extract_people_data(&json, filter_open_to_work))
                })
                .unwrap_or_default()
        }
        Err(_) => Vec::new(),
    }
}

async fn scrape_linkedin_profiles(search_query: &str, filter_open_to_work: bool) -> Result<()> {
    let client = Client::new();
    let pb = ProgressBar::new(MAX_PAGES.into()).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} pages ({eta})")
            .map_err(|e| anyhow::anyhow!("Failed to create progress bar template: {}", e))?
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    let people_data: Vec<PersonData> = stream::iter(1..=MAX_PAGES)
        .map(|page| {
            let client = &client;
            let search_query = search_query;
            let pb = pb.clone();

            async move {
                let result = process_page(client, search_query, page, filter_open_to_work).await;
                pb.inc(1);
                result
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS)
        .flat_map(stream::iter)
        .collect()
        .await;

    pb.finish_and_clear();

    if people_data.is_empty() {
        eprintln!(
            "No LinkedIn profiles found for search query: '{}' (filter: {})",
            search_query,
            if filter_open_to_work { "open-to-work only" } else { "all profiles" }
        );
        return Ok(());
    }

    save_people_as_html(&people_data, search_query, filter_open_to_work).await?;
    println!("Successfully scraped {} profiles", people_data.len());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <search_query> [filter_open_to_work=true/false]", args[0]);
        std::process::exit(1);
    }

    let search_query = &args[1];
    let filter_open_to_work = args.get(2).map_or(false, |arg| arg == "true");

    scrape_linkedin_profiles(search_query, filter_open_to_work)
        .await
        .context("Failed to scrape LinkedIn profiles")
}
