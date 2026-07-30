---
title: Zero-Secret Migration in cool/ - Plan
type: refactor
date: 2026-07-30
topic: zero-secret-migration
artifact_contract: ce-unified-plan/v1
artifact_readiness: implementation-ready
product_contract_source: ce-brainstorm
execution: code
---

## Goal Capsule

- **Objective:** 将 `cool/` 目录下 3 个源码文件中的硬编码生产密钥提取为纯环境变量注入（未设置时明确报错退出），用 `git filter-repo` 重写历史清除已泄露的密钥，并轮换实际凭证。
- **Authority:** systems-documentation plan R11 要求文档中无真实密钥，但源码中仍存在——必须先修源码才能写文档。
- **Blocker (resolved):** EncryptUtil.java 确认为手写源码（`src/main/java/org/example/EncryptUtil.java`），非 Maven 生成产物，安全修改。
- **Stop conditions:** 编译失败；token 输出与迁移前不一致；grep 仍发现明文密钥；git filter-repo 未安装且无替代方案。

---

## Product Contract

### Summary

将 `cool.java`、`cool2.java`、`EncryptUtil.java` 中的 3 组硬编码密钥全部改为环境变量注入，废弃未完成的 `cool2.java`，删除零调用者的 `decodeTicket()`，通过 `git filter-repo` 清除 git 历史中的明文密钥，并同步轮换 O2OA 平台的凭证。这是写系统文档的前置条件。

### Problem Frame

`cool.java:162-166` 的 `oaMd5()` 方法和 `cool.java:197` 的 `decodeTicket()` 硬编码了 3 组生产密钥。相同的密钥在 `cool2.java:104-108` 和 `EncryptUtil.java:172-176,207` 中重复出现。这些密钥已经进入 git 历史，任何能访问仓库的人都能通过 `git log -p` 恢复。当前系统文档计划 R11 只约束"文档输出中不出现真实密钥"，不约束源码——如果先写文档，文档会描述一个"已修复"的架构，而实际源码仍有生产密钥。

`decodeTicket()` 在仓库中无任何外部调用者（grep 确认），是已定义的死代码，其硬编码密钥同样需要处理。

### Requirements

**Secret extraction**

- R1. `cool.java` 的 `oaMd5()` 方法签名改为 `oaMd5(String key, String secret)`，从环境变量读取密钥，未设置时抛出自定义异常并退出。
- R2. `cool.java` 的 `decodeTicket()` 方法删除（零外部调用者，含硬编码密钥）。
- R3. `EncryptUtil.java` 的 `oaMd5()` 方法签名改为 `oaMd5(String key, String secret)`，从环境变量读取密钥，未设置时抛出自定义异常并退出。
- R4. `EncryptUtil.java` 的 `decodeTicket()` 方法删除（零外部调用者）。
- R5. `cool2.java` 文件废弃删除（135 行未完成简化，缺少 `decodeTicket()`/`aesDecrypt()`/`BaseOut`）。

**API compatibility**

- R6. 密钥注入方式为方法参数传递（`oaMd5(String key, String secret)`），调用方显式传入从环境变量获取的值。
- R7. 所有调用点的编译和运行结果与迁移前一致（同一输入 → 同一输出 token）。

**Git history**

- R8. 使用 `git filter-repo` 清除所有历史提交中的 3 组明文密钥字符串。
- R9. 重写后的 commit hash 变更记录在交付说明中（已 clone 的用户需要 re-clone）。

**Key rotation**

- R10. 旧密钥在 O2OA 平台侧同步轮换（作废），新密钥仅通过环境变量分发，不进入任何文件系统。

### Key Decisions

- **纯环境变量，无 fallback。** 与 `watering` 的 `log.Fatal` 模式一致，未设置时明确失败而非静默使用占位符。train 的 env+fallback 模式（`os.getenv("KEY", "YOUR_KEY")`）被有意排除——占位符 fallback 会在忘记 export 时产生静默的有效性错误。
- **参数传递而非静态初始化。** `oaMd5` 的方法签名改为接受 key/secret 参数，调用方负责从环境变量获取并传入。侵入面是 13 处调用点，但语义清晰、测试友好。
- **`decodeTicket()` 删除而非修复。** grep 全仓库确认零外部调用者，是已定义的死代码。修复它只会增加迁移工作量而不产生价值。
- **3 个文件全部处理。** EncryptUtil.java 确认为手写源码（`src/main/java/org/example/EncryptUtil.java`），不是 Maven 生成产物，安全修改。
- **git filter-repo + 密钥轮换。** 历史重写清除已泄露的密钥字符串，同时 O2OA 平台侧轮换凭证使历史中的旧值失效。双保险。
- **cool2.java 废弃删除。** 135 行的未完成简化，缺少 `decodeTicket()`、`aesDecrypt()`、`BaseOut` 类，与 cool.java 功能重叠 80% 但从未完成合并。保留只会增加维护陷阱。
- **环境变量命名：`COOL_OA_KEY` / `COOL_OA_SECRET`。** 遵循仓库既有约定 `{SYSTEM}_{PURPOSE}` ALL_CAPS（参考 `DINGTALK_APP_KEY`、`BAIDU_API_KEY`）。

### Scope Boundaries

**In scope**

- `cool.java` 的 `oaMd5()` 签名修改 + 4 处调用点更新
- `cool.java` 的 `decodeTicket()` 删除
- `EncryptUtil.java` 的 `oaMd5()` 签名修改 + 5 处调用点更新
- `EncryptUtil.java` 的 `decodeTicket()` 删除
- `cool2.java` 的废弃删除
- `git filter-repo` 历史重写
- 密钥轮换的文档记录（轮换动作本身由 O2OA 平台管理员执行，不在本仓库范围内）

**Deferred for later**

- `cool/` 的测试覆盖补充（当前零测试，本次不动）
- 全局 secret 扫描 CI（可以后续做，但不是迁移本身的一部分）
- `compile_all.bat` 在迁移后的验证（属于 systems-doc plan 的回归步骤）

**Outside this scope**

- oa/ 目录下其他模块的凭证审计（本次只修 cool/ 目录）
- O2OA 平台的密钥轮换执行动作（需要平台管理员操作）
- 系统文档的编写（R11 的前置条件，但文档编写本身是独立任务）

### Dependencies / Assumptions

- **O2OA 平台接受新密钥。** 密钥轮换需要平台侧同步更新，否则迁移后 SSO 登录会失败。
- **`git filter-repo` 可用。** 仓库当前未使用 filter-repo（传统 filter-branch 已废弃），需要确认工具已安装。
- **`COOL_OA_KEY` / `COOL_OA_SECRET` 环境变量在部署时已设置。** 遵循仓库既有命名约定 `{SYSTEM}_{PURPOSE}` ALL_CAPS。

### Outstanding Questions

- **O2OA 平台密钥轮换的协调方式**（blocks R10 交付）—— 需要确认谁持有平台管理员权限，以及轮换是否需要停机窗口。

---

## Planning Contract

### Key Technical Decisions

- **环境变量注入点定位在方法签名层。** `oaMd5(String key, String secret)` 替代 `oaMd5()`，密钥从环境变量读取的位置在 `main()` 的调用点而非方法内部。这使得 env var 缺失时的失败信号在调用点显式触发，而非在加密函数内部静默失败。
- **`decodeTicket()` 整体删除。** 仓库 grep 确认零外部调用者，是已定义的死代码。修复它只会增加迁移工作量而不产生价值。
- **`git filter-repo` 而非 `filter-branch`。** `filter-branch` 已废弃且对大型仓库性能差；`filter-repo` 是官方推荐的替代工具，支持精确的字符串替换。
- **密钥轮换与历史重写并行。** 仅靠历史重写不够——密钥可能已通过其他渠道泄露。平台侧轮换使历史中的旧值彻底失效。
- **EncryptUtil.java 直接编辑。** 确认为手写源码（`src/main/java/org/example/EncryptUtil.java`），编译输出到 `target/` 且 `.gitignore` 已排除，安全修改。

### Assumptions

- `git filter-repo` 可通过 `pip install git-filter-repo` 或系统包管理器安装
- 新密钥已由 O2OA 平台管理员生成并准备分发
- `COOL_OA_KEY` 和 `COOL_OA_SECRET` 环境变量在部署前设置
- 迁移后 `cool.java` 和 `EncryptUtil.java` 的 token 输出与迁移前一致（同一密钥值 → 同一 MD5 → 同一 AES 加密结果）

---

## Implementation Units

### U1. 提取 cool.java 的密钥并修改调用点

**Goal:** 将 `cool.java` 中 `oaMd5()` 的硬编码密钥提取为方法参数，更新 4 处调用点，删除 `decodeTicket()`。

**Requirements:** R1, R2, R4, R5, R6, R7

**Dependencies:** 无

**Files:**
- `cool/cool.java` — 修改

**Approach:**
- 修改 `oaMd5()` 签名为 `public static String oaMd5(String key, String secret)`，移除方法体内的硬编码字符串
- 在 `main()` 的 4 处调用点（214, 220, 226, 232），从环境变量获取密钥后传入：`oaMd5(System.getenv("COOL_OA_KEY"), System.getenv("COOL_OA_SECRET"))`
- 在 `main()` 入口处检查两个 env var，任一为空则打印错误信息并 `System.exit(1)`
- 删除 `decodeTicket()` 方法（含行 197 的硬编码密钥）

**Patterns to follow:** `watering/main.go:26-29` 的 env var 校验模式（未设置时明确失败）

**Test scenarios:**
- 正常路径：`COOL_OA_KEY` 和 `COOL_OA_SECRET` 均已设置，`oaMd5()` 返回正确的 MD5 哈希
- 缺失路径：任一 env var 未设置，程序打印错误并退出（非静默失败）
- 输出一致性：使用与迁移前相同的密钥值运行，4 个 token 的输出与迁移前一致

**Verification:** `mvn compile` 通过；grep `cool.java` 无硬编码密钥字符串；`java cool` 输出与迁移前一致

---

### U2. 提取 EncryptUtil.java 的密钥并删除 decodeTicket()

**Goal:** 将 `EncryptUtil.java` 中 `oaMd5()` 的硬编码密钥提取为方法参数，删除 `decodeTicket()` 方法。

**Requirements:** R3, R4, R5, R6, R7

**Dependencies:** U1

**Files:**
- `cool/src/main/java/org/example/EncryptUtil.java` — 修改

**Approach:**
- 修改 `oaMd5()` 签名为 `oaMd5(String key, String secret)`，移除方法体内的硬编码字符串
- 更新 `main()` 中 5 处 `oaMd5()` 调用点（220, 226, 234, 242, 250），从环境变量获取密钥后传入
- 完全删除 `decodeTicket()` 方法（含 `decryptData(byte[])` 中的硬编码密钥）
- 删除 `main()` 中打印 `OaMd5:` 的调试输出（如存在）

**Patterns to follow:** U1 的修改模式

**Test scenarios:**
- 正常路径：env var 已设置，`oaMd5()` 返回正确的 MD5 哈希
- 缺失路径：env var 未设置，程序打印错误并退出
- `decodeTicket()` 删除后，grep 确认全仓库无调用引用

**Verification:** `mvn compile` 通过；grep `EncryptUtil.java` 无硬编码密钥字符串；grep 全仓库无 `decodeTicket` 调用

---

### U3. 删除 cool2.java

**Goal:** 移除未完成的简化副本，消除密钥残留和维护陷阱。

**Requirements:** R5

**Dependencies:** U1

**Files:**
- `cool/cool2.java` — 删除

**Approach:**
- 删除 `cool/cool2.java` 文件
- 确认 `compile_all.bat` 中没有引用 `cool2.java` 的步骤
- 确认 `docs/systems/cool.md` 中没有引用 `cool2.java` 的内容

**Test scenarios:**
- `mvn compile`（通过 cool.java）仍然通过
- `grep -r "cool2"` 全仓库无引用

**Verification:** `cool/cool2.java` 不存在；`grep -r "cool2"` 无结果

---

### U4. 重写 git 历史清除密钥

**Goal:** 使用 `git filter-repo` 清除所有历史提交中的 3 组明文密钥字符串。

**Requirements:** R8, R9

**Dependencies:** U1, U2, U3（当前代码已无硬编码密钥，历史重写才有效）

**Files:**
- `.git/` — 操作对象（历史重写）
- 交付说明 — 记录 commit hash 变更

**Approach:**
- 确认 `git filter-repo` 已安装：`git filter-repo --help`
- 若未安装，`pip install git-filter-repo`
- 创建密钥替换表达式文件，定义 3 组密钥的替换规则
- 执行 `git filter-repo --replace-text <expr-file>`
- 验证：`git log -p --all | grep "coolcollege20201211sc"` 应无结果

**Test scenarios:**
- `git log -p --all` 中无任何 3 组密钥字符串
- `git rev-parse HEAD` 返回新的 commit hash（与重写前不同）
- 仓库仍可正常 `git log`、`git status`、`git diff`

**Verification:** grep 全历史无密钥；git 操作正常；交付说明已记录 hash 变更

---

### U5. 验证迁移完整性 + 密钥轮换文档

**Goal:** 确认迁移后的系统行为与迁移前一致，记录密钥轮换操作。

**Requirements:** R7, R10

**Dependencies:** U1, U2, U3, U4

**Files:**
- `docs/plans/2026-07-30-001-refactor-zero-secret-migration-plan.md` — 更新验证结果
- `docs/systems/cool.md` — 更新密钥管理说明（迁移后可反映真实机制）

**Approach:**
- 使用与迁移前相同的测试密钥值运行 `cool.java`，确认 4 个 token 输出一致
- 使用与迁移前相同的测试密钥值运行 `EncryptUtil.java`，确认输出一致
- 全仓库 grep 确认无硬编码密钥残留（包括 `.class` 文件意外提交的情况）
- 记录密钥轮换的执行确认（平台侧已完成轮换 → 旧密钥作废）

**Test scenarios:**
- Token 一致性：相同输入 → 相同输出（迁移前后对比）
- 无残留：`grep -rn "coolcollege20201211sc\|135990bd839c5fe0a1ca9cbee2475431\|whM1376SiX5=78" cool/` 无结果
- 环境变量缺失：移除 env var 后运行，程序明确报错退出

**Verification:** 所有 grep 检查通过；token 输出一致；env var 缺失时明确失败

---

## Verification Contract

**Compile verification:**

```bash
cd cool && mvn compile -q
```

**Secret scan — current code:**

```bash
grep -rn "coolcollege20201211sc\|135990bd839c5fe0a1ca9cbee2475431\|whM1376SiX5=78" cool/
```

Expected: no results.

**Secret scan — git history (after U4):**

```bash
git log -p --all | grep "coolcollege20201211sc\|135990bd839c5fe0a1ca9cbee2475431\|whM1376SiX5=78"
```

Expected: no results.

**Token equivalence verification:**

使用迁移前已知的测试密钥值，分别运行迁移前后的代码（或对比已知正确的 token 输出），确认 4 种身份类型的 token 一致。

**Dead code verification:**

```bash
grep -rn "decodeTicket" cool/
```

Expected: no results.

**Git integrity verification:**

```bash
git log --oneline | head -5
git status
```

Expected: log shows new commit hashes; status is clean.

---

## Definition of Done

- [ ] `cool.java` 中无硬编码密钥，`oaMdly()` 接受 key/secret 参数
- [ ] `cool.java` 中 `decodeTicket()` 已删除
- [ ] `EncryptUtil.java` 中无硬编码密钥，`oaMd5()` 接受 key/secret 参数
- [ ] `EncryptUtil.java` 中 `decodeTicket()` 已删除
- [ ] `cool2.java` 已删除，全仓库无引用
- [ ] `mvn compile` 通过（`cool/` 和 `oa/` 均可编译）
- [ ] `grep` 当前代码无 3 组明文密钥
- [ ] `git log -p` 全历史无 3 组明文密钥
- [ ] Token 输出与迁移前一致（相同输入 → 相同输出）
- [ ] 环境变量缺失时程序明确报错退出（非静默失败）
- [ ] 密钥轮换已在 O2OA 平台侧完成（旧密钥作废）
- [ ] 交付说明记录了 git hash 变更
