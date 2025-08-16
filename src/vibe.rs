use crate::{
    VibeSearch, VibeSearchStdHash,
    protocol::{DisplayPromptFormatter, StdHashPromptFormatter},
};
use std::{fmt::Display, hash::Hash};

#[cfg(feature = "serde")]
use crate::{VibeSearchSerde, protocol::SerdePromptFormatter};
#[cfg(feature = "sha256")]
use crate::{VibeSearchSha256, protocol::Sha256PromptFormatter};
#[cfg(feature = "serde")]
use serde::Serialize;

impl<T: Iterator<Item = impl Display>> VibeSearch<T::Item> for T {
    fn vibe_find(
        &mut self,
        client: &crate::VibeSearchClient,
        element: T::Item,
    ) -> impl Future<Output = Vec<u64>> {
        let prompt = DisplayPromptFormatter::to_prompt(self, element);

        async move { client.prompt(prompt).await.unwrap_or_default() }
    }
}

impl<T: Iterator<Item = impl Hash>> VibeSearchStdHash<T::Item> for T {
    fn vibe_find_hash(
        &mut self,
        client: &crate::VibeSearchClient,
        element: T::Item,
    ) -> impl Future<Output = Vec<u64>> {
        let prompt = StdHashPromptFormatter::to_prompt(self, element);

        async move { client.prompt(prompt).await.unwrap_or_default() }
    }
}

#[cfg(feature = "sha256")]
impl<'a, T: Iterator<Item = &'a [u8]>> VibeSearchSha256<'a> for T {
    fn vibe_find_sha256(
        &'a mut self,
        client: &crate::VibeSearchClient,
        element: &[u8],
    ) -> impl Future<Output = Vec<u64>> {
        let prompt = Sha256PromptFormatter::to_prompt(self, element);

        async move { client.prompt(prompt).await.unwrap_or_default() }
    }
}

#[cfg(feature = "serde")]
impl<T: Iterator<Item = impl Serialize>> VibeSearchSerde<T::Item> for T {
    fn vibe_find_serde(
        &mut self,
        client: &crate::VibeSearchClient,
        element: T::Item,
    ) -> impl Future<Output = Vec<u64>> {
        let prompt = SerdePromptFormatter::to_prompt(self, element);

        async move {
            if let Ok(prompt) = prompt {
                client.prompt(prompt).await.unwrap_or_default()
            } else {
                vec![]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_default_formatters() {
        let data = [1, 2, 3, 4, 5];

        let client = VibeSearchClient::new_from_env();

        assert_eq!(data.iter().vibe_find(&client, &2).await, [1]);
        assert_eq!(data.iter().vibe_find_hash(&client, &3).await, [2]);
    }

    #[tokio::test]
    #[cfg(feature = "serde")]
    async fn test_serde_formatter() {
        let data = [1, 2, 2, 5, 4, 8, 2];

        let client = VibeSearchClient::new_from_env();

        assert_eq!(data.iter().vibe_find_serde(&client, &2).await, [1, 2, 6]);
    }

    #[tokio::test]
    #[cfg(feature = "sha256")]
    async fn test_sha256_formatter() {
        let data: Vec<&[u8]> = vec![&[1], &[1], &[3], &[3], &[2], &[1]];

        let client = VibeSearchClient::new_from_env();

        assert_eq!(
            data.into_iter().vibe_find_sha256(&client, &[1]).await,
            [0, 1, 5]
        );
    }
}
