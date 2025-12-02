// SPDX-FileCopyrightText: 2024 Semiotic AI, Inc.
//
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use crate::BlockNumber;

use super::{
    single_block_request::{BlockHashAndNumber, Reference},
    Response, SingleBlockRequest,
};

impl SingleBlockRequest {
    /// Create a Firehose [`SingleBlockRequest`] for the given *block number*.
    ///
    /// We're leaving this method here for backwards compatibility.
    pub fn new(num: u64) -> SingleBlockRequest {
        SingleBlockRequest::new_by_block_number(num)
    }

    /// Create a Firehose [`SingleBlockRequest`] for the given *block number*.
    pub fn new_by_block_number(num: u64) -> SingleBlockRequest {
        SingleBlockRequest {
            reference: Some(Reference::BlockNumber(BlockNumber { num })),
            ..Default::default()
        }
    }

    /// Create a Firehose [`SingleBlockRequest`] for the given *block hash*.
    pub fn new_by_block_hash_and_number(hash: String, num: u64) -> SingleBlockRequest {
        SingleBlockRequest {
            reference: Some(Reference::BlockHashAndNumber(BlockHashAndNumber {
                hash,
                num,
            })),
            ..Default::default()
        }
    }
}

/// Work with block numbers or slots in a unified way.
///
/// This trait provides a common interface for accessing block identifiers,
/// which can either be a block number (for execution layer blocks) or a slot
/// (for consensus layer blocks). By implementing this trait, types can expose
/// their block number or slot in a standardized manner, enabling generic handling
/// of different block types in streaming and processing workflows.
///
/// # Requirements
///
/// Types implementing this trait must also implement [`Clone`], [`Send`], and
/// `'static` to ensure compatibility with asynchronous and concurrent contexts.
///
/// # Example
///
/// ```rust
/// use firehose_rs::HasNumberOrSlot;
///
/// #[derive(Clone)]
/// struct ExecutionBlock {
///     block_number: u64,
/// }
///
/// #[derive(Clone)]
/// struct ConsensusBlock {
///     slot: u64,
/// }
///
/// impl HasNumberOrSlot for ExecutionBlock {
///     fn number_or_slot(&self) -> u64 {
///         self.block_number
///     }
/// }
///
/// impl HasNumberOrSlot for ConsensusBlock {
///     fn number_or_slot(&self) -> u64 {
///         self.slot
///     }
/// }
///
/// fn process_block<T: HasNumberOrSlot>(block: &T) {
///     println!("Processing block with identifier: {}", block.number_or_slot());
/// }
///
/// let execution_block = ExecutionBlock { block_number: 42 };
/// let consensus_block = ConsensusBlock { slot: 24 };
///
/// process_block(&execution_block);
/// process_block(&consensus_block);
/// ```
///
/// # Use Case
///
/// This trait is particularly useful in scenarios where both execution and
/// consensus layer blocks need to be processed generically, such as in blockchain
/// indexing or synchronization applications.
pub trait HasNumberOrSlot: Clone + Send + 'static {
    /// Return the block number or slot.
    ///
    /// This value uniquely identifies the block within its respective layer,
    /// either as a block number (execution layer) or a slot (consensus layer).
    fn number_or_slot(&self) -> u64;
}

/// Convert protocol buffer messages into domain-specific block types.
///
/// This trait simplifies the deserialization and conversion process when streaming
/// data from a Firehose gRPC service. Implementations provide a uniform way to
/// transform a [`Response`] message into a concrete domain type.
///
/// # Example
///
/// ```rust
/// use firehose_rs::{FromResponse, Response};
/// use std::fmt;
///
/// #[derive(Debug)]
/// struct MyBlock {
///     cursor: String,
/// }
///
/// #[derive(Debug)]
/// struct MyError(String);
///
/// impl fmt::Display for MyError {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "{}", self.0)
///     }
/// }
///
/// impl FromResponse for MyBlock {
///     type Error = MyError;
///
///     fn from_response(msg: Response) -> Result<Self, Self::Error> {
///         Ok(MyBlock { cursor: msg.cursor })
///     }
/// }
/// ```
///
/// # Errors
///
/// Implementations should return an error if the conversion fails. This can
/// occur due to invalid data, missing fields, or other deserialization issues.
/// The error type must implement [`Display`](std::fmt::Display) and [`Send`].
pub trait FromResponse: Sized
where
    Self::Error: Display + Send,
{
    type Error;

    /// Convert a [`Response`] into the implementing type.
    ///
    /// # Parameters
    ///
    /// * `msg`: The [`Response`] message received from the Firehose stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the converted type on success, or `Self::Error`
    /// if the conversion fails.
    fn from_response(msg: Response) -> Result<Self, Self::Error>;
}
