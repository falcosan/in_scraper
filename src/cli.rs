use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "in_scraper")]
#[command(about = "A fast LinkedIn scraper written in Rust")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, env = "LINKEDIN_EMAIL")]
    pub email: Option<String>,

    #[arg(short, long, env = "LINKEDIN_PASSWORD")]
    pub password: Option<String>,

    #[arg(short = 'f', long, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Person {
        url: String,
    },
    People {
        query: String,
        #[arg(short, long)]
        location: Option<String>,
        #[arg(long, default_value = "0")]
        details: usize,
    },
    Company {
        url: String,
        #[arg(long)]
        employees: bool,
    },
    Jobs {
        query: String,
        #[arg(short, long)]
        location: Option<String>,
        #[arg(long, default_value = "0")]
        details: usize,
    },
    Job {
        url: String,
    },
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum OutputFormat {
    Json,
    Pretty,
    Table,
    Summary,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Pretty => write!(f, "pretty"),
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Summary => write!(f, "summary"),
        }
    }
} 