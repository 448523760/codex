# `tui/src/bottom_pane/mod.rs`

The `BottomPane` module manages the lower section of the TUI, which handles user input, status display, and temporary overlays (popups).

## Struct `BottomPane`

```rust
pub(crate) struct BottomPane {
    composer: ChatComposer,
    view_stack: Vec<Box<dyn BottomPaneView>>,
    status: Option<StatusIndicatorWidget>,
    queued_user_messages: QueuedUserMessages,
    // ...
}
```

### Components

1.  **`ChatComposer`**: The text input widget. It handles typing, pasting, and history navigation (Up/Down).
2.  **`StatusIndicatorWidget`**: An animated widget shown when the agent is "Working". It replaces or sits above the composer.
3.  **`view_stack`**: A stack of `BottomPaneView` trait objects. This allows modal dialogs (like the approval popup or file search) to overlay the composer.
4.  **`queued_user_messages`**: Displays messages that the user has typed but haven't been sent yet (because the agent is busy).

## Key Responsibilities

### Input Routing (`handle_key_event`)

*   If `view_stack` is not empty, the key event is sent to the **active view** (the top of the stack).
*   If the stack is empty, the key event is sent to the **composer**.
*   **Esc Handling**:
    *   If a view is active, Esc typically closes it.
    *   If a task is running, Esc sends an interrupt signal.

### View Management

*   `push_view`: Adds a new view (e.g., `ApprovalOverlay`, `ListSelectionView`) to the stack.
*   `active_view`: Returns the current top view.
*   `on_ctrl_c`: Handles `Ctrl+C`. If a view is open, it might close it. If the composer has text, it clears it. If empty, it shows a quit hint.

### Rendering

The `BottomPane` renders itself by checking the state:
1.  If a view is active, render the view.
2.  Otherwise, render the `StatusIndicator` (if active), `QueuedUserMessages`, and the `ChatComposer`.

## Sub-modules

*   `chat_composer.rs`: The text input logic (using `tui-textarea`).
*   `approval_overlay.rs`: The "Allow/Deny" popup for tool execution.
*   `list_selection_view.rs`: Generic list picker (used for `/model`, `/feedback`).
*   `file_search_popup.rs`: The `@` file search popup.
*   `status_indicator_widget.rs`: The "Working..." spinner.
