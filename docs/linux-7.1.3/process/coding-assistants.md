AI 编程助手
++++++++++++++++++++

本文档为在参与 Linux 内核开发时使用 AI 辅助的 AI 工具与开发者提供指导。

协助 Linux 内核开发的 AI 工具应遵循标准的内核开发流程：

- Documentation/process/development-process.rst
- Documentation/process/coding-style.rst
- Documentation/process/submitting-patches.rst

## 许可与法律要求


所有贡献必须符合内核的许可要求：

- 所有代码必须与 GPL-2.0-only 兼容
- 使用适当的 SPDX 许可标识符
- 详见 Documentation/process/license-rules.rst

## Signed-off-by 与开发者来源证书


AI 代理不得添加 Signed-off-by 标签。只有人类才能合法地认证开发者来源证书
（DCO）。人类提交者负责：

- 审查所有 AI 生成的代码
- 确保符合许可要求
- 添加其自己的 Signed-off-by 标签以认证 DCO
- 对贡献承担全部责任

## 署名


当 AI 工具参与内核开发时，适当的署名有助于追踪 AI 在开发过程中不断演变的
角色。

```
  Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]

```

其中：

- `AGENT_NAME` 是 AI 工具或框架的名称
- `MODEL_VERSION` 是所使用的特定模型版本
- `[TOOL1] [TOOL2]` 是可选的专业分析工具（例如 coccinelle、sparse、smatch、clang-tidy）

基本的开发工具（git、gcc、make、编辑器）不应列出。

```

  Assisted-by: Claude:claude-3-opus coccinelle sparse

```
