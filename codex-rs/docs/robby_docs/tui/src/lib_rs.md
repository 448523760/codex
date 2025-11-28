# `tui/src/lib.rs`

This file is the library entry point for the `codex-tui` crate. It is responsible for initializing the application environment, loading configuration, and starting the main event loop.

## Key Functions

### `run_main`

```rust
pub async fn run_main(
    mut cli: Cli,
    codex_linux_sandbox_exe: Option<PathBuf>,
) -> std::io::Result<AppExitInfo>
```

This is the primary entry point called by the binary. Its responsibilities include:

1.  **Configuration Loading**:
    *   Parses CLI arguments (`Cli` struct).
    *   Loads `config.toml` using `load_config_as_toml_with_cli_overrides`.
    *   Resolves the OSS provider if `--oss` is used.
    *   Determines the model to use (CLI override > OSS default > Config default).
    *   Sets up `ConfigOverrides` based on CLI flags (e.g., sandbox mode, approval policy).

2.  **Environment Setup**:
    *   Checks for login restrictions (`enforce_login_restrictions`).
    *   Sets up logging (file logging to `codex-tui.log` and OpenTelemetry if configured).
    *   Initializes the `CodexFeedback` system.

3.  **TUI Initialization**:
    *   Calls `run_ratatui_app` to start the UI.

### `run_ratatui_app`

```rust
async fn run_ratatui_app(
    cli: Cli,
    initial_config: Config,
    overrides: ConfigOverrides,
    cli_kv_overrides: Vec<(String, toml::Value)>,
    active_profile: Option<String>,
    feedback: codex_feedback::CodexFeedback,
) -> color_eyre::Result<AppExitInfo>
```

This function manages the lifecycle of the Ratatui application:

1.  **Terminal Setup**:
    *   Installs `color_eyre` for panic handling.
    *   Sets up a custom panic hook to restore the terminal state on crash.
    *   Calls `tui::init()` to enter raw mode and prepare the terminal.

2.  **Onboarding & Trust**:
    *   Checks if the user needs to go through onboarding (`should_show_onboarding`).
    *   If so, runs `run_onboarding_app`.
    *   Reloads configuration if the user changes trust settings during onboarding.

3.  **Session Resumption**:
    *   Determines whether to start a new session or resume an existing one based on CLI flags (`--resume`, `--resume-last`).
    *   May run the `resume_picker` UI if needed.

4.  **Main Loop**:
    *   Initializes the `App` struct.
    *   Calls `App::run` to enter the main event loop.
    *   Handles the result, ensuring `restore()` is called to reset the terminal state before exiting.

## Helper Functions

*   `restore()`: Restores the terminal to its original state (leaves raw mode, shows cursor).
*   `get_login_status()`: Checks if the user is authenticated with OpenAI (if required).
*   `should_show_trust_screen()`: Determines if the project trust prompt is needed.
