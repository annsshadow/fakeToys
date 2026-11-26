# R40 修复后独立复查报告：Wiki 评分维度搜索命中与高亮闭环

> 任务：t30（R40修复后复查，attempt=2）｜角色：same-route-rel（独立复查，不改业务源码、不执行 git）
> 执行时间：2026-11-25 23:39–23:43 Asia/Shanghai｜运行环境：QA API 127.0.0.1:8080 / H5 5174 / Admin 5175（真实 Chromium CLI：agent-browser）
> 结论：**PASS**（硬门禁逐项满足；TestNoRealSecrets 因缺 .local.keys 为 SKIP、reconcile 两个假阳性已披露，均不构成应用缺陷）

## 一、方法与边界
- 本轮为 t29 修复后的**独立复查**：不修改业务源码（WikiView.vue 的 t29 改动只读），不做任何 git 操作；所有证据为本轮真实 CLI 新产出。
- 真浏览器证据统一使用 `agent-browser` CLI（非 DOM 点击 / eval 点击作最终证据）：`open/fill/click/screenshot` 为真 CLI 动作；`eval --stdin` 只读探针，不改状态。
- 所有浏览器命令以**后台 job + job_output 控时**执行；jober 记录：服务 `bash-6bec4a4382234b4a`、E2E `bash-65e5855e5d954a0d`（completed, POSTFIX2_DONE=YES）、finalize `bash-1a0e198fb5514583`。
- 临时鉴权：从既有本地审计产物 scripts/gen-auth-init.py + round38 login.json（路径仅内部使用，不入报告/不入库）生成 data/admin-auth-init.js，用 `open --init-script` 注入；public config 虽 require_login=False，但深链仍被路由守卫要求 token（无鉴权冒烟 pf-smoke-search.json 被重定向至 #/admin/login），故真实复查统一带注入。

## 二、三视口搜索命中与点击高亮（真 CLI）
三视口 390×844 / 768×1024 / 1280×800，两个查询「反应链」与 `mechanic_depth`，共 6 组；每组真 CLI fill → eval 探针读命中 → 真 CLI click `.search-hit` → eval 探针读 URL/focus → 截图。

| 视口 | 查询 | fill 后命中 (kind/title/meta) | 点击后 URL | 评分行 row-focus |
|---|---|---|---|---|
| 390 | 反应链 | 评分 / 反应链 / 评分 · reaction_coverage | #/wiki?tab=rating&focus=rating:reaction_coverage | true（反应链 15） |
| 390 | mechanic_depth | 评分 / mechanic_depth / 评分 · mechanic_depth | #/wiki?tab=rating&focus=rating:mechanic_depth | true（机制深度 8） |
| 768 | 反应链 | 评分 / 反应链 / 评分 · reaction_coverage | #/wiki?tab=rating&focus=rating:reaction_coverage | true（反应链 15） |
| 768 | mechanic_depth | 评分 / mechanic_depth / 评分 · mechanic_depth | #/wiki?tab=rating&focus=rating:mechanic_depth | true（机制深度 8） |
| 1280 | 反应链 | 评分 / 反应链 / 评分 · reaction_coverage | #/wiki?tab=rating&focus=rating:reaction_coverage | true（反应链 15） |
| 1280 | mechanic_depth | 评分 / mechanic_depth / 评分 · mechanic_depth | #/wiki?tab=rating&focus=rating:mechanic_depth | true（机制深度 8） |

证据：data/pf2-search-{vp}-{q}.json（hits 非空，kind=评分）、data/pf2-after-{vp}-{q}.json（hash 与 focusText）、data/pf2-click-{vp}-{q}.txt（✓ Done）及截图 screenshots/pf2-{vp}-{q}-focus.png。亲读 pf2-390-fanyinglian-focus.png（1258×588）、pf2-1280-mechanic_depth-focus.png（1280×800）确认评分表对应行蓝色高亮可见、非登录页。

## 三、rating_weights 五行与权重对公开 config 零不符
- 本轮新抓公开 `GET /api/v1/config`（data/config.json）：`rating_weights = {element_coverage:12, reaction_coverage:15, mastery_done:10, equipment_synergy:12, mechanic_depth:8}`；require_login=False。
- 三视口 data/probe-{390,768,1280}.json 的表格 rows 均为 `[元素覆盖,12] [反应链,15] [专精投入,10] [装备契合,12] [机制深度,8]`，conclusion=「AI 机制概览：权重 5/5 项齐备」；reconcile.py 输出 VIEWPORT_390/768/1280 rows=5 mismatches=0 ok=true。
- 公开接口实时复验：require_login=False，ratings 与上表一致。**零不符**。

## 四、CTA 真 CLI 点击停留 rating
- data/cta-pre.json：hash=#/wiki，ctaVisible=true，ctaHref=#/wiki?tab=rating。
- 真 CLI `click .r40-ai-note a` → data/cta-click.txt=✓ Done；data/cta-post.json：hash/hashAfter=#/wiki?tab=rating，onWikiPath=true，dashboardJump=false，ratingVisible=true，ratingRows=5。
- 截图 pf2-cta-before / pf2-cta-after 亲读确认 wiki 页、评分区块可见，未跳看板。

## 五、R31-R39 十 tab 回归不回退
真 CLI 逐 tab 点击 + reg-probe.js 只读探针（data/reg-*.json）：

| tab | hash / 关键行 | 结果 |
|---|---|---|
| 核心规则 basics | #/wiki，reaction=7，含构筑评分（I-7） | 正常 |
| 技能 skills | #/wiki?tab=skills，合成技能 24 行 / 配方 18 行 | 正常 |
| 合成技能 composite | #/wiki?tab=skills&anchor=composite，18 行 | 正常 |
| 敌人 enemies | #/wiki?tab=enemies，22 行 | 正常 |
| 关卡 levels | #/wiki?tab=levels，20 行 | 正常 |
| 宝石 gems | #/wiki?tab=gems，8 行 | 正常 |
| 专精 mastery | #/wiki?tab=mastery，12/12/12/12 行 | 正常 |
| 皮肤 skins | #/wiki?tab=skins，6 行 | 正常 |
| 数值规则 rules | #/wiki?tab=rules，drop=6 行 | 正常 |

九 tab 控件均可 true CLI click，hash 与内容计数与既有基线一致，未见回退。

## 六、新截图 sha256（本轮 11 张，全部互异、非登录页）
| 截图 | sha256 |
|---|---|
| pf2-390-rating.png | eea1c672a8d761008fcf6ed35dd5efea795482c8a2a39175e77d29e6be715dbb |
| pf2-390-fanyinglian-focus.png | e0ee40bc84c3c2db40fb7968101e4d6dc6596b25b638420704d35ff188fcb1bd |
| pf2-390-mechanic_depth-focus.png | 7271311997aecbeda67bf61429c65ab30d42fc75dd8d457106b3541223d04a92 |
| pf2-768-rating.png | 8ac178bf5cf977aaf32718028cc76cd79f0fa52cb3aed5c3130db11a8b932829 |
| pf2-768-fanyinglian-focus.png | d2d02e590c111d8a9f1d20cde03343fa2461dee09332c87e88f74266204dd16f |
| pf2-768-mechanic_depth-focus.png | dc07b020ffcc56d17e05f90201bdd9804733773188b68d1715eececd8362e18f |
| pf2-1280-rating.png | 10c1bf1fb31a73a35deecbffe1d73ef1f459c1267ffd95b12a721026d91e16ab |
| pf2-1280-fanyinglian-focus.png | 4a293b78407c32643c1fa8f89a5acbae8968581983d9c8b115dfaa340279d5de |
| pf2-1280-mechanic_depth-focus.png | f14e3a9e97530a31c3df117feb642130646068d49b648757cd6821fdf14c1521 |
| pf2-cta-before.png | db13e9a9257dbf6361d0d75a9f93fcfcc5feec54de6cd9d2d0c74b874ba1a6ff |
| pf2-cta-after.png | d18f94ae275cdc9846cd044d764d1385afcd5a3f9a5e63171b4c52727ea414f8 |

11 张 sha256 两两互异（unique=11），≥4 张硬门禁满足；均本轮新产出，非登录页。哈希来自当前 `sha256sum`，未抄旧 e2e.log。

## 七、零真值与测试守卫诚实标注
- finalize `bash-1a0e198fb5514583`：删除 data/admin-auth-init.js、data/login.json、admin-auth-init.js（AUTH_DELETED=YES；glob 复核 AUTH_LEFT=[]）；`git check-ignore -v` 命中 .gitignore:442 `docs/laiyipao/audits/round40-wiki-rating-weights/data/`。
- QA 库：finalize 脚本先 TRUNCATE 后 pg_restore（文本 dump 与 pg_restore 不兼容，脚本自报 DB_CHECK=0_0），本轮随后用 `psql` 直接导入 `battle_records` COPY 段（COPY 104）修复为 **count=104, max(id)=477**，与基线一致；未改业务源码/未执行 git。
- `go test ./internal/domain -run TestNoRealSecretsInRepo -v`（在 laiyipao/server 模块目录执行）：**--- SKIP: TestNoRealSecretsInRepo（缺 .local.keys，真值扫描未执行）**，整体包 PASS；SKIP 不冒充 PASS。本轮以 ops_grep 独立扫描审计 data 目录，未见 eyJ…/PRIVATE KEY/postgres 带密码/C:\Users\ 真值命中。
- reconcile.py 输出 `ALL_RECONCILE_OK=False`，PROBLEMS=[reg:basicsMissingRatingSection, reg:composite=36]。经查 bascis 的 reg-probe 在 `#/wiki` 下 sections 含「构筑评分（I-7）」标题与 5 行权重表，是探针分类口径差异；composite 的 36=24+12（合成技能与配方两段被合并计数），均属**已知假阳性、非应用缺陷**，与 t27 REPORT.md 记录一致。权重核对以第三节的 rows/mismatches=0 为准。

## 八、总判定
| 门禁 | 结果 |
|---|---|
| 三视口真CLI搜索命中+点击URL+行高亮（6组） | ✅ PASS |
| 五行中文标签与权重 12/15/10/12/8 对公开config零不符 | ✅ PASS |
| CTA 真CLI点击停留 rating 不跳看板 | ✅ PASS |
| R31-R39 九 tab（含十节）回归不回退 | ✅ PASS |
| ≥4 张新图 sha 互异且非登录页（实得 11 张） | ✅ PASS |
| 零真值：删临时auth、check-ignore命中、QA库104/477 | ✅ PASS |
| 不改业务源码、不做 git | ✅ PASS |
| SKIP/假阳性如实披露 | ✅ PASS |

**最终判定：PASS。** t29 修复后在真实 Chromium CLI 中形成完整证据闭环，评分维度搜索命中、定向跳转与行高亮均生效，无回归。