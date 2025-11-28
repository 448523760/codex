# Session 任务管理机制详解

本文档详细解释 `core/src/tasks/mod.rs` 中 `impl Session` 块的代码逻辑。该模块主要负责 `Session` 级别的任务（Task）生命周期管理，包括任务的启动、中止、完成回调以及并发控制。

## 核心概念

在 Codex 中，`Session` 维护着当前的对话状态。一个 Session 在同一时间通常只有一个活跃的“回合”（Turn），而一个回合中运行着一个具体的“任务”（Task，如 `RegularTask`, `ReviewTask` 等）。

`impl Session` 提供了对这些 `SessionTask` 的调度能力。

## 主要方法

### `spawn_task<T: SessionTask>`

启动一个新的异步任务。这是执行具体业务逻辑（如处理用户输入、执行代码、Review 等）的入口。

**参数:**
- `turn_context`: 当前回合的上下文。
- `input`: 输入给任务的数据（如用户消息）。
- `task`: 实现了 `SessionTask` trait 的具体任务实例。

**执行流程:**
1.  **中止旧任务**: 首先调用 `self.abort_all_tasks(TurnAbortReason::Replaced)`。这确保了 Session 的单任务（或受控并发）特性，新任务会“顶掉”旧任务。
2.  **准备控制原语**:
    -   创建 `CancellationToken`: 用于优雅地通知任务取消。
    -   创建 `Notify` (`done`): 用于任务完成时的信号通知。
3.  **启动后台协程 (`tokio::spawn`)**:
    -   在后台运行 `task.run(...)`。
    -   任务运行结束后，调用 `flush_rollout()` 确保数据持久化。
    -   **完成回调**: 如果任务未被取消（`!is_cancelled()`），调用 `self.on_task_finished(...)` 处理正常结束流程。
    -   **通知结束**: 触发 `done.notify_waiters()`，告知外部等待者任务已退出。
4.  **注册任务**: 将任务封装为 `RunningTask` 结构（包含句柄、Token、上下文等），并通过 `register_new_active_task` 存入 Session 的状态中。

### `abort_all_tasks`

中止当前 Session 中所有正在运行的任务。

**参数:**
- `reason`: 中止原因（如 `Interrupted` 用户打断, `Replaced` 被新任务替换）。

**执行流程:**
1.  调用 `take_all_running_tasks()` 原子地取出并移除所有活跃任务。
2.  遍历这些任务，逐一调用 `handle_task_abort` 进行中止处理。

### `on_task_finished`

任务正常运行结束时的回调方法。

**功能:**
1.  **清理状态**: 锁定 `active_turn`，根据 `sub_id` 移除对应的任务。如果移除后当前回合没有其他任务，则重置 `active_turn` 为 `None`。
2.  **发送事件**: 发送 `TaskComplete` 事件给客户端，包含最后生成的 Agent 消息（如果有）。

## 内部辅助方法

### `register_new_active_task`
- **功能**: 将构建好的 `RunningTask` 存入 `self.active_turn`。
- **逻辑**: 获取锁，如果当前没有 `ActiveTurn` 则新建一个，然后将任务加入其中。

### `take_all_running_tasks`
- **功能**: 取出所有正在运行的任务，用于中止流程。
- **逻辑**: 获取锁，取出 `ActiveTurn`，清理其中的 pending 状态，并 `drain`（排出）所有任务返回。

### `handle_task_abort`
处理单个任务的中止逻辑，包含优雅退出和强制中止的超时机制。

**执行流程:**
1.  **检查状态**: 如果 `cancellation_token` 已经被取消，说明已经在处理中，直接返回。
2.  **触发取消**: 调用 `task.cancellation_token.cancel()`，通知任务内部循环停止。
3.  **等待退出 (Graceful Shutdown)**:
    -   使用 `select!` 等待任务的 `done` 信号。
    -   设置 `GRACEFULL_INTERRUPTION_TIMEOUT_MS` (100ms) 超时。
    -   如果超时未退出，记录 `warn` 日志。
4.  **强制中止**: 调用 `task.handle.abort()`，这是 Tokio 任务句柄的强制中止方法，作为最后的兜底手段。
5.  **任务清理**: 调用 `task.abort(...)`，允许任务执行自定义的清理逻辑（如释放资源）。
6.  **发送事件**: 发送 `TurnAborted` 事件通知客户端任务已中止。

## 总结

`core/src/tasks/mod.rs` 中的 `impl Session` 实现了健壮的任务调度模型：
- **排他性**: 通过 `spawn_task` 自动中止旧任务。
- **优雅退出**: 结合 `CancellationToken` 和超时机制，优先让任务自行结束，必要时才强制杀死。
- **状态一致性**: 通过 `active_turn` 锁保护任务列表，确保状态在并发环境下的正确性。
