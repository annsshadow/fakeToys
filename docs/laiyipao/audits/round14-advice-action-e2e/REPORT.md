# R14 真浏览器端到端闭环复查报告（advice-action 一键替换）

> 任务：t127 · 非物理真机（桌面 Chrome 157 + agent-browser 0.41.1，H5 视口 390×844）
> 环境：本地回环 QA（H5 :5174 / API :8080 / PG :15440 回环映射），均在内存注入真值，报告与截图零真值。
> 日期：2026-11-23

## 1. 结论速览

| 项 | 结果 |
| --- | --- |
| 战报页端到端闭环（建议→一键→两步确认→写入→DB 落库） | **PASS（真实可达）** |
| 防作弊空态（未拥有候选无按钮） | **PASS** |
| 非法元素 / 游客空态 | **PASS** |
| 图鉴页端到端闭环 | **FAIL — 发现真实缺陷 R14-BUG-1【高】**，核心功能不可达 |
| 探针 pe/ce/af/fr | 全 0（所有受测页面） |
| 真值文本 + 截图二进制 | 零泄露 |
| R1–R13 不回退 | PASS（源码锚点逐项核对） |

## 2. 数据准备（QA 库，测后已完全还原）

user10 原始态：4 场真实战报（近 7 天，主导元素 lightning）、skill slots `0:1, 1:4, 2:7`、拥有技能 {1,4,7,10}。
为构造成功态，仅向本地回环 QA 库临时授予 4 个已解锁的更优技能（fire:3 温压弹 / ice:6 极寒冰弹 / lightning:8 电磁穿刺 / corrosion:18 液化毒雾）。全部实测结束后执行 `delete from user_skills where user_id=10 and skill_id in (3,6,8,18)` 与 `update user_skill_slots set skill_id=4 where slot=1`，最终复核拥有={1,4,7,10}、槽=`0:1,1:4,2:7`，与原始态逐字节一致。

服务端裁决探测（带 user10 令牌）：授予前 4 元素 NO_OWNED_CANDIDATE、kinetic READY；授予后五元素全部 READY（如 lightning → cand=8 电磁穿刺 repl=4 干冰弹）。

## 3. 战报页闭环（PASS，真浏览器逐步可观测）

1. **看建议**：展开第 40 关战报详情，`.reaction-learning` 显示「焰+电→过热 ×51；下一局补毒可解锁燃烧云」。
2. **一键**：`#r14-advice-action-btn` 文案「一键替换为 电磁穿刺 →」；探针捕获 `advice/action 200, element=lightning, cand=8`。
3. **两步确认**：点 CTA 弹出 `.r7-compare-mask`，`.r7-compare-title`「当前 vs 推荐·确认替换」，行「干冰弹（当前）→电磁穿刺（推荐）」，每波伤害 2403.5→2521.1，依据 `battle_records.elements_used · reactions_used · user_skills · /config · source=battle`，按钮 [取消, 确认替换]。
4. **写入**：点「确认替换」→ `.apply-done`「已替换为「电磁穿刺」，出战配置已更新」。
5. **落库验证**：DB skill slots 由 `0:1,1:4,2:7` 变为 `0:1,1:8,2:7`（槽 1 干冰弹→电磁穿刺），证明写入经服务端事务生效而非本地乐观值。

证据：`r14-battle-01-cta-ready.png`(sha 4d51b8d7) / `r14-battle-02-compare.png`(a0dd4589) / `r14-battle-03-applied.png`(69415263)，均亲读确认为非登录页、390×844。探针 pe=0/ce=0/af=0/fr=0。

## 4. 防作弊与空态（PASS）

- 撤销授予后重载：战报卡降级为 `.r14-advice-empty`，文案「AI 暂时没有足够数据给出建议／已拥有技能里没有…基础伤害高于当前 干冰弹(id=4) 的候选…」，`#r14-advice-action-btn` **不存在**，advice 响应 `200, code=NO_OWNED_CANDIDATE, cand=0, cn=''`（证据 `r14-battle-05-empty-no-cta.png`, sha 77662e45）。
- 游客请求 `element=fire` → `200/NOT_AUTHENTICATED/cand=0`；非法 `element=INVALID_X` → `200/INVALID_ELEMENT/cand=0`、`element` 被归一为小写 `invalid_x`。防作弊红线（空态不得带非零 id/params）成立。

## 5. 真实缺陷 R14-BUG-1【高】：图鉴页建议卡片完全不可达

**现象**：图鉴页 `reactions.vue` 打开的 7 条 advice 请求全部 `200 / cand 非零（3/6/8）/ empty=null`，但 DOM 中 `.r14-advice`、`.r14-advice-empty`、`#r14-advice-action-btn` **计数全为 0**——玩家在图鉴页完全看不到任何 R14 建议或一键替换入口（证据 `r14-rx-04-bug-no-card.png`, sha 3dade418，亲读确认页面只有 R6 运营卡，无任何 R14 卡）。

**根因（跨层契约失配，三处决定性实验）**：
1. 服务端 `advice_action.go`：expected_reaction 直接取 query 的 reaction 原值；图鉴页 `reactions.vue:487` 调 `fetchAdviceAction({element, source:'reactions'})` **不传 reaction**，故响应 `expected_reaction=''`（浏览器内 fetch 对照：不传→`""`/cand=3；传 reaction=steam_burst→`"steam_burst"/cand=3`）。
2. 前端归一器 `r14AdviceAction.ts:72-73`：`const expectedReaction = str(payload.expected_reaction); if (!element || !expectedReaction) return null` —— 对空串直接判无效，返回 null。
3. 浏览器内直接 import 归一器实测：`normalizeAdviceAction({...expected_reaction:'', cand:3})` → **null**；同一对象改 `expected_reaction:'steam_burst'` → **ready/cand=3**。根因 100% 落在该行校验，与服务端、数据无关。

**规范依据**：`SPEC.md` A.1 明确 reaction「可选，仅用于依据展示，不参与鉴权与计算」，A.2/A.3 六字段契约要求 expected_reaction 存在但未要求非空；C.2 要求图鉴条目「同源渲染 CTA 或空态」。前端把可选字段当必填，属实现违背规范。

**测试盲区**：`r14AdviceActionFlow.contract.test.ts` 成功态与空态夹具均固定 `expected_reaction:'overheat'`，从未覆盖图鉴页「不传 reaction → 空串」的真实路径，故 1763 tests 全绿仍漏检。

**修复建议（交 h5-ui，test-first RED→GREEN，本任务只读未改源码）**：
- 方案 A（推荐，最小）：`normalizeAdviceAction` 去掉 `!expectedReaction` 的强制非空，改为 `expectedReaction = str(payload.expected_reaction)` 允许空串（图鉴页场景），保留 element 必填与其余防作弊校验；补契约用例「expected_reaction='' 且其余字段齐全 → ready」。
- 方案 B：图鉴页 `loadR14Advice` 按条目反应 key 传 reaction（如 steam_burst），使 expected_reaction 非空；但会把「期望反应」语义塞进图鉴条目，需产品确认。

## 6. R1–R13 不回退（源码锚点逐项核对，PASS）

首屏新手引导 `first-run-guide`（R1）；AoE 25% 范围伤害与「半径 120 像素」图例文案（R4/t41）；养成前后差值卡 `r10-growth` 在 battles/battle 两页（R10）；数值格式化 `fmtEv` 在 skills/equipment/reactions（R11）；失败复盘 `settle-defeat-review`/`defeat-review`（R12）；反应机制表「传播/溅射半径」（R13）；0 星实时差额「还差 ${target-score} 分就能拿」（R7）。全部在位，无回退。

## 7. 假阳性排除

- 战报页展开首帧 DOM 未更新（uni-app keep-alive），以 4s 等待 + 二次 eval 为准，非缺陷。
- `play() request was interrupted` 为 t6 已知音频自动播放噪声，探针 init-script 先于 sfx 过滤器捕获，单列不计业务缺陷；本轮受测页面 pe/ce 均为 0。

## 8. 产物与真值

截图 5 张（均 390×844 非物理真机）落 `screenshots/`；脚本探针落本目录；日志 `data/`。真值扫描：JWT=0、Bearer=0、私钥=0、口令字面=0、非回环域名=0；`auth-init.js`（含测试令牌）采集后已删除，`auth-init.tpl.js` 保留 `__LYP_TOKEN__` 占位符。全目录 20 文件、0 泄露。

---

## 9. R14-BUG-1 修复后真浏览器复测（t132 决定性证据）

> 缘起：t131 设计复审因缺修复后运行时证据判 needs_revision。h5-ui t130 修复 `r14AdviceAction.ts`（expected_reaction 空串仅作展示、不阻断 ready）与图鉴页空串提示后，本轮补做真浏览器复测。

**环境**：本地回环 QA（后端 :8080 / H5 :5174 / PG 回环 15440），桌面 Chrome + agent-browser 0.41.1，H5 视口 390×844，非物理真机。

**数据准备（已完全还原）**：user10 有 4 场真实战报、原始拥有技能 {1,4,7,10}、槽 `0:1,1:4,2:7`。为构造 ready 态临时授予 fire3/ice6/lightning8/corrosion18（授予后 5 元素 advice 全 READY），实测后 `delete ... skill_id in (3,6,8,18)` 并复核拥有={1,4,7,10}、槽=`0:1,1:4,2:7`，与原始逐字节一致。token 由 guest_token 经 /auth/guest 重签得 157 字符，用 document-start init-script 在应用 bootstrap 前注入，采集后已删除含真值脚本。

### 9.1 图鉴页建议卡真实渲染（修复前 DOM 计数 0 → 修复后 >0）

- 真浏览器进 `#/pages/reactions/reactions`（title=反应图鉴），DOM 探针：`r14Advice=7 / r14Cta=7 / r14Empty=0`；7 条 advice 请求全部 `200 / cand 非零(3/6/8) / empty_reason=null`（修复前同场景 r14Advice/r14Cta/r14Empty 全 0）。
- 空串提示真实显示：卡文案 `换上「温压弹」可提升当前构筑（服务端未给出目标反应）`，证明 expected_reaction 空串不再阻断 ready；`source=reactions` 正确。
- 截图 `r14-rxfix-01-ready.png`（sha 285bf60f…）、`r14-rxfix-06-card-visible.png`（sha bc6352ec…，滚动到卡处亲读见「AI·可执行建议 / 换上温压弹 / 依据 battle_records… / 一键替换为 温压弹 →」）。

### 9.2 点 CTA → 两步确认 → 写入落库 → 反馈可见

- 点 `#r14-advice-action-btn` 弹出 `.r7-compare-mask`，title「当前 vs 推荐 · 确认替换」，行「干冰弹（当前）→温压弹（推荐）」，按钮 [取消, 确认替换]；依据含 `source=reactions`（截图 `r14-rxfix-02-compare.png`, sha a028a565…）。
- 点确认后 `.apply-done`「已替换为「温压弹」，出战配置已更新。」（截图 `r14-rxfix-03-applied.png`, sha 84b423f8…）；DB skill slots 由 `0:1,1:4,2:7` 变为 `0:1,1:3,2:7`，服务端事务落库坐实。

### 9.3 空态路径与防作弊

- 游客态图鉴页：`r14Advice=7 / r14Empty=7 / r14Cta=0`，空态文案「AI 暂时没有足够数据给出建议 / 近 7 天还没有真实战报，先打一场再给一键替换建议 / 去打一场战斗 →」，无一键按钮（截图 `r14-rxfix-04-guest-empty.png`, sha 26935a75…）。
- 非法元素 `element=INVALID_X` → `200 / code=INVALID_ELEMENT / cand=0 / schema=advice-action-v1`（空态 id 始终为 0，无 params）。
- 隐私探针：图鉴页 ready/游客态、战报详情页 pe/ce/af/fr 全 0，`domainInBody=false`（6 张截图二进制 strings 扫描无真域名/JWT/路径）。

### 9.4 战报页 R14 闭环不回退

- user10 战报详情页 DOM：`r14Advice=1 / r14Cta=1 / r14Empty=0 / reaction-learning=1`，探针全 0、`domainInBody=false`，截图 `r14-rxfix-05-battles-nonreg.png`（sha defc58af…）。

### 9.5 结论与证据清单

**R14-BUG-1 修复后真浏览器 GREEN**：图鉴页建议卡从完全不可达变为 7 处真实渲染 + 可执行 CTA + 落库闭环；空串提示、空态、防作弊、战报不回退全部 PASS。6 张 390×844 截图 sha 两两不同且均亲读确认为非登录页、有数据态：

| 截图 | sha256(前12) | 场景 |
| --- | --- | --- |
| r14-rxfix-01-ready.png | 285bf60f851c8 | 图鉴页 R14 卡 ready（DOM 计数证据） |
| r14-rxfix-02-compare.png | a028a5651bd9 | 两步确认浮层（source=reactions） |
| r14-rxfix-03-applied.png | 84b423f8b4de | 写入成功反馈 |
| r14-rxfix-04-guest-empty.png | 26935a75fe41 | 游客诚实空态无按钮 |
| r14-rxfix-05-battles-nonreg.png | defc58af375f | 战报页 R14 闭环不回退 |
| r14-rxfix-06-card-visible.png | bc6352ec1ee5 | 滚动到 R14 卡亲读可见 CTA |

真值复扫（文本+截图二进制）：JWT=0、Bearer=0、口令字面=0、私钥=0、非回环域名=0、worktree id=0、个人目录路径=0。`auth-init.js` 已删，`auth-init.tpl.js` 仅含 `__LYP_TOKEN__` 占位符；采集脚本中的 agent-browser 路径与截图路径已改为 PATH 命令与 `$PWD` 相对引用。新增探针 `rx-probe.js`/`rx-state-probe.js`/`rx-modal-probe.js`/`rx-applied-probe.js` 与脚本 `rx-ready.sh`/`rx-empty-guest.sh`/`rx-battles-nonreg.sh`/`rx-scroll-card.sh`，日志 `data/rx-*.log`。