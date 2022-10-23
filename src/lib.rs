use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// File path for the JSON quotes file
    #[arg(short, long = "file", value_name = "FILE")]
    pub file_path: String,

    /// Pick a random quote from the Tag you'd like to see
    #[arg(short, long = "tag", value_name = "TAG")]
    pub tag: Option<String>,

    /// Show all quotes
    #[arg(short, long = "all", value_name = "ALL")]
    pub all: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Quote {
    quote: String,
    author: String,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AllQuotes {
    quotes: Vec<Quote>,
}

impl Quote {
    pub fn get_quote(&self) -> String {
        if self.quote.is_empty() {
            return "<Quote missing.>".to_string();
        }
        
        if self.author.is_empty() {
            self.quote.to_string()
        } else {
            format!("{} - {}", self.quote, self.author)
        }
    }
}

fn get_quotes(config: Config) -> Result<Vec<Quote>, Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;
    let all_quotes: AllQuotes = serde_json::from_str(&contents)?;

    let quotes: Vec<Quote> = all_quotes.quotes;

    if config.all {
        return Ok(quotes);
    }
    
    let mut final_quotes: Vec<Quote> = vec![];

    if config.tag.is_some() {
        let tag: &String = &config.tag.as_ref().unwrap().to_lowercase();

        for quote in quotes.iter() {
            for t in &quote.tags {
                if t.to_lowercase().eq(tag) {
                    final_quotes.push(quote.clone());
                }
            }
        }
        Ok(final_quotes)
    } else {
        Ok(quotes)
    }
}

pub fn print_quotes(quotes: Vec<Quote>, show_all_quotes: bool) {
    let mut rng = rand::thread_rng();
    
    if quotes.is_empty() {
        println!(
            "Selected Tag returned no matching quotes." 
        );
    } else if show_all_quotes{
        for quote in quotes {
            println!("{}", quote.get_quote());
            println!("\n");
        }
    } else {
        println!("{}", quotes[rng.gen_range(0..quotes.len())].get_quote());
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let show_all_quotes = config.all;
    let quotes = get_quotes(config).unwrap_or_else(|e| {
        eprintln!(
            "Invalid JSON file.\nError: {}", e
        );
        std::process::exit(1);
    });

    print_quotes(quotes, show_all_quotes);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_quote_from_valid_json() {
        let correct_json = r#"
                {
                    "quote": "Mindset is everything",
                    "author": "unknown",
                    "tags": ["Motivational", "Positivity"]
                }
            "#;

        let quote: Quote = serde_json::from_str(correct_json).unwrap();
        let printed_json = "Mindset is everything - unknown";

        assert_eq!(quote.get_quote(), printed_json);
    }

    #[test]
    fn alert_empty_quote() {
        let correct_json = r#"
                {
                    "quote": "",
                    "author": "",
                    "tags": [""]
                }
            "#;

        let quote: Quote = serde_json::from_str(correct_json).unwrap();
        let printed_json = "<Quote missing.>";

        assert_eq!(quote.get_quote(), printed_json);
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Config::command().debug_assert()
    }
}
