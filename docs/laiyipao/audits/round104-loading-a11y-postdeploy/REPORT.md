# R104 骨架屏加载态 a11y 语义 · 部署后真浏览器复查报告

- 任务：t286（R104 五批精确发布，attempt bd03a304）
- 轮次：round104-loading-a11y-postdeploy
- 视口：414×896 / 820×1180 / 1440×900（真 Chromium，agent-browser 0.41.1）
- 起点基线：b4e0194ca46de1bfe022315b289ee5fc9e6a0a9c（发布链见 git log）
- QA 源报告：docs/laiyipao/audits/round104-loading-a11y/REPORT.md（PASS，三视口 390/768/1280）
- 终审：docs/laiyipao/reviews/ROUND104_DESIGN_REVIEW.md（verdict=pass，六维全绿，7 条 low 不阻断）
- H5 产物：index-w89sIeTZ.js（index.html 引用 assets/index-w89sIeTZ.js），sha256=2546624aca71cc95900340854e3bbe645b15699aed7f887f3c4cbbac8ddd3d5e；后端/admin 无变化沿用不回退。

## 1. 范围与方法

本轮部署后复查覆盖 R104 十处加载语义改点：A 类 7 页（announcements / bag / battles / mastery / rank / shop / signin）加载根 role=status + aria-busy=true + aria-label=数据加载中；B 类 3 页（equipment / reactions / skills）骨架根去 aria-hidden 并补同三属性。

生产静态构建（dist/build/h5，5201）先做 POC：注入脚本 __r104Set 在生产 bundle 上返回 no-instance-with-r6Loading（生产构建 script setup 的 setupState 闭包化，外部无法遍历 __vueParentComponent 取得业务响应式变量），与 R103 同一根因，无法复现加载态。故部署后 DOM/AX 复查改用 dev server（5174）复现 QA 同一注入路径；生产 bundle 仅做构建产物 sha256 核对。

方法：每页 close --all → set viewport → open dev server 页面 → 注入合成 config/令牌（占位令牌 r104-qa-placeholder-token，exp 2099，零真值）→ eval set-loading.js 注入 loading/r6Loading=true → DOM 探针 probe.js → 截图 → eval unset-loading.js → DOM 探针坐实卸载。AX 用完整 snapshot --json 在 ann/equip 两页坐实 status 节点（-i 交互快照会过滤 status，不作证据）。

## 2. 三视口 × 10 页加载期 DOM 三属性（全 PASS）

三视口 414 / 820 / 1440 各 10 页，加载期每页 aria-busy=true 根恰 1 个：UNI-VIEW、role=status、busy=true、hidden=null、label=数据加载中；iw=cw=sw 视口宽、hOverflow=false、isLogin=false；B 类 3 页 skelCount=1（骨架根），A 类 7 页 skelCount=0。30/30 组全部满足，set-loading 注入成功。

| 视口 | A 类 7 页 | B 类 3 页 | 横溢/登录 |
|---|---|---|---|
| 414 | 各 busyCount=1，role=status/busy=true/label=数据加载中/hidden=null | 同三属性 + skelCount=1 | iw=cw=sw=414，hOverflow=false，isLogin=false |
| 820 | 同上（逐页实测一致） | 同上 | iw=cw=sw=820，零横溢，非登录页 |
| 1440 | 同上（逐页实测一致） | 同上 | iw=cw=sw=1440，零横溢，非登录页 |

数据来源：data/pd-w-414.txt / pd-w-820.txt / pd-w-1440.txt 每页 --LOAD_PROBE-- 段，解析脚本汇总 30/30 全绿。skills 页处在文件末段，其卸载探针实际在原始 txt 中 busyCount=0/skelCount=0（解析器正则末段边界误判为 None，已回读原始段确认），非真实缺陷。

## 3. 卸载后加载语义从 DOM 消失（全 PASS）

unset-loading 设 loading/r6Loading=false 后，三视口 30 页 --UNLOAD_PROBE-- 全部 busyCount=0 且 skelCount=0：加载根 v-if 卸载后 role=status/aria-busy 节点确实从 DOM 消失。部分页面保留既有空态 role=status aria-live=polite（如 announcements「暂无公告」、equip「AI 因果链」），属 R101 既有语义，非加载节点，未误改。

## 4. AX 语义快照（完整 snapshot --json，ann/equip 两页）

- announcements（414）加载期：`- status "数据加载中"` 下挂 `- StaticText "加载中…"`，AX status 语义节点可感知。
- equipment（820）加载期：`- status "数据加载中"` 与原空态 `- status`（AI 因果链/装备表为空）并存，互不覆盖；与 QA §4 完全一致。
- 证据文件：data/ax-check.txt（AX_FULL ann 414 / AX_FULL equip 820 两段完整 snapshot）。
- 未对 10 页逐页做完整 AX 快照（DOM 三属性已逐页坐实），与 QA 同一诚实边界。

## 5. 截图（30 张，UNIQUE 30/30，全 ≠ QA 30 图）

三视口 × 10 页各一张 pd104-load-{page}-{W}.png，均 screenshot 于加载态注入后。sha256sum 实测 UNIQUE=30/30；与 QA screenshots 30 张 sha256 集合求交集=0（完全不同）。以下为逐张 sha256：

| 页面 | 414 | 820 | 1440 |
|---|---|---|---|
| announcements | `2fac37a9fec5a5b2a09da84f0380c003f5aa4da3795a936bded86effa0640b0c` | `c8fa9420315871007315ac9d064eb68f7fb0973b22f6acb2aa583eb4c23642ee` | `baee74fbe47520919e3d108074f9e6bc257fa6515218fc47d6ed2fb2ed0d0101` |
| bag | `250e84b04ebcb8448ccbfba17f6b2fd24382d1bc18cd40e602150f97660cd23c` | `a02d0ed7551997279bd69c7f71094d64dd700577166ad832edeee98bbed8d0db` | `297cff7971ea77a5e1a45066dc189342d3573bd23ad54edcbc1f4bd0dbb59b22` |
| battles | `818929d0296cee7d6add8a0886f1ea10cc85161d7d7c2508a12c10624a2463c0` | `128c65a618def19dc7e5ea93d8c54f10da676bcea16ddd9a05312d4384f3519d` | `db888423ff92fccce09fc077abdf8fb177fa0c6184a8484e3db51d1fd5248d58` |
| mastery | `b809a464ce1d6083c7c74aa1000ae350d09f6196a738dbd428f2c3ca9ab6f2fe` | `bd9da86a84764309b81ee8f6664847dd01892991a6f50c23fd205417c9973d94` | `41e28a18a70eed66e50d578187b2d2211e1f8630ed810700013da8797f3d9258` |
| rank | `85db916baab30b7dbbb3c1ae70f0da13c743d1adc5eae87af5d0557d5f47ffb6` | `3e6569cb5068ca73e9a07d0746f52a46abb0351a97e1784be962536c30e79c2e` | `da09d09a6c12e8a7199b031eefe50c87167b471981e70a6b32352b3997cef100` |
| shop | `de5fc3d3a0dd60600f0d906a5002a2bc66e884f157e99b4c950de47de2b38967` | `de99e2e9be4903c52feb4d6bf0bf96c69279b68d39ed47db1923298c3c7b0ec3` | `ab0c03b67c03781b0efcad0b4b66ebdda0c754d6736bd06dd796e649818b7308` |
| signin | `25af392c8cde61167c7613579b8c28a0996d7a0be068caa70c2d4571b861e8e6` | `3c3b8756aead0a83690f5b5fbcb11a9a9e7667c42c8eae6b8383a30f2f3500ee` | `054e544d1b1c8aa19d1ff1094baebf336fe30ee736bbf8751f865bfdbfdfe03e` |
| equipment | `f29da4f47bfc995275a599aefea37bcf8136202deffd832a8b5fa7f57454f52e` | `fefdce97f982e475f2873302f47277dcd2ce478cd8ea7b91fc3a5b4792f38384` | `4ac28773776dfecc777645d9c9aef0d727a4c4f036c3b985ff7985f522efca78` |
| reactions | `03210014afca113685d3285b7d55e0d4242c6dd87b2da189fb7fc35dbbecc5a2` | `4279456abc8f9593bb2224c08e83801023f518d9905c06e087f1b476cd05997d` | `bcfe4812e46d1ebc8b4c55f1ed5dcafabceed6ba81aa50e29d22078b43d22a5b` |
| skills | `f3c9ddf4f5570cd45290c2bc7323b5b8a6718241d273499be7798f6df14856a9` | `c370ddd552f6be38b1f86750a8d1bc16d0a9f8b2fe96289be7b538593e0a1d65` | `eadb360e63f62da54db2696e940dae3dd17ec86c5ae0f6931193582b0cba5467` |

## 6. 亲读截图（≥2 张非登录页）

- pd104-load-announcements-414.png（414×896，sha 2fac37a9…）：公告页「加载中…」文字卡 + 三行骨架占位条，标题「公告与版本 RELEASE NOTES · RULE ENGINE」与 AI 徽标；非登录页，视觉与 DOM/AX 加载态一致。
- pd104-load-equipment-820.png（820×1180，sha fefdce97…）：装备图鉴骨架屏，紫框骨架块 +「待配置 / 0/0 已拥有/装备总数 / 收集进度」+「AI 因果链」空态；非登录页，B 类 r6-skeleton 加载态视觉坐实。
- pd104-load-battles-1440.png（1440×900，sha db888423…）：我的战报页「加载中…」文字卡 + 三行骨架占位条，上方「战斗效率趋势」空态；非登录页，A 类加载态视觉坐实。

## 7. 门禁与 hygiene

- QA 门禁 G1–G7 全 RC=0（r104 契约 13/13 + r101/r60/r61 20 + 10 页单测 143 + 全量 2175 tests/1 skip + type-check + build:h5 + diff-check）。
- Go 门禁（job bash-60c41775480c4ae0，EXIT=0）：TestNoInfraIdentifiersInRepo PASS 8.22s(cached)；TestNoRealSecretsInRepo SKIP 非 PASS（secrets_leak_test.go:182 逐字「读不到 .local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」）。
- auth 六路径全 ABSENT（.env.local / .env.local.keys / laiyipao/miniapp/.env.local / laiyipao/admin/.env.local / laiyipao/admin/.env.local.keys / laiyipao/deploy/.local.keys）。
- .gitignore 666 行，R104 四规则 663–666 拦 data/ 与 admin-auth-init.js；git ls-files 中 round104-loading-a11y*/data = 0，本轮 tracked data=0。
- 占位令牌 r104-qa-placeholder-token（exp 2099）仅内存/localStorage 注入零真值；git status 非 ?? 项仅预期写域。

## 8. 诚实边界

1. 生产构建注入失败：生产 bundle 的 script setup setupState 闭包化，__r104Find/__r104Set 无法复现加载态；部署后 DOM/AX 复查在 dev server 复现 QA 注入路径完成，生产 bundle 仅 sha256 核对。
2. 后端 8080 未起：全部网络响应由 init-script 合成，非真实服务端数据；加载态经 Vue 实例注入 loading/r6Loading=true 复现，非自然网络延迟触发。bag 页源码 loading=ref(false) 且无 loading.value=true，加载根不会自然出现，必须注入。
3. AX 完整快照仅在 ann/equip 两页坐实，未对 10 页逐页做（DOM 三属性逐页坐实）。
4. 亲读 3/30 截图，其余 27 张以 sha256 + DOM 探针坐实。
5. QA 库本轮未连（本轮为前端 a11y 语义，不涉数据一致性）。

## 9. 结论

PASS。三视口 × 10 页加载期 role=status / aria-busy=true / aria-label=数据加载中（B 类 aria-hidden 缺席）全部坐实，卸载后语义节点从 DOM 消失；AX status 语义、零横溢、非登录页与既有契约均保持；30 图 UNIQUE 且全 ≠ QA 30 图，亲读 3 张非登录页；门禁与 hygiene 全绿。发布链五批 hash 见 git log。
