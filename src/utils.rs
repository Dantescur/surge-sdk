/*
  src/utils.rs
*/
//! Module for utility functions in the Surge SDK.
//!
//! This module provides helper functions for generating random domain names and converting JSON
//! objects to command-line arguments, primarily used to support the Surge SDK's operations. It
//! includes utilities for creating memorable, hyphenated identifiers (e.g., `adjective-noun`) and
//! constructing `.surge.sh` domain names with optional random numbers. Additionally, it offers
//! functionality to transform JSON data into a format suitable for command-line arguments, which is
//! useful for interacting with the Surge API's command-line interface conventions.
//!
//! Key functions include:
//! - `choose`: Generates a random two-word identifier combining adjectives, nouns, or verbs.
//! - `generate_domain`: Creates a `.surge.sh` domain name, optionally appending a random number.
//! - `json_to_argv`: Converts a JSON object into a vector of command-line arguments.
//! - `words_from`: A helper function to parse static word lists into trimmed vectors.
//!
//! The module uses word lists (`adjectives.txt`, `nouns.txt`, `verbs.txt`) included at compile time
//! to generate identifiers and relies on the `rand` crate for randomization. It also includes a test
//! suite to verify the correctness of domain name generation and JSON-to-argv conversion.
//!
//! # Example
//! ```
//! use surge_sdk::utils::{generate_domain, json_to_argv};
//!
//! // Generate a domain name
//! let domain = generate_domain(true);
//! println!("Generated domain: {}", domain); // e.g., "happy-cat-1234.surge.sh"
//!
//! // Convert JSON to command-line arguments
//! let json = r#"{"_": ["dist/"], "endpoint": "https://surge.surge.sh", "stage": false}"#;
//! let args = json_to_argv(json);
//! println!("Arguments: {:?}", args); // e.g., ["dist/", "--endpoint", "https://surge.surge.sh", "--stage", "false"]
//! ```

use rand::Rng;
use rand::prelude::IndexedRandom;
use serde_json::Value;

const ADJECTIVES: &str = include_str!(".././dict/adjectives.txt");
const NOUNS: &str = include_str!(".././dict/nouns.txt");
const VERBS: &str = include_str!(".././dict/verbs.txt");

/// Converts a static string of words into a vector of trimmed, non-empty lines.
///
/// # Arguments
/// * `s` - A static string containing words, one per line.
///
/// # Returns
/// A vector of trimmed, non-empty words.
fn words_from(s: &'static str) -> Vec<&'static str> {
    s.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}

/// Generates a random two-word identifier (e.g., `adjective-noun`, `verb-noun`, or `adjective-verb`).
///
/// # Returns
/// A hyphenated string combining two words.
pub fn choose() -> String {
    let adjectives = words_from(ADJECTIVES);
    let nouns = words_from(NOUNS);
    let verbs = words_from(VERBS);

    let mut rng = rand::rng();

    match rng.random_range(0..3) {
        0 => {
            let adj = adjectives.choose(&mut rng).expect("No adjectives");
            let noun = nouns.choose(&mut rng).expect("No nouns");

            format!("{}-{}", adj, noun)
        }
        1 => {
            // verb-noun
            let verb = verbs.choose(&mut rng).expect("No verbs");
            let noun = nouns.choose(&mut rng).expect("No nouns");
            format!("{}-{}", verb, noun)
        }
        _ => {
            // adjective-verb (or any other combination you prefer)
            let adj = adjectives.choose(&mut rng).expect("No adjectives");
            let verb = verbs.choose(&mut rng).expect("No verbs");
            format!("{}-{}", adj, verb)
        }
    }
}

/// Generates a `.surge.sh` domain name, optionally with a random number.
///
/// # Arguments
/// * `with_number` - Whether to append a random number (0-9999).
///
/// # Returns
/// A domain name string (e.g., `happy-cat.surge.sh` or `happy-cat-1234.surge.sh`).
pub fn generate_domain(with_number: bool) -> String {
    let base = choose();
    let mut rng = rand::rng();
    if with_number {
        let num: u16 = rng.random_range(0..=9999);
        format!("{base}-{num}.surge.sh")
    } else {
        format!("{base}.surge.sh")
    }
}

/// Converts a JSON object to a vector of command-line arguments.
///
/// # Arguments
/// * `json` - A JSON string representing arguments.
///
/// # Returns
/// A vector of strings representing command-line arguments.
///
/// # Panics
/// Panics if the JSON is invalid.
pub fn json_to_argv(json: &str) -> Vec<String> {
    let mut args = Vec::new();
    let parsed: Value = serde_json::from_str(json).expect("Invalid JSON");

    if let Some(positional) = parsed.get("_").and_then(|v| v.as_array()) {
        for val in positional {
            if let Some(s) = val.as_str() {
                args.push(s.to_string());
            }
        }
    }

    for (key, value) in parsed.as_object().unwrap() {
        if key == "_" {
            continue; // ya lo manejamos
        }

        args.push(format!("--{}", key));

        match value {
            Value::String(s) => args.push(s.to_string()),
            Value::Bool(b) => args.push(b.to_string()),
            Value::Number(n) => args.push(n.to_string()),
            _ => args.push(value.to_string()), // fallback
        }
    }

    args
}

#[cfg(test)]
mod tests {
    use crate::{SURGE_API, generate_domain, json_to_argv};
    use regex::Regex;

    /// Tests generating a domain without a number.
    #[test]
    fn test_generate_domain_without_number() {
        let domain = generate_domain(false);
        let re = Regex::new(r"^[a-z]+-[a-z]+\.surge\.sh$").unwrap();
        assert!(
            re.is_match(&domain),
            "Domain {} does not match pattern",
            domain
        );
    }

    /// Tests generating a domain with a number.
    #[test]
    fn test_generate_domain_with_number() {
        let domain = generate_domain(true);
        let re = Regex::new(r"^[a-z]+-[a-z]+-[0-9]{1,4}\.surge\.sh$").unwrap();
        assert!(
            re.is_match(&domain),
            "Domain {} does not match pattern",
            domain
        );
    }

    /// Tests converting JSON to command-line arguments.
    #[test]
    fn test_json_to_argv() {
        let json = r#"{
            "_": ["dist/"],
            "endpoint": "https://surge.surge.sh",
            "stage": false
        }"#;
        let args = json_to_argv(json);
        assert_eq!(
            args,
            vec!["dist/", "--endpoint", SURGE_API, "--stage", "false"]
        );
    }

    /// Tests that invalid JSON panics.
    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test_json_to_argv_invalid_json() {
        let json = r#"{ invalid: json }"#;
        json_to_argv(json);
    }
}
