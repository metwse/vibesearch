use crate::{VibeSearchClient, protocol::DisplayPromptFormatter};
use std::fmt::Display;
use std::future::Future;

/// Extension trait for batch searching operations
pub trait VibeBatchSearch<I> {
    /// Find all indices of multiple elements in the collection
    fn vibe_find_batch(
        &mut self,
        client: &VibeSearchClient,
        elements: Vec<I>,
    ) -> impl Future<Output = Vec<Vec<u64>>>;
}

impl<T: Iterator<Item = impl Display> + Clone> VibeBatchSearch<T::Item> for T 
where
    T::Item: Display + Clone,
{
    fn vibe_find_batch(
        &mut self,
        client: &VibeSearchClient,
        elements: Vec<T::Item>,
    ) -> impl Future<Output = Vec<Vec<u64>>> {
        async move {
            let mut results = Vec::with_capacity(elements.len());
            
            for element in elements {
                let mut iter_clone = self.clone();
                let prompt = DisplayPromptFormatter::to_prompt(&mut iter_clone, element);
                let result = client.prompt(prompt).await.unwrap_or_default();
                results.push(result);
            }
            
            results
        }
    }
}