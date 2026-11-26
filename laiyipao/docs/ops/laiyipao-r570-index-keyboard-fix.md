# R570 laiyipao 启动首页 index.vue 9 类裸 @click 伪按钮键盘可达修复（WCAG 2.1 SC 2.1.1 A / 4.1.2 A）

## 一、结论与方案取舍

真缺陷 1 项 P2 已修复：启动首页 pages[0] 是游客 0 跳直达、全项目无 tabBar 的唯一枢纽，9 类裸 @click 伪按钮（重试/全部公告/公告项/空维补导/边际收益/主行动卡/玩法宫格/图鉴宫格/元素 chip）此前无 role/tabindex/aria、AX generic/name=null、焦点停 BODY、干净 Tab 30 次 0 命中，键盘用户被困首页。

方案 A（main.ts 全局 MutationObserver 增强器）Fail Loud 排除：① 越界他页——增强器对全项目所有 @click 元素生效，超出本任务写域；② 动态名不可靠派生——公告项取 a.title、宫格取 m.label、元素 chip 取 e.name、边际收益拼收益值均需模板上下文，全局 DOM 扫描无法保真；③ 与 R561/R562/R563/R564/R565/R567/R568 同族范式一致，逐页显式补可审计。故采方案 B：9 处开标签就地追加 a11y 属性，业务 handler 函数体零改。

## 二、写域与零改清单

- M laiyipao/miniapp/src/pages/index/index.vue（仅 9 处开标签纯 a11y 属性追加，git diff 仅属性行：26 insertions / 5 deletions）
- A laiyipao/miniapp/src/pages/R570.indexKeyboardA11y.contract.test.ts
- A laiyipao/docs/ops/laiyipao-r570-index-keyboard-fix.md（本文件）

零改：go/goAnnouncements/goStage/retry/toggleAnn 函数体、9 条 @click、api/client.ts、store/game.ts、样式、他页、main.ts、已有 a11y 契约、svg[aria-label=构筑五维雷达图] 与 svg[aria-label=闯关进度] 正面对照。契约守卫测试逐项钉死。

## 三、9 处修复表（纯属性追加，handler 复用）

| # | 锚点 | role | aria-label | keydown 复用 |
|---|------|------|------------|--------------|
| 1 | :40 retry | button | 静态「重试」 | retry |
| 2 | :62 annAll | link | 静态「查看全部公告」 | goAnnouncements |
| 3 | :68 annItem | button + aria-expanded | a.title + 展开/收起态 | toggleAnn(a.id) |
| 4 | :141 dimHint | link | 去补充 d.label：d.hintLabel | go(d.hint) |
| 5 | :162 mgRow | link | m.label + 收益 + hintLabel | m.hint && go(m.hint) |
| 6 | :186 actionCard | link | 开始第 store.unlockedLevel 关 | goStage |
| 7 | :227 playItem | link | m.label | go(m.path) |
| 8 | :240 codexItem | link | m.label | go(m.path) |
| 9 | :256 elemChip | link | 查看 e.name 相关反应 | go(reactions) |

每处均补 tabindex="0" 与 @keydown.enter / @keydown.space.prevent，原 @click 全保留。

## 四、契约 RED → GREEN 与三 verify

- 契约 12 用例 = 9 核心（blockFrom 起始锚点向下扫 trimEnd endsWith 收尾，适配多行开标签）+ 3 守卫（业务 handler/数据源/样式/他页/main/正面对照零改 + 9 @click 保留 + 无非法 tablist/tab/aria-selected）。
- 真父 514b87363 RED（git show 覆盖源码，sha=68608d9b…）：EXIT=1，9 failed | 3 passed（9 核心全 fail，3 守卫 pass）。
- 修复版 GREEN（sha=91d0052e…）：EXIT=0，12 passed。FINAL HEAD 未变（仍 514b87363）。
- verify1 定点 vitest：EXIT=0，1 file / 12 tests passed。
- verify2 全量 pnpm test：EXIT=0，162 files passed | 1 skipped（163），1572 tests passed | 2 skipped（1574）。
- verify3 pnpm run type-check（vue-tsc --noEmit）：EXIT=0，无类型错误。
- 字面命令在 laiyipao/miniapp 目录下执行；根目录 pnpm test 不适用本子项目。

## 五、真浏览器 CDP 决定性判据（390x844 DPR2 双主题）

自起 laiyipao/miniapp H5（dev:h5），Playwright chromium headless、isMobile/hasTouch；stub 仅拦 /api/v1/（config/announcements/wallet/me/stars/auth）；预置 lyp_token 跳过 guestLogin。6 组（light/dark × withData/empty/error）+ 2 功能探针组。

- 决定性判据：document.activeElement 命中目标且 isTarget=true、isBody=false、tag=UNI-TEXT/UNI-VIEW（非宿主 wrapper 假阳性，R535 教训）；getFullAXTree focused=true 非 RootWebArea 节点 role/name 与 getPartialAXTree 一致；DOM.getAttributes role/tabindex/aria-label 达标。
- 9 类 focused AX 全达标：annAll=link/查看全部公告、annItem=button/R570公告一，展开正文（aria-expanded=false）、dimHint=link/去补充元素覆盖：去背包配元素、mgRow=link/装备契合，收益260，去配装备、actionCard=link/开始第 2 关、playItem=link/关卡、codexItem=link/反应、elemChip=link/查看焰相关反应、retry=button/重试。
- Tab 可达：withData 30 次命中 annAll=1/annItem=3/dimHint=2/mgRow=3/actionCard=1/grid-item=15/elem-chip=4；error 态 retry=15；empty 态无关卡公告但补导/边际/行动卡/宫格全命中。
- Enter/Space 功能真命中：annItem Enter→展开态 annOpen0=true；annAll→#/pages/announcements/announcements；dimHint→#/pages/bag/bag；mgRow→#/pages/equipment/equipment；actionCard Enter 与 Space→#/pages/stage/stage?level=2；playItem→stage；codexItem/elemChip→reactions；retry Enter 与鼠标 click 均使错误态消失（retryExists=false，恢复首页）。非仅焦点移动。
- 注入对照 removed→restored：剥离 role/tabindex/aria 后 8 类 AX 退化为 generic/name=null、focused=[]；恢复后 role/name 与 Tab 命中全回，证明判据对源码属性真实敏感。
- 正面对照：svg[aria-label=构筑五维雷达图] AX role=image/name=构筑五维雷达图（withData/empty 均 count=1）；error 页无该 svg（count=0，符合预期，据实披露）。
- 鼠标零回归：retry 鼠标 click 恢复首页；fatals=0，pageErrors=0。

## 六、据实披露（Fail Loud）

1. RED replay 曾误把修复前备份当修复版恢复，导致源码回到真父；已重新施加 9 处修复并重建修复版备份（sha=91d0052e）后复核 GREEN，坐实真父 RED / 修复版 GREEN。
2. runner 两轮修正后 fatals 归零：① 首轮 error 态功能探针在 reset 后 cfgCalls 未清零致 retry 按钮不重现（dispatchEvent 超时 30s）；② 次轮 hash-only page.goto 为同文档导航不重置 Vue 状态（R558 教训）致 retry 仍不重现；改 reset 用 page.reload({waitUntil:domcontentloaded}) + waitForFunction polling:100 按 mode 等待目标控件后第三轮 fatals=0。
3. error 态无雷达图 svg 属该态预期（页面走 loginError 分支），正面对照仅在 withData/empty 采集。
4. 截图 light/dark 同 sha（H5 无主题机制，仅 localStorage 占位切换，视觉一致），据实披露；决定性判据在截图前采集，不依赖截图。截图 780x4462 目视正常：公告 3 条+雷达图+补导+边际 3 条+行动卡+玩法 9/图鉴 6 宫格+元素 5 chip。
5. 证据目录 oa4rust/test-results/r570/ 属根 .gitignore 已忽略，借用并据实披露，不入库。

## 七、SHA-256 证据表（runner 紧凑 JSON sha 与文件 sha256sum 分列）

| 证据 | runner 紧凑 JSON sha | 文件 sha256sum |
|------|----------------------|----------------|
| R570_DONE 汇总 | 7e74ff5ad45c078af6a68822cbd9a5f18b6880d096b99ad61065362244b06ee1 | fa5287d8889da2ff69e7de8eb20bd431fb49edacc62b9b7d83b6debe7634b0a4（/tmp/r570-e2e.log） |
| r570-e2e.cjs | — | 1da264366c13869cb255fa0300e282d2b635aeb1a20992c9371a2bd32cd41cf2 |
| r570-report.json | — | 1388e77cf6b39e9702c8f0394471d5809b8b80890fd533bbddc2981703f0fbed |
| r570-withData-light.png | — | 12f593987d3ec3f4a154a54f3f26029b801167f6043325ef1d3241bfbfc956ce |
| r570-withData-dark.png | — | 12f593987d3ec3f4a154a54f3f26029b801167f6043325ef1d3241bfbfc956ce |
| r570-empty-light.png | — | a47de0b2c5309c0fb9872414228759f1a8d24423105363d226ce35a780a1b8e5 |
| r570-empty-dark.png | — | 8751e87d51ff71517927c7dac8814eba1de4b7104299c9bfdedc9aa05bcace28 |
| r570-error-light.png | — | 87ba7c000dd97c07cf1587241e6cefba7ebf37787d202bae1a2c6f8e76a0d9bc |
| r570-error-dark.png | — | 87ba7c000dd97c07cf1587241e6cefba7ebf37787d202bae1a2c6f8e76a0d9bc |
| index.vue 真父 | — | 68608d9b8b745210c9aabe2652ee2cb7dc13fe8e0f2805acec52e82e67463107 |
| index.vue 修复版 | — | 91d0052edd24f86bd6e6e88387215ddeb2824828367f5e6b316d7de72b8f9a36 |
| R570 契约 | — | 543897f50edac8e429da456dc348f69ac683115d7846acbfe6aa300a00ffa5cc |
| /tmp/r570-red.log | — | da05eed0d7ddf4559a9f427b1dc96ea5606a0dce608e7071895a65e7b842be49 |
| /tmp/r570-green.log | — | a02a2acfdd09259e0b0f6b60a0e1de9bf299110fc9b815a7c6e442d7ac9a3b49 |
| /tmp/r570-v1.log | — | 51defe1b397a7be9ebde78ab3e91551904cd03b48a44ab71668356b1c8956e63 |
| /tmp/r570-v2.log | — | 3e611471376e58b2caccd88a367bdd84f3c062c57b72ecdccbea09159f11d5fa |
| /tmp/r570-v3.log | — | 37fb02605f763c6cdb22885730991887ea6c34bf242808be4857988263d5f84f |

## 八、脱敏

测试基址仅以 <TEST_HOST>:<PORT> 占位，全文零真值（无域名/IP/端口/令牌/个人路径）；E2E 证据目录 gitignored 未 track。
