# `tui/src/app.rs`

`App` is the central state container and event orchestrator for the TUI. It manages the lifecycle of the application, processes events, and coordinates between the UI (`ChatWidget`) and the backend (`ConversationManager`).

## Struct `App`

```rust
pub(crate) struct App {
    pub(crate) server: Arc<ConversationManager>,
    pub(crate) app_event_tx: AppEventSender,
    pub(crate) chat_widget: ChatWidget,
    pub(crate) auth_manager: Arc<AuthManager>,
    pub(crate) config: Config,
    // ... other state fields
}
```

Key fields:
*   `server`: The `ConversationManager` that handles the core Codex logic.
*   `chat_widget`: The main UI component.
*   `app_event_tx`: Channel for sending internal events.
*   `overlay`: Manages full-screen overlays (like the Diff view or Transcript view).
*   `backtrack`: State for the "Esc to backtrack" feature.

## Main Loop (`App::run`)

The `run` method is the heart of the application. It:

1.  **Initialization**:
    *   Sets up the `AppEvent` channel.
    *   Handles model migration prompts if necessary.
    *   Initializes the `ConversationManager`.
    *   Creates the `ChatWidget` (either fresh or from a resumed session).
    *   Spawns background tasks (e.g., Windows world-writable scan).

2.  **Event Loop**:
    It uses `tokio::select!` to listen for two types of events:
    *   `app_event_rx.recv()`: Internal application events (`AppEvent`).
    *   `tui_events.next()`: User input events (`TuiEvent`).

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

### `handle_event` (`AppEvent`)

Processes logical events:
*   `NewSession`: Resets the `ChatWidget` for a new conversation.
*   `InsertHistoryCell`: Adds a new item to the chat history (and the transcript overlay if active).
*   `CodexEvent`: Forwards backend events to `chat_widget.handle_codex_event`.
*   `DiffResult`: Shows the diff overlay.
*   `UpdateModel`, `UpdateReasoningEffort`: Updates configuration.
*   `Open...`: Opens various popups (Reasoning, Full Access, Feedback, etc.) by delegating to `chat_widget`.

### `handle_tui_event` (`TuiEvent`)

Processes terminal input:
*   **Overlays**: If an overlay is active (`self.overlay.is_some()`), events are routed to `handle_backtrack_overlay_event`.
*   **Key Events**:
    *   `Ctrl+t`: Toggles the Transcript overlay.
    *   `Esc`: Handles backtracking logic (if composer is empty).
    *   Other keys: Forwarded to `chat_widget.handle_key_event`.
*   **Paste**: Forwarded to `chat_widget`.
*   **Draw**: Triggers a render of the `chat_widget`.

## Key Features Implemented Here

*   **Model Migration**: Checks if the user is on a deprecated model and prompts for upgrade (`handle_model_migration_prompt_if_needed`).
*   **Windows Sandbox Checks**: Spawns a background thread to check for world-writable directories on Windows (`spawn_world_writable_scan`).
*   **Commit Animation**: Manages a background thread that sends `CommitTick` events to animate streaming output (`StartCommitAnimation`, `StopCommitAnimation`).
