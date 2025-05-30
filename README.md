# Surge SDK for Rust

<!--toc:start-->

- [Surge SDK for Rust](#surge-sdk-for-rust)
  - [Features](#features)
  - [Installation](#installation)
  - [Getting Started](#getting-started)
  - [API Overview](#api-overview)
  - [Configuration](#configuration)
    - [The Config struct](#the-config-struct)
    - [Error Handling](#error-handling)
    - [Logging](#logging)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgments](#acknowledgments)
  - [Contact](#contact)
  <!--toc:end-->

The surge-sdk is a Rust library for interacting with the
[Surge.sh](https://surge.sh) API,
enabling developers to programmatically manage static site deployments,
domains, SSL certificates, DNS records, and more. Built with asynchronous
Rust using reqwest and tokio, it provides a robust and type-safe
interface for publishing projects, handling authentication, and retrieving analytics.

---

## Features

- Project Publishing: Upload project directories as
  .tar.gz archives to [Surge.sh](https://surge.sh) domains, with support for production
  and work-in-progress (WIP) deployments.
- Domain Management: List, tear down, roll back, roll forward, or switch domain revisions.
- SSL and DNS: Manage SSL certificates, DNS records, and domain zones.
- Account Operations: Fetch account details, log in, update plans, and manage collaborators.
- Streaming Support: Handle real-time NDJSON event streams for publishing and
  encryption operations.
- Error Handling: Unified error handling with detailed error types and logging.
- Random Domain Generation: Generate human-readable .surge.sh domains for
  previews (e.g., happy-cat-1234.surge.sh).

---

## Installation

Add surge-sdk to your project by including it in your Cargo.toml:

```toml
[dependencies]
surge_sdk = "0.1.0"
```

> **Ensure you have Rust and Cargo installed. The library requires Rust 1.65 or
> later due to its use of modern async features.**

---

## Getting Started

- Prerequisites

  - A Surge.sh account with an API token or username/password credentials.
  - A project directory to publish (e.g., a static site).

- Basic Usage

  - Here's a quick example to publish a project to a Surge.sh domain:

```rust
use surge_sdk::{Config, SURGE_API, SurgeSdk, Auth, utils::generate_domain};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), surge_sdk::SurgeError> {
// Initialize the SDK
let config = Config::new(SURGE_API, "0.1.0")?;
let sdk = SurgeSdk::new(config)?;
let auth = Auth::Token("your-api-token".to_string());

    // Publish a project
    let project_path = Path::new("./my-project");
    let domain = generate_domain(false);
    let mut event_stream = sdk.publish(project_path, &domain, &auth, None, None).await?;

    // Process events
    while let Some(event) = event_stream.next().await {
        match event {
            Ok(event) => println!("Event: {}", event),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())

}
```

> This example configures the SDK, authenticates with a token, and publishes a
> project directory to a random generated domain, printing real-time events
> from the NDJSON stream.

- Generating a Preview Domain to publish a work-in-progress version to a unique
  preview domain:

```rust
use surge_sdk::{Config, SURGE_API, SurgeSdk, Auth, generate_domain};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), surge_sdk::SurgeError> {
let config = Config::new(SURGE_API, "0.1.0")?;
let sdk = SurgeSdk::new(config)?;
let auth = Auth::Token("your-api-token".to_string());

    // Generate a random preview domain
    let domain = generate_domain(true); // e.g., "happy-cat-1234.surge.sh"
    let project_path = Path::new("./my-project");

    // Publish WIP
    let mut event_stream = sdk.publish_wip(project_path, &domain, &auth, None, None).await?;
    while let Some(event) = event_stream.next().await {
        println!("Event: {}", event?);
    }

    Ok(())

}
```

- Managing SSL Certificates to upload an SSL certificate for a domain:

```rust
use surge_sdk::{Config, SURGE_API, SurgeSdk, Auth};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), surge_sdk::SurgeError> {
let config = Config::new(SURGE_API, "0.1.0")?;
let sdk = SurgeSdk::new(config)?;
let auth = Auth::Token("your-api-token".to_string());

    let domain = "my-site.surge.sh";
    let pem_path = Path::new("./certificate.pem");

    sdk.ssl(domain, pem_path, &auth).await?;
    println!("SSL certificate uploaded for {}", domain);

    Ok(())

}
```

**\*** Requires a Surge.sh Professional Account.\*\*

---

## API Overview

- The SurgeSdk provides a wide range of methods for interacting with the
  Surge.sh API. Key methods include:

  > **Note**: Features marked with `*` require a **Surge.sh Professional Account**.

  - Publishing:

    - **publish**: Upload a project to a production domain.
    - **publish_wip**: Upload a project to a preview domain.

  - Domain Management:
    - **list**: List all domains or filter by a specific domain.
    - **teardown**: Remove a domain.
    - **rollback**, **rollfore**, **cutover**, **discard**: Manage domain revisions.
  - SSL and DNS:
    - **certs**: Fetch SSL certificate information.
    - **ssl\***: Upload an SSL certificate.
    - **dns\***, **dnsadd\***, **dnsremove\***: Manage DNS records.
    - **zone\***, **zone_add\***, **zone_remove\***: Manage domain zones.
  - Account and Analytics:
    - **account**: Fetch account details.
    - **login**: Authenticate and retrieve a token.
    - **analytics**, **usage**, **audit**: Retrieve domain analytics, usage,
      and audit logs.
    - **plan**, **card**: Update account plan or payment card.
  - Miscellaneous:
    - **bust**: Clear a domain's cache.
    - **invite**, **revoke**: Manage domain collaborators.
    - **encrypt\***: Request SSL encryption for a domain (returns an NDJSON stream).

> See the API documentation [https://docs.rs/surge-sdk](https://docs.rs/surge-sdk)
> for detailed method signatures and parameters.

---

## Configuration

### The Config struct

The SDK provides a default API URL for convenience:

```rust
use surge_sdk::{Config, SURGE_API};

// Recommended: Use the built-in constant
let config = Config::new(SURGE_API, "0.1.0")?;
/*
* Note: The SURGE_API constant points to https://surge.surge.sh. Override it only
* if you need a custom endpoint (e.g., for testing).
*/
```

Usage:

```rust
use surge_sdk::{Config, SurgeSdk, SURGE_API};

let config = Config::new(SURGE_API, "0.1.0")?
.with_timeout(60) // Set timeout to 60 seconds
.with_insecure(true); // Allow invalid SSL certificates (for testing)
let sdk = SurgeSdk::new(config)?;
```

### Error Handling

The SDK uses a unified _SurgeError_ enum to handle errors, including HTTP, API,
JSON, I/O, and event-related issues.

Example:

```rust
match sdk.publish(projectpath, domain, &auth, None, None).await {
Ok(stream) => { /\* process stream \_/ },
Err(surge_sdk::SurgeError::Api(api_err)) => eprintln!("API error: {:?}", api_err.errors),
Err(e) => eprintln!("Error: {}", e),
}
```

### Logging

The SDK uses the _log_ crate for detailed logging. Configure a logger
(e.g., env_logger) to see debug output:

```rust
use env_logger;

env_logger::init(); // Enable logging with RUST_LOG=debug
```

---

## Contributing

Contributions are welcome! To contribute:

- Fork the repository.
- Create a feature branch (git checkout -b feature/my-feature).
- Commit your changes (git commit -am 'Add my feature').
- Push to the branch (git push origin feature/my-feature).
- Open a pull request.

> Please include tests and update documentation for new features.

Development Setup

- Clone the repository and install dependencies:

```bash
git clone https://github.com/your-username/surge-sdk.git
cd surge-sdk
cargo build
```

- Run tests:

```bash
cargo test
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE)
file for details.

---

## Acknowledgments

- Built with Rust and love :heart:.
- Inspired by the [Surge.sh](https://github.com/syntaxi/surge) CLI and API.
- Word lists for domain generation sourced
  from [names](https://crates.io/crates/names) crate.

---

## Contact

For questions or support, open an issue on GitHub or contact the maintainer at [cesardaniel.9611@gmail.com](mailto:cesardaniel.9611@gmail.com).
For bugs or feature requests, [open an issue](https://github.com/dantescur/surge-sdk/issues).
