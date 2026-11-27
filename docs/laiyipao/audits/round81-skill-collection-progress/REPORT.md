# R81 技能图鉴收集进度条一致性美化 — 真浏览器 QA 报告

- 轮次：R81 QA（verification），same-route-qa
- attempt_id：c59f9fda-ed6e-48f2-9f16-3a2233263444
- 基线 HEAD：74a895223（工作树已含 t177 实施 skills.vue +6/-0、t178 契约修复）
- 日期：2026-11-27
- 工具：agent-browser 0.41.1（真 Chromium）、静态 H5 5201、API 8080（回环）
- 判定：**PASS**

## 1. 目标与口径

独立真浏览器坐实 skills 页 KPI 区后新增的 `.r3-progress-row` 收集进度条真实可见、绑定非零百分比、零横向溢出，G66 覆盖矩阵与既有不变量完好，并独立复跑全部门禁与回归守卫。修复源码（QA 只读核实）：`laiyipao/miniapp/src/pages/skills/skills.vue:52-55` 模板进度行，`:494` `const r3SkillPct = computed(() => skills.value.length ? Math.round(ownedCount.value / skills.value.length * 100) : 0)`。权威范式 equipment.vue:34-38 / :412（含 `is-ai`，skills 版按规范不用）。

## 2. 三视口 DOM 探针实测

探针脚本 data/r81-probe.js，导航 `#/pages/skills/skills`，等待渲染 6 秒后读取。结果落 data/probe-{390,768,1280}.txt。

| 视口 | iw/cw/sw | hOverflow | afterKpi | beforeFilter | labelText | numText | fill style | fill computed | track width |
|---|---|---|---|---|---|---|---|---|---|
| 390×844 | 390/390/390 | false | true | true | 收集进度 | 已解锁 7/24 | width: 29% | 60.9531px | 210.203px |
| 768×1024 | 768/768/768 | false | true | true | 收集进度 | 已解锁 7/24 | width: 29% | 60.2656px | 207.844px |
| 1280×800 | 1280/1280/1280 | false | true | true | 收集进度 | 已解锁 7/24 | width: 29% | 112.219px | 387px |

关键结论：
- 进度行位于 `.r3-kpi` 之后、`.r3-filter` 之前（afterKpi=true / beforeFilter=true，用 compareDocumentPosition 位判定），与规范 §2 插入位置一致。
- 标签「收集进度」、计数「已解锁 7/24」、填充 `width: 29%` 三视口一致；computed 宽度随视口自适应（约 29%×track）。
- **非零填充来自真实账号 /me 快照**（store.build.skills 含 7 项 / 共 24），非 mock 注入；比规范 §6「无后端时 ownedCount=0→0% 属预期」更进一步，坐实真实非零百分比渲染。
- 零横向溢出：三视口 iw===cw===sw，hOverflow=false。
- progText 实测「收集进度已解锁 7/24」，与 DOM 结构一致。

## 3. G66 矩阵与不变量（同批探针）

| 视口 | skill-matrix | covered 格 | 总格 | 标题「元素 × 类型覆盖」 | 筛选 chip 数 | active chip | 结果计数 |
|---|---|---|---|---|---|---|---|
| 390 | true | 5 | 10 | true | 6 | 全部 24 | 显示 24 / 共 24 条 |
| 768 | true | 5 | 10 | true | 6 | 全部 24 | 显示 24 / 共 24 条 |
| 1280 | true | 5 | 10 | true | 6 | 全部 24 | 显示 24 / 共 24 条 |

- G66 元素×类型覆盖矩阵（5 元素 × 主动/被动 = 10 格）保留，covered 高亮 5 格，标题在，未被进度条替代。
- 筛选 chip 6 个（全部/焰/冰/电/毒/动能），active=「全部 24」；结果计数「显示 24 / 共 24 条」在。
- 搜索占位：首轮探针用 `input[placeholder]` 未命中（phHits=[]），补探针 data/r81-dumpinput.js 坐实根因——uni-app 编译后搜索框 placeholder 落成 `aria-label="搜索技能名称 / 元素 / 主动被动"`（input.uni-input-input，maxlength=140），并非 placeholder 属性；不变量（搜索入口文案）实际保留，契约 it5 静态断言也确认 `placeholder="搜索技能名称 / 元素 / 主动被动"` 逐字在源码。首轮 phHits=[] 系探针选择器口径，非业务回退，已用 dump 探针澄清。
- filter aria-label 实测「按元素筛选技能」，与契约一致。

## 4. equipment 权威范式并排对照

探针 data/r81-probe-eq.js，导航 `#/pages/equipment/equipment`，结果落 data/probe-eq-{390,768,1280}.txt。

| 视口 | 进度行 class | 是否 is-ai | label | num | fill style |
|---|---|---|---|---|---|
| 390 | r3-progress-row is-ai r3-enter | true | 收集进度 | 已拥有 0/18 | width: 0% |
| 768 | 同上 | true | 收集进度 | 已拥有 0/18 | width: 0% |
| 1280 | 同上 | true | 收集进度 | 已拥有 0/18 | width: 0% |

- skills 版与 equipment 版视觉同构：均为 r3-progress-row r3-enter + 收集进度标签 + r3-progress-fill 绑定百分比 + 计数文案。
- 差异符合规范：equipment 带 `is-ai`（AI 专属），skills 不带；skills 文案「已解锁」、equipment「已拥有」。
- equipment 页 ownedCount=0/18→0% 为真实空态（该账号无装备），非缺陷。

## 5. 截图证据

三视口各 1 张，落 screenshots/。sha256（与报告逐字一致）：

| 文件 | sha256 |
|---|---|
| r81-skills-390.png | c370ad7a03930fe0222bd6c3c480988b7cd1cfe72958842dd823c3ad56394207 |
| r81-skills-768.png | e9bc62e4d8254bede9165f72f99d7b9680509c5110af17583d8a9adb0fe0fdb5 |
| r81-skills-1280.png | f2765b73e747951426be46de0237735e11665598f9ea3b819553f02fb76630da |

UNIQUE=3/3（三张全互异）。

亲读 2 张非登录页确认：768 与 1280 均为「技能图鉴」页面（可见「技能构筑台」「AI 当前最该强化·跨系统优先级」「当前构筑可触发反应」「AI·构筑反应诊断」等），1280 截图中可见 KPI「7/24 已解锁/技能总数」「18 合成技能」、收集进度条（红橙渐变填充约 29%）及「收集进度 已解锁 7/24」，筛选 chip 与内容卡正常；非登录页、无白屏、无错位。390 仅列 sha 未逐字亲读（视口窄，进度条在首屏以下，DOM 探针已坐实）。无作废/失败样本。

## 6. 回归守卫（静态）

- 令牌三锚点：tokens.css:14 `--lyp-token-version: lyp-tokens-v1` / :15 `--lyp-r3-version: lyp-tokens-v3-r3` / :16 `--lyp-r6-version: lyp-tokens-r6` 逐字在。
- 业务源码零 `--r81-` 令牌（grep 排除契约后 NONE）。
- 键盘锚点实测（OPS grep 全 src）：`role="button"` 51 处、`tabindex="0"` 51 处、`@keydown.enter` 50 处、`placeholder-class="ph"` 17 处（历史记为 48，本轮按实测记录：跨轮累积已增长，未回退）。
- 不变量在：verify.vue:101/:433 fmtVerifyTime（R71）、gems.vue:169/:190 normalizeGemGain（R66）、me.vue:94/:97 nicknameSubmitting（R69）、skills.vue:226 search-empty-text（R72）。
- 写域边界：`git diff --name-only HEAD` 仅 `.gitignore` + `laiyipao/miniapp/src/pages/skills/skills.vue` 两项（均预期）；审计目录 UNTRACKED 未入库。

## 7. 门禁（独立复跑，全 RC0）

| 项 | 命令 | 结果 |
|---|---|---|
| G1 R81 契约 | pnpm exec vitest run src/styles/r81SkillCollectionProgress.contract.test.ts --reporter=verbose | 1 passed / 6 passed，RC0 |
| G2 全量 vitest | pnpm exec vitest run --reporter=verbose | 225 passed | 1 skipped (226) files；2061 passed | 2 skipped (2063) tests，RC0 |
| G3 类型 | pnpm exec vue-tsc --noEmit | RC0 |
| G4 构建 | pnpm run build:h5 | RC0（Compiler 5.24 vue3，DONE Build complete） |
| G5 diff | git diff --check | RC0（仅 .gitignore LF→CRLF warning） |
| Go NoInfra | go test ./internal/domain/... -run TestNoInfraIdentifiersInRepo -count=1 | PASS 23.577s，RC0 |
| Go NoRealSecrets | go test ./internal/domain/... -run TestNoRealSecretsInRepo -count=1 -v | **SKIP 非 PASS**，逐字：「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」 |

G1-G5 逐条 RC 见 data/gates-out.txt；Go 见 data/go-out.txt 与 verbose 复核。全部门禁在 laiyipao/miniapp（前端）与 laiyipao/server（Go，go.mod 所在）下执行。

## 8. 诚实边界与 hygiene

- 本轮为真浏览器复查，不改业务源码、契约、令牌、CSS；只读核实源码 + 写本审计目录。
- 非零进度 29% 来自真实账号 /me 快照（已解锁 7/24），非 mock 合成；未额外构造 localStorage ownedSkillIds mock（真实数据已覆盖非零场景）。
- 搜索占位首轮探针 phHits=[] 系选择器口径（编译后为 aria-label），已用 dump 探针澄清，非缺陷、非作废业务样本；补探针落 data/probe2-768.txt 与 data/dumpinput-768.txt。
- 截图亲读 2/3（768/1280），390 仅列 sha。
- auth-init.js 由 QA 库 guest_token 换取令牌经 init 脚本生成于 gitignored data/（.gitignore:597 命中），报告不写令牌真值；收尾删除至 ABSENT。
- QA 库只读：select id,nickname from users where id in (104,477) → 实测 477|rc_shd 存在，104 无记录（历史各轮并列写「104|477」，本轮按实测如实标注）。
- git 非 ?? 变更仅 .gitignore + skills.vue（预期两项）；零真值入库；自启 job 全部 kill。
- hOverflow 定义：iw===cw 且 sw<=cw+1；三视口实测均满足。

## 9. 结论

R81 skills 收集进度条真浏览器三视口验证 **PASS**：进度条位于 KPI 与筛选行之间、文案/绑定/非零填充正确、零横向溢出、G66 矩阵与筛选/计数/搜索不变量完好、与 equipment 权威范式视觉同构（无 is-ai 差异属预期）；门禁与回归守卫全绿，Go 真值守卫 SKIP 如实标注。可进入终审。
