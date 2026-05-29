# The Basics

## Cargo

Cargo is the package manager and build tool for any Rust project.

### Cargo.toml

Every Cargo project contains a `Cargo.toml` file where the basic package settings,
dependencies, and workspace configuration are set.

Example:

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2024"

[dependencies]
serde_json = "1.0"
log = "0.4"

[workspace]
members = [
    "web_api",
    "business",
    "tests"
]
```

`package` - metadata for the current package, such as name, version, and Rust edition

`dependencies` - external crates used by the package

`workspace` - a group of related packages that are built and managed together.

### Cargo.lock

`Cargo.lock` records the exact dependency versions used for a build. Applications
usually commit this file so builds are reproducible. Libraries can also commit it,
but downstream projects resolve their own dependency versions.

### Project structure

```text
my_project/
├── Cargo.toml
└── src/
    └── main.rs
```

`src/main.rs` is the default entry point for a binary application created with
`cargo new my_project`.

`src/lib.rs` is the default entry point for a library created with
`cargo new my_project --lib`.

### Common Cargo commands

```bash
# Create new project
cargo new my_project
cargo new my_project --lib  # Create library project

# Build and run
cargo build          # Like 'dotnet build'
cargo run            # Like 'dotnet run'
cargo test           # Like 'dotnet test'
cargo check          # Type-check without producing the final binary

# Package management
cargo add serde      # Add dependency (like 'dotnet add package')
cargo update         # Update dependencies

# Release build
cargo build --release  # Optimized build
cargo run --release    # Run optimized version

# Documentation
cargo doc --open     # Generate and open docs

# Formatting and linting
cargo fmt            # Format Rust code
cargo clippy         # Run Rust lints
```
