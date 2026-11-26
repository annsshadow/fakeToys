# R27 设计终审报告：实时波次推进 HUD 六门禁独立复核

- 任务：t210（design-reviewer-wb，attempt_id=57501df7-21e4-46cf-9ed6-5fa2d015e7bd）
- 复核对象：`waveProgressHud()` 只读快照 + `buildWaveProgress()` 纯函数派生 + H5 `.r27-wave-progress` 渲染
- **verdict = pass**（六门禁全部满足）
- 基线 HEAD：`4f1a704593d3a116fd64c6821c2529f351bfe3e8`
- 口径：设计终审独立复核，亲读 SPEC/源码/报告/5 图并独立复跑测试，**不采信实施与 QA 自述**；agent-browser 桌面浏览器自动化（390×844，非物理真机）、本地回环，限制如实标注。
- 未 git，未改业务源码。

---

## 门禁① 规范与契约一致（pass）

- `docs/laiyipao/specs/round27-wave-progress-hud/SPEC.md`（69 行）亲读：§1 只读快照四字段 `{waveIndex, waveTotal, onField, pending}` + 硬禁令（不得调 step/beginWave/record/emit/finish、不得赋值 score/kills/leaked/replay_hash，L/A/S/X 段不变）；§2 clamp 口径、一位小数、空态、漏怪/Boss 语义、`ai` 串格式；§3 七类选择器；§4 AI 四判据；§5 诚实红线；§6 RED 门禁「禁止内嵌参考实现」+ ERR_MODULE_NOT_FOUND；§7 R15~R26 不回退。
- `battle_r27_wave_progress.contract.test.ts`（190 行）亲读：第 4 行从 `./waveProgressHud` **具名导入真实函数**，测试体内无同名本地算法复刻；9 个算法 case（resolved 差值、一位小数 66.7%、三处 clamp、非法数据空态、total=0 空态、漏怪不拆字段、Boss 按 1 只、AI 数字、0.0%/100.0% 边界）+ 真实源码守卫（engine 方法体禁写权威字段、battle.vue 选择器/逐字文案/CSS/导入、SPEC 逐字锚点、令牌三锚点无 `--r27-`、零真值）。
- 独立复跑契约：`pnpm exec vitest run battle_r27_wave_progress.contract.test.ts --config vitest.config.ts` → **1 file / 23 tests passed**（exit 0，19:56）。RED 基线由 t206 的 `ERR_MODULE_NOT_FOUND` 输出佐证（规范 §6 钉死）。

## 门禁② 引擎只读快照不写权威（pass）

- `engine.ts:811-821` 亲读：`waveProgressHud()` 只读 `cfg.level.waves[waveIndex].spawns.count` 求和、`enemies.filter(!dead).length`、`spawnQueue.length`，返回**新对象**；方法体内无 step/record/beginWave/emit/finish 调用，无 score/kills/leaked/replay_hash 赋值。契约 `:104-109` 源码守卫正则扫描该方法体，逐一排除上述写操作。
- 权威路径不回退：`battle.vue:1280/1290/1298` 仍由 `eng.settleInput(tokenId.value)` → `api.settleBattle` 结算，R27 快照不参与。
- 动态实测：真实点击通关 `status=200`、`battle_id=478`、`回放哈希 7ddbb762bc017a11`，`hashResponses=[]`（HUD 未额外触发 hash 端点）——权威 hash 未被污染。

## 门禁③ 真实配置口径与 Boss/漏怪诚实性（pass）

- `waveProgressHud.ts`（54 行）亲读：`buildWaveProgress(raw:unknown)` 对 null/非对象/waveIndex 非非负整数/计数字段非有限数一律 `empty()`；`total=max(0,floor(waveTotal))`，onField/pending/resolved 三处 clamp 到 0..total，`pct=total>0?round(resolved/total*1000)/10:0` 再 clamp 0..100；`total===0→empty`。resolved=击败+漏怪合计，**不拆 kills/leaked、不反推击杀**；Boss 按 1 只不加权；ai 串全部取 clamp 后真实值、`toFixed(1)`。
- 权威配置逐字段：`GET /api/v1/levels/5` spawns 聚合 L5 五波 total = **3/5/6/8/14**，与 96 帧 tick-log 逐帧 total 完全一致；config 不下发 waves，HUD 只读 `eng.cfg.level.waves`，未自造数据（报告 §1）。
- 47 帧 DOM 双层解析：fill 宽度与 domPct 偏差 `>0.15` 命中 **0 帧**；hasUndefined/hasNaN 全 false；清波 100.0%、下一波换 total 归零 0.0% 逐字段一致。

## 门禁④ AI 四判据与移动布局（pass）

- `battle.vue:41-64` 亲读七类选择器：`.r27-wave-progress[data-r27-progress]`、`.r27-progress-track`、`.r27-progress-fill`、`.r27-progress-text`、`.r27-progress-empty`、`.r27-ai-note`（含 title/conclusion/basis/cta/honesty）。实际标题为 **「AI · 波次推进」**（可见汉字 AI）。
- 四判据齐备：①可见 AI 标签 ②真实数据结论（ai 串逐帧随 DOM 变化）③依据行逐字「依据：引擎 waveProgressHud() 只读快照（当前波 spawns 总数 − 场上存活 − 待出场）；只读派生不进 replay_hash」④CTA「继续清理场上敌人 →」。诚实红线逐字「辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准」；空态逐字「本波暂无敌人进度数据」。全部与 SPEC 逐字一致。
- 布局：`battle.vue:2643-2667` R27 容器 `inline-flex/nowrap/flex-shrink:0/max-height:52rpx/overflow:hidden/pointer-events:none`，自身单行紧凑；`battle.vue:2040-2043` `.hud-row` 保留 `flex-wrap: wrap`（契约 `:123-130` 独占守卫，不回退 R9）。5 图 390×844 实测 `docOverflow=false`、`hudWrap=wrap`，R24 元素层与 R26 下一波简报同屏可见不挤出。

## 门禁⑤ 真浏览器有效证据与权威 hash 不变（pass）

- 5 图 sha256 本人亲算，与 QA 报告 §7 逐字一致且**两两不同**，全部 390×844 亲读为战斗/结算态非登录页：
  - r27-01-first-wave.png `1747bcd7584476e3eb50d581e639123fb04a217e7eebe8006cb8b8efe213297e`（本波 1/3 · 33.3%）
  - r27-02-mid-wave.png `0816cb98df703f09834950cded2b5b0ba1bb784e83b78efef0492479a5d45d23`（2/5 · 40.0%）
  - r27-03-clear-100.png `73ca5f4c22c0bb2449df10559b6b8f81f9c54257b10c413dc5febd4c04e50746`（3/3 · 100.0% + 选卡）
  - r27-04-next-wave.png `2ab016400876e1d009fc26014daa42c9b48ad2d146e6a60594e77f40e2f68611`（0/5 · 0.0% 换波归零）
  - r27-05-settle.png `06ea3c7589e133f816e80654f639fd7128128a556899d2b8a6e3aecf29d8979c`（通关结算 hash 7ddbb762bc017a11）
- 探针：原始 pageErrors 64 条逐条为 `play() request was interrupted by pause()` 的 Chrome Autoplay 噪声（与 R24/R25/R26 同口径），过滤后 `peReal=0`；consoleErrors=0/apiFailures=0/failedRequests=0；`settleResponses=[{status:200,battle_id:478}]`。
- QA 数据还原：`pg_restore --clean` 从快照恢复，battle_records=104、id=478 计数=0、max(id)=477，E2E 新增战报已清除（报告 §8，终审未重跑 DB 写操作、仅静态核对）。

## 门禁⑥ 真值零泄漏及 R1-R26 不回退（pass）

- `docs/laiyipao/audits/round27-wave-progress-hud/` OPS 正则扫描（JWT / Bearer / 带口令连接串 / 个人路径 / 公网 IP）：命中 2 处均为 **127.0.0.1 本地回环**（`e2e-full.sh:13` 打开本地 H5、`mint-token.sh:4` 本地 guest 接口），属允许回环 + 占位口令，非真值泄露。
- `auth-init.js`（含真 JWT 注入脚本）已删除：`ops_glob` 审目录 `auth-init*` 返回空，仅留占位模板 `auth-init.tpl.js`；root `.gitignore:331-332` 忽略 `round27-wave-progress-hud/data/` 与 `auth-init.js`。
- 令牌三锚点 `tokens.css:14-16` `lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6` 保留，全文无 `--r27-`；契约 `:171-173` 保留 R24/R26/R22 HUD 锚点（`r24-element-hud`/`r26-wave-brief`/`r22-synergy-card`），R15~R26 既有契约不改。
- Go 真值守卫独立复跑：`go test ./internal/domain -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo' -count=1` exit 0，`TestNoInfraIdentifiersInRepo` PASS（8.16s）；`TestNoRealSecretsInRepo` 因本机无 `deploy/.local.keys` 被 **SKIP**（见诚实边界），由上面的独立 OPS 扫描补足。

---

## 独立复跑命令汇总（本人执行，均 exit 0）

| 命令 | 结果 |
|---|---|
| `pnpm exec vitest run battle_r27_wave_progress.contract.test.ts --config vitest.config.ts` | 1 file / 23 tests passed |
| `pnpm test`（miniapp 全量） | 209 passed \| 1 skipped files，1923 passed \| 2 skipped（1925），40.22s |
| `pnpm type-check`（vue-tsc --noEmit） | exit 0 |
| `pnpm type-check:test`（tsc -p tsconfig.vitest.json） | exit 0 |
| `pnpm build:h5`（uni build） | DONE Build complete，exit 0 |
| `go test ./internal/domain -run 'TestNoRealSecretsInRepo\|TestNoInfraIdentifiersInRepo' -count=1` | ok（8.215s，见诚实边界） |

## 诚实边界与观察项（非阻塞，不改业务代码）

1. `REPORT.md:37` 写「AI · 波次进度」，而实际 UI/截图/samples 均为「AI · 波次推进」——**既有 QA 报告小笔误**，终审如实注明，不构成缺陷、不修改业务代码。
2. `TestNoRealSecretsInRepo` 因本机 `laiyipao/deploy/.local.keys` 不存在被 SKIP，真值子串比对这条守卫**未在本环境执行**；已用独立 OPS 正则扫描（0 真值命中、仅 2 处回环）补足证据，但如实标注该环境限制。
3. L5 每波均有敌人（total≥3），浏览器侧空态「本波暂无敌人进度数据」不可达；由契约 `:50-66` 非法/0 总数用例补证，未伪造浏览器空态截图。
4. agent-browser 为桌面自动化、非物理真机，物理触控/性能未覆盖；本轮为本地回环环境实测。

## 最终结论

六门禁全部 PASS。R27 实时波次推进 HUD 以只读快照 + 纯函数派生实现，权威 replay_hash/settle 路径未被污染，AI 解读真实可验、诚实红线与空态逐字落地，真浏览器证据有效（5 图亲算亲读 + 96 帧采样 + 权威 hash 不变），真值零泄漏且 R1-R26 契约不回退。上述 4 项诚实边界均为环境/文案观察，不影响门禁判定。**verdict = pass**。
