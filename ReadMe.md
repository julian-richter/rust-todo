# Rust WebAssembly Todo App

A simple todo application built with Rust and WebAssembly (WASM) to learn WASM development and improve Rust skills. Rust is still new to me, so this project focuses on hands-on practice.

## Development Environment Setup

### Loading Environment Variables
Load `.env` variables into your Bash/Zsh shell **without external dependencies**.

```bash
set -a && source .env && set +a
```

**Why this way?**
- No dependencies: Pure shell—no crates/tools just to read `.env`.
- `set -a`: Auto-exports variables for child processes (e.g., `cargo run`).
- `source .env`: Executes `KEY=VALUE` lines in current session.
- `set +a`: Disables auto-export afterward.

**Steps:**
1. Copy `.env.example` → `.env` and edit values.
2. Run the command above.
3. Verify: `echo $APP_SERVICE_NAME` (should show "todo").

**Pro tip:** Add to `~/.bashrc` or `~/.zshrc`:
```bash
alias loadenv='set -a && source .env && set +a'
```

## Project Goals
- Learn Rust WebAssembly fundamentals
- Build a browser-based todo app
- Practice Rust ownership, borrowing, and structs
- No heavy frameworks—just vanilla WASM + JS interop