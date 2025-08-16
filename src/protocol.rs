//!
//! Serialize iterable data structures into prompt.

use rand::Rng;
use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

#[cfg(feature = "sha256")]
use base64::prelude::*;
#[cfg(feature = "sha256")]
use ring::digest;

#[cfg(feature = "serde")]
use crate::Error;
#[cfg(feature = "serde")]
use serde::Serialize;

fn random_base36(len: usize) -> String {
    let mut res = String::with_capacity(len);

    for _ in 0..len {
        let c: u8 = rand::rng().random_range(0..36);
        res.push(char::from_digit(c.into(), 36).unwrap());
    }

    res
}

#[cfg(feature = "sha256")]
fn sha256_base64(data: &[u8]) -> String {
    let hash = digest::digest(&digest::SHA256, data);
    hash.as_ref();

    BASE64_STANDARD.encode(hash)
}

#[cfg(feature = "serde")]
fn serialize_base64<T: Serialize>(data: T) -> Result<String, Error> {
    Ok(BASE64_STANDARD.encode(&bincode::serde::encode_to_vec(
        data,
        bincode::config::standard(),
    )?))
}

/// A trait for iterators whose items can be converted into a search prompt
/// using their [`std::fmt::Display`] implementation.
pub trait DisplayPromptFormatter<I> {
    fn to_prompt(&mut self, searching_for: I) -> String;
}

/// A trait for iterators whose items can be converted into a search prompt
/// by hashing each item using the [`std::hash::Hash`] trait.
pub trait StdHashPromptFormatter<I> {
    fn to_prompt(&mut self, searching_for: I) -> String;
}

/// A trait for iterators over slices that can be converted into a search
/// prompt using SHA-256 hashing algorithm.
#[cfg(feature = "sha256")]
pub trait Sha256PromptFormatter<'a> {
    fn to_prompt(&'a mut self, searching_for: &[u8]) -> String;
}

/// A trait for iterators whose items can be serialized into a search prompt
/// using the [`serde::Serialize`] trait.
#[cfg(feature = "serde")]
pub trait SerdePromptFormatter<I> {
    fn to_prompt(&mut self, searching_for: I) -> Result<String, Error>;
}

impl<T: Iterator<Item = impl Display>> DisplayPromptFormatter<T::Item> for T {
    fn to_prompt(&mut self, searching_for: T::Item) -> String {
        let element_separator = random_base36(12);
        let mut res = format!("{element_separator}\nfind {searching_for}\n");

        for (i, element) in self.enumerate() {
            res.push_str(&format!("{element_separator}\n{i},{element}\n"));
        }

        res
    }
}

impl<T: Iterator<Item = impl Hash>> StdHashPromptFormatter<T::Item> for T {
    fn to_prompt(&mut self, searching_for: T::Item) -> String {
        let mut hasher = DefaultHasher::new();
        searching_for.hash(&mut hasher);
        let mut res = format!("#\nfind {}\n", hasher.finish());

        for (i, element) in self.enumerate() {
            let mut hasher = DefaultHasher::new();
            element.hash(&mut hasher);
            res.push_str(&format!("#\n{i},{}\n", hasher.finish()));
        }

        res
    }
}

#[cfg(feature = "sha256")]
impl<'a, T: Iterator<Item = &'a [u8]>> Sha256PromptFormatter<'a> for T {
    fn to_prompt(&mut self, searching_for: &[u8]) -> String {
        let mut res = format!("#\nfind {}\n", sha256_base64(searching_for));

        for (i, element) in self.enumerate() {
            res.push_str(&format!("#\n{i},{}\n", sha256_base64(element)));
        }

        res
    }
}

#[cfg(feature = "serde")]
impl<T: Iterator<Item = impl Serialize>> SerdePromptFormatter<T::Item> for T {
    fn to_prompt(&mut self, searching_for: T::Item) -> Result<String, Error> {
        let element_separator = random_base36(12);
        let mut res = format!(
            "{element_separator}\nfind {}\n",
            serialize_base64(searching_for)?
        );

        for (i, element) in self.enumerate() {
            res.push_str(&format!(
                "{element_separator}\n{i},{}\n",
                serialize_base64(element)?
            ));
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_prompt_formatter() {
        let data = ["foo", "bar", "foobar"];
        let mut iter = data.iter();
        let prompt = DisplayPromptFormatter::to_prompt(&mut iter, &"bar");

        let lines: Vec<_> = prompt.split("\n").collect();
        let separator = lines[0];

        assert_eq!(lines[1], "find bar");
        assert_eq!(lines[2], separator);
        assert_eq!(lines[3], "0,foo");
        assert_eq!(lines[4], separator);
        assert_eq!(lines[5], "1,bar");
        assert_eq!(lines[6], separator);
        assert_eq!(lines[7], "2,foobar");
    }

    fn default_hasher(i: impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        i.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_hash_prompt_formatter() {
        let data = [123, 53, 351, 412];
        let mut iter = data.iter();
        let prompt = StdHashPromptFormatter::to_prompt(&mut iter, &53);

        let lines: Vec<_> = prompt.split("\n").collect();
        let separator = lines[0];

        assert_eq!("#", separator);
        assert_eq!(lines[1], format!("find {}", default_hasher(53)));
        assert_eq!(lines[2], "#");
        assert_eq!(lines[3], format!("0,{}", default_hasher(123)));
        assert_eq!(lines[4], "#");
        assert_eq!(lines[5], format!("1,{}", default_hasher(53)));
        assert_eq!(lines[6], "#");
        assert_eq!(lines[7], format!("2,{}", default_hasher(351)));
        assert_eq!(lines[8], "#");
        assert_eq!(lines[9], format!("3,{}", default_hasher(412)));
    }

    #[test]
    #[cfg(feature = "sha256")]
    fn test_sha256_prompt_formatter() {
        let data: Vec<&[u8]> = vec![&[1], &[2], &[3]];
        let mut iter = data.into_iter();
        let prompt = Sha256PromptFormatter::to_prompt(&mut iter, &[2]);

        let lines: Vec<_> = prompt.split("\n").collect();
        let separator = lines[0];

        assert_eq!("#", separator);
        assert_eq!(lines[1], format!("find {}", sha256_base64(&[2])));
        assert_eq!(lines[2], "#");
        assert_eq!(lines[3], format!("0,{}", sha256_base64(&[1])));
        assert_eq!(lines[4], "#");
        assert_eq!(lines[5], format!("1,{}", sha256_base64(&[2])));
        assert_eq!(lines[6], "#");
        assert_eq!(lines[7], format!("2,{}", sha256_base64(&[3])));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_prompt_formatter() -> Result<(), Error> {
        let data = [1, 2, 3];
        let mut iter = data.iter();
        let prompt = SerdePromptFormatter::to_prompt(&mut iter, &2)?;

        let lines: Vec<_> = prompt.split("\n").collect();
        let separator = lines[0];

        assert_eq!(lines[1], format!("find {}", serialize_base64(2)?));
        assert_eq!(lines[2], separator);
        assert_eq!(lines[3], format!("0,{}", serialize_base64(1)?));
        assert_eq!(lines[4], separator);
        assert_eq!(lines[5], format!("1,{}", serialize_base64(2)?));
        assert_eq!(lines[6], separator);
        assert_eq!(lines[7], format!("2,{}", serialize_base64(3)?));

        Ok(())
    }
}
