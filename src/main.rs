use clap::Parser;
use in_scraper::{LinkedInClient, Result};
use rpassword::read_password;
use std::io::{self, Write};

mod cli;
mod commands;

use cli::Cli;
use commands::execute_command;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    if args.verbose {
        eprintln!("in_scraper v{}", env!("CARGO_PKG_VERSION"));
    }

    let email = get_credential(
        args.email,
        "LinkedIn Email",
        "LINKEDIN_EMAIL",
        false,
    )?;
    
    let password = get_credential(
        args.password,
        "LinkedIn Password",
        "LINKEDIN_PASSWORD",
        true,
    )?;

    if args.verbose {
        eprintln!("Logging into LinkedIn...");
    }

    let client = LinkedInClient::login(&email, &password).await
        .map_err(|e| {
            eprintln!("Failed to login to LinkedIn: {}", e);
            eprintln!("Please check your credentials and try again.");
            e
        })?;

    if args.verbose {
        eprintln!("Successfully logged in!");
    }

    execute_command(&client, args.command, args.format, args.output, args.verbose).await?;

    Ok(())
}

fn get_credential(
    provided: Option<String>,
    prompt: &str,
    env_var: &str,
    is_password: bool,
) -> Result<String> {
    if let Some(cred) = provided {
        return Ok(cred);
    }

    if let Ok(cred) = std::env::var(env_var) {
        return Ok(cred);
    }

    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    let credential = if is_password {
        read_password().map_err(|e| {
            in_scraper::LinkedInError::Unknown(format!("Failed to read password: {}", e))
        })?
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            in_scraper::LinkedInError::Unknown(format!("Failed to read input: {}", e))
        })?;
        input.trim().to_string()
    };

    if credential.is_empty() {
        return Err(in_scraper::LinkedInError::Unknown(
            format!("{} cannot be empty", prompt)
        ));
    }

    Ok(credential)
} 