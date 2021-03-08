use std::{io::{BufRead, Write, stdout}, process::exit};

#[macro_use]
extern crate crossterm;

use ascii::IntoAsciiString;
use battlefield_rcon::rcon::{RconClient, RconError, RconQueryable, RconResult};
use clap::{Arg, SubCommand};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use dotenv::{dotenv, var};

#[allow(clippy::or_fun_call)]
#[tokio::main]
async fn main() -> RconResult<()> {
    dotenv().ok(); // load (additional) environment variables from `.env` file in working directory.

    let ip = var("BFOX_RCON_IP").unwrap_or("127.0.0.1".into());
    let port = var("BFOX_RCON_PORT")
        .unwrap_or("47200".into())
        .parse::<u16>()
        .unwrap();
    let password = var("BFOX_RCON_PASSWORD").unwrap_or("smurf".into());

    let matches = clap::App::new("rcon-cli")
        .version("0.1")
        .about("Extremely simple and BF4-specifics-unaware library to send and receive strings.")
        .author("Kiiya (snoewflaek@gmail.com)")
        .subcommand(SubCommand::with_name("query")
            .about("Send single query and print result, instead of going into interactive mode")
            .arg(Arg::with_name("query-words").min_values(1))
        )
        .get_matches();

    // println!("Connecting to {}:{} with password ***...", ip, port);
    let rcon = match RconClient::connect((ip.as_str(), port, password.as_str())).await {
        Ok(rcon) => rcon,
        Err(err) => {
            println!("Failed to connect to Rcon at {}:{} with password ***: {:?}", ip, port, err);
            exit(-1);
        }
    };
    // let bf4 = Bf4Client::new(rcon).await.unwrap();
    // println!("Connected!");

    // if user provided "query" subcommand, just do that. Otherwise, go into interactive mode.
    if let Some(singlequery) = matches.subcommand_matches("query") {
        let words = singlequery.values_of("query-words").unwrap().collect::<Vec<_>>();
        handle_input_line(words, &rcon).await?;
    } else {
        print!("-> ");
        std::io::stdout().flush()?;
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            let words = line.split(" ");
            handle_input_line(words, &rcon).await?;
            print!("-> ");
            std::io::stdout().flush()?;
        }
    }

    Ok(())
}

async fn handle_input_line(words: impl IntoIterator<Item = &str>, rcon: &RconClient) -> RconResult<()> {
    let mut words_ascii = Vec::new();
    for word in words {
        words_ascii.push(word.into_ascii_string()?);
    }
    let result = rcon.query(&words_ascii,
        |ok| Ok(ok.to_owned()),
        |err| Some(RconError::other(err)),
    ).await;
    match result {
        Ok(ok) => {
            let mut str = String::new();
            for word in ok {
                str.push(' ');
                str.push_str(word.as_str());
            }
            execute!(
                stdout(),
                SetForegroundColor(Color::Black),
                SetBackgroundColor(Color::Green),
                Print("<- OK".to_string()),
                SetForegroundColor(Color::Green),
                SetBackgroundColor(Color::Reset),
                Print(str),
                ResetColor,
                Print("\n".to_string())
            ).unwrap();
        }
        Err(err) => {
            execute!(
                stdout(),
                SetForegroundColor(Color::Black),
                SetBackgroundColor(Color::Red),
            ).unwrap();

            match err {
                RconError::Other(str) => {
                    // println!("{}", str.on_dark_red());
                    execute!(
                        stdout(),
                        Print("<- Error".to_string()),
                        SetForegroundColor(Color::Red),
                        SetBackgroundColor(Color::Reset),
                        Print(" ".to_string()),
                        Print(str)
                    ).unwrap();
                },
                RconError::ConnectionClosed => {
                    print_error_type("Connection Closed").unwrap();
                },
                RconError::InvalidArguments {our_query: _} => {
                    print_error_type("Invalid Arguments").unwrap();
                },
                RconError::UnknownCommand {our_query: _} => {
                    print_error_type("Unknown Command").unwrap();
                },
                _ => panic!("Unexpected error: {:?}", err),
            };
            execute!(
                stdout(),
                ResetColor,
                Print("\n".to_string())
            ).unwrap();
        }
    }

    Ok(())
}

fn print_error_type(typ: &str) -> Result<(), crossterm::ErrorKind> {
    execute!(
        stdout(),
        SetBackgroundColor(Color::DarkRed),
        Print("<- ".to_string()),
        Print(typ),
    )
}
