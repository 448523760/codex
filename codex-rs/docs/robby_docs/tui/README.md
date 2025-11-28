# Codex TUI

The `codex-tui` crate provides the terminal user interface for Codex. It is built using [Ratatui](https://github.com/ratatui-org/ratatui) and [Crossterm](https://github.com/crossterm-rs/crossterm).

## Detailed Source Documentation

We have detailed documentation for the most important source files:

*   **Core Architecture**
    *   [`lib.rs`](src/lib_rs.md) - Entry point, config, and initialization.
    *   [`main.rs`](src/main_rs.md) - Binary entry point and CLI parsing.
    *   [`app.rs`](src/app_rs.md) - Main application state and event loop.
    *   [`tui.rs`](src/tui_rs.md) - Terminal backend wrapper.
    *   [`app_event.rs`](src/app_event_rs.md) - Internal event bus definition.

*   **UI Components**
    *   [`chatwidget.rs`](src/chatwidget_rs.md) - The main chat interface.
    *   [`bottom_pane/mod.rs`](src/bottom_pane_mod_rs.md) - Input area, status, and popups.
    *   [`history_cell.rs`](src/history_cell_rs.md) - Chat history items.

*   **Features & Logic**
    *   [`file_search.rs`](src/file_search_rs.md) - `@` file search logic.
    *   [`slash_command.rs`](src/slash_command_rs.md) - Slash command definitions.
    *   [`session_log.rs`](src/session_log_rs.md) - Session recording.

*   **Rendering**
    *   [`markdown_render.rs`](src/markdown_render_rs.md) - Core Markdown rendering logic.
    *   [`markdown.rs`](src/markdown_rs.md) - High-level Markdown helpers.

## Directory Structure

- `src/`: Source code for the TUI.
  - `bin/`: Standalone binary entry point.
  - `lib.rs`: Library entry point, configuration loading, and main loop initialization.
  - `app.rs`: Main application state and event loop.
  - `tui.rs`: Low-level terminal handling (initialization, event stream, drawing).
  - `chatwidget.rs`: The core UI component managing the conversation and interaction.
  - `render/`: Rendering logic for various components.
  - `markdown.rs` & `markdown_render.rs`: Markdown parsing and rendering.
  - `bottom_pane/`: UI components for the bottom input and status area.

## Key Components

- **App**: The top-level application state manager. It handles the main event loop, processes events, and manages the global state (config, auth, etc.).
- **Tui**: Wraps the terminal backend, handling raw mode, alternate screens, and the drawing loop.
- **ChatWidget**: The primary widget that displays the chat history, handles user input, and manages the interaction with the Codex backend.

## Running

The TUI is typically run via the `codex` CLI. The entry point is `run_main` in `src/lib.rs`.
