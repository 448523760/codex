# Codex Doc

## spawn

`Codex::spawn` 是 Codex 系统的入口点，用于初始化一个新的 Codex 会话。它负责设置通信通道、加载配置、初始化会话状态，并启动主事件循环。

### 整体流程

1.  **初始化通信通道**：创建两个异步通道，一个用于发送提交（`Submission`），另一个用于接收事件（`Event`）。
2.  **加载配置与策略**：
    *   获取用户指令（`user_instructions`）。
    *   加载执行策略（`exec_policy`），如果加载失败则返回致命错误。
    *   构建 `SessionConfiguration` 对象，聚合所有会话相关的配置信息。
3.  **创建 Session**：调用 `Session::new` 创建一个新的会话实例。这一步会初始化 Rollout 记录器、检测 Shell 环境、加载历史记录元数据等。
4.  **启动主循环**：使用 `tokio::spawn` 启动 `submission_loop` 任务，该任务会持续监听并处理提交请求。
5.  **返回结果**：构建并返回 `CodexSpawnOk` 结构，其中包含 `Codex` 实例（用于后续交互）和会话的 `conversation_id`。

### 关键部分详解

*   **SessionConfiguration**：
    这是一个核心配置结构体，包含了模型信息（provider, model）、推理配置（effort, summary）、指令（developer/user/base instructions）、安全策略（approval/sandbox policy）、工作目录（cwd）以及功能特性开关（features）。它是 Session 运行的基础。

*   **Session::new**：
    这是会话初始化的重头戏。它并行执行了多个耗时的初始化任务以减少启动延迟：
    *   `RolloutRecorder::new`: 初始化会话记录器。
    *   `shell::default_user_shell`: 检测用户的默认 Shell。
    *   `history_metadata`: 加载历史记录元数据。
    *   `compute_auth_statuses`: 计算 MCP 服务器的认证状态。
    
    此外，它还负责发送 `SessionConfigured` 事件以及可能的弃用通知（DeprecationNotice）。

*   **submission_loop**：
    这是一个后台运行的无限循环（直到收到 Shutdown 信号）。它从 `rx_sub` 通道接收 `Submission`，根据 `Op` 类型（如 `UserInput`, `Interrupt`, `Undo` 等）调用相应的处理函数（handlers）。这是 Codex 响应外部请求的核心驱动器。

### 潜在问题分析

1.  **初始化失败风险**：
    在 `Session::new` 中，虽然使用了 `tokio::join!` 并行处理初始化任务，但如果关键组件（如 `RolloutRecorder`）初始化失败，整个 `spawn` 过程会直接报错返回。这虽然保证了会话的一致性，但也意味着任何一个子系统的严重故障都会阻止 Codex 启动。
    > 设计codex不能在该情况下启动

2.  **后台任务监控**：
    `submission_loop` 是通过 `tokio::spawn` 启动的后台任务。如果该任务因不可预见的 panic 而终止，`Codex` 结构体持有的发送端可能不会立即感知（除非通道关闭），外部调用者可能需要通过检测通道错误来判断会话是否存活。
    > 没有潜在问题。发送到会处理发送失败error。


3.  **配置依赖**：
    `spawn` 方法高度依赖传入的 `Config` 对象。如果 `Config` 中的路径（如 `cwd`）无效或不一致（例如不是绝对路径），会在初始化早期被拦截，但这也要求调用方必须确保配置的正确性。
    > config的创建是内部的，系统必须要保证config正确才能执行
