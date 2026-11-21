# R5 玩法深度实测：未覆盖功能（t46）

> 所有截图均为 **非物理真机**：真实 Chromium 无头浏览器 + 本地回环地址，非手机实物；
> 页面交互使用真实鼠标事件/输入事件，不直接调用业务函数。

## 1. 实测环境与范围

- H5：`http://127.0.0.1:<本地端口>/`（本地回环；Vite 运行模式）
- 后端：本地 API 服务；PostgreSQL 本地实例
- 浏览器：`agent-browser`，命名会话 `lyp-gp-r5`，viewport 1258×588
- 写域：`laiyipao/miniapp/src/game/**`、`laiyipao/miniapp/src/pages/battle/battle.vue` 及相应测试/审计文档

## 2. 本次走通的 4 条真实流程

### 2.1 签到与任务领取
- 入口：首页「签到」。真实点击「领取今日奖励」，DOM 出现「签到成功，获得 金币 1000」。
- 离线收益：DOM 显示「38 金币可领」，真实点击后显示「领取成功：43 金币」。
- 任务：真实点击「今日完成签到 1/1」后金币 3043 → 3543，按钮变「已领」，toast「已领取 金币 500」。
- 说明：同屏 energy 30 → 31 属随时间自然恢复；接口返回该任务 reward 仅 `{coin:500}`，不是奖励漏报。
- 证据：`screenshots/r5-signin-before.png`、`r5-signin-after.png`、`r5-signin-claims.png`、`r5-signin-task-claimed.png`。全过程 pageerror 为空。

### 2.2 公告筛选与真实空态
- 首页进入公告，真实点击「近 7 天」，在搜索框填入 `zzz_no_such_announcement`。
- DOM 出现「没有匹配「zzz_no_such_announcement」的公告」与「清除搜索」；空态不是假空数据。
- 证据：`screenshots/r5-announcements-list.png`、`r5-announcements-empty.png`。pageerror 为空。

### 2.3 第 1 关完整战斗闭环
- 关卡页真实点击「开始战斗（消耗 6 体力）」进入战斗；切换 8×；每波真实点击「全部不要了，直接下一波」。
- 结算：通关、击杀 39 / 漏怪 0 / 反应 91 / 本地分数 19746；服务端按 60s 满分规则裁剪为 11646（79s 内上限）；金币 +1250 / 体力 +5；回放哈希 `7edf9d08e6c1482e`。
- `/api/v1/me/battles?limit=1` 返回 battle_id=1445，哈希一致。
- 证据：`screenshots/r5-stage-before-battle.png`、`r5-battle-started.png`、`r5-battle-8x.png`、`r5-battle-result.png`。pageerror 为空。

### 2.4 战绩验真（正向 + 伪造反例）
- 验真页填入 1445，等待本地重放完成后点击「取复现信息」，显示「一致 — 分数可复现 / 验真通过」。
- 将哈希改为伪造值后再次验真，显示「验真失败 — 分数被判定为不可复现」，记录 #1445 标为不一致。
- 注意：输入后必须等待约 0.8s 再点击，否则前端会提示「请输入有效的 battle_id」；这是输入防抖而非缺陷。
- 证据：`screenshots/r5-verify-before.png`、`r5-verify-1445-info.png`、`r5-verify-1445-pass.png`、`r5-verify-1445-falsify.png`。pageerror 为空。

## 3. 真实缺陷与 RED→GREEN 修复

### 3.1 0 星结算仍空喊「冲 3 星」（写域内，已修复）
- **实测证据**：第 1 关结算 stars=0、score=11646，而 `/api/v1/config` 与关卡页给出的 1★ 门槛是 11941；页面却显示「下一步：再稳一点，冲 3 星拿满钥匙。」玩家尚未拿到 1 星，文案严重误导。见 `screenshots/r5-result-nextgoal-fixed.png`。
- **根因**：`battle.vue` 的 `nextGoal` 只按星数分支，完全没读服务端下发的 `star_targets` 与实际分数。
- **RED**：在 `src/pages/battle/battle.test.ts` 新增测试，构造 `eng.cfg.level.star_targets=[11941,16916,19503]`、`settle={win:true,stars:0,score:11646}`，断言文案含「1 星」「11941」「295」且不含「冲 3 星」。
  `npx vitest run --config vitest.config.ts src/pages/battle/battle.test.ts -t '0 星且分数未达' --reporter=verbose`
  → exit 1，`AssertionError: expected '防线800 / 1000第 undefined 关 · undefined…' to contain '1 星'`。
- **GREEN**：`battle.vue` 的 `nextGoal` 从 `engine.value.cfg.level.star_targets` 取权威门槛，用 `r.score`（服务端结算后的权威分数）算下一星差额：
  「还差 ${gap} 分就能拿 ${nextStar} 星（门槛 ${target} 分）—— 少漏怪、多触发反应就能提分。」缺门槛数据时回退旧文案。
  `npx vitest run --config vitest.config.ts src/pages/battle/battle.test.ts` → 41/41 passed。
- **真浏览器复测**：首次同会话复测仍看到旧文案，确认是浏览器 HMR 旧模块缓存（dev server 已返回新代码）。用带新查询串的地址强制重新加载后，同一流程结算 DOM 显示：
  「下一步：还差 1917 分就能拿 1 星（门槛 11941 分）—— 少漏怪、多触发反应就能提分。」
  本局 score=10024、击杀 39 / 漏怪 0 / 反应 95，pageerror 为空。见 `screenshots/r5-result-nextgoal-hardreload.png`。

### 3.2 跨端缺陷：首页仍称反应伤害「与面板攻击力无关」（写域外，已上报）
- 首页 DOM 逐字：「敌人对不同元素的抗性不同。两种元素叠加会触发反应链，反应伤害与面板攻击力无关 —— 这是低养成玩家翻盘的主要来源。」见 `screenshots/r5-home-start.png`。
- R4 已用 `damage.ts` 证明反应伤害的攻击侧权重最高 300‰（关系不大，但不是无关）；`battle.vue` 已改为「关系不大」。
- `src/pages/index/index.vue` 仍在说「无关」，属于 h5-ui 写域，不得在 gameplay 单端掩盖，已单独上报队长/h5-ui。

## 4. 校验结果

- `cd laiyipao/miniapp && npm test` → exit 0；Test Files 171 passed | 1 skipped (172)，Tests 1620 passed | 2 skipped (1622)。`no_deadlock.test.ts` 的「关 xx won/lost」是全量基线自身的 100 关模拟打印，不是失败。
- `cd laiyipao/miniapp && npx vue-tsc --noEmit -p tsconfig.json` → exit 0。
- `cd laiyipao/miniapp && npx tsc --noEmit -p tsconfig.vitest.json` → exit 0。
- `src/pages/battle/battle.test.ts` 夹具 `FakeEngine.cfg` 补充 `star_targets?: number[]` 类型，修复 t47 反馈的 TS2339；断言未弱化。
- 未执行任何 git 操作；证据中不含密钥、真实域名、口令或私钥。

## 5. 截图清单

`r5-home-start.png`、`r5-signin-before.png`、`r5-signin-after.png`、`r5-signin-claims.png`、`r5-signin-task-claimed.png`、`r5-announcements-list.png`、`r5-announcements-empty.png`、`r5-stage-before-battle.png`、`r5-battle-started.png`、`r5-battle-8x.png`、`r5-battle-result.png`、`r5-verify-before.png`、`r5-verify-fetched.png`、`r5-verify-1445-info.png`、`r5-verify-1445-pass.png`、`r5-verify-1445-falsify.png`、`r5-result-nextgoal-fixed.png`、`r5-result-nextgoal-hardreload.png`，均位于 `docs/laiyipao/audits/round5-gameplay/screenshots/`。
