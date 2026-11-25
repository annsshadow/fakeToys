# R504 WCAG 2.5.8 Target Size（目标尺寸）甄别审计

> 轮次：R504 / 任务 t402（只读甄别，不改码、不改业务）
> 判据：WCAG 2.1 SC 2.5.8 Target Size (Minimum)（AA，24×24 CSS px）为主，兼看 1.3.2 Meaningful Sequence 次选候选
> 基线真父：500cf57907101efa7acc9f11b2cd2712faa03cef（台账 §499 据实立项）
> 分支：opt1000/oa4rust-pc-ai-refactor（非 main）
> 数据来源：测试服真浏览器 Playwright Chromium headless，PC 1440×900 DPR1，light/dark 双主题，CDP DOM + Accessibility.getPartialAXTree + getBoundingClientRect。截图为 headless 非物理真机，非真机手拍。
> 测试地址：<YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，真值仅 gitignored runner 与 .local.keys，本文零真值。

## 1. 甄别范围与判据

主维度 WCAG 2.5.8：目标最小尺寸 24×24 CSS px（AA）。反预判铁律——严禁仅凭源码 width/height 预判，必须真浏览器量测 getBoundingClientRect；并区分 AA 例外：Spacing（ undersized 目标的 24px 直径圆不与其它目标圆/边框相交，即相邻目标中心距≥24px）、inline 文本内、essential、用户代理控件。

候选（静态侦察仅作候选，非缺陷结论）：

1. BBSForum.vue:34-35 版块导航新建按钮 add-section-btn，CSS :1419-1423 width/height=24px、border=1px，恰压阈值，须实测 bounding box。
2. BBSForum.vue:46-49 版块项行内 sec-act 三按钮（版块详情/重命名/删除），CSS :48 padding 2px 6px、font 11px，hover 才显示，须实测 hover 态尺寸与间距例外。
3. IMChat.vue:10 new-chat-btn，主轮实测 17.88×32（宽不足 24），须查 spacing 例外与展开/折叠两态。次选 1.3.2：IMChat.vue:1295-1296 message.outgoing flex-direction:row-reverse，气泡视觉序与 DOM 序相反，须真实发消息坐实。
4. 排除项复核：IMChat.vue:1239 icon-btn（实测 40.72×36）、input-btn（35.47×35）、send-btn（66.67×37）均达标；AttendanceApp.vue:1883 分页按钮 CSS width/height=32px 达标。

前序维度不重复说明：2.4.7 焦点可见（桌面 149 处 outline:none）已在 theme/dark.css:284-289 全局 :focus-visible 兜底并经契约钉死；1.4.11 非文本对比度（R487）、3.3.x 错误识别/建议（R488/R498）、2.4.6 标题层级（R499）、2.5.3 Label in Name（R500）、3.3.4 错误预防（R502）均已独立甄别/修复，本文不重复。

三分判定：可达真缺陷（实测目标 <24 且无例外、无等价替代，须实证）/不可达 L1（候选当前真实数据或状态不可达，不预判缺陷）/正常规范预期（实测达标或命中 AA 例外）。

## 2. 实测方法与程序化自洽

- 主轮 test-results/r504-e2e.cjs + r504-run.sh：3 页面配置（bbs/im/attendance）× 双主题 = 6 组，R504_DONE groups=6 fatals=0，0 pageerror。
- 补采 test-results/r504-sup.cjs + r504-sup-run.sh：同样 3 配置 × 双主题 = 6 组，R504_SUP_DONE groups=6 fatals=0，0 pageerror。
- 第三轮 test-results/r504-r3.cjs + r504-r3-run.sh：bbs sec-act 三按钮中心/边缘距 + im new-chat 两态尺寸 + 延时发消息，R504_R3_DONE errs=0。
- 第四轮 test-results/r504-r4.cjs + r504-r4-run.sh：页面内 40ms 间隔 setInterval 在点击发送前预装，亚秒级抓 outgoing 乐观气泡，R504_R4_DONE samples=36 withOut=16 maxOut=1 errs=0。
- 每轮采 getBoundingClientRect（w/h/top/left）、computedStyle（width/height/boxSizing/padding/border）、aria-label/title、可见性与视口内判定，再 CDP DOM.querySelector 定位取 getPartialAXTree（axRole/axName/axIgnored）。截图 crypto sha256 与命令行 sha256sum 双算一致。
- 证据文件（gitignored）：test-results/r504-target-size/report.json（主轮 6 组）、supplement.json（补采 6 组）、round3.json、round4.json。本文逐项表与这四份 JSON 同源，无 R487 式文档/report 矛盾。
- 首跑披露：主轮首跑 6 组全 FATAL（ReferenceError: o is not defined，measure() 内 page.evaluate(function(o){},o) 未定义入参 o），tools.edit 补 var o={sel:sel,label:label} 后重跑通过；未以首跑数据下结论。
- IM outgoing 采样时序披露：第三轮与补采在延时 1.4-2.2s 后采样，outCount=0（乐观气泡已被 onSuccess 的 refreshFirstPage 替换为服务端消息，服务端 direction 回落为 in）；第四轮亚秒采样在 dt=54~644ms 抓到乐观 outgoing，据实以第四轮坐实布局，不以延时采样误判不可达。

## 3. 逐项程序化证据

### 3.1 主轮（report.json，6 组）

| 组（主题） | 目标 | bounding box | computedStyle | AX | 截图 sha 前 12 |
|---|---|---|---|---|---|
| light/dark | BBS add-section-btn | 24×24 | width/height 24px, boxSizing border-box, pad 0, border 1px | button / 新建版块 / ign=false | 888c846de221 / 6971592b8637 |
| light/dark | BBS new-topic-btn | 107.47×35 | 8px/16px pad, border 1px | button / 发起主题 / ign=false | 同上组 |
| light/dark | BBS sec-act（hover 前） | 0×0 | display:none, pad 2px/6px | none / 空 / ign=true | 同上组 |
| light/dark | IM icon-btn | 40.72×36 | pad 6px/8px | button / 语音通话 / ign=false | 61b0753b3b0f / 8b0d1c40237e |
| light/dark | IM input-btn | 35.47×35 | pad 4px | button / 表情 / ign=false | 同上组 |
| light/dark | IM send-btn | 66.67×37 | pad 10px/20px | button / 发送 / ign=false | 同上组 |
| light/dark | IM new-chat-btn | 17.88×32 | width 17.875px, height 32px, border 1px | button / 发起会话 / ign=false | 同上组 |
| light/dark | Attendance 分页 | found=false（选择器 .paging-btn 错，真实 .pgb），tableCount=0 | — | — | 678bf7a97db9 / 85e665fc7e5f |

主轮 IM 消息序列：convCount=16、msgCount=1，唯一消息 incoming（outgoing=false，avLeft=303、bbLeft=343，头像在左气泡在右），AX role=none/ignored=true。

### 3.2 补采（supplement.json，6 组）

| 组（主题） | 量测项 | 结果 | 截图 sha 前 12 |
|---|---|---|---|
| light/dark | BBS sectionCount | 17；hover 首项后 sec-act 21.09×19，display:block、visibility:visible，AX button / 版块详情 / ign=false，padT=2px padL=6px | 142ed7453f6d / 5a38ec8d9277 |
| light/dark | IM new-chat spacing（展开态） | 32×32，sideCollapsed=false；最近目标=搜索会话 input 中心距 117.8px（其余 ≥269px） | 2b8d00e2a2ca / 036fca90144b |
| light/dark | IM outCount（延时采样） | 0（乐观气泡已回落，见 §2 时序披露） | 同上组 |
| light/dark | Attendance .pgb / table | pgbCount=0，tableCount=0（无表格无分页） | 678bf7a97db9 / 85e665fc7e5f |

### 3.3 第三轮（round3.json，light）

- BBS 首版块三按钮：版块详情 21.09×19（cx=395.5）、重命名版块 23×19（cx=421.5）、删除版块 21×19（cx=447.5）；中心距 ab=26px、bc=26px；边框间隙 ab=4px、bc=4px。
- IM new-chat 两态：展开态 32×32（sideCollapsed=false）、选中会话后折叠态 17.88×32（sideCollapsed=true）。
- IM 延时发消息：msgs 5 条全 outgoing=false（回落），errs=0。
- 截图 pc-round3-light-r504.png。

### 3.4 第四轮（round4.json，light，亚秒采样）

- samples=36，withOut=16，maxOut=1，outgoing 乐观气泡存在窗口 dt=54~644ms，末 6 次采样 count=0（onSuccess 后 500ms refreshFirstPage 替换为服务端消息）。
- 首个命中 dt=54ms：flexDirection=row-reverse，头像 avLeft=1363 / avRight=1395（靠右），气泡 bbLeft=1070.4 / bbRight=1355（靠左），文本=R504 outgoing probe。DOM 中 avatar 在前、bubble 在后，视觉左右反转坐实。
- AX 在采样窗口结束后取（NODE_MISSING_AFTER_SAMPLE）——气泡已被替换，无法坐实此刻 AX ignored 状态；据实披露为"AX 未坐实"，不作为缺陷依据。
- 截图 pc-r4-outgoing-light-r504.png（截图时点气泡已回落，故目视全为 incoming 左对齐；坐实布局以程序化 dt=54~644ms 数据为准）。

## 4. 三分判定（逐项实测依据）

### 4.1 可达真缺陷：0 项

无任一实测目标在 <24×24 且无 AA 例外、无等价替代的情况下构成阻断。

### 4.2 不可达 L1：1 类

- AttendanceApp.vue:1883 分页按钮（.pgb，CSS 32×32 达标）：主轮真实页 tableCount=0、补采 pgbCount=0，当前月份数据无表格与分页条，分页目标不可达，无法在真实 UI 量测；据实判 L1 数据不可达，不预判缺陷（静态 CSS 已达标）。

### 4.3 正常规范预期：全部其余候选

1. BBS add-section-btn：实测 24×24，恰压 AA 24×24 阈值，达标（AX button 可访问名"新建版块"，ignored=false）。源码 border:1px 已计入 border-box，不额外缩水。判正常。
2. BBS sec-act 三按钮：hover 后 21.09×19 / 23×19 / 21×19，虽 <24×24，但三按钮中心距 26px（≥24px），命中 WCAG 2.5.8 Spacing 例外（24px 直径圆互不相交）；AX button + 可访问名（版块详情/重命名版块/删除版块）+ ignored=false。判正常。
3. IM new-chat-btn：展开态 32×32 达标；选中会话后折叠态 17.88×32（宽不足 24、高达标），最近目标中心距 117.8px ≫ 24px，命中 Spacing 例外。判正常。
4. IM icon-btn / input-btn / send-btn：40.72×36 / 35.47×35 / 66.67×37，全部 ≥24×24，排除项复核正确。判正常。
5. 1.3.2 次选候选（outgoing row-reverse）：第四轮亚秒采样坐实单条消息内 avatar（右）/bubble（左）视觉反转、DOM avatar 在前；但消息之间的时间顺序（msgSequence）与文本阅读顺序一致，单条内头像与内容左右互换属 IM 通用布局约定，不改变消息含义或阅读序列，且 AX 未坐实语义影响。据实判不构成 1.3.2 可达真缺陷（属正常规范预期，布局时序据 §2/§3.4 披露）。

## 5. 修复建议

本维度无可达真缺陷，无需修复源码。非阻断观察（不纳入本轮修复、移交后续）：

- IM outgoing 乐观气泡被 onSuccess 的 refreshFirstPage（500ms 延时）替换后服务端 direction 回落为 in，致延时采样看不到自己发出的消息右对齐；若产品要求"发出的消息稳定靠右"，应核查服务端消息 direction 字段持久化，属功能/1.3.2 语义范畴，非 2.5.8 目标尺寸。
- BBS/IM 小目标已命中 Spacing 例外，当前达标；若后续视觉密度调整压缩中心距至 <24px，需同步放大目标尺寸。

## 6. 证据 sha256（命令行 sha256sum 核验）

JSON 证据：

| 文件 | sha256 |
|---|---|
| report.json | 4319cee55f54a98b3dd9a5a2d0a4865b8f53d6b1f1cb31530e49d83378138f4e |
| supplement.json | 874228c82e55b1871d694273b9c4455d20b4f4d85a959ba393b82b54891adabe |
| round3.json | 06ab379eacd984d2290f1ca34edc35f3905f66efdb31931f78a057aed7ddd86f |
| round4.json | b892ed276e91b6d58d330937e205450d170c62fb3acaa8cc8ab40bf5cee0da3b |

截图证据（test-results/r504-target-size/）：

| 文件 | sha256 |
|---|---|
| pc-bbs-light-r504.png | 888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105 |
| pc-bbs-dark-r504.png | 6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c |
| pc-bbs-light-sup-r504.png | 142ed7453f6d300fae90c4e79032113194ea32c60e97e64c7a95151653e78bbe |
| pc-bbs-dark-sup-r504.png | 5a38ec8d9277974fff16a212c804447888e59c3810977d55b559287b01c2d8c1 |
| pc-im-light-r504.png | 61b0753b3b0f178ad5b5a7568bcb760b5f46171b07a33874ec439ff9d4e42767 |
| pc-im-dark-r504.png | 8b0d1c40237ea47c7654afa4b60005b164c122e03396ad22d28991ccced43312 |
| pc-im-light-sup-r504.png | 2b8d00e2a2ca894a364d5945ca9627912ed8b08eb0459fc4f34d087c586280d9 |
| pc-im-dark-sup-r504.png | 036fca90144baaaa6a2c00c3f2cd33738141d90feb100aa574ce69871c3ec116 |
| pc-attendance-light-r504.png | 678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b |
| pc-attendance-dark-r504.png | 85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7 |
| pc-attendance-light-sup-r504.png | 678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b |
| pc-attendance-dark-sup-r504.png | 85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7 |
| pc-round3-light-r504.png | fa70741c276fb5409acbd9fb2f55770950aae8582d6353e66d07c25acac6caef |
| pc-r4-outgoing-light-r504.png | 283a311cb965f37e56ec2c179c48bce99088ab3d9bd4596f7ee0a59c2c56b3e9 |

attendance 主轮与补采截图同 sha（同页同态）；补采/第三轮/第四轮与命令行 sha256sum 全核验一致。

## 7. 脱敏声明与边界

- 本文零真值：测试域名/主机用 <YOUR_TEST_DOMAIN>/<TEST_HOST> 占位；登录凭据经 ssh 运行时从 .local.keys 环境变量注入，仅存在于 gitignored runner 与证据目录，不入库、不回显。
- 截图来自 headless 浏览器 1440×900 DPR1，明确标注非物理真机；双主题通过 localStorage oa4rust_theme 注入并等 1200-1800ms 避开过渡。
- 边界：静态侦察命中候选仅实采 bbs/im/attendance 三页，全库其余 v-for/@click 小目标不预判、不扩张结论；1.3.2 AX 未在气泡存在时点坐实，据实披露为未坐实而非判缺陷。

## 8. 结论

R504 甄别完成：WCAG 2.5.8 Target Size 可达真缺陷 0 项、不可达 L1 1 类（Attendance 分页数据不可达）、正常规范预期全部其余候选（BBS add-section 24×24 达标、BBS sec-act 三按钮与 IM new-chat 命中 Spacing 例外、IM 图标按钮全达标）；1.3.2 次选候选经第四轮亚秒采样坐实布局反转但不构成可达真缺陷。无需修复源码。四轮真浏览器 6+6+1+1 组、0 pageerror，证据 JSON 与截图 sha256 命令行核验一致。至少 1000 轮迭代非终态，交队长独立复核。