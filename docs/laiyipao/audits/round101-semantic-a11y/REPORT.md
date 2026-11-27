# R101 语义化无障碍真浏览器复核报告（round101-semantic-a11y）

## §1 范围与方法

- **对象**：t264 实施的四处语义化补强（承接 t263 规范 RED 1 failed|5 passed → t264 实施 GREEN 9/9 + r61 5/5）。
- **四改点（源码只读核实全部落地）**：
  - `laiyipao/miniapp/src/pages/index/index.vue:119` — `.ann-item` 补 `:aria-expanded="String(expanded.has(a.id))"`（位于 `@keydown.enter` 之后）。
  - `laiyipao/miniapp/src/pages/enemies/enemies.vue:102` — `.resist-toggle` 开标签末补 `:aria-expanded="String(expandedEnemy === en.id)"`。
  - `laiyipao/miniapp/src/pages/battles/battles.vue:190` — `.battle-row` 补 `:aria-expanded="String(expandedId === it.id)"`。
  - `laiyipao/miniapp/src/pages/me/me.vue:103-104` — `nicknameMsg` 文本补 `role="status"` + `aria-live="polite"`。
- **方法**：真 Chromium（agent-browser 0.41.1），三视口 390×844 / 768×1024 / 1280×800；每改点经 `open --init-script` 注入合成数据使控件自然渲染（后端 8080 未起，见 §5 诚实边界），DOM 探针取 before→click→open→click→close 三态，AX snapshot 抓无障碍树；截图 sha256sum 以落盘实物为准。
- **键盘不回退**：三处展开控件均保留 `role="button"` / `tabindex="0"` / `@click` / `@keydown.enter` 同名 handler（探针每态均回读 `role=button`、`tab=0`；契约 it6 静态守），本轮未改动键盘三件套，R100 的 Tab/Enter 结论继续有效。

## §2 三视口四改点 aria-expanded 切换证据

三视口数据完全一致；每格为 DOM 探针实测值（`ariaExpanded` 为编译后 DOM 属性值）。

| 视口 | 改点 | before | open（点击后） | close（再点击后） | 零横溢 |
|---|---|---|---|---|---|
| 390×844 | index `.ann-item`（n=2） | ariaExpanded=false / bodyClamped=true（两条） | 第一条 true / bodyClamped=false；第二条 false | 两条 false / bodyClamped=true | iw=cw=sw=390, hOverflow=false |
| 768×1024 | index `.ann-item`（n=2） | ariaExpanded=false / bodyClamped=true（两条） | 第一条 true / bodyClamped=false；第二条 false | 两条 false / bodyClamped=true | iw=cw=sw=768, hOverflow=false |
| 1280×800 | index `.ann-item`（n=2） | ariaExpanded=false / bodyClamped=true（两条） | 第一条 true / bodyClamped=false；第二条 false | 两条 false / bodyClamped=true | iw=cw=sw=1280, hOverflow=false |
| 390×844 | enemies `.resist-toggle`（n=1） | false / 抗性图 ▼ / radar=0 | true / 收起抗性图 ▲ / radar=1 | false / 抗性图 ▼ / radar=0 | iw=cw=sw=390, hOverflow=false |
| 768×1024 | enemies `.resist-toggle`（n=1） | false / 抗性图 ▼ / radar=0 | true / 收起抗性图 ▲ / radar=1 | false / 抗性图 ▼ / radar=0 | iw=cw=sw=768, hOverflow=false |
| 1280×800 | enemies `.resist-toggle`（n=1） | false / 抗性图 ▼ / radar=0 | true / 收起抗性图 ▲ / radar=1 | false / 抗性图 ▼ / radar=0 | iw=cw=sw=1280, hOverflow=false |
| 390×844 | battles `.battle-row`（n=1） | false / expandedCls=false / detail=0 | true / expandedCls=true | false / expandedCls=false | iw=cw=sw=390, hOverflow=false |
| 768×1024 | battles `.battle-row`（n=1） | false / expandedCls=false / detail=0 | true / expandedCls=true | false / expandedCls=false | iw=cw=sw=768, hOverflow=false |
| 1280×800 | battles `.battle-row`（n=1） | false / expandedCls=false / detail=0 | true / expandedCls=true | false / expandedCls=false | iw=cw=sw=1280, hOverflow=false |

- `ariaCount`（DOM 中 `aria-expanded` 属性数）：index=2（两条公告各一）、enemies=1、battles=1，三视口一致；改前该计数为 0（见 §5）。
- index 展开视觉佐证用 `bodyClamped`（`.ann-body :class="{ clamped: !expanded.has(a.id) }"`）：true→false→true 与 aria-expanded false→true→false 同步；`.ann-item` 自身的 `open` class 探针恒为 false 系该类由另一条件控制，不影响本轮 aria-expanded 判定。
- enemies 展开视觉佐证用 radar 容器数 0→1→0；battles 用 `expandedCls` false→true→false。三处 aria-expanded 与各自展开态**同源同值**，无漂移。

### me nicknameMsg 失败态 status 证据

构造：me 页点「改名」→ 编辑态 input 清空 → 点「保存」→ 本地校验 `!name` → `nicknameMsg="昵称不能为空"`（走本地校验，无网络依赖）。

| 视口 | before statusNodes/liveNodes | after `probe-me` | 精确 `probe-status` | AX |
|---|---|---|---|---|
| 390×844 | [] / [] | text=昵称不能为空, statusCount=1, liveCount=1 | UNI-TEXT role=status, ariaLive=polite, cls=dim, text=昵称不能为空 | `- status` → `- StaticText "昵称不能为空"` |
| 768×1024 | [] / [] | text=昵称不能为空, statusCount=1, liveCount=1 | 同上 | `- status` → `- StaticText "昵称不能为空"` |
| 1280×800 | [] / [] | text=昵称不能为空, statusCount=1, liveCount=1 | 同上 | `- status` → `- StaticText "昵称不能为空"` |

- **编译结构说明**：`probe-me` 命中的是内层 `SPAN`（role=null/ariaLive=null），而 `role="status"`/`aria-live="polite"` 落在 Vue 编译后的外层 `UNI-TEXT` 容器上；`probe-status` 精确列 `[role=status]` / `[aria-live]` 节点坐实该容器承载两属性，AX 中出现 `- status` live region。改前该错误文本仅为 `- StaticText`（见 §5）。

## §3 截图与亲读

截图落盘 `docs/laiyipao/audits/round101-semantic-a11y/screenshots/`，共 12 张（4 改点 × 3 视口，均取展开/失败态）。sha256 为 `sha256sum` 对落盘实物计算，12 张全互异（UNIQUE 12/12）。

| 截图 | sha256 | 截图 | sha256 |
|---|---|---|---|
| r101-ann-390.png | 58d620b3019ec56e08c591c3595214bbcb861b57285871f13f401fef6a691f68 | r101-ann-768.png | 54cf742b69330b7c891aedd2ba8c90bf2dbfa7deb7d435d2e857570e0bfe6996 |
| r101-ann-1280.png | 46917d17a12201c92b04f0c5bb61edc4d3dbf5c47e7aefcaa85bff96017e75cd | r101-resist-390.png | f95c9541abfc6c6c483f40315b49cf35775d3955ef980bf8611a096e213ab54f |
| r101-resist-768.png | b633a2704cd04237b2fba82cd39e6b8f6f4516648958f919d7af1436eec455bd | r101-resist-1280.png | 4053ec7320705610d6fe8295e85048ee6654cf80a7a7e9983279bcf7d4733d59 |
| r101-battle-390.png | a2f94e88f1cbaf35a79ce30cc5db7306d4c889a9cd05e3cc34cc4d6e88445b38 | r101-battle-768.png | 2ce0d952eed8e19fdae5738c4fefbb9e854ca70d6b90d8cf5a6eaa2709ec7b69 |
| r101-battle-1280.png | c683277f5a123210e9ac21cff12a49e3d22d8b1d3997f3d70b6dbf8f001815a2 | r101-me-390.png | 1fa1563ad2186df8825d49b10bc27220c0192e156a360c69a27670c4b619ffcd |
| r101-me-768.png | 351377ee8e8831e097cb7458d37eef05e264316eb12bbf6b7d46617f8afdcae4 | r101-me-1280.png | fb4b0f291ef78cee4509b1bcc07b9a0557d5ee46a77ee8ad22b5422eb9807e05 |

**亲读 ≥2 张非登录页**：
- `r101-ann-768.png`（768×1024，sha 54cf742b…）：首页「QA玩家」游客态，公告区「QA公告一 / 11-27」处于展开态（⌃ 向上、显示完整正文），非登录页，与 AX expanded=true 一致。
- `r101-me-1280.png`（1280×800，sha fb4b0f29…）：指挥官档案页，昵称编辑框内「昵称不能为空」红字提示可见，非登录页，与 AX `- status` / StaticText 一致。

## §4 改前基线对照

引用前序侦察报告（本轮不重造改前版本）：
- `docs/laiyipao/recon/ROUND101_A_ARIA_EXPANDED_RECON.md`（t261）：改前三视口视觉展开生效（annClamped 3↔2 / toggle.open，radarWrap 0↔1），但 DOM `ariaExpanded=null`、页面 `aria-expanded` 属性计数 0、AX expanded 0/0；battles 游客 0 战报未坐实。对应改前截图 6 张 sha 见该报告（`r101a-index-{390,768,1280}` / `r101a-enemies-{390,768,1280}`）。
- `docs/laiyipao/recon/ROUND101_B_RESULT_ANNOUNCE_RECON.md`：改前 me 失败态「昵称不能为空」AX 仅 `- StaticText`，零 live/status；AX 对照实验证 `role=status`+`aria-live=polite` → `- status` 节点。对应改前 me 截图 3 张 + login 对照 1 张 sha 见该报告。
- 结论：本轮改后三处 `aria-expanded` 由「计数 0 / AX expanded 0」变为「计数 2/1/1 / AX expanded=true」；me 错误文本由「仅 StaticText」变为「`- status` live region」，四处缺口均关闭。

## §5 诚实边界

- **后端未起**：8080 未监听，所有页面数据经 `open --init-script` 注入合成响应（`r101-init.js`：patch XHR.open/send 记录请求、send 直接 reply 合成体不发真实请求、setTimeout 触发 onload；patch window.fetch）。公告 2 条、敌兵 1 条、战报 1 条（id 9001）、profile 均为合成数据，**非真值**。占位令牌 `r101-qa-placeholder-token` 仅内存注入 localStorage，无真值入库。
- **QA 库只读**：DSN 未暴露（NO_DSN_SKIP），本轮未写库；历史记录玩家 477 存在 / 104 无记录（沿用前轮结论，未在本轮复核）。
- **me 失败态构造**：本地校验触发（清空昵称后保存），无网络依赖；`role=status` 落在编译后外层 `UNI-TEXT`，内层 `SPAN` 无语义属性，已由 `probe-status` + AX 双证坐实，不冒充为 span 自身属性。
- **键盘**：本轮以探针回读 role/tab 与静态契约确认三件套未回退，未重复执行 Tab/Enter 动态用例（R100 已坐实，且本轮未改 handler 绑定）。
- **截图亲读 2/12**：仅 r101-ann-768 与 r101-me-1280 人工目视，其余 10 张以 sha + 探针/AX 数据为准。
- **Vue 测试告警**：定向/全量 vitest 输出含 `Failed to resolve directive: aria-native` 等既有告警，非本轮引入，测试结论以 RC=0 与 passed 计数为准。

## §6 门禁、静态回归与 hygiene

门禁（`data/gates-out.txt`，job exit 0，G1–G9 全 RC=0）：

| 门 | 内容 | 结果 | RC |
|---|---|---|---|
| G1 | r101 语义契约 `r101SemanticA11y.contract.test.ts` | Tests 9 passed (9) | 0 |
| G2 | r61 表单契约 | Tests 5 passed (5) | 0 |
| G3 | R60 键盘范式契约 | Tests 6 passed (6) | 0 |
| G4 | R100 交互行契约 | Tests 12 passed (12) | 0 |
| G5 | 定向 4 文件（index/battles/enemies_fresh_load/me） | Test Files 4 passed；Tests 71 passed (71) | 0 |
| G6 | miniapp 全量 | Test Files 233 passed \| 1 skipped (234)；Tests 2141 passed \| 2 skipped (2143) | 0 |
| G7 | vue-tsc（type-check） | 通过 | 0 |
| G8 | build:h5 | 通过 | 0 |
| G9 | git diff --check | 通过（仅既有 .gitignore 行尾告警） | 0 |

- **静态回归（OPS grep 实测）**：`tokens.css:14/15/16` 三锚点 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 在；业务 `laiyipao/miniapp/src` 下零 `--r101-`（NO_MATCH，PASS）；`aria-expanded` 三处 index:119 / enemies:102 / battles:190 各在。
- **hygiene**：HEAD=`187aeb0eeaba475f9e2eb4bf5e9b74175699973b` 未变；`git status` 非 `??` 项仅 `.gitignore` + 四页 vue（battles/enemies/index/me）+ `r61FormA11y.contract.test.ts`，均属 t264 预期写域；auth 五路径（`.env` / `laiyipao/admin/.env.local.keys` / `laiyipao/deploy/.local.keys` / `laiyipao/server/.local.keys` / `laiyipao/miniapp/.env.local`）全 ABSENT；零真值；本轮自启 H5 dev server（5174）于收尾 kill 并复验端口关闭。

## §7 判定

四处语义化补强在三视口真 Chromium 全部坐实：三处 `aria-expanded` 随展开态 false→true→false 同步切换且 AX 可感知，me 错误文本成为 status live region；改前基线缺口全部关闭；键盘三件套未回退；截图 sha 与落盘实物一致、亲读 2 张非登录页；门禁 G1–G9 全绿、静态回归与 hygiene 通过。合成数据非真值与 QA 库 DSN 未暴露已如实披露。**判定 PASS**。
