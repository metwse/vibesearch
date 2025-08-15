//! `vibesearch` is a Rust library that performs element lookups in iterable
//! data structures by querying the OpenAI API.
//!
//! Instead of using traditional, local search algorithms, it serializes the
//! data collection and the target item into a specialized text protocol.
//! This protocol is then sent as a prompt to an AI model, which identifies the
//! index of the element.
//!
//! ## Core Concepts
//! - Flexible Data Protocol: The library provides a suite of traits to handle
//!   various data types and use cases through different serialization
//!   strategies (Display, Hash, SHA-256, Serde)
//! - This allows it to process everything from simple text to complex, custom
//!   data structures.

#![forbid(unsafe_code, unused_must_use)]
#![warn(clippy::all)]

mod client;
mod error;

/// Defines the communication protocol for querying the AI model.
pub mod protocol;

use openai_dive::v1::api::Client;

pub use error::Error;

// Finds positions of element with some time complexity.
pub struct VibeSearchClient {
    openai_client: Client,
}
