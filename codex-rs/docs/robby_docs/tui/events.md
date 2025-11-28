# Event System

The application is driven by an asynchronous event loop.

## AppEvent (`src/app_event.rs`)

`AppEvent` is the primary enum for internal application events.

```rust
pub enum AppEvent {
    // Backend Events
    CodexEvent(Event),
    CodexOp(Op),

    // UI Updates
    InsertHistoryCell(Box<dyn HistoryCell>),
    DiffResult(String),
    
    // Configuration
    UpdateModel(String),
    UpdateReasoningEffort(Option<ReasoningEffort>),
    
    // Animation
    StartCommitAnimation,
    StopCommitAnimation,
    CommitTick,
    
    // Lifecycle
    ExitRequest,
    NewSession,
    
    // ... and more
}
```

## TuiEvent (`src/tui.rs`)

`TuiEvent` captures raw input from the terminal.

```rust
pub enum TuiEvent {
    Key(KeyEvent),
    Paste(String),
    Draw,
}
```

## Event Flow

1.  **Input**: The user presses a key. `crossterm` captures it, and `Tui::event_stream` yields a `TuiEvent::Key`.
2.  **Processing**: `App::run` receives the event and calls `App::handle_tui_event`.
3.  **Dispatch**: If the key is a shortcut (e.g., `Ctrl+C`), it's handled immediately. Otherwise, it's passed to `ChatWidget::handle_key_event`.
4.  **Action**: The `ChatWidget` might update the input buffer, trigger a slash command, or send an `Op` to the backend via `codex_op_tx`.
5.  **Backend Response**: The backend processes the `Op` and sends an `Event` back.
6.  **Update**: `App::run` receives the `AppEvent::CodexEvent`, passes it to `ChatWidget::handle_codex_event`, which updates the UI state and requests a redraw.
