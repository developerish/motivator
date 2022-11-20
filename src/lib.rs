pub mod quotes;

use crate::quotes::{AllQuotes, Quote};
use clap::Parser;
use rand::Rng;
use std::{error::Error, fs};

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

    /// Suppress default message
    #[arg(short, long = "quiet", value_name = "QUIET")]
    pub quiet_output: bool,
}

/// Return a vector of quotes
fn get_quotes(
    file_path: String,
    tag: String,
    show_all: bool,
) -> Result<Vec<Quote>, Box<dyn Error>> {
    // file_path is either a user provided file or a built-in qupte from get_file_name method
    let contents: String = fs::read_to_string(&file_path).unwrap_or(file_path.to_owned());
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
fn print_quotes(quotes: Vec<Quote>, config: &Config) {
    let mut rng = rand::thread_rng();

    if config.file_path.is_none() && !config.quiet_output {
        println!("--------");
        println!("<Quotes file not provided. Showing a random quote from built-in quotes.>");
        println!("run 'motivator -h' for more options.");
        println!("--------\n");
    }

    if quotes.is_empty() {
        println!("Selected Tag returned no matching quotes.");
    } else if config.show_all {
        quotes.iter().for_each(|q| println!("{}\n", q));
    } else {
        // println!("{}", quotes[rng.gen_range(0..quotes.len())].get_quote());
        println!("{}", quotes[rng.gen_range(0..quotes.len())]);
    }
}

fn get_file_name(f: &Option<String>) -> String {
    match f {
        Some(f) => f.to_string(),
        _ => {
            return Quote::built_in_quotes();
        }
    }
}

/// Take the config and call relevant functions to print the quote(s)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let quotes_file = get_file_name(&config.file_path);
    let tag = match &config.tag {
        Some(tag) => tag.to_string(),
        _ => String::from(""),
    };
    let show_all_quotes = config.show_all;

    let quotes = get_quotes(quotes_file, tag, show_all_quotes).unwrap_or_else(|e| {
        eprintln!("Invalid JSON file.\nError: {}", e);
        std::process::exit(1);
    });

    print_quotes(quotes, &config);
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
    fn verify_cli() {
        use clap::CommandFactory;
        Config::command().debug_assert()
    }
}
