use crate::{Error, VibeSearchClient};
use openai_dive::v1::{
    api::Client,
    models::FlagshipModel,
    resources::chat::{
        ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage,
        ChatMessageContent, JsonSchemaBuilder,
    },
};

impl VibeSearchClient {
    pub fn new(api_key: String) -> Self {
        Self {
            openai_client: Client::new(api_key),
        }
    }

    pub fn new_from_env() -> Self {
        Self {
            openai_client: Client::new_from_env(),
        }
    }

    pub async fn find(&self, promt: String) -> Result<Vec<u64>, Error> {
        let parameters = ChatCompletionParametersBuilder::default()
            .model(FlagshipModel::Gpt4O.to_string())
            .messages(vec![
                ChatMessage::System {
                    content: ChatMessageContent::Text(String::from(
                        "You are a array search tool. \
                         Find the index of given element, in given array. \
                         Data given in format: \n\
                         {elements_separator}<newline>\n\
                         find {searching_element}<newline>\n\
                         {element_separator}<newline>{index},{element}<newline>...",
                    )),
                    name: None,
                },
                ChatMessage::User {
                    content: ChatMessageContent::Text(promt),
                    name: None,
                },
            ])
            .response_format(ChatCompletionResponseFormat::JsonSchema {
                json_schema: JsonSchemaBuilder::default()
                    .name("search")
                    .schema(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "result": {
                                "type": "array",
                                "items": {
                                    "type": "integer",
                                    "minimum": 0
                                },
                            }
                        },
                        "required": ["result"],
                        "additionalProperties": false
                    }))
                    .strict(true)
                    .build()?,
            })
            .build()?;

        dbg!(self.openai_client.chat().create(parameters).await?);

        Ok(vec![])
    }
}
