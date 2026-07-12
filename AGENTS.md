# AGENTS.md — patina-agent

## 构建 & 测试

```bash
# 构建
cargo build

# 运行
cargo run

# 运行示例
cargo run --example stream_chat

# 单个测试
cargo test <test_name>
```

项目无 CI、无 lint/formatter 配置、无测试文件。

## 架构要点

- **Rust edition 2024** — 使用 `use` 捕获规则、`unsafe` 语义等 2024 版特性。依赖 `edition = "2024"` 的 crate 需要 nightly 或较新的 stable toolchain。
- **lib + bin** — `src/lib.rs` 作为库暴露 `llm`、`models`、`constant` 模块；`src/main.rs` 是 CLI 入口，直接调用 `structured_ds` 生成 ActionPlan 并打印。
- **两种结构化输出路径**，注意区分：
  - `structured.rs`：使用 OpenAI 原生 `ResponseFormat::JsonSchema`，将 `schemars` 生成的 schema 作为 API 参数传入（OpenAI / 兼容 OpenAI 推理的 provider）。
  - `structured_ds.rs`：使用 `ResponseFormat::JsonObject`，将 JSON Schema 以文本方式注入 system prompt。适用于 **DeepSeek** 等不支持原生 structured output 的模型。
- **模型常量** 定义在 `constant.rs`，当前只有 `deepseek-v4-flash`。
- **全局 Semaphore（5 并发）** 在 `semaphore.rs`，通过 `OnceLock` 延迟初始化。示例 `stream_chat` 使用它控制并发。

## 环境要求

- `.env` 文件需包含 `OPENAI_BASE_URL` 和 `OPENAI_API_KEY`，由 `dotenvy` 自动加载。
- 项目硬编码了 DeepSeek 兼容的 API base URL 用法（`Client::new()` 读取环境变量 `OPENAI_BASE_URL`），未显式设置 `OPENAI_BASE_URL` 时会 panic。

## 代码惯例

- 使用 `anyhow::Result` 作为公共 API 的错误类型。
- 使用 `tracing`（非 `log`）进行日志记录。
- JSON 反序列化失败时通过 `serde_json::from_str` + `map_err(Into::into)` 转为 anyhow error。
- Stream 函数内部使用 `async-stream` 宏 `stream! {}` 定义返回的 `impl Stream`。
- `models/action_plan.rs` 是空文件，实际 ActionPlan 定义在 `models.rs`。新增模型放在 `models/` 目录并用 `models.rs` re-export。
