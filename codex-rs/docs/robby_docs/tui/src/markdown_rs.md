# `tui/src/markdown.rs`

This module provides a high-level helper for rendering Markdown text into Ratatui `Line`s.

## Function `append_markdown`

```rust
pub(crate) fn append_markdown(
    markdown_source: &str,
    width: Option<usize>,
    lines: &mut Vec<Line<'static>>,
)
```

This is the primary entry point used by widgets (like `ChatWidget`) to render Markdown content.

1.  **Delegation**: It calls `crate::markdown_render::render_markdown_text_with_width` to perform the actual parsing and styling.
2.  **Output**: It appends the resulting `Line` objects to the provided `lines` vector.

## Tests

This module contains significant unit tests (`mod tests`) that verify the rendering behavior, ensuring that:
*   Citations (e.g., `【F:/x.rs†L1】`) are rendered correctly.
*   Indented code blocks preserve whitespace.
*   Plain text is not unnecessarily split.
*   Ordered lists are formatted correctly.
