# R4 战斗玩法真实体验实测报告

> 截图 r4-battle-start.png 由桌面 Chrome 真浏览器页面截取，**非物理真机**；访问本地回环开发服务，未包含任何公网域名、账号或口令真值。

## 范围与结论

- 逐段审计焰/冰/雷/毒/动能七条反应与图例文案：发现 1 个客户端可修的误导文案（已 RED→GREEN），1 个跨端伤害实现与文案不符的缺陷（已定量取证并移交后端 t41）。
- 技能/装备/宝石策略差异：现有 src/game/loadout_attacker.test.ts 已用「改一个输入 → 观测战斗结果变化」证明护甲、热量上限、机制卡强度三条养成线确实生效；本轮未发现新的客户端侧「显示有数值、战斗无变化」缺陷。
- 长局性能/内存：全量 npm test 覆盖 perf_baseline.test.ts 与 no_deadlock.test.ts（100 关基线全胜），本轮 168 files passed / 1593 passed；真浏览器长局采样因 agent-browser CLI 多次 open 挂起未完成，记录为环境限制，不把未跑到的项目写成已验证。

## 缺陷 1（已修）：结算建议谎称反应伤害完全不吃攻击力

- 证据：src/pages/battle/battle.vue 原文「多凑一种元素触发反应链，反应伤害不吃攻击力加成。」；同页图例写「反应伤害与面板攻击力关系不大」，而 src/game/damage.ts:281-318 的反应伤害攻击侧贡献最高 300‰。玩家照这句洗掉攻击词条会得到错误养成结论。
- RED：src/pages/battle/battle.test.ts 新增「反应偏少时的下一步建议不得谎称反应伤害完全不吃攻击力」，断言不含「不吃攻击力加成」且含「关系不大」。实测 1 failed / 39 skipped，逐字错误：AssertionError: 不能把「关系不大」说成「完全不吃」，否则玩家会误洗攻击词条: expected '防线800 / 1000第 undefined 关 · undefined…' not to contain '不吃攻击力加成'。
- GREEN：battle.vue 文案改为「多凑一种元素触发反应链，反应伤害与面板攻击力关系不大，优先凑齐元素搭配。」；battle.test.ts 40/40 通过。

## 缺陷 2（跨端，已移交）：反应范围伤害名不副实

- 文案承诺：elements.ts 的 steam_burst/overheat 描述为「半径内敌人受到 25% 范围伤害」。
- 定量探针：直接调用 engine.ts:1783 triggerReactionAoe(origin, "overheat", 10000n, att)，邻居应受 2500；关掉暴击实测 1000（10%），默认属性实测 1500（15%）。根因：engine.ts:1795 算出 25% 后把该值当 skillDamage 再走 resolveHit，被 DIRECT_WEIGHT=400‰ 与 roll=9999（判据为 roll >= 1000 - critPermille，必暴击）二次缩放；注释「roll:9999 // 不暴击」也写反。
- 边界：服务端 domain/engine_combat.go 同款算法，service/recompute_engine.go 会整局重算比对 replay_hash；客户端单独改会造成回放验真失配。按队长指示不改 engine.ts，交由 backend t41 双端同步并重生向量，t42 回归。探针文件为一次性取证，已删除，不留在测试套件中。

## 验证命令

- npx vitest run --config vitest.config.ts src/pages/battle/battle.test.ts：40 passed。
- npx vitest run --config vitest.config.ts src/pages/battle/battle.test.ts -t 「反应偏少时的下一步建议」：RED 1 failed，GREEN 通过。
- npm test：168 files passed | 1 skipped，1593 passed | 2 skipped。
- npm run type-check && npm run type-check:test：均 exit 0。

## t42：AoE 修复后回归实测（2026-11-22）

> 截图 r4-aoe-battle-start.png / r4-aoe-result.png / r4-aoe-report.png / r4-aoe-verify.png / r4-aoe-legend.png 均由桌面 Chrome 真浏览器访问本地回环开发服务截取，**非物理真机**；无公网域名、账号、口令或私钥真值。

- 定量探针：`src/game/overheat_aoe.test.ts` 新增「t42: 暴击与攻击力不得污染 25% 反应溅射（零护甲定量探针）」，直接以 `reactionDmg=10000n` 调 `triggerReactionAoe`，把邻居攻击力设 1000、暴击率 1000‰、暴击倍率 3000‰，实收恰为 2500n（25%），不写元素层数；与 `reaction_splash_contract.test.ts` 的 8 条跨端向量共 12/12 通过。旧实现把 splash 再走 `resolveHit`，必暴击会彻底改写该值，因此本用例是可失败的回归钉。
- 第 1 关 5 波完整闭环：真点击首页开始 → 关卡「开始战斗（消耗 6 体力）」→ 8× 倍速 → 每波选卡遮罩点「全部不要了，直接下一波」→ 结算 `win`，击杀 39 / 漏怪 0 / 反应 88 / 分数 11351（服务端按 77s 时长上限从 19729 修正，属既有规则）。
- 资源入账：结算显示金币 +1220、体力 +5；随后 `/api/v1/wallet` 返回 coin=3220、energy=31，`/api/v1/me/battles?limit=3` 返回 battle_id=1444、replay_hash=74507badead2208b、result=win、score=11351，与结算页完全一致。
- 回放与战报：真打开 `/#/pages/battles/battles?battle_id=1444`，战报页展示同一 hash、掉落、元素/反应统计；点「去验真此局」再点「本地重放并提交验真」，页面返回「一致 — 分数可复现 / 验真通过」，验真记录 #1444 落库。全程 `agent-browser errors --json` 为空。
- 图例文案一致性：真打开战斗图例，DOM 逐字「反应伤害与面板攻击力关系不大，凑齐元素搭配更重要」及蒸汽爆发/过热「半径 120/90 像素内的敌人受到 25% 范围伤害」，与定量探针 25% 一致。
- 附带发现（文档缺陷，已修）：`battle.vue` 图例入口旧注释仍写「打开图例不改变战斗状态」，但 R3 已改为打开即暂停、关闭自动继续，真浏览器复测也确认 `paused:true → close → paused:false`。已把注释更新为真实行为；该行为已有 `battle.test.ts:858` 的回归测试覆盖，故注释修订不另造弱测试。
- 全量验证：`npm test` 169 files passed | 1 skipped，1604 passed | 2 skipped；`npm run type-check`、`npm run type-check:test` 均 exit 0。
- 可重放脚本：`play_t42.sh` 与 `play_t42.log` 保留在同一目录，记录真坐标点击与各波状态轮询，便于复查。
