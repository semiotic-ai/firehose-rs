use std::fmt::Display;

use crate::BlockNumber;

use super::{single_block_request::Reference, Response, SingleBlockRequest};

impl SingleBlockRequest {
    /// Create a Firehose [`SingleBlockRequest`] for the given *block number*.
    pub fn new(num: u64) -> SingleBlockRequest {
        SingleBlockRequest {
            reference: Some(Reference::BlockNumber(BlockNumber { num })),
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
/// # Provided Method
///
/// * `number_or_slot`: Returns the block number or slot as a `u64`.
///
/// # Example
///
/// ```rust
/// use firehose_client::HasNumberOrSlot;
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
///
/// [`Clone`]: std::clone::Clone
/// [`Send`]: std::marker::Send
///
pub trait HasNumberOrSlot: Clone + Send + 'static {
    /// Return the block number or slot.
    ///
    /// This value uniquely identifies the block within its respective layer,
    /// either as a block number (execution layer) or a slot (consensus layer).
    fn number_or_slot(&self) -> u64;
}

/// Convert protocol buffer messages into domain-specific block types.
///
/// This trait is intended to simplify the deserialization and conversion process
/// when streaming data from a Firehose gRPC service. Implementations of this trait
/// provide a uniform way to transform a `firehose_protos::Response` message into
/// a concrete type.
///
/// # Example
///
/// ```rust,
/// use firehose_client::FromResponse;
/// use firehose_protos::Response;
///
/// struct MyBlock;
///
/// impl FromResponse for MyBlock {
///     fn from_response(msg: Response) -> Result<Self, ClientError> {
///         // Perform conversion logic here.
///         Ok(MyBlock)
///     }
/// }
/// ```
///
/// # Errors
///
/// Implementations should return a `ProtosError` if the conversion fails. This can
/// occur due to invalid data, missing fields, or other deserialization issues.
///
/// # Usage
///
/// The `FromResponse` trait is typically used in conjunction with generic streaming
/// methods, such as `stream_blocks`, allowing these methods to work with
/// different block types by specifying the type parameter:
///
/// ```rust
/// let stream = client
///     .stream_blocks_generic::<FirehoseBeaconBlock>(start, total)
///     .await?;
/// ```
///
pub trait FromResponse: Sized
where
    Self::Error: Display + Send,
{
    type Error;

    /// Convert a `crate::Response` into the implementing type.
    ///
    /// # Parameters
    ///
    /// * `msg`: The `Response` message received from the Firehose stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the converted type on success, or a `ClientError`
    /// if the conversion fails.
    fn from_response(msg: Response) -> Result<Self, Self::Error>;
}
