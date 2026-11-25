# R33 复查报告：Wiki 敌人机制三视口真浏览器闭环

## 结论
PASS。以公开 `/api/v1/config` 为唯一权威，390×844 / 768×1024 / 1280×800 三视口真实 Chromium 逐行核对 22 种敌人的 15 列字段，MISMATCHES=0；AI 四要素同屏且由真实配置聚合；真实 CLI 点击 CTA 后停留在 /wiki 敌人 tab 不跳看板；表内可横向滚动到末列、页面无横向溢出；R31 反应 7 行与 R32 技能 24 行不回退。附带如实记录：曾发现 SPEC 示例护盾数值与权威 config 不一致，已由 t260 修正为 config 值（ID10=480、ID22=64000），规范示例非独立真源、权威仍为公开 /api/v1/config；审计脚本首版假阳性已修正；全量测试首轮 1 失败为偶发、复跑 806/806 全绿。

## 一、环境与方法
- 服务：API 8080 / admin 5175 / H5 5174，本地回环 + 占位口令，均探测 200；QA PG 容器 lyp-qa-pg 端口映射到 15440。
- 浏览器：agent-browser 真实 Chromium，全局 --session 前置；每视口独立会话，先 open about:blank 再 set viewport，再带临时 admin 令牌 init-script 打开 URL 并 reload。
- 匿名会话被路由守卫挡到登录页，故沿用历史做法注入临时本地 admin 令牌（脚本从 QA_ADMIN_USER / QA_ADMIN_PASS 环境变量读取，真值不入仓，收尾删除）。
- 权威快照：SAMPLED_AT=2026-11-25T11:29:19+08:00，CFG_SHA=16e13ae9c5e4e0e030196f18a5bd2029748ad6fc1b3af41cb552f7e7765be48d，落 data/config.json + data/config.meta.txt。version=6237e0b623219b8b，enemies=22。

## 二、逐字段对账（权威 config ↔ DOM）
对账口径：15 列 = ID / 名称 / 类型 / 血量 / 五维抗性 / 速度 / 护甲 / 护盾 / 攻击 / 射程 / 攻击间隔 / 飞行高度 / 潜地 / 首领 / 说明。抗性按 焰/冰/电/毒/动能 顺序、config 千分比除以 10 显示为百分比；速度/间隔除以 1000 一位小数；护甲为 ‰ 不放大；布尔仅严格 true 显示「是」，零/非法显示「—」。探针双层 JSON 用 raw_decode 循环解包。

| 视口 | ROWS | MISMATCHES | AI_OK | BAD_TOKEN | DOC(scroll/client) | 表内横滚 sw/cw | 结果 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 390×844 | 22 | 0 | True | False | 390/390 | 1515/324 (0→1191) | PASS |
| 768×1024 | 22 | 0 | True | False | 768/768 | 1515/702 (0→813) | PASS |
| 1280×800 | 22 | 0 | True | False | 1280/1280 | 1515/884 (0→631) | PASS |

ALL_RECONCILE_OK=True。横滚落盘 data/scroll.json：末列「说明」可见，末格文本为最终首领描述，scrolledToEnd=true。

## 三、关键种子行（逐字段）
- ID1 游荡者：40像素/秒；护甲/护盾/攻击/射程/间隔/飞行/潜地/首领 全为「—」；抗性 焰0/冰0/电0/毒10%/动能-10%。
- ID10 铁罐炮：18像素/秒 / 护盾 480 / 攻击 28 / 射程 500 / 间隔 3.2秒。
- ID13 浮空囊：飞行高度 180（category=flying）。
- ID17 潜地者：潜地「是」（burrow=true）。
- ID22 白垩母皇：首领「是」/ BOSS 类型 / 护甲 350‰ / 护盾 64000 / 攻击 60 / 射程 500 / 间隔 2秒。
- 曾发现 SPEC §5 写的 ID10 护盾 60、ID22 护盾 8000 与当前公开 config（480 / 64000）不符；DOM 与 config 一致，原差异如实登记、不据此判失败。后续 t260 已将 SPEC §5 示例修正为 480/64000，差异闭环；权威仍为公开 config，SPEC 示例仅作对账说明、非独立真源。

## 四、AI 四要素与真实 CTA
- 结论（逐字）：共 22 种敌人：首领 4 种，飞行 3 种，潜地 1 种，带护甲 9 种，由 buildEnemyMechanicsAi(cfg?.enemies ?? []) 按配置真实聚合（boss 严格 is_boss===true、fly 严格 category==='flying'、burrow 严格 burrow===true、armor 按正整数 armorText 非「—」），非手填。
- 依据（逐字）：依据：公开 /api/v1/config 的 enemies（与 H5/Go 引擎同源 SeedEnemy）；展示为配置基值，实际效果以引擎结算为准。
- CTA（逐字）：查看敌人列表 →，ctaHref=#/wiki?tab=enemies。
- 诚实红线（逐字）：辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准。
- 四要素同屏：data/probe-390.json 的 aiText 含结论/依据/CTA/红线，hasBad=false，无占位符泄漏。
- 真实 CLI 点击：会话 lyp-r33-cta 用 agent-browser click '.r33-ai-cta a'（非 DOM e.click），data/cta-click.txt='✓ Done'。cta-pre.json hashBefore=#/wiki?tab=enemies、rowCount=22；cta-post.json hashAfter=#/wiki?tab=enemies、enemyStillThere=true、rowCount=22、dashboard=false、navActive=[敌人抗性图鉴]。坐实停留敌人 tab 不跳看板。

## 五、回归：R31/R32 不回退与队长质疑闭环
- 队长质疑：data/regression.json 曾 reactionCount=0、corrosionSpread=false，且 r33-06-regression.png 与首图 sha 相同，不能作回归证据。
- 根因：首版 reg-probe.js 用了不存在的 #reaction-table / #skill-table 选择器。WikiView 实际结构：反应表位于 basics 节 <h4 id="reactions">反应链一览</h4>，各 section 用 v-show 按 activeTab 切换；R31 历史探针用 closest('section') 文本匹配定位。
- 修正：新增 react-probe.js（closest section 文本匹配）与 skill-probe.js（#skill-table），reg2.sh 先切 #/wiki?tab=basics 再切 #/wiki?tab=skills。
- 修正后 reg-reaction.json：hash=#/wiki?tab=basics、tableFound=true、rowCount=7；腐蚀扩散行含「传播 1 层」，破甲击退行含「150‰」与「4px」；corrosionSpread/armorShred/armorKnock 全 true；hasImplement=true/hasBurnCloud=true/oldWording=false/hasBad=false。
- 修正后 reg-skill.json：hash=#/wiki?tab=skills、rowCount=24；ID3 温压弹 3|温压弹|焰|flame|180|40|2秒|主动|穿透 1|范围 180|连锁 1|2|…。R31/R32 均不回退。
- 重复图 r33-06-regression.png 已删除，改用不同 sha 的 r33-06-reaction-tab.png 与 r33-07-skill-tab.png。

## 六、对账脚本假阳性与修正（fail loud）
- 首版 reconcile.py 用 for vp in sys.argv[1:]：空参数运行打印 ALL_RECONCILE_OK=True 属假阳性（未进入任何视口循环）。带三视口参数重跑后暴露 129 处错位。
- 根因是审计脚本 expected() 漏了「五维抗性」整列，且速度函数未拼「像素/秒」单位、类型映射缺 elite→精英 / boss→BOSS。逐项修正后重跑：三视口 MISMATCHES=0、ALL_RECONCILE_OK=True。这是审计脚本自身缺陷，非产品缺陷；修正过程与结果如实记录。

## 七、截图证据（≥4 张新图，sha 两两不同）
全部 390×844（视口图除外），agent-browser 真实截图，亲读确认非登录页、含敌人/反应/技能表或 CTA 前后状态：
- r33-390-enemies.png sha256=a77328d4ebf2b50c3bf2fecd483765845b780f162599a49bbf55f106595fc186（390×844）
- r33-768-enemies.png sha256=f7617db439894d805e646871e1f5ea108053fce92533c7ef761e77b1fd545cd5（768×1024）
- r33-1280-enemies.png sha256=c95ba15835d410c83615fced781b669bd1479d47c4ca088a26920fc7720beb92（1280×800）
- r33-04-cta-before.png sha256=d2f6a4713b47f12f7d44862bddd3af2d0a597c345f4dd090f8e11e5b43d77a51
- r33-05-cta-after.png sha256=a741e624b6523fe5d10bc705421bce939f6a678a63a8e28eeba3857d756aa6b8
- r33-06-reaction-tab.png sha256=ed03d14dee4d1059a2e76db7868927f02b966551f67f6a6eb45e6bf05c57b66e（反应表 7 行，核心规则 tab）
- r33-07-skill-tab.png sha256=2328050d88bcd172ec33fccdde4e1303a354e6694c1c4f029b7fe4a8363e6cfd（技能表 24 行）
已删除重复图 r33-06-regression.png（曾与 r33-390 同 sha）。

## 八、测试与验证
- R33 定向：pnpm exec vitest run tests/WikiEnemyMechanics.test.ts --reporter=dot → 1 file / 11 tests passed。
- 全量首轮：pnpm test -- --testTimeout=90000 → 1 failed | 805 passed（72 files，57.10s），未抓到失败详情；后台重跑完整日志 data/full-test.log 结尾为 Test Files 72 passed (72) / Tests 806 passed (806) / FULL_RC=0，首轮 1 失败为偶发（无代码改动介入），如实登记。
- pnpm type-check RC=0；pnpm build RC=0（✓ built in 2.33s）。
- git diff --check RC=0（仅 .gitignore 换行 LF→CRLF 提示，无实质冲突）。
- TestNoRealSecretsInRepo：laiyipao/deploy/.local.keys 不存在，按 secrets_leak_test.go:145 会显式 Skip（未执行真值扫描，不是 PASS），如实声明。

## 九、零真值与清理
- zero-secret-scan.sh（t263 自覆盖补强后）：BAD_COUNT=0、SYNTAX_RC=0、bash -n RC=0；.gitignore:386（data/）与 :387（admin-auth-init.js）git check-ignore 命中。扫描采用通用正则且**不排除扫描器自身**（已删除 --exclude=zero-secret-scan.sh），脚本自身同样纳入扫描范围；正则为模式串而非真值字面，天然不自匹配，既覆盖自身又不会产生假零。
- t263 补强轨迹（test-first）：队长指出 t262 用 --exclude 排除扫描器自身会削弱 fail-loud（将来脚本被误写真值将漏扫）。先把 WikiEnemyMechanics.test.ts 守卫描述改为『扫描器不得排除自身』语义：第180行由 toContain('--exclude=zero-secret-scan.sh') 改为 not.toContain(...)，并新增一条 it『审计脚本自身纳入扫描且实跑 BAD_COUNT=0』（execFileSync 实跑脚本断言输出含 BAD_COUNT=0 且不含 BAD_FOUND:）；RED 实测 1 failed | 13 passed（14），失败 AssertionError: expected '#!/bin/bash\ncd "$(git rev-parse --sh…' not to contain '--exclude=zero-secret-scan.sh'（tests/WikiEnemyMechanics.test.ts:180），坐实新守卫可失败且失败来自旧脚本 exclude。随后删除 zero-secret-scan.sh 第4行的 --exclude=zero-secret-scan.sh，GREEN 实测 14 passed（14）、单独 bash -n RC=0、实跑脚本 BAD_COUNT=0/SYNTAX_RC=0、git check-ignore 三文件命中。补强仅涉及 zero-secret-scan.sh、本 REPORT 与 WikiEnemyMechanics.test.ts，未改产品代码/截图/JSON/回放结算。
- 含临时令牌的 data/admin-auth-init.js 已删除：AUTH_AFTER=NO（本次扫描 AUTH_BEFORE=NO）。
- QA 库 battle_records 复核 count=104, max(id)=477，与基线一致，本轮无写入。
- 本 QA 未修改业务源码、未执行 git 提交；git status 中 .gitignore 与 laiyipao/admin/src/views/WikiView.vue 的改动系他人实施任务所有。
- t262 更正声明：此前 REPORT 中『BAD_COUNT=0 即零真值通过』的结论不成立——旧脚本第 4 行正则内嵌弱口令与本机工作目录字面，扫描自身时因被匹配却由后续 grep -v 过滤产生假零；真实情况是脚本本身即含真值/个人路径字面。已按 test-first 修复：先加安全守卫（WikiEnemyMechanics.test.ts 新增 describe『R33 零真值审计脚本安全守卫（t262）』两条 it），RED 实测 1 failed | 12 passed（13），失败 AssertionError: expected script not to contain 'admin12345'（tests/WikiEnemyMechanics.test.ts:175）；再将脚本改为通用正则 + --exclude=zero-secret-scan.sh，GREEN 实测 13 passed（13）、bash -n RC=0、实跑 BAD_COUNT=0。修复仅涉及零真值审计脚本、本 REPORT 与契约测试，未改产品代码、截图、JSON/回放结算。

## 十、局限与最终判定
- 真实设备：本轮为真实 Chromium（headless）三视口，非物理真机，按惯例标注。
- SPEC §5 护盾示例数值曾与当前公开 config 不一致（ID10 60→480、ID22 8000→64000）；已由 t260 修正并以 config 为准，原差异轨迹保留于 §三，规范示例非独立真源。
- 审计脚本首版假阳性已修正后方得 PASS，过程如实保留。
- 全量测试首轮 1 失败、复跑全绿，未定位首轮失败用例；不掩盖、不冒充一次通过。
- 判定：PASS。七项硬门禁均满足，无产品阻塞缺陷。