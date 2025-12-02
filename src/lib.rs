// SPDX-FileCopyrightText: 2024 Semiotic AI, Inc.
//
// SPDX-License-Identifier: Apache-2.0

//! # Firehose in Rust
//!

mod firehose_v2;

pub(crate) use firehose_v2::single_block_request::BlockNumber;

/// Interact programatically with the Firehose v2 Fetch API.
pub use firehose_v2::fetch_client::FetchClient;

/// Create Firehose API fetch requests.
pub use firehose_v2::Request;

/// Work with Firehose API streaming responses.
pub use firehose_v2::Response;

/// Create Firehose API streaming requests.
pub use firehose_v2::SingleBlockRequest;

/// Receive Firehose API fetch responses.
pub use firehose_v2::SingleBlockResponse;

/// Work with the Firehose v2 Stream API.
pub use firehose_v2::stream_client::StreamClient;

pub use crate::firehose_v2::request::{FromResponse, HasNumberOrSlot};
