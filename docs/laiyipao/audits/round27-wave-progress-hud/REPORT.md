# R27 复查报告：实时波次推进 HUD 真浏览器闭环

- 任务：t209（attempt_id=70eb5df9-ee93-44e2-9479-e74c1bcb9fef）
- 复查对象：`waveProgressHud()` 只读快照 + `buildWaveProgress()` 纯函数派生 + H5 `.r27-wave-progress` 渲染
- 判定：**PASS**（六条硬门禁全部满足）
- 口径：QA 只读复查，未改业务源码、未 git；agent-browser 真浏览器（390×844，非物理真机），非物理真机限制如实标注。

## 1. 权威数据源与逐字段比对

权威波次端点 `GET /api/v1/levels/5`（带 Bearer token）实测 spawns 聚合 total：

| 波次 | 权威 spawns | 权威 total | E2E tick-log 逐帧 total | 一致 |
|---|---|---|---|---|
| W0 | 1×3 | 3 | 3 | ✅ |
| W1 | 1×5 | 5 | 5 | ✅ |
| W2 | 1×4 + 1×2 | 6 | 6 | ✅ |
| W3 | 2×5 + 1×3 | 8 | 8 | ✅ |
| W4 | 2×7 + 4×4 + 3×3 | 14 | 14 | ✅ |

config 不下发 waves（`levels[5].waves=0`），客户端仅 `api.fetchLevel()` 取权威波次；HUD 只读 `eng.cfg.level.waves`，未自造数据。

## 2. 四时点 DOM 逐字段核验（samples.jsonl 双层解析 47 帧）

| 时点 | 截图 | data-r27-progress | progText | fill 宽度 | AI 结论 |
|---|---|---|---|---|---|
| 首波有数据 | r27-01 | 0 | 本波 1/3 · 33.3% → 3/3 · 100.0% | 33.3%→100% | 本波共 3 只，已处理 3 只，场上 0 只、待出场 0 只，清理进度 100.0% |
| 中途 | r27-02 | 1 | 本波 2/5 · 40.0% | 40% | 本波共 5 只，已处理 2 只，场上 3 只、待出场 0 只，清理进度 40.0% |
| 清波 100% | r27-03 | 0 | 本波 3/3 · 100.0% | 100% | 本波共 3 只，已处理 3 只，场上 0 只、待出场 0 只，清理进度 100.0% |
| 下一波换 total 归零 | r27-04 | 1 | 本波 0/5 · 0.0% | 0% | 本波共 5 只，已处理 0 只，场上 5 只、待出场 0 只，清理进度 0.0% |

- `resolved = total − onField − pending`，含击败+漏怪不拆字段；清波 100.0%、下一波换 total 且归零，均逐字段一致。
- fill 宽度与 clearedPct 一致性：47 帧全检，`|fill% − domPct| > 0.15` 命中 **0** 帧。
- 百分比严格一位小数（t208 修复点）：所有 progText 均为 `x.x%`（33.3/40.0/100.0/0.0）。

## 3. AI 四判据 / 依据 / CTA / 诚实红线（逐字）

- ① 可见汉字 AI：5 张截图均见「AI · 波次进度」。
- ② 真实数据结论：逐帧 AI 结论数值与 DOM progText 同步变化。
- ③ 依据行逐字：`依据：引擎 waveProgressHud() 只读快照（当前波 spawns 总数 − 场上存活 − 待出场）；只读派生不进 replay_hash` —— basis 全帧命中（BASIS_OK=true）。
- ④ CTA 逐字：`继续清理场上敌人 →`（绑 openHelp），截图可见。
- 诚实红线逐字：`辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准` —— honesty 全帧命中（HONESTY_OK=true）。
- hasUndefined / hasNaN：47 帧全 false。

## 4. hash 不污染（静态 + 动态）

- 静态源码：`engine.ts:811-821` `waveProgressHud()` 仅读 `cfg.level.waves`（spawns.count 求和）、`enemies.filter(!dead)`、`spawnQueue.length`，返回新对象，不触碰 step()/replayHash()/buildSnapshotSegment()/score/settle/loot。契约 `battle_r27_wave_progress.contract.test.ts:162` 源码守卫 `不得调用 / step() / replay_hash / score / loot` 通过。
- 动态实测：真实点击通关，结算 `result=通关`、`status=200`、`battle_id=478`、`回放哈希 7ddbb762bc017a11（可验真）`；`hashResponses=[]`（HUD 未额外触发 hash 端点）。

## 5. 空态 / 非法数据（纯函数补证，如实标注浏览器侧不可达）

- L5 每波均有敌人（total≥3），浏览器侧空态 `本波暂无敌人进度数据` 不可达，如实标注。
- 契约测试 `battle_r27_wave_progress.contract.test.ts:54-63` 覆盖非法 raw / waveTotal=0 → `empty=true, ai=''`，不抛异常、不编造；`waveProgressHud.ts:30-42` 对 `raw/waveIndex/waveTotal/onField/pending` 全做有限数校验并 clamp。

## 6. 探针（过滤 Autoplay 噪声）

- 原始 `pageErrors=64`，逐条均为 `unhandled:The play() request was interrupted by a call to pause()`（Chrome Autoplay 限制噪声，与 R24/R25/R26 同口径），过滤后 `peReal=0`。
- `consoleErrors=0 / apiFailures=0 / failedRequests=0`；`settleResponses=[{status:200,battle_id:478}]`。

## 7. 截图（5 张，sha 两两不同，全部亲读非登录页 390×844）

| 文件 | sha256 |
|---|---|
| r27-01-first-wave.png | 1747bcd7584476e3eb50d581e639123fb04a217e7eebe8006cb8b8efe213297e |
| r27-02-mid-wave.png | 0816cb98df703f09834950cded2b5b0ba1bb784e83b78efef0492479a5d45d23 |
| r27-03-clear-100.png | 73ca5f4c22c0bb2449df10559b6b8f81f9c54257b10c413dc5febd4c04e50746 |
| r27-04-next-wave.png | 2ab016400876e1d009fc26014daa42c9b48ad2d146e6a60594e77f40e2f68611 |
| r27-05-settle.png | 06ea3c7589e133f816e80654f639fd7128128a556899d2b8a6e3aecf29d8979c |

- 防线 1100/1100、波次 x/5、击杀、星级、钳分-距落分、R24 元素层（焰+冰预判）、R26 下一波简报（游荡者×5）在 390×844 全部可见，`docOverflow=false`、`hudWrap=wrap`，无横向溢出、未挤出视口。

## 8. 零真值 / 清理 / QA 还原

- 审计目录正则扫描（JWT / Bearer / 带口令连接串 / 个人路径 / 内网 IP）：0 命中。
- 含真 JWT 的 `auth-init.js` 已删（AUTH_DELETED=YES，仅留 `auth-init.tpl.js` 占位符模板）；`.gitignore:331-332` 已忽略 `data/` 与 `auth-init.js`，`git check-ignore` 确认。
- QA 还原：`pg_restore --clean --if-exists` 从 `data/qa-before.dump`（sha256=d6f5527b6ff50b962873162f31e3e2be2bc1ef9af92599679f9ad53bc5f2eaee）恢复；还原后 `battle_records=104`、`id=478 计数=0`、`max(id)=477` —— E2E 新增战报已清除。

## 9. 验证命令汇总

- 契约：`npx vitest run src/pages/battle/battle_r27_wave_progress.contract.test.ts` → 1 file / 23 tests ALL PASSED（343ms，19:49:47 复跑）。
- type-check exit 0（依赖阶段 t208 已跑，本轮未改源码）。
- 主 E2E：`e2e-full.sh`（session=lyp-r27-full，真实 click 通关）跑满 96 帧到 SETTLED。

## 10. 脚手架说明与限制

- 首跑曾因 `agent-browser eval` 外层 JSON 包裹使 tick 状态首字段带引号导致精确匹配失败（卡在 CARD_PHASE 不推进、不截图）——为脚手架问题非产品 bug；已在解析处 `tr -d '"'` 修复并重跑成功，未用 shim 掩盖、未误判产品缺陷。
- agent-browser 为桌面浏览器自动化，非物理真机；物理真机触控/性能未覆盖。
- 本报告基于实跑证据，未采信实施方自述。
