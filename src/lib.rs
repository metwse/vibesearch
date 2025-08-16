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
//!
//! ## Usage
//! First, add `vibesearch` to your `Cargo.toml` and enable the features you
//! need.
//!
//! ```toml
//! [dependencies]
//! vibesearch = { version = "0.1", features = ["serde", "sha256"] }
//! ```
//!
//! Then, you can use the provided extension traits on any iterator to perform
//! a search.
//! ```rust,no_run
//! use vibesearch::{
//!     VibeSearchClient,
//!     VibeSearch, // For standard .vibe_find()
//!     VibeSearchStdHash, // For .vibe_find_hash()
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     // 1. Initialize the client from the OPENAI_API_KEY environment variable.
//!     let client = VibeSearchClient::new_from_env();
//!
//!     // --- Example 1: Basic search using the Display trait ---
//!     let data = [1, 2, 3, 4, 3, 5];
//!     let indices = data.iter().vibe_find(&client, &3).await;
//!     assert_eq!(indices, [2, 4]);
//!
//!     // --- Example 2: Search using the standard Hash trait ---
//!     let indices_hash = data.iter().vibe_find_hash(&client, &4).await;
//!     assert_eq!(indices_hash, [3]);
//! }
//! ```

#![forbid(unsafe_code, unused_must_use)]
#![warn(clippy::all)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod client;
mod error;
mod vibe;

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

/// Provides an interface for searching for items in a collection using their
/// [`std::fmt::Display`] implementation.
pub trait VibeSearch<I> {
    fn vibe_find(
        &mut self,
        client: &VibeSearchClient,
        element: I,
    ) -> impl Future<Output = Vec<u64>>;
}

/// Searches for items using the standard [`std::hash::Hash`] trait, hashing
/// them before sending to the AI.
pub trait VibeSearchStdHash<I> {
    fn vibe_find_hash(
        &mut self,
        client: &VibeSearchClient,
        element: I,
    ) -> impl Future<Output = Vec<u64>>;
}

/// Offers a secure search method by cryptographically hashing items with
/// SHA-256.
#[cfg(feature = "sha256")]
pub trait VibeSearchSha256<'a> {
    fn vibe_find_sha256(
        &'a mut self,
        client: &VibeSearchClient,
        element: &[u8],
    ) -> impl Future<Output = Vec<u64>>;
}

/// Enables searching for complex data structures (like structs) by serializing
/// them with [`serde`].
#[cfg(feature = "serde")]
pub trait VibeSearchSerde<I> {
    fn vibe_find_serde(
        &mut self,
        client: &VibeSearchClient,
        element: I,
    ) -> impl Future<Output = Vec<u64>>;
}
