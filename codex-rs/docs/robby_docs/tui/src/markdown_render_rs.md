# `tui/src/markdown_render.rs`

This module handles the rendering of Markdown text into Ratatui `Text` objects. It is used extensively for displaying agent responses.

## Function `render_markdown_text`

```rust
pub fn render_markdown_text(input: &str) -> Text<'static>
```

This is the main entry point. It parses the input string using `pulldown-cmark` and converts the event stream into styled terminal text.

## Struct `Writer`

The core logic resides in the private `Writer` struct, which iterates over Markdown events.

### Features

1.  **Styling**:
    *   **Headings**: Bold, underlined, or italic depending on level.
    *   **Emphasis/Strong**: Italic/Bold styles.
    *   **Links**: Rendered as `text (url)` with the URL cyan and underlined.
    *   **Blockquotes**: Indented with `> ` and colored green.

2.  **Lists**:
    *   Handles nested lists.
    *   Automatically numbers ordered lists.
    *   Manages indentation for list items.

3.  **Code Blocks**:
    *   Renders code blocks with a slightly different background or indentation (implementation detail: currently just indentation).
    *   **Note**: Syntax highlighting is *not* handled here; it's often handled by `syntect` in other parts of the app or left plain.

4.  **Word Wrapping**:
    *   Uses `textwrap` (via `crate::wrapping`) to wrap text to the terminal width.
    *   Crucially, it preserves indentation contexts (e.g., wrapping text *inside* a list item or blockquote correctly).

### Indentation Handling (`IndentContext`)

The `Writer` maintains a stack of `IndentContext`s. This allows it to correctly prefix lines when deeply nested (e.g., a paragraph inside a blockquote inside a list item).

```rust
struct IndentContext {
    prefix: Vec<Span<'static>>, // e.g., "  "
    marker: Option<Vec<Span<'static>>>, // e.g., "- " or "1. "
    is_list: bool,
}
```

When a new line is started, the `Writer` reconstructs the full prefix from this stack.
