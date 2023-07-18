use clap::{crate_authors, crate_description, crate_name, crate_version, Command};
use serde::{Deserialize, Serialize};
use std::env;
use reqwest;
use reqwest::header::USER_AGENT;
use log::{debug, info, error, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use tera::Tera;
use tera::Context;
use crate::notification::send_email_smtp;
use crate::constants::*;

mod constants;
mod notification;

#[derive(Deserialize, Debug, Clone, Serialize)]
#[allow(non_snake_case)]
struct Offer {
    dateTime: String,
    time: String,
    url: String,
}

#[derive(Deserialize, Debug, Clone)]
struct APIResponse {
    error: Option<String>,

    #[serde(default)]
    offers: Vec<Offer>,
}

fn setup_logger() {
    let file_path: String = env::var("LOG_PATH").expect("No log path").parse().unwrap();

    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {l} - {m}\n")))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    let _ = log4rs::init_config(config);
}

fn build_email_body(offers: Vec<Offer>) -> String {
    let mut tera = Tera::default();
    tera.add_template_file("templates/email.txt", Some("email.txt")).unwrap();

    let mut context = Context::new();
    context.insert("offers", &offers);
    context.insert("base_url", DISNEY_ROOT_URL);
    tera.render("email.txt", &context).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();
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
                    let text_response = response.text().await.unwrap();
                    debug!("{:?}", text_response.clone());
                    match serde_json::from_str::<APIResponse>(text_response.as_str()) {
                        Ok(parsed) => {
                            info!("{:?}", parsed.clone());
                            if !parsed.error.is_none() {
                                error!("Request error message {}",  parsed.error.expect("Unknown error").as_str());
                                std::process::exit(1);
                            }
                            if !parsed.offers.is_empty() {
                                let body = build_email_body(parsed.offers.clone());
                                send_email_smtp(body.as_str()).await.expect("Error sending email");
                            }
                        }
                        Err(e) => error!("An error occurred while parsing: {}", e)
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
