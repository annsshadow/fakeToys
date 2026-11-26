# R7 玩法深度实测报告（新手引导 · 战斗策略 · 战后复盘）

> 测试说明：全部截图来自本地回环 H5 真浏览器（非物理真机），本地服务 HTTP `127.0.0.1:5174`，API `127.0.0.1:8080`。仅使用本地开发账号与占位数据；未记录任何口令、令牌或真实域名。

## 1. 实测范围

从清空本地存储的“新玩家”视角进入，依次走过：

1. 首页首屏 → 主行动卡 → 关卡选择
2. 战斗 → 8× → 选卡遮罩 → 跳过本波 → 5 波通关
3. 结算 → 一键查看本局战报 → 战绩/诊断
4. 成长页：技能图鉴、装备锻造台、宝石收集雷达
5. 反应图鉴：非空链与空链两种文案状态

截图见 `screenshots/`（均为非物理真机）：

- 首页与关卡：`r7-01-home-fresh.png`、`r7-02-home-primary-action.png`、`r7-03-stage-select.png`
- 战斗与选卡：`r7-04-battle-start.png`、`r7-05-card-choice.png`
- 修复后首屏引导：`r7-06-home-guide-fixed.png`
- 反应图鉴：`r7-07-reactions-claim-fixed.png`、`r7-08-reactions-empty-fixed.png`
- 结算：`r7-09-battle-result.png`
- 战报：`r7-10-battle-report.png`
- 成长：`r7-11-skills.png`、`r7-11-equipment.png`、`r7-11-gems.png`

浏览器 `errors --json` 全流程返回 `errors: []`。

## 2. 已修复的真实缺陷（test-first RED → GREEN）

### D1 新玩家首屏没有明确目标与下一步

**证据**：`r7-01-home-fresh.png` 第一屏只有 AI 洞察、公告、构筑评分；“当前关卡/开始”要滚到约 y=1348 才出现（`r7-02-home-primary-action.png`）。新玩家进入后不知道该做什么。

**RED**：`src/pages/index/index.test.ts` 新增测试“新玩家首屏给出明确目标与下一步：引导卡指向第 1 关”，断言 `.first-run-guide` 存在、含“新玩家先打第 1 关”“守住防线”，点击跳转到 `/pages/stage/stage?level=1`。实测 `expected false to be true`。

**GREEN**：`src/pages/index/index.vue` 在 `store.maxStage === 0` 时渲染可点击引导卡；文案给出目标、完成条件、后续解锁与“去第 1 关 →”。真浏览器硬重载并清空本地存储后，首屏直接出现引导卡（`r7-06-home-guide-fixed.png`），真实点击 (629,124) 后进入 `#/pages/stage/stage?level=1`。

### D2 反应图鉴空态谎称“反应伤害与攻击力无关”

**证据**：`src/pages/reactions/reactions.vue` 空链分支原文“反应伤害与面板攻击力无关”，但同页非分支文案已写 30%，且 `damage.ts` 反应攻击侧最高 300‰。这是会误导构筑决策的数值谎报。

**RED**：`src/pages/reactions/reactions.test.ts` 新增“R7 空反应链提示不得谎称攻击力完全无关”，断言不含“反应伤害与面板攻击力无关”、含“面板攻击力也有贡献”和“30%”。实测断言失败。

**GREEN**：空链文案改为“反应伤害中元素侧是主力，面板攻击力也有贡献但权重上限 30%”。真浏览器临时把出战配置改为单元素后重载，DOM 逐字确认新文案（`hasOld=false / hasNew=true`，`r7-08-reactions-empty-fixed.png`）；随后已恢复原配置 `[1,4,7,0]`。

## 3. 正向观察

- 选卡遮罩明确说明“点一张就装备上”“弃掉回 1 点热量”“全部不要了，直接下一波”，三张卡存在技能/属性/机制取舍，反馈即时（`r7-05-card-choice.png`）。
- 8× 通关结算给出下一星精确差额：实测“还差 1032 分就能拿 1 星（门槛 11941 分）—— 少漏怪、多触发反应就能提分。”（`r7-09-battle-result.png`），比空喊“冲 3 星”更可执行。
- 战报保留 AI 诊断、命中率、波次、峰值热量、元素/反应/地形、回放哈希与验真入口（`r7-10-battle-report.png`）。
- 技能/装备/宝石页均给出 AI 推荐、缺口与依据（`r7-11-*.png`）。

## 4. 最值得优先升级的 3 个玩法缺口

1. **战后复盘缺少“为什么没拿星”的归因**。战报只说“防守扎实：0/39 漏怪”，但本局 0 星、10909 分，未把“得分超时长额度 19726 → 10909（74s 内上限）”与星级门槛关联成可执行建议。玩家看不出该提速还是该加反应。证据：`r7-09-battle-result.png`、`r7-10-battle-report.png`。
2. **关卡目标感停留在数字门槛**。关卡页展示星级分数线，但战斗中没有实时提示“当前分数距下一星还差多少”；玩家只能结算后被动知道结果。证据：`r7-03-stage-select.png`、`r7-09-battle-result.png`。
3. **成长页推荐与实战构筑之间缺少一键落地**。技能页推荐“重炮弹”，装备页推荐“母核碎片”，但本局实际靠焰/冰/电反应拿分；推荐与当前出战构筑的因果关系没有被解释成“换上后预计提升什么”。证据：`r7-11-skills.png`、`r7-11-equipment.png`、`r7-10-battle-report.png`。

## 5. 验证

- 定向 RED：2 failed / 30 passed；定向 GREEN：2 files passed / 32 tests passed。
- 全量 `npm test`：172 files passed | 1 skipped；1634 tests passed | 2 skipped，exit 0。
- `npm run type-check`、`npm run type-check:test`：exit 0。
- 真浏览器复测同一流程：首页引导点击、空反应链文案、8× 五波通关、战报与成长页，`errors: []`。
- 未执行任何 git 操作；未写入真实域名、IP、口令或令牌。

## 6. 追加修复：Top3 缺口 1 + 2（t64）

### D3 战后复盘没有把钳分上限、真实得分与星级门槛串起来

**证据**：第 1 关 8× 五波通关后，结算卡分散显示「得分超时长额度：19684 → 7813（53s 内上限，满分需 60s）」与「分数 7813 / 0 星」，玩家无法直接看出「为什么没星、该改什么」（修复前同类证据 `r7-09-battle-result.png`）。

**RED**：`src/pages/battle/battle.test.ts` 新增「R7 结算归因把超时长钳分上限、实得分、星级门槛和漏怪数串成可执行建议」与「R7 结算归因没有钳分数据时不得编造超时长上限」。首轮定向 RED：3 failed / 41 skipped，逐字断言 `expected ... to contain '74s 内得分上限 19726'`。

**GREEN**：`src/pages/battle/battle.vue` 新增 `settleAttribution` computed，只从服务端返回的 `clamp_note` 解析时长与上限，从 `engine.cfg.level.star_targets` 取门槛，结合权威分数与漏怪数输出一句建议；没有 `clamp_note` 时不编造上限。真浏览器同一流程复测 DOM 逐字：「53s 内得分上限 19684，实得 7813；距 1 星门槛 11941 还差 4128；漏怪 0；建议缩短清波时间或少漏怪、多触发反应提分。」，`errors: []`。截图 `screenshots/r7b-02-result-attribution.png`。

### D4 战斗中没有实时星级差额

**证据**：修复前 HUD 只有防线/波次/击杀，星级门槛只在选关页和结算卡出现，玩家打到第 4 波仍不知道离 1 星还差多少。

**RED**：新增「R7 战斗 HUD 实时显示距下一星差额（门槛来自关卡配置，随分数更新）」，断言差距随引擎分数从 10000 → 12000 由「距 1 星 1941」变为「距 2 星 4916」；首轮 RED 逐字 `expected ... to contain '距 1 星 1941'`。

**GREEN**：`battle.vue` 新增 `levelStarTargets()` 与 `liveStarGap` computed 并在 HUD 渲染胶囊；门槛只取服务端关卡配置，不硬编码，结算后自动隐藏。真浏览器复测同一局 HUD 从「距 1 星 10424」随击杀推进到 8412 / 3884 / 331，跨门槛后切到下一星；截图 `screenshots/r7b-01-hud-live-star-gap.png`。

**验证**：定向 RED 首轮 3 failed / 41 skipped；GREEN 4 passed / 40 skipped。全量 `npm test`：173 files passed | 1 skipped；1642 tests passed | 2 skipped，exit 0。`npm run type-check` 与 `npm run type-check:test` 均 exit 0。全部截图为本地回环 H5 真浏览器，标注「非物理真机」；未执行 git 操作，未写入任何真值。

### D5 t63 §1F：战斗页两处「关系不大」同为误导文案

**证据**：`src/pages/battle/battle.vue:145` 图例写「反应伤害与面板攻击力关系不大」，`nextGoal` 反应偏少分支（原 :394，现约 :404）写同一口径；但 `src/game/damage.ts` 反应攻击侧贡献最高 300‰，与首页/图鉴已修口径冲突。

**RED**：`src/pages/battle/battle.test.ts` 两条定向断言分别要求图例与结算建议都不出现「关系不大」，且必须出现「面板攻击力也有贡献」与「30%」；两条 RED 逐字 `expected ... not to contain '关系不大'`。

**GREEN**：图例改为「元素侧是主力，面板攻击力也有贡献但权重上限 30%」；结算建议同步改为同一口径。真浏览器单元素构筑（skill_ids=[1,0,0,0]）第 1 关复测，整页 DOM `hasOld=false`（无「关系不大」）；因仅带 1 技能失守未进入 win 建议分支，胜利分支由定向/全量测试钉住。截图 `screenshots/r7b-03-honest-reaction-advice.png`（非物理真机 / 本地回环）。
