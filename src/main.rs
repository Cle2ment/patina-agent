use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use crate::constant::DEEPSEEK_V4_FLASH_MODEL;
use crate::llm::complete::chat_complete;

mod llm;
mod constant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let url = std::env::var("OPENAI_BASE_URL")?;
    println!("{url}");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let content = chat_complete(
        DEEPSEEK_V4_FLASH_MODEL,
        Some("你是一个全能Agent。"),
        "中国首都是哪里？"
    ).await?;

    println!("{content}");

    Ok(())
}
