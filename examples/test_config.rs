use vibesearch::{VibeSearchClient, VibeSearchConfig};

fn main() {
    // Test that we can create a client with config
    let config = VibeSearchConfig::new()
        .with_caching(true)
        .with_model("gpt-4".to_string());
    
    // This should compile without errors
    let _client = VibeSearchClient::new_with_config("fake-key".to_string(), config);
    
    println!("Successfully compiled with new features!");
}