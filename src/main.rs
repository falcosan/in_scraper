use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{ info, error };
use clap::{ Parser, Subcommand };
use in_scraper::{
    config::Config,
    utils::ProxyValidator,
    pipeline::JsonPipeline,
    spiders::{ CompanyProfileSpider, JobsSpider, PeopleProfileSpider, Spider },
};

#[derive(Parser)]
#[command(name = "in-scraper")]
#[command(about = "LinkedIn data scraper", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CompanyProfile {
        #[arg(long)]
        urls: Vec<String>,

        #[arg(short, long, default_value = "1")]
        concurrent: usize,

        #[arg(short, long, default_value = "data")]
        output: String,

        #[arg(long)]
        timeout: Option<u64>,

        #[arg(long)]
        retries: Option<u32>,

        #[arg(long, value_delimiter = ',')]
        proxies: Option<Vec<String>>,
    },
    Jobs {
        #[arg(long)]
        keywords: String,
        #[arg(long)]
        location: String,

        #[arg(short, long, default_value = "1")]
        concurrent: usize,

        #[arg(short, long, default_value = "data")]
        output: String,

        #[arg(long)]
        timeout: Option<u64>,

        #[arg(long)]
        retries: Option<u32>,

        #[arg(long, value_delimiter = ',')]
        proxies: Option<Vec<String>>,
    },
    PeopleProfile {
        #[arg(long)]
        profiles: Vec<String>,

        #[arg(short, long, default_value = "1")]
        concurrent: usize,

        #[arg(short, long, default_value = "data")]
        output: String,

        #[arg(long)]
        timeout: Option<u64>,

        #[arg(long)]
        retries: Option<u32>,

        #[arg(long, value_delimiter = ',')]
        proxies: Option<Vec<String>>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let mut config = Config::from_env();

    match &cli.command {
        Commands::CompanyProfile { urls: _, concurrent, output, timeout, retries, proxies } => {
            configure_and_run_company_profile(
                &mut config,
                &cli.command,
                *concurrent,
                output,
                *timeout,
                *retries,
                proxies
            ).await?;
        }
        Commands::Jobs {
            keywords: _,
            location: _,
            concurrent,
            output,
            timeout,
            retries,
            proxies,
        } => {
            configure_and_run_jobs(
                &mut config,
                &cli.command,
                *concurrent,
                output,
                *timeout,
                *retries,
                proxies
            ).await?;
        }
        Commands::PeopleProfile { profiles: _, concurrent, output, timeout, retries, proxies } => {
            configure_and_run_people_profile(
                &mut config,
                &cli.command,
                *concurrent,
                output,
                *timeout,
                *retries,
                proxies
            ).await?;
        }
    }

    Ok(())
}

async fn configure_and_run_company_profile(
    config: &mut Config,
    command: &Commands,
    concurrent: usize,
    output: &str,
    timeout: Option<u64>,
    retries: Option<u32>,
    proxies: &Option<Vec<String>>
) -> Result<()> {
    if let Commands::CompanyProfile { urls, .. } = command {
        configure_common(config, concurrent, output, timeout, retries, proxies).await?;
        let config = Arc::new(config.clone());
        let pipeline = Arc::new(JsonPipeline::new(config.clone()));
        let spider = CompanyProfileSpider::new(config.clone(), urls.clone());
        run_spider(spider, pipeline).await?;
    }
    Ok(())
}

async fn configure_and_run_jobs(
    config: &mut Config,
    command: &Commands,
    concurrent: usize,
    output: &str,
    timeout: Option<u64>,
    retries: Option<u32>,
    proxies: &Option<Vec<String>>
) -> Result<()> {
    if let Commands::Jobs { keywords, location, .. } = command {
        configure_common(config, concurrent, output, timeout, retries, proxies).await?;
        let config = Arc::new(config.clone());
        let pipeline = Arc::new(JsonPipeline::new(config.clone()));
        let spider = JobsSpider::new(config.clone(), keywords.clone(), location.clone());
        run_spider(spider, pipeline).await?;
    }
    Ok(())
}

async fn configure_and_run_people_profile(
    config: &mut Config,
    command: &Commands,
    concurrent: usize,
    output: &str,
    timeout: Option<u64>,
    retries: Option<u32>,
    proxies: &Option<Vec<String>>
) -> Result<()> {
    if let Commands::PeopleProfile { profiles, .. } = command {
        configure_common(config, concurrent, output, timeout, retries, proxies).await?;
        let config = Arc::new(config.clone());
        let pipeline = Arc::new(JsonPipeline::new(config.clone()));
        let spider = PeopleProfileSpider::new(config.clone(), profiles.clone());
        run_spider(spider, pipeline).await?;
    }
    Ok(())
}

async fn configure_common(
    config: &mut Config,
    concurrent: usize,
    output: &str,
    timeout: Option<u64>,
    retries: Option<u32>,
    proxies: &Option<Vec<String>>
) -> Result<()> {
    config.concurrent_requests = concurrent;
    config.output_dir = output.to_string();

    if let Some(timeout) = timeout {
        config.request_timeout = timeout;
    }

    if let Some(retries) = retries {
        config.max_retries = retries;
    }

    if let Some(proxies) = proxies {
        info!("Validating {} proxies from command line...", proxies.len());
        let validator = ProxyValidator::new();
        let valid_proxies = validator.validate_proxies(proxies).await;

        if valid_proxies.is_empty() {
            error!("No valid proxies found! Running without proxy rotation.");
            config.proxy_rotation_enabled = false;
        } else {
            config.proxies = valid_proxies;
            config.proxy_rotation_enabled = true;
            info!("Using {} validated proxies", config.proxies.len());
        }
    }

    if config.has_proxies() {
        info!("Proxy rotation enabled with {} proxies", config.proxy_count());
    } else {
        info!("Running without proxy rotation");
    }

    Ok(())
}

async fn run_spider<S: Spider + 'static>(spider: S, pipeline: Arc<JsonPipeline>) -> Result<()> {
    info!("Starting spider: {}", spider.name());

    let semaphore = Arc::new(Semaphore::new(spider.get_config().concurrent_requests));
    let mut request_queue = spider.start_requests().await;
    let mut handles = vec![];

    while !request_queue.is_empty() || !handles.is_empty() {
        while let Some(request) = request_queue.pop() {
            let spider_clone = spider.clone();
            let pipeline_clone = pipeline.clone();
            let semaphore_clone = semaphore.clone();

            let handle = tokio::spawn(async move {
                let _permit = semaphore_clone.acquire().await.unwrap();

                match spider_clone.execute_request(request).await {
                    Ok((items, next_requests)) => {
                        for item in items {
                            if
                                let Err(e) = pipeline_clone.process_item(
                                    spider_clone.name(),
                                    item
                                ).await
                            {
                                error!("Pipeline error: {}", e);
                            }
                        }
                        next_requests
                    }
                    Err(e) => {
                        error!("Spider error: {}", e);
                        vec![]
                    }
                }
            });

            handles.push(handle);
        }

        if !handles.is_empty() {
            let (result, _, remaining) = futures::future::select_all(handles).await;
            handles = remaining;

            if let Ok(next_requests) = result {
                request_queue.extend(next_requests);
            }
        }
    }

    info!("Spider {} completed", spider.name());
    Ok(())
}
