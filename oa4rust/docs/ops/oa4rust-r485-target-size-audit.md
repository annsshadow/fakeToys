# R485 WCAG 2.5.8 Target Size Minimum 目标尺寸最小值甄别报告

## 1. 概况

- 轮次：R485（台账 §480 据实立项）；真父提交 b866925ea492c6b0e75ce42608578457b6c450a6。
- 任务性质：只读甄别，不改任何入库源码；唯一写域为本文档。
- 判据：WCAG 2.5.8 Target Size Minimum（AA，目标至少 24×24 CSS px，或满足 24px 间距豁免）；明确区分 2.5.5 Target Size Enhanced（AAA，44×44px），本轮按 AA 24px 判定，不以 AAA 阈值混判。
- 证据来源：Playwright Chromium headless 非物理真机，PC 1440×900 DPR1，light/dark 双主题，attendance/bbs/file 三页共 6 组；几何来自 getBoundingClientRect（CSS 像素，非 DPR 放大），根因来自 CDP CSS.getMatchedStylesForNode + DOM.getBoxModel。
- 测试服：受控测试环境（域名以 <YOUR_TEST_DOMAIN> 占位）；凭据经环境变量注入，真值不入库。

## 2. 方法与几何判据

- 遍历 selector：button / a[href] / input:not([type=hidden]) / select / textarea / [role=button] / [tabindex]:not([tabindex=-1])；过滤 width/height<=0、display:none、visibility:hidden、opacity:0 的不可见控件。
- 达标判据 meets24 = (w>=24 且 h>=24)，w/h 为四舍五入到 0.01px 的 CSS 像素宽高。
- 间距豁免（Spacing exception）：对未达标控件，计算其中心到最近其他可交互控件中心的欧氏距离；以 24px 直径圆不相交的一阶几何近似（中心距>=24px）视为豁免。file 页实测中心距 13–16px 远小于 24px，无论采用精确圆-框相交判据或一阶近似均构成违反，结论稳健。
- 阈值纪律：WCAG 2.5.8 为 AA 24px，2.5.5 为 AAA 44px；本轮所有"达标/违反"均以 24px 计，44px 仅作增强说明不参与判定。

## 3. 静态侦察（仅作候选，不作预判）

- AttendanceApp.vue：.ba/.br（:1880-1881 padding:4px 12px）、.mini-add（:1887）、.cfg-mini（:1894 padding:4px 10px）、.cfg-tab（:1904 padding:2px 10px）等紧凑按钮；.pgb（:1883 32×32px）尺寸达标。
- BBSForum.vue：.add-section-btn（:1419-1423 width/height:24px，恰为边界）、.sec-icon（:1446 width:20px）、.sec-act（:1427-1430 padding:2px 6px）。
- FileManager.vue：.icon-action 按钮（:61-64 ℹ/⬇/↗/🗑，:85 🗑，:124/:137 ✕），全局检索未发现 .icon-action 的 min-width/min-height 自定义规则。
- 全库无 targetSize 相关基础设施（glob 命中为空）；R469/R477/R479/R482/R484 等历轮均未覆盖目标尺寸维度。
- 纪律：以上均为静态可疑信号，全部结论以真浏览器几何实测为准，严禁凭 padding/width 源码文本预判。

## 4. 真浏览器几何实测（决定性）

- 实测 EXIT=0：R485_DONE groups=6 shots=6 errors=0，0 pageerror。汇总：total=242，meets24=184，small(<24px)=58，其中间距豁免 42，真实违反 16。

| 组 | 控件总数 | 达标 | <24px | 间距豁免 | 真实违反 |
| --- | --- | --- | --- | --- | --- |
| attendance-light | 52 | 38 | 14 | 14 | 0 |
| attendance-dark | 52 | 38 | 14 | 14 | 0 |
| bbs-light | 31 | 27 | 4 | 4 | 0 |
| bbs-dark | 31 | 27 | 4 | 4 | 0 |
| file-light | 38 | 27 | 11 | 3 | 8 |
| file-dark | 38 | 27 | 11 | 3 | 8 |

### 4.1 file 页真实违反（16 项，light/dark 各 8）

- 文件列表每行末尾 4 个 .icon-action：ℹ 详情/预览、⬇ 下载、↗ 分享、🗑 删除（FileManager.vue:61-64）。
- 实测尺寸（CSS px）：ℹ 15.02×21、⬇ 11.30×21、↗ 17.34×21、🗑 13.33×21；宽高均小于 24px。
- 相邻中心距：ℹ↔⬇ 13px、⬇↔↗ 13–14px、↗↔🗑 14–16px，全部 <24px，间距豁免不成立。
- light 8 项（两行各 4 个）+ dark 8 项（两行各 4 个）= 16 项真实违反；为高频文件操作（预览/下载/分享/删除），运动障碍用户点中困难，可达真缺陷成立。
- 同页另 3 个 small 控件（全局 input.search-input、button.command-palette-trigger、input.fi）因中心距 109–143px 获间距豁免，不判缺陷。

### 4.2 attendance / bbs 未达缺陷（据实不判）

- attendance 双主题各 14 个 <24px 控件：全局 input.search-input（237.36×15）、button.command-palette-trigger（31.64×18）与 12 个 .cfg-tab（高 21px，宽 44–66px）。中心距 33.24–70px 均 >=24px，全部获间距豁免；静态疑似的 .ba/.br/.cfg-mini 实测未落入 small 列表（实际宽高 >=24px），静态"padding 疑不足"预判被真浏览器数据反证。
- bbs 双主题各 4 个 <24px 控件：全局 input.search-input、command-palette-trigger、页内 input.search-input（160×15）、button.search-btn（30.67×21）；中心距 86.15–143px 均豁免。.add-section-btn 恰为 24×24px 达标，.sec-act 为 hover 态非默认可达目标。

## 5. CDP 根因坐实

- CDP CSS.getMatchedStylesForNode 对 .file-row .icon-action 命中规则：button、input/textarea/select/button（font-size/line-height:normal）、input[type=button]…button（padding-block 1px、padding-inline 6px、border-width 2px）、*（padding:0）等 UA 默认规则；无任何自定义 min-width/min-height 或固定尺寸规则。
- DOM.getBoxModel 实测该按钮边框盒 15.02×21px（坐标 586.48,418.5 → 601.5,439.5），与几何探针一致。
- 根因：.icon-action 未声明最小点击区域，宽度完全由单个符号字符 + UA 默认内边距决定，窄至 11.30px；同行四按钮紧凑排布、gap 不足 24px，故尺寸与间距双判据均不满足。
- 修复建议（供后续 RED→GREEN 轮，本轮不改码）：在 FileManager.vue scoped 样式为 .icon-action 增加 min-width:24px; min-height:24px（并将行内 gap 调至满足中心距>=24px，或保持 24px 见方即可由尺寸判据达标），同步覆盖 :124/:137 面板关闭 ✕ 按钮，不变更 @click/title/aria-label 与业务视觉语义。

## 6. 目视核验

- read_image 目视 pc-file-light-r485.png（sha 066053df…）与 pc-file-dark-r485.png（sha d7ebe750…），均 1440×900 真 OA4RUST admin 文件管理页、非物理真机：我的文件列表两行，每行末尾 ℹ/⬇/↗/🗑 四个极小图标紧密相邻（13–16px 间距），与几何读数一致，名实相符；浅/深主题结构一致。

## 7. 三分判定

- 可达真缺陷：1 项 P2 —— FileManager.vue:61-64 文件列表行操作 .icon-action（ℹ/⬇/↗/🗑）双主题共 16 个控件，实测 11.30–17.34px 宽、21px 高且中心距 13–16px，尺寸与间距双判据均不满足 WCAG 2.5.8 AA；CDP 根因+双主题截图三证合一。P2 定级：高频文件操作、运动障碍用户影响明确，但存在 title/aria-label 名称且功能可通过其他路径（如右键/详情面板）触达。
- 不可达 L1：0（所有被测控件均可见可达，无 aria-hidden/display:none 排除项）。
- 正常规范预期：其余 226 个控件（含 184 个达标、42 个间距豁免）；attendance .cfg-tab 与 bbs .add-section-btn 静态可疑经实测反证或达标。
- 据实声量：不臆造、不凑轮次；2.5.8(AA 24px) 与 2.5.5(AAA 44px) 阈值未混判。

## 8. 证据与 sha256（命令行 sha256sum 原始值）

- pc-attendance-light-r485.png 678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b
- pc-attendance-dark-r485.png 85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7
- pc-bbs-light-r485.png 888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105
- pc-bbs-dark-r485.png 6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c
- pc-file-light-r485.png 066053df18c5f1c676bd606d31b2a046745444692e2e06ce106958f71fe7547f
- pc-file-dark-r485.png d7ebe750e0bf26efbca0896ab619ae7dae03e2af697ff92616fa43b1c35b9875
- report.json 676b26083520283ed2deedd87db331771d43e8bcfd303da777496ed7060d9bd8
- cdp-icon-action.json 4179e9731b2bc33d897b9905ef785fd794eaa964eae759aac642b0a2f2c3cb08
- analysis.txt 80741ff06c395b0fb9bbaf2cf17e405667eab5e9bed38ed1d2a1e858a1a3fd67
- 证据目录（gitignored）：oa4rust/test-results/r485-target-size/（6 PNG + report.json + cdp-icon-action.json + analysis.txt + 脚本）。

## 9. 真值纪律

- 测试域名/IP/口令/SSH 端口/个人路径等真值一律不入库，域名用 <YOUR_TEST_DOMAIN>、主机用 <TEST_HOST> 占位。
- 凭据在 runner 运行时从受控凭据源提取后注入进程环境，真值仅存在于 gitignored 脚本，不回显、不入库。
- 截图来源如实标注为 Playwright Chromium headless 非物理真机，未冒充物理真机读屏人工听感。

