# Project Ghost

Project Ghost is a Rust terminal application that combines:

- A standup note + draft assistant
- A live shadow dashboard for basic host telemetry

## Purpose

Use one terminal app to:

- Capture rough standup notes quickly
- Produce a structured update (`Yesterday`, `Today`, `Blockers`)
- Watch socket/auth activity in dashboard mode
- Surface context-aware anomalies based on tracked ports

## Stack

- `tokio` for async runtime and background tasks
- `ratatui` + `crossterm` for terminal UI
- `reqwest` for optional remote LLM calls
- `serde` / `serde_json` for local note persistence

## Project Structure

- `src/main.rs` - App loop, keybindings, and mode switching
- `src/app.rs` - State model, events, context mapping, anomaly scoring
- `src/tasks.rs` - Background monitors, note persistence, LLM call/fallback
- `src/ui.rs` - Decoy and dashboard rendering

## Prerequisites

- Stable Rust toolchain installed
- Terminal that supports ANSI escape sequences

## Run

From this directory:

```bash
cargo run
```

From repository root:

```bash
cd project-ghost
cargo run
```

## Controls

- `Ctrl+A` - Generate standup draft
- `Ctrl+G` - Enter shadow dashboard
- `Esc` - Return to standup view
- `Ctrl+C` - Quit application

## Data Files

- `.ghost_notes.json` - Local autosaved notes (created at runtime)

## Optional AI Integration

If `GHOST_LLM_URL` and `GHOST_LLM_KEY` are set, draft generation uses your remote endpoint.
If either is missing, the app uses an internal fallback response so the workflow still works.

Example:

```bash
export GHOST_LLM_URL="https://your-llm-endpoint"
export GHOST_LLM_KEY="your-api-key"
```

## Future Enhancements

- Add configurable project profiles from a JSON/TOML file.
- Add alert suppression and severity tuning rules.
- Add richer auth-log parsing and event classification.
- Add integration tests for monitors and parser edge cases.
- Add export options for standup drafts (Markdown/plain text).
