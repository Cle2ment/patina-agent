use crate::models::ActionPlan;
use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
};

pub async fn chat_complete_structured(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> anyhow::Result<String> {
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
        .max_tokens(2048u32)
        .build()?;

    let response = client.chat().create(request).await?;

    tracing::info!("Response: {:#?}", response);

    let content = response
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .ok_or_else(|| anyhow::anyhow!("No message content"))?;

    Ok(content)
}
