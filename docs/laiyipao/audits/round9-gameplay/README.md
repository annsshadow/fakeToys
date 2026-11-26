# R9 玩法深度实测：元素反应策略与战斗反馈

> 测试环境：本地回环 H5 / API + 真浏览器 Chromium 1258×588，**非物理真机**。截图均来自本地回环真浏览器，不含真实线上域名、IP、凭据。

## 1. 实测范围

从新玩家视角连续完成第 1～3 关（每关 5 波，共 15 波），全程 8× 倍速，逐波点击「全部不要了，直接下一波」推进；观察战斗 HUD、反应反馈、卡组 offer、结算归因与关卡衔接。采集 `errors --json` 全程为 `[]`。

- 第 1 关：通关 / C / 击杀 39 / 漏怪 0 / 反应 67 / 8992 分 / 钳分 19684→8992（61s）/ 金币 +1160 / 回放哈希 e796cd8deaf8d7a2。
- 第 2 关：通关 / C / 击杀 38 / 漏怪 0 / 反应 94 / 10130 分 / 钳分 19208→10130（71s）/ 金币 +1230 / 回放哈希 3e1904113b0a6dfa。
- 第 3 关：通关 / B / 击杀 32 / 漏怪 0 / 反应 102 / 11521 分 / 钳分 16297→11521（76s）/ 金币 +1290 / 回放哈希 5179a696d5813b1f。

## 2. 真实缺陷与 RED→GREEN

### D1 反应只有瞬时 canvas 粒子，8× 下无持久可读反馈

证据：`screenshots/r9-04-combat-frame1.png` 战斗画布中央仅有「过热!」弹字；8× 下粒子和弹字一闪即逝，HUD 无「刚才触发了什么、造成多少伤害」的记录。

- RED：`src/pages/battle/battle.test.ts` 新增「R9 反应事件在 HUD 留下可读的最近反应记录，而不只是瞬时粒子」，注入 reaction 事件后断言 `.reaction-feed` 含反应名与伤害；实测 `expected false to be true`。
- GREEN：`src/pages/battle/battle.vue` 新增 `recentReactions` / `noteReactions(events)`，从引擎 reaction 事件读取反应键、伤害与抗性，保留最近 3 条，显示在 HUD 顶部 `.reaction-feed`；`pointer-events:none` 不遮挡战场，每局开始清空。
- 复测截图：`screenshots/r9-06-reaction-feed.png`，HUD 左上可读「过热 -216 / 超导 -133 / 超导 -65」，战场与操作区未被遮挡。

### D2 通关后没有下一关入口，前 3 关衔接需要多步手动找关

证据：原结算只有「查看本局战报 / 再来一局 / 返回关卡」，返回选关页仍停在当前关；连续闯关需退出后重新选关。

- RED：`src/pages/battle/battle.test.ts` 新增「R9 通关后提供下一关入口，直接带着解锁关卡进入选关页」，断言 `.next-level-btn` 存在且 `uni.redirectTo` 到 `stage?level=2`；另加「结算失败不显示下一关入口」防谎报解锁。实测 `expected false to be true`。
- GREEN：`battle.vue` 新增 `nextLevelId` / `canGoNextLevel` / `goNextLevel()`：仅 `win && !settleFailed`，且下一关已存在并解锁时，才显示「前往第 N 关 →」并 `redirectTo` 对应选关页。
- 复测截图：`screenshots/r9-07-next-level-cta.png`；真实点击「前往第 2 关 →」后 URL 切到 `stage?level=2`，DOM 显示「当前关卡 2」；再连续点「前往第 3 关 →」到 `stage?level=3`，DOM 显示「当前关卡 3」。第 3 关结算继续出现「前往第 4 关 →」（`screenshots/r9-08-level3-result.png`），跨关无残留反应记录。

## 3. 验证

- 定向 RED：2 failed / 1 passed；GREEN：3 passed / 3。
- `npx vitest run --config vitest.config.ts src/pages/battle/battle.test.ts`：48 passed。
- `npm run type-check`（vue-tsc）与 `npm run type-check:test`（tsc）均 exit 0。
- `npm test`：176 files passed | 1 skipped (177)，1686 tests passed | 2 skipped (1688)，exit 0。
- 真浏览器全程 `errors --json` 为 `[]`。
- 仅改 `src/pages/battle/battle.vue` 与 `src/pages/battle/battle.test.ts`，未碰 admin / 后端，未执行 git。

## 4. 最值得优先升级的 3 个战斗层缺口

1. **反应策略因果链仍不完整**：现在能读到反应名和伤害，但没把「当前元素组合 → 触发哪种反应 → 适合什么敌人/场景」连成一句话；玩家看到「过热 -216 / 超导 -133」仍难判断下一张该拿什么元素卡。建议在未来版本把反应图鉴的克制信息压缩成战斗内一行 contextual hint。
2. **选卡取舍反馈偏弱**：本轮三关都全程跳过卡牌仍通关（C/C/B），且结算没有把「跳过卡牌」与分数/反应数变化建立因果；玩家难以判断装备卡牌到底值不值。建议给选卡卡面增加与当前构筑的相对收益提示，并在结算统计弃/取卡数与对应反应数变化。
3. **钳分与星级预警缺少战斗中换算**：HUD 已有「距满分 Xs」与实时星级差额，但「当前速度会损失多少分 / 掉到几星」要到结算才一次算出；玩家在关键波次前无法及时权衡稳守还是抢速。建议把结算归因里的 `上限→实得→差额` 公式以轻量形式提前到 HUD。

> 截图目录：`docs/laiyipao/audits/round9-gameplay/screenshots/`（全部非物理真机、本地回环真浏览器）。