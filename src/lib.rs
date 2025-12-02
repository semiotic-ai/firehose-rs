// SPDX-FileCopyrightText: 2024 Semiotic AI, Inc.
//
// SPDX-License-Identifier: Apache-2.0

//! # firehose-rs
//!
//! A Rust client library for [StreamingFast's Firehose protocol](https://firehose.streamingfast.io/),
//! providing gRPC bindings for streaming and fetching blockchain data.
//!
//! ## Features
//!
//! - **gRPC client bindings** for Firehose v2 API
//! - **Streaming support** via [`StreamClient`] for continuous block sequences
//! - **Fetch support** via [`FetchClient`] for individual block retrieval
//! - **Serde integration** for JSON serialization of all message types
//! - **Flexible block requests** by number, hash, or cursor
//!
//! ## Quick Start
//!
//! ### Streaming Blocks
//!
//! ```rust,no_run
//! use firehose_rs::{StreamClient, Request};
//! use tonic::transport::Channel;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let channel = Channel::from_static("https://your-firehose-endpoint:443")
//!     .connect()
//!     .await?;
//!
//! let mut client = StreamClient::new(channel);
//!
//! let request = Request {
//!     start_block_num: 1000,
//!     stop_block_num: 2000,
//!     final_blocks_only: true,
//!     ..Default::default()
//! };
//!
//! let mut stream = client.blocks(request).await?.into_inner();
//!
//! while let Some(response) = stream.message().await? {
//!     println!("Received block at cursor: {}", response.cursor);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Fetching a Single Block
//!
//! ```rust,no_run
//! use firehose_rs::{FetchClient, SingleBlockRequest};
//! use tonic::transport::Channel;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let channel = Channel::from_static("https://your-firehose-endpoint:443")
//!     .connect()
//!     .await?;
//!
//! let mut client = FetchClient::new(channel);
//!
//! let request = SingleBlockRequest::new_by_block_number(12345);
//! let response = client.block(request).await?;
//!
//! println!("Fetched block: {:?}", response.into_inner().block);
//! # Ok(())
//! # }
//! ```
//!
//! ### Creating Block Requests
//!
//! ```rust
//! use firehose_rs::SingleBlockRequest;
//!
//! // By block number
//! let request = SingleBlockRequest::new_by_block_number(12345);
//!
//! // By hash and number
//! let request = SingleBlockRequest::new_by_block_hash_and_number(
//!     "0xabc123...".to_string(),
//!     12345,
//! );
//! ```

mod firehose_v2;

pub(crate) use firehose_v2::single_block_request::BlockNumber;

/// gRPC client for the Firehose v2 Fetch API.
///
/// Use this client to fetch individual blocks by number, hash, or cursor.
pub use firehose_v2::fetch_client::FetchClient;

/// Streaming request configuration for the Firehose v2 API.
///
/// Configure start/stop block numbers, cursor position, and whether to
/// receive only finalized blocks.
pub use firehose_v2::Request;

/// Streaming response from the Firehose v2 API.
///
/// Contains the block data, fork step information, cursor for resumption,
/// and optional block metadata.
pub use firehose_v2::Response;

/// Request for fetching a single block from the Firehose API.
///
/// Supports fetching by block number, block hash + number, or cursor.
pub use firehose_v2::SingleBlockRequest;

/// Response from a single block fetch request.
///
/// Contains the block data and optional metadata.
pub use firehose_v2::SingleBlockResponse;

/// gRPC client for the Firehose v2 Stream API.
///
/// Use this client to stream continuous sequences of blocks.
pub use firehose_v2::stream_client::StreamClient;

/// Trait for unified access to block numbers or slots.
///
/// See [`HasNumberOrSlot`](crate::firehose_v2::request::HasNumberOrSlot) for details.
pub use crate::firehose_v2::request::HasNumberOrSlot;

/// Trait for converting Firehose responses to domain types.
///
/// See [`FromResponse`](crate::firehose_v2::request::FromResponse) for details.
pub use crate::firehose_v2::request::FromResponse;
