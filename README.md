# INI Parser

## Project Description

This project implements an INI parser using Rust, to help parsing and storing key and values and create an ini file from
the stored key and values

## Installation & Setup

Ensure you have **Rust** installed. To use this library in your project, follow these steps:

1. Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
ini-parser = { git = "repo link" }
```

2. Run `cargo build` to download and compile the library:

```sh
cargo build
```

3. Import the library in your Rust project:

```rust
use ini_parser::IniParser;
```

## Usage

### Creating a New INI Parser

```rust
let mut ini_parser = IniParser::new_ini_parser();
```

### Loading Configuration from a File

```rust
ini_parser.load_from_file("config.ini").expect("Failed to load INI file");
```

### Retrieving a Value

```rust
if let Some(value) = ini_parser.get("default", "forwardx11") {
    println!("forwardx11: {}", value);
}
```

### Getting All Section Names

```rust
let sections = ini_parser.get_section_names();
println!("Sections: {:?}", sections);
```
