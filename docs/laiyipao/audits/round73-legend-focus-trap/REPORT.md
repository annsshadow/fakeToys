# R73 战场图例焦点陷阱与键盘可达性 — 真浏览器 QA 报告

- 任务：t159 / attempt 18efeab1-ba5a-438c-8a46-434f98323768
- 轮次基线：HEAD `def111dd8`；修复写域仅 `laiyipao/miniapp/src/pages/battle/battle.vue`（+ `.gitignore` 忽略项）。
- 验证方式：agent-browser 0.41.1 真 Chromium，三视口 390×844 / 768×1024 / 1280×800，真实点击 + 真实键盘（press Enter / Escape）。
- 口径：零真值入库；业务源码只读；探针/截图落 `docs/laiyipao/audits/round73-legend-focus-trap/`（data/ 命中 .gitignore:592）。

## 一、修复源码只读核实（完整落地）

- `battle.vue:112-120` 帮助 chip：`:class="{ on: helpVisible, 'help-first': !helpSeen }"`、`role="button"`、`tabindex="0"`、`aria-label="战场图例"`、`@click="openHelp"`、`@keydown.enter="openHelp"`。
- `battle.vue:177-185` 图例弹层：`v-if="helpVisible"`、`role="dialog"`、`aria-modal="true"`、`aria-label="战场图例"`、`@click="closeHelp"`、`@keydown.esc="closeHelp"`、`@keydown.tab="trapLegendTab"`；关闭按钮 `legend-close` 为 `role="button" tabindex="0" @keydown.enter="closeHelp"`。
- `battle.vue:980-1007` `openHelp()`：先 `closeReactionTimeline()` 互斥 → `helpVisible=true` → 未暂停且未结算时 `doPause()` 并记 `helpPausedByPage=true` → `helpSeen` 记忆 → `nextTick()` 内 `legendCloseRef` 聚焦。
- `battle.vue:1011-1017` `closeHelp()`：`helpVisible=false` → 若 `helpPausedByPage` 则 `doResume()` → 焦点归还 `helpChipRef`。
- `battle.vue:1020-1035` `trapLegendTab(e)`：`overlay.querySelectorAll('[tabindex="0"],[role="button"]')` 取首尾，Shift+Tab 在首/外部 → 末元素聚焦并 preventDefault；Tab 在末/外部 → 首元素聚焦并 preventDefault。
- R29 抽屉：`openReactionTimeline()`（约 :1040 起）第一步即 `closeHelp()` 保证与图例互斥。

## 二、真浏览器环境

- API：本地回环 127.0.0.1:8080（`/api/v1/config` = 200，version 占位哈希）；静态 H5：127.0.0.1:5201（build:h5 产物）；旧端口 5174 已弃用。
- 登录：本地回环 guest token 注入脚本生成于 gitignored data/，验证期间存在，收官删除。
- 打开方式：`open`（空页）→ `set viewport` → `navigate http://127.0.0.1:5201/#/pages/battle/battle?level=1`，自动开局，走真实 DOM 与真实键盘。
- 探针口径：`hOverflow = document.documentElement.scrollWidth <= document.documentElement.clientWidth + 1`，即 **true 表示无横向溢出**（命名沿用历史，勿反读）。

## 三、三视口核心链路（click 路径，探针 data/r73-{390,768,1280}-core.txt）

| 视口 | 阶段 | 焦点元素 | 弹层 | role / aria-modal | 暂停 | 横向溢出 |
| --- | --- | --- | --- | --- | --- | --- |
| 390 / 768 / 1280 | 打开前 | BODY | 关 | null / null | false | 无 |
| 390 / 768 / 1280 | 点 help-chip 后 | legend-close（UNI-VIEW，文本 ×，在弹层内） | 开 | dialog / true | true | 无（sw==cw） |
| 390 / 768 / 1280 | ESC 后 | help-chip（文本 ?，弹层外） | 关 | null / null | false | 无 |

- 打开后 chip 仍为 `role=button`、`tabindex=0`；关闭按钮同为 `role=button`、`tabindex=0`。
- 基线对照（修复前）：焦点停 BODY、Tab 横跳背景、ESC 后弹层仍开且 role=null；本轮全部修复。

## 四、Tab / Shift+Tab 焦点陷阱（data/r73-{V}-tab.txt / -shifttab.txt）

- 三视口各 6 次 Tab：每一步 `activeCls=legend-close`、`activeInOverlay=true`、`overlayOpen=true`，焦点从未逃出弹层。
- 三视口各 6 次 Shift+Tab：同上，焦点恒留弹层内。
- 全过程 `paused=true`、`role=dialog` 稳定。

## 五、真实键盘 Enter 与手动暂停分支（data/real-out.txt）

- `focus chip` + `press Enter` → 弹层开：焦点 legend-close、overlayOpen=true、role=dialog、aria-modal=true、paused=true。
- `focus close` + `press Enter` → 弹层关：焦点归还 help-chip、overlayOpen=false、paused=false（自动恢复）。
- 手动暂停分支：先点暂停按钮 → paused=true；再 `focus chip` + Enter 开图例 → paused=true；`press Escape` 关图例 → **paused 仍为 true**，手动暂停未被误恢复。此分支坐实 `helpPausedByPage` 记账正确。
- 结论：click 与 Enter 双通道、开暂停/关自动继续、手动暂停不误恢复，均通过真键盘验证。

## 六、R29 抽屉互斥（data/mutex-out.txt）

- 探针确认 `.r28-feed-cta` 存在（feedCta=true）。
- 先开图例（overlayOpen=true、焦点 legend-close、paused=true），再点 `.r28-feed-cta`：图例 `overlayOpen` 由 true→false、焦点归还 help-chip，`paused` 保持 true（抽屉路径代为暂停）。
- 判定：图例被关闭，互斥核心断言坐实。**诚实边界**：本轮探针未捕获到抽屉展开态 DOM（afterCta `drawerOpen=false`），故「抽屉已展开」不予宣称；`openReactionTimeline()` 首行 `closeHelp()` 的互斥语义由图例关闭得到真浏览器证据，抽屉内部展示另由静态源码与既有 R29 契约覆盖。

## 七、截图 sha256（报告与实测逐字一致）

| 截图 | sha256 |
| --- | --- |
| r73-390-open.png | 3d9c053fb55ee031912ca489c4952d2b099df1934717ed2fa1eadd3d79fa5876 |
| r73-390-closed.png | ebe1606af79854e2212b080d3faf502864f2515c5f4e6f6eeff493b33c8e144e |
| r73-768-open.png | 0093c0eb470ccf29990c38e0cb9c2e2aace6e72d21b5f111bb47ec045c7db4f6 |
| r73-768-closed.png | bea370f679dc229024ef4e8dc3a435bd626214f27315093aa0c82bd4bce73482 |
| r73-1280-open.png | 61b386cb85e69294ca5d781f0cb2607dcc60dc0b6cd50742eb55186927eebd6e |
| r73-1280-closed.png | 752421e255f12ccd2e5cd32b2968f8937a3b8ff8dffd5d2be401510eecc57521 |
| trial-open-768.png | dcdd3a5db4c1d279e11ea5f4cd614863e15f4107436059088d3ab21f290d6580 |

- UNIQUE=7/7；`trial-open-768.png` 为首轮 trial 的额外证据，非三视口正式组。
- 亲读 2 张：`r73-768-open.png` 为「战场图例」弹层（标题 + 关闭 × + 热量条/技能槽说明），`r73-1280-closed.png` 为战场主界面（波次/技能/AI 诊断），均非登录页。其余 5 张仅列 sha，未逐张亲读（如实标注）。

## 八、回归与门禁

- 静态回归（OPS grep）：令牌三锚点 `styles/tokens.css:14/15/16` 逐字在；`role=button` / `tabindex="0"` / `@keydown.enter` 计数均为 48（battle.vue 图例贡献 :117/:118/:120/:185/:1030）；R71 `fmtVerifyTime`/`pretty`/`buildSnapshotCard`、R72 `search-empty-text`、R66 `normalizeGemGain`、R69 `nicknameSubmitting` 均在；业务源码 `--r73-` 零命中（仅契约断言 r73LegendFocus.contract.test.ts:99）。
- 前端门禁（data/gates-out.txt，整体 exit 0）：G1 契约 r73LegendFocus 6/6；G2 battle.test 61/61；G3 全量 224 files passed | 1 skipped（225）、2055 tests passed | 2 skipped（2057）；G4 vue-tsc RC0；G5 build:h5 RC0；G6 git diff --check RC0（仅 .gitignore LF→CRLF 提示）。
- Go 门禁（data/go-gates-out.txt，exit 0）：`TestNoInfraIdentifiersInRepo` **PASS**（8.01s）；`TestNoRealSecretsInRepo` **SKIP 非 PASS**，逐字：「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」。

## 九、诚实边界

- 验证对象为本地回环静态 H5 的真实战斗页（`?level=1` 自动开局），非生产/远端环境；键盘事件由真实浏览器 press 产生，非合成 dispatch（首轮 `dispatchEvent(KeyboardEvent)` 未触发 Vue `.enter`，已作废并以真实 press 重跑）。
- hOverflow 探针 true=无溢出；`drawerOpen` 未捕获展开态已如实标注。
- 截图 7 张中亲读 2 张；其余仅 sha。
- 令牌值与真实部署地址未写入本目录；QA 库仅只读查询。

## 十、结论

**PASS**：三视口 × click/真实键盘双通道，焦点陷阱、打开聚焦、ESC 归还、开暂停/关恢复、手动暂停不误恢复、R29 互斥核心断言全部坐实；基线缺陷全部修复；回归与门禁全绿（TestNoRealSecrets 如实记 SKIP 非 PASS）。抽屉展开态未捕获为已知边界，不影响本轮焦点陷阱与键盘可达性判据。
