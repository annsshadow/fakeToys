# R109 AI 优先级 CTA 键盘可达性 部署后复查报告

## 1. 范围与方法

- 对象：R109 在 9 页的 10 处 AI 优先级 CTA（`.r12-priority-cta` 4 处 + `.r3-ai-cta` view 版 6 处）补键盘三件套后的生产构建复查。
- 视口：414×896、820×1180、1440×900 三视口，真 Chromium（agent-browser 0.41.1）。
- 产物：H5 重建 bundle `index-BOXIvyW1.js`（index.html 引用 assets/index-BOXIvyW1.js），sha256 `ba28e852fcad6d1bec6b60530ad45f4ad8b6beafefbc4ae1d1f46bca52ef038d`。后端/admin 沿用不回退。
- 机制：生产构建直接注入成功——复用 QA 的 init-script（data/r109-init.js，网络层 XHR.open/send 双 patch + fetch patch + localStorage 配置/占位令牌缓存）合成 config/wallet/me/战报/排行榜等响应，使 10 处 CTA 在生产构建自然渲染（不依赖 Vue 实例 setupState 注入，无需 dev server）。占位令牌 `r109-qa-placeholder-token` exp=2099，零真值。后端 8080 未起，全部为合成响应。
- 采集：每页 open(init-script) → 重设 viewport → DOM 探针（role/tabindex/tabIndexProp/cw/sw/hOverflow）→ 真 Tab 10 次读 focusin 日志与橙色焦点环 → stub uni.navigateTo/pageScrollTo 后 click 与 Enter 对比 → 截图。逐视口原始数据在 data/pd-w-{414,820,1440}.txt。

## 2. 三视口逐页证据（10 处目标 CTA 全绿）

下表结论在 414/820/1440 三视口完全一致（每页 DOM 探针 role=button / tabindex="0" / tabIndexProp=0，真 Tab 命中目标 CTA，click 与 Enter 命中同一 CTA；自指分支 nav=[] 为页内定位）：

| 页面 | 目标 CTA | role / tabindex | 真Tab命中 | click vs Enter | 零横溢 |
|---|---|---|---|---|---|
| gems | 1×r12 + 2×r3(text) | button / 0 / 0 | 是 | 同 .r12-priority-cta | iw=cw=sw=W |
| skills | 1×r12 | button / 0 / 0 | 是 | 同 .r12-priority-cta | 是 |
| equipment | 1×r12 | button / 0 / 0 | 是 | 同 .r12-priority-cta（自指页内强化） | 是 |
| bag | 1×r12 + 1×r3(view) | 均 button / 0 / 0 | 是 | 同 .r12-priority-cta | 是 |
| me | 1×r3(view) | button / 0 / 0 | 是 | 同 .r3-ai-cta | 是 |
| battles | 1×r3(view) | button / 0 / 0 | 是 | 同 .r3-ai-cta（自指展开战报） | 是 |
| enemies | 1×r3(view) | button / 0 / 0 | 是 | 同 .r3-ai-cta（自指切换元素） | 是 |
| rank | 1×r3(view) | button / 0 / 0 | 是 | 同 .r3-ai-cta | 是 |
| skins | 1×r3(view) | button / 0 / 0 | 是 | 同 .r3-ai-cta（自指筛选） | 是 |

- 九页共 27 组（9 页 × 3 视口）全部 isLogin=false、hOverflow=false、cw=sw=视口宽。gems 页 focusin 日志 8 次中 6 次命中 CTA（role=button / tab=0），真 Tab 序列交替命中 r12/r3 CTA 与 INPUT。
- Enter 事件 keys 中 isCta=true 坐实键盘激活落在目标 CTA（如 gems 414：两条 {"key":"Enter","isCta":true,"cls":"r12-priority-cta"}）。click 与 Enter 均命中同一 class。

## 3. 焦点环与 AX 快照

- 焦点环：真 Tab 聚焦目标 CTA 时橙色焦点环 active=true，414/1440 为 solid 1px rgb(255, 107, 53) offset=1px，820 为 solid 3px rgb(255, 107, 53) offset=2px（3rpx 随视口 DPR 换算，颜色一致 --accent）；common.css 两层 :focus-visible 未回退。
- AX snapshot（gems 页三视口全量）坐实 - button "去配装备 →"、- button "去商城找宝石 →"、- button "去背包镶嵌 →"，改前基线为 StaticText（引用 ROUND109_A 侦察，未重做改前 A/B）。其余 8 页以 DOM role/tabindex/tabIndexProp + focusin 焦点日志坐实。

## 4. 门禁与 Go 守卫

- QA 独立复跑 G1-G6 全 RC=0：契约 16/16、五 a11y 契约 48/48、全量 vitest（240 files/2225 tests，2 skip）、vue-tsc、build:h5、git diff --check。本轮 postdeploy 独立复核 H5 重建 RC=0。
- Go 守卫：TestNoInfraIdentifiersInRepo PASS 8.22s(cached)；TestNoRealSecretsInRepo SKIP 非 PASS（secrets_leak_test.go:182 逐字「读不到 .local.keys —— 真值扫描这条守卫没有执行，不是通过了」），GO_RC=0。

## 5. 截图 sha256sum（27 张落盘实物实测，UNIQUE 27/27）

| 页面 | 414 | 820 | 1440 |
|---|---|---|---|
| bag | 76cc3a37b5dcc1010642d6a65548f0dfbea5d850669e0e1f7505cc704cd73a37 | d159ec9132c8ce89d6b5c8c510f87d70e3ca3079d564061bc8210934a8a65ad6 | dc61d446d31d24d9e4191107d026176f47f5cf99a959672b5190bb6236d092bf |
| battles | dc80c055b900df6e6df6de328860112c601af5e51b36ba120361bc9e293645ca | c49ba71af9511f2ea6bb894554660d47fa853024849eca8bdb9656f5745cd920 | 6492b02eb0d5c5412f93d9dc6f880f44d24f5b98fcfce7fa5329a22a2c274806 |
| enemies | 79bf5a4ca8ff7d0c725a1a609f88c6b1f53ca106a7bd71e8ad3f6becda854f14 | 4558bf261b6bf0bb0ddcfb0aa94515d55839899311a799d800dc2a2aa3cf130d | 6bdf92a6d0093d1fb393cb5a1e532ffa1886c75aa65e1971bf38d69be14f97e8 |
| equipment | 426a30bc23a3e80b329e977735ea40a5054f9b65f77a880e91df101bcb8bcc91 | fdc607f99901736dc1627d0d1af19bca0f6511bdbdd95ee9e3dde4fcbb1fb207 | 076bbe5c57fcbcbcee88322de480f2765a3792fc266e3d5d78a8bd93b37337cd |
| gems | 34c6cbfe5b505650c49894c2351cba84dd06b97e8128a601ee274d4b75ad052c | dcf32c336363558e9b104874d0c10fa6e2355b96d48875360940d5aa683f931e | fbc240dd873007f9f5812d061d83479dbd132c5e9aeef81def402336b8feaacc |
| me | c445c29bec99dfbb311aa9825f8cdfbc044b61b92e18c9176fb6ecd28a68d9c4 | 7a5b5f56b8291374643bf501d915cd8f17c7ae77bc4e07fb905bcb96f8f3790a | c0b55c062e7c83715cac152f3a0edbab416074a3e3c53b24f8553c390bc417ca |
| rank | 7cb88eb103bc79d711657a703d2ac95190cc3bc0824ee04a0dbf0d441a9c1dbb | 5fe1f6d66f97627b23c31d62e1c97fba22b3426f707eb2591d462f286ef0207d | 67a2577b6dee83fa357dd21c3a9cb405b3fa340fdbe351614313b0f878fa447f |
| skills | ae49a6f550f725179ed13da328f45425b21e78887cffba04d77ff2c75dde876e | 8adf2a83d12b4c2df9b85efaf157edd5327f183c222238c433022614c8f0b089 | 605f3f67fb71fb5dd9e1b67fdd702df740ba371dda4e1f077ee4d8886910082b |
| skins | dcf360fce0947cce8ba0510ce1f3b058fa5bee66c49cee6dd0c3485f0abb1604 | 02d95a36bce1a1f3104431901670d56e9e0700c5535db5c3585504f06ed11cb2 | 2d6910f49be6c4e80a6b4b299ff9c39d45effc992db44d44aefdff3e3ec754e4 |

文件名格式 pd109-<page>-<W>.png，存放 screenshots/。UNIQUE 27/27，与 QA 27 图 sha 交集 = 0。亲读 3 张非登录页：pd109-gems-414（宝石图鉴 AI 优先级 CTA 去配装备 + AI 因果链）、pd109-bag-820（背包 AI 优先级 CTA 去配装备 + AI 构筑诊断去技能槽）、pd109-battles-1440（我的战报战绩总账 + 看战报详情 CTA），视觉与 DOM 探针一致。

## 6. 诚实边界

1. 后端 8080 未起，全部响应由 init-script 合成（含 skills/enemies/skins/leaderboard/rank/battles 数据依赖），占位令牌 r109-qa-placeholder-token 仅内存/localStorage 注入，零真值；非真实后端返回。
2. 截图首轮 driver 把 screenshot 放在 click/Enter（触发导航或页内跳转）之后，部分视口拍到导航后的装备图鉴页而非目标页，820 视口 4 页 sha 完全相同（8ea5be09…）。发现后重写 shots-driver.sh，每页 open 后 sleep 2 立即截图（在 click/Enter 之前）重拍 27 张，重拍后 UNIQUE 27/27 且与 QA 交集 0，亲读 3 张确认为目标页。本报告 §5 sha 为重拍后实物。
3. click/Enter 同 handler 通过 stub 拦截 navigateTo/pageScrollTo 记录 URL 验证，nav=[] 含自指分支页内定位；未做跳转后的 DOM 断言。
4. AX snapshot 仅 gems 页三视口全量抓取，其余 8 页以 DOM role/tabindex/tabIndexProp + focusin 焦点日志坐实。
5. 焦点环线宽 414/1440 为 1px、820 为 3px，系 3rpx 随视口 DPR 换算，颜色一致，非缺陷。
6. 改前基线引用 docs/laiyipao/recon/ROUND109_A_PRIORITY_CTA_KEYBOARD_RECON.md（改前 role=null/tabIndex=-1/AX StaticText），本轮未重做改前 A/B。
7. Go TestNoRealSecretsInRepo 因 .local.keys 不存在 SKIP（设计如此），非真值扫描通过。

## 7. 结论与 hygiene

- R109 10 处 AI 优先级 CTA 键盘可达性在生产构建三视口真 Chromium 全部坐实：role=button / tabindex=0 / 真 Tab 命中 / Enter 与 click 同 CTA / 橙色焦点环；改前 StaticText 与跳过缺口全部修复。
- 27 图 sha 实测 UNIQUE 27/27 且全不等于 QA 27 图；门禁全绿；令牌三锚点、零 --r109-、既有不变量（G2 五契约 48/48）、零横溢保持。
- hygiene：起点 HEAD=652e237fbdf5ce38460f1f159e5b48a6ec9c7646；五批提交见 git log；auth 五路径全 ABSENT；.gitignore=685 行（R109 三规则 683-685）；tracked 本轮 postdeploy/data=0；零真值；自启 job 全部回收。
判定：PASS（含上述诚实边界）。