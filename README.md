# Rust gRPC CW721 Indexer

This project is a Rust-based indexer for CW721 tokens. 
The indexer leverages gRPC for efficient communication and aims to provide a robust and performant solution for indexing and querying CW721 tokens.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)

## Overview

The Rust GRPC CW721 Indexer is designed to index and manage CW721 tokens on the Cosmos blockchain. 
It provides an efficient and scalable solution to track and query token metadata, ownership, and other relevant information using gRPC.

## Features

- **High Performance**: Built with Rust for maximum performance and reliability.
- **gRPC Integration**: Utilizes gRPC for efficient communication between services.
- **Scalable**: Designed to handle large volumes of CW721 tokens.
- **Configurable**: Offers a range of configuration options to customize the indexing process.

## Installation

To get started with the Rust GRPC CW721 Indexer, follow these steps:

1. **Clone the Repository**:
    ```sh
    git clone https://github.com/tokenizin-agency/rust-grpc-cw721-indexer.git
    cd rust-grpc-cw721-indexer
    ```

2. **Install Rust**: Ensure you have Rust installed. You can download it from [here](https://www.rust-lang.org/).

3. **Build the Project**:
    ```sh
    cargo build --release
    ```

## Usage

After building the project, you can run the indexer using:

```sh
cargo run --release
```

## Configuration

Configuration options are specified in the `Cargo.toml` file and can be adjusted to suit your needs. Additionally, you can modify the source files to extend functionality.

### Key Files

- `src/main.rs`: The main entry point of the application.
- `src/structs.rs`: Contains the data structures used by the indexer.
- `cached_data.json`: Example data used for testing and demonstration purposes.

## Contributing

We welcome contributions to the Rust GRPC CW721 Indexer. If you have suggestions or improvements, please create a pull request or open an issue.

### Steps to Contribute

1. Fork the repository.
2. Create a new branch with a descriptive name.
3. Make your changes and commit them.
4. Push your changes to your forked repository.
5. Create a pull request to the main repository.

## License

=> **WTFPL**
