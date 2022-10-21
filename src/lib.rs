use assert_json::assert_json;
use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
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

fn get_quotes(contents: &str, tag: &Option<String>) -> Vec<Quote> {
    let all_quotes: AllQuotes = serde_json::from_str(contents).unwrap();

    let quotes: Vec<Quote> = all_quotes.quotes;
    let mut final_quotes: Vec<Quote> = vec![];

    if tag.is_some() {
        let tag: &String = tag.as_ref().unwrap();

        for quote in quotes.iter() {
            for t in &quote.tags {
                if t.contains(tag) {
                    final_quotes.push(quote.clone());
                }
            }
        }
        final_quotes
    } else {
        quotes
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;
    let mut rng = rand::thread_rng();
    
    let quotes: Vec<Quote> = get_quotes(&contents, &config.tag);

    if quotes.is_empty() {
        println!("Selected Tag returned no matching quotes");
    } else {
        println!("{:?}", quotes[rng.gen_range(0..quotes.len())].print_quote());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_json_format() {
        let correct_json = r#"
                {
                    "quote": "Mindset is everything",
                    "author": "unknown",
                    "tags": ["Motivational", "Positivity"]
                }
            "#;

        let quote = "Mindset is everything";
        let author = "unknown";

        assert_json!(correct_json, {
            "quote": quote,
            "author": author,
            "tags": [
                "Motivational",
                "Positivity"
            ]
        });
    }

    #[test]
    fn print_json() {
        let correct_json = r#"
                {
                    "quote": "Mindset is everything",
                    "author": "unknown",
                    "tags": ["Motivational", "Positivity"]
                }
            "#;

        let quote: Quote = serde_json::from_str(correct_json).unwrap();
        let printed_json = "Mindset is everything - unknown";

        assert_eq!(quote.print_quote(), printed_json);
    }

    #[test]
    fn check_quote_intake() {
        let correct_json = r#"
                {
                  "quotes": [
                    {
                      "quote": "Mindset is everything",
                      "author": "",
                      "tags": ["Motivational", "Positivity"]
                    },
                    {
                      "quote": "You will never reach your destination if you stop and throw stones at every dog that barks",
                      "author": "Winston Churchill",
                      "tags": ["Stoic"]
                    }
                  ]
                }
            "#;

        let tags = &Some(String::from("Motivational"));
        let quote = get_quotes(correct_json, tags);
        let printed_json = "Mindset is everything";

        assert_eq!(printed_json, quote[0].print_quote());
    }
}