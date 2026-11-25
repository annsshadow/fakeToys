# R53 独立终审：首页 dims.pct 取整消除 AI 结论长浮点

> 终审员：same-route-reviewer（独立终审，队长同路由）
> 对象：t79 实施（index.vue dims 五维 pct 加 Math.round）与 t80 真浏览器 QA
> 规范：docs/laiyipao/specs/round53-home-pct-rounding/SPEC.md（57 行）
> 契约：laiyipao/miniapp/src/styles/r53HomePct.contract.test.ts（84 行）
> QA 报告：docs/laiyipao/audits/round53-home-pct-rounding/REPORT.md（65 行，t80 PASS 带诚实边界）
> 终审日期：2026-11-26

## 0. 独立性声明与总判定

全程只读取证：亲读 SPEC/契约/QA 报告/index.vue diff 与不变量区块、审计证据；独立复跑全部门禁；sha256 亲算比对；8 张截图全部亲读；未改任何业务源码、未执行 git 写操作。唯一落盘产出为本报告。

**总判定 verdict = PASS（可发布）**。六维门禁全绿，无阻塞项；诚实标注均为非阻塞观察。

## 1. 维度一：SPEC 与契约逐条对照

| 检查项 | 结论 | 证据 |
|---|---|---|
| SPEC 自洽、缺口真实 | PASS | 57 行，§0 逐字列 index.vue:485/492/499/506/513 五条裸浮点 pct（8/18×100=44.44444444444444）与 :75 结论模板直出；权威对照 r12GrowthPriority.ts:68 Math.round(Math.min(100,(cur/cap)*100))；:522 weightPct 与 :547 ratingGrade.pct 已取整唯 dims 漏网；§1 修复判据、§2 不变量、§6 RED 摘要齐备 |
| 契约真实读源码、无内嵌实现 | PASS | 契约 84 行，import node:fs readFileSync 与 node:path join，readSrc 读 index.vue 与 tokens.css；dimsBody() 截取 const dims 至 const weaknesses 区块，pctLines() 过滤 pct: 行；真实 import ../pages/_shared/r12GrowthPriority 用 8/18 案例断言 pct=44 整数 |
| 4 个 it 覆盖缺口与不变量 | PASS | ①五条 pct 全含 Math.round、无裸 pct: (( 与 pct: Math.min(100, (( 模式 ②AI 结论模板 {{ aiInsight.weak.pct }}% 与 aiInsight 取 dims.value.find 不重算 pct ③helper 8/18→装备契合 44 整数、首页对应行含 Math.round ④marginalGain=weightPct×gap 排序、进度条 width: d.pct、雷达 dims.value[i]?.pct、空态 d.pct===0、令牌三锚点与无 --r53- |
| RED 曾真 | PASS（依 SPEC 实测记录与 GREEN 现状采信） | SPEC §6 记载真跑 exit=1、1 failed file、2 failed|2 passed、Duration 334ms；两条逐字失败：契约 :38『pct 裸浮点未取整: pct: ((r?.element_coverage ?? 0) / 5) * 100, expected to contain Math.round(』、:67『首页装备契合 pct 必须 Math.round 取整，与 helper 44% 一致』；无 Transform failed/模块缺失/超时；原始 RED 日志未单独留存 |

## 2. 维度二：实现 diff 严格仅 index.vue 五处

git diff --stat tracked 仅 .gitignore（+8 R53 预置 ignore 规则，含 :506 data/）与 laiyipao/miniapp/src/pages/index/index.vue（+5/-5）。index.vue diff 精确 5 行替换，全部位于 dims computed：

- 元素覆盖 pct: Math.round(((r?.element_coverage ?? 0) / 5) * 100)
- 反应链 pct: Math.round(((r?.reaction_coverage ?? 0) / 7) * 100)
- 专精投入 pct: Math.round(((r?.mastery_done ?? 0) / 8) * 100)
- 装备契合 pct: Math.round(Math.min(100, ((r?.equipment_synergy ?? 0) / 18) * 100))（保留上界再取整，等价权威 helper :68）
- 机制深度 pct: Math.round(((r?.mechanic_depth ?? 0) / 8) * 100)

不变量亲读确认全未动：weightPct 仍 Math.round 且除零兜底 0；ratingGrade.pct 仍 Math.min(100,Math.round(...))；marginalGain 仍 d.weightPct×gap + sort((a,b)=>b.gain-a.gain)；aiInsight 仍 dims.value.find 取 weak；AI 结论模板逐字 {{ aiInsight.weak.pct }}%（权重 {{ aiInsight.weak.weightPct }}%）：优先补它，收益 {{ aiInsight.gain }} 不变；雷达 radarPolygonActual 仍消费 dims.value[i]?.pct；进度条 width: d.pct + '%'；空态 d.pct === 0 && d.hint；DIM_HINT 五维导航与 go(aiInsight.hint) 不变。

保护面：后端 / engine / replay / hash / score / settle / loot / tokens 零改；tokens.css:14-16 三锚点原样（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6），全 src 无 --r53- 新增；.gitignore 仅审计 ignore 属预置非业务保护面。

## 3. 维度三：QA 报告逐条复核

- 三视口：case1(390×844)/case2(768×1024)/case3(1280×800) probe conclusion 逐字『装备契合仅 44%（权重 21%）：优先补它，收益 210』、longFloat=false、innerW==docW 无溢出。收益序列 mg=[装备契合(还差10)收益210, 反应链(还差4)收益104, 元素覆盖(还差0)收益0]。
- 五维进度条/雷达：element100%/reaction43%/mastery100%/equipment44%/mechanic100%（全整数），雷达 viewBox=0 0 200 180、rings=3、axes=5、五中文标签不溢出、shape 多边形可见；空 case 五维 width 0% 且引导『→ 去背包配元素』。
- 回归 R12/R51/R48：skills/equipment/gems/bag 四页均判同短板装备契合（r12Text 8/18 44% 权重21% 优先补它），equipment 页 CTA『在本页强化装备契合 →』保持自指（R48），其余三页『去配装备 →』；me 页整数 44% + 雷达 viewBox/rings/axes/honesty 全保留（R51）；元素短板 case 技能页 CTA『去背包配元素 →』与权威 helper 一致。
- 8 图 sha256 亲算与 REPORT §三逐行一致且两两互异：

| 截图 | 视口 | sha256 |
|---|---|---|
| r53-case1 | 390×844 | 028a4520f60de3513f64bcfdc53e84bd7dd3517915c6947e34b875c10cb06417 |
| r53-case2 | 768×1024 | e05869677508c8e0e3eb79f80e2fa63da60c627debfe128f5f497befcf5f68fd |
| r53-case3 | 1280×800 | f744ad3e470d107a904ed0a8b2040b5d5a663628065ad3b807c13fc75ec832e6 |
| r53-empty | 390×844 | dc9b9c1247c4bd92648f26914db839b0da7d5076a48d5483baea5bb5ef0a4719 |
| r53-reg4-equipment | 390×844 | edfa76305e2fc1a99e5323262114e6a57cd8006e07cadafab977c8681b66c2f4 |
| r53-reg4-me | 390×844 | db55d757690e7ae28426dd12f224d59073dcf6ae9f0a12e3b5a818021871d01e |
| r53-reg4-element | 390×844 | dd7eb60c9cfae15a8c62d1bf6a026a018421c0dc74a9aeade6d87881c2c8845c |
| r53-reg4-skills | 390×844 | e23de2aac9306ed6357baf701ebcdbb6d566f138504210e2d60ec57da3d3eab3 |

8 张全部 read_image 亲读、均为非登录页：case1/2/3 见 AI 战术洞察『装备契合仅 44%』整数结论与去补强 CTA、构筑评分 C165 与雷达；empty 见『装备契合仅 0%』与 0 分空雷达+去背包配元素入口；reg4-equipment 见装备图鉴自指 CTA『在本页强化装备契合 →』；reg4-me 见 me 页 R51 雷达与整数 44%；reg4-element/skills 见技能图鉴 R12 短板一致。

## 4. 维度四：独立复跑门禁

| 命令 | 结果 |
|---|---|
| cd laiyipao/miniapp && pnpm exec vitest run src/styles/r53HomePct.contract.test.ts | 1 file passed、4 tests passed、RC=0（433ms） |
| cd laiyipao/miniapp && pnpm test | 214 files passed|1 skipped(215)、2006 tests passed|2 skipped(2008)、RC=0（26.43s，前台重跑统计行确认；stderr build.attacker 占位告警非失败） |
| pnpm run type-check && pnpm run build:h5 | exit=0、DONE Build complete（uni build 5.24） |
| git diff --check（R53 自有文件） | exit=0 零告警（全树仅 .gitignore LF→CRLF 既有 warning） |
| Go TestNoInfraIdentifiersInRepo | PASS（5.07s） |
| Go TestNoRealSecretsInRepo | SKIP 非 PASS（缺 .local.keys，逐字『真值扫描这条守卫没有执行，不是通过了』） |

QA 报告记全量 2006 passed|2 skipped，与本轮独立复跑一致；GREEN 契约 4/4 与 RED 2 failed|2 passed 计数演变正常。

## 5. 维度五：零真值收尾

- auth 注入五路径（审计 data/admin-auth-init.js、data/login.json、data/auth-init.js、根 admin-auth-init.js、login.json）ops_bash 全 ABSENT。
- git check-ignore -v 命中 .gitignore:506 docs/laiyipao/audits/round53-home-pct-rounding/data/。
- QA 库 docker exec lyp-qa-pg（psql -U postgres -d laiyipao）查 battle_records count|max = 104|477，与基线一致。
- 令牌三锚点 tokens.css:14-16 原样；全 src 扫 --r53- 仅契约否定断言（契约第 6 行注释 + 两处 expect().not.toContain），无实际新增。
- 保护面：tracked diff 仅 .gitignore 与 index.vue，后端/engine/replay/hash/score/settle/loot/tokens 零改。

## 6. 诚实标注（非阻塞观察）

1. TestNoRealSecretsInRepo 为 SKIP 非 PASS：缺 .local.keys，真值扫描守卫未执行，不冒充通过。
2. RED 原始日志未单独留存；依 SPEC §6 逐字实测记录（2 failed/2 passed、失败行 :38/:67）与本轮 GREEN 现状交叉采信。
3. QA 的 build_rating / config 为浏览器会话侧 network route 注入（mock），未改 DB 或服务端；权重与服务端权威 {12,15,10,12,8} 同值。
4. 空 rating 分支未触发：mock build_rating:null 时首页仍由 ?? 0 兜底产出 aiInsight（装备契合仅 0% 收益 378、五维 width 0%），SPEC §4 期望的『AI 暂时没有足够构筑数据』文案未出现，QA 已如实披露不伪称触发。
5. 早期 driver 缺陷（case2/3 写成 case1、skills/ 尾斜杠、$4 未传、echo 被 JS 模板提前展开）已修为 driver4；旧无效探针与截图剔除。
6. 重复截图剔除：reg4-me 与 R51 case1 同 sha（db55d757…）、reg4-bag/gems 与旧 driver 相同，只列最终 reg4 集合一次。
7. 端口纪律：5174 旧 302 服务不用，静态 5201/API 8080 打开前均验证 HTTP 200，避免 chrome-error 错误页证据。
8. reg4-me 与 R51 case1 同 sha 说明 me 页未因本轮修复改变视觉，属预期（R53 只改首页 pct），非重复计数。

## 7. 结论

六维门禁全部达标：SPEC/契约真实且 RED 曾真、diff 严格限 index.vue 五处 pct、QA 三视口整数 44% 与回归成立、独立复跑全绿、零真值与保护面达标、8 图 sha 一致且全部亲读非登录页。裁决 verdict=PASS，可进入分批发布流程。非阻塞观察见第 6 节，不影响发布判定。
