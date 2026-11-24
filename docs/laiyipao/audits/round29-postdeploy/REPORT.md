# R29 独立部署后真浏览器回归报告（round29-postdeploy）

- 轮次：R29 完整反应时间线抽屉（结算页入口 + 战斗态入口）。
- 发布时间线：本目录为「部署后独立证据」；t228 预部署 QA 图（round29-live-reaction-timeline/screenshots 15 张、round29-postfix-recheck/screenshots 4 张）属部署前证据，两套 SHA 不同，互不冒充。
- 环境口径：地址统一用占位符 <YOUR_TEST_DOMAIN>；仅本地回环 127.0.0.1 用于自测，不写入仓库真值。

## 一、本地门禁（发布前）
- pnpm test：FULL TEST SUITE 全过；pnpm type-check（vue-tsc）与 pnpm type-check:test（tsc）全过；pnpm build:h5（uni build）全过。
- Go 守卫 TestNoRealSecretsInRepo：本机缺少 .local.keys，打印「真值扫描这条守卫没有执行，不是通过了」并 SKIP，如实标注本机未执行，非通过。
- 全部门禁脚本输出 === ALL GATES DONE ===。

## 二、三端部署与健康
- H5 重建：bundle 产物 index-BxR5Dc75.js，SHA256=06811f001ca9d4d792d4c43f593ba42d7ccc597261949151111e743cca7c3803。选择器 .r29-result-timeline-cta / .r29-timeline-overlay / .r29-timeline-row 命中 battle-DjcG2MoV.css 与 pages-battle-battle.DKRBR369.js。
- 后端沿用二进制 SHA256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 版本，未回退）；admin 沿用 index-CGo_zhD3=d5e9c483992f2a370b5def55d7418da958b695deda4db9c3446e26bd04d58c70（R20 版本，未回退）。
- 健康：/healthz=200、/readyz=200、/api/v1/config=200、H5(5174)=200、admin(5175/laiyipao-admin/)=200；/api/healthz 与 /api/readyz=404 为既有路由口径，非故障。

## 三、真浏览器独立回归（无 shim）
- agent-browser 无头 Chromium 390x844，通过注入占位 token 的 auth-init.js 打开本地回环 battle?level=5；token 真值仅本地生成，脚本结束后删除，未入库。
- 111 帧推进至 SETTLED，CARD_CLICKS=4，SKIPS=0，无失败请求；结算 status=200 battle_id=478，hashResponses 为空。
- 结果页 CTA 真实 CLI 点击：exists=true，文案「查看完整反应时间线 →」，位于结算 sheet 内，命中测试 isCta=true，pointer-events=auto；点击 rc=0，抽屉打开；关闭 rc=0；重开 rc=0。
- 抽屉几何：box 宽 378 高 706，z-index=40（高于图例/暂停 30 与结算 sheet），71 行每行无溢出（right=371，scrollWidth=clientWidth=351），文档无横向溢出，hasUndefined=false、hasNaN=false，滚动容器 overflow-y=auto（内容高度 591=clientHeight，本局无需滚动）。
- 打开/重开探针：state=SETTLED、timelineOpen=true、rowCount=71、result=通关、helpVisible=false；R24/R27/R28 HUD 共存 true。关闭探针：timelineOpen=false、rowCount=0、result=通关。
- 逐帧 14 个 samples 全 3 条 feed；AI 四判据齐；诚实红线「辅助展示非权威判据，权威以 replay_hash 为准」在抽屉逐字呈现。
- 控制台 pageErrors=61，逐条复核全为 Chrome 自动播放策略噪声，真实错误 real=0；consoleErrors=0、apiFailures=0、failedRequests=0。

## 四、权威对账
- 服务端详情接口含 reaction_timeline 与 replay_hash；TIMELINE_TOTAL=71、渲染 EVENTS=71、TRUNCATED=false；DOM 行与 API 全量 71 行逐字段比对 MISMATCHES=0，FULL_RECONCILE_OK=true。
- 末 3 行逐字段样例：第 5 波 tick 1540 超导 #135 132.0 伤害；tick 1544 蒸汽爆发 #138 326.0 伤害；tick 1554 蒸汽爆发 #138 472.0 伤害；与服务端 wave/tick/reaction/enemy_uid/damage 一致。
- 回放哈希展示为「回放哈希 d5397c28552b43c2（可验真）」，与结算响应一致；抽屉为只读旁路，不进 replay_hash。

## 五、截图证据（新拍，非部署前图）
- r29post-01-live.png：实时 feed + AI 四判据 + CTA。
- r29post-02-result-entry.png：结算页入口按钮。
- r29post-03-result-drawer.png：71 行抽屉 + 结论 + 依据 + 诚实行。
- r29post-04-result-closed.png：关闭抽屉回到结算页。
- 四图 SHA256 依次为 1b55852a0dd93a3fe3cc5374fd6d64a0c00ed6ccae1c55b338434d04d5cf3f2b、f1b059e72838f4f4af42016ceb5eeac110dff543cadb4ca8ff2c1cc657bb5e23、a409f47df562e50ddb4d6f0579f0a5c2ed5b2bdf2774fab87245e19d05f3e195、7db4e629ce487c2c9ee2360a9e7b6ee410a6cbcfa1f11bf07f0212e44c53449d；两两不同，且均不等于任何部署前 QA 图 SHA。
- r29post-05-reopen.png 与 03 内容重复、r29post-05-scroll-bottom.png 因抽屉无需滚动而无效，二者已删除，不入库。四张图均亲读确认为 390x844 非登录页。

## 六、真值与清理
- data/ 与 auth-init.js 由 .gitignore 拦截；auth-init.js 在 E2E 后删除，git check-ignore 三条全命中，git diff --check OK。
- pg_restore 还原 QA 库后 battle_records=104、max_id=477、id=478 记录数=0。
- 本批精确 git add 显式路径，排除 data/、auth-init.js 与所有历史噪声脚本/失败截图；逐批真值扫描零命中。

## 七、结论
- R29 发布验收通过：部署产物命中选择器，三端健康，独立部署后真浏览器 L5 通关并真实点击结算页 CTA，抽屉 71 行与服务端权威逐字段一致，4 张新图与部署前证据不同，真值未入库，数据库已还原。