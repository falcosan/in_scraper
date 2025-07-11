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

    let client = LinkedInClient::login_with_retry(&email, &password, 2).await
        .map_err(|e| {
            match &e {
                in_scraper::LinkedInError::AuthenticationFailed => {
                    eprintln!("Authentication failed. Please check your LinkedIn credentials.");
                    eprintln!("Make sure you can log in via web browser first.");
                }
                in_scraper::LinkedInError::Unknown(msg) if msg.contains("challenge") => {
                    eprintln!("LinkedIn security challenge detected.");
                    eprintln!("{}", msg);
                    eprintln!("\nRun './linkedin_auth_guide.sh' for detailed troubleshooting steps.");
                }
                in_scraper::LinkedInError::RateLimited => {
                    eprintln!("Rate limited by LinkedIn. Please wait and try again later.");
                }
                _ => {
                    eprintln!("Failed to login to LinkedIn: {}", e);
                    eprintln!("Please check your credentials and network connection.");
                    eprintln!("Run './linkedin_auth_guide.sh' for troubleshooting help.");
                }
            }
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