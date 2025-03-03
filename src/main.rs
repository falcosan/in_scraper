use chrono::Local;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::{from_str, Value};
use std::{env, io::Write, sync::Arc};
use tokio;

const MAX_PAGES: u32 = 100;
const CONCURRENT_REQUESTS: usize = 20;

static JSON_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("code[id*='bpr-guid']").unwrap());
static SEARCH_SESSION_TOKEN: Lazy<String> =
    Lazy::new(|| env::var("SEARCH_SESSION_TOKEN").expect("SEARCH_SESSION_TOKEN not found"));

async fn fetcher(client: &Client, search_query: &str, page: u32) -> Result<String, reqwest::Error> {
    client
        .get(format!(
            "https://www.linkedin.com/search/results/people/?keywords={}&page={}",
            search_query, page
        ))
        .header("cookie", format!("li_at={}", *SEARCH_SESSION_TOKEN))
        .send()
        .await?
        .text()
        .await
}

fn extract_people_data(
    json_value: &Value,
    filter_open_to_work: bool,
) -> Vec<(String, String, String, String)> {
    json_value.get("included")
        .and_then(Value::as_array)
        .map_or_else(Vec::new, |included| {
            included.iter()
                .filter_map(|item| {
                    if item.get("$type").and_then(Value::as_str) == Some("com.linkedin.voyager.dash.search.EntityResultViewModel") {
                        let name = item.pointer("/title/text").and_then(Value::as_str).unwrap_or_default();
                        let profile_link = item.get("navigationUrl").and_then(Value::as_str).unwrap_or_default();
                        let position = item.pointer("/primarySubtitle/text").and_then(Value::as_str).unwrap_or_default();
                        let image_url = item.pointer("/image/attributes/0/detailData/nonEntityProfilePicture/vectorImage/artifacts/0/fileIdentifyingUrlPathSegment")
                            .and_then(Value::as_str)
                            .unwrap_or_default();
                        let is_open_to_work = item.pointer("/image/accessibilityText")
                            .and_then(Value::as_str)
                            .map_or(false, |text| text.to_lowercase().replace(' ', "_").contains("open_to_work"));

                        if !filter_open_to_work || is_open_to_work {
                            Some((name.to_string(), position.to_string(), image_url.to_string(), profile_link.to_string()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect()
        })
}

fn save_people_as_html(
    people: &[(String, String, String, String)],
    search_query: &str,
    filter_open_to_work: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let sanitized_keyword = search_query
        .replace(|c: char| !c.is_alphanumeric(), "_")
        .to_lowercase();
    let filename = format!(
        "{}_{}_{}-people_{}.html",
        sanitized_keyword,
        people.len(),
        if filter_open_to_work {
            "open-to-work"
        } else {
            "all"
        },
        Local::now().format("%d-%m-%Y_%H-%M-%S")
    );
    let output_dir = env::current_dir()?.join("output");
    std::fs::create_dir_all(&output_dir)?;

    let mut output = Vec::with_capacity(people.len() * 200);
    write!(
        &mut output,
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{} {} profiles</title><style>
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
        </style></head><body><ul>"#,
        sanitized_keyword.replace("_", " ").to_uppercase(),
        people.len()
    )?;

    for (name, position, image_url, profile_link) in people {
        write!(
            &mut output,
            r#"<li><a href="{}" target="_blank"><img src="{}" alt="{}" width="100" height="100"><h3>{}</h3><p>{}</p></a></li>"#,
            profile_link, image_url, name, name, position
        )?;
    }

    write!(&mut output, "</ul></body></html>")?;
    std::fs::write(output_dir.join(&filename), &output)?;
    open::that(output_dir.join(&filename))?;
    Ok(())
}

async fn process_page(
    client: Arc<Client>,
    search_query: Arc<String>,
    page: u32,
    filter_open_to_work: bool,
) -> Vec<(String, String, String, String)> {
    match fetcher(&client, search_query.as_str(), page).await {
        Ok(body) => Html::parse_document(&body)
            .select(&JSON_SELECTOR)
            .filter_map(|element| from_str::<Value>(&element.text().collect::<String>()).ok())
            .find_map(|json| {
                json.pointer("/data/data/searchDashClustersByAll")
                    .map(|_| extract_people_data(&json, filter_open_to_work))
            })
            .unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

async fn final_result(
    search_query: &str,
    filter_open_to_work: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(Client::new());
    let search_query = Arc::new(search_query.to_owned());
    let pb = ProgressBar::new(MAX_PAGES.into()).with_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} pages ({eta})")?
            .progress_chars("##-"),
    );

    let people_data: Vec<_> = stream::iter(1..=MAX_PAGES)
        .map(|page| {
            let client = client.clone();
            let search_query = search_query.clone();
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

    if !people_data.is_empty() {
        save_people_as_html(&people_data, &search_query, filter_open_to_work)?;
    } else {
        eprintln!(
            "No LinkedIn profiles found for search query: '{}' (filter: {})",
            search_query,
            if filter_open_to_work {
                "open-to-work only"
            } else {
                "all profiles"
            }
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} <search_query> [filter_open_to_work=true/false]",
            args[0]
        );
        std::process::exit(1);
    }

    let filter_open_to_work = args.get(2).map_or(false, |arg| arg == "true");

    if let Err(e) = final_result(&args[1], filter_open_to_work).await {
        eprintln!("Execution error: {}", e);
    }
}
