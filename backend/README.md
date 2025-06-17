# TierMaster Backend

This is the backend API for the TierMaster application, built with Rust.

## Contributors

- FRIN Arthur
- MENARD Paul
- DHOMMEAUX Adrien

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.85.1 or newer)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)
- [CMake](https://cmake.org/download/) (needed to build aws library)
- [Clang](https://releases.llvm.org/download.html) (needed to build aws library)

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

### CMake and Clang

The aws-lc-sys library requires CMake and a C/C++ compiler because it builds native code.
- On Linux and macOS, your system's default compiler (such as clang or gcc) should work as long as CMake is available.
- On Windows, you must use Clang, because MSVC does not support <stdatomic.h>. 
While MinGW-w64 may work, it is currently untested.

To verify CMake is installed:
```bash
cmake --version
```

Once Clang is installed on Windows, configure your environment like so:
```bash
# Tell Rust to use Clang
set CC=clang
set CXX=clang++

# Tell CMake to use Clang
set CMAKE_C_COMPILER=clang
set CMAKE_CXX_COMPILER=clang++
```

You should be able to build the project !

> **Note:** If you tried building the project before setting up CMake and Clang, make sure to clean your previous build with:
> ```bash
> cargo clean
> ```

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