# Synapse — Architecture Design

> This document captures high-level architecture decisions. Implementation details live in the code.

## Vision

An open-source, extensible Coding Agent. The user describes intent; the Agent reads, writes, and executes.

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Core language | Rust | Single binary, performance, Codex/Goose validated |
| Web UI | TypeScript (React) | Frontend ecosystem, responsive design |
| LLM strategy | Claude First + Provider trait | Deep optimization over broad compatibility |
| Tool system | Built-in + MCP + Skills | Claude Code proven model |
| Security | Permission approval | OS sandbox too fragile for dev toolchains |
| Interaction | CLI/TUI + Server + Web UI | TUI for local, Web for remote/mobile |
| Architecture | Unified binary, modular crates | Simplest deployment, zero IPC overhead |

## Agent Loop

```
User Message → Context Manager → LLM (Claude) → Response Router
                    ↑                                    │
                    │                              ┌─────┴─────┐
                    │                           Text       Tool Calls
                    │                              │           │
                    │                           Output    Permission Gate
                    │                                         │
                    └─────────────── Tool Result ──────── Tool Executor
```

## Crate Dependency Graph

```
synapse (binary)
├── synapse-core
│   ├── synapse-provider
│   ├── synapse-tools
│   ├── synapse-mcp
│   ├── synapse-skills
│   └── synapse-permissions
├── synapse-tui
│   └── synapse-core
└── synapse-server
    └── synapse-core

web/ (TypeScript, connects to synapse-server via WebSocket)
```

## Phases

1. **MVP** — Agent loop + Claude Provider + core tools + CLI mode
2. **TUI** — Ratatui interface + context management + project instructions
3. **Extensibility** — MCP Client + Skill system
4. **Remote** — Axum server + Web UI + auth
5. **Future** — Multi-session, background agents, IDE extensions
