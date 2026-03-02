# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust web application for DeFi asset management — tracks crypto fund balances and allocations across different schemes. Built with Axum (async web framework) + HTMX frontend.

## Build & Run Commands

```bash
cargo build --release          # Build release binary
cargo build                    # Build debug binary
cargo run                      # Run dev server (binds 0.0.0.0:3999)
cargo clippy                   # Lint
cargo fmt                      # Format code
```

There are no tests in this project currently.

### Environment Variables

- `STORAGE_PATH` — binary storage file location (default: `./storage.bin`)
- `RUST_LOG` — log level (default: `info`)

### Docker

```bash
docker build -t crypto-assets .   # Multi-stage build (rust:1.84-slim → debian:bookworm-slim)
```

Health check endpoint: `GET /ping` → "pong"

## Architecture

### Dual API Pattern

Every feature exposes two parallel API layers:
- **REST JSON** at `/api/v1/` — programmatic access
- **HTMX HTML** at `/api/v1-htmx/` — returns server-rendered HTML fragments for the frontend

Both share the same business logic in `methods.rs` files.

### Source Layout

```
src/
├── main.rs                    # Entry point, storage load/save (bincode, 60s interval)
├── app_state.rs               # AppState: Arc<Mutex<Storage>> shared across handlers
├── models/
│   ├── models.rs              # Token, Scheme, Balance, Allocation structs
│   └── storage.rs             # Storage struct with bincode persistence
└── routers/
    ├── mod.rs                 # Router composition
    ├── utils.rs               # HTTP response helpers
    ├── index/                 # Static asset serving (/assets/*)
    ├── balances/              # CRUD for token balances
    │   ├── methods.rs         # BalanceStore trait with business logic
    │   ├── rest.rs            # JSON handlers
    │   └── htmx.rs            # HTML fragment handlers
    ├── allocations/           # CRUD for scheme allocations
    │   └── (same structure)
    └── views/                 # Aggregated analytics (TokenInfo)
        └── (same structure)
```

### Key Patterns

- **Shared state**: `Arc<Mutex<Storage>>` passed to all handlers via Axum's state extraction
- **Persistence**: bincode serialization to `storage.bin`, auto-saved every 60 seconds by a background Tokio task
- **Templates**: HTML files in `templates/` with placeholder replacement (not a template engine)
- **Frontend**: Single-page `assets/index.html` using HTMX for dynamic updates; Sakura CSS for styling
- **Builder pattern**: `TokenInfo` uses `derive_builder` for construction in views

### Data Model

- **Token** — cryptocurrency symbol + exchange rate
- **Scheme** — allocation category/strategy
- **Balance** — current holdings per token
- **Allocation** — token amount assigned to a specific scheme

Tokens and schemes are auto-created when first referenced in a balance or allocation.

## CI/CD

GitHub Actions (`.github/workflows/build.yml`): builds Docker image on push to master, pushes to `ghcr.io` tagged with `latest` and git SHA.

## Project Rules

### Git Commits
- Do not add Co-Authored-By lines to commit messages

### Pull Requests
- Do not add Claude as co-author or mention Claude in PR descriptions
