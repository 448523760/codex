# Rendering

The TUI uses [Ratatui](https://github.com/ratatui-org/ratatui) for rendering. The rendering logic is split across several modules.

## Markdown Rendering (`src/markdown_render.rs`)

Codex relies heavily on Markdown for displaying agent responses. The `render_markdown_text` function converts a Markdown string into a Ratatui `Text` object.

It uses the `pulldown-cmark` crate to parse the Markdown event stream. A custom `Writer` struct iterates over these events and builds the Ratatui `Text`.

### Features
-   **Word Wrapping**: Supports wrapping text while preserving indentation for lists and blockquotes.
-   **Styling**: Applies styles for headings, emphasis, links, etc.
-   **Code Blocks**: Renders code blocks (syntax highlighting is handled separately or by the terminal if supported).

## Diff Rendering (`src/diff_render.rs`)

The `DiffSummary` struct is responsible for rendering git diffs. It parses the diff output and applies color coding (green for additions, red for deletions) to make it readable in the terminal.

## Streaming (`src/streaming/`)

When the agent is generating a response, the content is streamed to the UI. The `StreamController` manages this process. It handles:
-   Accumulating tokens.
-   Updating the current `HistoryCell` with new content.
-   Triggering redraws.

## Custom Terminal (`src/custom_terminal.rs`)

This module provides a wrapper around the Ratatui `Terminal` to handle specific needs like:
-   **Inline Viewport**: Rendering the chat below the user's previous shell output, rather than clearing the entire screen.
-   **Alternate Screen**: Switching to a full-screen view for things like large diffs or the transcript history.
