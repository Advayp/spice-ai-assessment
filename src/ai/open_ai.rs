use async_openai::{config::OpenAIConfig, Client};

// Create OpenAI client and point it to Spice's runtime
pub fn get_client() -> Client<OpenAIConfig> {
    Client::with_config(OpenAIConfig::new().with_api_base("http://localhost:8090/v1"))
}
