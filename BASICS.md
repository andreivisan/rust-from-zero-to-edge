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

## Reading Input and CLI Arguments

### Console input & output

**Output**

```rust
fn main() {
    print!("no newline at the end");
    println!("with a newline");
    eprintln!("this goes to standard error, not standard out");
}
```

The key distinction beginners often miss: print!/println! write to stdout, 
while eprint!/eprintln! write to stderr. This matters because of how shells work. 
Normal program output goes to stdout; error messages, diagnostics, and progress 
info should go to stderr. That way a user can do myprogram > results.txt and 
still see error messages on screen, because only stdout got redirected to the file.

One thing to keep in mind: println! is relatively slow because it locks stdout 
and flushes on every call. For a quick CLI tool this is totally fine, and I'd 
recommend not worrying about it yet. Just know that if you're printing in a tight 
loop later, there's a faster way using a locked, buffered handle.

ust's stdout is line-buffered. That means the buffer is flushed whenever a newline 
character passes through it, not on every write. So it's not that println! has a 
special "auto-flush" feature and print! lacks one. It's that println! appends a 
\n, and that newline triggers the line buffer to flush. print! usually has no 
newline in it, so its text sits in the buffer waiting.

The proof that it's about the newline and not the macro: print!("hi\n") will 
flush (because of the \n), and print!("hi") will not.


```rust
use std::io::{self, Write};

fn main() {
    // Reading a line of input
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // flush because print! doesn't auto-flush

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // remove trailing newline
    println!("Hello, {name}!");

    // Parsing input
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    match input.trim().parse::<i32>() {
        Ok(number) => println!("You entered: {number}"),
        Err(_)     => println!("That's not a valid number."),
    }
}
```

### CLI Arguments

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //  args[0] = program name (like C#'s Assembly name)
    //  args[1..] = actual arguments

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]); // eprintln! → stderr
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("Processing {filename}");
}
```

### Environment vars

```rust
use std::env;

let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".to_string());
// env::var returns Result<String, VarError> — no nulls!
```

### Production CLI Apps with `clap`

For anything beyond trivial argument parsing, use the `clap` crate.

```rust
use clap::Parser;

/// A simple file processor — this doc comment becomes the help text
#[derive(Parser, Debug)]
#[command(name = "processor", version, about)]
struct Args {
    /// Input file to process
    #[arg(short, long)]
    input: String,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<String>,

    /// Enable verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Number of worker threads
    #[arg(short = 'j', long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let args = Args::parse(); // auto-parses, validates, generates --help

    if args.verbose {
        println!("Input:   {}", args.input);
        println!("Output:  {:?}", args.output);
        println!("Threads: {}", args.threads);
    }

    // Use args.input, args.output, etc.
}
```
