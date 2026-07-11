use std::result::Result::Ok;
use futures::StreamExt;
use patina_agent::{
    constant::DEEPSEEK_V4_FLASH_MODEL, llm::{complete::chat_complete, stream::chat_stream, structured::chat_complete_structured},
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let url = std::env::var("OPENAI_BASE_URL")?;
    println!("{url}");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let s = chat_stream(
        DEEPSEEK_V4_FLASH_MODEL,
        Some("你是一个全能Agent。"),
        "我想去观看今年的美加墨世界杯，应该如何安排？",
    );

    futures::pin_mut!(s);
    let mut output = String::new();
    while let Some(result) = s.next().await {
        match result {
            Ok(txt) => {
                output.push_str(&txt);
                print!("{txt}");
            }
            Err(err) => {
                tracing::error!("\nError while streaming: {}", err);
                return Err(err);
            }
        }
    }

    println!("Result: {output}");

    Ok(())
}
