use std::io::{self, Write};
use clap::{command, Command};
use std::fs;

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
            fs::remove_dir_all(&path).unwrap();
            println!("Deleted ~/.harflex directory");
            std::process::exit(0);
        }

        println!("No ~/.harflex directory found");
        std::process::exit(1);
    }

    match dirs::home_dir() {
        Some(mut path) => {
            path.push(HARFLEX_DIR);

            if !path.exists() &&! path.is_dir() {
                init();
            }
        }
        None => {
            eprintln!("Unable to determine the user's home directory.");
            std::process::exit(1);
        }
    }
}

fn init() {
    println!("Initial setup is required.");
    println!("Do you want to continue? (y/n): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");

    let response = response.trim().to_lowercase();

    if response != "y" && response != "yes" {
        println!("Setup aborted. Exiting...");
        std::process::exit(0);
    }

    println!("Please visit https://id.getharvest.com/developers and create a new personal access token.");

    print!("Insert token here: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut access_token = String::new();
    io::stdin().read_line(&mut access_token).expect("Failed to read line");

    let access_token = access_token.trim();

    let mut path = dirs::home_dir().unwrap();
    path.push(HARFLEX_DIR);

    fs::create_dir_all(&path).unwrap();
    let token_file_path = path.join("access_token");
    fs::write(&token_file_path, access_token).unwrap();

    println!("Access token saved to file: {:?}", token_file_path);
}