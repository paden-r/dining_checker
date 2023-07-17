use clap::{crate_authors, crate_description, crate_name, crate_version, Command};
use serde::Deserialize;
use std::env;
use reqwest;
use reqwest::header::USER_AGENT;
use crate::constants::*;

mod constants;

#[derive(Deserialize, Debug)]
struct Offer {
    datetime: String,
    time: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct APIResponse {
    error: Option<String>,

    #[serde(default)]
    offers: Vec<Offer>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(
            Command::new("check")
                .about("Checks for dining reservation availability.")
        );

    let m = cmd.get_matches_mut();

    match m.subcommand() {
        Some(("check", _sub_m)) => {
            let http_client = reqwest::Client::new();
            let response = http_client
                .get(SPACE_URL.as_str())
                .header(USER_AGENT, VALID_USER_AGENT)
                .send()
                .await
                .expect("Invalid Response");

            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<APIResponse>().await {
                        Ok(parsed) => println!("{:?}", parsed),
                        Err(_) => println!("An error occurred while parsing")
                    };
                }
                other => {
                    panic!("Uh oh! Something unexpected happened: {:?}", other);
                }
            };
        }
        Some((_, _)) => {
            cmd.print_help().unwrap();
        }
        None => {
            cmd.print_help().unwrap();
        }
    }

    Ok(())

}
