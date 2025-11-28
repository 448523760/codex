# `tui/src/chatwidget.rs`

`ChatWidget` is the primary UI component of the application. It encapsulates the conversation state, the input area (`BottomPane`), and the logic for interacting with the Codex backend.

## Struct `ChatWidget`

```rust
pub(crate) struct ChatWidget {
    app_event_tx: AppEventSender,
    codex_op_tx: UnboundedSender<Op>, // Channel to the backend agent
    bottom_pane: BottomPane,          // Input and status area
    active_cell: Option<Box<dyn HistoryCell>>, // The cell currently being streamed/updated
    // ... state for rate limits, streaming, interrupts, etc.
}
```

## Core Responsibilities

### 1. Backend Communication
*   **Sending Operations**: `submit_op` and `codex_op_tx` are used to send commands (`Op`) to the backend (e.g., user input, undo, slash commands).
*   **Receiving Events**: `handle_codex_event` processes `EventMsg` from the backend. This is a massive match statement handling everything from `AgentMessageDelta` (streaming text) to `ToolCall` events.

### 2. Streaming & Rendering
*   **`handle_streaming_delta`**: Receives text chunks from the agent. It uses a `StreamController` to buffer and animate the output.
*   **`active_cell`**: Holds the `HistoryCell` that is currently being modified (e.g., a streaming agent message or a running exec command).
*   **`flush_active_cell`**: When a turn ends or the type of content changes (e.g., text -> tool call), the active cell is "flushed" (sent to `App` to be permanently added to the history list).

### 3. Input Handling
*   **`handle_key_event`**: Delegates keys to `bottom_pane`.
*   **`submit_user_message`**: Called when the user presses Enter. It constructs a `UserMessage` and sends it to the backend. It also handles local shell commands (starting with `!`).
*   **Slash Commands**: `dispatch_command` handles commands like `/reset`, `/diff`, `/model`.

### 4. Tool Execution UI
*   **`handle_exec_begin_now`**: Starts rendering a command execution block (`ExecCell`).
*   **`handle_exec_end_now`**: Updates the block with the exit code and output.
*   **`handle_mcp_tool_call_...`**: Similar logic for MCP tool calls.

### 5. Review Mode
*   **`on_entered_review_mode`**: Displays a banner when review mode starts.
*   **`on_exited_review_mode`**: Displays the review summary/findings.

## Helper Components

*   **`StreamController`**: Manages the token streaming animation.
*   **`InterruptManager`**: Queues UI events that occur while the UI is busy (e.g., ensuring `ExecEnd` doesn't process before `ExecBegin` if they arrive close together).
*   **`RateLimitWarningState`**: Tracks token usage and generates warnings.

## Initialization

*   **`new`**: Starts a fresh session. Spawns the backend agent using `spawn_agent`.
*   **`new_from_existing`**: Resumes a session. Spawns the agent using `spawn_agent_from_existing` and replays history.
