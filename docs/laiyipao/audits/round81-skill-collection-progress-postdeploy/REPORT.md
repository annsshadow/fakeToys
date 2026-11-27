# R81 skills 收集进度条一致性美化 — 部署后真浏览器回归报告

- 轮次：R81 部署后回归（postdeploy），same-route-rel（发布者 release-ops）
- attempt_id：5f6d5018-6c88-4eaa-a87a-93629ee59b4c（t181 五批发布）
- 起点 HEAD：74a8952236797a52104c248da21e229925bbe0ea（R73 收官）
- 日期：2026-11-27
- 工具：agent-browser 0.41.1（真 Chromium）、静态 H5 5201、API 8080（回环）
- 判定：部署后三视口回归通过，可发布

## 1. 五批 commit 链

| 批次 | 类型 | 完整 hash | 主题 |
|---|---|---|---|
| B1 | feat | edc6b7947f1f7cfcbee0b55aa67e8730e5f67b61 | R81 skills图鉴收集进度条一致性美化（skills.vue +6/-0） |
| B2 | test | 4108e6730ef54b14079080e9054ac87461381e63 | R81 收集进度条契约 6it |
| B3 | docs | 86eb122133ccc4019233a7a5b4fac017323c1db7 | R81 规范与独立终审 |
| B4 | audit | 4502d1228092b73d8d8ada988f3e4c467e791fac | R81 真浏览器QA审计与RED日志 |
| B5 | chore | 本批次（自引用哈希悖论，提交后为最终 HEAD） | R81 部署后回归报告 |

每批精确 git add 逐文件，禁 -A / 点号 / 目录通配；每批 guard 验证 DATA_IN_STAGE=0、LEAK_OK、CHECK_RC=0。

## 2. 产物 sha（三产物沿用/重建，不回退）

| 产物 | 文件 | sha256 | 说明 |
|---|---|---|---|
| H5 主 bundle | index-iiibg12t.js | fa808127fc84a738238e8541a4aaffce0b0a11b6678a036805df4ee085169a57 | 本轮 pnpm run build:h5 重建（index.html 引用，≠ R73 index-B1_dpZ3t.js） |
| 后端 | lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | 沿用基线不回退 |
| admin | WikiView-ST1PbAt2.js | f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b | 沿用基线不回退 |

## 3. 部署后三视口 DOM 探针（真 Chromium 导航 #/pages/skills/skills）

注入两处 auth-init.js（guest 令牌经 init 脚本，不写真值）；视口 414×896 / 820×1180 / 1440×900（区别于 QA 390/768/1280）。

| 视口 | loggedGuest | afterKpi / beforeFilter | label / num | fill style | computed 宽 / 轨道宽 | iw/cw/sw | hOverflow | 矩阵 5/10 | chip / 计数 |
|---|---|---|---|---|---|---|---|---|---|
| 414 | true | true / true | 收集进度 / 已解锁 7/24 | width: 29% | 64.75px / 223.281px | 414/414/414 | false | true / 5/10 | 6 / 显示 24 / 共 24 条 |
| 820 | true | true / true | 收集进度 / 已解锁 7/24 | width: 29% | 53.3906px / 184.125px | 820/820/820 | false | true / 5/10 | 6 / 显示 24 / 共 24 条 |
| 1440 | true | true / true | 收集进度 / 已解锁 7/24 | width: 29% | 112.219px / 387px | 1440/1440/1440 | false | true / 5/10 | 6 / 显示 24 / 共 24 条 |

结论：进度行位于 KPI 区之后、筛选行之前（compareDocumentPosition 位判定），标签/计数/29% 宽度三视口一致，非零填充来自真实账号 /me 快照（7/24），零横向溢出；G66 元素×类型覆盖矩阵（5×2=10 格，covered 5）、标题「元素 × 类型覆盖」、6 筛选 chip、结果计数「显示 24 / 共 24 条」全部完好。与 QA 三视口结论一致。

## 4. 部署后截图 sha（3 张，全 ≠ QA 三图，UNIQUE=3/3）

| 文件 | sha256 |
|---|---|
| pd-skills-414.png | 08f983984c88262627536955caa3d307cf73758ce7b6128e8f60f6c77c05e8a0 |
| pd-skills-820.png | b56e70a1ee058aa2d7f4aea41818fbeea6fd6de3afc89d17747eb27e9deeede5 |
| pd-skills-1440.png | 589397d0dfac09173854e38dcad55ef04650fde403d955ac5690883105d45bf0 |

QA 三图基线：390=c370ad7a03930fe0222bd6c3c480988b7cd1cfe72958842dd823c3ad56394207 / 768=e9bc62e4d8254bede9165f72f99d7b9680509c5110af17583d8a9adb0fe0fdb5 / 1280=f2765b73e747951426be46de0237735e11665598f9ea3b819553f02fb76630da（三张 pd 图全不等于 QA 图）。

亲读 3/3：414 与 1440 为「技能图鉴」页，清晰可见「收集进度 已解锁 7/24」红橙渐变约 29% 进度条、KPI「7/24 已解锁/技能总数」「18 合成技能」、筛选 chip 与内容卡；820 为同页顶部（进度条在首屏以下，DOM 探针已坐实绑定）。三张均非登录页、无白屏、无错位。

## 5. 回归锚点（静态，全部在位）

- 令牌三锚点：tokens.css:14 --lyp-token-version: lyp-tokens-v1 / :15 --lyp-r3-version: lyp-tokens-v3-r3 / :16 --lyp-r6-version: lyp-tokens-r6 逐字在。
- 业务源码零 --r81- 令牌（仅契约否定断言，业务 0 命中）。
- R60 role="button" 51 处；R61 v-aria-native 24 处；R55 placeholder-class="ph" 23 处（跨轮累积增长，未回退）。
- R71 fmtVerifyTime 在 verify.vue 2 处；R66 normalizeGemGain 在 gems.vue 3 处；R69 nicknameSubmitting 在 me.vue 6 处；R72 search-empty-text 在 skills.vue/common.css 各 1 处；R73 trapLegendTab/legendOverlayRef 在 battle.vue 4 处。
- B4 审计写入的 RED 日志经去尾部空行后入库，diff-check RC0。

## 6. 门禁（部署后复跑）

| 项 | 命令 | 结果 |
|---|---|---|
| Go NoInfra | go test ./internal/domain/... -run TestNoInfraIdentifiersInRepo -count=1 | PASS 4.673s，RC0 |
| Go NoRealSecrets | go test ./internal/domain/... -run TestNoRealSecretsInRepo -count=1 -v | SKIP 非 PASS，逐字「读不到 …laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是『通过了』」，RC0 仅表示命令正常退出 |
| diff-check | git diff --check @{u} HEAD | RC0（仅 .gitignore LF→CRLF warning） |

前端门禁（契约 6/6、全量 225 files/2061 tests、vue-tsc、build:h5）由 QA/终审轮独立复跑全绿，本轮不重复。

## 7. 诚实边界

1. guest 令牌仅换取前端会话，不改 QA 库；QA 库只读核查 users id=477 存在（rc_shd），104 无记录（历史各轮并列写 104|477 按实测如实标注）。
2. 部署后 3 张截图 UNIQUE=3/3 且全 ≠ QA 三图；亲读 3/3（820 进度条在首屏以下以 DOM 探针为差异证据）。
3. 非零进度 29% 来自真实账号 /me 快照（7/24），非 mock；ownedCount=0→0% 场景由 equipment 页真实空态（0/18）对照。
4. Go TestNoRealSecrets 因缺 .local.keys 为 SKIP 非 PASS，真值扫描守卫未执行，如实标注不冒认通过。
5. 搜索占位编译后落 aria-label（uni-app 编译行为），契约静态断言源码占位逐字在，非缺陷。
6. B5 报告自引用提交哈希悖论：本文件哈希由本批次提交确定，故 B5 hash 留待提交后确认。
7. 真值 hygiene：data/ 与 admin-auth-init.js 被 .gitignore 拦截，收官删除两处 auth-init.js 至 ABSENT；无 token/口令/域名/IP 回显。

## 8. 结论

R81 skills 收集进度条一致性美化部署后真浏览器三视口回归通过：进度条位置/文案/29% 非零填充/零溢出/G66 矩阵与不变量全部符合规范，与 QA 结论一致、截图全互异且不与 QA 重合，产物 sha 不回退，零真值入库。可发布。
