# alma-agentic

**The agentic framework powering Alma Labs.**

A Rust library for building autonomous AI agents with vectorial memory, tool orchestration, and multi-provider LLM support. Built to power the Alma ecosystem — where AI agents write, review, and improve code.

> **Status:** Phase 2 in progress — `alma-executor` and `alma-memory` boundaries established. Conversation persistence and federation pending.

---

## What is alma-agentic?

alma-agentic is the orchestration layer of Alma Labs' autonomous software factory. It provides:

- **Multi-provider LLM support** — OpenAI, Anthropic, Gemini, Azure, OpenRouter, Ollama, Groq, Together AI
- **Vectorial memory** — Per-project and per-agent memory backed by Qdrant
- **Tool orchestration** — Concurrent tool dispatch
- **Streaming** — SSE/JSONL streaming with pause/resume
- **MCP support** — Model Context Protocol integration
- **Embeddings** — Unified embedding model interface with distance metrics
- **OpenTelemetry** — Full GenAI Semantic Convention tracing

## Architecture

```
alma-agentic/
  crates/
    alma-executor/   # HTTP executor — Alma API boundary, zero vendor types in routes
    alma-memory/     # Vectorial memory — Alma types, Qdrant confined to internal adapter
  vendor/
    (provider infrastructure)
```

Alma crates depend on Alma-owned types. All vendor dependencies are hidden behind internal adapters.

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

## HTTP API (alma-executor)

`alma-executor` exposes a provider-agnostic HTTP interface. The active provider is configured via environment variables.

```bash
OPENROUTER_API_KEY=<key> \
ALMA_DEFAULT_MODEL=anthropic/claude-sonnet-4 \
cargo run -p alma-executor
```

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/v1/prompt` | POST | Single-turn prompt |
| `/v1/chat` | POST | Multi-turn chat with history |
| `/v1/stream` | POST | SSE streaming response |

## Memory (alma-memory)

`alma-memory` provides Alma-typed memory operations backed by Qdrant. `qdrant-client` is confined to an internal adapter — consumers of `alma-memory` never import vendor types.

```bash
QDRANT_URL=http://localhost:6334 \
ALMA_MEMORY_COLLECTION=alma \
cargo run -p alma-executor
```

## Roadmap

- [x] **Phase 1** — Prune vendor base, establish workspace structure
- [x] **Phase 2a** — `alma-executor`: Alma API boundary, vendor confined to adapter
- [x] **Phase 2b** — `alma-memory`: Alma memory types, Qdrant adapter, wired into system state
- [ ] **Phase 2c** — Conversation persistence
- [ ] **Phase 2d** — `alma-federation`
- [ ] **Phase 2e** — Specialized agents

## License

MIT — See [LICENSE](LICENSE) for details.

See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for third-party attributions.

---

**Alma Labs** — Building the autonomous software factory.
