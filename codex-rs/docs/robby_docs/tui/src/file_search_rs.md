# `tui/src/file_search.rs`

This module implements the `FileSearchManager`, which orchestrates the asynchronous file search functionality triggered by the `@` symbol in the chat composer.

## Struct `FileSearchManager`

```rust
pub(crate) struct FileSearchManager {
    state: Arc<Mutex<SearchState>>,
    search_dir: PathBuf,
    app_tx: AppEventSender,
}
```

The manager is responsible for:
1.  **Debouncing**: Waiting for the user to stop typing before starting an expensive search.
2.  **Cancellation**: Cancelling in-flight searches if the query changes significantly (i.e., the new query is not a refinement of the old one).
3.  **Concurrency Control**: Ensuring only one search runs at a time, while keeping the UI responsive.

## Key Logic

### `on_user_query`

This method is called whenever the text after `@` changes.

1.  **Update State**: It updates the `latest_query` in the shared state.
2.  **Optimistic Cancellation**: If the new query does *not* start with the current active search's query, the active search is cancelled immediately.
3.  **Scheduling**: If no search is currently scheduled (debouncing), it spawns a background thread to wait for the debounce period.

### Background Thread (Debounce & Spawn)

The background thread spawned by `on_user_query`:
1.  Sleeps for `FILE_SEARCH_DEBOUNCE` (100ms).
2.  Waits until any currently `active_search` finishes (polling `ACTIVE_SEARCH_COMPLETE_POLL_INTERVAL`).
3.  Once free, it takes the `latest_query` and spawns the actual search thread (`spawn_file_search`).

### `spawn_file_search`

This runs the actual search logic (using `codex_file_search` crate) in a separate thread.
*   It passes a `cancellation_token` to the searcher.
*   If the search completes without cancellation, it sends an `AppEvent::FileSearchResult` back to the main `App` loop.
*   Finally, it clears the `active_search` state so the next queued search can run.

## Constants

*   `MAX_FILE_SEARCH_RESULTS`: 8
*   `FILE_SEARCH_DEBOUNCE`: 100ms
