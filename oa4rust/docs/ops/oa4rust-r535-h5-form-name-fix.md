# R535 H5 表单控件可访问名修复（WCAG 2.1 SC 4.1.2 / 关联 3.3.2）

- 轮次：R535（implementation，t35）
- 真父：68db97466895e422e088694eb39a56e54ac03bf0（台账 §526，已 push）；对比父 3d97bf229f8da13d1ebb5b8bff9023bd3aaccbe5
- 缺陷：据 R534 判定 1 项 P2（WCAG 2.1 SC 4.1.2 Name, Role, Value，Level A，关联 SC 3.3.2）——uni-app H5 把 placeholder 编译为 div.uni-input-placeholder 文本，未进入 AX name；calendar 与 bbs 发帖表单控件宿主层 AX name=null（真浏览器移动视口 CDP 坐实）。
- 修复：仅给 9 个控件源码开标签追加 aria-label（日历名称/日历选择/事件标题/开始时间/结束时间/地点/版块/标题/内容），业务零改。

## 一、写域（精确 4 文件）

| 文件 | 变更 |
| --- | --- |
| oa4rust/mobile/src/pages/calendar/calendar.vue | M：:301 日历名称 input、:306 picker、:314-317 事件 4 input 各补 1 处 aria-label（共 6 处） |
| oa4rust/mobile/src/pages/bbs/new.vue | M：:83 版块 picker、:91 标题 input、:95 内容 textarea 各补 1 处 aria-label（共 3 处） |
| oa4rust/mobile/src/pages/R535.h5FormName.contract.test.ts | A：6 用例源码级契约（2 核心 + 4 守卫） |
| oa4rust/docs/ops/oa4rust-r535-h5-form-name-fix.md | A：本文档 |

守卫钉死零改：:295 role=button + aria-label=新建日历 + @tap=toggleCalendarForm 正面对照、v-model / placeholder / @tap / @change / maxlength / picker handler、submitCalendar/submitEvent/canSubmit/onForumChange 业务函数、模板结构与样式、其他页面均逐字未改。无关未跟踪 augmentor/web/pnpm-lock.yaml 未入库。

## 二、契约 RED → GREEN

- 契约 6 用例：核心 2（日历 6 控件 + 发帖 3 控件 aria-label 精确文案）；守卫 4（正面对照按钮保留、日历 v-model/placeholder/@tap/picker handler 零改、发帖 maxlength/@change/@tap 零改、业务 handler 锚点零改）。
- 真父 3d97bf229 态 RED：RED_EXIT=1，2 failed（日历/发帖 aria-label 缺失）| 4 passed（守卫全过）。日志 oa4rust/test-results/r535/r535-red.log。
- 施加修复后 GREEN：6 passed（定点 V1 日志 r535/verify1.log 即 GREEN 实证，未另建 r535-green.log，不伪造）。

## 三、三 verify（全 exit 0，自行探索 H5 子项目命令）

任务描述中字面 verify 路径对 H5 子项目不可直接运行（oa4rust/mobile 未挂根级 pnpm test/typecheck 脚本），探索 package.json 后改等价命令，Fail Loud 披露如下：

| # | 任务字面 | 实际命令（workdir=oa4rust/mobile） | 结果 | 日志 |
| --- | --- | --- | --- | --- |
| V1 | H5 子项目测试命令 | pnpm test src/pages/R535.h5FormName.contract.test.ts（即 vitest run） | exit 0，6 passed | r535/verify1.log |
| V2 | H5 子项目测试命令（全量） | pnpm test（78 files / 514 tests） | exit 0，全绿 | r535/verify2.log |
| V3 | H5 子项目类型检查 | pnpm typecheck（tsc --noEmit） | exit 0，无报错 | r535/verify3.log |

mobile 子项目有独立 vitest.config.ts（environment=node，include=src/**/*.test.ts），test 脚本=vitest run，typecheck=tsc --noEmit；根 oa4rust 另有 test:mobile 过滤脚本。

## 四、真浏览器移动视口 CDP 实证（390x844 DPR2，isMobile/hasTouch，headless）

- Runner：oa4rust/test-results/r535-e2e.cjs（本地起 uni-app H5 dev 服务 pnpm dev:h5，BASE=<TEST_HOST>，API stub 为占位数据无真实凭据；编译产物为真实 Chromium 渲染）。
- 报告：oa4rust/test-results/r535/r535-ax-report.json，R535_E2E_DONE groups=2 / namesBad=[] / fatals=0 / pageErrors=0 / bizBad=0。
- light 与 dark 两组一致：10 个控件宿主层 AX name 全非空——新建日历（正面对照）+ 日历名称 / 日历选择 / 事件标题 / 开始时间 / 结束时间 / 地点 / 版块 / 标题 / 内容（与 R534 name=null 形成决定性对照）。
- 正面对照零回归：.calendar-create-btn（UNI-TEXT）DOM role=button aria-label=新建日历，AX partial/full role=button name=新建日历 ignored=false。
- 业务零改实证（全 true）：calendarFormOpened / composeOpened / placeholderIntact / submitBtnIntact / bbsPageReached / bbsFieldCount=3 / bbsPlaceholderIntact / bbsSubmitIntact；0 pageerror。
- H5 无主题切换：App.vue 无 oa4rust_theme / data-theme 机制，themeInfo.htmlTheme=null；light/dark 截图同 sha 印证，据实记录不伪造暗色差异（与 R534 口径一致）。

### Fail Loud：uni-app 原生内层 input 限制

诊断（r535-domdiag.cjs / r535-axdetail.cjs）dump 编译产物确认：uni-app H5 把源码 <input> 编译为 <uni-input aria-label="X"><div class=uni-input-wrapper><div class=uni-input-placeholder>X</div><input class=uni-input-input autocomplete=off></div></uni-input>。aria-label 落在宿主 wrapper（UNI-INPUT role=generic name=X, ignored=false），内层原生 input（role=textbox）AX name 仍为 null。查 @dcloudio/uni-h5 dist-x/uni-h5.cjs.js:5346-5393 createVNode("input",{...}) 硬编码属性集不含 ariaLabel，写域内（仅两 vue 模板补标记）无法让原生层获名。
决策：runner 与 R534 审计口径一致 probe 宿主层（R534 picker 亦按 UNI-PICKER 宿主判定），宿主层 name 非空即满足可访问名要求；原生内层 name=null 在报告 nativeLayer 字段据实披露，不隐藏、不伪称已修。此属框架封装硬限制，非本次模板标记可解；若要覆盖原生层需升级/改造 uni-h5 框架（出写域，另案）。

## 五、证据 sha256

| 产物 | sha256 |
| --- | --- |
| calendar.vue（修复后） | 75c2c2bdbbb1f1e78b5259749e278158164105fcb8512f94208f2977983d4838 |
| bbs/new.vue（修复后） | 9442b5f32bd8c4f055b0c21f7f927d409ec49c5846fb1e104e2941d1e991e190 |
| R535 契约 | 4f165d7e4c15f26671b7d6e450693517f903866175502561e7e4f1d9ba359ed4 |
| r535-e2e.cjs | dedfe1a603908644b166267dbb8b29c707eaf7e1ac39d20574db05285c6290a9 |
| r535-ax-report.json | b1ea78ecf8951d2dc9840afcefd5716b06979d869cd450adec429e0a3aad2094 |
| r535-red.log | dc0f1e61037ecf6c2c602d7b9dad30180d7ca00fb29cef2e91b984a5794ff1f3 |
| r535/verify1.log | 1dbf72dbb10577a61141ae94ce79960f409ec86c526bd32033c59ce03656f4db |
| r535/verify2.log | 30771d6966f74d87a55b929b3f6cddb7297317fb5446c98bb0b281c9dde19b49 |
| r535/verify3.log | 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92 |
| r535-calendar-light.png = r535-calendar-dark.png | 5f34261a9164ccce692af4a8ff5861d2d26fbefb2a2275ee4c12d8cafa9662f1（780x1687，112276B） |
| r535-bbs-light.png = r535-bbs-dark.png | f03dee35dddd696f7bedbba3fb349f4988b4ef91431db8e76cb02d5a6f8c66f0（780x1687，112027B） |

4 张移动视口截图（日历表单 / 发帖表单，两组主题）均 780x1687 目视正常：日历页日历名称/日历选择/事件标题/开始 2026-09-22 09:00/结束 2026-09-22 10:00/地点/创建日历/保存事件与发帖页版块/标题/内容/发布主题均渲染正常。runner/日志/截图/报告位于 .gitignore 排除的 oa4rust/test-results/，不入库。

## 六、脱敏

全文无密钥/口令/真实域名/IP/主机名/SSH 端口/本机绝对路径/个人目录；服务地址以 <TEST_HOST> 占位，API stub 为占位数据，0 真值入库。
