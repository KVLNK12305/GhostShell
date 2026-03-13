# GhostShell

GhostShell is a Rust terminal application with two modes:

- `Decoy mode`: standup notes + draft generation
- `Ghost mode`: live socket and auth-log telemetry dashboard

The application source lives in `project-ghost/`.

## What It Actually Does Today

- Captures notes in a scratchpad.
- Autosaves notes to `project-ghost/.ghost_notes.json`.
- Generates standup text (`Yesterday`, `Today`, `Blockers`) with `Ctrl+A`.
- Switches to a telemetry dashboard with `Ctrl+G`.
- Streams socket snapshots and auth-log tails in the background.
- Highlights context-aware anomalies based on tracked project ports.

## What It Does Not Do (Yet)

- No `Tab` panel switching.
- No custom keybinding config file.
- No extended-attribute/xattr hidden storage.
- No process renaming or terminal history wiping.
- No built-in `lsof` monitor.

## Quick Start

1. Install Rust stable.
2. From repository root:

```bash
cd project-ghost
cargo run
```

## Controls

- `Ctrl+A`: generate standup draft
- `Ctrl+G`: enter ghost dashboard
- `Esc`: return to decoy mode
- `Ctrl+C`: quit

## Optional AI Setup

If these variables are set, draft generation uses your remote endpoint:

```bash
export GHOST_LLM_URL="https://your-llm-endpoint"
export GHOST_LLM_KEY="your-api-key"
```

If not set, the app uses a local fallback draft so the flow still works.

## Project Layout

```text
project-ghost/
  Cargo.toml
  src/
    main.rs   # event loop and key handling
    app.rs    # app state and anomaly scoring
    tasks.rs  # monitors, persistence, and LLM call
    ui.rs     # decoy and ghost mode rendering
```

## Future Enhancements

- Configurable polling intervals via environment variables.
- Pluggable LLM providers and model selection.
- Persistent anomaly history and filtering.
- Additional telemetry sources (`lsof`, process snapshots, disk/network stats).
- Test coverage for parser and anomaly-scoring paths.

## Zig Additions Plan

Goal: add Zig where it provides clear systems-level value without replacing the Rust UI/runtime.

Phase 1: Foundation
- Add `zig/` workspace with a minimal CLI utility (`ghost_probe`) and `build.zig`.
- Keep Rust as the orchestrator; call Zig binary from Rust tasks for one isolated function.
- Choose first target: fast socket snapshot parsing or log normalization.

Phase 2: Integration
- Add a Rust wrapper module (`project-ghost/src/zig_bridge.rs`) to execute Zig tools safely.
- Define stable JSON I/O contracts between Rust and Zig (`stdin` request, `stdout` response).
- Add graceful fallback to pure Rust path if Zig binary is missing.

Phase 3: Performance + Reliability
- Benchmark Rust-only vs Rust+Zig path on parsing-heavy workloads.
- Add integration tests validating parity between Rust and Zig outputs.
- Add timeout and error classification for Zig subprocess failures.

Phase 4: Packaging
- Add build docs for Zig (`zig build`) and release flow.
- Produce release artifacts that bundle both `project-ghost` and Zig helper binary.
- Verify Linux/macOS support first; extend to Windows after parity checks.

Success Criteria
- No regressions in existing controls or UI flow.
- Zig path is optional and fault-tolerant.
- Measurable performance benefit on at least one hot path.

## Notes

- This project is for defensive/observability workflows.
- Respect local policy and legal requirements when monitoring systems.
