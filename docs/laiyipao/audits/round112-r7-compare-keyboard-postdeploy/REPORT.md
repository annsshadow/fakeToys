# R112 R7 对比弹层 dialog 键盘闭环 部署后复查报告

## 1. 范围与方法

- 对象：R112 在 skills/equipment 两页的 R7 收益对比弹层补 dialog 语义（role=dialog / aria-modal=true / aria-label）、ESC 关闭、Tab 陷阱（正向与 Shift+Tab 循环）、焦点移入与归还后的生产构建复查。
- 视口：414×896、820×1180、1440×900 三视口，真 Chromium（agent-browser 0.41.1）。
- 产物：H5 重建 bundle `index-BPIWOMRH.js`（index.html 引用 assets/index-BPIWOMRH.js），sha256 `3745ace0f73ed510276f20b17f9c3fc0fdedeade508c123e14ef1b0499f3b254`。后端/admin 沿用不回退。
- 机制：生产构建直接注入成功——复用 QA 的 init-script（data/r112-init.js，网络层 XHR.open/send 双 patch + fetch patch + localStorage 配置/占位令牌缓存）合成 config/推荐/impact/wallet/me 等响应，使收益对比 CTA 与弹层在生产构建自然渲染（不依赖 Vue 实例 setupState 注入，无需 dev server）。占位令牌 `r112-qa-placeholder-token` exp=2099，零真值。后端 8080 未起，全部为合成响应。
- 采集：每页 open(init-script) → 重设 viewport → trigger 打开弹层 → DOM 探针（cmpExist/role/ariaModal/ariaLabel/nFocus/cw/sw/hOverflow）→ AX snapshot → 两步确认探针 → 截图（在 ESC 前拍）→ focus-first → ESC 关闭探针 → focus-last → Shift+Tab 读 focusin 日志 → focus-first → 6×Tab 读 focusin 日志 → confirm-click。逐视口原始数据在 data/pd-driver.out，汇总在 data/pd-summary.json（ALL_OK=True，6/6）。

## 2. 三视口逐页证据（6 组全绿）

下表结论在 414/820/1440 三视口 × skills/equipment 两页共 6 组完全一致：

| 页面 | dialog 语义 | focusables | AX | 两步确认 | focus-first | ESC | focus-last | Shift+Tab | 6×Tab | confirm | 零横溢 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| skills | role=dialog / aria-modal=true / aria-label=当前 vs 推荐·确认替换 | 2（取消 / 确认替换） | dialog + button e6/e7 | previewState=true / doneVisibleBeforeConfirm=false | ok / inMask | cmpExist=false | ok / inMask / tabindex=0 | 1 条日志在 mask | 12 条日志全在 mask | clicked=true | cw=sw=W / isLogin=false |
| equipment | 同上 | 同上 | 同上 | 同上 | 同上 | 同上 | 同上 | 同上 | 同上 | clicked=true | 同上 |

- 6 组均为 cmpExist=true、role=dialog、ariaModal=true、ariaLabel=当前 vs 推荐·确认替换、nFocus=2、cw=sw=视口宽、hOverflow=false、isLogin=false。
- AX snapshot 坐实 `- dialog "当前 vs 推荐·确认替换"` 下挂 `button "取消" [ref=e6]` 与 `button "确认替换" [ref=e7]`；改前基线为 StaticText（引用 recon，未重做改前 A/B）。

## 3. ESC、Tab 陷阱与两步确认

- ESC：focus-first 命中弹层首个 .r7-btn（inMask=true）后 press Escape，probe 显示 cmpExist=false，弹层关闭。
- Shift+Tab：focus-last 命中末位 .r7-btn primary（确认替换，tabindex=0，inMask=true），Shift+Tab 后 focusin 日志 1 条落在 .r7-btn 取消（inMask=true），从末位循环到首位。
- 正向 6×Tab：focus-first 后连按 6 次 Tab，focusin 日志 12 条全部为 .r7-btn / .r7-btn primary 且 active.inMask=true，焦点不逃逸弹层。
- 两步确认：打开弹层时 previewState=true（确认替换按钮可见）但 doneVisibleBeforeConfirm=false（应用完成态未提前出现），坐实「打开仅预览、确认后才写入」；confirm-click 后 clicked=true。本轮触发的是失败分支（保留原配置请重试），成功写入分支未触发（诚实边界 §6）。
- 焦点归还：ESC 关闭与 confirm-click 后 activeTag=BODY、activeInMask=false（未坐实归还到触发 CTA），与 QA §诚实边界 L2 一致——H5 UNI-TEXT 的 focus() 不生效，由契约静态守，不冒充 PASS。

## 4. 门禁与 Go 守卫

- QA 独立复跑 G1-G6 全 RC=0：契约 18/18（12 缺口 + 6 不变量）、六项 a11y 回归、全量 vitest、vue-tsc、build:h5、git diff --check。本轮 postdeploy 独立复核 H5 重建 RC=0（DONE Build complete），契约源码 18it 独立复跑 18 passed(18)。
- Go 守卫：TestNoInfraIdentifiersInRepo PASS 8.22s(cached)；TestNoRealSecretsInRepo SKIP 非 PASS（secrets_leak_test.go:182 逐字「读不到 .local.keys —— 真值扫描这条守卫没有执行，不是通过了」），GO_RC=0。

## 5. 截图 sha256sum（6 张落盘实物实测，UNIQUE 6/6）

| 页面 | 414 | 820 | 1440 |
|---|---|---|---|
| skills | ccc37c162db215fd4f050216e3ad7a0d870b1f4f414ee3dc82e715290384b0b8 | 0827b56e7d6a2770a4877fab876069d675a121c746fadfb38bb483b703ec1e36 | a12a3a4d8050f357d4f4409b69296d745f15ae0ce5f10fdc465fc42990fbfa10 |
| equipment | 3bd9369002be570b4b819bc75bce91071603b68bb65b67c134fca70074870a83 | 9c06ef1011439a4770fd5d92b5dc4c55056cf0ea7bd9177001f7093845e72905 | be74f775c1fedfb09f7a71c2dc48b2bcdda94fc3fe5d22d5fe0a61dfdcacda49 |

文件名格式 pd112-<page>-<W>.png，存放 screenshots/。UNIQUE 6/6，与 QA 6 图（视口 390/768/1280）sha 交集 = 0。亲读 3 张非登录页：pd112-skills-414（技能图鉴对比弹层，取消/确认替换两按钮）、pd112-equipment-820（装备图鉴对比弹层，属性变化明细）、pd112-skills-1440（桌面技能图鉴弹层居中），视觉与 dialog 探针/AX 一致。

## 6. 诚实边界

1. 后端 8080 未起，全部响应由 init-script 合成（含 config/推荐/impact/wallet/me），占位令牌 r112-qa-placeholder-token 仅内存/localStorage 注入，零真值；非真实后端返回。生产构建可直接注入系网络层 XHR+fetch 双 patch，非 Vue setupState 注入。
2. 焦点移入/归还：focus-first/focus-last 坐实焦点进入弹层并在弹层内循环，但 ESC 关闭与 confirm 后 activeTag=BODY（未归还到触发 CTA），H5 UNI-TEXT focus() 不生效；由契约静态守，不冒充 PASS。
3. ESC 关闭须焦点先入弹层才成立；未验证焦点在弹层外时 ESC 的行为。
4. 成功写入分支未触发（推荐 API 为合成响应，confirm 后走失败分支保留原配置请重试），两步确认仅坐实「打开仅预览、未提前显示完成态」。
5. 改前基线引用 docs/laiyipao/recon/ 下 R112 侦察（改前 role=null / 弹层无 dialog 语义），本轮未重做改前 A/B。
6. 截图亲读 3/6；另 3 张（equipment-414/820/1440 中未亲读部分）以 sha 与 DOM 探针坐实。
7. Go TestNoRealSecretsInRepo 因 .local.keys 不存在 SKIP（设计如此），非真值扫描通过。

## 7. 结论与 hygiene

- R112 R7 对比弹层 dialog 键盘闭环在生产构建三视口真 Chromium 全部坐实：role=dialog / aria-modal=true / aria-label / AX dialog+button / ESC 关闭 / 正向与 Shift+Tab 不逃逸 / 两步确认预览态；改前无 dialog 语义与跳过缺口全部修复。焦点归还落 BODY 按诚实边界如实标注。
- 6 图 sha 实测 UNIQUE 6/6 且全不等于 QA 6 图；门禁全绿；令牌三锚点、零 --r112-、既有不变量、零横溢保持。
- hygiene：起点 HEAD=5bc72e4c012494aaa843f3bb501ad3b2603195db；五批提交见 git log；auth 七路径全 ABSENT；.gitignore=697 行（R112 三规则 695-697）；tracked 本轮 postdeploy/data=0；零真值；自启 job 全部回收。
判定：PASS（含上述诚实边界）。