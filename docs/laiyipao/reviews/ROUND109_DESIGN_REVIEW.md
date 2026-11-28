# R109 AI 优先级 CTA 键盘可达性 独立终审报告

- 轮次：R109（reviewedTask=t314 实施：9 文件 10 改点补键盘三件套）
- 终审角色：same-route-reviewer（独立终审，继承队长路由）
- 基线 HEAD：652e237fbdf5ce38460f1f159e5b48a6ec9c7646（全程未变）
- 验收契约：六维（①RED/GREEN 诚信 ②diff 精确 ③QA 证据 ④键盘闭环 ⑤门禁独立复跑 ⑥不变量卫生）
- 裁定：**PASS（verdict=pass）**——六维全绿，无 blocker/high/medium，8 条 low 不阻断，同意分批发布。

## 维度① RED→GREEN 诚信

- 契约 `laiyipao/miniapp/src/styles/r109PriorityCtaKeyboard.contract.test.ts`（114 行/5599B/rev=5ec228ff…）真实 `fs.readFileSync` 读 10 页 vue，按 class+@click handler 双锚点定位开标签单行，不依赖行号。
- RED 证据：`docs/laiyipao/specs/round109-priority-cta-keyboard/RED-raw.log` 逐字锚点——第31行『⎯⎯ Failed Tests 10 ⎯⎯』、第194行『Test Files  1 failed (1)』、第195行『Tests  10 failed | 6 passed (16)』、第197行 Duration 310ms；10 个 FAIL（行33/49/65/81/97/113/129/145/161/177）全为各自缺口 role=button 缺失，无 PARSE_ERROR，6 不变量全过。RED-evidence.md（36 行/2130B）。
- 独立复跑 GREEN：`npx vitest run src/styles/r109PriorityCtaKeyboard.contract.test.ts --reporter=verbose` exitCode=0，逐字『Test Files 1 passed (1) / Tests 16 passed (16) / Duration 394ms』（vitest v4.1.12），10 缺口 it + 6 不变量 it 全过。**RED 真红、GREEN 真绿，诚信成立。**

## 维度② diff 精确

- `git diff --numstat`：.gitignore 4+/0-、bag.vue 2+/2-、其余 8 页（battles/enemies/equipment/gems/me/rank/skills/skins）各 1+/1-；合计 10 文件 14+/10-，其中业务改动 10+/10-。任务描述『11+/11-』系笔误，SPEC §2 实列 10 处。
- 10 改点全部同一模式：给 `<view>` 追加 `role="button" tabindex="0" @keydown.enter="同handler"`，@keydown.enter 与 @click handler 逐字一致（r12Go()/r4GoBag()/r4GoBattle()/r4FocusEnemy()/r4GoMe()/r4GoRank()/r4FocusSkin()）；无死变量、无无关改动。
- .gitignore 4 行为 R109 数据目录 ignore 规则，属上游预置卫生。SPEC=`docs/laiyipao/specs/round109-priority-cta-keyboard/SPEC.md`（61 行/4352B/rev=ff83a42c…），§2 改点表行号与 diff 逐处吻合；§0 权威锚点 AI_VISUAL_SYSTEM.md:137 CTA 键盘可达 + WCAG2.4.7 焦点可见、:926 底线、:66 可访问性优先、:120 卡内 CTA 焦点 + WCAG2.1.1。**diff 零多余，吻合 SPEC。**

## 维度③ QA 证据（27 图 sha 独立重算 + 亲读）

- QA 报告 `docs/laiyipao/audits/round109-priority-cta-keyboard/REPORT.md`（101 行/9397B/rev=9df73840…）：三视口 390×844/768×1024/1280×800 真 Chromium（agent-browser 0.41.1），后端 8080 未起用 init-script 合成响应 + 占位令牌（仅内存零真值），补全数据依赖使 10 处 CTA 自然渲染。
- 截图实际目录 `docs/laiyipao/audits/round109-priority-cta-keyboard/screenshots/`（9 页×3 视口=27 张；data/ 由 .gitignore:683 拦）。用 `ops_stat` revision 对 27 张逐字重算，与 QA 报告 §6 sha 表 **MATCH 27/27，UNIQUE 27/27**。关键：r109-gems-768=10ab3c2359e0506b6f1b6e8d7f0c50f2821f52d0d42796b1dcf846938f52660d、r109-battles-1280=4f92fe42b689fbdf0b9cc9e065e4edd4e10dc17275196f13af9d80cc55a37dde、r109-bag-390=bd41b964353137f39005c8623812dfd71b9f7577f33d76e13ddd51873ee5ab95。
- 亲读 2 张非登录页（tools.read_image，非空态）：r109-gems-768（宝石图鉴 AI 建议卡『去商城找宝石→』『去背包镶嵌→』两枚 CTA + AI 佩戴收益卡 + 搜索栏）、r109-battles-1280（我的战报战绩总账 1 场/30 击杀/3 漏怪/1200 均分 + 关卡胜率 100% + 星级分布 + 交锋热度 + 战斗效率趋势）。视觉与 DOM 探针一致。

## 维度④ 键盘闭环

- 原始探针 data/w-390.txt gems 390：r12-priority-cta（UNI-VIEW『去配装备 →』role=button tabindex=0 tabIndexProp=0 w338 h48）+ 2×r3-ai-cta（UNI-TEXT role=button）；AX 坐实三枚 button（改前 StaticText）；真 Tab 序列 8 条交替命中 3 枚 CTA + INPUT；CLICK 与 ENTER 同为 navigateTo /pages/equipment/equipment，ENTER keys 含 isCta:true。
- 我独立抽查 w-768/w-1280：gems/skills/equipment/bag 各页 CTA 均 role=button/tabIndexProp=0、真 Tab 命中、CLICK=ENTER 同 URL；equipment 自指页内强化 nav=[] 且 ENTER keys isCta:true。三视口一致，零横溢（cw=sw=hviewport）。
- 自指分支源码坐实：equipment.vue:270 `if(r12Cta.value.self)`；enemies.vue:273 `counterElement.value = r4AiEnemy.value.target`；skins.vue:262 `keyword.value = r4AiSkin.value?.keyword ?? ''`；battles.vue:881 `r4GoBattle()` 展开战报——四处 click/Enter 均 nav=[]（页内定位）。
- 焦点环：真 Tab 聚焦橙色焦点环 active=true 三视口一致；390/1280 outline solid 1px rgb(255,107,53) offset1px（3rpx 随 DPR 缩放），768 solid 3px offset2px，颜色均 --accent；common.css:914-921 两层 `:where(a,button,input,textarea,select,[tabindex]):focus-visible` + 通用 `:focus-visible` 未回退。text 版 r3-ai-cta 9 处保留不回退。**键盘闭环成立。**

## 维度⑤ 门禁独立复跑

- 自建脚本 `.t316-review-gates.sh` 后台独立复跑（输出 `.t316-review-gates.out` 7739 行末 REVIEW_GATES_DONE），逐项 RC：
- G1 r109 契约 16/16 RC0；G2 五 a11y 契约（r100/r102/r104/r96/r60）48/48 RC0；G3 全量 vitest『Test Files 240 passed | 1 skipped (241) / Tests 2225 passed | 2 skipped (2227)』RC0；G4 vue-tsc RC0；G5 build:h5（uni build）RC0；G6 git diff --check RC0（.gitignore LF→CRLF warning 非错）；G7 Go 守卫 `go test ./internal/domain/ -run TestNoRealSecretsInRepo -v` RC0——但因 `.local.keys` 不存在输出『--- SKIP: TestNoRealSecretsInRepo』，**如实记 SKIP，非 PASS**。
- 与 QA gates-out.txt（7741 行）G1..G6 结果一致。全门禁真绿，唯一 skipped 为设计如此。

## 维度⑥ 不变量卫生

- 令牌三锚点 tokens.css:14 `--lyp-token-version: lyp-tokens-v1` / :15 `--lyp-r3-version: lyp-tokens-v3-r3` / :16 `--lyp-r6-version: lyp-tokens-r6`（ops_grep 命中）全部未动。
- 业务源码零 `--r109-`（ops_grep 对 laiyipao/miniapp/src 全量命中 0；仅契约自身 2 处断言）。既有不变量 R60/R96/R99/R100/R102/R104 由 G2 五契约 48/48 守住，无回退。
- HEAD=652e237fbdf5ce38460f1f159e5b48a6ec9c7646 全程未变；`git status --porcelain` 非 ?? 项仅 `.gitignore` + 9 个预期 vue（t314 实施），其余全为历史遗留 ?? 项非本轮产出；auth 五路径（.env / admin,deploy,server 的 .local.keys / miniapp .env.local）全 IGNORED/ABSENT；`git ls-files` 无任何 .env/.local.keys/remote-config.env/.pem/.key 真值文件被跟踪；报告零真值。

## 问题分级（8 条 low，不阻断放行）

1. L1：任务描述『11+/11-』为笔误，实际业务改动 10+/10-（SPEC §2 仅 10 处）——口径瑕疵，不影响实现。
2. L2：后端 8080 未起，全部响应 init-script 合成 + 占位令牌内存注入，非真实后端返回；但 CTA 属性/焦点/handler 与数据来源无关。
3. L3：click/Enter 同 handler 用 stub 拦截 navigateTo/pageScrollTo 记录 URL 验证，未做跳转后 DOM 断言。
4. L4：AX snapshot 仅 gems 页三视口全量抓取，其余页以 DOM role/tabindex/tabIndexProp + focusin 焦点日志坐实。
5. L5：改前基线引用 ROUND109_A 侦察（改前 role=null/tabIndex=-1/AX StaticText），本轮未重做改前 A/B。
6. L6：焦点环线宽 390/1280 为 1px、768 为 3px，系 3rpx 随 DPR 换算，颜色一致，非缺陷。
7. L7：G7 Go 守卫因 .local.keys 不存在 SKIP（设计如此），非真值扫描通过；已在维度⑤如实标注。
8. L8：G3/G4 中『[来一炮] build.attacker 缺失』为测试既有 stderr 告警，非本轮引入；G6 .gitignore LF→CRLF warning 非错。

## 终审裁定

六维验收全部满足，无 blocker/high/medium。RED→GREEN 诚信、diff 精确、27 图 sha 独立一致、键盘闭环三视口坐实、门禁 G1-G7 全 RC0（Go 守卫 SKIP 如实）、不变量与卫生全绿。8 条 low 均为诚实边界或既有告警，不阻断。**verdict=pass，同意分批发布。**

## hygiene

- HEAD 未变；非 ?? 项仅 .gitignore + 9 vue；27 图 sha UNIQUE；报告零真值；临时脚本 `.t316-review-gates.sh` / `.t316-review-gates.out` 终审结算后清理；无自启后台 job 残留（门禁 job bash-e922519b6db24eb1 completed）。
