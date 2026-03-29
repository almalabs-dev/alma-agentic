# alma-agentic

**The agentic framework powering Alma Labs.**

A Rust library for building autonomous AI agents with vectorial memory, tool orchestration, and multi-provider LLM support. Built to power the Alma ecosystem — where AI agents write, review, and improve code.

> **Status:** Phase 1 — Fork pruned and consolidated. Phase 2 (alma-* API encapsulation) in progress.

---

## What is alma-agentic?

alma-agentic is the orchestration layer of Alma Labs' autonomous software factory. It provides:

- **Multi-provider LLM support** — OpenAI, Anthropic, Gemini, Azure, OpenRouter, Ollama, Groq, Together AI
- **Vectorial memory** — Per-project and per-agent memory backed by Qdrant
- **Tool orchestration** — Concurrent tool dispatch with the `#[derive(Tool)]` macro
- **Streaming** — SSE/JSONL streaming with pause/resume
- **MCP support** — Model Context Protocol integration
- **Embeddings** — Unified embedding model interface with distance metrics
- **OpenTelemetry** — Full GenAI Semantic Convention tracing

## Architecture

```
alma-agentic/
  vendor/
    rig-core/          # Rig base retained for providers, tools, embeddings, streaming, MCP, telemetry
    rig-derive/        # Rig proc macros retained while Alma closes its own API boundary
    rig-qdrant/        # Qdrant integration retained as inherited infrastructure
  crates/
    alma-executor/     # Alma-owned HTTP executor and the first Alma API boundary
```

> Phase 2 keeps Rig as the base, but new Alma crates depend on Alma-owned types and internal adapters instead of exposing `rig::*` directly.

## Providers

| Provider | Type | Purpose |
|----------|------|---------|
| OpenAI | Cloud | Primary completions & embeddings |
| Anthropic | Cloud | Primary completions |
| Gemini | Cloud | Primary completions & embeddings |
| Azure OpenAI | Cloud | Enterprise deployments |
| OpenRouter | Gateway | Access to 100+ models (DeepSeek, Qwen, Llama, Mistral...) |
| Ollama | Local | Privacy, offline, development |
| Groq | Cloud | Fast & cheap inference |
| Together AI | Cloud | Open-source model hosting |

**OpenRouter is the strategic gateway** — one integration, access to every model worth using.

## Quick Start

```bash
cargo add rig-core
```

```rust
use rig::{client::CompletionClient, completion::Prompt, providers::openai};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = openai::Client::from_env();

    let agent = client
        .agent("gpt-4o")
        .preamble("You are a helpful assistant.")
        .build();

    let response = agent.prompt("Hello!").await?;
    println!("{response}");

    Ok(())
}
```

## Vector Store

Qdrant integration for dense vector retrieval:

```bash
cargo add rig-qdrant
```

## Roadmap

- [x] **Phase 1** — Fork & prune (remove unused providers and vector stores)
- [ ] **Phase 2** — Encapsulate under `alma-*` API (alma-agent-core, alma-memory, alma-faults, alma-tools)
- [ ] **Phase 3** — Selective replacement of inherited modules

## License

MIT — See [LICENSE](LICENSE) for details.

This project is derived from [Rig](https://github.com/0xPlaygrounds/rig) by Playgrounds Analytics Inc. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for attribution.

---

**Alma Labs** — Building the autonomous software factory.
