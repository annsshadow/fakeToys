# R8 缺口3：战斗时间预算与钳分实时预警（t82）

- 任务：把「时间预算 / 钳分」从结算后的事后解释，提前到战斗 HUD；数值必须来自服务端同款规则与战斗状态，不能写死 60s/75s。
- 环境：H5 `http://127.0.0.1:5174/`（本地回环）、API `http://127.0.0.1:8080`（本地回环），真浏览器 Chromium 视口 1258×588。
- **截图均为「非物理真机 / 本地回环真浏览器」**，不是手机真机。

## 1. 实测证据与真实发现

硬重载第 1 关并真实点击「开始战斗」后，HUD 显示 `钳分中 · 距满分 133s`。
按 8× 逐波清除（真实点击 4 次「全部不要了，直接下一波」），HUD 随战斗时钟从 133s 递减到 60s / 54s；结算显示：

`83s 内得分上限 19732，实得 12236；距 2 星门槛 16916 还差 4680；漏怪 0；建议缩短清波时间或少漏怪、多触发反应提分。`

击杀 39 / 漏怪 0 / 反应 88 / 分数 12236 / B 级 / 回放哈希 7306ed0bde9702ad；浏览器 `errors --json` 返回 `errors=[]`。

**关键发现：原任务语境中的「保分窗口」与实际规则相反。** 服务端 `scoreCapFor` 是负向闸：`sec < anchor` 时按比例裁剪，`sec >= anchor` 才拿满分。也就是说，锚点前一直处于钳分区，倒计时是「距离免钳分还有多久」，而不是「可以放心打多久」。如果 HUD 写「保分窗口 133s」，会让玩家误以为拖到 133s 会被扣分，属于谎报规则。已按真实语义改为「钳分中 · 距满分 Xs（临近锚点时提示即将免钳分）」、锚点后显示「已免钳分」。

服务端规则来源逐行核对：`laiyipao/server/internal/domain/battle.go` 的 `SpeedAnchorPermille = 9000`、`theoreticalClearSec`（每波 `delay+(count-1)*interval` 取最大后求和并取整秒）、`speedAnchorSec = lower * 9000 / 1000`。

## 2. RED → GREEN

1. 第一轮 RED：4 failed / 57 passed。缺失 `SPEED_ANCHOR_PERMILLE` 与 `scoreTimeBudget`，HUD 无时间胶囊。
2. GREEN 实现：`laiyipao/miniapp/src/game/score.ts` 新增跨端系数与纯函数 `scoreTimeBudget(level, elapsedMs)`；`battle.vue` 新增 `timeBudget` computed 与 HUD 胶囊。实测时发现系数被误写成 ×0.9（得到 450s），已按服务端源码修正为 ×9。
3. 第二轮 RED：2 failed。钉住诚实语义：锚点前不得出现「保分窗口」，临近锚点提示「即将免钳分」，锚点后「已免钳分」。
4. 第二轮 GREEN：定向 `score.test.ts + battle.test.ts` 2 files / 61 tests passed，exit 0。

测试钉住的内容：
- 跨端解析 Go 源码，确保 `SpeedAnchorPermille = 9000` 与 TS 常量一致（防止双端漂移）。
- 不同波次配置得到不同锚点（5 波 → 45s，3 波 → 27s），没有波次配置返回 null，不编造 60s。
- 39s/40s/46s 三个时点分别验证安全、临近提示、已免钳分；HUD 随 `elapsedMs` 更新并带 `warn` / `clamped` class。

## 3. 真浏览器复测截图

- `screenshots/r8tb-01-clamp-active.png`：开局 HUD `钳分中 · 距满分 133s`，与星级差额胶囊并排，不遮挡战场。
- `screenshots/r8tb-02-clamp-near-end.png`：第 5 波 HUD `钳分中 · 距满分 54s`，8× chip 高亮，说明倒计时随战斗状态更新。
- `screenshots/r8tb-03-result.png`：结算钳分串与 HUD 语义一致（83s 仍小于 133s 锚点，所以仍被裁）。

> 诚实边界：第 1 关实战约 83s 就结束，达不到 133s 的免钳分锚点，真浏览器只能拍到「钳分中」两个时点；「已免钳分」状态由定向单测覆盖，未伪造浏览器画面。

## 4. 验证与协作

- `npx vitest run --config vitest.config.ts src/game/score.test.ts src/pages/battle/battle.test.ts`：2 files / 61 tests passed，exit 0。
- `npm run type-check`（vue-tsc）与 `npm run type-check:test`（tsc）：exit 0。
- 中途复跑曾出现 2 条与 t82 无关的 stage 页失败（`r5H5Pages.contract.test.ts` / `stage.test.ts`），已通知 design-lead 并由其修复；t82 未修改 stage.vue。
- 最终 `npm test`：exit 0，Test Files 176 passed | 1 skipped (177)，Tests 1683 passed | 2 skipped (1685)。
- 未执行任何 git 操作；无真实域名/IP/口令/私钥入库或日志。
