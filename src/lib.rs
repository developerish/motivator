use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display, fs};

const PACKAGED_QUOTES_FILE_PATH: &str = "quotes.json";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
/// Struct to define command line parameters
pub struct Config {
    /// File path for the JSON quotes file
    #[arg(short, long = "file", value_name = "FILE")]
    pub file_path: Option<String>,

    /// Pick a random quote from the Tag you'd like to see
    #[arg(short, long = "tag", value_name = "TAG")]
    pub tag: Option<String>,

    /// Show all quotes
    #[arg(short, long = "all", value_name = "ALL")]
    pub show_all: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
/// Struct to define the body of a quote
pub struct Quote {
    quote: String,
    author: String,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
/// Struct to hold all quotes read from the input file
struct AllQuotes {
    quotes: Vec<Quote>,
}

impl Display for Quote {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.quote.is_empty() {
            return write!(fmt, "<Quote missing.>");
        }

        if self.author.is_empty() {
            write!(fmt, "{}", self.quote)
        } else {
            write!(fmt, "{} - {}", self.quote, self.author)
        }
    }
}

/// Return a vector of quotes
fn get_quotes(
    file_path: String,
    tag: String,
    show_all: bool,
) -> Result<Vec<Quote>, Box<dyn Error>> {
    let contents: String = fs::read_to_string(file_path)?;
    let all_quotes: AllQuotes = serde_json::from_str(&contents)?;

    let quotes: Vec<Quote> = all_quotes.quotes;

    if show_all {
        return Ok(quotes);
    }

    let mut final_quotes: Vec<Quote> = vec![];

    if !tag.is_empty() {
        let user_tag: &String = &tag.to_lowercase();

        quotes.iter().for_each(|quote| {
            quote.tags.iter().for_each(|t| {
                if t.to_lowercase().eq(user_tag) {
                    final_quotes.push(quote.clone());
                }
            });
        });
        Ok(final_quotes)
    } else {
        Ok(quotes)
    }
}

/// Print individual quote(s) from the Quote vector
fn print_quotes(quotes: Vec<Quote>, show_all_quotes: bool) {
    let mut rng = rand::thread_rng();

    if quotes.is_empty() {
        println!("Selected Tag returned no matching quotes.");
    } else if show_all_quotes {
        quotes.iter().for_each(|q| println!("{}\n", q));
    } else {
        // println!("{}", quotes[rng.gen_range(0..quotes.len())].get_quote());
        println!("{}", quotes[rng.gen_range(0..quotes.len())]);
    }
}

fn get_file_name(f: Option<String>) -> String {
    match f {
        Some(f) => f,
        _ => {
            println!("--------");
            println!("<Quotes file not provided. Showing a random quote from buil-in file.>");
            println!("run 'motivator -h' for more options.");
            println!("--------\n");
            return String::from(PACKAGED_QUOTES_FILE_PATH);
        }
    }
}

fn get_tag(t: Option<String>) -> String {
    match t {
        Some(t) => t,
        _ => String::from(""),
    }
}

/// Take the config and call relevant functions to print the quote(s)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let quotes_file = get_file_name(config.file_path);
    let tag = get_tag(config.tag);
    let show_all_quotes = config.show_all;

    let quotes = get_quotes(quotes_file, tag, show_all_quotes).unwrap_or_else(|e| {
        eprintln!("Invalid JSON file.\nError: {}", e);
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

        assert_eq!(quote.to_string(), printed_json);
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

        assert_eq!(quote.to_string(), printed_json);
    }

    #[test]
    fn get_default_file() {
        assert_eq!(get_file_name(None), String::from(PACKAGED_QUOTES_FILE_PATH));
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Config::command().debug_assert()
    }
}
