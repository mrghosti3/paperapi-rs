# paper-api

A Rust library for interacting with the PaperMC API. This crate provides: JSON
data deserialization structs, compile-time checked API path builder api.

This crate is merely for reducing code duplication accross projects in terms of:
defining data structs, building API paths with code rather as `str`.

## Features

- **Type-safe API**: Strongly-typed structures for versions, builds, and support status
- **Builder pattern**: Easy construction of PaperMC API endpoints
- **Error handling**: Comprehensive error types for API interactions
- **No runtime dependencies**: Pure Rust implementation

## TODO

- Continue writing documentation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
paper-api = { git = "https://github.com/mrghosti3/paperapi-rs.git" }
```

Or:

```shell
cargo add --git "https://github.com/mrghosti3/paperapi-rs.git"
```

## Usage

For further examples, I encourage looking through unit tests.

### Basic Example

```rust
use paper_api::api::Endpoint;
use paper_api::api::builder::endpoints::ProjectVersions;
use paper_api::api::ids::Project;

// Create an endpoint for getting Paper versions
let endpoint = Endpoint::builder()
    .set_domain(Default::default())
    .set_endpoint(ProjectVersions::new().v3().set_project(Project::Paper))
    .build()
    .unwrap();

println!("Endpoint URL: {}", endpoint);
```

## API Documentation

```shell
cargo doc --open
```

## License

TBA

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
