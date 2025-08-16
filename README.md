# vibesearch
Why solve one of computer science's most fundamental problems with a boring,
efficient, and near-instant `.iter().position()`? That's so last century. 🙄

## Why vibesearch?
- 🦀 Peak Rust Safety: Written in 100% safe Rust because we
  `forbid(unsafe_code)`
- 🩸 Bleeding edge technology: Next-generation solution for finding an element
  in an array by asking a planet-sized AI model
- ™️ Blazingly Slow: Swap those pesky nanoseconds of local computation for
  hundreds of milliseconds of glorious network latency. Feel the weight of
  progress!
- 🏆 Featuring way 👋 too 2️⃣ many 🤯 emojis in the 📖 readme 💨

While the entire concept is fundamentally unreliable, at least you won't get a
segmentation fault. You're welcome. 🦀 🔥

## Usage
### 1. Add to your `Cargo.toml`:
```toml
[dependencies]
vibesearch = "0.1"
tokio = { version = "1", features = ["full"] }
```

### 2. Set your OpenAI API key:
```sh
export OPENAI_API_KEY="sk-..."
```

### 3. Find it:
```rs
use vibesearch::{VibeSearchClient, VibeSearch};

#[tokio::main]
async fn main() {
    let client = VibeSearchClient::new_from_env();
    let data = [5, 2, 8, 2, 9];

    // Find the indices of '2'
    let result = data.iter().vibe_find(&client, &2).await;

    // Expected output: [1, 3]
    println!("{:?}", result);
}
```
