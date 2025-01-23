// Project : Check if the user accounts were leaked.

use std::path::PathBuf;

mod account;
mod error;
mod hibp;
mod scanner;

use clap::{Args, Error, Parser, Subcommand};

// parser clap definition
#[derive(Parser)]
#[clap(version, author, about)]
struct AppArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check duplicate passwords from command line
    Group(GroupArgs),

    /// Check if a port is open
    Ping {
        /// Host name or IP address
        host: String,
        /// Port number
        port: u16,
    },
}

#[derive(Args)]
struct GroupArgs {
    #[clap(required = false)]
    /// Account to check
    account: Vec<account::Account>,
    #[clap(short, long)]
    /// Load passwords from a file
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = AppArgs::parse();
    // we can write a login:password in command line or load it from a file
    match args.command {
        Command::Group(args) => {
            if !args.account.is_empty() {
                let mut grouped_accounts = account::Account::group(&args.account); // en utilisant une référence ici (après avoir bien définit la signature de la fonctioin group) le vecteur accounts initial n'a pas été consommé, juste référencé, ce qui signifie qu'on pourra faire des opérations complémentaires sur les comptes originels
                grouped_accounts.retain(|_, v| v.len() > 1);

                for (pass, log) in grouped_accounts {
                    let logins = log
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("The password {} is used by: {}", pass, logins);
                }
            } else if let Some(file) = args.file.as_deref() {
                let accounts = account::Account::from_file(file);
                match accounts {
                    Ok(account_vec) => {
                        match hibp::check_accounts(&account_vec) {
                            Ok(result) => {
                                println!("{:#?}", result)
                            }
                            Err(e) => {
                                println!("error check accounts: {}", e)
                            }
                        };
                    }
                    Err(error) => match error {
                        error::Error::IoError(io_error) => {
                            eprintln!("I/O Error occurred: {}", io_error);
                        }
                        error::Error::NoColon => {
                            eprintln!("Error: A line in the file is missing a colon (':')");
                        }
                        error::Error::ParseIntError(parse_error) => {
                            eprintln!("Parsing error: {}", parse_error);
                        }
                        error::Error::ReqwestError(req_error) => {
                            eprintln!("Request error: {}", req_error);
                        }
                    },
                }
            } else {
                println!("you have to right an argument !!!!")
            }
        }
        Command::Ping { host, port } => {
            let is_open = scanner::net::tcp_ping(host.as_str(), port).await;
            let address = format!("{}:{}", host, port);
            if is_open {
                println!("\n    {} is open 😁\n", &address);
            } else {
                println!("\n    {} is closed 😤\n", &address);
            }
        }
    }

    Ok(())
}
