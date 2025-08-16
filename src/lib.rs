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

/// Manages the connection and interaction with the OpenAI API for VibeSearch.
///
/// Encapsulates the logic for sending structured search prompts to an AI model
/// and parsing the results back into a usable format. It is the main entry
/// point for using the library.
///
/// # Examples
///
/// ```no_run
/// use vibesearch::VibeSearchClient;
///
/// // Create a client from an environment variable (`OPENAI_API_KEY`)
/// let client = VibeSearchClient::new_from_env();
///
/// // Or create it with a specific API key
/// let client_with_key = VibeSearchClient::new("YOUR_OPENAI_API_KEY".to_string());
/// ```
pub struct VibeSearchClient {
    openai_client: Client,
}
