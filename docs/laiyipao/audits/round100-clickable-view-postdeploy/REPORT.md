# R100 可点击交互行键盘补漏 — 部署后真浏览器复查报告（B5）

## 1. 范围与结论

- 本轮为 R100「可点击交互行键盘补漏」五批发布的第五批（B5）：重建 H5 产物后，在生产静态构建上以三视口真 Chromium 复查九处交互行的键盘可达性。
- 覆盖页面/选择器：首页 index 的 .ann-item、.action-card、.grid-item、.mg-row、.elem-chip；战报页 battles 的 .battle-row；敌人图鉴 enemies 的 .resist-toggle；关卡页 stage 的 .chapter-chip（未自然渲染，见 §7 诚实边界）。
- 结论：**PASS**。三视口 414×896 / 820×1180 / 1440×900 各四页共 12 组回归，九处已渲染交互行全部 role=button、tabindex=0、cursor=pointer、真 Tab 命中、焦点环 solid rgb(255,107,53)、iw=cw=sw 零横向溢出、非登录页；21 组 Enter 送达验证（7 处 × 3 视口）全部 isCta=true 送达预期交互行。

## 2. 五批链与部署产物

- B1 feat：a17c71bf9452734485056eca29272be82462c5e7（四页 vue 九处交互行补键盘三属性）
- B2 test：fed814a58e65f9861312edf55227a75e85a69e53（契约 r100ClickableView.contract.test.ts，8 CASES + 4 不变量）
- B3 docs：dadf1592872c0f75955134827219036718d60246（SPEC + RED-evidence + 独立终审 verdict=pass）
- B4 audit：5f0a7e58764d684e6961c3ff14c6a58f62787109（.gitignore 修正 + QA 报告 + 12 张 QA 截图）
- B5 本报告：以最终 git log 五批链为准。
- H5 主 bundle：assets/index-CNP3_F9X.js，sha256=3f8f9ebea254306cb1e1e533f6b5a471b7db6168ba43a21a2602756c30333d1f（index.html:21 引用）。
- 后端产物沿用前轮不回退。

## 3. 部署后三视口真浏览器回归（12 组）

生产静态构建经 python -m http.server 5201 提供，init-script r100-init.js（占位令牌 r100-qa-placeholder-token，非真值）注入合成 config/公告/战报/敌人数据。以 r100-probe.js 取属性与尺寸、r100-hit.js 逐次 Tab 判定命中、r100-focus.js 取焦点环。

| 视口 | 页面 | 交互行 n(role/tabindex) | Tab 命中次数 | 焦点环 | 尺寸 iw/cw/sw | 溢出 | 非登录 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 414 | index | ann=2(button/0) action=1(button/0) grid=15(button/0) mg=3(button/0) elem=5(button/0) | 28 | solid rgb(255,107,53) | 414/414/414 | false | true |
| 414 | battles | battle-row=1(button/0) | 19 | solid rgb(255,107,53) | 414/414/414 | false | true |
| 414 | enemies | resist-toggle=1(button/0) | 12 | solid rgb(255,107,53) | 414/414/414 | false | true |
| 414 | stage | chapter-chip=0（未自然渲染，见 §7） | 0 | — | 414/414/414 | false | true |
| 820 | index | ann=2 action=1 grid=15 mg=3 elem=5 全 button/0 | 28 | solid rgb(255,107,53) | 820/820/820 | false | true |
| 820 | battles | battle-row=1(button/0) | 19 | solid rgb(255,107,53) | 820/820/820 | false | true |
| 820 | enemies | resist-toggle=1(button/0) | 12 | solid rgb(255,107,53) | 820/820/820 | false | true |
| 820 | stage | chapter-chip=0（未自然渲染） | 0 | — | 820/820/820 | false | true |
| 1440 | index | ann=2 action=1 grid=15 mg=3 elem=5 全 button/0 | 28 | solid rgb(255,107,53) | 1440/1440/1440 | false | true |
| 1440 | battles | battle-row=1(button/0) | 19 | solid rgb(255,107,53) | 1440/1440/1440 | false | true |
| 1440 | enemies | resist-toggle=1(button/0) | 12 | solid rgb(255,107,53) | 1440/1440/1440 | false | true |
| 1440 | stage | chapter-chip=0（未自然渲染） | 0 | — | 1440/1440/1440 | false | true |

## 4. Enter 键送达验证（21 组 + 3 组诚实边界）

以 r100-observer.js 监听 keydown（记录 isCta）并 patch navigateTo/redirectTo，用 QA 各 focus-*.js 聚焦目标后按真实 Enter。结果：ann/action/grid/mg/elem/battle/resist 七处 × 三视口共 21 组全部 `enterCta=1`（keydown 目标为 CTA）。导航效果：action→#/pages/stage/stage?level=1、grid→stage、mg→equipment、elem→reactions 四处真实跳转；battle/resist 为 DOM 展开/事件送达；ann 为公告行事件送达（不改变 hash，属既有业务语义）。stage 的 chapter-chip 三视口均 n=0 无法聚焦，Enter 落在非 CTA（isCta=0），如实归入诚实边界，由契约静态断言保证。

## 5. 部署后截图与 sha256（12 张，UNIQUE 12/12 且全 ≠ QA 12 图）

| 截图 | sha256 |
| --- | --- |
| pd100-index-414.png | f6ae0c310b00c660aa0e15c4dea1537c7b75cd8707273907993dfebb444aa5ba |
| pd100-battles-414.png | 938615cb3c8d8c4f70c78ea352b099a333b6cba94969f96cfb15474b016c141d |
| pd100-enemies-414.png | 79bf5a4ca8ff7d0c725a1a609f88c6b1f53ca106a7bd71e8ad3f6becda854f14 |
| pd100-stage-414.png | 3c8162f783bbba88e1ee70a67df0f3295ecefefb1dc52981f1b07d522a761245 |
| pd100-index-820.png | 91dfcbf9cf5ac26c72165ca7954ba4fdd61c725f0dcf8afcb4e9b8ab62de17c1 |
| pd100-battles-820.png | 3b179fa3c40c6a378e2b0eb3fe4e716aaf70543a2865a0d57be912c809a8d260 |
| pd100-enemies-820.png | be029e806507809bd5f00d0f7e5aea796de65218aff36569819df4afcba11ffd |
| pd100-stage-820.png | afa864972c6f3d138e0513f4b5be7d68e1232e5ae81dfd60e3982c0ff02bb53b |
| pd100-index-1440.png | b9cf47d6de62d950c8bafa21df3fd5b0ec8d2ff65b4e03a1760a8fb9668e5983 |
| pd100-battles-1440.png | beb73116932fb76bed25498d811bbd5f92ec653e851eea45686aea6b116ad1cc |
| pd100-enemies-1440.png | 6bdf92a6d0093d1fb393cb5a1e532ffa1886c75aa65e1971bf38d69be14f97e8 |
| pd100-stage-1440.png | 12901e9cedb4cf0223ce5b7596cf1c9a764715b5fb588e08ab1dc70539d709da |

已逐张 sha256sum 核验与上表一致；12 张互不相同，且与 QA 阶段 12 张（r100-*-390/768/1280）首段 sha 全部不同。亲读 6 张（index-414、battles-414、enemies-414、index-820、battles-820、enemies-820），均为真实业务页（来一炮首页/我的战报/敌人图鉴），非登录页。

## 6. 门禁与回归锚点

- Go 门禁：`go test ./internal/domain -run "TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo"` EXIT=0。TestNoInfraIdentifiersInRepo PASS 8.22s；TestNoRealSecretsInRepo **SKIP 非 PASS**（secrets_leak_test.go:182 逐字：读不到 .local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」）。
- 静态回归锚点：tokens.css 三版本锚点在；业务源码零 --r100-（仅契约否定断言）；九处交互行 role=button/tabindex=0/@keydown.enter 三属性经契约 12/12 与全量 232 files passed(1 skip)/2132 tests(2 skip) 坐实。
- QA 库只读：docker exec lyp-qa-pg psql -U postgres -d laiyipao → users=853、battle_records=104（未写库）。
- 数据 hygiene：tracked 本轮 data=0；auth 五路径全 ABSENT；.gitignore 646 行含 R100 四行（643-646），本目录 data/ 被 :645 实际拦截（git check-ignore RC=0）。

## 7. 诚实边界与环境偏差

- stage .chapter-chip 三视口自然渲染 n=0（关卡页无已解锁章节 chip 数据），无法在真浏览器观测其 Tab/Enter，按 QA 与终审一致口径由契约静态断言保证；本报告不将其计入 21 组送达样本。
- 契约要求代理 127.0.0.1:12450，实测该代理未监听（curl -x 返回 000/RC=7）。按环境偏差改直连 push（env -u 清代理 env + git -c http.version=HTTP/1.1），如实记录，非流程省略。
- 首轮 Enter driver 因多 agent-browser 并发冲突输出全空作废已 kill；改用单 driver 独占重跑，单组 POC 与 24 组 v2 结果一致，未将作废数据计入结论。
- 部署后 data/ 下采集脚本与中间文本被 .gitignore 拦截不入库；B5 仅提交本报告与 12 张截图。
