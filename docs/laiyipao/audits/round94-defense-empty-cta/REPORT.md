# R94 DefensesView 空态 CTA 与 reason 一一对应 —— 真浏览器 QA 报告

## §1 范围与方法
- 承接 t216 规范 RED（3 failed/3 passed）+ t217 实施 GREEN（契约 6/6，队长复核）；唯一业务写域 `laiyipao/admin/src/views/DefensesView.vue`，本 QA 只读业务源码，产物限定本审计目录。
- 真浏览器：agent-browser 0.41.1 真 Chromium，三视口 390×844 / 768×1024 / 1280×800。
- 环境：admin vite 5175（后台 job，结束后清理），base `/laiyipao-admin/`。后端未起，采用 init-script `data/r94-init.js` patch `window.fetch` 造数据：`/admin/me` 立即 200，`/admin/defenses` 带筛选参数（keyword/owner_id/shielded）时返回空表 total=0，无筛选参数时返回 3 行合成防线；令牌用占位串 `r94-qa-placeholder` 仅注入 localStorage，零真值。
- 空态构造：真实 `fill` 搜索框（placeholder「按防线名 / 玩家昵称搜索」）为 `NO_SUCH_zzz` + `press Enter` 触发 `@keyup.enter` → fetch 带 `keyword=NO_SUCH_zzz` → 空态。点击 CTA 用 agent-browser `click ".empty-state .empty-state-action"`（真实鼠标事件）。
- 探针 `data/r94-probe.js` 字段：innerW/clientW/scrollW/hOverflow（scrollW>clientW+1 为有溢出）、keyword、emptyExists、reasonText、ctaText、rowCount、resultCount、hits（`/admin/defenses` 请求 URL 序列）、isLogin、bodyLen。
- 双 open 预热 init-script；每视口 before（空态）+ after（CTA 后）各一张截图。

## §2 三视口实测表（CTA 前后）

| 视口 | 阶段 | hOverflow | keyword | 空态 | reasonText | CTA 文案 | 表格行数 | resultCount | 最后 defenses URL 含筛选 |
|---|---|---|---|---|---|---|---|---|---|
| 390×844 | before | false（390=390=390） | NO_SUCH_zzz | true | 当前筛选条件下没有防线；可清空筛选或刷新后重试。 | 清空筛选 | 0 | 共 0 条防线 | 是（keyword=NO_SUCH_zzz） |
| 390×844 | after | false（390=390=390） | "" | false | — | — | 3 | 共 3 条防线 | 否（...offset=0） |
| 768×1024 | before | false（768=768=768） | NO_SUCH_zzz | true | 同上逐字 | 清空筛选 | 0 | 共 0 条防线 | 是 |
| 768×1024 | after | false（768=768=768） | "" | false | — | — | 3 | 共 3 条防线 | 否 |
| 1280×800 | before | false（1280=1280=1280） | NO_SUCH_zzz | true | 同上逐字 | 清空筛选 | 0 | 共 0 条防线 | 是 |
| 1280×800 | after | false（1280=1280=1280） | "" | false | — | — | 3 | 共 3 条防线 | 否 |

- 三视口 before/after 的 fetch 序列最后一<u>条</u>`/admin/defenses` URL 均由带 `keyword=NO_SUCH_zzz` 变为裸 `?limit=50&offset=0`，坐实 CTA 真实清空筛选并重载。
- 首次 trial 用 eval 原生 setter 注入 keyword 未触发 Vue 的 `@keyup.enter`（fetch 仍为无筛选、空态未出现），该轮作废；改用 agent-browser `fill` + `press Enter` 后链路成立（见 `data/walk-out.txt`、`data/trial2-driver.sh` 输出）。
- BattlesView 范式对照（`data/{390,768,1280}-battles.txt`）：三视口均见「清空筛选」按钮，与 DefensesView 同构，范式未回退。

## §3 截图与 sha256（6 张全互异 UNIQUE=6/6）

| 文件 | sha256 |
|---|---|
| screenshots/r94-390-before.png | c65aeebc0ed45bf049f8d6de37eef8bd0489f984c4e1a9bb699db3e4c17209fb |
| screenshots/r94-390-after.png | 2563f674f3d57fa10c5c2cc0ad524a474b197903053c71980dafddb3d4089cb9 |
| screenshots/r94-768-before.png | b109ad1ade63523f81cb55233831cdde7812a42d411ad4ba02b92dc6a11fc4ab |
| screenshots/r94-768-after.png | 9d7891c031e19ff396c3c808828c783fc0b7b221e2f34c474847815b54486722 |
| screenshots/r94-1280-before.png | 936347bc44956db032f9f4dafe7ff0ae94989f855f22e82893f75ff5e97f4af9 |
| screenshots/r94-1280-after.png | 33cb4639f13295fdaf7491cfd3c18bb99068a24b4fe020390222284c44597311 |

- 亲读 2 张：`r94-768-before.png` 为防线值守空态（搜索框 NO_SUCH_zzz、空态卡「还没有人设置防线」、reason「可清空筛选」、CTA「清空筛选」）；`r94-1280-after.png` 为 CTA 后恢复的 3 行防线数据（KPI 3/0/3、列表 3 行）。均非登录页。其余 4 张仅列 sha，未亲读。
- 截图目录 `screenshots/` 未被 .gitignore 忽略，作为 untracked 证据保留，不入库。

## §4 不变量源码级与静态回归
- `data/static-out.txt`（RC0）：令牌三锚点 `tokens.css:12/13/14`（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6）在；`DefensesView.vue` 的 loadSeq 守卫 `:192/195/198/203/211/214`、fetchDefenses 参数 `:198`、resetFilters 四清零+page=1+load() `:219-224`、reason `:445`、CTA `:446`、result-count aria-live `:369`、data-grid `:372`、onDeleteDefense `:232` 全在；业务源码 `--r94-` 零命中（NO_MATCH）。
- 写域仅 `.gitignore`（队长预置）+ `laiyipao/admin/src/views/DefensesView.vue`；HEAD `b4bedf83944d41ce142cd2dfd5191ef516610540` 未变。
- auth 四路径（仓库根/admin/.env/.local.keys 等约定位置）ABSENT：本轮未创建真值凭据文件，令牌仅 init-script 内存注入占位串。

## §5 门禁（全绿，命令见 gates.sh，输出 data/gates-out.txt）

| 门禁 | 结果 | RC |
|---|---|---|
| G1 R94 契约 6 it | Test Files 1 passed / Tests 6 passed | 0 |
| G2 r5Mapping 范式 7 it | Tests 7 passed | 0 |
| G3 PageSkeleton 范式 9 it | Tests 9 passed | 0 |
| G4 admin 全量 vitest | Test Files 83 passed (83) / Tests 915 passed (915) | 0 |
| G5 type-check（vue-tsc --noEmit） | 通过 | 0 |
| G6 build:only（vite build） | 通过 | 0 |
| G7 git diff --check | 通过（仅 .gitignore LF→CRLF warning，无 error） | 0 |

- 首轮门禁因 workdir 设为审计目录导致脚本 `ROOT=$(pwd)` 取错、OUT 双重拼路径全部 tee 失败（RC1）；改以仓库根为 workdir 重跑后 7/7 RC0，首轮作废。
- 本轮无 Go 守卫要求（任务 §acceptance 未含 TestNoInfra/TestNoRealSecrets）；如需可另跑。

## §6 诚实边界与判定
- 后端未起，全部数据来自 init-script mock（合成 3 行防线），非真实生产战报；坐实的是 CTA 行为链（清空筛选→重载→URL 无筛选→表格恢复），不夸大为真值业务验证。
- 截图仅亲读 2/6；其余 4 张仅列 sha，未逐张视觉核验。
- 三视口 before/after 的 `qaCleared` 字段恒为 true 系 mock 中首次裸请求即置位，不作为判据；判据以 fetch URL 序列与 DOM 实测为准。
- QA 库只读核实（lya_qa）：users 表 id=477 存在、104 无记录；battle_records 中 477 有 1 条历史记录、104 无记录。本轮未写 QA 库，零真值入库。
- 判定：5 条 acceptance 全部满足，PASS。三视口文案一致、CTA 兑现「清空筛选」承诺、零横溢、范式与门禁全绿、hygiene 合规。