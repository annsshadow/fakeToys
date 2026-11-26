# R10 玩法实测：装备·技能·宝石构筑策略深度（t98）

> 环境：本地回环 H5 真浏览器（agent-browser / Chromium，会话 `lyp-gp-t98`，视口 1258×588），API :8080。**非物理真机**。
> 证据目录：`docs/laiyipao/audits/round10-gameplay/`。未写入真实域名/IP/口令/私钥；未执行 git。

## 1. 实测路径

1. 首页 → 技能图鉴 `#/pages/skills/skills`：24 项技能、已拥有 3/24、合成技能 18；AI 推荐「重炮弹 每热量 6 伤害，尚未解锁」；近 7 场统计已出现。截图 `r10-08-skills-impact-unavailable.png`。
2. 装备图鉴 `#/pages/equipment/equipment`：18 件、已拥有 2/18、部位天花板 6；AI 推荐「母核碎片 护符天花板」，因果链引用战报 #1924 / `501a22c7`。截图 `r10-08-equipment-impact-unavailable.png`。
3. 宝石图鉴 `#/pages/gems/gems`：0/8，AI 收集缺口「锋锐石 攻击」，CTA 可去商城；商城确有「随机宝石 花费 60 钻石 → 1 颗」，非虚假入口。截图 `r10-07-gems-bag-cta-visible.png`。
4. 背包 `#/pages/bag/bag`：技能/装备/宝石页签真实可切；宝石页签显示「我的宝石（已佩戴 0/6） / 还没有宝石 / 去商城买一颗 →」。截图 `r10-09-bag-gem-tab.png`。
5. 技能升级：滚动到「升级 100 金币」后真实点击，DOM 出现升级成功反馈，燃烧弹伤害 100→105（每级 +50‰）。
6. 第 1 关 8× 战斗两次：升级前 39 杀/0 漏/92 反应/10614 分/C/0 星，钳分 19736→10614（72s），hash `501a22c719d74d7e`；升级后 39 杀/0 漏/93 反应/12088 分/B/1 星，钳分 19762→12088（82s），hash `1c36fcc2458e4316`。截图 `r10-05-upgraded-battle-result.png`。

## 2. 真实缺陷与 RED→GREEN

### D1 推荐两步确认成功后，写入结果不可见

源码事实：`skills.vue`/`equipment.vue` 的 `r7ConfirmApply()` 成功后先设 `r7ApplyDone`，再立刻 `r7CompareOpen=false`；成功文案只渲染在遮罩内，玩家永远看不到。

RED：新增 `src/pages/skills/skills_apply_feedback.test.ts` 与 `src/pages/equipment/equipment_apply_feedback.test.ts`，注入合法 rec-v1 impact，真点「应用推荐 → 确认替换」，断言写入参数、遮罩仍开、`.apply-done` 可见且含「已替换为「重炮弹」/「母核碎片」」、主按钮变「完成」。业务 RED 逐字 `expected false to be true`。

GREEN：两页成功文案改 `.apply-done`（role=status / aria-live=polite），主按钮成功后变「完成」、次按钮变「关闭」，移除成功后的自动关闭；各自 `<style scoped>` 用既有 `var(--ok)` 上色。未改 `common.css`，未新增视觉令牌。

### D2 宝石图鉴没有通往实际镶嵌页的入口

源码事实：`gems.vue` 唯一 CTA 是 `r3GoShop()`「去商城找宝石 →」，但宝石镶嵌发生在背包宝石页签。玩家在图鉴找不到「怎么戴上」。

RED：`src/pages/gems/gems.test.ts` 新增测试，断言 `.r3-ai-cta.secondary` 含「去背包镶嵌」且点击调用 `uni.navigateTo({url:'/pages/bag/bag'})`。RED 逐字 `expected false to be true`。

GREEN：新增第二个 CTA 与 `r3GoBag()`，复用既有 `.r3-ai-cta` 样式并仅加 `margin-left:16rpx` 的 scoped 次级样式。真浏览器硬重载后 DOM 含「去背包镶嵌 →」，真实点击后 URL 到 `?r10=7#/pages/bag/bag`，`errors=[]`。截图 `r10-07-gems-bag-cta-visible.png`。

### 契约回归插曲

首次全量测试失败：`r7ImpactApply.contract.test.ts` 报告 `skills 使用了未定义的 r7-apply-done`。因新类名带 `r7-` 前缀会被全局 CSS 契约扫描；把新类名改为非 `r7-` 的 `.apply-done`（仍为 scoped 样式）后契约与全量测试恢复绿，未越写域改 `common.css`。

## 3. 验证命令

```bash
cd laiyipao/miniapp
npx vitest run --config vitest.config.ts src/pages/skills/skills_apply_feedback.test.ts src/pages/equipment/equipment_apply_feedback.test.ts --reporter=verbose  # 2 passed
npx vitest run --config vitest.config.ts src/pages/gems/gems.test.ts --reporter=verbose                                                    # 3 passed
npx vitest run --config vitest.config.ts src/styles/r7ImpactApply.contract.test.ts ...                                                    # 20 passed
npm test          # 180 files passed | 1 skipped；1702 passed | 2 skipped；exit 0
npm run type-check && npm run type-check:test   # exit 0
```

## 4. Top3 养成层缺口与依据

1. **推荐收益卡未把「无可写替换」原因前置**：一局后技能/装备页的 AI 推荐卡已从 `NO_HISTORY` 变为规则引擎统计，但「真实收益预估」仍是「接口未提供真实预估」，没有「应用推荐」CTA。后端 t100 已核实这不是新服务端缺陷：t77 的归属安全语义要求候选必须是玩家已拥有且更优的项；实测账号只有初始 3 技能/2 装备，服务端在 `insights[].basis` 已写明「已拥有技能里没有同为主战且基础伤害高于当前…」，故意不下发不可写 id/replace。强行请求配置候选虽能预览，但写入必 400；授予归属后同一接口自动下发权威参数。真实缺口在信息展示：客户端错误位未消费 `insights[].basis`，玩家需展开「为什么推荐」才看到原因；应另开前端任务把该依据显示在收益卡错误位，服务端不得伪造可写 id（证据 `docs/laiyipao/audits/round10-recommendation/REPORT.md`，非物理真机/本地回环）。
2. **升级收益与战斗结果之间缺因果视图**：真实把燃烧弹 100→105 后，同关分数 10614→12088、评级 C→B、反应 92→93，但钳分仍把 19762 裁到 12088；玩家无法从页面判断「这 5 点基础伤害值不值 100 金币」。建议在战报中并列「本次养成前/后同一关的可信差值」，数据必须来自两次真实结算而非估算。
3. **宝石收集、镶嵌、收益三段割裂**：宝石图鉴只讲收集缺口，真正的佩戴在背包宝石页签，商城购买又是第三个入口；本次已补「去背包镶嵌 →」最短路径，但图鉴仍不能解释某颗宝石戴上后对反应伤害/热量的贡献。建议后端给佩戴后的属性差分，前端在同一页呈现，避免玩家在三个页面猜因果。

## 5. 截图清单（均非物理真机 / 本地回环）

- `r10-05-upgraded-battle-result.png`：升级后第 1 关结算，B/1★/39杀/0漏/93反应/12088分，钳分与选卡反馈可见。
- `r10-07-gems-bag-cta-visible.png`：宝石图鉴双 CTA「去商城找宝石」+「去背包镶嵌」。
- `r10-08-skills-impact-unavailable.png`：技能推荐有历史统计但无真实收益 CTA。
- `r10-08-equipment-impact-unavailable.png`：装备推荐同样停在收益不可用。
- `r10-09-bag-gem-tab.png`：背包宝石页签真实面板。
- `r10-06-gems-bag-cta.png`：点击新 CTA 后到达背包页。