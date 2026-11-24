# R27 规范：实时波次推进 HUD（round27-wave-progress-hud）

> 目标：战斗中一眼看到「本波还剩多少敌人」。全部为只读派生展示，不改 spawn/step/record/replay_hash/score/settle/loot。

## 0 缺口与依据（亲读源码带行号）

- `laiyipao/miniapp/src/pages/battle/battle.vue:13-40` 顶部 `overlay-top`/`hud-row` 只有防线(:15-25)、波次(:27)、击杀(:28)、星级差额(:29)、钳分(:33-39)，没有本波总数/已处理/场上/待出场。
- `laiyipao/miniapp/src/game/engine.ts:583` `waveIndex`（0 基当前波）；:589 `enemies: Enemy[]`；:630 `private spawnQueue: Array<{enemyId,atTick,uid}>`；:652 `readonly totalEnemies`（全关总数，构造时 :710-714 累加所有波 `spawns.count`，**不是本波总数**）。
- `engine.ts:949-974` `beginWave` 清空并按当前波 spawns 展开 spawnQueue（每组 count 个），:976-981 确定性洗牌；:1171-1177 到点 spawn 后 `compactInPlace` 出队；:1259-1263 dead 敌人移出 enemies。
- `engine.ts:2180-2205` `checkWaveEnd` 只依赖 `spawnQueue.length` 与 `enemies.length`；清波后 `card_select` 时 waveIndex 仍指向刚清完的波，直到 `beginWave(next)` 才前进（:933）。
- 结论：本波总数须按 `cfg.level.waves[waveIndex].spawns[].count` 求和；场上=`enemies` 中 `!dead`；待出场=`spawnQueue.length`；已处理=总数−场上−待出场。现有引擎无只读快照可拿 `spawnQueue`，需新增只读方法（§1）。

## 1 引擎只读快照 waveProgressHud()

- 在 `engine.ts` 新增只读方法 `waveProgressHud()`，返回四个原始计数：`{ waveIndex: number; waveTotal: number; onField: number; pending: number }`。
  - `waveTotal = (cfg.level.waves?.[waveIndex]?.spawns ?? []).reduce((s, sp) => s + (Number(sp?.count) > 0 ? Number(sp.count) : 0), 0)`
  - `onField = this.enemies.filter(e => !e.dead).length`
  - `pending = this.spawnQueue.length`
- 照抄 R15/R24 只读范例（`engine.ts:771-779` `reactionTimeline().slice()`、`:791-804` `elementLayerHud()`）：只读、返回新对象/新数组，不暴露内部引用。
- 硬禁令：方法体内不得调用 `step()` / `beginWave()` / `record()` / `emit()` / `finish()`，不得赋值 `score` / `kills` / `leaked` / replay_hash 相关字段；replay_hash 的 L/A/S/X 段（:868-878）保持不变。
- H5 渲染在 step 后（raf 与 fallback 两条路径）调用该只读快照，再交给 §2 纯函数；不得在模板里直接改引擎。

## 2 H5 纯函数 buildWaveProgress（真实模块）

- 模块路径：`laiyipao/miniapp/src/pages/battle/waveProgressHud.ts`，具名导出 `buildWaveProgress(raw)`。
- 入参 raw = §1 四字段；返回 `{ waveIndex, waveTotal, onField, pending, resolved, clearedPct, empty, ai }`。
- 非法数据（raw 为空/非对象；`waveIndex` 非非负整数；任一计数字段非有限数）→ `empty=true`、所有计数归 0、`clearedPct=0`、`ai=""`；不抛异常、不编造。
- clamp 口径：`total = max(0, floor(waveTotal))`；`onField = clamp(0, total)`；`pending = clamp(0, total)`；`resolved = clamp(total - onField - pending, 0, total)`；`clearedPct = total>0 ? round(resolved/total*1000)/10 : 0`，再 clamp 到 0..100。负数/超额按 clamp 收敛，不因此判空；全程不得产出 NaN/Infinity/负百分比。
- 空态另一分支：`total===0`（本波无敌人/索引越界）→ `empty=true`、`ai=""`。**空态逐字文案「本波暂无敌人进度数据」**由 battle.vue 的 `.r27-progress-empty` 渲染。
- 已处理（resolved）= 击败 + 漏怪的合计（二者都不再构成威胁，与 `engine.ts:2188-2202`「kills + leaked >= 总怪数」清空即胜口径一致）。函数**不得**拆出 kills/leaked 字段，也不得据 resolved 反推击杀数。
- Boss 语义：Boss 只是敌种标记，按 1 只计入 waveTotal/onField/resolved，不加权、不产出额外威胁字段；威胁判断归 R24/R26，不在本快照重复编造。
- `ai` 由真实快照确定性派生：`本波共 X 只，已处理 Y 只，场上 Z 只、待出场 W 只，清理进度 P%`（X/Y/Z/W/P 全部来自 clamp 后真实值，P 保留一位小数）。

## 3 选择器与 390×844 布局

- `.r27-wave-progress`：紧凑进度容器，挂在顶部 `.hud-row`（battle.vue:14）内、既有钳分胶囊（:33-39）之后；容器带 `[data-r27-progress]` 属性（值为 waveIndex）。
- `.r27-progress-track` + `.r27-progress-fill`：细轨（高度不超过 6rpx），fill 宽度 = `clearedPct + "%"`。
- `.r27-progress-text`：逐字格式 `本波 {{resolved}}/{{waveTotal}} · {{clearedPct}}%`。
- `.r27-progress-empty`：空态逐字「本波暂无敌人进度数据」。
- `.r27-ai-note` 及其 `.r27-ai-conclusion` / `.r27-ai-basis` / `.r27-ai-cta` / `.r27-ai-honesty`。
- 布局硬约束：`.r27-wave-progress` 容器**自身**单行紧凑（`flex-shrink: 0`、内部不换行，用 `max-height` + `overflow: hidden` 截断），不抢占已有胶囊空间；`.hud-row` **必须保留既有 `flex-wrap: wrap`**（battle.vue:2008-2016，R9 390×844 窄屏换行修复），绝不改成 `nowrap`/单行 flex，也不得把防线(:15-25)、波次(:27)、击杀(:28)、星级(:29)、钳分(:33-39)、R24(:52) 与 R26(:239) 已有 HUD 挤出 390×844 视口；窄屏时 `.r27-wave-progress` 与既有胶囊一起自然换行，但胶囊本身（`flex-shrink: 0`，:2023-2027）不压缩成竖排；`.overlay-top` 的 `pointer-events: none` 语义保留（:1618/:1996），进度胶囊不拦截画布点击。

## 4 AI 四判据

- ① 可见汉字「AI」（`.r27-ai-note` 内标题）。
- ② 真实数据结论：§2 ai 串，数字全部来自 clamp 后快照。
- ③ 依据行逐字：「依据：引擎 waveProgressHud() 只读快照（当前波 spawns 总数 − 场上存活 − 待出场）；只读派生不进 replay_hash」。
- ④ 可执行 CTA 逐字「继续清理场上敌人 →」，绑定既有 openHelp。

## 5 诚实红线（逐字）

- 「辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准」。
- 只读派生：不改 spawn/step/record/replay_hash/score/settle/loot；waveIndex 在 card_select 期间不前进，清波后显示 100% 而非猜测下一波。
- 非法数据/本波无敌人走空态，绝不编造敌人数或进度；漏怪算「已处理」但**不**等同于胜利，胜负仍由引擎与服务端权威判定。

## 6 契约测试设计（test-first，真实模块导入 + RED 门禁）

- 文件：`laiyipao/miniapp/src/pages/battle/battle_r27_wave_progress.contract.test.ts`。
- **禁止内嵌参考实现**：必须从 `./waveProgressHud` 具名导入 `buildWaveProgress`；测试文件内不得出现同名本地函数或算法复刻（t204 教训：内嵌实现让功能缺失仍全绿）。
- 算法断言（全部调用真实导入函数）：resolved 差值、一位小数四舍五入、onField/pending/resolved 三处 clamp、非法数据空态、Boss 不加权、漏怪不拆字段、ai 含真实数字。
- **真实源码守卫**：engine.ts 含 `waveProgressHud()` 只读方法且方法体不写权威字段；battle.vue 含 R27 选择器/空态文案/红线文案/CSS max-height、import 并调用 `buildWaveProgress`、保留 overlay-top/hud-pill 既有锚点；tokens.css 三锚点保留且无 `--r27-`。
- **RED 门禁**：`waveProgressHud.ts` 尚不存在时测试因模块解析失败整体加载失败（RED），须保留逐字失败输出（Cannot find module ./waveProgressHud / ERR_MODULE_NOT_FOUND）作为 test-first 证据；t206 只交付 RED，不创建业务实现。
- 零真值：SPEC 与测试不含域名/IP/JWT/口令/个人绝对路径。

## 7 三端不回退与分工

- 令牌锚点 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6` 保留，禁用 `--r27-` 新令牌。
- R15~R26 既有契约不改；H5 顶部 HUD、R24 元素层、R26 下一波简报均不回退；零后端改动。
- t206 交付本 SPEC 与契约测试（RED）；引擎只读方法/H5 渲染由后续实施任务完成。
