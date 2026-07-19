use anyhow::Ok;
use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, FinishReason, ResponseFormat, ResponseFormatJsonSchema,
};
use backon::{ExponentialBuilder, Retryable};

use crate::gaia::models::GaiaOutput;

pub const GAIA_PROMPT: &'static str = r#"
You are a general AI assistant. I will ask you a question. First, determine if you can solve this problem with your current knowledge and capabilities and set `is_solvable` accordingly. 
If so, set `is_solvable` to true and proceed to solve the problem. Provide your answer in `final_answer` field. 
Otherwise, set `is_solvable` to false and explain why you cannot solve it in `unsolvable_reason` field.
Your final answer should be a number, as few words as possible, OR a comma-seperated list of numbers and/or strings.
If you are asked for a number, do not use a comma to write your number neither use units such as "$" ir percent sign unless explicitly asked for. If you are asked for a list, do not use any brackets or quotes.
If you are asked for a string, do not use articles, neither abbreviations (e.g., for cities), and write the digits in plain text. Do not use any punctuation in your answer unless explicitly asked for.
If you are asked for a comma-separated list, do not use any brackets or quotes, and write the digits in plain text. Do not use any punctuation in your answer unless explicitly asked for. Apply the above rules to each item in the list.
"#;

pub async fn solve_problem_with_retry(
    model: &str,
    system: &str,
    prompt: &str,
) -> anyhow::Result<GaiaOutput> {
    let op = || async { solve_problem(model, system, prompt).await };
    op.retry(ExponentialBuilder::default().with_max_times(3))
        .await
}

async fn solve_problem(model: &str, system: &str, prompt: &str) -> anyhow::Result<GaiaOutput> {
    let schema = schemars::schema_for!(GaiaOutput);
    let schema_json = serde_json::to_value(&schema)?;
    let format_setting = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: Some("GAIA problem solving output".into()),
            name: "gaia_output".into(),
            schema: schema_json,
            strict: Some(true),
        },
    };

    let client = async_openai::Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ])
        .response_format(format_setting)
        .build()?;

    let response = client.chat().create(request).await?;

    let choice = response
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No choices returned from the model"))?;
    if choice.finish_reason == Some(FinishReason::ContentFilter) {
        return Ok(GaiaOutput {
            is_solvable: false,
            unsolvable_reason: "Model refuse to answer".to_string(),
            final_answer: String::new(),
        });
    }

    let content = choice
        .message
        .content
        .ok_or_else(|| anyhow::anyhow!("No content in response"))?;
    let output: GaiaOutput = serde_json::from_str(&content)?;
    Ok(output)
}
