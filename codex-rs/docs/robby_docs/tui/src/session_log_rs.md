# `tui/src/session_log.rs`

This module handles the recording of user sessions to a JSONL file. This is useful for debugging, telemetry, and analyzing agent behavior.

## Global Logger

The logger is a global singleton `LOGGER` (using `LazyLock` and `OnceLock`), ensuring that session logging can be accessed from anywhere in the TUI without passing a handle around.

## Key Functions

### `maybe_init(config: &Config)`

Initializes the session logger if the environment variable `CODEX_TUI_RECORD_SESSION` is set to true.
*   It determines the log file path (either from `CODEX_TUI_SESSION_LOG_PATH` or a default timestamped file in the log directory).
*   It writes a `session_start` header record containing metadata like the current working directory, model, and provider.

### `log_inbound_app_event(event: &AppEvent)`

Logs events flowing *into* the TUI (from the user or the backend).
*   **`AppEvent::CodexEvent`**: Logged as `to_tui` / `codex_event`.
*   **`AppEvent::InsertHistoryCell`**: Logs that a new chat cell was added.
*   **`AppEvent::StartFileSearch`**: Logs user search queries.
*   **`AppEvent::FileSearchResult`**: Logs the number of matches found.

### `log_outbound_op(op: &Op)`

Logs operations sent *from* the TUI to the backend (e.g., `Op::StartSession`, `Op::SubmitFeedback`).

## Log Format

The log is a JSON Lines (JSONL) file. Each line is a JSON object with at least:
*   `ts`: Timestamp (RFC3339).
*   `dir`: Direction (`meta`, `to_tui`, `from_tui`).
*   `kind`: The type of event (e.g., `session_start`, `codex_event`).
*   `payload`: (Optional) The detailed data of the event.
