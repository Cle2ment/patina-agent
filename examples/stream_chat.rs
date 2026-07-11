use patina_agent::{
    constant::DEEPSEEK_V4_FLASH_MODEL,
    llm::{semaphore::get_semaphore, stream::chat_stream_with_retry},
};
use tokio::task::JoinSet;
use tracing::{Instrument, Level};
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

    let prompts = vec![
        "用三句话解释Rust的所有权系统。",
        "什么是异步编程？",
        "解释一下机器学习中的过拟合问题。",
        "如何在Rust中实现一个简单的Web服务器？",
        "什么是区块链技术，它是如何工作的？",
        "解释一下量子计算的基本原理。",
        "什么是人工智能，它与机器学习有什么区别？",
    ];

    let mut set = JoinSet::new();
    for prompt in prompts {
        let span = tracing::info_span!("Chat", prompt = prompt);
        set.spawn(
            async move {
                let permit = get_semaphore().acquire().await?;
                let output = chat_stream_with_retry(
                    DEEPSEEK_V4_FLASH_MODEL,
                    Some("你是一个全能Agent。"),
                    prompt,
                )
                .await?;
                drop(permit);
                Ok::<_, anyhow::Error>((prompt, output))
            }
            .instrument(span),
        );
    }

    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok((prompt, result))) => tracing::info!("\n{prompt}\n{result}"),
            Ok(Err(err)) => tracing::error!("Task panicked: {err}"),
            Err(err) => tracing::error!("Task join error: {err}"),
        }
    }

    Ok(())
}
