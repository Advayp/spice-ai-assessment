use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs},
    Client,
};

// Create OpenAI client and point it to Spice's runtime
pub fn get_client() -> Client<OpenAIConfig> {
    Client::with_config(OpenAIConfig::new().with_api_base("http://localhost:8090/v1"))
}

// Draft a prompt such that GPT 4 responds only in JSON and address all the provided queries
fn get_initial_prompt() -> ChatCompletionRequestMessage {
    let message = "You will be given a series of questions about a specific dataset. Your role is provide accurate answers to each question in unformatted JSON and nothing else. 
  Specifically, return an array formatted in valid JSON where ith index contains the response (in valid unformatted JSON) to the ith question. 
  The response must be formatted such that I can run a JSON parser on your response without error.".to_string();

    ChatCompletionRequestSystemMessageArgs::default()
        .content(message)
        .build()
        .unwrap()
        .into()
}
