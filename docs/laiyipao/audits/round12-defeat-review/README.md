# R12 失败复盘与改进建议闭环（t111）

> 取证环境：本地回环 H5（http://127.0.0.1:5174/）+ 本地回环 API（http://127.0.0.1:8080/api/v1），Chrome 真浏览器操作；**非物理真机**。截图均来自该环境，未写入任何真实域名 / IP / 口令 / 令牌。

## 1. 方法与真实流程

1. 本地测试账号进度设为第 40 关，构筑削弱为单技能 + 无装备（均通过本地测试库 / API 操作，真值不入仓）。
2. 浏览器打开 `?r12=7#/pages/battle/battle?level=40`，真实点击 8×，逐波真实点击「全部不要了，直接下一波」。
3. 结算得到失败局：防线失守 / D / 击杀 23 / 漏怪 24 / 反应 0 / 分数 11812 / 第 7 波 / hash `06c586ba2975b621`（截图 `screenshots/r12-03-defeat-result.png`）。
4. 点击「查看本局战报」：详情有波次、峰值热量、元素与 hash，但只有泛化「漏怪偏高」，没有失败根因与下一动作（截图 `screenshots/r12-04-defeat-report.png`）。
5. 修复后重复同一流程，最终失败局为第 6/8 波失守 / 击杀 16 / 漏怪 11 / 反应 0 / 跳过选卡 5 波；结算与战报详情均出现「AI · 失败复盘」并给出可执行动作（截图 `screenshots/r12-05-defeat-review-fixed.png`、`screenshots/r12-06-defeat-report-fixed.png`）。浏览器 `errors --json` 为 `errors=[]`。

## 2. 真实缺陷（RED → GREEN）

### D1 失败结算没有根因与可执行动作
- 证据：`r12-03-defeat-result.png` 中结算卡只有「下一局可换元素搭配对比收益」的泛化文案；`settleAttribution` 仅对胜利输出。
- RED：`laiyipao/miniapp/src/pages/battle/battle.test.ts` 新增「R12 失败结算给出失守波次与可执行改进动作」；`TypeError: defeatReview is not a function` / `expected false to be true`。
- GREEN：`laiyipao/miniapp/src/game/feedback.ts` 新增纯函数 `defeatReview(input)` 与 `DefeatReviewInput`，仅消费真实波次 / 击杀 / 漏怪 / 反应 / card_picks / 热量 / 元素数据；核心字段全缺时明确「不做估算」。`battle.vue` 新增 `settleDefeatReview`（仅失败且非结算失败时展示）与 `.settle-defeat-review`。

### D2 战报详情无法回答「为什么输、下一局改什么」
- 证据：`r12-04-defeat-report.png` 有波次 7 / 峰值热量 92 / 焰×351 / 漏杀比，但没有归因句。
- RED：`laiyipao/miniapp/src/pages/battles/battles.test.ts` 新增「失败战报详情给出失守波次与可执行复盘」；`expected false to be true`（`.defeat-review` 不存在）。
- GREEN：`battles.vue` 新增 `detailDefeatReview`（仅 `result === 'lose'`）与 `.defeat-review`，复用同一纯函数，战报也有「AI · 失败复盘」。

### D3 复盘文案标点重复（二次 test-first）
- 证据：首次 GREEN 真浏览器截图出现 `提高清怪。；依据：...`（`。；`）。
- RED：`feedback.test.ts` 增 `expect(review).not.toContain('。；')`，逐字 `expected ... not to contain '。；'`。
- GREEN：动作句去掉尾句号，由统一 join / 末尾句号产生规范标点。

## 3. 复盘规则（可重放、不编造）

- 字段来源：结算来自 `engine.waveIndex + 1` / `level.wave_count` / `settleResult` / 权威 `card_picks`；战报来自 `wave_reached` / `kills` / `leaked` / `reactions` / `heat_max` / `elements_used`。
- 分支：`leaked > kills` → 「漏怪多于击杀，火力/清怪不足」；`reactions === 0` → 「反应链未触发，元素搭配未形成收益」；`skipped >= 2` → 「多次跳过选卡，构筑补强不足」；否则「防线压力高于本局构筑承受力」。
- 动作：缺反应 → 补第二种元素 + 升主技能/装备；跳卡 → 不要连续跳过选卡；其它 → 升主技能/装备并调整元素搭配。
- `basis` 逐项列出实际使用的字段名，缺字段不显示。胜利局、结算失败局不展示失败复盘。

## 4. 验证

- RED：3 failed / 65 passed，exit 1；标点 RED：1 failed / 4 skipped，exit 1。
- GREEN 定向：`feedback.test.ts` / `battle.test.ts` / `battles.test.ts` 全绿；`npm test` 全量通过；`npm run type-check`（vue-tsc）与 `npm run type-check:test`（tsc）exit 0。
- 真浏览器复测：`r12=10` 硬重载同一失败流程，结算与战报详情均出现新文案，`errors=[]`。
- 未执行任何 git 操作；未改动 admin / 后端 / stage.vue。

## 5. Top3 真实战斗层缺口（按优先级）

1. **失败归因仍偏宏观**：`feedback.ts` 依据字段只到波次、击杀/漏怪、反应数、热量与元素种类；战报已有发射/命中、关键波次时间线等数据，但未进入 `defeatReview`，无法说清「哪一波、哪一类敌人、哪次过热」击穿防线。建议下一轮把波次事件流接入归因。
2. **改进动作缺少一键落地**：当前只给文字建议；R10 已证实服务端推荐 CTA 无可写替换 id，客户端不能伪造装备/技能 id。需要后端提供权威替换项后，才能把「补第二种元素 / 升主技能」变成可直接点击的动作。
3. **回放定位仍弱**：复盘给出「第 6 波失守」，但战报/回放只有静态 hash 与统计，没有跳到失败波次画面的入口；玩家无法把建议与当时操作对应起来。

## 6. 截图清单

- `screenshots/r12-01-high-level-start.png`：高关卡起始态。
- `screenshots/r12-02-level40-weak-start.png`：第 40 关弱构筑开局。
- `screenshots/r12-03-defeat-result.png`：修复前失败结算（无根因）。
- `screenshots/r12-04-defeat-report.png`：修复前战报详情（无复盘）。
- `screenshots/r12-05-defeat-review-fixed.png`：修复后结算复盘（sha256 `6116e1055cccac81c84cebe02b0eed59443ca1c2a603d289d2de93d7a14407c4`）。
- `screenshots/r12-06-defeat-report-fixed.png`：修复后战报复盘（sha256 `c3141cd97908b59185c9fd37cdd1b95e851e1a77679f2e421e06d3b4ce6ccc68`）。
