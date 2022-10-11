use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// File path for the JSON quotes file
    #[arg(short, long = "file", value_name = "FILE")]
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Quote {
    quote: String,
    author: String,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AllQuotes {
    quotes: Vec<Quote>,
}

impl Quote {
    pub fn print_quote(&self) -> String {
        if self.author != "" {
            return format!("{} - {}", self.quote, self.author);
        }
        format!("{}", self.quote)
    }
}

fn print_quote_json(contents: &str) {
    let all_quotes: AllQuotes = serde_json::from_str(contents).unwrap();

    let quotes = all_quotes.quotes;
    // ToDo: add options to search tags
    // for i in quotes {
    //     if i.tags.contains(&"Motivational".to_owned()) {
    //         println!("{:?}", i.quote);
    //     }
    // }
    let mut rng = rand::thread_rng();
    println!("{:?}", quotes[rng.gen_range(0..quotes.len())].print_quote());

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // print_quote(&contents);
    print_quote_json(&contents);
    
    Ok(())
}
