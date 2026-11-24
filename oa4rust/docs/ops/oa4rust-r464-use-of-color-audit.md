# R464 WCAG 1.4.1 Use of Color 颜色独立性甄别（t323）

> 审计类别：只读甄别（不改任何入库源码/契约/台账）；唯一交付为本文档。
> 真父提交：e87cf50d847e7d9614011ad955e4ae7b6284f28d。
> 分支：opt1000/oa4rust-pc-ai-refactor；审计时 HEAD=UP=e87cf50d，AB=0 0，工作树干净。

## 1. 审计目标与空白维度确认

WCAG 1.4.1（Use of Color）要求：颜色不能作为传达信息、指示动作、提示状态或区分视觉元素的唯一手段。判据：每个承载语义的颜色状态必须同时具备文字标签、图标、形状、下划线或其他非颜色线索。

队长 m02412 静态核实：全库无专门针对 WCAG 1.4.1 的测试/契约，属真实空白维度；状态徽章语义色命中 133 处。已甄别维度严禁重复：motion/focus/44px/响应式/键盘焦点/骨架（R453–R456）/对比度 R458/forced-colors R461/prefers-contrast R462/文字缩放 R463。既有 nonTextContrast 属 1.4.11、formErrorA11y 属表单语义、requiredA11y 属必填，均不构成本维度覆盖，故本轮独立成立。

三分判定口径：规范预期（语义色伴随文字/图标，颜色仅增强 → 非缺陷）、真实退化（纯色点无文字无图标、灰度下不可辨 → 可达真缺陷并标 P 级）、不可达 L1。0 缺陷须据实判 0，严禁臆造。

## 2. 真浏览器实测方法

- 引擎：Playwright Chromium headless，非物理真机（NOT a physical device）。
- PC：1440x900 DPR1，双主题 light+dark，8 路由 × 2 视觉态 = 16 组；主题通过 localStorage[oa4rust_theme] + html data-theme 切换。
- H5：390x844 DPR2 isMobile/hasTouch，锁浅色，5 路由 × 2 视觉态 = 10 组（其中 5 组入证据，5 组索引同态）。
- 视觉态：normal 与 CDP Emulation.setEmulatedVisionDeficiency(achromatopsia 全色盲灰度)，另脚本支持 deuteranopia；在浏览器渲染层去掉颜色信息，模拟 WCAG 1.4.1 的去色判据。
- 状态采集：.badge/.status/.status-badge/[class*="status-"]/[class*="badge"]/.type-tag/[class*="required"]/.tag/.pill/.chip，逐元素采 text/textLen/color/bgColor/bgImage/border/radius/fontWeight/underline/outline/hasIconChild/beforeContent/ariaLabel/title/dataTestid。
- CDP：先 DOM.enable + CSS.enable 再 CSS.getMatchedStylesForNode，取状态元素实际命中规则。
- 登录与凭据：PC 用 input[placeholder*="用户名"] + getByRole(/登/)；H5 用 input[type=text]/[type=password] + uni-button filter(/登/)；凭据经 ssh 在测试服运行时从测试服凭据文件提取并注入环境变量，域名真值仅存在于 gitignored runner，不入库。
- 测试服：<TEST_HOST>，systemctl active，http 回环端口 HTTP=200，入口构建产物与本地 dist 一致。
- 实测汇总：R464_DONE pc=16 h5=5 states=46 errors=0，ERRORS=[]。

证据目录（全部 gitignored）：oa4rust/test-results/r464-use-of-color/，含 42 张 PNG + report.json，source 标注 Playwright Chromium headless Emulation.setEmulatedVisionDeficiency(deuteranopia/achromatopsia) 非物理真机。

## 3. 静态侦察：语义色消费页与非颜色线索

三处主状态徽章全部以模板插值渲染文字，颜色类仅作为附加 class：

- AttendanceApp.vue:165  span.badge :class=statusClass(r.status) 内为 statusTxt(r.status)；:188 申诉状态同理 appealClass/appealStatus。:403-415 statusTxt 返回「正常/迟到/—」，appealStatus 返回「已通过/已驳回/待审批/—」；:1870-1872 .badge.normal/.approved 绿（--color-success-glow/success）、.badge.late/.pending 黄（warning）、.badge.rejected 红（error）。
- BamApp.vue:63  td 内 span.status :class=statusCls(e.status) 直接渲染 e.status 文字；:674-677 .status.active 青/.done 绿/.failed 红/.error 红。
- ThreeMemberApp.vue:27  span.status-badge :class=statusClass(item) data-testid=three-status-badge 内为 statusLabel(item)；:233 基线、:235-237 .status-active/.disabled/.pending。

纯色状态点候选（初看像仅颜色，逐一读上下文后均判非缺陷）：

- IMChat.vue:53  presence-dot 后紧跟「{{ onlineCount }} 人在线」文字；:49-51 chat-status 已带「在线/离线」文字，:1227 .chat-status.online 只是附加绿色。
- ProcessDesigner.vue:488-489 ps-dot 后紧跟「✓ 流程有效 / 空流程 / ⚠ 需要修复」文字（自带 ✓/⚠ 形状符号）；:1542 bt-dot 后 :1543 紧跟「分支ID: 详情」文字；:1775 hp-ind-dot 旁 :1774 为指标名+数值；:686 retry-dot 使用 ●/○ 形状字符区分。
- XformRuntime.vue:13 required 星号 aria-hidden，同时 :20/:29/:38/:55 有 :required / :aria-required 语义，不依赖颜色。

结论：静态层面未发现任何仅颜色承载状态的候选；颜色均为文字/图标/形状之上的增强。

## 4. 逐状态证据（46 项）

分析脚本 r464-analyze.cjs 输出：TOTAL states=46 withText=46 noText=0 withIconOrBefore=0。状态文案去重为「— | 已启用」——测试服业务数据稀疏，多数状态显示占位「—」，但每一项均带可见文字标签。

分组明细：dashboard 0/0；attendance 1/1（文案「—」）；bam 0/0（本次数据未渲染状态行）；three-member 22/22（文案「已启用」1 项 + 「—」21 项）；org/file/bbs/meeting 0/0；H5 五页 0/0。

关键 light 态实例：

- attendance badge：text=「—」，cls=badge，bgColor=rgba(0,0,0,0)，color=rgb(15,23,42) —— 状态由文字承载，颜色未单独编码。
- three-member「已启用」：cls=status-badge status-active，bgColor=rgba(37,99,235,0.05)，color=rgb(37,99,235) —— 蓝色为增强，文字「已启用」独立可读。
- three-member 21 项「—」：cls=status-pending，与 active 同色系但文字不同，去色后仍可依文字识别。

## 5. 灰度对照与 CDP 根因

灰度对照（achromatopsia 全色盲模拟，1440x900）：pc-attendance-light-normal 与 pc-attendance-light-gray、pc-three-member-light-normal 与 pc-three-member-light-gray 成对目视。灰度下状态文字「已启用」「—」仍清晰可读，颜色差异消失但信息由文字承载，布局完整无崩坏；attendance 彩色统计数字（1 蓝/0 绿/0 橙/0 紫）在灰度图变灰，但下方文字标签「应出勤/实际出勤/迟到/请假」始终可辨。

CDP 匹配规则坐实颜色仅为附加装饰：

- pc-attendance：.badge[data-v-7c505ee1] 命中 border-radius:8px，无隐藏文字/无仅颜色伪元素承载状态。
- pc-three-member：.status-badge[data-v-bfac699e] 命中 background:var(--bg-elevated)、color:var(--text-secondary)、border-radius:999px；.status-active[data-v-bfac699e] 命中 background:var(--color-primary-soft)、color:var(--color-primary)。文字由模板插值独立渲染，颜色规则仅修改 background/color。
- pc-bam：空（本次数据未渲染状态行，据实记录，不臆造）。

## 6. 三分判定

- 可达真缺陷：0。46/46 状态元素均带文字标签；所有纯色状态点候选（IMChat presence-dot、ProcessDesigner ps-dot/bt-dot/hp-ind-dot）都伴随相邻文字或形状符号，灰度下信息不丢失。
- 不可达 L1：0。
- 正常-规范预期：若干。颜色仅作增强，文字标签为主线索；去色后信息完整保留，符合 WCAG 1.4.1。

据实判 0 缺陷，不臆造、不为凑缺陷降低判据。本次未新增修复任务；后续轮次若出现无文字纯色状态点，按本判据自测即可。

## 7. 截图 sha256（命令行 sha256sum 原始值，42 张）

来源：Playwright Chromium headless，Emulation.setEmulatedVisionDeficiency(deuteranopia/achromatopsia)，非物理真机；奇偶为 normal/gray 对照。

079bf353d1c7e84f09ed470a7c7e14bf1cad447916b781aeb955e3d188dc5456  h5-index-gray.png
0dd77c4a8929b970a994ef9ddb0b4be91e535f11991407a6cb79eb82e72922a7  pc-attendance-dark-gray.png
16a2d7b26956dd45a5df9f5ef91a2a1c74d05ba34f1a125196d9f1b2ea543778  h5-attendance-gray.png
184cbadb7f65caf4c71983aff74f9a1828f14be88cb1978f8661d1dedcda581f  pc-dashboard-dark-normal.png
1eabc8c751a64eca7ad1304ab6def4bdef87e3579c976255fde4d188b313b2d7  pc-meeting-light-gray.png
1f20eed881fd07c40745a0d7f5b1bc5f6f503c51089eb870506f19d0b942006c  h5-process-gray.png
20d84cfa6f0672db4606204932ad5d51f556f315a7c83036715b5802f29af45d  pc-org-light-gray.png
2a12c2b0649cfad3e9f115fd3c135c6f5436eed94df710750f87da9f45ad3c5f  pc-three-member-light-normal.png
2ae9d177d8de40dda6e27821b16d501a1293defb66c1cbf14bb9ba40c596d496  h5-attendance-normal.png
3d31d68884f1a929904e46f0b0b3419761a416df13bae2724c5c48f87c3ca4f3  pc-attendance-light-gray.png
497d1deb739931298caa2a5332264083cde28ddf46f4417d48409aa29ac4b251  pc-bbs-light-normal.png
4bc892130f13db396faeffaadf047f5a1e5bfc569450b7e29070a596490bdc9a  pc-bam-light-gray.png
4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf  pc-file-light-normal.png
53f61850ceea5caeadfbcb35e6ad0af2299d319311149a148482ed91b88c2bef  pc-file-light-gray.png
5522920fc73c02adfbe1f78a1dc7067076b11874354102e4de8b3b15187c9223  h5-meeting-gray.png
5a05e70b89960f634b47f99d91ce1f204ccce4e4af65f742a7dd035a48b86343  pc-bbs-light-gray.png
5b8ef288cb5639625926087383ce63dde4999b796fa7bd5bb5511075ede10310  pc-dashboard-light-gray.png
5faf382dd5c4c9f3205155fe393338d87177ed09e6ec06a515aab01c9d7ab51c  h5-meeting-normal.png
666a73ddf9d070807f581f77694912e39848e1e8f20673e9673f244cc620f8b1  pc-dashboard-dark-gray.png
6bd6fc06c3bf13ca2621cdbe74b8e30f48d5c23b442c218bc668f3e4fa51baa2  pc-bbs-dark-gray.png
77d20bbbe7b86503dc5bfe6b54f710a0d55fced4a8cd93c90dbd4654a20563ce  pc-bbs-dark-normal.png
803f39ce719360f5e95c925e22c1895bb4fb8669066a947f8885f56c0caed8e4  pc-bam-dark-normal.png
82709fb708363d64384451470e2fcbb6086f5be6807223daa5486483ddd0c9d9  pc-three-member-light-gray.png
887da0a8cbdb5e02ad1b6aebbbaa7b849f087bd14426b2a46b6ef05040f11738  pc-org-dark-normal.png
893879034acfe63734763a09a5a880a81c0a2df9a58380b1066bbc710a59f9eb  pc-dashboard-light-normal.png
94f4985914fc20808246a883da8c61e2039f00414e8696cbcdc9b2096adaf47c  pc-file-dark-normal.png
9a5f4cff1bd6328806e387dcca7bc2296d28aaf9b68237404270b102b615fc3a  pc-three-member-dark-normal.png
9cf60092816c044d1d50315662df9cbb8730e8c97322fecc306babe12a9f4f97  pc-attendance-dark-normal.png
9d3baca0c770dbaf8bcebb3f13bcb6792de20423fd18e3b0b3408fafa2596da2  pc-meeting-dark-normal.png
c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96  h5-index-normal.png
c5c2ef70736e628e5d632b5a76e1fc567e97ae5a5f7fb35edb602016fd6d5361  pc-meeting-light-normal.png
cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385  h5-mine-normal.png
d49f922353a407eb61eb5d291c5fec26b5500c2398c81f34df198615fc12381c  pc-file-dark-gray.png
d7e32e08a56712d7637405d4af2def4db23c2b618b0ae85c71d2af7a2ff534d8  h5-process-normal.png
db9f61789cd8952e790619961dd01111f755f2523f3fcaa3c46cdbd49f3074b7  pc-bam-light-normal.png
dd9dc024eb70f959e23837d464db210b15706d1e1d1d262fec96a891de18954b  pc-meeting-dark-gray.png
e329465dfb692f983ae4c18e127ad13cda6d64df13b9a9a5044da026643915c1  pc-org-dark-gray.png
e43740aea4ae43b571b94a334e33176a1325107f7936c7dc09cb7e2bb4b7c145  pc-attendance-light-normal.png
e5e708d91352876b3844fe893f5286eaf5c8158d0369444f8f12f3536bf54db9  pc-bam-dark-gray.png
e6660e9702b990795f1c5a852f7505011642073e3c429ec7ce6b63727dfb1e7b  pc-org-light-normal.png
f31dba1f7c57efad7f5d5947fe3c87ab69abf8de9cf352274b942ed90a266767  pc-three-member-dark-gray.png
f7b3983216dfaae96eca77f9a5c90c4e2175bfdeb361b6076601725289c108c3  h5-mine-gray.png

## 8. 真值纪律与交付边界

- 域名/测试主机用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，正文 0 真值；口令仅在 gitignored runner 运行时从测试服读取并注入环境变量，未写入任何入库文件。
- 截图均标注 Playwright Chromium headless 非物理真机，未冒充真机。
- 本轮只读甄别，未改任何源码/契约/台账，未执行任何真实写操作（所有交互限登录与只读浏览）。
- 项目处于 ≥1000 轮迭代中一环，非终态。
