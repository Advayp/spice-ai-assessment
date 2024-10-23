use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        CreateChatCompletionRequest, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
    },
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

fn get_message_args(messages: Vec<String>) -> Vec<ChatCompletionRequestMessage> {
    let mut res: Vec<ChatCompletionRequestMessage> = vec![get_initial_prompt()];

    for message in messages {
        res.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(message)
                .build()
                .unwrap()
                .into(),
        );
    }

    res
}

fn get_request_params(messages: Vec<String>) -> CreateChatCompletionRequest {
    let processed_messages = get_message_args(messages);

    CreateChatCompletionRequestArgs::default()
        .model("local_analytics_model")
        .messages(processed_messages)
        .build()
        .unwrap()
}

pub async fn make_request(
    client: Client<OpenAIConfig>,
    messages: Vec<String>,
) -> CreateChatCompletionResponse {
    let params = get_request_params(messages);

    client.chat().create(params).await.unwrap()
}
