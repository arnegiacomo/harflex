use std::io::{self, Write};
use clap::{command, Command};

static HARFLEX_DIR: &str = ".harflex";

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("reset")
                .about("deletes ~/.harflex directory")
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("reset") {
        let mut path = dirs::home_dir().unwrap();
        path.push(HARFLEX_DIR);

        if path.exists() && path.is_dir() {
            std::fs::remove_dir_all(&path).unwrap();
            println!("Deleted ~/.harflex directory");
            std::process::exit(0);
        }
    }

    match dirs::home_dir() {
        Some(mut path) => {
            path.push(HARFLEX_DIR);

            if !path.exists() &&! path.is_dir() {
                setup();
            }
        }
        None => {
            eprintln!("Unable to determine the user's home directory.");
            std::process::exit(1);
        }
    }
}

fn setup() {
    let mut path = dirs::home_dir().unwrap();
    path.push(HARFLEX_DIR);

    std::fs::create_dir_all(&path).unwrap();
    println!("Please visit https://id.getharvest.com/developers and create a new personal access token.");

    print!("Insert token here: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut access_token = String::new();
    io::stdin().read_line(&mut access_token).expect("Failed to read line");

    let access_token = access_token.trim();

    println!("You entered access token: {}", access_token);
}