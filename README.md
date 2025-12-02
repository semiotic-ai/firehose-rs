# firehose-rs

[![Crates.io](https://img.shields.io/crates/v/firehose-rs.svg)](https://crates.io/crates/firehose-rs)
[![Documentation](https://docs.rs/firehose-rs/badge.svg)](https://docs.rs/firehose-rs)
[![License](https://img.shields.io/crates/l/firehose-rs.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/semiotic-ai/firehose-rs/tests.yml?branch=main)](https://github.com/semiotic-ai/firehose-rs/actions)
[![REUSE status](https://api.reuse.software/badge/github.com/semiotic-ai/firehose-rs)](https://api.reuse.software/info/github.com/semiotic-ai/firehose-rs)

A Rust client library for [StreamingFast's Firehose protocol](https://firehose.streamingfast.io/), providing gRPC bindings for streaming and fetching blockchain data.

## Features

- **gRPC client bindings** for Firehose v2 API
- **Streaming support** via `StreamClient` for continuous block sequences
- **Fetch support** via `FetchClient` for individual block retrieval
- **Serde integration** for JSON serialization of all message types
- **Flexible block requests** by number, hash, or cursor

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
firehose-rs = "0.3"
```

### Build Requirements

**Protoc compiler must be installed** - the build script compiles protocol buffer definitions to generate gRPC code. Install via:

```bash
# macOS
brew install protobuf

# Ubuntu/Debian
apt install protobuf-compiler

# Or download from https://github.com/protocolbuffers/protobuf/releases
```

## Quick Start

### Streaming Blocks

```rust
use firehose_rs::{StreamClient, Request};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Firehose endpoint
    let channel = Channel::from_static("https://your-firehose-endpoint:443")
        .connect()
        .await?;

    let mut client = StreamClient::new(channel);

    // Create a streaming request starting from block 1000
    let request = Request {
        start_block_num: 1000,
        stop_block_num: 2000,
        final_blocks_only: true,
        ..Default::default()
    };

    // Stream blocks
    let mut stream = client.blocks(request).await?.into_inner();

    while let Some(response) = stream.message().await? {
        println!("Received block at cursor: {}", response.cursor);
    }

    Ok(())
}
```

### Fetching a Single Block

```rust
use firehose_rs::{FetchClient, SingleBlockRequest};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("https://your-firehose-endpoint:443")
        .connect()
        .await?;

    let mut client = FetchClient::new(channel);

    // Fetch block by number
    let request = SingleBlockRequest::new_by_block_number(12345);
    let response = client.block(request).await?;

    println!("Fetched block: {:?}", response.into_inner().block);

    Ok(())
}
```

### Fetching by Hash and Number

```rust
use firehose_rs::SingleBlockRequest;

// Fetch a specific block by its hash and number
let request = SingleBlockRequest::new_by_block_hash_and_number(
    "0xabc123...".to_string(),
    12345,
);
```

## API Overview

### Clients

| Client | Description |
|--------|-------------|
| `StreamClient` | Streaming RPC for continuous block sequences |
| `FetchClient` | Unary RPC for individual block retrieval |

### Request Types

| Type | Description |
|------|-------------|
| `Request` | Streaming request with start/stop block configuration |
| `SingleBlockRequest` | Single block request by number, hash, or cursor |

### Response Types

| Type | Description |
|------|-------------|
| `Response` | Streaming response with block data and cursor |
| `SingleBlockResponse` | Single block fetch response |

### Traits

| Trait | Description |
|-------|-------------|
| `HasNumberOrSlot` | Unified access to block number or slot |
| `FromResponse` | Convert protobuf responses to domain types |

## Protocol Reference

This library implements the [Firehose v2 protocol](https://github.com/streamingfast/proto/blob/develop/sf/firehose/v2/firehose.proto) by StreamingFast.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development

```bash
# Run tests
cargo test --all-features

# Run lints
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

The protocol buffer definitions in `protos/` are from [StreamingFast](https://github.com/streamingfast/proto) and are also licensed under Apache 2.0.

## Acknowledgments

- [StreamingFast](https://www.streamingfast.io/) for creating the Firehose protocol
- [Tonic](https://github.com/hyperium/tonic) for the excellent gRPC framework
- [Prost](https://github.com/tokio-rs/prost) for protocol buffer support

## Resources

- [Firehose Documentation](https://firehose.streamingfast.io/)
- [API Reference](https://docs.rs/firehose-rs)
- [GitHub Repository](https://github.com/semiotic-ai/firehose-rs)
