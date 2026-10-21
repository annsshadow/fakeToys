# AGENTS.md
These rules apply to every task unless explicitly overridden. Bias toward caution over speed on non-trivial work; use judgment on trivial tasks.

## Engineering Rules
1. **Think before coding** — State assumptions. If ambiguous, present interpretations and ask rather than guess. Stop when confused and name what's unclear.
2. **Simplicity first** — Minimum code that solves the problem. No speculative features, no abstractions for single-use code.
3. **Surgical changes** — Touch only what you must. Don't refactor or reformat working code. Match existing style.
4. **Read before you write** — Before adding code, read its exports, callers, and shared utilities. If unsure why code is structured a way, ask.
5. **Goal-driven execution** — Define success criteria, then loop until verified instead of following fixed steps.
6. **Use the model only for judgment** — Classification, drafting, summarization, extraction. If code can answer deterministically, code answers.
7. **Surface conflicts, don't average them** — If two patterns contradict, pick one (more recent / more tested), explain why, and flag the other for cleanup.
8. **Tests verify intent** — Tests encode *why* behavior matters, not just *what* it does. A test that can't fail when business logic changes is wrong.
9. **Checkpoint after each significant step** — Summarize what's done, verified, and left. Don't continue from a state you can't describe back.
10. **Match codebase conventions** — Conformance over taste. If a convention is genuinely harmful, surface it; don't fork silently.
11. **Fail loud** — "Completed" is wrong if anything was skipped silently. "Tests pass" is wrong if any were skipped. Surface uncertainty, don't hide it.
12. **No secrets or environment specifics in the repo** — Never commit keys, tokens, passwords, private keys, real server addresses (domain / IP / host / SSH port), or personal-environment details (name, email, phone, home paths, private file locations). Read "Secrets & Environment Data" below for the criteria and the correct patterns.

## Secrets & Environment Data
本仓库推到公开远端（GitHub 可见）。以下内容**一律不入库**，也不要在对话、工作日志、提交信息里回显。

**禁止入库（真值）**
- 密钥、token、口令、私钥、证书、API key、签名盐值 —— 无论出现在代码、注释、文档、测试夹具、日志
- 带口令的连接串（`postgres://user:pass@host/db`）、`Authorization` / `Cookie` 头
- 真实服务器地址：生产/测试域名、IP、主机名、SSH 端口、非回环端口映射
- 个人与环境信息：姓名、邮箱、手机号、真实用户目录（`C:\Users\<name>`）、本机绝对路径、私人目录/文件路径
- 任何绕过 GitHub Secrets 的凭据（`DEPLOY_SSH_KEY` 这类一律走仓库 Secrets）

**正确做法**
- 配置走环境变量，值由部署环境注入；代码只读 `env` / 配置，自身不落真值
- 需要在仓库里出现的"形状"用占位符：`<YOUR_TEST_DOMAIN>`、`<TEST_HOST>`、`<YOUR_API_KEY>`、`${VAR_NAME}`，本地回环 + 占位口令（如 `postgres://user:pass@localhost:5432/db`）
- 模板入仓、真值不入仓：`.env.example` / `*.example` 入库；`.env`、`.local.keys`、`remote-config.env`、`*.pem`、`*.key` 必须进 `.gitignore`
- 部署文档只写"值从哪读"，不写"值是多少"；真值文件**路径**可以写（如 `laiyipao/deploy/.local.keys`），**内容**不行
- 提交前自检：`git status --porcelain` + `git diff --cached`，逐个新文件确认没有真值；拿不准就问，别猜

**发现已泄露**
- 凭据泄露 = 事故：先轮换，再 `git rm --cached` + 补 `.gitignore`，需要时用 `git filter-repo` 清历史，然后明确告知用户。**不要"先提交再改"**
- 域名 / IP / 私钥路径属定位信息：单独不足以进门，但会把"该往哪打"写清楚，同样要清

**现有守卫（新增子项目照抄这个模式）**
- `laiyipao/server/internal/domain/secrets_leak_test.go` —— 两条判据：① 从 `.local.keys` 读真值做子串比对；② 私钥路径与 `root@<IP>` 登录。扫的是**整个工作树**（含 `.workbuddy/` 等未入库目录），所以本地工作日志里也别写真值
- `.gitignore` 已拦 `.env` / `.local.keys` / `remote-config.env`；新增子项目要把同类文件一并拦掉

## Custom Instructions
- **Language**: Always respond in Chinese (Simplified), including summaries and explanations for all plugin commands (e.g. `/claude-hud`, `/ce:plan`).
- **Errors**: Explain technical errors (e.g. PostgreSQL 23505) in Chinese first, then the technical details.

## Compact Instructions
When compressing, preserve in priority order:
1. Architecture decisions (never summarize away)
2. Modified files and their key changes
3. Current verification status (pass/fail)
4. Open TODOs and rollback notes
5. Tool outputs (drop detail, keep pass/fail)

## Knowledge Store
`docs/solutions/` — documented solutions to past problems (bugs, best practices, architecture patterns, workflow improvements), organized by category with YAML frontmatter (`module`, `tags`, `problem_type`). Relevant when implementing or debugging in the oa4rust or oa subproject areas.
