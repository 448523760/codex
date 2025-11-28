# Widgets and UI Components

## ChatWidget (`src/chatwidget.rs`)

The `ChatWidget` is the central component of the TUI. It is responsible for:

-   **Conversation History**: Stores and renders the list of `HistoryCell`s (user messages, agent responses, tool calls).
-   **Protocol Handling**: Processes `EventMsg` from the backend to update the UI state (e.g., streaming tokens, showing tool execution).
-   **Input Handling**: Delegates key events to the `BottomPane` or handles global shortcuts (like `Ctrl+C`).
-   **Slash Commands**: Dispatches commands like `/reset`, `/diff`, `/model`.

### Key Methods
-   `handle_codex_event`: Dispatches backend events to specific handlers (e.g., `on_agent_message`, `on_tool_call`).
-   `render`: Draws the chat history and the bottom pane.
-   `submit_user_message`: Sends user input to the backend.

## BottomPane (`src/bottom_pane/`)

The `BottomPane` manages the area at the bottom of the screen, which includes:

1.  **Composer**: The text input area (`ChatComposer` in `src/bottom_pane/chat_composer.rs`).
2.  **Status Indicator**: Shows the current state of the agent (e.g., "Working", "Thinking").
3.  **Popups/Overlays**: Handles temporary views like:
    -   **Approvals**: `approval_overlay.rs`
    -   **File Search**: `file_search_popup.rs`
    -   **Selection Lists**: `list_selection_view.rs` (used for model selection, etc.)

## History Cells (`src/history_cell.rs`)

The chat history is composed of "cells". Each cell represents a distinct item in the conversation:
-   `UserHistoryCell`: A user's message.
-   `AgentMessageCell`: A response from the agent (can be streaming).
-   `ExecCell`: A command execution block.
-   `McpToolCallCell`: A tool call from an MCP server.

Cells implement the `HistoryCell` trait, which defines how they are rendered to `ratatui::text::Line`s.

## Overlays

The `App` can display full-screen overlays that sit on top of the chat interface:
-   **Diff View**: Shows git diffs.
-   **Transcript View**: Shows the full conversation history in an alternate screen buffer.
