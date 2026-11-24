# R26 设计终审报告：选卡阶段下一波敌情简报（round26-next-wave-brief）

- 任务：t202 / attempt 662ea0fe-f722-4f75-8446-2a18cd62e718
- 终审人：design-reviewer-wb（设计终审 · workbuddy 路由）
- 终审对象：SPEC + 纯函数 nextWaveBrief.ts + battle.vue 接线 + t201 真浏览器复查证据
- **verdict = pass**
- 终审方式：独立亲读 SPEC / 契约 / 源码 / 审计报告 / 截图 / 状态日志 + 独立复跑测试与守卫，**不采信实施/复查成员自述**。
- 约束：QA 只读终审，未改业务源码、未 git 提交。

## 门禁① 规范与契约（SPEC 符合 + 契约真实导入）

- SPEC.md（64 行）亲读，§0 缺口、§1 只读派生边界、§2 纯函数契约、§3 选择器与 390×844 布局、§4 AI 四判据、§5 诚实红线与不回退、§6 契约 test-first 门禁、§7 三端分工齐备。
- 契约 battle_r26_next_wave_brief.contract.test.ts（168 行 / **21 测试**）亲读：顶部 `import { buildNextWaveBrief } from './nextWaveBrief'` 为**真实模块导入**，测试文件内无内嵌参考实现（符合 SPEC §6 禁止事项），避免"功能缺失仍全绿"的伪 RED。
- 断言覆盖：按 enemy_id 聚合同组相加、count 降序 + 同数量 enemy_id 升序、威胁优先级首领>飞行>潜地>远程、未知敌种 `#id`+unknown=true+空威胁、五类空态、AI 结论真实来自聚合；六选择器 `r26-wave-brief`/`r26-brief-head`/`r26-enemy-row`/`data-r26-enemy`/`r26-brief-empty`/`r26-ai-note`、逐字空态/红线、CSS 限高、battle.vue 真实调用、engine 依据、令牌三锚点、零真值文本守卫。
- **独立复跑**：`vitest run battle_r26_next_wave_brief.contract.test.ts` → 1 file / **21 passed** / exit 0。
- 判定：**PASS**。

## 门禁② test-first 证据（RED → GREEN 链）

- t204 RED：契约补强为真实模块导入后，在 nextWaveBrief.ts 不存在时跑出 `Cannot find module ./nextWaveBrief`（ERR_MODULE_NOT_FOUND，exit 1）。本人独立 `git cat-file -e HEAD:.../nextWaveBrief.ts` 得 exit 128 + "exists on disk, but not in HEAD"，**证明 RED 基线真实**（HEAD 无实现、工作树才有）。
- t200 GREEN：纯函数 + computed + 模板 + CSS 落地，契约 19/19、全量、type-check、build:h5 绿；CTA 措辞改为不承诺机制。
- t205 两个真实缺陷 RED→GREEN：①按敌种 id 查找兼容真实 Map/Record（契约断言 `new Map([[42,裂隙兽],[7,首领]])` 按 id 命中，非数组下标）；②下一波标题 off-by-one（契约断言含 `r26Brief.waveIndex + 1` 且 **不含** `+ 2`）。
- 判定：**PASS**。

## 门禁③ 集成正确（Map 取名 + 无 off-by-one + 权威口径）

- nextWaveBrief.ts 亲读：`defs instanceof Map ? defs.get(enemy_id) : defs[enemy_id]`，按 id 查找；未知敌种 fallback `#${enemy_id}` + unknown=true + threat 空串，不编造。
- battle.vue:806 亲读：`buildNextWaveBrief(eng.cfg.level.waves, eng.waveIndex, store.enemyMap)` 传**真实 Map**（store.enemyMap），非 `store.config?.enemies as any`（契约有反向断言）。
- 标题 `下一波 · 第 {{ r26Brief.waveIndex + 1 }} 波`（battle.vue:240），waveIndex 为下一波 0 起下标，+1 得 1 起显示，与 UI 既有波次口径一致，无 off-by-one。
- 权威数据逐字段比对：data/level5.json 亲读 waves[1]=1×5、waves[2]=1×4+1×2、waves[3]=2×5+1×3、waves[4]=2×7+4×4+3×3，与 REPORT §2 表格「第2波游荡者×5 / 第3波游荡者×6 / 第4波疾行者×5·游荡者×3 / 第5波疾行者×7·腐化犬×4·壮硕体×3」**逐字段一致**。
- 判定：**PASS**。

## 门禁④ 视觉与 390×844（限高滚动 + 手卡/跳过可见）

- 六图 SHA256 亲算与 REPORT §8 **逐字一致**、两两不同：r26-01=3d4b23ff…、r26-02=5450d595…、r26-03=856c8300…、r26-04=ec75b3e8…、r26-05=f158b31e…、r26-06=c519e59d…（完整值见 REPORT）。
- 亲读 5 图（01/02/03/04/06）均 390×844、非登录页：r26-01 第2波游荡者×5+AI 四判据；r26-02 第4波疾行者×5/游荡者×3；r26-06 第4波电鳍兽×2[远程]+AI [远程]×2；三图均可见 3 张手卡与「全部不要了，直接下一波」按钮。r26-04 为滚动后帧：简报行区收起、仅余 AI 区，手卡/跳过仍完整可见，**证明 max-height 限高滚动生效且未把选卡挤出视口**。
- battle.vue:2252-2254 CSS 落地 `.r26-wave-brief { max-height: 160rpx; overflow-y: auto; }`，契约有正则断言。
- 判定：**PASS**。

## 门禁⑤ 行为无回归（权威 hash 不变 + 纯派生无副作用）

- r26-03-settle 亲读：结果=通关，回放哈希 `ec1451ab96e66b20`（可验真），有「查看本局战报」按钮；网络探针 settle=[{status:200, battle_id:477}]，hashResponses=[]（R26 简报自身未产生任何 replay_hash 写响应）。
- state-log.txt（67 行）亲读：状态机 WAIT→CARD_PHASE→…→SETTLED，各帧 ce/af/fr 全 0；pe 计数为 Autoplay `play() request was interrupted` 噪声（与 R24/R25 同口径），过滤后真实页错误 peReal=0。
- nextWaveBrief.ts 为纯函数：仅入参 waves/下标/敌种表，返回只读快照，不含 step()/beginWave()/settle()/写引擎/写结算；契约 `SPEC 含只读派生边界` 断言 `不得调用`/step()/replay_hash/score/loot 逐字在案。权威 replay_hash 与 score/loot/发牌链路未受影响。
- **独立复跑**：miniapp 全量 `vitest run` → 208 passed | 1 skipped（files）、**1900 passed | 2 skipped**（tests）/ exit 0；`vue-tsc --noEmit` exit 0；`npm run build:h5` → Build complete / exit 0。
- 判定：**PASS**。

## 门禁⑥ 安全与交付（零真值 + 不回退）

- 含真凭据的 auth-init.js 已删（`ls` 返回 No such file），仅留 auth-init.tpl.js 且仅含 `__LYP_TOKEN__` 占位符。
- .gitignore:327-328 覆盖 `round26-next-wave-brief/data/` 与 `round26-next-wave-brief/auth-init.js`（git check-ignore 命中，见 REPORT §9）。
- OPS 精确正则扫描审计目录：JWT 形状 / 带口令 postgres DSN / 私钥或 root@IP / 个人 C:\Users 路径 —— 各 **0 命中**；Go 真值守卫 `go test ./internal/domain/...`（TestNoRealSecretsInRepo 等）独立复跑 exit 0。
- 不回退：令牌三锚点 `lyp-tokens-v1`/`lyp-tokens-v3-r3`/`lyp-tokens-r6` 保留且无 `--r26-` 新令牌；battle.vue 保留 `showCards`/`class="sheet"`/`hand-card`/`r22-synergy-card` 锚点（R22 协同卡不回退），R15 时间线 / R18-R25 端点与契约未改（契约断言在案）。
- 判定：**PASS**。

## 诚实边界（非阻塞）

- agent-browser 为 Chromium 模拟视口 390×844，**非物理真机**；所有 DOM/网络结论来自本地回环实跑，未夸大为线上真机覆盖。
- 空态 / 未知敌种：L5、L22 权威波次全为已知敌种，线上不存在空 spawns，浏览器侧未覆盖，**未伪造**；由纯函数契约（未知敌种 #id / 五类空态）补证。
- QA 库已还原（battle_records 回落 103，本轮战报消失），终审未改业务源码、未 git。

## 最终结论

六门禁逐条独立核验均满足，未发现新真实缺陷。R26「选卡阶段下一波敌情简报」为只读派生、口径与权威波次逐字段一致、真浏览器证据有效、结算哈希未污染、零真值与历史契约不回退。**verdict = pass**，t202 通过。
