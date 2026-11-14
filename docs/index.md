# fakeToys 文档

本站收录三个项目的完整文档；源码仓库共 5 个子项目，另两个的文档留在仓库 `docs/` 下、未纳入发布白名单。全站支持全文检索，按 `/` 直接搜。

## 项目

<div class="grid cards" markdown>

-   :material-gamepad-variant:{ .lg .middle } **来一炮 · laiyipao**

    ---

    塔防 + 元素反应玩法的 **H5 游戏**（小程序端已于 2026-11-14 移除）：玩法设计、系统架构、实现规模与验证状态、运维手册。

    [:octicons-arrow-right-24: 开始阅读](laiyipao/README.md)

-   :material-brain:{ .lg .middle } **LLM 实验 · llm-lab**

    ---

    从零手写一个 GPT 的 11 篇讲义（00–10）：分词、注意力、Transformer、预训练、微调对齐与推理优化。

    [:octicons-arrow-right-24: 开始阅读](llm-lab/README.md)

-   :material-office-building:{ .lg .middle } **OA4Rust**

    ---

    三端办公自动化系统：Rust 后端、Vue 桌面端、uni-app 移动端的架构、接口与维护约定。

    [:octicons-arrow-right-24: 开始阅读](oa4rust/README.md)

</div>

## 按目的找

| 你想要的 | 去这里 |
|---|---|
| 了解游戏玩法怎么设计的 | [来一炮 · 游戏设计](laiyipao/GAME_DESIGN.md) |
| 看懂来一炮的代码结构 | [来一炮 · 系统设计](laiyipao/system/README.md) |
| 学大模型是怎么训出来的 | [LLM 实验 · 学习路线](llm-lab/00-学习路线.md) |
| 把 OA4Rust 部署跑起来 | [OA4Rust · 安装与运行](oa4rust/06-install-and-run.md) |

!!! tip "文中的路径指的是源码仓库"
    `laiyipao/server/...`、`oa4rust/backend/...` 这类路径都相对于源码仓库根目录。本站只有文档，不含源码。

## 仓库根目录入口文件（约定）

本站只发布 `docs/` 下三个项目目录。源码仓库根目录另保留两个 Markdown 入口，
**不要把它们搬进 `docs/`**：

| 文件 | 职责 | 为什么必须留在根 |
| --- | --- | --- |
| `README.md` | GitHub 仓库首页 / 全仓导览 | 平台固定读取根目录 README；augmentor 的六道文档守卫以 `ROOT / "README.md"` 为判据源 |
| `AGENTS.md` | 全仓工程规范与密钥纪律 | Agent 工具链按约定从仓库根读取；`mkdocs.yml` 已显式排除 |

原根目录的 `MEMORY.md`（2026-09-07 历史里程碑快照）已归档为
`docs/ops/history-milestones-2026-09-07.md`，只作历史事实留痕。
当前各项目实测现量见 `docs/ops/current-state.md`；`augmentor/`、`auto-checkin/`、
`linux-7.1.3/` 等文档仍在仓库 `docs/` 下，按白名单约定不纳入本站。
