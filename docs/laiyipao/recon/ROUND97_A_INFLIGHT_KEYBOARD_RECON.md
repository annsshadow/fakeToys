# R97-A 侦察报告：在途态按钮键盘通道与防连点守卫一致性

- 轮次：R97-A
- 任务：t233（attempt_id=779aa365-af34-4954-9a5d-f9326b3570ae）
- 角色：design-reviewer-wb（独立终审/只读侦察）
- 基线 HEAD：`586f77c227642bce4f69385d1f9ec7be64053e06`（收尾一致）
- 工作根：`D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248`
- 结论：**无推荐修复候选**。键盘可达的在途态按钮全部已有 handler 首行 in-flight guard，重复提交已被挡住，四条件不成立；不硬凑候选。

## §0 任务与范围

只读侦察「在途态按钮（`btn-disabled` / applying 态）键盘通道（role=button + tabindex=0 + @keydown.enter）与防连点守卫一致性」。
判定问题：是否存在『键盘 Enter 可绕过鼠标 `pointer-events:none` 在途态造成重复提交』的真实用户可见缺陷。
**未写任何业务源码**；`git diff --name-only` 为空（无已跟踪文件改动），`git rev-parse HEAD` 仍为基线值。

## §1 方法与证据来源

1. 静态审计：逐处核实所有 `btn-disabled` / applying 态按钮的模板事件绑定，追踪 `@click` 与 `@keydown.enter` 指向的 handler，核对 handler 首行守卫。
2. 历史范式对照：R268/R269/R270/R66/R273/R275/R285 已确立的 in-flight guard 口径。
3. 真浏览器坐实：H5 dev server（端口 5175，`curl` HTTP 200），agent-browser CLI 0.41.1。

## §2 静态审计结果（决定性）

### 2.1 唯一真正键盘可达的在途态按钮：4 处 apply 按钮

同时带 `role=button` + `tabindex=0` + `@keydown.enter` 且处于 `btn-disabled` 态的按钮，**仅 4 处 apply 确认按钮**：

| 文件 | 模板行 | applying 变量 | @click / @keydown.enter 指向 |
|---|---|---|---|
| laiyipao/miniapp/src/pages/battles/battles.vue | 344 | r14Applying | r14ConfirmApply() / r14ConfirmApply() |
| laiyipao/miniapp/src/pages/equipment/equipment.vue | 247 | r7Applying | r7ConfirmApply() / r7ConfirmApply() |
| laiyipao/miniapp/src/pages/reactions/reactions.vue | 277 | r14Applying | r14ConfirmApply() / r14ConfirmApply() |
| laiyipao/miniapp/src/pages/skills/skills.vue | 285 | r7Applying | r7ConfirmApply() / r7ConfirmApply() |

四处 `@click` 与 `@keydown.enter` **指向同一函数**，无分叉。四个 handler 全部有 in-flight guard 早返回（`tools.grep` 实测确认）：

- `laiyipao/miniapp/src/pages/battles/battles.vue:795`：`if (!a || a.status !== 'ready' || r14Applying.value) return`
- `laiyipao/miniapp/src/pages/equipment/equipment.vue:544`：`if (!impact || r7Applying.value) return`
- `laiyipao/miniapp/src/pages/reactions/reactions.vue:602`：`if (!a || a.status !== 'ready' || r14Applying.value) return`
- `laiyipao/miniapp/src/pages/skills/skills.vue:642`：`if (!impact || r7Applying.value) return`

四处均为 `设 applying=true → await → finally 复位` 结构。结论：鼠标 `pointer-events:none` 只挡鼠标；键盘 Enter 在在途态可触发一次，但第二次调用被 handler 首行 guard 挡回，**无重复提交**。

### 2.2 其余 btn-disabled 按钮：键盘根本不可达

其余 `btn-disabled` 按钮均为裸 `<view class="btn" ... @click>`，无 `role` / `tabindex` / `@keydown.enter`，**不在 Tab 序列**，键盘 Enter 无法触发（不构成在途态键盘通道问题）：

- `verify.vue:67`（fetching→fetchReplay，`:452` `if(fetching.value)return`）、`verify.vue:150`（falsifying→doFalsify，`:529` guard）
- `stage.vue:324`（!canEnter→startBattle，`:768` battleNavigating 同步锁）
- `signin.vue:71`（canSign→doSignIn，`:416` guard）、`signin.vue:185`（claim，`:454` claimingIds Set guard）
- `me.vue:299`（challenge，`:1582` simulating guard）、`me.vue:97/:596/:648`（nickname/password/upgradeSubmitting，各 handler 首行 guard：`:1137/:1187/:1247`）

> 裸 view 按钮键盘不可达属 R96-A 已记录的 WCAG 盲区，非本轮『在途守卫』问题，避免重复立案。

### 2.3 全局 CSS 与历史范式

- `common.css:443-446`：`{opacity:.4;pointer-events:none}`；`:1014-1016`：`{cursor:not-allowed}`；`:1597`：`.r7-btn.btn-disabled{opacity:.55}`。
- `login.vue:274-282` 注释确认：`pointer-events:none` 只挡鼠标，`submit()` 用 `if(submitting.value)return` 兜底防重入。
- 历史守卫权威口径：R268（signin 兑换在途）、R269（signin claimingIds）、R270（signin signingIn）、R66（round66-gem-cache-guard/REPORT.md）、R273（me simulating）、R275（me saveDefense/toggleShield）、R285（verify replaying）——**均为 handler 首行 in-flight guard 范式，系统性防护已存在**。

## §3 真浏览器坐实

- **已坐实（前序 DOM 文本探针，session lyp-r97a，390×844）**：`url=#/pages/verify/verify`，`title=战绩验真`，`isLogin=false`（非登录页）；`r5-note-cta` 为 UNI-TEXT `role=button tabindex=0`；「取复现信息」为 UNI-VIEW `role=null tabIndex=-1`——**实测证实裸 view 按钮不在 Tab 序列**。input[type=number] 存在。
- **未坐实（如实记录）**：`verify.vue` 的 `fetchReplay` / `doFalsify` 在途态难稳定构造（`network route` 仅支持 `--body`/`--abort`，无 delay；Vue 组件实例探针多次返回 `Invalid response: EOF while parsing a value at line 1 column 0`，守护进程不稳定）。按 captain 收紧指示，不反复重试，转入静态裁定。
- **本次重复截图为白屏，不计为有效坐实**：`docs/laiyipao/audits/round97-a-inflight-keyboard/r97a-verify-390.png`（sha256 `da058609513da16a28e69fca9afd07de614f441c43563b6d7d79f81148345088`，1258×588，3483B）经亲读为纯白画面；不推翻 §3 前序 DOM 文本实测，也不作为新证据。

## §4 四条件逐项裁定

| # | 判据 | 裁定 | 依据 |
|---|---|---|---|
| ① | 真实用户可见缺陷 | **不成立** | 4 处键盘可达 apply 按钮均有 handler 首行 guard；键盘可触发一次，第二次被挡，无重复提交 |
| ② | 可定位到具体可修复点 | **无** | 现有 guard 已覆盖唯一键盘可达路径；裸 view 按钮不在 Tab 序列，无键盘通道 |
| ③ | 修复不越界（不碰保护面） | 不适用 | 无候选修复 |
| ④ | 不与既有范式/历史轮次冲突 | 不适用 | 现有实现即团队既有 guard 范式（R268/R269/R270/R273/R275/R285） |

## §5 结论

**无推荐修复候选**。本轮在途态按钮键盘通道与防连点守卫一致——键盘可达路径全部有守卫，不构成重复提交缺陷。按『无推荐候选不硬凑』原则，不产出修改建议。

## §6 诚实边界

- 在途态真浏览器交互坐实未完成（§3 已如实标注）；裁定以静态审计决定性证据 + 前序 DOM 文本实测为准。
- 白屏截图仅作过程记录，不作为证据。
- 全程未写业务源码；HEAD 与基线一致；`git diff --name-only` 为空。
- 未回显任何真值（token / 域名 / IP / 个人路径）。
