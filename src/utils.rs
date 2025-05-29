use rand::Rng;
use rand::prelude::IndexedRandom;
use serde_json::Value;

const ADJECTIVES: &str = include_str!(".././dict/adjectives.txt");
const NOUNS: &str = include_str!(".././dict/nouns.txt");
const VERBS: &str = include_str!(".././dict/verbs.txt");

fn words_from(s: &'static str) -> Vec<&'static str> {
    s.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}

/// Retorna un string `adjective-noun`
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

/// Genera un dominio `.surge.sh`
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
    use crate::{generate_domain, json_to_argv};
    use regex::Regex;

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
            vec![
                "dist/",
                "--endpoint",
                "https://surge.surge.sh",
                "--stage",
                "false"
            ]
        );
    }

    #[test]
    #[should_panic(expected = "Invalid JSON")]
    fn test_json_to_argv_invalid_json() {
        let json = r#"{ invalid: json }"#;
        json_to_argv(json);
    }
}
