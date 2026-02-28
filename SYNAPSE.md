# Synapse

Cross-platform Coding Agent built in Rust + TypeScript.

## Commands
- `cargo build`: Build all crates
- `cargo test`: Run all tests
- `cargo clippy -- -D warnings`: Lint
- `cargo fmt --check`: Check formatting
- `pnpm --prefix web dev`: Web UI dev server
- `pnpm --prefix web build`: Web UI production build

## Architecture
- **Monorepo**: Cargo workspace (`crates/`) + TypeScript (`web/`)
- **Single binary**: `synapse` — CLI, TUI, and Server in one
- **Agent Core**: ReAct loop (LLM call → tool execution → loop)
- **LLM**: Claude First, Provider trait for extensibility
- **Tools**: Built-in + MCP + Skills
- **Security**: Permission approval system (no OS sandbox)

## Crates
- `synapse` — Binary entry point (CLI subcommands)
- `synapse-core` — Agent loop, context management
- `synapse-provider` — LLM Provider trait + Claude implementation
- `synapse-tools` — Built-in tools (Read, Write, Edit, Bash, Glob, Grep...)
- `synapse-mcp` — MCP Client
- `synapse-skills` — Skill loading and execution
- `synapse-permissions` — Permission approval engine
- `synapse-tui` — TUI interface (Ratatui)
- `synapse-server` — HTTP + WebSocket API (Axum)

## Code Style
- Rust 2021 edition, stable toolchain
- `cargo fmt` for formatting, `clippy` for linting
- No `unwrap()` in library crates — use `Result`/`?`
- `#[async_trait]` for async trait definitions
- Prefer `thiserror` for error types
- Tests: `#[tokio::test]` for async, `insta` for snapshots

## Rules
- DO NOT edit `target/` or `web/dist/`
- Keep crate boundaries clean — no circular dependencies
- Provider trait must remain LLM-agnostic
- Tool trait must remain execution-environment-agnostic
