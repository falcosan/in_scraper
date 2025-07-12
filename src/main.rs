use clap::Parser;
use std::io::{ self, Write };
use rpassword::read_password;
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

    let email = get_credential(args.email, "LinkedIn Email", "LINKEDIN_EMAIL", false)?;
    let password = get_credential(args.password, "LinkedIn Password", "LINKEDIN_PASSWORD", true)?;

    if verbose {
        eprintln!("Logging into LinkedIn...");
    }

    let client = LinkedInClient::login_with_retry(&email, &password, 2).await.inspect_err(|e| {
        match e {
            in_scraper::LinkedInError::AuthenticationFailed => {
                eprintln!("Authentication failed. Please check your LinkedIn credentials.");
                eprintln!("Make sure you can log in via web browser first.");
            }
            in_scraper::LinkedInError::RateLimited => {
                eprintln!("Rate limited by LinkedIn. Please wait and try again later.");
            }
            msg => {
                eprintln!("{msg}");
            }
        }
    })?;

    if verbose {
        eprintln!("Successfully logged in!");
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

fn get_credential(
    provided: Option<String>,
    prompt: &str,
    env_var: &str,
    is_password: bool
) -> Result<String> {
    if let Some(cred) = provided {
        return Ok(cred);
    }

    if let Ok(cred) = std::env::var(env_var) {
        return Ok(cred);
    }

    print!("{prompt}: ");
    io::stdout().flush().unwrap();

    let credential = if is_password {
        read_password().map_err(|e| {
            in_scraper::LinkedInError::Unknown(format!("Failed to read password: {e}"))
        })?
    } else {
        let mut input = String::new();
        io
            ::stdin()
            .read_line(&mut input)
            .map_err(|e| {
                in_scraper::LinkedInError::Unknown(format!("Failed to read input: {e}"))
            })?;
        input.trim().to_string()
    };

    if credential.is_empty() {
        return Err(in_scraper::LinkedInError::Unknown(format!("{prompt} cannot be empty")));
    }

    Ok(credential)
}
