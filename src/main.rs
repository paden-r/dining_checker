use clap::{arg, crate_authors, crate_description, crate_name, crate_version, ArgAction, ArgGroup, Command, Id};
use color_eyre::Result;
use gitlab::api::projects::merge_requests::{MergeRequestChanges, MergeRequestChangesBuilderError};
use gitlab::api::ApiError;
use gitlab::{api::Query, Gitlab};
use owo_colors::OwoColorize;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::From;
use std::convert::TryFrom;
use std::env;
use std::io::Write;
use std::num::ParseIntError;
use std::iter::Iterator;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};


const SPACE_URL: &str = "https://disneyworld.disney.go.com/finder/api/v1/explorer-service/dining-availability/%7B9DDDAEF3-9DAC-46B6-B55B-A12FC04588DF%7D/wdw/19634138;entityType=restaurant/table-service/2/2023-09-13/?mealPeriod=80000717";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/116.0";


fn main() -> Result<()> {
    let mut cmd = Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(
            Command::new("check")
                .about("Checks for dining reservation availability.")
        );

    let m = cmd.get_matches_mut();

    color_eyre::install()?;
    match m.subcommand() {
        Some(("check", sub_m)) => {
            let http_client = ClientBuilder::new(reqwest::Client::new())
                .build();
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
