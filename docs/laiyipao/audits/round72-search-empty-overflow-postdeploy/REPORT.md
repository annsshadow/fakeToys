# R72 搜索空态极端值溢出修复 — 部署后真浏览器回归报告

- 任务：t156 / attempt 41853573-ec1d-4fed-9145-b52b81441bc9
- 工作树：D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
- 起点 HEAD：ea82592a6d0b1993b8918bda4bfef6ffd6781450（R71 收官）
- 执行日期：2026-11-27（Asia/Shanghai，UTC+8）
- 结论：**PASS**（带诚实边界）。R72 修复在重建后的 H5 产物上、三视口真 Chromium 注入极端串复验全部通过：极端串不再溢出、关键词完整、XSS 转义、清除搜索复位，回归锚点与门禁无回退。

## 1 五批发布链表
| 批次 | commit | 主题 |
|---|---|---|
| B1 feat | 4d58209b7 | feat(laiyipao): R72 搜索空态极端值溢出修复（.search-empty-text）|
| B2 test | ac7dfb45a | test(laiyipao): R72 搜索空态溢出契约 5it |
| B3 docs | a87931057 | docs(laiyipao): R72 规范与独立终审 |
| B4 audit | 923f9d3df | test(laiyipao): R72 真浏览器QA报告与截图 |
| B5 postdeploy | 本批 | chore(laiyipao): R72 部署后回归报告 |

每批均为逐文件路径精确 git add（禁 -A/点号），泄露守卫（带口令串连接/私钥/Bearer/Cookie）零命中，data/ 入库数 0，git diff --cached --check RC=0（仅 .gitignore LF→CRLF warning）。

## 2 产物 sha256（重建后实测，未回退）
| 产物 | sha256 | 说明 |
|---|---|---|
| H5 主 bundle index-Cr-I6ffw.js | 81d973577dc195e49170bc0498f0848907ca392c3571eb48869a05b21bcb97cd | 本轮 pnpm run build:h5 重建（RC=0），index.html:21 引用；R71 为 index-BJrAi0qp.js，已更替 |
| 后端 lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | 沿用 R21 基线，未回退 |
| admin WikiView-ST1PbAt2.js | f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b | 沿用基线，未回退（任务描述猜测的 JpQ4MFXA.js 本树不存在，以实际为准） |

## 3 部署后真浏览器复核（9 组：三视口 × 三页）
服务：API 8080（go run ./cmd/api，/api/v1/config=200）、H5 静态 5201（python http.server，/=200）。鉴权经 gitignored data/auth-init.js 注入游客态（不回显、不入库）。操作链路：navigate 到页面 → inject-search.js 用原生 setter 写入极端串（len=356）并 dispatch input/change → pd-probe.js 读 DOM → clear-search.js 点击「清除搜索」→ 再读探针确认复位。极端串由 init-script 侧合成（非真值）。视口 414×896 / 820×1180 / 1440×900 与 QA 的 390/768/1280 互异。

| 页面 / 视口 | cw = sw | hOverflow | 空态样式 display / word-break / max-width | 文本 sw = cw | 关键词完整 & <img> 字面 | badTokens | 清除后 |
|---|---|---|---|---|---|---|---|
| skills_skills / 414 | 414 | false | block / break-all / 100% | 355 / 355 | 是 | [] | emptyCount=0, allClear=[] |
| skills_skills / 820 | 820 | false | block / break-all / 100% | 444 / 444 | 是 | [] | emptyCount=0, allClear=[] |
| skills_skills / 1440 | 1440 | false | block / break-all / 100% | 506 / 506 | 是 | [] | emptyCount=0, allClear=[] |
| skins_skins / 414 | 414 | false | block / break-all / 100% | 355 / 355 | 是 | [] | emptyCount=0, allClear=[] |
| skins_skins / 820 | 820 | false | block / break-all / 100% | 444 / 444 | 是 | [] | emptyCount=0, allClear=[] |
| skins_skins / 1440 | 1440 | false | block / break-all / 100% | 506 / 506 | 是 | [] | emptyCount=0, allClear=[] |
| gems_gems / 414 | 414 | false | block / break-all / 100% | 355 / 355 | 是 | [] | emptyCount=0, allClear=[] |
| gems_gems / 820 | 820 | false | block / break-all / 100% | 444 / 444 | 是 | [] | emptyCount=0, allClear=[] |
| gems_gems / 1440 | 1440 | false | block / break-all / 100% | 506 / 506 | 是 | [] | emptyCount=0, allClear=[] |

9/9 组注入态 sw === cw、hOverflow=false，侦察基线（HEAD=501fa8733，skills/skins 六组合 sw 975/1920/1390）溢出全部消失；`.search-empty-text` 的 block / break-all / 100% 宽度约束在三视口生效。关键词含 Z×200 与 `<img src=x onerror=alert(1)>` 字面文本完整回显且文本自身 sw === cw（无内部溢出）。9/9 组 badTokens=[]（NaN/Infinity/undefined/[object Object] 全无），XSS 串被 Vue 插值转义为字面文本、无脚本执行。9/9 组点击「清除搜索」后 emptyCount=0、allClear=[]、hOverflow=false，CTA 可复位。

## 4 部署后新截图 sha256（9 张，实测，全互异）
| 文件 | sha256 |
|---|---|
| pd-skills_skills-414.png | 3a8ba24f0ba4a7441b908a34445cd2c9a49f48f0af18324493f9ef6831320dbf |
| pd-skills_skills-820.png | b82d4fadab574026fb4d350ee06eeaf9dafd1d42eec4724cd276487f7c32dbc7 |
| pd-skills_skills-1440.png | 89d2788312e4a16cf132107272b22580867daa512c7b5e75ac49a4f4e21c1f5a |
| pd-skins_skins-414.png | 9b87481d5dfee21964629f294457c5a8fc151c51a1a2fa4c71975d3d1e75585a |
| pd-skins_skins-820.png | 3d003e0b0f25a0592c7b29a0e7d4c178dae4b1cdd89d305f526fb37562c40946 |
| pd-skins_skins-1440.png | 626b18db7d641eca083a46884ffde5f87378290f6c19c467fedff3ef2beb7260 |
| pd-gems_gems-414.png | 73a268ed5abbdbede5b1298b6e47f7db897b2c3987355568df72467ac3ebf104 |
| pd-gems_gems-820.png | e407dd3397c31397f08505fb8fb7ef20f78113e75332025efd41d6b61bc69a08 |
| pd-gems_gems-1440.png | 5513af1b1a520988f870a2de7f2e8e96b1d5340d209debf877e96e439d454701 |

UNIQUE=9/9，全部 ≠ QA 15 图基线（视口互异 414/820/1440 vs 390/768/1280）。亲读 pd-skills_skills-1440.png / pd-skins_skins-820.png / pd-gems_gems-414.png 三张，确认为技能图鉴 / 皮肤图鉴 / 宝石图鉴页（非登录页）；pd-skins_skins-820.png 搜索框可见极端串「没有匹配-A…」完整回显且未撑破容器。

## 5 回归锚点（不回退）
- 令牌三锚点：tokens.css:14/15/16 逐字在（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6）。
- --r72-：全 src 仅契约 r72SearchEmpty.contract.test.ts:100 一处否定断言，业务源码 0 命中。
- R60 键盘：tabindex="0" 48 处、@keydown.enter 48 处；R61 aria：v-aria-native 17 处；R55 placeholder-class="ph" 21 处。
- R71/R66/R69：verify.vue:101 fmtVerifyTime(replayInfo.created_at) 在；normalizeGemGain / 诚实空态在；nicknameSubmitting / 提交中… 在。
- .search-empty-text 修复：common.css:360 及 7 页空态回显（skills:221 / skins:131 / announcements:96 / enemies:161 / equipment:219 / gems:155 / reactions:254）均在。

## 6 门禁与收尾
- 契约 R72：5/5 passed（QA 阶段复跑）；全量 vitest 223 files passed | 1 skipped、2049 tests passed | 2 skipped；vue-tsc RC0；pnpm run build:h5 RC0（本轮重建）；git diff --check RC0（仅 .gitignore LF→CRLF warning）。
- Go（本轮复跑）：TestNoInfraIdentifiersInRepo PASS（5.92s，INFRA_RC=0）；TestNoRealSecretsInRepo **SKIP 非 PASS**（缺 .local.keys，逐字「真值扫描这条守卫没有执行，不是『通过了』」，RC=0）。
- QA 库：docker 容器 lyp-qa-pg，battle_records count|max(id)=104|477（只读，未改）。
- 零真值：两处 data/auth-init.js 在 finalize 删除；git ls-files 无本轮 auth/data；五批泄露扫描零命中。

## 7 诚实边界
1. 极端串由会话侧 init-script 注入（非真值），仅用于语义验证；后端与服务端 config 未改。
2. 真浏览器复验覆盖 skills/skins/gems 三页 × 三视口共 9 组；其余 4 页（announcements/enemies/equipment/reactions）的同类回显由契约 it2 静态确认，未在部署后真浏览器逐页复验（QA 阶段另覆盖 announcements）。
3. 9 图 UNIQUE=9/9 且全 ≠ QA 基线；亲读仅 3/9（其余以 DOM 探针为证）。
4. Go TestNoRealSecretsInRepo 因缺 .local.keys SKIP 非 PASS，绝不记为通过。
5. guest_token 注入仅客户端会话，不改 QA 库数据。
6. 部署后报告自引用哈希悖论：本文件 sha 不纳入自身校验。

## 8 结论
R72 搜索空态极端值溢出修复在重建后的 H5 产物、部署后三视口真 Chromium 注入复验中全部符合预期，溢出消失、关键词完整、XSS 转义、清除复位，回归锚点、门禁与零真值收尾均无回退，判定 PASS（带上述诚实边界）。
