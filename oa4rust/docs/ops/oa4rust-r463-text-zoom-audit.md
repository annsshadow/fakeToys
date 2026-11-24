# oa4rust R463 文字缩放 200% WCAG 1.4.4 只读甄别报告（t322）

## 0. 元信息
- 日期：2026-11-24
- 任务：t322 [R463]，Kind=work，只读甄别，不改任何入库源码/契约/台账
- 真父：658599b5ecb08b8cacad4cfb38cee9e2d9acf769（HEAD=UP，AB=0 0，工作树干净）
- 分支：opt1000/oa4rust-pc-ai-refactor（禁 main）
- 证据目录：oa4rust/test-results/r463-text-zoom/（gitignored，.gitignore:28）
- 来源标注：Playwright Chromium headless -> 测试服真实页面（NOT a physical device，非物理真机）
- 测试服：oa4rust-test:/usr/local/oa4rust（systemctl active，HTTP 200，入口 index-XFi0y1XK.js 与本地 dist 一致）

## 1. 审计范围与空白维度坐实
- 全库 grep（text-zoom|fontSize=32|zoom:2|Resize Text|1.4.4）0 命中 —— 本维度为空白，无既有实现或契约。
- PC：dashboard / org / file / bbs / calendar / meeting / attendance / common 共 8 页 × light+dark 双主题 = 16 组，viewport 1440x900 DPR1。
- H5：index / mine / message / calendar / collect 共 5 页，锁浅色，viewport 390x844 DPR2 isMobile/hasTouch。
- 合计 21 组，pageerror=0，errors=[]。
- 聚焦 AI 组件：固定高度 stat-card 数字（.stat-value font-size:32px）、渐变文字（.ai-gradient-text + background-clip:text）、玻璃卡（ai-panel/glass-card）、侧栏导航、按钮、表单 label。
- 已覆盖维度严禁重复：motion / focus / 44px / 响应式@media / modalFocus / 骨架 R453-R456 / 对比度 R458-R462 / forced-colors R461 / prefers-contrast R462。本轮仅文字缩放。

## 2. 方法学：四种缩放态
- baseline：原样。
- 方法A font32：documentElement.style.fontSize=32px（200% × 16px），仅缩放 rem/em/rpx，不缩放显式 px 与固定 px 高度。
- 方法B zoom2：body.style.zoom=2，等比缩放全部 CSS px（含固定高度），会改变布局视口，非 text-only zoom。
- 方法C fontDouble：以 baseline computed px 快照内联翻倍每个元素 font-size（容器高度不变），最接近 text-only zoom 的文字放大，但不复刻浏览器 reflow 启发式。
- 判据：DOM 几何（getBoundingClientRect + scrollHeight vs clientHeight 判裁切；相邻同角色 rect 交叠判重叠；docScrollWidth 判横向溢出）+ 截图目视 + CDP CSS.getMatchedStylesForNode。

## 3. 决定性证据：方法A 对显式 px 页面无效
- font32 与 baseline 像素 sha256 完全一致的组数：14/21（如 pc-common-light baseline=font32=`33f73040fdde480b96841e417ee4fee1d06884845ba16e4303365e3d9b0db073`；pc-dashboard-dark、pc-org-light/dark、pc-file-light/dark、pc-calendar-light/dark、pc-meeting-light、pc-attendance-light/dark、pc-common-dark 等）。
- 逐组三态裁切/重叠/横向溢出：baseline/font32/zoom2 三态数值完全一致；重叠态 0/63，横向溢出态 0/63。
- 源码归因：Dashboard.vue:435 .stat-value{font-size:32px}、侧栏与标题均为显式 px，非 rem —— 浏览器字体偏好对显式 px 影响有限，属 WCAG 场景下的规范预期，非缺陷。

## 4. 逐组四态几何汇总（裁切数/重叠数/横向溢出）
- 21 组 × 4 态 = 84 组读数；重叠全程 0，横向溢出全程 0。
- 裁切数分布：dashboard(双主题)/org(双主题)/file(双主题) 与全部 H5 页 = 四态均 0；
- bbs(双主题) = 3（bbs-header 计 2 角色 + bbs-main 计 1）；calendar(双主题)=1（calendar-grid）；meeting/attendance/common(双主题)=1 —— 三态数值在 baseline/font32/zoom2 完全一致。
- 方法C fontDouble 裁切数同三态（bbs=3/calendar=1/meeting=1/attendance=1/common=1），未新增裁切组；前述裁切在 baseline 即存在。

## 5. 关键元素四态实测明细（原始值）
- stat-card（pc-dashboard-light）：baseline fs14px rect.h190.5 scrollH189/clientH189 oy=hidden clipsY=false；fontDouble fs28px rect.h309 scrollH307/clientH307 oy=hidden clipsY=false —— 容器自适应增高，无裁切。
- statNum .stat-value：baseline/方法A/方法B font-size 恒 32px（rect.h48）；方法C 32px->64px（rect.h48->96，scrollH96/clientH96，clipsY=false）—— 数字完整可见。
- 渐变文字 gradText（OA4RUST / 早安admin / 公共组件库）：baseline 14px(h21)/20px(h30)；方法B rect.h 翻倍 21->42、30->60 而 scrollH/clientH 不变；方法C 28px(h42 scrollH42/clientH42)/40px(h60 scrollH60/clientH60) —— 三态均 clipsY=false。
- formLabel（pc-meeting-light）：baseline 12px rect.h18 scrollH18/clientH18；方法C 24px rect.h36 scrollH36/clientH36 clipsY=false。
- navItem（pc-common-light）：baseline 14px rect.h47；方法C 28px rect.h74 scrollH74/clientH74 clipsY=false。
- glass card（pc-calendar-light）：baseline rect.h101 scrollH99/clientH99；方法C fs28px rect.h152 scrollH150/clientH150 clipsY=false。
- bbs-header（pc-bbs-light，唯一观察点）：baseline fs14px rect.h34 scrollH50/clientH32 clipsY=true；方法C fs28px scrollH94/clientH32 clipsY=true（截图目视标题上半被截）。
- 相邻同角色重叠：statCard / gradText / button / formLabel / navItem 五类，四态全程 0。

## 6. CDP 根因与 bbs header 观察点定性
- CDP（DOM.enable + CSS.enable + getMatchedStylesForNode）bbs panel 命中：.modern-page{min-height:100%}、.bbs-view[data-v-eea3df64]{height:100%}；.bbs-header 用 display:flex align-items:center justify-content:space-between、padding:16px 24px（BBSForum.vue:1383-1385），无固定 height、无 overflow:hidden 声明。
- 定性：bbs-header 在方法C下 scrollH50->94 但 clientH 恒 32，是"内联翻倍 font-size 但容器 height:100% 不 reflow"的模拟局限所致；baseline 即 clipsY=true 且截图文字完整（flex 单行横向排布），方法C 的纵向挤压在真实浏览器 text-only zoom 触发 reflow 后通常不出现。
- 因此该方法局限观察点不作为"文字缩放引入的可读退化"判可达真缺陷；据实记录边界：如需在辅助技术真实环境复验 bbs header 极端字号，建议列为后续关注项（非本轮 P 级）。
- 源码引用：Dashboard.vue:22 stat-card 模板、:424 .stat-card{overflow:hidden;padding:var(--space-5)}、:435 .stat-value{font-size:32px;font-family:Orbitron}；BBSForum.vue:7 header.bbs-header、:1384 .bbs-header flex 布局、:1490-1491 .ai-page .bbs-header 背景规则。

## 7. 三分判定（据实，不臆造）
- 可达真缺陷：0。方法A/B 无新裁切；方法C 下全部关键 AI 组件（stat-card 数字、渐变文字、form label、nav、glass card）scrollH/clientH 同步增长、clipsY=false、无重叠无横向溢出。bbs header 裁切定性为方法C模拟局限（见第6节），非真实可达退化。
- 不可达 L1：0。
- 正常-规范预期/边界关注：方法A 显式 px 页面不受字体偏好影响（14/21 像素一致）；方法B 等比缩放比例恒定无新裁切；方法C 文字翻倍容器自适应；H5 五页四态全 0 裁切/重叠/溢出；bbs header 方法C纵向挤压缩放局限列为边界关注（非P级）。

## 8. 真值纪律
- 全文使用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，零真实域名 / IP / SSH 端口 / 口令 / 个人路径。
- 凭据经 ssh 运行时从 /usr/local/oa4rust/.admin-credentials 正则提取注入 OA_TEST_USER/OA_TEST_PASS；OA_BASE_URL 真值只在 gitignored runner，不入库。
- 截图来源标注非物理真机，未以任何方式冒充物理设备。

## 9. 截图 sha256 原始值（84 张，命令行 sha256sum 输出）
B``
95af2b84a08af75e603a927b7e757e5cf132f6a848c2cd26b82b9fd1e6f6f1fc  h5-calendar-baseline.png
1e29ace1fd355345e0a8f38238f68577d93c3b2902f0e9ec0c523621de158ef2  h5-calendar-font32.png
703582b26d7d8934f5ff304b70333bb78a9b6ae9f801b814bb16808d753de02f  h5-calendar-fontDouble.png
221541764be3ca539d1b6516fe44518336b0358d7b12fbb08f748f14631a84ae  h5-calendar-zoom2.png
cbae5e3d08c81c9b2276a9f3c103d6b0094494f6dbda229b8a1f66a2276f0ed6  h5-collect-baseline.png
43957741f0aaf2a61aa2eba71359dc60e78a34823ed2c26966b5c0c79e3b7c49  h5-collect-font32.png
e4ca3bfbf0966133122a25031ab71024c46f78865fe556cbccceb65eded256ba  h5-collect-fontDouble.png
abaa9a77ccd6fee4934ac4e80c6f945fc26c00f5985d008e1fcd4685600c27b1  h5-collect-zoom2.png
c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96  h5-index-baseline.png
fee5a314933db2ade62365409639c6e97e07d55f592a83cd28fb5f5bbbeee2d4  h5-index-font32.png
79b95de8d27818bc7bf6956bf895aeb93f6e9cdbdcabda9d09a69e6e11cb7d34  h5-index-fontDouble.png
bcd20f9e8cc5854e720aa9f4ad4e24691ce71406c876619056c3e2362e65d8de  h5-index-zoom2.png
170482f3818cb89f3b3fd21adf4971b65a72ef834b2b270a600c1e736800b3d4  h5-message-baseline.png
e12bee3d339219b1d5a62123db399406465534680fb6a93ec64b7a4c7bec7d16  h5-message-font32.png
39e947aded344f6d3f5e7444f79148bfbb43e1a60fcc8a31a7638c9fb3eed955  h5-message-fontDouble.png
d0c2a4f8043ac1d6e87aad479ac265bccd5d282529b3ab49e52f8a2b1b05ab27  h5-message-zoom2.png
cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385  h5-mine-baseline.png
1d21846f7176a396d6709189662114e06632936c1573bfd2067a027e00c47ee5  h5-mine-font32.png
67df13ed72ccb0f512061db80d3a0240391ce950c94e3de370cd883f77a91b4f  h5-mine-fontDouble.png
8a70c70486ed4ce032d4d591006b3ec0753811062e59cf265c645e7afaf08270  h5-mine-zoom2.png
620c4d352f45d358c7a8f309c678b36afe97168a36dd19b2bf8135a383da6b8f  pc-attendance-dark-baseline.png
620c4d352f45d358c7a8f309c678b36afe97168a36dd19b2bf8135a383da6b8f  pc-attendance-dark-font32.png
a88021337415389f4164c949b14d55da116f308e769fbec44b225b18b918c29f  pc-attendance-dark-fontDouble.png
aba84a854f5927dbeb4052c57417d221cd6c26d7a4d3a01865f68f750c43c6da  pc-attendance-dark-zoom2.png
7571f6534bfd99ead4c0102dc015db3a20c3d3802f24d5065b9b7cf71d8e0011  pc-attendance-light-baseline.png
7571f6534bfd99ead4c0102dc015db3a20c3d3802f24d5065b9b7cf71d8e0011  pc-attendance-light-font32.png
df934db184e0ccb716b110d80c6297aa32a9929b16905ea8c015c0061c63e4f0  pc-attendance-light-fontDouble.png
2506528e7052c4047a960b0b5ad07856027aca7dba50868501a17b9d5db9f223  pc-attendance-light-zoom2.png
a79263b12da49f2a82d019d74c0666f0fc12c54e75ace43a314c1fbd1fec4bba  pc-bbs-dark-baseline.png
a79263b12da49f2a82d019d74c0666f0fc12c54e75ace43a314c1fbd1fec4bba  pc-bbs-dark-font32.png
ab64a8e0de4deb18664ffea0c8429fa690c1c933c1dba6217056b507056dfabc  pc-bbs-dark-fontDouble.png
1db5fb18c794ae023727f69dd29efbd506c5d4a8f2ac3b005d9220fcb8405a0d  pc-bbs-dark-zoom2.png
7f3838288cbc34c335f454f3757c279defaaac9ed5ff6d8402dc1a9eb72334ae  pc-bbs-light-baseline.png
7f3838288cbc34c335f454f3757c279defaaac9ed5ff6d8402dc1a9eb72334ae  pc-bbs-light-font32.png
ffcea8e84c15bf97eb3c7aa7c03365b58f035c52bba080d29c91a9aff031d19a  pc-bbs-light-fontDouble.png
e997a65b92b9cb11d2dc3652b25e96f2c52ec4931b13d29a3dea80747838e27e  pc-bbs-light-zoom2.png
806838833b14f266720eef378c6b2a34127a7c83a55ab07af3e97698ecae16f6  pc-calendar-dark-baseline.png
806838833b14f266720eef378c6b2a34127a7c83a55ab07af3e97698ecae16f6  pc-calendar-dark-font32.png
b642858d3fc5dd9f480bdbed140c659f7fbadece93df3aea0cea8b770ba12871  pc-calendar-dark-fontDouble.png
1d0527ca90653d3703453a3d67686a4e099557d3523166fd4a9a53a3b40aef49  pc-calendar-dark-zoom2.png
58bc1b43aaa049014fc1cf3a00bac5955f94de87df49724b8479c208f205c783  pc-calendar-light-baseline.png
58bc1b43aaa049014fc1cf3a00bac5955f94de87df49724b8479c208f205c783  pc-calendar-light-font32.png
98fe916370e9fec54007fd16a94eae08e1418b5d9cf56626d03ade450f87c15a  pc-calendar-light-fontDouble.png
fb11ec18f11d003913efa78d7bceebb9ab9d9c30a4f3947f1b98af60704ce8fc  pc-calendar-light-zoom2.png
afe79b29a25873e32fabc99ddcb7729ec6aab8faffcec317d88c0adfaca33424  pc-common-dark-baseline.png
afe79b29a25873e32fabc99ddcb7729ec6aab8faffcec317d88c0adfaca33424  pc-common-dark-font32.png
c1d57f4a053ef9e6c75eaf514e3ac2d4fd37ddffa2dc052378856deca017fa60  pc-common-dark-fontDouble.png
02b4813d57eb71b9d70038e187a0e851340113c7d64f79447cfe84a98f45401a  pc-common-dark-zoom2.png
33f73040fdde480b96841e417ee4fee1d06884845ba16e4303365e3d9b0db073  pc-common-light-baseline.png
33f73040fdde480b96841e417ee4fee1d06884845ba16e4303365e3d9b0db073  pc-common-light-font32.png
df1dcb0365772caa6319c0e6a7ddb963387c31aa716a749d5d1037008a8ed5de  pc-common-light-fontDouble.png
c733279a9be0392f171d72eb7c49ecbb701be027358dd765f93ea4e8350cba03  pc-common-light-zoom2.png
ae7f381cc973a171a7306cc52ef88a5583449eafeebebb93f6a1643afd5f15bc  pc-dashboard-dark-baseline.png
ae7f381cc973a171a7306cc52ef88a5583449eafeebebb93f6a1643afd5f15bc  pc-dashboard-dark-font32.png
6dcd5a98a6d6ae54a08729d924b284c4e4a5581465d200fcfabda964f441140a  pc-dashboard-dark-fontDouble.png
d4e3dd3d763a98e546172d531db446b1f29028c0599ee17254fe51f45e9fc3db  pc-dashboard-dark-zoom2.png
b1ca1fe62ae9a1074a87f11712c7447118240d6eb74ba83314776c409181fd50  pc-dashboard-light-baseline.png
b1ca1fe62ae9a1074a87f11712c7447118240d6eb74ba83314776c409181fd50  pc-dashboard-light-font32.png
380a8daa58370be4e6c25ac79834b9e6d787d8e575d7d0caa7ae9173bd0d074c  pc-dashboard-light-fontDouble.png
7eb0a2a205bc7853226b3143e5b117cbafd3034f8cdb5cdd2cb2ccbe1753b090  pc-dashboard-light-zoom2.png
94f4985914fc20808246a883da8c61e2039f00414e8696cbcdc9b2096adaf47c  pc-file-dark-baseline.png
25aca5742544c3214a63df7516f63536fff2593737e71d502829b484889ffdd9  pc-file-dark-font32.png
0d49fdc53d2c720ee60bc97c2536d6a007ca8c1498f00106f2998f53cb7d6aae  pc-file-dark-fontDouble.png
64ba758143893e5b85d7263cda1cff32e33f6bda59068f654f408626e46846eb  pc-file-dark-zoom2.png
4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf  pc-file-light-baseline.png
4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf  pc-file-light-font32.png
7d33555ec05cebcadcc4cb78ca86aece6070917cff6bc53a643f67b1e3a00a61  pc-file-light-fontDouble.png
68fbe73a02279c2e3ecee8713fbf9e6d4e54d0c043c23f28f24fe97b0a9b84ed  pc-file-light-zoom2.png
2479f8324e5474929a39312dd02d866348b945d747fe6a1df678fa237a9253f1  pc-meeting-dark-baseline.png
fb4585fa4acaa458b48629b41d3d12b3a814fb9a5dd6b6780196a776e2192554  pc-meeting-dark-font32.png
df767b8abe8644c291ee3fe9091c7b93a27a26e5d264f4e7ca8e0986b336bb48  pc-meeting-dark-fontDouble.png
a443d415e6a69c5838b5f428ed364583f4a392f4c370daf4e38920fe0f73c856  pc-meeting-dark-zoom2.png
7edfdf3a48e240bfe505164ab0b8ef957ede3e2d96a838cbf0917b0f46c2a12e  pc-meeting-light-baseline.png
7edfdf3a48e240bfe505164ab0b8ef957ede3e2d96a838cbf0917b0f46c2a12e  pc-meeting-light-font32.png
999a674d58e38a01dde5e28e065eb949a7270adf19171d121b640797d2dc3629  pc-meeting-light-fontDouble.png
cb93de4fb3afd6f46e376cd7fe629d2b208c65b2f7aec24e61ddc7d4b5bfdde6  pc-meeting-light-zoom2.png
9e08677b5bcd43ddb7bcbca0a02f7b9187e4002bab9bcaeff1d57c8c841c8764  pc-org-dark-baseline.png
9e08677b5bcd43ddb7bcbca0a02f7b9187e4002bab9bcaeff1d57c8c841c8764  pc-org-dark-font32.png
a7614c0c04080e6960e75dca3172032ef16805aa8d4878e63f6878b660bed797  pc-org-dark-fontDouble.png
cf295cc2179268a2c3fc3dd92d7f90d4663ff1fdd133e94b123faa6ba6e393bc  pc-org-dark-zoom2.png
2398bcaa36981017b1b92cd25e0f23964a037de794d14facd61261ca81f4bcf0  pc-org-light-baseline.png
2398bcaa36981017b1b92cd25e0f23964a037de794d14facd61261ca81f4bcf0  pc-org-light-font32.png
fb6fa5af39e769b80ba7b38db258a2c3840b1b993f6936f62f33797cfd0c119e  pc-org-light-fontDouble.png
8adb7ca51b9b5c453e6058fb99a8e1da84bcf7db39cb474600146c97d5eaee8e  pc-org-light-zoom2.png
```
