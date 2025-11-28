# `tui/src/history_cell.rs`

This module defines the `HistoryCell` trait and its implementations. A "cell" represents a single block of content in the chat history, such as a user message, an agent response, or a tool execution result.

## Trait `HistoryCell`

```rust
pub(crate) trait HistoryCell: std::fmt::Debug + Send + Sync + Any {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>>;
    fn desired_height(&self, width: u16) -> u16;
    fn transcript_lines(&self, width: u16) -> Vec<Line<'static>>;
    // ...
}
```

*   `display_lines`: Renders the cell content into Ratatui `Line`s for the main chat view.
*   `transcript_lines`: Renders the content for the full-screen transcript view (often identical to `display_lines`).

## Implementations

### `UserHistoryCell`
Displays a user's message.
*   Prefix: `› ` (bold, dim).
*   Wraps text using `word_wrap_lines`.

### `AgentMessageCell`
Displays a text response from the agent.
*   Prefix: `• ` (dim) for the first line, indentation for subsequent lines.
*   Supports streaming updates (via `is_stream_continuation`).

### `ReasoningSummaryCell`
Displays the "reasoning" block (chain of thought) from models that support it.
*   Renders as a collapsed/summarized block (dimmed, italic).
*   Can parse "Experimental" reasoning formats.

### `McpToolCallCell`
Displays an MCP tool invocation.
*   **State**: Can be "Calling" (spinner) or "Called" (result).
*   **Rendering**:
    *   Shows the tool name and arguments.
    *   Shows the result (success/failure) with a tree-like structure (`└`).
    *   Handles complex content like images or embedded resources.

### `PatchHistoryCell`
Displays a summary of file changes (for `apply_patch` tool).
*   Delegates to `create_diff_summary` in `diff_render.rs`.

### `SessionInfoCell` / `SessionHeaderHistoryCell`
Displays the session banner at the top of the chat.
*   Shows the Codex version, current model, and working directory.
*   Renders inside a box border.

### `PlanUpdateCell`
Displays a plan update from the agent.
*   Renders steps as a checklist (`□`, `✔`).

### `UpdateAvailableHistoryCell`
Shows a notification if a new version of Codex is available.

## Helper Functions

*   `with_border`: Wraps lines in a box.
*   `padded_emoji`: Adds a hair space after an emoji for better rendering.
