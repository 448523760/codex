# Codex Handlers

## handlers::user_input_or_turn

本文档解释 `core/src/codex.rs` 中 `handlers::user_input_or_turn` 的处理流程、关键细节与潜在问题，并补充说明其依赖的 `Session::inject_input` 的行为。

### 整体流程概述
- 接收一次来自上层（CLI/VSCode/服务端）的 `Op`，只处理两类：
	- `Op::UserTurn { ... , items }`：同时携带“本回合设置更新”和“用户输入项”。
	- `Op::UserInput { items }`：仅携带用户输入项，无设置更新。
- 将 `Op` 解析为 `(items, updates)`：
	- `UserTurn` 会填充一个 `SessionSettingsUpdate`（包含 `cwd`、`approval_policy`、`sandbox_policy`、`model`、`reasoning_effort`、`reasoning_summary`、`final_output_json_schema`）。
	- `UserInput` 则使用默认空更新。
- 基于解析出的 `updates` 调用 `sess.new_turn_with_sub_id(sub_id, updates)` 构建当前回合的 `TurnContext`，并通过 OTel 记录一次 `user_prompt(&items)`。
- 首选“注入输入”到当前正在运行的任务：`sess.inject_input(items)`。
	- 若注入成功（有活动任务），则本次不启动新任务，函数结束。
	- 若注入失败（当前没有活动任务），则：
		1) 若与上一回合上下文存在环境差异，则生成并记录一条 `EnvironmentContext::diff(...)` 到对话历史与rollout；
		2) 调用 `sess.spawn_task(current_context, items, RegularTask)` 启动一个新任务；
		3) 更新 `previous_context = Some(current_context)`。

### 关键细节解析
- `SessionSettingsUpdate` 映射（仅 `UserTurn` 分支）：
	- 目录：`cwd: Some(cwd)`
	- 批准策略：`approval_policy: Some(approval_policy)`
	- 沙箱策略：`sandbox_policy: Some(sandbox_policy)`
	- 模型：`model: Some(model)`
	- 推理配置：`reasoning_effort: Some(effort)`、`reasoning_summary: Some(summary)`
	- 结构化输出：`final_output_json_schema: Some(final_output_json_schema)`
	这些更新会在 `new_turn_with_sub_id` 内应用到 `SessionState.session_configuration`，进而影响新建的 `TurnContext`。

- `OTel` 打点：
	- 在尝试注入/启动任务之前，统一调用 `current_context.client.get_otel_event_manager().user_prompt(&items)` 记录用户提示，确保观测链完整。

- 环境差异记录（仅在需要启动新任务时）：
	- 通过 `sess.build_environment_update_item(previous_context.as_ref(), &current_context)` 计算差异。
	- 内部比较使用 `EnvironmentContext::from(prev)` 与 `EnvironmentContext::from(next)`，并用 `equals_except_shell` 判断是否“除 shell 外完全一致”。
	- 若不一致，则构造 `EnvironmentContext::diff(prev, next)` 并作为 `ResponseItem` 写入对话历史与rollout（`record_conversation_items`）。

- 启动任务：
	- `sess.spawn_task(Arc::clone(&current_context), items, RegularTask).await`
	- `RegularTask` 是普通对话/工具调用的执行器；`items` 会作为首批输入喂给 `run_task`。

### `Session::inject_input` 行为说明
源码位置：`core/src/codex.rs` 中 `impl Session`：

```
pub async fn inject_input(&self, input: Vec<UserInput>) -> Result<(), Vec<UserInput>> {
		let mut active = self.active_turn.lock().await;
		match active.as_mut() {
				Some(at) => {
						let mut ts = at.turn_state.lock().await;
						ts.push_pending_input(input.into());
						Ok(())
				}
				None => Err(input),
		}
}
```

- 若存在活动任务（`active_turn` 为 `Some`）：
	- 取到内部 `turn_state`，调用 `push_pending_input(...)` 将该批输入添加到“待处理输入队列”。
	- 返回 `Ok(())`，表示“已注入当前任务，不需要开新任务”。
- 若不存在活动任务：
	- 返回 `Err(input)` 原样返还输入。上层（即 `user_input_or_turn`）收到 `Err` 后会走“记录环境差异 + 启动新任务”的分支。

### 潜在问题与注意事项
- 设置更新的即时性与“注入成功”路径：
	- `UserTurn` 分支会把设置更新（如 `cwd/model/policy/...`）应用到 `SessionState` 并构造新 `TurnContext`，但如果 `inject_input` 成功，当前运行中的任务仍沿用其启动时的旧 `TurnContext` 与环境；
	- 这意味着：设置更新会“生效于后续新任务”，对当前已在运行的任务不产生即时影响；同时本次并不会记录环境变更项（只有在新任务启动前才会记录）。
	- 可能的改进：在注入成功的情况下，也可以考虑记录一次“环境变更”（或者限制在 `UserInput` 场景不允许携带设置更新，以避免歧义）。

- Shell 变更不触发环境差异事件：
	- 由于使用 `equals_except_shell` 判断是否需要生成差异，若仅 shell 发生变化，将不会记录 `EnvironmentContext::diff`。
	- 这有利于减少噪音，但也可能导致 UI/观察端感知不到 shell 变化。根据产品需求，需确认这是否符合预期。

- 待处理输入队列规模：
	- `push_pending_input` 没看到容量限制或背压机制；如果 UI 高频注入消息，而模型/任务处理速度跟不上，队列可能无限增长。
	- 建议在 `TurnState` 层增加上限或策略（截断/合并/告警）。

- 额外的开销：
	- 即便最终只是“注入成功”，也会先创建一个新的 `TurnContext`（`new_turn_with_sub_id`），随后丢弃；这带来一些小额分配/日志开销，但一般可接受。

- 并发/竞态考虑：
	- `inject_input` 与 `spawn_task` 之间已通过 `Mutex`/`await` 序列化访问，常见竞态（例如任务刚结束）会落入“注入失败 -> 启动新任务”的安全分支；整体设计较稳健。

### 小结
- `handlers::user_input_or_turn` 的设计优先复用当前任务（注入输入），只有在没有活动任务时才启动新任务；
- 对于带设置更新的 `UserTurn`，更新会立即落到 `SessionState`，但若注入成功，并不会影响正在运行的任务，也不会立刻记录环境差异；
- 上述行为在可预期性与观测性上需要团队层面确认是否完全符合产品期望，特别是 shell 变更与“注入成功时的环境差异记录”两点。