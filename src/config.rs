use openai_dive::v1::models::FlagshipModel;

/// Configuration for the VibeSearchClient
#[derive(Debug, Clone)]
pub struct VibeSearchConfig {
    /// The OpenAI model to use
    pub model: String,
    /// Temperature for the model (0.0 to 2.0)
    pub temperature: Option<f32>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Whether to use caching
    pub use_caching: bool,
}

impl Default for VibeSearchConfig {
    fn default() -> Self {
        Self {
            model: FlagshipModel::Gpt4O.to_string(),
            temperature: None,
            max_tokens: None,
            use_caching: false,
        }
    }
}

impl VibeSearchConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the model to use
    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    /// Set the temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the maximum tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Enable or disable caching
    pub fn with_caching(mut self, use_caching: bool) -> Self {
        self.use_caching = use_caching;
        self
    }
}