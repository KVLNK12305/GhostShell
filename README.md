# GhostShell

GhostShell is a Rust terminal app for two workflows in one place:

- Standup drafting from rough notes
- Live telemetry monitoring in a hidden dashboard view

The active app source lives in `project-ghost/`.

## Repository Layout

- `project-ghost/` - Rust crate (application code)
- `.env.example` - Optional environment template for AI integration
- `.gitignore` - Ignore rules for build output and local state

## Quick Start

1. Install stable Rust.
2. Open a terminal in this repository.
3. Run:

```bash
cd project-ghost
cargo run
```

## What The App Does

- Collects notes in a scratchpad and autosaves them locally.
- Generates a standup draft from your notes.
- Switches into a dashboard mode showing socket/auth snapshots.
- Highlights context-aware anomalies for tracked project ports.

## Controls

- `Ctrl+A` - Generate standup draft
- `Ctrl+G` - Enter dashboard mode
- `Esc` - Exit dashboard mode
- `Ctrl+C` - Quit

## Optional AI Configuration

By default, the app uses a local fallback draft response.
To call a remote model endpoint, set:

```bash
export GHOST_LLM_URL="https://your-llm-endpoint"
export GHOST_LLM_KEY="your-api-key"
```

You can copy values from `.env.example`.

## Future Enhancements

- Add pluggable provider support for multiple LLM APIs.
- Add persistent anomaly history with search/filter.
- Add configurable telemetry polling intervals.
- Add unit tests for socket parsing and anomaly scoring.
- Add packaging targets for Linux/macOS/Windows releases.
