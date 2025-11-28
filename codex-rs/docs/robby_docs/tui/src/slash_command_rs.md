# `tui/src/slash_command.rs`

This module defines the available slash commands (e.g., `/new`, `/diff`, `/quit`) using a Rust enum.

## Enum `SlashCommand`

```rust
#[derive(EnumString, EnumIter, ...)]
pub enum SlashCommand {
    Model,
    Approvals,
    Review,
    New,
    Init,
    Compact,
    Undo,
    Diff,
    Mention,
    Status,
    Mcp,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    TestApproval,
}
```

The enum variants represent all the commands the user can type. The order in the enum definition determines the presentation order in the UI popup.

## Key Methods

*   **`description(self) -> &'static str`**: Returns the help text shown in the command palette (e.g., "start a new chat during a conversation").
*   **`command(self) -> &'static str`**: Returns the command string (e.g., "new", "diff").
*   **`available_during_task(self) -> bool`**: Determines if the command can be executed while the agent is busy running a task.
    *   Allowed: `/diff`, `/status`, `/quit`, etc.
    *   Blocked: `/new`, `/undo`, `/model` (actions that change conversation state).
*   **`is_visible(self) -> bool`**: Hides internal/debug commands (like `/rollout` or `/test-approval`) in release builds.

## Usage

The `built_in_slash_commands()` function returns a list of all visible commands, which is used by the `SlashCommandPopup` to populate the autocomplete list.
