# R468 WCAG 1.4.10 Reflow 窄视口回流甄别报告

- 任务：t328（Attempt a32c1485-a47b-40d5-bb99-af410b2a7cc6），只读甄别，不改入库源码/契约/台账。
-  auditors：design-lead；测试服由环境变量注入，域名仅以 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，0 真值。
- 判定基准：WCAG 1.4.10 Reflow——320 CSS px 宽度（等价 1280px 缩放到 400%）下内容须回流为单列、无信息丢失、无二维滚动；数据表格等允许双向滚动须据实区分豁免。
- 证据来源：Playwright Chromium headless **非物理真机**。PC 320×256（等价 400% 缩放）+ 1280×1024 baseline，双主题；H5 390×844 DPR2 isMobile/hasTouch。0 pageerror。

## 一、审计概况

本轮为全库 WCAG 1.4.10 专门测试/契约命中 0 的真实空白维度。与已覆盖的 R463 文字缩放（字号翻倍）不同，本维度只改变视口宽度、字号不变。覆盖 PC 8 页（dashboard / common / bbs / bam / org / file / attendance / process-work）×双主题 16 组，H5 5 页（index / mine / message / calendar / collect）锁浅色。逐页采集 documentElement.scrollWidth vs clientWidth、AI 卡片 scrollHeight/clientHeight、渐变标题换行、侧栏/主区几何、表单溢出、表格横向滚动，并以 1280px baseline 横比；另做 320px 冷启动对照排除 resize 残留。

## 二、方法学与证据链

1. 主 E2E：同一登录态 context 内在 1280×1024 与 320×256 间切换，逐页 domcontentloaded + 700ms 稳定后采样并截图。
2. 冷启动对照（r468-cold.cjs）：以 storageState 在全新 320px context 直接加载各页，排除“从 1280 resize 导致抽屉展开残留”的可能。
3. CDP 坐实（r468-cdp-shell.cjs）：读取 .app-shell/.sidebar/.main-area/.content/.topbar 的 computedStyle（transform / margin-left / width / x）与 --sidebar-width 令牌，1280 与 320 双宽度对照。
4. 目视双重判定：逐张读取 320px 截图与 baseline 截图人工核验（不单凭 computedStyle）。

## 三、逐页读数（320px 视口）

格式：KEY | baseline doc | 320 doc | AI卡片数 | 裁切卡片 | 标题数 | 换行/裁切标题 | 表单溢出 | 表格 | 导航节点。

pc-dashboard-light | 1280/1280 | 320/320 | 23 | 8 | 6 | 4 | 0 | 0 | 5
pc-common-light | 1280/1280 | 320/320 | 4 | 0 | 2 | 0 | 0 | 1 | 11
pc-bbs-light | 1280/1280 | 320/320 | 6 | 3 | 3 | 1 | 0 | 0 | 8
pc-bam-light | 1280/1280 | 320/320 | 10 | 3 | 2 | 1 | 0 | 0 | 5
pc-org-light | 1280/1280 | 320/320 | 4 | 0 | 1 | 0 | 0 | 0 | 5
pc-file-light | 1280/1280 | 320/320 | 5 | 2 | 5 | 1 | 0 | 0 | 5
pc-attendance-light | 1280/1280 | 320/320 | 17 | 3 | 2 | 0 | 0 | 0 | 5
pc-process-work-light | 1280/1280 | 320/320 | 4 | 2 | 2 | 1 | 0 | 0 | 5
pc-dashboard-dark | 1280/1280 | 320/320 | 23 | 8 | 6 | 4 | 0 | 0 | 5
pc-common-dark | 1280/1280 | 320/320 | 4 | 0 | 2 | 0 | 0 | 1 | 11
pc-bbs-dark | 1280/1280 | 320/320 | 6 | 3 | 3 | 1 | 0 | 0 | 8
pc-bam-dark | 1280/1280 | 320/320 | 10 | 3 | 2 | 1 | 0 | 0 | 5
pc-org-dark | 1280/1280 | 320/320 | 4 | 0 | 1 | 0 | 0 | 0 | 5
pc-file-dark | 1280/1280 | 320/320 | 5 | 2 | 5 | 1 | 0 | 0 | 5
pc-attendance-dark | 1280/1280 | 320/320 | 17 | 3 | 2 | 0 | 0 | 0 | 5
pc-process-work-dark | 1280/1280 | 320/320 | 4 | 2 | 2 | 1 | 0 | 0 | 5
h5-index | - | 980/980 | 2 | 0 | 0 | 0 | 0 | 0 | 0
h5-mine | - | 980/980 | 0 | 0 | 0 | 0 | 0 | 0 | 0
h5-message | - | 980/980 | 0 | 0 | 0 | 0 | 0 | 0 | 0
h5-calendar | - | 980/980 | 0 | 0 | 0 | 0 | 0 | 0 | 0
h5-collect | - | 980/980 | 0 | 0 | 0 | 0 | 0 | 0 | 0

说明：PC 16 组文档级 scrollWidth 均等于 clientWidth（无文档级横向滚动条），但这是因 .app-shell 为 width:100vw + overflow:hidden，内容被裁进窄缝后在 .content 内 overflow:auto 滚动——见第四节根因。H5 五页 980/980 是 DPR2 下 CSS 视口换算（390 CSS px × 2 物理像素 = 780，documentElement 报 980 由 uni-app viewport 设定），布局完整、无裁切。

## 四、决定性根因（CDP 坐实）

冷启动对照：COLD-dashboard / COLD-bbs / COLD-bam / COLD-process-work 与对应 resize 截图 sha256 完全一致（dashboard b12c51b0…、bbs c6d04290…、bam e8e08925… 同时等于 pc-bam-light），排除 resize 残留，确认为稳定可达状态。

CDP 双宽度几何（同一页 dashboard）：

| 元素 | 1280px | 320px |
| --- | --- | --- |
| .app-shell class | app-shell | app-shell **mobile** |
| .sidebar 宽/x/transform | 240 / 0 / none | **280 / 0 / matrix(1,0,0,1,0,0)（可见）** |
| .main-area 宽/x/margin-left | 1040 / 240 / 240px | 320 / 0 / 0px |
| .content 宽/x/margin-left | 1040 / 240 / 0px | **80 / 240 / 240px** |
| .topbar 宽/x/margin-left | 1040 / 240 / 0px | **80 / 240 / 240px** |
| --sidebar-width | 240px | 240px |

根因链（oa4rust/frontend/packages/ui/src/components/AppShell.vue）：
1. :446 `isMobile.value = window.innerWidth < 768` → 320px 时 .app-shell 加 mobile 类。
2. :1187-1195 移动端媒体查询：.sidebar 基线 transform:translateX(-100%)（抽屉隐藏），但 `.app-shell.mobile .sidebar { transform: translateX(0) }` **无条件** 把 280px 侧栏常驻显示。
3. :1201-1205 `.topbar, .content { margin-left: var(--sidebar-width) }` 移动端仍给主内容加 240px 左外边距。
4. 合成结果：320px 视口减去 240px 左边距，主内容（含顶栏与全部业务内容）仅剩 80px 可用宽度，被压成文字碎片。

连锁 AI 组件退化：dashboard .stat-card scrollW 53/clientW 40（clipX=true）；bbs/bam/attendance/process-work 的 .modern-hero scrollH 323/465/156/524 对 clientH 32/32/32/48（纵向内容被裁）；渐变标题（团队论坛/业务活动监控/文件管理/工作流待办）被挤到 20-28px 宽竖折行。均为 80px 窄缝的次生现象，非独立缺陷。

## 五、目视双重判定

- 1280px baseline（baseline-dashboard-light 78a8315d…）：侧栏 240px + 主内容 1040px，统计卡/快捷入口/待办/渐变问候标题布局完整清晰。
- 320px（pc-dashboard-light 73aab5c4…、pc-bam-light e8e08925…、pc-process-work-light 0a692e15…）：可见 280px 抽屉侧栏常驻 + 底部 tabbar，主内容仅右侧 55-80px 窄缝，标题"admin"、数字"1"、业务文字呈碎片状，无法阅读。与几何数据一致。
- H5（h5-index c531e964…）：品牌渐变统计卡（8/1）、功能宫格、底部 tabbar 全部完整，无裁切无横向滚动。

## 六、三分判定

- 可达真缺陷：**1 项，P2**——PC 端 320px 视口（等价 400% 缩放）下 AppShell 移动布局侧栏常驻 + content/topbar 双重 240px 左外边距，致全部 8 个审计页（双主题一致）主内容仅剩 80px、文字碎片化不可读，构成 WCAG 1.4.10 的实质内容不可用。定 P2 而非 P1：存在 H5 独立应用作窄屏替代路径，且属极窄视口/高缩放场景，非默认桌面分辨率下的全局阻断。
- 不可达 L1：0。
- 正常-规范预期若干：H5 5 页布局完整；common 页 1 个表格未触发横向滚动不判缺陷；PC 文档级 scrollWidth=clientWidth（但内容为窄缝内部滚动，已在根因说明，不作为“无缺陷”依据）。

修复建议（供后续 RED→GREEN 轮次，本轮不改码）：移动端应让 .sidebar 默认 translateX(-100%) 隐藏、仅抽屉打开态滑出；并移除移动端 .topbar/.content 的 margin-left:var(--sidebar-width)，使主内容占满 100% 宽度。

## 七、截图 sha256 清单（命令行 sha256sum 原始值）

证据目录 gitignored：oa4rust/test-results/r468-reflow/。共 41 个 PNG（PC baseline 16 + PC 320 窄视口 16 + H5 5 + 冷启动对照 4）。全部为 Playwright Chromium headless 非物理真机截图，来源已在第二节标注。

0a692e15a7b6d0c2a1663f7d268c51725345641fec99d0edb247422c6247730b  pc-process-work-light.png
0fde41802463600c1e0cf68bb4984490fc751aaefa67128ecc6241f321f46ff4  pc-bbs-dark.png
170482f3818cb89f3b3fd21adf4971b65a72ef834b2b270a600c1e736800b3d4  h5-message.png
47b36b66580508b7654ef36304c25af042b49bafa5eb7d0ca038887af08fc253  baseline-org-light.png
4a4d1efa6cf79565279eea34d2b12564b8d054449b1ba2aeac68971304a5a093  pc-bbs-light.png
5b1e6f4017c23865ef3960412501110f091ee3c575eefa885f6428155949927a  pc-org-dark.png
5ff813507e44a26e44a843592bbecb2b9e52dc5c062e910c15c44168e0161162  pc-attendance-light.png
6b1271956fca8389ae44dd517f8b2ad54922028138dfa7477f2544d15e93829c  baseline-common-light.png
6c5488989fbc0d690adfcd2d5e65f4ba0a870d9c8d74fa46e6c54e2fc405c1c4  pc-file-dark.png
73aab5c49a8a1792aaa15c33933d21b102df638593c79472c93bd84f7bd3cea0  pc-dashboard-light.png
78a8315db86b89e8c734f6e12832340703eda66702813dd12b463614f3f9960f  baseline-dashboard-light.png
79f083d494c87331237f29ef637c3b701f78b501d2ed3d45b0c5437f52559f81  baseline-attendance-dark.png
7b19912616bbdf17cb6391ac4fc04390fee1943658f227b5a5ab8a0fe7aa924e  baseline-org-dark.png
7b35cd391f272f78a56fc303a124fe329141ccb0f885d7965af2d635004bc686  pc-org-light.png
856be4fffc0044a41fffc4faed6df41aa9191c7cc3615c9eadacf4e0f6a1ea7f  baseline-file-light.png
89609bede8dd18e41b17a4a1eff4601a920ae34d320901bb3d4bc320df7dc8d7  baseline-bam-light.png
90d610e626f18db23e598d2fc4a395bf72fe0f85b362a79a17cf93a1884a5c3f  baseline-process-work-light.png
944cf343b58a4dcab74e43e9ffe0dc856ed3a7f53e6d46d23aa4137b5e8ef915  baseline-file-dark.png
95af2b84a08af75e603a927b7e757e5cf132f6a848c2cd26b82b9fd1e6f6f1fc  h5-calendar.png
9f01b61a414afdc06ae1064b5fb599a85a188fa1f8c7ab6fd28bbc16d8464f8b  baseline-bbs-dark.png
a41758b660c3bb0d86035f252586201abed1d3baf64a70738281af7263681a8c  COLD-process-work.png
a8f1c16da404147a27dbc51a3fb9abf91b08dfcd96caa2242c1c0e89771f684b  pc-dashboard-dark.png
ad470ef807d5a956a6c0973136fb359d03d932092b9bc37b3fd75edf821ec26b  pc-file-light.png
adf044073ee58f19dd7e9e0bfe3505ae32f35af84999669384c17d88c81c8a50  baseline-common-dark.png
b12c51b04f9b17b7ca37d8e64d7329760f2931cc4d174a436d168bf77a807f6f  COLD-dashboard.png
b1a89f044484e6738dab07bf6d2d86a1a0a4b8d92a7101dd5439fb193417aeb3  baseline-process-work-dark.png
c46115970eadcee66c918f3c484aa7956fcaff5e7b5d134fdd733dd9f20b665c  baseline-attendance-light.png
c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96  h5-index.png
c635a3bcab1f4ce58f755663703a48c98caf468e444f686051616067a47a6c23  baseline-bbs-light.png
c6d04290191c7b6235b0ee9e5ceb3f420fe11b37adb53c7f2c6d528897851bbf  COLD-bbs.png
cbae5e3d08c81c9b2276a9f3c103d6b0094494f6dbda229b8a1f66a2276f0ed6  h5-collect.png
cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385  h5-mine.png
d3c8acfc0b9c675dd2104a1c3fa9efe79ec00a91d5ef7b1cc78c507ef5373789  pc-common-light.png
deb25b9527dce5ee6dcf6e8619ea393a3c5712cb64aff07f619a4abc0dfdef65  pc-bam-dark.png
e28566449ad5939c4542325f90621c0ef6b4796a3d2baf0b4762f82cea080c3f  baseline-dashboard-dark.png
e8e0892540da8997c6558f29148b3f277a67110276f8c746af1706aab79fc82a  COLD-bam.png
e8e0892540da8997c6558f29148b3f277a67110276f8c746af1706aab79fc82a  pc-bam-light.png
f0ba0fbb78f8df26409170fdac57cd06f674eb45dc23441a2a6e3fb857602bf7  pc-attendance-dark.png
f1d2732ac52a37cff529ae5dfb5fdf400eb3d2f15b10c5a49eb0013145eeac22  pc-common-dark.png
f9d0862d5d00f06a2ef41b789f3fd12fa8dd4b1dbcbd4dcb1b412ec57bc1a0b9  baseline-bam-dark.png
fba27ce7625e61076c2eb344ca8b11dce44183b6f001f3baa7c13b93366dcd10  pc-process-work-dark.png

注：COLD-bam 与 pc-bam-light sha 完全一致（e8e08925…），COLD-dashboard / COLD-bbs / COLD-process-work 亦与对应 resize 截图一致，这是冷启动对照“排除 resize 残留”结论的直接证据，非截图复用错误。

## 八、真值与证据纪律

- 脱敏：域名用 <YOUR_TEST_DOMAIN>、主机用 <TEST_HOST> 占位，0 真值入库；真域名仅存在于 gitignored runner（oa4rust/test-results/r468-e2e-run.sh 等），不回显、不入库。
- 口令：测试账号/口令在 gitignored runner 中于运行时从受控凭据源提取后注入进程环境，不落盘、不入库、不回显。
- 截图来源：全部标注为 Playwright Chromium headless 非物理真机（PC 320×256/1280×1024 DPR1，H5 390×844 DPR2 isMobile/hasTouch），未冒充物理真机。
- 判定纪律：1 项 P2 可达真缺陷据实上报，未因历史多数轮次判 0 而压低；代码层修复留给后续 RED→GREEN 轮次，本轮只读甄别不改入库源码。
- 0 pageerror（PC+H5 全 21 组 + 冷启动 4 页 + CDP 探针）。
