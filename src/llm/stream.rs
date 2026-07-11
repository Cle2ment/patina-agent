use crate::models::ActionPlan;
use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
};
use async_stream::stream;
use futures::{Stream, StreamExt};

pub fn chat_stream(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {
        let client = async_openai::Client::new();
        let mut messages = vec![];

        if let Some(system) = system {
            messages.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system)
                    .build()?
                    .into(),
            )
        }

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        );

        let schema = schemars::schema_for!(ActionPlan);
        let schema_json = schema.as_value().clone();
        let format_setting = ResponseFormat::JsonSchema {
            json_schema: ResponseFormatJsonSchema {
                description: Some(
                    "A step-by-step agent action plan with difficulty and time escimate".into(),
                ),
                name: "action_plan".into(),
                schema: schema_json,
                strict: Some(true),
            },
        };

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(messages)
            .response_format(format_setting)
            .max_tokens(2048u32)
            .build()?;

        let mut stream = client.chat().create_stream(request).await?;

        while let Some(response_result) = stream.next().await {
            match response_result {
                Ok(chunk) => {
                    if let Some(choice) = chunk.choices.first()
                    && let Some(new_text) = &choice.delta.content {
                            yield Ok(new_text.clone())
                        }
                    }
                Err(err) => yield Err(err.into())
            }
        }
    }
}
