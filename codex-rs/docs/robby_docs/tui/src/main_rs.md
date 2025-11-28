# `tui/src/main.rs`

This is the binary entry point for the TUI application.

## Function `main`

The `main` function is responsible for:

1.  **Argument Parsing**: It uses `clap` to parse command-line arguments into a `TopCli` struct.
    *   `TopCli` wraps the inner `Cli` config and `CliConfigOverrides`.
2.  **Arg0 Dispatch**: It calls `codex_arg0::arg0_dispatch_or_else` to handle special execution modes (like being called as a specific tool binary).
3.  **Execution**: It calls `codex_tui::run_main` (defined in `lib.rs`) to start the actual TUI application loop.
4.  **Exit Reporting**: If the session consumed tokens, it prints the final token usage to stdout after the TUI exits.

## Struct `TopCli`

A wrapper struct that combines configuration overrides (like `--model`) with the main TUI CLI arguments.
