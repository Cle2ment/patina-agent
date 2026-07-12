# Patina Agent

Rust 异步 AI Agent 框架，基于 `async-openai`，提供 LLM 交互原语。

## 功能

- **同步对话** (`chat_complete`) — 标准 Chat Completion
- **流式对话** (`chat_stream_with_retry`) — 带指数退避重试的 streaming
- **结构化输出 — OpenAI 原生** (`chat_complete_structured`) — 通过 `ResponseFormat::JsonSchema` 约束输出为 `ActionPlan`
- **结构化输出 — DeepSeek** (`chat_complete_structured_ds`) — 将 JSON Schema 作为文本注入 system prompt，适配不支持原生 structured output 的模型
- **并发控制** (`Semaphore`) — 全局信号量限制同时进行的 LLM 请求数（默认 5）

## 快速开始

### 环境变量

创建 `.env` 文件：

```
OPENAI_BASE_URL=https://api.deepseek.com/v1
OPENAI_API_KEY=sk-xxx
```

### 构建 & 运行

```bash
cargo build
cargo run
```

### 运行示例

```bash
cargo run --example stream_chat
```

## 架构

```
src/
├── main.rs                 # 入口：调用 structured_ds 生成行动计划
├── lib.rs                  # 库根
├── constant.rs             # 模型常量
├── models.rs               # 数据模型（ActionPlan）
├── models/
│   └── action_plan.rs      # （预留，暂未启用）
└── llm/
    ├── complete.rs          # 基础 Chat Completion
    ├── stream.rs            # 流式 + 重试
    ├── structured.rs        # OpenAI 原生 JSON Schema 结构化输出
    ├── structured_ds.rs     # DeepSeek JSON Object 模式 + prompt 注入 Schema
    └── semaphore.rs         # 全局并发信号量
```

## 依赖

| 库 | 用途 |
|---|---|
| `async-openai` | LLM API 客户端 |
| `schemars` | 从 Rust 类型生成 JSON Schema |
| `backon` | 指数退避重试 |
| `tokio` | 异步运行时 |
| `tracing` | 结构化日志 |
| `dotenvy` | 加载 `.env` 环境变量 |

## 版权
Copyright (C) 2026  Clement CHEN.