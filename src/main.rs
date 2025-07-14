use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{ info, error };
use clap::{ Parser, Subcommand };
use in_scraper::{
    config::Config,
    pipeline::JsonLinesPipeline,
    spiders::{ CompanyProfileSpider, JobsSpider, PeopleProfileSpider, Spider },
};

#[derive(Parser)]
#[command(name = "in-scraper")]
#[command(about = "LinkedIn data scraper", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "1")]
    concurrent: usize,

    #[arg(short, long, default_value = "data")]
    output: String,

    #[arg(long)]
    timeout: Option<u64>,

    #[arg(long)]
    retries: Option<u32>,
}

#[derive(Subcommand)]
enum Commands {
    CompanyProfile {
        #[arg(long)]
        urls: Vec<String>,
    },
    Jobs {
        #[arg(long)]
        keywords: String,
        #[arg(long)]
        location: String,
    },
    PeopleProfile {
        #[arg(long)]
        profiles: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let mut config = Config {
        concurrent_requests: cli.concurrent,
        output_dir: cli.output,
        ..Default::default()
    };

    if let Some(timeout) = cli.timeout {
        config.request_timeout = timeout;
    }

    if let Some(retries) = cli.retries {
        config.max_retries = retries;
    }

    let config = Arc::new(config);
    let pipeline = Arc::new(JsonLinesPipeline::new(config.clone()));

    match cli.command {
        Commands::CompanyProfile { urls } => {
            let spider = CompanyProfileSpider::new(config.clone(), urls);
            run_spider(spider, pipeline).await?;
        }
        Commands::Jobs { keywords, location } => {
            let spider = JobsSpider::new(config.clone(), keywords, location);
            run_spider(spider, pipeline).await?;
        }
        Commands::PeopleProfile { profiles } => {
            let spider = PeopleProfileSpider::new(config.clone(), profiles);
            run_spider(spider, pipeline).await?;
        }
    }

    Ok(())
}

async fn run_spider<S: Spider + 'static>(
    spider: S,
    pipeline: Arc<JsonLinesPipeline>
) -> Result<()> {
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
