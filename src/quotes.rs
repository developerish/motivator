use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
/// Struct to define the body of a quote
pub struct Quote {
    pub quote: String,
    pub author: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
/// Struct to hold all quotes read from the input file
pub struct AllQuotes {
    pub quotes: Vec<Quote>,
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

impl Quote {
    pub fn built_in_quotes() -> String {
        r#"{
          "quotes": [
            {
              "quote": "Mindset is everything",
              "author": "",
              "tags": [
                "Motivational",
                "Positivity"
              ]
            },
            {
              "quote": "If you want people to do something, make it easy",
              "author": "Richard Thaler",
              "tags": [
                "Motivational"
              ]
            },
            {
              "quote": "You will never reach your destination if you stop and throw stones at every dog that barks",
              "author": "Winston Churchill",
              "tags": [
                "Stoic"
              ]
            },
            {
              "quote": "Comparison is the thief of joy",
              "author": "Theodore Roosevelt",
              "tags": []
            },
            {
              "quote": "I've missed more than 9000 shots in my career. I've lost almost 300 games. 26 times, I've been trusted to take the game winning shot and missed. I've failed over and over and over again in my life. And that is why I succeed.",
              "author": "Michael Jordan",
              "tags": []
            },
            {
              "quote": "I can give you a six-word formula for success: Think things through, then follow through.",
              "author": "Captain Edward V. Rickenbacker",
              "tags": []
            },
            {
              "quote": "We are what we repeatedly do. Excellence, therefore, is not an act but a habit.",
              "author": "Aristotle",
              "tags": []
            },
            {
              "quote": "Do, or do not, there is no try",
              "author": "Yoda",
              "tags": []
            },
            {
              "quote": "Whether you think you can or think you can't, you're right.",
              "author": "Henry Ford",
              "tags": []
            },
            {
              "quote": "When it is dark enough, you can see the stars.",
              "author": "Ralph Waldo Emerson",
              "tags": []
            },
            {
              "quote": "Wait for the right opportunity not the next available one",
              "author": "",
              "tags": []
            },
            {
              "quote": "Sometimes you need to slow down to speed up",
              "author": "",
              "tags": []
            },
            {
              "quote": "Great moments are born from great opportunities",
              "author": "",
              "tags": []
            },
            {
              "quote": "If I had an hour to solve a problem I'd spend 55 minutes thinking about the problem and 5 minutes thinking about solutions.",
              "author": "Albert Einstein",
              "tags": []
            },
            {
              "quote": "If you do your best you might get outscored but you will never lose",
              "author": "",
              "tags": []
            },
            {
              "quote": "Be yourself; everyone is already taken",
              "author": "Oscar Wilde",
              "tags": []
            },
            {
              "quote": "Never let a good crisis go to waste",
              "author": "Rahm Emanuel",
              "tags": []
            },
            {
              "quote": "Pain is mandatory, suffering is optional",
              "author": "",
              "tags": []
            },
            {
              "quote": "Our choices today affects our options tomorrow",
              "author": "",
              "tags": []
            },
            {
              "quote": "Between stimulus and response there is a space. In that space is our power to choose our response. In our response lies our growth and our freedom",
              "author": "",
              "tags": []
            },
            {
              "quote": "A smooth sea never made a skillful mariner",
              "author": "",
              "tags": []
            },
            {
              "quote": "A peaceful mind generates power",
              "author": "",
              "tags": [
                "Stoic"
              ]
            },
            {
              "quote": "The happiness of our lives depends upon the quality of our thoughts",
              "author": "Marcus Aurelius",
              "tags": [
                "Stoic"
              ]
            },
            {
              "quote": "Hard choices, easy life. Easy choices, hard life.",
              "author": "Jerzy Gregorek",
              "tags": []
            },
            {
              "quote": "Success in anything is just a byproduct of learning, and learning is a byproduct of curiosity.",
              "author": "",
              "tags": []
            },
            {
              "quote": "Ultimately, if you are curious about something, you will be successful at it, and the more curious you are about it, the more successful you will be at it.",
              "author": "Naval",
              "tags": []
            },
            {
              "quote": "Nullius in verba (take nobody's word for it)",
              "author": "",
              "tags": []
            },
            {
              "quote": "New goals don't deliver new results. New lifestyles do. And a lifestyle is a process, not an outcome. For this reason, your energy should go into building better habits, not chasing better results.",
              "author": "@JamesClear",
              "tags": []
            },
            {
              "quote": "The plan won't give you the action, the action will give you the plan.",
              "author": "",
              "tags": []
            },
            {
              "quote": "If you always do what you always did, you always get what you always got.",
              "author": "",
              "tags": []
            },
            {
              "quote": "Nothing in life is to be feared, it is only to be understood. Now is the time to understand more, so that we may fear less.",
              "author": "Marie Curie",
              "tags": []
            },
            {
              "quote": "Wants make you a servant",
              "author": "Seneca",
              "tags": [
                "stoic"
              ]
            },
            {
              "quote": "Discomfort with uncertainty causes more anxiety than guaranteed bad news.",
              "author": "",
              "tags": []
            },
            {
              "quote": "The praise from being right becomes the pain from being wrong.",
              "author": "Naval",
              "tags": []
            },
            {
              "quote": "Leisure without study is death - a tomb for the living person.",
              "author": "Seneca",
              "tags": [
                "Stoic"
              ]
            }
          ]
        }"#.to_string()
    }
}
