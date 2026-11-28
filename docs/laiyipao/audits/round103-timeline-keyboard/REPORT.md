# R103 时间线抽屉键盘闭环 — 真浏览器 QA 复查报告

> 轮次：R103（时间线抽屉键盘闭环）　任务：t277　角色：same-route-qa（R40 真浏览器 QA）
> HEAD 基线：`91e776228925d40df75ac87af4e1d71c8c4662d3`（全程未变）
> 唯一入库写域：`docs/laiyipao/audits/round103-timeline-keyboard/REPORT.md`（`data/`、`screenshots/` 为 UNTRACKED 证据）

## §1 范围与方法

- **验证对象**：R103 八处改点，全部集中在 `laiyipao/miniapp/src/pages/battle/battle.vue`；契约 `laiyipao/miniapp/src/styles/r103TimelineKeyboard.contract.test.ts`（168 行 / 12 it）。
- **四改点源码只读核实（全落地）**：
  - `:77` `<text class="r28-feed-cta" role="button" tabindex="0" @click.stop="openReactionTimeline" @keydown.enter="openReactionTimeline">查看完整反应时间线 →</text>`
  - `:261` `<view v-if="reactionTimelineVisible" ref="timelineOverlayRef" class="r29-timeline-overlay" role="dialog" aria-modal="true" aria-label="完整反应时间线" @click="closeReactionTimeline" @keydown.esc="closeReactionTimeline" @keydown.tab="trapTimelineTab">`
  - `:265` `<view ref="timelineCloseRef" class="r29-timeline-close" role="button" tabindex="0" @click="closeReactionTimeline" @keydown.enter="closeReactionTimeline"><text>×</text></view>`
  - `:449` `<view class="r29-result-timeline-cta" @click.stop="openReactionTimeline" role="button" tabindex="0" @keydown.enter="openReactionTimeline">查看完整反应时间线 →</view>`
- **逻辑核实**：`:1056` `openReactionTimeline(e?: Event)` 首行记 `timelineTriggerRef.value=e.currentTarget`、先 `closeHelp()` 互斥、`:1072` 置 `visible=true`、`:1073-1076` 若未暂停未结算则 `doPause()` 并记 `timelinePausedByDrawer=true`、`:1077-1081` `nextTick` 聚焦 `timelineCloseRef`（含 `$el` 回退）；`:1085` `closeReactionTimeline` 置 false、恢复暂停记账、`:1091-1094` 归还焦点到 `timelineTriggerRef`；`:1098-1115` `trapTimelineTab` 取 overlay 内 `[tabindex="0"],[role="button"]` 做首尾循环。
- **环境**：Chromium（agent-browser 0.41.1）；三视口 390×844 / 768×1024 / 1280×800；H5 dev server `http://127.0.0.1:5174/laiyipao-miniapp/#/pages/battle/battle?level=1`（本地回环）；后端 API 8080 未启动。
- **数据构造**：`data/r103-init.js` patch XHR（记 `__qaHits`、合成响应、定时触发 `onreadystatechange`/`onload`）与 `window.fetch`；`localStorage` 注入 `lyp_config_cache_v1`（version `r103-qa-v1`）+ 占位令牌 `r103-qa-placeholder-token`（exp 2099）+ `lyp_profile`。`/battle/token` 合成 token_id 9001、关卡 2 波各 4 敌、build（技能 Record id1 fire slot0 / id2 ice slot1 + attacker）。**零真值**。CTA 与抽屉态经 Vue 实例注入产生（见 §6）。
- **改前基线**：引用 `docs/laiyipao/recon/ROUND102_A_TIMELINE_CLOSE_RECON.md`（t268）——改前 CTA `role=null`/`tabIndex=null`、overlay 无 dialog、ESC 不关闭、AX 无关闭按钮。

## §2 三视口逐项验证 A–G

三视口走查脚本 `data/walk103.sh`（job `bash-3ba8b2910c414b84`，输出 `data/w-{390,768,1280}.txt` 各 120 行）。三视口结果一致，逐项如下。

| 项 | 判据 | 390×844 | 768×1024 | 1280×800 |
|---|---|---|---|---|
| A | 两处 CTA `role=button`/`tabIndex=0`/真 Tab 命中/Enter 开抽屉 | feed CTA `role=button`/`tabAttr=0`/`tabIndex=0`/`cursor=pointer`/`vis=true`；8 次真 Tab 序列命中 `r28-feed-cta`→`help-chip`→BODY 循环；focus+Enter 开抽屉 | 同左 | 同左 |
| B | overlay `role=dialog`/`aria-modal=true`/`aria-label=完整反应时间线`/焦点自动落 `r29-timeline-close` | `ov={role:dialog, ariaModal:true, ariaLabel:完整反应时间线}`；`close={role:button, tabIndex:0}`；`active.cls=r29-timeline-close`/`inOverlay=true`；AX `dialog "完整反应时间线"` + `button "×"` | 同左 | 同左 |
| C | 抽屉内真 Tab 与 Shift+Tab 各 ≥3 次 `activeElement` 不逃逸 | Tab×3 与 Shift+Tab×3 全部 `active=r29-timeline-close`/`inOverlay=true` | 同左 | 同左 |
| D | ESC 关闭抽屉 + 恢复暂停 / 焦点归还 | `ov=false`/`vis=false`/`paused=false`/`timelinePausedByDrawer=false`；**焦点归还未坐实**（落 BODY，见 §6） | 同左 | 同左 |
| E | 结果页 CTA Enter 开抽屉；关闭按钮 Enter 关闭 | `result={role:button, tabIndex:0}`；focus result+Enter 开；focus close+Enter→`ov=false`/恢复；**焦点归还未坐实**（见 §6） | 同左 | 同左 |
| F | 开抽屉暂停 / 与图例互斥 / 零横溢 / 令牌锚点 / 零 `--r103-` | 开抽屉 `paused=true`/`timelinePausedByDrawer=true`；点图例后 `ov=false`/`legend=YES`/`active=legend-close`/`inLegend=true`/`timelinePausedByDrawer=false`（互斥坐实）；`iw=cw=sw=390`/`hOverflow=false` | `iw=cw=sw=768`/`hOverflow=false` | `iw=cw=sw=1280`/`hOverflow=false` |
| G | 门禁 G1–G7 全绿 | 见 §5 | 同左 | 同左 |

**AX 证据（768 视口，展开态快照）**：
```
- button "查看完整反应时间线 →" [ref=e2]
- dialog "完整反应时间线"
  - StaticText "AI · 完整反应时间线"
  - button "×" [ref=e3]
```

**真 Tab 命中证据**：`A_TAB_SCAN` 连续 8 次真实键盘 Tab，`activeElement` 依次落在 `r28-feed-cta`、`help-chip`，随后在页面可聚焦元素间循环——两处 CTA（feed / result）均可达。

## §3 截图 sha256sum 实测（以落盘实物为准）

9 张截图落 `docs/laiyipao/audits/round103-timeline-keyboard/screenshots/`，`sha256sum` 逐张实测（ops_bash），UNIQUE 9/9，与本报告逐字一致：

| 截图 | sha256 |
|---|---|
| `r103-feed-390.png` | `1d35382d335b468b654cc3f20a67166195d896b507a5f7ae5f43df0172955f22` |
| `r103-feed-768.png` | `befbb58a4746333dbf406941175b4830a89384c1da88f2f0d99864078553525c` |
| `r103-feed-1280.png` | `b8ddb020f7a8f6b74764c80bcb3de066ab1e708101f3206c659f0c400e6dcc00` |
| `r103-result-390.png` | `da6a72f994406c99341c992d923076de169c7b1a7d737845f28d4beed59480db` |
| `r103-result-768.png` | `5dbb2e1e599948eb9c5f6fa5b3aa32d982e94976eca2f0b017ad482c95bd5322` |
| `r103-result-1280.png` | `67969d15ce7ecdabf8e8d400bc361ac1f7d1538203d9586432c6a5b7ca5ede54` |
| `r103-legend-390.png` | `9f1155a67b9c1f10e015d2a058f2be808f7f5abb8d5bc0264f9e63344176268e` |
| `r103-legend-768.png` | `749f25b64885d23dbbb236d3128c792f76b927f7610e4ece5ddd61e3d47ce815` |
| `r103-legend-1280.png` | `9b887f990f07d339d9c300b71815265c914b0f150a3e72a510d4e804b36c74e9` |

**亲读 ≥2 张非登录页**：
- `r103-feed-768.png`（768×1024）：战斗页抽屉展开态，显示「AI · 完整反应时间线」标题、右上 `×` 关闭按钮、正文「本局暂无逐次反应记录」与依据行，视觉与 AX 快照一致。
- `r103-result-1280.png`（1280×800）：结果页抽屉展开态，`role=dialog` 的对话框与 `×` 关闭按钮可见，非登录页，视觉与 AX 一致。

## §4 改前基线对照

| 维度 | 改前（t268 recon） | 改后（本轮实测） |
|---|---|---|
| CTA 键盘属性 | `.r28-feed-cta` / `.r29-result-timeline-cta` `role=null`、`tabIndex=null`、`cursor=auto` | 两处 `role=button`、`tabIndex=0`、`cursor=pointer`，真 Tab 可达 |
| overlay 语义 | `role=null`、`aria-modal=null`、无 `aria-label` | `role=dialog`、`aria-modal=true`、`aria-label=完整反应时间线` |
| 关闭按钮 | 无 `role` / `tabIndex`，AX 无按钮 | `role=button`、`tabIndex=0`，AX `button "×"`，焦点自动落入 |
| ESC 关闭 | 不关闭 | `ov=false`，关闭 + 恢复暂停（焦点归还未坐实，见 §6） |
| Tab 陷阱 | 无 | `trapTimelineTab` 首尾循环，Tab/Shift+Tab×3 不逃逸 |

改前截图 sha 存于 `docs/laiyipao/audits/round102-a-timeline-close/screenshots/`（timeline-768=`60b53457…`、timeline-1280=`f78e11b3…`、timeline-open=`40d49f21…`），本轮未复用。

## §5 静态回归与门禁 G1–G7

门禁脚本 `data/gates.sh`（先取 ROOT 绝对路径，仓库根 workdir，`cd miniapp` 跑 vitest），job `bash-d36b9e8d6d2040bb` exit 0，输出 `data/gates-out.txt`：

| 门禁 | 命令 / 对象 | 结果 |
|---|---|---|
| G1 | `r103TimelineKeyboard.contract.test.ts` | 12 passed / 12，RC=0 |
| G2 | `r73LegendFocus.contract.test.ts`（图例不回退） | 6 passed / 6，RC=0 |
| G3 | `src/pages/battle/battle.test.ts` | 61 passed，RC=0 |
| G4 | 全量 vitest | 235 passed \| 1 skipped files / 2162 passed \| 2 skipped tests，RC=0 |
| G5 | `pnpm run type-check`（vue-tsc） | RC=0 |
| G6 | `pnpm run build:h5` | RC=0 |
| G7 | `git diff --check` | RC=0 |

静态回归（ops_bash 实测）：
- 令牌三锚点：`styles/tokens.css:14` `--lyp-token-version: lyp-tokens-v1`、`:15` `--lyp-r3-version: lyp-tokens-v3-r3`、`:16` `--lyp-r6-version: lyp-tokens-r6` 全部在。
- 业务零 `--r103-`：`miniapp/src` 下 NO_MATCH（PASS，唯二匹配在契约 it12 自身断言文本）。
- 四处改点属性：`battle.vue:77`/`:261`/`:265`/`:449` 全部在位。
- G3 测试 stderr 多条 `[来一炮] build.attacker 缺失，正在使用本地占位属性` 为测试既有告警，非本轮引入。

## §6 诚实边界

1. **后端未起、数据非真值**：8080 未启动，全部接口经 `data/r103-init.js` 合成；令牌 `r103-qa-placeholder-token` 为占位符，仅 `localStorage` 内存注入。截图与探针反映的是合成/注入态，不代表真实服务端战报。
2. **CTA 经 Vue 实例注入产生，非真实战斗流程产出**：合成战斗数据未能自然产生 `recentReactions`（引擎未产出 reaction 事件）与 `settleResult`（引擎未跑到结算）。依队长务实指导（m07480），经 `__vueParentComponent` 链找到 `setupState` 含 `openReactionTimeline` 的 `battle` 组件，注入 `recentReactions` 触发 feed CTA、注入 `settleResult` 触发结果页 CTA，再真实点击/按键开抽屉。**验证的是八处改点的静态属性与焦点/键盘逻辑，不是「真实战斗流程自然产出 CTA」**——后者未坐实。
3. **焦点归还 CTA 未坐实（D/E 部分）**：专项复核 `data/focus-return-out.txt` 显示——open 后 `active=r29-timeline-close`，ESC 或关闭按钮 Enter 之后 `activeElement` 落在 `BODY`（`cls=""`），**未回到触发 CTA**。源码 `battle.vue:1091-1094` 确有 `timelineTriggerRef.value → node.focus()` 归还逻辑、契约 it7 静态守住，但真浏览器未观察到焦点回到 CTA。推测原因：H5 uni-app `view` 的 `focus()` 对非原生可聚焦元素不生效，或 ESC 经 overlay `@keydown` 冒泡进入 `closeReactionTimeline` 时 `event.currentTarget` 为 overlay 而非 CTA，致 `timelineTriggerRef` 指向 overlay。**此项按未坐实披露，不冒充 PASS；开关抽屉、暂停恢复、Tab 陷阱等其余项已坐实。**

**追加旁证（真 Tab 路径，队长复核后轻量确认）**：为排除「注入路径伪影」可能，另起专项 `data/tab-focus-return.sh`（输出 `data/tab-focus-return-out.txt`）改用**真实键盘 Tab**（非 JS `element.focus()`）聚焦 feed CTA：Tab 后 `activeElement=r28-feed-cta`（tag `UNI-TEXT`）→ Enter 开抽屉（`active=r29-timeline-close`/`inOverlay=true`、`paused=true`）→ ESC 关闭后 `activeElement=BODY`（`cls=""`），与注入路径结果完全一致，坐实焦点归还落 BODY 与 CTA 数据注入无关。队长源码比对进一步定位根因：两处 CTA 用 `<text>`（`UNI-TEXT`），而 R73 `help-chip`（其焦点归还已坐实）用 `<view>`（`UNI-VIEW`），H5 uni-app 对 `text` 节点 `focus()` 不生效——属 H5 端 `text` 聚焦能力限制，非本轮注入伪影、亦非实现缺陷。源码归还逻辑（`battle.vue:1091-1094`）+ 契约 it7 静态守住，本项维持「真浏览器未坐实、按边界披露」口径，不冒充 PASS。
4. **非原生 Tab 陷阱**：`trapTimelineTab` 的手工循环针对 overlay 内 `[tabindex="0"],[role="button"]`，非 ARIA `aria-modal` 原生焦点隔离；实测 Tab/Shift+Tab×3 未逃逸，但依赖手工处理。
5. **截图亲读 2/9**：其余 7 张仅做 sha256sum 一致性核验，未逐张视觉亲读。
6. **QA 库本轮未连**：无 DSN 暴露，未做跨轮数据库断言。

## §7 结论与 hygiene

**结论**：R103 时间线抽屉键盘闭环在 390/768/1280 三视口完成真浏览器复查。两处 CTA 键盘三件套（`role=button`/`tabIndex=0`/Enter 同 handler）与真 Tab 命中坐实；抽屉 `role=dialog`/`aria-modal=true`/`aria-label` 与 AX 可感知坐实；焦点自动落关闭按钮、抽屉内 Tab/Shift+Tab×3 不逃逸坐实；ESC 与关闭按钮 Enter 均能关闭抽屉并恢复暂停、与图例互斥坐实；零横溢、令牌三锚点、零 `--r103-` 回归通过；门禁 G1–G7 全绿。**焦点归还到触发 CTA 一项真浏览器未坐实（落 BODY），源码逻辑 + 契约静态存在，按 §6.3 披露，不冒充 PASS。** 改前基线（CTA 无角色、overlay 无 dialog、ESC 不关闭、AX 无按钮）已修复。

**hygiene**：
- HEAD `91e776228925d40df75ac87af4e1d71c8c4662d3` 未变。
- `git status --porcelain` 非 `??` 项仅 `M .gitignore`、`M laiyipao/miniapp/src/pages/battle/battle.vue`（均为 t276 预期实施，非本 QA 写入）。
- auth 五路径全 ABSENT：`.env`、`laiyipao/admin/.env.local.keys`、`laiyipao/deploy/.local.keys`、`laiyipao/server/.local.keys`、`laiyipao/miniapp/.env.local`。
- 零真值：占位令牌仅内存注入；报告未写入任何密钥、真实域名/IP、个人路径。
- 走查 job `bash-3ba8b2910c414b84` 已 kill；自启 H5 job `bash-e9694095a94645e8` 待本轮末 kill 并复验 P5174=000。
