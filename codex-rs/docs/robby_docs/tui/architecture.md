# Architecture

The Codex TUI follows a standard event-driven architecture typical of Ratatui applications, but with some specific adaptations for async operations and streaming.

## Main Loop (`App::run`)

The main entry point is `App::run` in `src/app.rs`. It sets up the event channels and enters a loop that selects between:

1.  **App Events (`AppEvent`)**: Internal events triggered by the application logic, backend responses, or background tasks.
2.  **TUI Events (`TuiEvent`)**: Raw terminal events (keypresses, resize, paste) captured by `src/tui.rs`.

```rust
while select! {
    Some(event) = app_event_rx.recv() => {
        app.handle_event(tui, event).await?
    }
    Some(event) = tui_events.next() => {
        app.handle_tui_event(tui, event).await?
    }
} {}
```

## Event Handling

### App Events
Defined in `src/app_event.rs`, these include:
-   `CodexEvent`: Protocol events from the backend (e.g., agent messages, tool calls).
-   `CodexOp`: Operations to be sent to the backend.
-   `InsertHistoryCell`: Adds a new item to the chat history.
-   `DiffResult`: Result of a git diff operation.
-   `UpdateModel`, `UpdateReasoningEffort`: Configuration changes.

### TUI Events
Defined in `src/tui.rs`, these are wrappers around Crossterm events:
-   `Key`: Keyboard input.
-   `Paste`: Bracketed paste input.
-   `Draw`: Trigger for rendering a frame.

## State Management

The `App` struct holds the global state:
-   `chat_widget`: The main UI component.
-   `config`: Current configuration.
-   `auth_manager`: Authentication state.
-   `overlay`: Current overlay (e.g., diff view, history transcript).

## Terminal Abstraction (`Tui`)

The `Tui` struct in `src/tui.rs` manages the terminal backend. It:
-   Initializes raw mode and mouse capture.
-   Provides an async stream of `TuiEvent`.
-   Manages the "inline" viewport vs. "alternate screen" modes.
-   Handles frame scheduling to coalesce draw requests.

## Backend Communication

The TUI communicates with the Codex backend (managed by `codex-core`) via channels.
-   **Outgoing**: `codex_op_tx` (in `ChatWidget`) sends `Op` enums to the backend agent.
-   **Incoming**: The backend sends `Event` objects, which are wrapped in `AppEvent::CodexEvent` and processed by `ChatWidget::handle_codex_event`.
