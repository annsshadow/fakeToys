# R105 index 顶栏 768 横溢修复契约 RED 证据

- 任务：t290 / attempt_id=bb80c86b-73ca-4901-bb34-9893ab704d8e
- 契约：laiyipao/miniapp/src/styles/r105TopbarOverflow.contract.test.ts（10 it = 3 缺口 + 7 不变量）
- 基线 HEAD：0151ce97e1216d70efd899e85d72e7ad2f93b750（未改业务源码）
- 命令：`cd laiyipao/miniapp && npx vitest run src/styles/r105TopbarOverflow.contract.test.ts --reporter=verbose`
- 原始输出：`RED-raw.log`（同目录）。vitest v4.1.12，Duration 319ms。

## 一、总体结果

- exit code = **1**（RED 预期）。
- `Test Files 1 failed (1)`；`Tests 3 failed | 7 passed (10)`。
- 3 个失败全部为真实缺口（业务 CSS 未改），无 PARSE_ERROR、无模块加载错误、无定位辅助函数自身异常。
- 7 个不变量全部通过，证明契约的定位器（cssBlock / mediaBlock / tagBefore / compact）在基线源码上正确工作，失败可归因于缺口而非测试自身缺陷。

## 二、逐 it 期望与实际

| # | it | 类型 | 期望 | 实际 |
|---|---|---|---|---|
| 1 | .player-info 块含 min-width:0 | 缺口 | RED：失败（基线无 min-width:0） | ✗ 失败：`.player-info 须声明 min-width:0: expected '{display:flex;flex-direction:column;}' to contain 'min-width:0'`（真实缺口） |
| 2 | ≤900px 媒体块内 .res-row flex-wrap:wrap | 缺口 | RED：失败（基线无 max-width:900px 媒体块） | ✗ 失败：`须唯一锚点定位 @media (max-width: 900px) 块: expected null not to be null`（真实缺口） |
| 3 | .topbar 含 max-width:100% + box-sizing:border-box | 缺口 | RED：失败（基线无两声明） | ✗ 失败：`.topbar 须声明 max-width:100%: expected '{position:sticky;top:0;left:0;right:0…' to contain 'max-width:100%'`（真实缺口） |
| 4 | .topbar 保留 display:flex + justify-content:space-between | 不变量 | 通过 | ✓ 通过 |
| 5 | .res-row 基础块保留 white-space:nowrap | 不变量 | 通过 | ✓ 通过 |
| 6 | R99/R102 键盘三件套保留（retry / ann-all / go(d.hint)） | 不变量 | 通过 | ✓ 通过 |
| 7 | R100/R101 .ann-item 三件套与 aria-expanded | 不变量 | 通过 | ✓ 通过 |
| 8 | 四货币胶囊插值 coin/gem/energy/keys | 不变量 | 通过 | ✓ 通过 |
| 9 | 令牌三锚点不动 | 不变量 | 通过 | ✓ 通过 |
| 10 | 业务源码零 --r105- 视觉令牌 | 不变量 | 通过 | ✓ 通过 |

## 三、自纠说明（诚实披露）

- 首轮契约曾为 9 it（令牌与零标记合为一条不变量），RED 得 3 failed | 6 passed (9)；与 SPEC §5 声明的 10 it / 3 failed | 7 passed 不一致。已将令牌三锚点与零 --r105- 拆为两条独立不变量 it，复跑得 3 failed | 7 passed (10)，与 SPEC 一致。首轮原始输出已被最终 RED-raw.log 覆盖；此处如实记录自纠过程。
- 首轮所有失败项与最终一致（同为三个缺口真实失败），拆分仅影响不变量计数，不改变缺口判定。

## 四、缺口真实性确认

- 三个缺口 it 的失败断言分别直接读取基线源码：`{display:flex;flex-direction:column;}`（.player-info 实值）、`@media (max-width:900px)` 定位返回 null（该媒体块不存在）、`.topbar` 实值无 max-width/box-sizing。全部为业务现状，非辅助函数误报。
- 实施轮按 SPEC §3 三处改点修改后，预期 10 it 全绿（GREEN）；QA 另须真浏览器三视口复验零横溢。

## 五、hygiene

- 本轮仅写：`docs/laiyipao/specs/round105-topbar-overflow/{SPEC.md,RED-evidence.md,RED-raw.log}` 与 `laiyipao/miniapp/src/styles/r105TopbarOverflow.contract.test.ts`，均 UNTRACKED。
- 未改业务 vue / 令牌 / 其它契约 / .gitignore；HEAD=0151ce97e1216d70efd899e85d72e7ad2f93b750 未变；git diff --check RC=0；零真值。
