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
    
    /// Tag for quotes you'd like to see
    #[arg(short, long = "tag", value_name = "TAG")]
    pub tag: Option<String>,
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
        if self.quote.is_empty() {
            return "No matches for your tag".to_string();
        }
        if !self.author.is_empty() {
            return format!("{} - {}", self.quote, self.author);
        }
        self.quote.to_string()
    }
}

// ToDo: handle empty quotes vector (unknown tag)
fn get_quote(contents: &str, tag: &Option<String>) {
    let all_quotes: AllQuotes = serde_json::from_str(contents).unwrap();

    let quotes: Vec<Quote> = all_quotes.quotes;
    let mut final_quotes: Vec<&Quote> = vec![];
    let mut rng = rand::thread_rng();
    
    if tag.is_some() {
        let tag: &String = tag.as_ref().unwrap();

        for quote in quotes.iter() {
            for t in &quote.tags {
                if t.contains(tag) {
                    final_quotes.push(quote);
                }
            }
        }
        println!("{:?}", final_quotes[rng.gen_range(0..final_quotes.len())].print_quote());
    } else {
        println!("{:?}", quotes[rng.gen_range(0..quotes.len())].print_quote());
    }
    
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;
    
    get_quote(&contents, &config.tag);
    
    Ok(())
}
