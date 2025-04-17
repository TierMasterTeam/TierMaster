# TierMaster Backend

This is the backend API for the TierMaster application, built with Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.85.1 or newer)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

## Getting Started

### Checking Rust Version

To check if Rust is installed and verify the version:

```bash
rustc --version
cargo --version
```

### Installing Rust

If Rust is not installed, you can install it using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the installation instructions and restart your terminal.

### Setting Up the Project


Set up environment variables:

```bash
cp .env.example .env
```

Edit the `.env` file to set your desired configuration.

### Installing Dependencies

All project dependencies are managed by Cargo. To install them:

```bash
cargo build
```

This will read the `Cargo.toml` file and download all required dependencies.

## Running the API

### Development Mode

```bash
cargo run
```

### Production Mode

```bash
cargo run --release
```

The API will be available at the configured address (default: http://127.0.0.1:3000).

## Docker Support

You can also run the backend using Docker:

```bash
docker build -t tiermaster-backend .
docker run -p 8080:8080 tiermaster-backend
```

Or with docker-compose:

```bash
docker-compose up
```

## Running Tests

```bash
cargo test
```