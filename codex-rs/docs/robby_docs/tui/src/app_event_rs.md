# `tui/src/app_event.rs`

This file defines the `AppEvent` enum, which serves as the internal message bus for the application. It decouples the UI components from the backend and from each other.

## Enum `AppEvent`

```rust
pub(crate) enum AppEvent {
    CodexEvent(Event),
    CodexOp(codex_core::protocol::Op),
    // ...
}
```

### Categories of Events

1.  **Backend Interaction**:
    *   `CodexEvent(Event)`: Incoming events from the `codex-core` backend (e.g., `AgentMessage`, `ToolCall`).
    *   `CodexOp(Op)`: Outgoing operations destined for the backend.

2.  **UI State Updates**:
    *   `InsertHistoryCell`: Adds a rendered cell to the chat history.
    *   `DiffResult`: Carries the result of a git diff operation to be displayed.
    *   `StartCommitAnimation`, `StopCommitAnimation`, `CommitTick`: Controls the "typing" animation for streaming output.

3.  **Configuration Changes**:
    *   `UpdateModel`, `UpdateReasoningEffort`: Changes the active model settings.
    *   `UpdateAskForApprovalPolicy`, `UpdateSandboxPolicy`: Changes security settings.
    *   `Persist...`: Requests to save these settings to `config.toml`.

4.  **Popups and Overlays**:
    *   `OpenReasoningPopup`: Shows the reasoning effort selector.
    *   `OpenFullAccessConfirmation`: Shows the "Full Access" warning.
    *   `OpenApprovalsPopup`: Shows the approval settings.
    *   `FullScreenApprovalRequest`: Triggers a full-screen approval modal (e.g., for large diffs).
    *   `OpenFeedbackNote`, `OpenFeedbackConsent`: Feedback UI.

5.  **File Search**:
    *   `StartFileSearch(String)`: Initiates a background file search (triggered by `@` in the composer).
    *   `FileSearchResult`: Returns the matches.

6.  **Lifecycle**:
    *   `NewSession`: Resets the session.
    *   `ExitRequest`: Signals the app to close.

## Usage

The `App` struct holds the receiver (`Receiver<AppEvent>`), while `ChatWidget` and other components hold an `AppEventSender` to dispatch these events. This allows deep UI components to trigger global actions (like opening a popup or exiting the app) without needing mutable access to the `App` state.
