# Motivator

Indulge in some motivation++ via random quote(s) from a JSON file.

## Quick Start

### Download and install from crates.io

`❯ cargo install motivator`

### Install from source

`❯ cargo build --release`

`❯ ./target/release/motivator -f <JSON filename>`

### Example usage:

```
❯ motivator -f
--------
<Quotes file not provided. Showing a random quote from buil-in file.>
run 'motivator -h' for more options.
--------

A smooth sea never made a skillful mariner

❯ motivator -f quotes.json
Whether you think you can or think you can't, you're right. - Henry Ford

❯ motivator -f quotes.json -t stoic
Wants make you a servant - Seneca
```
For a full list of command-line options:

```
❯ motivator -h
Indulge in some motivation++ via random quote(s).

Usage: motivator [OPTIONS]

Options:
  -f, --file <FILE>  File path for the JSON quotes file
  -t, --tag <TAG>    Pick a random quote from the Tag you'd like to see
  -a, --all          Show all quotes
  -h, --help         Print help information
  -V, --version      Print version information
```

### JSON file format

```
{
"quotes": [
  {
    "quote": "Motivational words",
    "author": "",
    "tags": ["Stoic", "Positivity"]
  }
 ]
}
```

#### Sample quotes file

[https://github.com/developerish/motivator/blob/main/quotes.json](https://github.com/developerish/motivator/blob/main/quotes.json)
