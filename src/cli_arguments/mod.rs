use clap::{crate_authors, crate_description, crate_name, crate_version, Command, Arg, ValueEnum};
use std::io::Write;

#[derive(ValueEnum, Clone)]
enum Restaurants {
    Boma,
    Space220Lounge,
    BeOurGuest
}

#[derive(Debug, Clone)]
pub struct CLIParameters {
    // restaurant: String,
    pub party_size: String,
    pub reservation_date: String,
    // reservation_time: String,
}


pub fn get_cli_arguments() -> Result<CLIParameters, &'static str> {
    let mut cmd = Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(
            Command::new("check")
                .about("Checks for dining reservation availability.")
                .arg(Arg::new("party_size")
                    // .value_parser(clap::value_parser!(u16).range(1..8))
                    .required(true)
                    .help("number of people in your party"))
                .arg(Arg::new("reservation_date").help("date format: YYYY-MM-DD"))
                // .arg(Arg::new("restaurant")
                //     .value_parser(clap::builder::EnumValueParser::<Restaurants>::new()))
        );

    let m = cmd.get_matches_mut();

    match m.subcommand() {
        Some(("check", sub_m)) => {
            let party_size = match sub_m.get_one::<String>("party_size") {
                Some(party_) => party_.to_string(),
                None => {
                    print!("Party size: ");
                    std::io::stdout().flush().unwrap();
                    let mut party_ = String::new();
                    std::io::stdin().read_line(&mut party_).unwrap();

                    party_.to_string()
                }
            };
            let date = match sub_m.get_one::<String>("reservation_date") {
                Some(date_) => date_.to_string(),
                None => {
                    print!("reservation date: ");
                    std::io::stdout().flush().unwrap();
                    let mut date_ = String::new();
                    std::io::stdin().read_line(&mut date_).unwrap();

                    date_.to_string()
                }
            };
            let args = CLIParameters {
                party_size,
                reservation_date: date
            };
            return Ok(args);
        }

        Some((_, _)) => {
            cmd.print_help().unwrap();
            Err("No matching command")
        }

        None => {
            cmd.print_help().unwrap();
            Err("No command found")
        }
    }
}