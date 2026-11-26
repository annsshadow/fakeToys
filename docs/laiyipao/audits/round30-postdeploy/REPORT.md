# R30 独立部署后真浏览器回归报告（round30-postdeploy）

- 轮次：R30 窄屏反应行盒模型修复（390×844 视口模拟，非物理真机）。
- 发布时间线：本目录为「部署后独立证据」，与部署前 QA 七图（round30-reaction-feed-mobile-fit/screenshots 下 r30-01..07）分属两套独立会话；八张部署后新图 SHA 两两不同且全不等于任何 QA 图 SHA，互不冒充。
- 环境口径：地址统一用占位符 <YOUR_TEST_DOMAIN>；仅本地回环 127.0.0.1 用于自测，不写入仓库真值。

## 一、本地门禁（发布前）
- 从 laiyipao/miniapp 目录执行：R30 契约 + R28 归因 + battle.test.ts 共 3 files / 97 tests passed；pnpm test 212 passed | 1 skipped files（1982 passed | 2 skipped）；pnpm type-check（vue-tsc）、pnpm type-check:test（tsc）、pnpm build:h5（uni build）全 exit 0，输出 ALL GATES DONE。
- Go 守卫 TestNoRealSecretsInRepo：本机缺少 .local.keys，打印「真值扫描这条守卫没有执行，不是通过了」并 SKIP，如实标注本机未执行，非通过；以 OPS 静态扫描与逐批 git diff --cached 零真值替代。

## 二、三端部署与健康
- H5 重建：bundle 产物 index-DCBbR3t9.js，SHA256=aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b；battle-R4Klvkvg.css 与 pages-battle-battle.CmxuijW-.js 命中 R30 盒模型（box-sizing:border-box / overflow-wrap:anywhere / white-space:normal / r28-attribution）与 R28/R29 选择器。
- 后端沿用二进制 SHA256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 版本，未回退）；admin 沿用 index-CGo_zhD3=d5e9c483992f2a370b5def55d7418da958b695deda4db9c3446e26bd04d58c70（R20 版本，未回退）。
- 健康复查：healthz=200、readyz=200、H5(5174)=200、admin(5175/laiyipao-admin/)=200。

## 三、真浏览器独立回归（无 shim）
- agent-browser 无头 Chromium 390×844，独立会话 lyp-r30-postdeploy，通过注入占位 token 的 auth-init.js 打开本地回环 battle?level=5；token 真值仅本地生成，脚本结束后删除，未入库。
- 80 帧逐帧采样推进至 SETTLED，CARD_CLICKS=4，SKIPS=0；states 为 WAIT 73 / CARD_PHASE 5 / SETTLED 2；80/80 帧 feed 恒 3 条，R24/R27/R28 共存 80/80、R26 在选卡态 5/5；docOverflow 与 feedOverflow 全 false，hasUndefined/hasNaN 全 false。
- 正常态量测：视口宽 390，reaction-feed 左右边界 10/380、宽 369、scrollWidth=clientWidth=369；三条反应行 right=380、width=369、sw=cw=367，归因样式 white-space=normal、overflow-wrap=anywhere，文档与行均无横向溢出。R28 基线 right=396/feedOverflow=true 的缺陷不再出现。
- 长文本压力：向 .reaction-name 与 .r28-attribution 注入超长无空格串（SUPERLONGUNBREAKABLETOKEN...），applied=true，feed sw=cw=369、docOverflow=false、anyOverflow=false；注入元素 white-space=normal / overflow-wrap=anywhere / overflow=visible，真实换行可见未被隐藏；恢复脚本 restored=true 且文案回到「焰+冰 → 蒸汽爆发」与「第1波 · #1」。
- CTA 真点击：结算页 .r29-result-timeline-cta 文案「查看完整反应时间线 →」，位于结算 sheet 内，命中测试 isCta=true，pointer-events=auto；真实 CLI click rc=0 打开 66 行抽屉，关闭 rc=0，重开 rc=0。
- WAIT 态 CTA 竞态补救：主 E2E 首次点击被选卡浮层覆盖（click 日志如实保留失败证据）；另起全新会话 lyp-r30-waitcta 命中测试 hit=r28-feed-cta 后真实 CLI click 成功，抽屉打开 3 行、box 宽 378 高 183、z=40、每行 right=371/w=sw=cw=351、无溢出、AI 四判据与诚实行齐，关闭后 timelineOpen=false，截图 r30post-08-wait-cta.png。
- 探针：pageErrors=67 全为 Chrome 自动播放策略噪声（The play() request was interrupted by a call to pause），真实错误 real=0；consoleErrors=0、apiFailures=0、failedRequests=0；hashResponses 为空。
- settle status=200 battle_id=478；结果页展示「回放哈希 3f6985097c23d9fc（可验真）」，与服务端详情 replay_hash 逐字一致。

## 四、权威对账
- 服务端详情 reaction_timeline：TIMELINE_TOTAL=66、渲染 EVENTS=66、TRUNCATED=false；DOM 行与 API 全量 66 行逐字段比对 MISMATCHES=0，FULL_RECONCILE_OK=true。
- 末 3 行逐字段：第 5 波 tick 1284 过热 #120 280.0 伤害；tick 1327 蒸汽爆发 #121 326.0 伤害；tick 1333 过热 #121 436.0 伤害；与服务端 wave/tick/reaction/enemy_uid/damage 一致。
- 抽屉为只读旁路，不进 replay_hash；结论「本局显示 66 条逐次反应记录」、依据「引擎 reactionTimeline() 只读旁路（前 200 条，不进 replay_hash）；反应名以服务端 config.reactions 为准」、诚实行「辅助展示非权威判据，权威以 replay_hash 为准」逐字呈现。

## 五、截图证据（新拍，非部署前图）
- r30post-01-normal-three-lines.png：正常三行 feed + AI 四判据 + CTA。
- r30post-02-stress-longtext.png：超长无空格串压力换行可见。
- r30post-03-wait-drawer.png：主 E2E WAIT 态点击被覆盖时的现场（如实保留失败尝试画面）。
- r30post-04-settle.png：结算 AI 复盘与入口。
- r30post-05-result-entry.png：结算页入口按钮独立截图。
- r30post-06-result-drawer.png：结算页 66 行抽屉 + 结论 + 依据 + 诚实行。
- r30post-07-result-closed.png：关闭抽屉回到结算页。
- r30post-08-wait-cta.png：独立会话 WAIT 态 CTA 真点击后 3 行抽屉。
- 八图 SHA256 依次为 b9cfe68e24e1893f6a31219dbab6e8d60a7a353a181b7ec3f8409a9c08a18ee9、180a4d04b74d1f785063a29d402627c6a7d44cb2f9384b2c293b078e66bd57aa、96188764503b69021a7185a42303c6c3cc741cc92ad8596a2d667de23afc4961、35ea251c24e604a5f6276f1ee969816885329654686102233e3db502cfb2283a、d908d76dadaa404b6a0fd5e5be5cfcf5121ce8696d50e23b38e561f59cd5f078、488b85ac2dd0cd0f9250565767e585072fec6b8925323378bcc8a5f0240b08be、b0a1a237e3d379b50056846d0b3552dd07597a6aa539ddd3c0764cea3199d537、16c295442602886386ec12833bf9f2b2d6ea53b31b85cc04d35581c759ad6468；两两不同且全部不等于任何部署前 QA 图 SHA；八图均亲读确认为 390×844 非登录页。

## 六、真值与清理
- data/ 与 auth-init.js 由 .gitignore（第 362-363 行）拦截；auth-init.js 在 E2E 后删除，git check-ignore 命中，git diff --check OK。
- pg_restore 还原 QA 库后 battle_records=104、max_id=477、id=478 记录数=0。
- 本批精确 git add 显式路径，排除 data/、auth-init.js 与所有历史噪声脚本/失败截图；逐批真值扫描零命中。

## 七、结论
- R30 发布验收通过：部署产物命中盒模型选择器，三端健康，独立部署后真浏览器 390×844 视口模拟通关且正常/长文本/CTA/SETLLED 四类证据齐全，66 行抽屉与服务端权威逐字段一致，8 张新图与部署前证据不同，真值未入库，数据库已还原。
