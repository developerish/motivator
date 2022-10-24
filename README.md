# Motivator

Indulge in some motivation++ via random quote(s) from a JSON file.

## Quick Start

### Download and install from crates.io

`cargo install motivator`

### Install from source

`cargo build --release`

`./target/release/motivator -f <JSON filename>`

### Example usage:
```
❯ motivator -f quotes.json
Whether you think you can or think you can't, you're right. - Henry Ford
```
For a full list of command-line options:

```
❯ motivator -h
Indulge in some motivation++ via random quote(s) from a JSON file.

Usage: motivator [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>  File path for the JSON quotes file
  -t, --tag <TAG>    Pick a random quote from the Tag you'd like to see
  -a, --all          Show all quotes
  -h, --help         Print help information
  -V, --version      Print version information
  ```
