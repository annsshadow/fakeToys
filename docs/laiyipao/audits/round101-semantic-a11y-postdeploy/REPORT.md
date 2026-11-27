# R101 语义可访问性补全 — 部署后真浏览器回归报告（round101-semantic-a11y-postdeploy）

## §1 范围与发布链

- 发布对象：t267 五批发布，起点 `187aeb0eeaba475f9e2eb4bf5e9b74175699973b`（R100 收官），四改点语义属性 + r61 契约守卫增补。
- 五批链（以最终 `git log` 为准）：
  - B1 `6c8e11825f5a8f90a000c3a879ec92538579b98f` — feat：4 页 vue 的 5 处语义属性 + `r61FormA11y.contract.test.ts` 第 7 处 NODES 守卫（r61 契约随 me.vue nicknameMsg 语义补强同批入 B1，作为配套守卫，特此说明）。
  - B2 `4eca85d0464cc4edf3796e85542704621599a476` — test：`r101SemanticA11y.contract.test.ts`（9 it）。
  - B3 `fa798542ffaf06a3e52bae59d5d8a019c6c0d7f2` — docs：SPEC + RED-evidence + 独立终审报告。
  - B4 `baa36d1471b2c492665d2bb14175e94a4011f445` — audit：.gitignore + QA 报告 + 12 张 QA 真浏览器截图。
  - B5（本报告所在提交）：postdeploy 报告 + 12 张部署后截图。
- 部署产物：H5 主 bundle `index-ChTi7S5L.js`，sha256 `a6afa8f8c152f603eeab18bed35cc6908ab9fde73bfc96519dff1eb9940bec83`（dist/build/h5/index.html 引用，≠ R100 的 index-CNP3_F9X.js）；后端/admin 无变化沿用。
- 方法：真 Chromium（agent-browser 0.41.1），三视口 414×896 / 820×1180 / 1440×900，生产静态构建 H5（python -m http.server 5201），复用 QA 的 `r101-init.js`（合成 config/公告/敌兵/战报/profile，占位令牌 `r101-qa-placeholder-token`，仅内存注入不入库）。

## §2 三处 aria-expanded 切换证据（before → open → close）

三视口×三控件共 9 组，每组 DOM 探针读三态。判定要求：`aria-expanded` 序列 false→true→false；视觉佐证（index 的 `.ann-body.clamped` true→false→true / enemies 的 `.resist-radar-wrap` 0→1→0 / battles 的 `.expanded` class false→true→false）；`role=button` 与 `tabindex=0` 保留；`iw=cw=sw` 零横溢；AX snapshot 含 `expanded=true`。

| 视口 | 控件 | aria-expanded 序列 | 视觉佐证序列 | role/tab | AX expanded | 零横溢 | 判定 |
|---|---|---|---|---|---|---|---|
| 414×896 | index `.ann-item`（n=2） | false→true→false（第一条） | clamped true→false→true | button/0 | true | 414/414/414 | PASS |
| 414×896 | enemies `.resist-toggle` | false→true→false | radar 0→1→0 | button/0 | true | 414/414/414 | PASS |
| 414×896 | battles `.battle-row` | false→true→false | expandedCls false→true→false | button/0 | true | 414/414/414 | PASS |
| 820×1180 | index `.ann-item`（n=2） | false→true→false（第一条） | clamped true→false→true | button/0 | true | 820/820/820 | PASS |
| 820×1180 | enemies `.resist-toggle` | false→true→false | radar 0→1→0 | button/0 | true | 820/820/820 | PASS |
| 820×1180 | battles `.battle-row` | false→true→false | expandedCls false→true→false | button/0 | true | 820/820/820 | PASS |
| 1440×900 | index `.ann-item`（n=2） | false→true→false（第一条） | clamped true→false→true | button/0 | true | 1440/1440/1440 | PASS |
| 1440×900 | enemies `.resist-toggle` | false→true→false | radar 0→1→0 | button/0 | true | 1440/1440/1440 | PASS |
| 1440×900 | battles `.battle-row` | false→true→false | expandedCls false→true→false | button/0 | true | 1440/1440/1440 | PASS |

- 页面 `[aria-expanded]` 属性计数：index=2 / enemies=1 / battles=1，三视口一致（QA 报告同值）。第二条公告未点击时恒 false，与第一条独立。
- 键盘三件套 `role=button` / `tabindex=0` / `@keydown.enter` 未回退：编译后 DOM 回读 `role=button`、`tab=0`，与 QA 报告 §2 一致；本轮 diff 仅追加 aria 属性，未触及 handler 绑定。

## §3 me 失败态 status live region 证据

构造：me 页点「改名」→ 清空输入 → 点「保存」→ 本地校验 `!name` → `nicknameMsg="昵称不能为空"`（无网络依赖）。三视口结果一致：

| 视口 | statusNodes | liveNodes | AX | 零横溢 |
|---|---|---|---|---|
| 414×896 | UNI-TEXT role=status ariaLive=polite text=昵称不能为空 | 同节点 ariaLive=polite | `- status` → `StaticText "昵称不能为空"` | 414/414/414 |
| 820×1180 | 同上 | 同上 | 同上 | 820/820/820 |
| 1440×900 | 同上 | 同上 | 同上 | 1440/1440/1440 |

- 改前基线（QA §4 引用侦察）：该错误文本 AX 仅 `- StaticText`、零 status/live；改后成为 status live region，缺口关闭。
- 编译结构说明：内层 `SPAN` 无语义属性，`role=status`/`aria-live=polite` 落外层 `UNI-TEXT`，由 `probe-status` + AX 双证坐实，非缺陷。

## §4 部署后截图与亲读

12 张 png 落盘 `docs/laiyipao/audits/round101-semantic-a11y-postdeploy/screenshots/`，`sha256sum` 落盘实物计算，UNIQUE 12/12，与 QA 12 图零交集（`comm -12` = 0）。

| 截图 | sha256 | 截图 | sha256 |
|---|---|---|---|
| pd101-ann-414.png | 642a01fcb4ce26fa1488455a7be30a941df3ad3fed7283a77bdff0881dd581bb | pd101-ann-820.png | 1ceb0922030aa118dc1a10c8a8e44a5a859bba306488363d7e5e15f0221fae55 |
| pd101-ann-1440.png | aa0c01d0aedcef80b8b6d5742dfd9d49d9c8f659f3e144d1ec069ff3e48416bf | pd101-resist-414.png | 72f0be52dfdc778b8ecd1697c8513a4543a2d51e0f1c75bf237f5bf97395a528 |
| pd101-resist-820.png | c7ce7bd6a4077b3be02df575d6a772fea97d428b557a6f7183ace31876f52f4a | pd101-resist-1440.png | a8e5ef01cff6ad58d1d4785302f07d14f82daa037974091baa416b55ef074018 |
| pd101-battle-414.png | e12ebf0ecc7bacbe749b76bcfea519a3366ae998578c0f6ea191fcef055e345e | pd101-battle-820.png | e49da7659f2a8f485bcd33255313124c8d43f2f7af0c7f33ef54c064d158011d |
| pd101-battle-1440.png | 1acb1571564d2a160b1881ae7003a033f25fef61a4e94125491dae5e0db81ae3 | pd101-me-414.png | b4536f45cfbfec1ab19ea150b899f7b64492f8595f1485d9b9fd47da8b537522 |
| pd101-me-820.png | 77752c1f2c3f6727c391ed2b327a3f4e93e7d401693e126618608023d7c79a15 | pd101-me-1440.png | c5f8a51e965387581eb38c30749b461886e8eab8a0d0be5aad31ece198d540cb |

**亲读 4 张非登录页**（≥3 要求满足）：
- `pd101-ann-820.png`：来一炮首页游客态「QA玩家」，QA公告一展开态（⌃ 向上，正文可见），非登录页，与 AX expanded=true 一致。
- `pd101-resist-414.png`：敌人图鉴页，QA敌兵条目「收起抗性图 ▲」展开态，抗性表可见，非登录页。
- `pd101-battle-1440.png`：我的战报页，战绩总账/关卡胜率等数据卡，非登录页。
- `pd101-me-820.png`：指挥官档案页，AI 养成诊断卡，非登录页；错误提示在滚动下方（探针与 AX 已坐实 status live region，截图取页面上部）。

## §5 门禁与产物

| 门 | 内容 | 结果 | RC |
|---|---|---|---|
| G1 | r101 语义契约 | 9/9 | 0 |
| G2 | r61 表单契约 | 5/5 | 0 |
| G3 | R60 键盘范式契约 | 6/6 | 0 |
| G4 | R100 交互行契约 | 12/12 | 0 |
| G5 | 定向 4 文件 | 71/71 | 0 |
| G6 | miniapp 全量 | 233 passed / 1 skipped (234)；2141 passed / 2 skipped (2143) | 0 |
| G7 | vue-tsc（type-check） | 通过 | 0 |
| G8 | build:h5 | DONE Build complete | 0 |
| G9 | git diff --check | 通过 | 0 |
| Go | TestNoInfraIdentifiersInRepo | PASS 14.87s | 0 |
| Go | TestNoRealSecretsInRepo | SKIP（非 PASS）：secrets_leak_test.go:182 逐字「读不到 .local.keys —— 真值扫描这条守卫没有执行，不是通过了」 | 0 |

- QA 12 图 sha256 逐张重算 12/12 与 QA 报告 §3 表逐字一致（SHA_FAIL=0）。
- 静态回归锚点：tokens.css:14/15/16 三令牌在；业务源码零 `--r101-`。

## §6 诚实边界

- 页面数据经 init-script 合成（公告 2 条/敌兵 1 条/战报 id=9001/profile），占位令牌 `r101-qa-placeholder-token` 仅内存注入，无真值入库；坐实的是属性存在+AX 语义生效的客户端因果，非服务端真实数据下的交互结果。
- Go TestNoRealSecrets SKIP 非 PASS：缺 .local.keys，真值扫描守卫未执行，不冒充通过。
- me 失败态走本地校验无网络依赖；role=status 落编译后外层 UNI-TEXT 非内层 SPAN，已双证。
- 本轮未重复执行 Tab/Enter 动态用例（R100 已坐实键盘三件套，本轮 diff 未触及 handler）；探针回读 role/tab 保留。
- 亲读 4/12（≥3 满足），其余以 sha + DOM 探针 + AX 数据为准。
- push 网络偏差：代理 12450 不通（PROXY12450=000 RC=7），按契约探测后改用可用方式，实际结果见终审汇报。

## §7 判定

三处 aria-expanded 在生产静态构建三视口坐实 false→true→false 与视觉/AX 同步，me 失败态成为 status live region；12 图 UNIQUE 且全≠QA、亲读 4 张非登录页；门禁 G1–G9 全 RC0，Go NoInfra PASS / NoRealSecrets SKIP 如实；零真值、tracked 本轮 data=0。**部署后回归 PASS**。
