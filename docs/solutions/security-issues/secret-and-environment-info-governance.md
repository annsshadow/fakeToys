---
title: "仓库密钥与个人环境信息的入库治理：分级判据、可移植路径写法与三道守卫"
date: 2026-10-21
last_refreshed: 2026-10-21
category: security-issues
module: AGENTS.md（仓库根规则） + .gitignore + laiyipao/server/internal/domain/secrets_leak_test.go
problem_type: secret_and_environment_leak_prevention
component: repo_policy
severity: high
symptoms:
  - "公开仓库的已跟踪文件里散落着真实用户名（C:\\Users\\<LOCAL_USER>\\...）、本机目录布局（D:\\<LOCAL_ROOT>\\...）与解释器绝对路径"
  - "一次性脚本把本机绝对路径写死，换机器/改目录后脚本直接坏掉（本仓 7 个 .cjs 指向已被改名的 oa4rust-web/）"
  - "规则只写在文档里、没有 .gitignore 兜底与守卫 ⇒ .env / *.pem 一旦新建就可能被 add 进去"
  - "测试夹具里的私钥既不能删（5 个用例依赖），又不能不管（怕有人把真密钥换进去）"
root_cause: policy_without_enforcement
resolution_type: policy_plus_guards_plus_code_fix
tags: [secrets, personal-info, environment-paths, gitignore, guard-test, portability, test-fixture, red-team]
applies_when:
  - "仓库要推到公开远端（GitHub 可见）"
  - "团队多人协作、各自本机目录布局不同"
  - "存在一次性脚本 / 生成器 / 文档里引用本机路径"
  - "代码里有必须入库的测试夹具密钥"
related_components:
  - AGENTS.md
  - .gitignore
  - laiyipao/server/internal/domain/secrets_leak_test.go
  - oa4rust/backend/crates/signature/src/lib.rs
  - oa4rust/scripts/add_form_funcs.cjs
  - oa4rust/backend/scripts/regen_endpoints.py
  - auto-checkin/run-once.bat
---

# 仓库密钥与个人环境信息的入库治理

## Problem

「不要把密钥提交进仓库」是一句没人记得的口头约定。本仓把它变成规则时，暴露了三层问题：

1. **规则没有强制点** —— 只写进 AGENTS.md，`.env` / `*.pem` 新建时照样能被 `git add`。
2. **规则没有判据** —— 「不要提交个人和环境信息」如果不落到具体形态（真实用户名？盘符绝对路径？仓外探针路径？），执行时无从判断。
3. **规则会误伤** —— 上游镜像文档里的 `/home/<上游作者>/` 不是本仓信息；账本正文里有一处**行内真实 CR 字符**（讨论「每行都前接一个 CR」时被引用的字符），按行处理会把它当行尾删掉。

第 3 点决定了：**不能写一条「扫描所有绝对路径」的守卫**。仓库自己的 `secrets_leak_test.go` 早就记下了这条教训 ——「试过：仓库里有大量形如 `1.2.3.4` 的版本号与 CGNAT 段，一条『禁止公网 IP』的规则会立刻变成噪声，而噪声守卫会被人加白名单绕过 —— 那还不如不做」。

## Resolution

### 三级分类：真值 / 定位信息 / 本仓环境信息

| 级别 | 内容 | 处置 |
|------|------|------|
| **真值** | 密钥、token、口令、私钥、带口令连接串 | 一律不入库；泄露 = 事故，先轮换再清理 |
| **定位信息** | 真实域名 / IP / 主机名 / SSH 端口 / 私钥路径 | 单独不足以进门，但会写清「该往哪打」，同样要清 |
| **本仓环境信息** | 真实用户名、本机目录布局、解释器绝对路径 | 可移植写法替代；不影响安全，但泄露本机结构 |

前两级已有守卫覆盖（`secrets_leak_test.go` 的两条判据 + `laiyipao/deploy/.local.keys` 真值比对）。本轮补的是第三级。

### 规则要落到具体形态

`AGENTS.md` 的「Secrets & Environment Data」小节把「个人和环境信息」写成可判定的形态清单，并给出替代写法：

| 形态 | 替代写法 |
|------|----------|
| `C:\Users\<真实用户名>\...` | `<HOME>/...`；或代码里用 `%LOCALAPPDATA%` / `os.tmpdir()` |
| `D:\<LOCAL_ROOT>\<仓内路径>` | 仓库相对路径 |
| `D:\<LOCAL_ROOT>\<仓外探针/venv>` | `<OUTSIDE_REPO>/...`（保留文件名，隐去目录布局） |
| `D:\<LOCAL_ROOT>\linux-7.1.3\...`（上游内核源树） | `<LINUX_DOCS_SRC>/...` |

### 强制点一：.gitignore 通用规则

```gitignore
.env
.env.*
*.env
!.env.example
!.env.*.example
!*.env.example
*.pem
*.key
*.p12
*.pfx
*.keystore
*.local.keys
remote-config.env
```

**为什么加 `*.env` 而不只是 `.env`**：本仓原先只拦 `oa4rust/backend/.env`，而 `augmentor/docker/.env`（其 README 明确让用户 `cp .env.example .env`）**实际未被忽略** —— 规则写了却没强制，等于假规则。

**为什么模板要显式反选**：`.env.example` / `.env.test.example` 必须入库，否则别人无从知道要配哪些变量。

### 强制点二：可移植路径写法（治本）

**本机绝对路径写在代码里，等于把「换机器就坏」和「泄露本机结构」绑在一起**。本仓 7 个 `.cjs` 一次性脚本同时中了这两枪：

```js
// 改前：泄露本机布局，且 oa4rust-web 已改名为 oa4rust/frontend ⇒ 脚本本来就是坏的
const content = fs.readFileSync('D:/<LOCAL_ROOT>/fakeToys/oa4rust-web/apps/desktop/src/views/FormDesigner.vue', 'utf8')
// 改后：从脚本自身位置推导，指向真实存在的位置
const TARGET = path.resolve(__dirname, '../frontend/apps/desktop/src/views/FormDesigner.vue')
const content = fs.readFileSync(TARGET, 'utf8')
```

Python 同理：

```python
# 改前
BASE = Path(r'D:\<LOCAL_ROOT>\fakeToys\oa4rust')
# 改后（且顺带修正了 BASE 层级：crates/ 实际在 oa4rust/backend/ 下）
BASE = Path(__file__).resolve().parents[1]
```

Windows 批处理用环境变量，既去掉用户名又不失功能：

```bat
REM 改前：真实账号名进仓
call :try_npm "C:\Users\<LOCAL_USER>\AppData\Local\nvm\v20.18.1\npm.cmd"
REM 改后：同一位置，不含账号名
call :try_npm "%LOCALAPPDATA%\nvm\v20.18.1\npm.cmd"
```

### 强制点三：测试夹具密钥 —— 「允许 + 自证」

私钥不能一刀切禁掉：签名模块的 5 个用例需要一对**固定的**自签密钥对（断言私钥导出的公钥 == 证书里的公钥）。

正确处置是**允许，但要求它可自证**。`oa4rust/backend/crates/signature/src/lib.rs`：

```rust
/// 夹具证书必须自签（subject == issuer）——真签发链不满足；
/// 主体必须带 `test` 标记——真密钥的主体是真实主体名；
/// 私钥必须与证书配对——成对出现是「测试夹具」的定义。
#[test]
fn test_fixture_key_pair_is_self_signed_and_marked_as_test() { /* … */ }
```

任一条不满足即红 ⇒ 有人把真实密钥换进夹具时当场被拦下，而不是等到泄漏。

**为什么这条守卫必须有牙**：守卫本身就是「私钥入库」的唯一合法口子，如果它只检查「文件在不在」，就等于给真密钥开了后门。

### 噪声边界：哪些不动

- **上游镜像文档正文**（`docs/linux-7.1.3/` 里 `/home/jdike/`、`/home/rob/proj`）—— 那是 Linux 内核上游作者的路径，不是本仓信息；改它反而破坏翻译保真度。
- **历史账本里的仓外探针引用**（`D:/<LOCAL_ROOT>/l248_probe.py`）—— 是取证记录的事实陈述。但路径前缀要脱敏 ⇒ `<OUTSIDE_REPO>/l248_probe.py`。
- **账本的行内真实 CR 字符** —— 见下。

## Why This Matters

「不提交密钥」靠人记着，就等于没有。本轮把它拆成三个可执行的强制点：**gitignore 兜底（提交前拦）→ 可移植写法（写代码时就不产生）→ 守卫测试（混入时当场红）**。

三层缺一不可：
- 只有规则 → 新建 `.env` 照样提交；
- 只有 gitignore → 硬编码路径继续在代码里扩散，换机器继续坏；
- 只有前两层 → 测试夹具这类「合法私钥」无判据，真密钥能混进去。

同时，**分级判据比一刀切更重要**：上游文档的 `/home/<作者>/` 误伤一次，守卫就会被加白名单绕过 —— 那比没有守卫更糟。

## When to Apply

- 仓库要推到公开远端；
- 新增子项目 / 新语言时（照抄 `secrets_leak_test.go` 的守卫形态）；
- 写一次性脚本 / 生成器 / 运维文档时（涉及路径与凭据）；
- 需要在仓库里放测试夹具密钥时。

## 批量脱敏的坑（本轮实测）

用脚本批量把路径替换成占位符时，有三处**内容损坏**风险，全部实测踩过：

1. **行内真实 CR 被当行尾删掉** —— 账本正文讨论「每行都前接一个 CR」时引用了真实 CR 字符（0x0D）。按行处理（`read_text().split("\n")`）会把它清掉，**正文语义被改**（`` `\r` `` 变成 `` `\n` ``）。⇒ **全程用 bytes 操作**，只做字面量替换，绝不按行处理。
2. **CRLF → LF 规范化混入** —— `write_text` 会按平台换行改写文件，产生与任务无关的整仓行尾 diff。⇒ 用 `write_bytes`。
3. **比较基准用错** —— `git show` 输出的是 LF 规范化后的内容，而工作树可能是 CRLF；拿 `git show` 当基准比行尾必然误报。⇒ 要比就用 `git cat-file blob`（仓库原始存储形态），并归一化行尾后再比。

核对方式也要对：按字符做 diff 会把**一个**路径 token 拆成多段显示（长度不同导致对齐错位），看起来像"多处非路径改动"。⇒ **先把路径 token 整体归一化成 `<PATH>` 再逐字节比对**。

## Related

- 规则与判据：`AGENTS.md`「Secrets & Environment Data」
- 忽略规则：`.gitignore`「Environment secrets」段
- 守卫形态（新增子项目照抄）：`laiyipao/server/internal/domain/secrets_leak_test.go`
- 测试夹具守卫：`oa4rust/backend/crates/signature/src/lib.rs`
- 既有路径越界治理：`../security-issues/api-path-whitelist-and-write-authz.md`
- **改历史的机制与坑**：`../development-workflow/git-history-scrub-mechanics-and-pitfalls.md`
  （本篇讲「什么算真值、怎么写」，那篇讲「已经进了历史怎么清」：
  filter-repo 在 Windows 上会把 blob 写成 CRLF、其清理会删掉被 ignore 的本地目录、
  `--refs` 不重写 `refs/remotes/*`，以及 rebase 范围容易数错一位等六个坑）
