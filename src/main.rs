use clap::{Parser, Subcommand};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::env;

#[derive(Parser)]
#[clap(name = "Devi", version = "1.0", author = "by WMouton", about = "Manages API tokens with expiration dates")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Update the token and expiration date")]
    Update {
        #[clap(subcommand)]
        update_command: Option<UpdateCommands>,
    },
    #[clap(about = "Displays the stored token and expiration date")]
    Token,
    #[clap(about = "Deletes the stored token and expiration date")]
    Delete {
        #[clap(subcommand)]
        delete_command: Option<DeleteCommands>,
    },
    #[clap(about = "Displays the version of Devi")]
    Version,
    #[clap(about = "Displays the author information of Devi")]
    Author,
}

#[derive(Subcommand)]
enum UpdateCommands {
    #[clap(about = "Prompts the user to update the token and expiration date")]
    Token,
}

#[derive(Subcommand)]
enum DeleteCommands {
    #[clap(about = "Deletes the stored token and expiration date")]
    Token,
}

const FILE_NAME: &str = "temp_devi_token.txt";

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let home_dir = env::var("HOME").expect("HOME environment variable is not set");
    let file_path = format!("{}/{}", home_dir, FILE_NAME);

    match args.command {
        Some(Commands::Update { update_command }) => match update_command {
            Some(UpdateCommands::Token) => update_token(&file_path)?,
            None => println!("Please provide a valid update command (token)."),
        },
        Some(Commands::Token) => display_token(&file_path)?,
        Some(Commands::Delete { delete_command }) => match delete_command {
            Some(DeleteCommands::Token) => delete_token(&file_path)?,
            None => println!("Please provide a valid delete command (token)."),
        },
        Some(Commands::Version) => display_version(),
        Some(Commands::Author) => display_author(),
        None => println!("Please provide a valid command (update token, token, delete token, version, author or help)."),
    }

    Ok(())
}

fn update_token(file_path: &str) -> io::Result<()> {
    let expiration_date = prompt_user("Please enter the expiration date (e.g., 1 January 2024) or 'never' for no expiration:")?;
    let token = prompt_user("Please enter the token:")?;

    let content = format!("Expiration Date: {}\nToken: {}", expiration_date, token);
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    println!("Token and expiration date updated. Run 'devi token' to display your token.");
    Ok(())
}

fn display_token(file_path: &str) -> io::Result<()> {
    if Path::new(file_path).exists() {
        let contents = fs::read_to_string(file_path)?;
        println!("Stored Token Information:\n{}", contents);
    } else {
        println!("No existing tokens. Please run 'devi update token'.");
    }
    Ok(())
}

fn delete_token(file_path: &str) -> io::Result<()> {
    if Path::new(file_path).exists() {
        fs::remove_file(file_path)?;
        println!("Token file deleted successfully.");
    } else {
        println!("No token file found to delete.");
    }
    Ok(())
}

fn display_version() {
    println!("Devi version 1.0");
}

fn display_author() {
    println!("Author: WMouton");
    println!("GitHub: https://github.com/wmouton");
}

fn prompt_user(prompt: &str) -> io::Result<String> {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
