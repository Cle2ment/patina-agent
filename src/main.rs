use anyhow::Ok;
use patina_agent::{
    constant::DEEPSEEK_V4_FLASH_MODEL, llm::structured_ds::chat_complete_structured_ds,
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

    let plan = chat_complete_structured_ds(
        DEEPSEEK_V4_FLASH_MODEL,
        "我想去观看今年的美加墨世界杯，应该如何安排？",
    )
    .await?;

    println!("Response: {plan:#?}");

    Ok(())
}
