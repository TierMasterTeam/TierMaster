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

1. Clone the repository (if you haven't already):

```bash
git clone https://github.com/yourusername/TierMaster.git
cd TierMaster/backend
```

2. Set up environment variables:

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

The API will be available at the configured address (default: http://127.0.0.1:8080).

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

## Project Structure

```
backend/
├── src/                # Source code
│   ├── main.rs         # Entry point
│   ├── routes/         # API routes
│   ├── models/         # Data models
│   └── services/       # Business logic
├── Cargo.toml          # Project dependencies
├── Cargo.lock          # Dependency lock file
├── .env.example        # Example environment variables
└── README.md           # This file
```

## API Endpoints

- `GET /` - Health check and API version
- `GET /api/v1/tiers` - Get available tiers
- `POST /api/v1/tiers` - Create a new tier

## Running Tests

```bash
cargo test
```

## Contributing

Please follow the standard Rust code formatting rules:

```bash
cargo fmt
cargo clippy
```

Make sure all tests pass before submitting a pull request.