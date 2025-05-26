/*
  src/main.rs
*/
use std::error::Error;

/*
  main.rs
*/
// use futures_util::StreamExt;
// use serde_json::Value;
use futures_util::StreamExt;
use serde_json::Value;
use surge_sdk::{Auth, Config, SurgeClient, print_domain_list};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config = Config::new("https://surge.surge.sh", "0.6.4");
    let client = SurgeClient::new(config);

    // let auth = Auth::UserPass {
    //     username: "testermyhost@gmail.com".to_string(),
    //     password: "Kilo2022*".to_string(),
    // };

    let auth = Auth::UserPass {
        username: "polandcuban2@gmail.com".to_string(),
        password: "Kilo2025*".to_string(),
    };

    let token = client.login(auth).await?;

    // let list = client.list(Auth::Token(token.token)).await?;
    // print_domain_list(&list);

    // Publish a project
    let json = r#"
    {
        "_": ["dist/"],
        "e": "https://surge.surge.sh",
        "endpoint": "https://surge.surge.sh",
        "s": false,
        "stage": false
    }
    "#;

    let argv = json_to_argv(json);
    let events = client
        .publish(
            std::path::Path::new("./dist"),
            "marginal-toss.surge.sh",
            Auth::Token(token.token),
            None,
            Some(&argv),
        )
        .await?;

    tokio::pin!(events);
    while let Some(event) = events.next().await {
        match event {
            Ok(e) => println!("Event: {:?}", e),
            Err(e) => {
                println!("Error: {:?}", e);
                // Add debug output here
                if let Some(raw) = e.source() {
                    println!("Raw response: {:?}", raw);
                }
            }
        }
    }
    Ok(())
}

fn json_to_argv(json: &str) -> Vec<String> {
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
