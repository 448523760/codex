# `tui/src/tui.rs`

This module provides the low-level interface to the terminal using `crossterm` and `ratatui`. It abstracts away the details of raw mode, event polling, and drawing.

## Struct `Tui`

```rust
pub struct Tui {
    pub(crate) terminal: Terminal, // CustomTerminal<CrosstermBackend<Stdout>>
    frame_schedule_tx: tokio::sync::mpsc::UnboundedSender<Instant>,
    draw_tx: tokio::sync::broadcast::Sender<()>,
    // ...
}
```

### Initialization (`init`)

The `init` function:
1.  Checks if stdout is a terminal.
2.  Calls `set_modes()`:
    *   Enables bracketed paste.
    *   Enables raw mode.
    *   Enables keyboard enhancement flags (for better key modifier support).
    *   Enables focus change events.
3.  Sets a custom panic hook to restore the terminal on crash.
4.  Creates the `CrosstermBackend` and wraps it in `CustomTerminal`.

### Event Stream (`event_stream`)

Returns a `Stream` of `TuiEvent`. It merges events from:
1.  `crossterm::event::EventStream`: Key presses, resize events, paste events, focus changes.
2.  `draw_tx`: Internal signals to trigger a redraw.

It handles `Ctrl+Z` (suspend) on Unix systems internally.

### Drawing (`draw`)

The `draw` method wraps Ratatui's drawing logic.
*   It uses `stdout().sync_update()` to ensure atomic updates (preventing tearing).
*   It handles viewport management:
    *   If the content grows, it scrolls the terminal region up to make room.
    *   It manages the "inline" viewport area.
*   It renders pending history lines (lines that are "committed" to the scrollback buffer) before drawing the active frame.

### Alternate Screen

*   `enter_alt_screen()`: Switches to the terminal's alternate screen buffer (full-screen mode). Used for diffs and the transcript view.
*   `leave_alt_screen()`: Restores the main screen and the inline viewport.

## Frame Scheduling

To avoid excessive drawing (e.g., during rapid streaming), `Tui` uses a background task (`spawn_frame_scheduler`).
*   `FrameRequester::schedule_frame()` sends a request.
*   The scheduler coalesces multiple requests and emits a single draw signal via `draw_tx` at the next deadline.

## `CustomTerminal`

The `Terminal` type alias refers to `crate::custom_terminal::Terminal`. This is a wrapper around Ratatui's `Terminal` that adds support for:
*   **Inline Viewport**: Rendering at the bottom of the current output rather than taking over the whole screen.
*   **History Insertion**: Printing lines directly to stdout (above the viewport) so they become part of the terminal's scrollback history.
