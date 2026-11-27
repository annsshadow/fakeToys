# R93 RecomputesView 骨架屏一致性 — 部署后回归报告

- 任务：t213（attempt_id=b5f5aec1-592c-4630-9a9b-f8774ac9b8c8），唯一发布者 release-ops。
- 起点：HEAD=43a252d90ee529fc31a02d02f9444278f8feada4（R88 收官），AB=0/0。
- 结论：**可发布**。部署后三视口 414/820/1440 加载态/到达态/局部刷新三态全部符合契约，与 QA（390/768/1280）视口互异且截图 sha 全不同；五批精确提交、零真值入库。

## 1. 五批 commit 链

| 批次 | commit | 内容 |
| --- | --- | --- |
| B1 feat | d07ace48f5911ceed09939e3312b3b1823ba0730 | RecomputesView.vue +3/-1：导入 PageSkeleton、根节点后 v-if=loading 插入、表格移除 v-loading=loading |
| B2 test | 8723ed732 | R93RecomputeSkeleton.contract.test.ts（69 行 6it） |
| B3 docs | ad49b6bb1 | SPEC.md（62 行）+ ROUND93_DESIGN_REVIEW.md（83 行） |
| B4 audit | 57bf430f3 | .gitignore +6 行 R93 白名单 + QA REPORT.md + 5 脚本 + 11 截图（18 文件 228+） |
| B5 postdeploy | （本报告提交） | 部署后回归报告 + pd-probe.js + pd-driver.sh + 6 张部署后截图 |

每批精确 git add 逐文件路径（禁 -A/点号），泄露守卫 LEAK_OK、DATA_IN_STAGE=0、CHECK_RC=0，warning 仅 .gitignore LF→CRLF。B4 严格排除 data/ 与 admin-auth-init.js（已被 .gitignore:605-608 拦截），未用 git add -f。

## 2. 产物 sha（部署后重建）

- admin 前端重建：laiyipao/admin 下 `pnpm run build:only`，RC=0（BUILD_RC=0）。
- 主 bundle：`index-uFWsk6dV.js` sha256=`c50f872d3b304934e13baa3dd24e9b6244cedb709357bb52296e5cb4671af00b`（dist/index.html 引用）。
- 视图 chunk：`RecomputesView-rYYffCff.js` sha256=`017333571820af80e50d8652ac51b1314c08c65b9d6a08a79cb5890ea66e0535`。
- 共享骨架 chunk：`PageSkeleton-BoNELPhT.js` sha256=`c94a41a4b423572746f83f53beb30819a661f49c6d948cdc37d5b35098d71a2e`（被 Battles/Content/Dashboard 等 10 视图共享）。
- 后端产物 `lyp-server-amd64` sha256=`3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6` 未回退（本轮不碰后端）。
- admin dist 无 tracked 改动（git status -- laiyipao/admin/dist/ 空）。

## 3. 部署后三视口三态探针（真 Chromium）

服务：admin 静态 5175（python http.server 处理 /laiyipao-admin/ base 前缀），init-script 注入 QA mock（占位令牌 r93-qa-placeholder、合成总账 42/7/16.7%/enforce、4 行列表、延迟 5000ms）。视口 414×896 / 820×1180 / 1440×900。证据：postdeploy/data/pd-{414,820,1440}-{load,arrive,refresh-mid,refresh-after}.txt。

| 态 | 断言 | 414 | 820 | 1440 |
| --- | --- | --- | --- | --- |
| 加载 | skeletonCount / visible | 1 / 1 | 1 / 1 | 1 / 1 |
| 加载 | skRole / skAriaBusy | status / true | status / true | status / true |
| 加载 | skKpiCards / skBars / skRows | 4 / 35 / 7 | 4 / 35 / 7 | 4 / 35 / 7 |
| 加载 | gridMaskCount / statMaskCount | 0 / 1（不可见） | 0 / 1（不可见） | 0 / 1（不可见） |
| 加载 | kpiValues / tableRows | 全「—」 / 0 | 全「—」 / 0 | 全「—」 / 0 |
| 加载 | skVisibleKpi / isLogin / title | true / false / 结算重算审计 | 同左 | 同左 |
| 到达 | skeletonCount / 遮罩 | 0 / 0 | 0 / 0 | 0 / 0 |
| 到达 | kpiValues / tableRows | 42,7,16.7%,enforce / 4 | 同左 | 同左 |
| 到达 | resultCount | 窗口 24h · 共 42 条 | 同左 | 同左 |
| 刷新 mid | skeleton / statMask / statVisible / gridMask | 0 / 1 / 1 / 0 | 同左 | 同左 |
| 刷新 after | statMask / statVisible | 0 / 0 | 0 / 0 | 0 / 0 |
| 全态 | scrollW==innerW==clientW / hOverflow | 相等 / false | 相等 / false | 相等 / false |

刷新 mid 态坐实「刷新总账」只触发 stat-row 局部 v-loading，首屏骨架不重现、KPI 与表格保持；after 态遮罩消失。与 QA 报告 §2 三视口结论一致。

## 4. 部署后截图 sha256（6 张，UNIQUE=6/6，全≠QA 11 图）

| 文件 | sha256 |
| --- | --- |
| pd-r93-414-load.png | 43b01f14278d4de75a32c9fa6899ecc1ca8000ba71e64dd07f73e0a043501da4 |
| pd-r93-414-arrive.png | 1ed10dbfc66c08fe742e644d16348c46394a2a3c8953e7c8d97a695ede08cdcd |
| pd-r93-820-load.png | c5646ad682331036850c9fd0d65701b8ac9e3bb9d955d0a319e089540b70c32b |
| pd-r93-820-arrive.png | 90d352c2dbd58a8d910f05abce7ea4ab7f2b8074d6d9a0979158ae454379d5e8 |
| pd-r93-1440-load.png | 50713d0b2499cd3b1b8a6b9d8302eba23557254bcaf88ee60166521aa39c7884 |
| pd-r93-1440-arrive.png | e4162facec9ced41cac3bae5e553fe5a429cac938560dae03aa0b72b922cdc18 |

亲读 3 张：pd-r93-414-load（414×896，非登录页，可见「结算重算审计」顶栏 + 左侧导航 + 4 张 KPI 骨架卡 + 表头与 6 行表格骨架）；pd-r93-1440-load（1440×900，非登录页，同构骨架全宽展开）；pd-r93-820-arrive（820×1180，非登录页，可见重算档位条 + AI 运营洞察卡 + KPI 真值 42/7/16.7%/enforce + stat-row 统计条 + 结果计数）。其余 3 张仅列 sha。QA 11 图（390/768/1280 六态 + trial）sha 见 QA REPORT §3，与本轮 6 图全不同。

## 5. 回归锚点（静态）

- 令牌三锚点：laiyipao/admin/src/styles/tokens.css:12/13/14 逐字在（--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6）。
- 业务源码零 --r93-（grep 排除契约后 NO_MATCH）。
- 共享骨架范式：admin/src/views 下含 `<PageSkeleton v-if="loading" />` 的视图 10 个（Recomputes 新增后），`class="page" v-loading="loading"` 整页遮罩零命中。
- 竞态守卫 statsLoadSeq/loadSeq 与 Promise.all([load(),loadStats(),loadMode()]) 不变量保留（QA data/static-out.txt 坐实）。

## 6. 门禁

| 项 | 结果 |
| --- | --- |
| R93 契约 + PageSkeleton 范式契约（发布方复跑） | 15/15 passed（2 files），RC=0 |
| QA 全量 admin vitest（t211） | 82 files / 909 tests passed，RC=0 |
| type-check（vue-tsc） | RC=0（QA/t212 复跑） |
| build:only | RC=0（本轮重建） |
| git diff --check（工作树 / @{u}..HEAD） | RC=0 |
| type-check:test | 队长裁定豁免：既存写域外 .ts 类型债，非本轮范围 |
| Go TestNoInfraIdentifiersInRepo（t211） | PASS 5.5s |
| Go TestNoRealSecretsInRepo（t211） | SKIP 非 PASS：读不到 .local.keys —— 真值扫描这条守卫没有执行，不是「通过了」 |

## 7. 零真值与 hygiene

- AUTH 四路径（round93 与 postdeploy 各 data/auth-init.js、admin-auth-init.js）实测 ABSENT；.env / .local.keys / remote-config.env 亦 ABSENT。
- 本轮五批变更 21 文件（B1 1 + B2 1 + B3 2 + B4 18 + B5 待提交 6），tracked data=0；.gitignore 真值规则 check-ignore 命中 .gitignore:605/607。
- 五批新增内容泄露扫描 LEAK_OK（带口令串 postgres://非零非零@ 避开无口令 loopback）；git diff @{u} HEAD --check RC=0。
- QA 库容器 lyp-qa-pg 只读：battle_records count/max=104|477；本轮未写库。
- tracked 工作树修改 NONE（提交后 clean）。

## 8. 诚实边界

1. 后端 8080 未启动，数据由 init-script mock 合成（占位令牌 + 42/7/16.7%/enforce + 4 行），坐实前端骨架与数据切换而非真实后端返回。
2. 6 张部署后图仅亲读 3 张，其余仅列 sha；视口 414/820/1440 与 QA 390/768/1280 互异。
3. QA 首轮全量 vitest 一条既存 flaky 超时（WikiEnemyMechanics 审计脚本扫描 22914ms > 20s），定向重跑 14/14、全量重跑 909/909，非本轮回归。
4. type-check:test 按队长裁定豁免，非 PASS。
5. Go TestNoRealSecretsInRepo SKIP 非 PASS（缺 .local.keys，真值扫描守卫没有执行），如实标注。
6. B5 报告自引用哈希悖论：本报告 sha 不含自身；B5 commit 哈希在提交后确定。
7. guest/mock 令牌仅注入内存，未改 QA 库。

## 结论

部署后三视口三态全部符合 R93 契约与 QA 范式，零横溢、零真值入库、五批精确提交、git 终态全绿。**可发布**。
