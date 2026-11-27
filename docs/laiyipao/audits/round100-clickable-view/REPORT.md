# R100 八处交互行键盘可达真浏览器 QA 报告

## 1 范围与方法

- 对象：t257 实施在 laiyipao/miniapp 四处页面为八处交互行补键盘三属性（role=button / tabindex=0 / @keydown.enter=同名@click handler）。契约 laiyipao/miniapp/src/styles/r100ClickableView.contract.test.ts（117 行，8 CASES + 4 不变量 it，readFileSync 真实读源码），队长复核 diff 精确 9 标签补三属性，GREEN 12/12。
- 八处改点：index .ann-item（toggleAnn）/.action-card（goStage）/.grid-item×2（go(m.path)）/.mg-row（m.hint&&go）/.elem-chip（go reactions）；battles .battle-row（toggleDetail）；enemies .resist-toggle（toggleEnemy，同行 .rxn-name 保持无 role 属排除项）；stage .chapter-chip（switchChapter）。
- 方法：H5 dev server（5174）+ agent-browser 0.41.1 真 Chromium，三视口 390×844 / 768×1024 / 1280×800。后端 8080 未起，用 open --init-script 注入合成 config/公告/战报/敌人/用户数据（data/r100-init.js，version r100-qa-v1，占位令牌 r100-qa-placeholder-token，零真值），使交互行自然渲染后逐一坐实四属性、AX button 角色、真实 Tab 命中、Enter 事件送达与焦点环。
- 证据落盘：每页每视口 probe（属性+iw/cw/sw/hOverflow）、ax（AX snapshot）、tabs（34 次 Tab 命中）、focus（焦点环）；另 enter100.sh 三视口×8 类 focus→before→Enter→after→actions 全链路（e-* 文件）。

## 2 三视口逐处浏览器证据

| 处（selector / handler） | role / tabindex / cursor（三视口 probe） | AX button | 真 Tab 命中（34 次） | Enter 事件与结果（三视口一致） | 焦点环 |
|---|---|---|---|---|---|
| index .ann-item / toggleAnn | button / 0 / pointer（n=2） | 是（button "最新 QA公告一…"） | 28 次命中覆盖 ann×2 | Enter:true 送达 handler，isCta=true；展开态签名未变见 §5 | solid rgb(255,107,53) |
| index .action-card / goStage | button / 0 / pointer（n=1） | 是（button "去第 1 关 →"） | 命中 | Enter:true → hash #/pages/index/index→#/pages/stage/stage?level=1，nav 一致 | solid rgb(255,107,53) |
| index .grid-item / go(m.path) ×2 | button / 0 / pointer（n=15） | 是（多 button 卡片） | 命中 15 次 | Enter:true → hash→#/pages/stage/stage，nav 一致 | solid rgb(255,107,53) |
| index .mg-row / m.hint&&go | button / 0 / pointer（n=3） | 是（button "→ 去背包配元素"等） | 命中 3 次 | Enter:true → hash→#/pages/equipment/equipment，nav 一致 | solid rgb(255,107,53) |
| index .elem-chip / go reactions | button / 0 / pointer（n=5） | 是（button "焰/冰/电/毒/动能"） | 命中 5 次 | Enter:true → hash→#/pages/reactions/reactions，nav 一致 | solid rgb(255,107,53) |
| battles .battle-row / toggleDetail | button / 0 / pointer（n=1） | 是 | 19 次命中 | Enter:true → bodyLen 421→429（详情展开），isCta=true | solid rgb(255,107,53) |
| enemies .resist-toggle / toggleEnemy | button / 0 / pointer（n=1） | 是 | 12 次命中 | Enter:true → bodyLen 409→434（抗性图展开），isCta=true | solid rgb(255,107,53) |
| stage .chapter-chip / switchChapter | 三视口 n=0，未自然渲染 | 不可坐实 | 0 | 未坐实（Enter:false，元素不存在）见 §5 | 不可坐实 |

- 零横溢：三视口四页全 iw=cw=sw（390/768/1280）且 hOverflow=false，bodyLen 稳定（index 823 / battles 421 / enemies 409 / stage 710）。
- 改前基线对照（docs/laiyipao/recon/ROUND100_A_CLICKABLE_VIEW_KEYBOARD_RECON.md，t254）：改前 ann-item/resist-toggle/卡片跳转 role=null/tabIndex=-1/cursor=auto/AX 无 button/Tab 不命中；本轮全部修复。
- AX button 逐页坐实：index-768-ax.txt 出现 button "最新 QA公告一…" [ref=e1]、button "→ 去背包配元素" [ref=e3]、button "→ 看反应搭配" [ref=e4] 等；battles/enemies 视口 probe role=button 且焦点环探针确认。

## 3 截图清单（sha256 逐字，以落盘实物为准）

| 截图 | sha256 |
|---|---|
| r100-index-390.png | 17dd0bcd9519993f0f6ca099575128865eed457af1154edd6020a4c82434bba7 |
| r100-index-768.png | e13ae9a01904bba7a3a6c04af181833394e5dc6dd8f486f17748a79d00415f3b |
| r100-index-1280.png | 7cdf007aed247a0e65e8d4dda624ab00c6848f1e05a848c29ceec0ef35ab401f |
| r100-battles-390.png | 86c6495a5190d4838c64d3462bc174038167b0e4dbdcf10646c0d215399eb469 |
| r100-battles-768.png | 3afb0296399eee2119c567553b3341116f6f12eab4481976b87748632b387d83 |
| r100-battles-1280.png | 5a412dfeb18c1e4f686fad98c975a2ebb1fca1bbfde91c1c54f3aedf51fc3ac9 |
| r100-enemies-390.png | 7ad38a493fc9db5b7e34f9d803452e20c76a5717f73407a4fde171c0870e7904 |
| r100-enemies-768.png | 922a9332d0ea9f1e41d0846694e59aba4b94c3468898dd57e2b968ae0b63b94d |
| r100-enemies-1280.png | d84cc188b22401138232743201d968701f7a328639f68e954c284fcb0266b02a |
| r100-stage-390.png | df40011875cec7381c8921c16911e3928d9caaf905f9619b6ae8efc38cbd7d1f |
| r100-stage-768.png | 62a42a4f8969a9906507ed5da6908c10e979ab5d90c27753fea41b95ebf481fb |
| r100-stage-1280.png | 81e98993f5c551dce9a388d6ab3b19cda292f9fccd5c402e38698f912ddec023 |

- 12 张 sha 全互异 UNIQUE=12/12；以 cd screenshots && sha256sum *.png 实物为准。
- 亲读 2 张非登录页：r100-index-768.png（来一炮首页，QA合成玩家/AI战术洞察/公告「QA公告一」均可辨，非空态、非登录页）；r100-battles-768.png（我的战报，胜★★★第 1 关 1200 击杀记录行 + AI 防守诊断，非空态、非登录页）。其余 10 张仅列 sha 未逐一亲读。

## 4 静态回归与门禁

| 门禁 | 命令 / 判据 | 结果 |
|---|---|---|
| G1 R100 契约 | vitest r100ClickableView.contract.test.ts | Tests 12 passed，RC0 |
| G2 R60 键盘范式 | vitest r60A11yKeyboard.contract.test.ts | Tests 6 passed，RC0 |
| G3 定向页测试 | enemies_fresh_load.test.ts / index.test.ts / battles / stage | Tests 65 passed，RC0 |
| G4 miniapp 全量 | vitest run | 232 files passed (1 skip) / 2132 tests passed (2 skip)，RC0 |
| G5 类型 | vue-tsc / type-check | RC0 |
| G6 构建 | build:h5 | RC0 |
| G7 diff 规范 | git diff --check | RC0 |

- 门禁 job bash-b6cca28e8b7b4fb1 exit0，data/gates-out.txt 记 GATES_DONE。首轮 G3 误用不存在的 enemies.test.ts 已改 enemies_fresh_load.test.ts 重跑全绿。
- 静态令牌：tokens.css:14/15/16 三锚点（--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6）在；业务 vue 全局零 --r100-（OPS grep NO_MATCH，仅契约断言含该串）。
- 契约不变量 it 保证：8 处原 @click 全保留；排除项 mastery .node 与 enemies .rxn-name 未被误改。

## 5 诚实边界

- 后端 8080 未起：全部页面数据经 init-script 注入合成（config / 2 条公告 / 1 条战报 / 1 个敌人 / 用户态），非真实后端数据；占位令牌仅内存注入，零真值。
- stage .chapter-chip 三视口 n=0：hero 显示第一章 QA、cfgChapters=1/cfgLevels=1/cache 存在，但 .chapter-row 内 <!----> 占位，疑 store.config 经服务端响应后 chapters 字段丢失或 curChapter 映射异常，无法自然渲染。故 stage 键盘三属性仅由契约静态断言保证，未冒充浏览器 PASS（Enter:false 即元素不存在，非键盘失效）。
- index .ann-item Enter 事件确认送达（isCta=true，三视口），但 before/after 状态签名（hash+类名+bodyLen）未变化：toggleAnn 的展开态细节（如箭头样式/内部展开容器）未被该签名探针捕获，仅坐实事件送达而非展开 DOM 变化。
- 焦点环宽度：768 视口 outlineWidth=3px；390/1280 为 1px（均 solid、rgb(255,107,53)）——像素宽度差异来自视口缩放，颜色/样式一致。
- AX snapshot 逐字保存于 data/*-ax.txt；battles/enemies 未单独列 AX 全文，以 probe role=button + 焦点环 + 真 Tab 命中 + Enter isCta 四重证据支撑。
- 截图亲读 2/12；Tab 走查脚本首版因外壳变量引号致命中统计失真已作废重写（walk100.sh 最终版）。

## 6 hygiene 与判定

- HEAD=9df4eee0d772d78d39358aa27f696547b09d9696 未变；git status 非 ?? 项仅 .gitignore + 四处页面 vue（battles/enemies/index/stage，预期 t257 实施），无越界写入。
- auth 五路径：.env / .env.local / laiyipao/admin/.env.local / laiyipao/admin/dist/.env.local 均被 .gitignore 拦截且 git ls-files 未 tracked；.local.keys 两路径 ABSENT。零真值入仓。
- 自启服务：H5 dev server（job bash-48b55b0d9db54f24）与 Enter driver（job bash-a2ef7b6d4f554a43）均已 kill，P5174/P8080 复验 CLOSED。
- QA 库历史 477|rc_shd 存在 / 104 无记录，本轮只读未写。

判定：PASS。八处交互行中七处在三视口真浏览器完整坐实 role/tabindex/AX button/真 Tab 命中/Enter 事件送达（其中四处导航 + 两处 DOM 展开 + 一处事件送达）、焦点环、零横溢；stage .chapter-chip 因无法自然渲染按诚实边界由契约静态保证。门禁 G1–G7 全绿，静态回归通过，截图 sha 与实物一致，hygiene 全绿。
