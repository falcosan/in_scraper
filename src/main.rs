use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{ info, error };
use clap::{ Parser, Subcommand };
use in_scraper::{
    config::Config,
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

        #[arg(short, long, default_value_t = 1)]
        concurrent: usize,

        #[arg(short, long, default_value = "data")]
        output: String,

        #[arg(long, default_value_t = 30)]
        timeout: u64,

        #[arg(long, default_value_t = 3)]
        retries: u32,
    },
    Jobs {
        #[arg(long)]
        keywords: String,
        #[arg(long)]
        location: String,

        #[arg(short, long, default_value_t = 1)]
        concurrent: usize,

        #[arg(short, long, default_value = "data")]
        output: String,

        #[arg(long, default_value_t = 30)]
        timeout: u64,

        #[arg(long, default_value_t = 3)]
        retries: u32,
    },
    PeopleProfile {
        #[arg(long)]
        profiles: Vec<String>,

        #[arg(short, long, default_value_t = 1)]
        concurrent: usize,

        #[arg(short, long, default_value = "data")]
        output: String,

        #[arg(long, default_value_t = 30)]
        timeout: u64,

        #[arg(long, default_value_t = 3)]
        retries: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let mut config = Config::from_env();

    match &cli.command {
        Commands::CompanyProfile { urls, concurrent, output, timeout, retries } => {
            configure_common(&mut config, *concurrent, output, *timeout, *retries);
            let config = Arc::new(config);
            let pipeline = Arc::new(JsonPipeline::new(config.clone()));
            let spider = CompanyProfileSpider::new(config.clone(), urls.clone());
            run_spider(spider, pipeline).await?;
        }
        Commands::Jobs { keywords, location, concurrent, output, timeout, retries } => {
            configure_common(&mut config, *concurrent, output, *timeout, *retries);
            let config = Arc::new(config);
            let pipeline = Arc::new(JsonPipeline::new(config.clone()));
            let spider = JobsSpider::new(config.clone(), keywords.clone(), location.clone());
            run_spider(spider, pipeline).await?;
        }
        Commands::PeopleProfile { profiles, concurrent, output, timeout, retries } => {
            configure_common(&mut config, *concurrent, output, *timeout, *retries);
            let config = Arc::new(config);
            let pipeline = Arc::new(JsonPipeline::new(config.clone()));
            let spider = PeopleProfileSpider::new(config.clone(), profiles.clone());
            run_spider(spider, pipeline).await?;
        }
    }

    Ok(())
}

fn configure_common(
    config: &mut Config,
    concurrent: usize,
    output: &str,
    timeout: u64,
    retries: u32
) {
    config.concurrent_requests = concurrent;
    config.output_dir = output.to_string();
    config.request_timeout = timeout;
    config.max_retries = retries;
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
