# R106 次级可点击控件触摸目标（min-height:88rpx）真浏览器 QA 报告

- 任务：t299 / attempt `e56c0071-d203-411d-9ff5-b17b6d7b167f`
- 轮次：R106 触摸目标修复（上游 t297 规范 RED 5fail|6pass → t298 实施 GREEN 11/11，diff 4+/2-）
- 唯一入库写域：`docs/laiyipao/audits/round106-touch-target/REPORT.md`（`data/`、`screenshots/` 为 UNTRACKED 证据）
- HEAD 基线：`7911e9a016f370f5e5d5609788b3089a8dc95937`（全程未变）
- 结论：**PASS（含诚实边界）**

## 1 范围与方法

验收四类次级可点击控件在三个视口的触摸目标修复：`.mg-row`、`.elem-chip`、`.ann-all`、`.dim-hint.link`，CSS 由 `min-height:auto` / 无声明统一改为 `min-height:88rpx`。

- 视口：390×844 / 768×1024 / 1280×800（真实 Chromium，agent-browser 0.41.1）。
- 改点（`laiyipao/miniapp/src/pages/index/index.vue`，共 8 insertions / 2 deletions）：
  - `.mg-row`：828 追加 `min-height:88rpx`；
  - `.dim-hint`：934-938 `min-height:auto` → `88rpx`（替换，非追加）；
  - `.elem-chip`：1149-1153 追加 `min-height:88rpx`；
  - `.ann-all`：1221-1225 追加 `min-height:88rpx`。
- 契约：`laiyipao/miniapp/src/styles/r106TouchTarget.contract.test.ts`（142 行 / 11 it：5 缺口 + 6 不变量，`fs.readFileSync` 真实读取 index.vue/common.css/tokens.css，按唯一 class 锚点定位 CSS 块）。
- 方法：自启 H5 dev server（5174）＋ init-script 合成响应（后端 8080 未起）→ 每视口 open 后重设 viewport → `getBoundingClientRect` + `getComputedStyle` 探针 → AX snapshot → 4 元素 focus → 10 次真 Tab → 截图。长 PROBE 行在 walk-out 被截断，另用精简 `probe2.js`/`walk2.sh` 补测完整 rect（§2 数据取自补测）。

## 2 三视口触摸目标 rect 实测（CSS px）

下表为 `getBoundingClientRect()` 实测（宽×高，CSS px）与 `getComputedStyle().minHeight`（换算后的 CSS px 值）。依据 `AI_VISUAL_SYSTEM.md` §91/137/200/211/230 触控目标 ≥88rpx；WCAG 2.5.5 硬判据 ≥44×44 CSS px。

| 视口 | 元素 | 数量 | rect 宽 | rect 高 | computed min-height | WCAG 44px 高 |
|---|---|---|---|---|---|---|
| 390 | `.ann-all` | 1 | 73.44 | 49.91 | 45.76px | 达标 |
| 390 | `.dim-hint.link` | 3 | 83.47 | 45.75 | 45.76px | 达标 |
| 390 | `.mg-row` | 3 | 333.97 | 57.13 | 45.76px | 达标 |
| 390 | `.elem-chip` | 5 | 27.03 | 45.75 | 45.76px | 达标 |
| 768 | `.ann-all` | 1 | 144.58 | 98.30 | 90.112px | 达标 |
| 768 | `.dim-hint.link` | 3 | 164.31 | 90.11 | 90.112px | 达标 |
| 768 | `.mg-row` | 3 | 451.53 | 111.58 | 90.112px | 达标 |
| 768 | `.elem-chip` | 5 | 53.23 | 90.11 | 90.112px | 达标 |
| 1280 | `.ann-all` | 1 | 70.63 | 48 | 44px | 达标 |
| 1280 | `.dim-hint.link` | 3 | 80.27 | 44 | 44px | 临界达标（恰=44） |
| 1280 | `.mg-row` | 3 | 506 | 55 | 44px | 达标 |
| 1280 | `.elem-chip` | 5 | 26 | 44 | 44px | 临界达标（恰=44） |

三视口 `iw=cw=sw`（390/768/1280），`hOverflow=false`，非登录页。`88rpx` 随 rpx 基准换算：390 约 45.76px、768 约 90.11px、1280 恰 44px（旧估 1280 约 25px 系按固定基准推算，**以实测为准**，实测为 44px）。

**诚实结论**：四类控件 rect 高度全部 ≥44px（1280 的 `.dim-hint.link`/`.elem-chip` 恰 44px 为临界达标）；但宽度上 `.elem-chip` 普遍 26–53px < 44px（窄 chip 宽度不达 WCAG 44×44 的完整方形判据），属本轮修复范围（仅 min-height）之外的残留观察，如实标注，**不冒充全绿**。

## 3 键盘不回退 / 焦点环 / 静态回归

- **键盘三件套**：四类控件模板保留 `role="button"` + `tabindex="0"` + `@keydown.enter` 同名 handler（`.ann-all`:109、`.dim-hint`:189-193、`.mg-row`:209-217、`.elem-chip`:305-313）。
- **真 Tab**：每视口 10 次 Tab 依次命中 elem-chip 系列 → btn ai-cta → ann-all → ann-item×3 → …，全 `role=button/tabIndex=0`；AX snapshot 坐实 `button "全部 3 条 →"`、`button "→ 去背包配元素"`、`button "1 反应链（还差 7）收益 182"`、`button "焰/冰/电/毒/动能"`，ann-item 带 `[expanded=false]`（R101 不变量保留）。
- **焦点环**：12 条（4 元素×3 视口）`isActive=true`，outline `solid rgb(255,107,53)`；390/1280 为 1px offset1px、768 为 3px offset2px（rpx 缩放差异，可见性均已坐实）。改前 recon 已确认焦点环本就覆盖（命题不成立），本轮仅守不回退。
- **令牌三锚点**（tokens.css）：`:14 --lyp-token-version: lyp-tokens-v1`、`:15 --lyp-r3-version: lyp-tokens-v3-r3`、`:16 --lyp-r6-version: lyp-tokens-r6` 全在。
- **业务零 `--r106-`**：业务 CSS NO_MATCH（唯一 3 处命中在契约自身断言 r106TouchTarget.contract.test.ts:10/121/122）。
- **common.css 两层 `:focus-visible`**（:914-921）`outline:3rpx solid var(--accent)` / `offset:2rpx` 未回退，`:914 [role=button]{cursor:pointer}` 在。
- **dim-hint 无 auto**：`.dim-hint` 的 `min-height:auto` 已被 `88rpx` 替换，computed 实测见 §2（三视口均 44+px，无 auto）。

## 4 门禁（G1–G7 全绿）

证据：`docs/laiyipao/audits/round106-touch-target/data/gates-out.txt`（job `bash-72171922ca9c42f2` exit 0，末尾 `GATES_DONE`）。

| 门禁 | 命令/对象 | 结果 | RC |
|---|---|---|---|
| G1 | r106TouchTarget.contract.test.ts | 11 passed | 0 |
| G2 | r100(12)+r102(9)+r105(10) 三契约 | 31 passed | 0 |
| G3 | index.test.ts | 27 passed | 0 |
| G4 | miniapp 全量 vitest | 238 passed\|1 skipped (239) files / 2196 passed\|2 skipped (2198) tests | 0 |
| G5 | vue-tsc (type-check) | 通过 | 0 |
| G6 | build:h5 (uni build) | 通过 | 0 |
| G7 | git diff --check | 通过（LF→CRLF warning 非错） | 0 |

G3/G4 中 `[来一炮] build.attacker 缺失，正在使用本地占位属性` 为测试既有 stderr 告警，非本轮引入，不影响 RC0。

## 5 截图 sha256sum（落盘实物实测）与亲读

| 截图 | 尺寸 | 字节 | sha256（实测） |
|---|---|---|---|
| r106-touch-390.png | 390×844 | 72229 | `a6cc3618f54b8d64ef9acdc4195f3450515e993493b7264bdcd811e1129eeeef` |
| r106-touch-768.png | 768×1024 | 161809 | `3c7301cf1ef91e28b32f12b133bafd80c4c3345829562510ff5abf477a823793` |
| r106-touch-1280.png | 1280×800 | 57212 | `ddea912d7a694e0f75cea1757a2c61aca1c222473954c198b671cd7eb0413d9f` |

UNIQUE 3/3（sha256sum 对落盘实物逐个实测）。**亲读 2 张非登录页**：

- `r106-touch-390.png`（390×844）：QA 玩家顶栏、AI 战术洞察「去补强」、公告「全部 3 条」、构筑评分 S·98765 五维雷达图，四类控件可见且与探针一致。
- `r106-touch-768.png`（768×1024）：顶栏 QA 玩家、攻方属性面板（暴击倍率 150% / 反应倍率 100% / 元素系数 100% 等）、第 10 关「开始」卡片、下一目标（第 56 关 / 工业废墟），视觉与探针一致。

## 6 改前基线对照

引用 `docs/laiyipao/recon/ROUND106_B_TOUCH_TARGET_RECON.md`（t296，95 行）：改前真 Chromium 实测 390 ann-all 81×19 / dim-hint 72×15 / mg-row 334×34 / elem-chip 27×16；1280 ann-all 78×19 / dim-hint 69×15 / mg-row 506×33 / elem-chip 26×16。四类控件高度普遍 15–34px，远不达 88rpx / 44px。本轮实测高度 390 45.75–57.13px、768 90.11–111.58px、1280 44–55px，**高度维度修复成立**；宽度的 elem-chip 窄 chip 问题改前改后均存在（非本轮命题）。

## 7 诚实边界

- 后端 8080 未起，全部响应由 init-script 合成（配置、公告、钱包、档案等），占位令牌 `r106-qa-placeholder-token` 仅 localStorage 内存注入，**零真值**。
- `build_rating` 各维度在 mock 中置 0（element_coverage/reaction_coverage/mastery_done=0/0/0）使 `.dim-hint.link` 因 `d.pct===0&&d.hint` 自然渲染 3 条；首版满分配置下 dimHint.n=0，已修正 mock 后重跑。
- 1280 的 `.dim-hint.link`/`.elem-chip` 恰 44px 为临界达标；`.elem-chip` 宽度 26–53px < 44px 的方形判据不达（残留观察，不冒充全绿）。
- 焦点环 12 条由 JS `focus()` 触发（outline 1px/3px 随 rpx），与 recon 真 Tab 聚焦的 3px 口径略有差异；焦点环可见性坐实，改前已覆盖。
- walk-out 长 PROBE 行被截断，§2 rect 数据取自精简补测 `probe2-out.txt`（`PROBE2_DONE`），非由截断日志推断。
- 截图亲读 2/3；第 3 张（1280）以 sha + 探针为据，未逐像素亲读。

## 8 结论与 hygiene

- HEAD `7911e9a016f370f5e5d5609788b3089a8dc95937` 全程未变；git status 非 ?? 项仅 `M .gitignore` + `M laiyipao/miniapp/src/pages/index/index.vue`（均为 t298 预期实施）。
- auth 五路径全 ABSENT（`.env`、`laiyipao/admin/.env.local.keys`、`laiyipao/deploy/.local.keys`、`laiyipao/server/.local.keys`、`laiyipao/miniapp/.env.local`）；QA 库本轮未连；`.gitignore:674` 拦 data/。
- 自启 H5 dev server（job `bash-013061135df34ca9`）本轮末清理并复验端口关闭。
- 综合：三视口四类控件触摸目标修复（高度维度）坐实、键盘/焦点/令牌不变量保持、门禁 G1–G7 全 RC0、截图与亲读齐备。R106 QA 判定 **PASS（含 §7 诚实边界）**。
