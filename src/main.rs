use clap::Parser;
use commands::execute_command;
use cli::{ Cli, OutputFormat };
use in_scraper::{ LinkedInClient, Result };

mod cli;
mod commands;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let args = Cli::parse();
    let (format, output, verbose) = get_command_options(&args);

    if verbose {
        eprintln!("in_scraper v{}", env!("CARGO_PKG_VERSION"));
    }

    let li_at_cookie = args.li_at
        .or_else(|| std::env::var("LINKEDIN_LI_AT").ok())
        .ok_or_else(||
            in_scraper::LinkedInError::Unknown(
                "LinkedIn li_at cookie is required. Provide it via --li-at parameter or LINKEDIN_LI_AT environment variable.".to_string()
            )
        )?;

    if verbose {
        eprintln!("Using li_at cookie for authentication...");
    }

    let client = LinkedInClient::new_with_cookie(&li_at_cookie).map_err(|e| {
        eprintln!("Failed to create client with li_at cookie: {}", e);
        e
    })?;

    if verbose {
        eprintln!("Successfully authenticated!");
    }

    execute_command(&client, args.command, format, output, verbose).await?;

    Ok(())
}

fn get_command_options(args: &Cli) -> (OutputFormat, Option<String>, bool) {
    let global_format = args.format.clone();
    let global_output = args.output.clone();
    let global_verbose = args.verbose;

    match &args.command {
        cli::Commands::Person { format, output, verbose, .. } => {
            (
                format.clone().unwrap_or(global_format),
                output.clone().or(global_output),
                *verbose || global_verbose,
            )
        }
        cli::Commands::People { format, output, verbose, .. } => {
            (
                format.clone().unwrap_or(global_format),
                output.clone().or(global_output),
                *verbose || global_verbose,
            )
        }
        cli::Commands::Company { format, output, verbose, .. } => {
            (
                format.clone().unwrap_or(global_format),
                output.clone().or(global_output),
                *verbose || global_verbose,
            )
        }
        cli::Commands::Jobs { format, output, verbose, .. } => {
            (
                format.clone().unwrap_or(global_format),
                output.clone().or(global_output),
                *verbose || global_verbose,
            )
        }
        cli::Commands::Job { format, output, verbose, .. } => {
            (
                format.clone().unwrap_or(global_format),
                output.clone().or(global_output),
                *verbose || global_verbose,
            )
        }
    }
}
