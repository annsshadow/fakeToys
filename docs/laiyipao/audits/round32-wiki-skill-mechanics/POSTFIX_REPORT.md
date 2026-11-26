# R32 修复后复查报告：Wiki 技能机制与 hash-router 安全 CTA 闭环

- 复查任务：t253（work，设计终审独立复查）
- 关联规范：docs/laiyipao/specs/round32-wiki-skill-mechanics/SPEC.md
- 原始缺陷报告：docs/laiyipao/audits/round32-wiki-skill-mechanics/REPORT.md（判定 FAILED：CTA 跳转缺陷）
- 修复后复查证据根目录：docs/laiyipao/audits/round32-postfix-recheck/
- 基线 HEAD：4f03d667c2135bc4a6ab24c33de8a771b3ab1688
- 复查判定：**PASS**（CTA 缺陷已闭环；下列诚实边界项如实标注，不冒充 PASS）

## 一、复查范围与方式

R32 原始 QA 报告判定 FAILED，唯一产品缺陷是 AI 卡「查看合成技能 →」CTA 在 hash-router 下把 bare `href="#composite"` 当作整段 hash 替换，被 catch-all 重定向到 `/admin/dashboard`。t248-t252 完成修复（SPEC A4 同步、契约 RED→GREEN、WikiView CTA 改为 `:href="urlFor('skills','composite')"` + `@click.prevent="goTo(...)"`）。本次复查独立验证修复是否真实闭环、是否引入回归、审计是否卫生。

复查方式：本会话服务端口（5175/8080/5174）探测均为未监听状态，**无法在本会话重新驱动真浏览器**。因此采信 QA 已落盘的原始探针/截图/脚本证据，并逐项独立复跑数据一致性脚本、测试、构建与守卫，对原始数据做交叉对账。凡本会话未能亲自重驱的操作，均在第四节如实标注。

## 二、八项门禁逐项核验

### 门禁1：三视口 × 24 技能全字段对账（PASS）
- 独立跑 `reconcile.py 390 768 1280`→ `ROWS=24 MISMATCHES=0 AI_OK=True BAD=False`，`ALL_RECONCILE_OK=True`。
- 权威源：公开 `/api/v1/config`（`config.meta.txt`：SAMPLED_AT=2026-11-25T03:29:47+08:00，CFG_SHA=62b41b31e635e2dc51bc4dbd1fce7059e16dcacc3a7e1638f7b038521541677d，VERSION=6237e0b623219b8b，SKILLS=24）。
- AI 聚合：共 24 个技能 / 主动 22 / 被动 2 / 多目标 20，与配置计算一致。
- 关键 ID：ID3 温压弹=主动/2秒/穿透1/范围180/连锁1；ID7 电弧弹=主动/0.9秒/—/—/连锁2；ID19 侦察无人机=被动/—/—/—/—，逐字段与配置一致。
- 三视口 `DOC=390/390|768/768|1280/1280`（scrollWidth<=innerWidth+1），无横溢。

### 门禁2：CTA 真实 CLI 点击（PASS）
- `data/cta-pre.json`：hashBefore=#/wiki?tab=skills，ctaText=查看合成技能 →，ctaHref=#/wiki?tab=skills&anchor=composite。
- `cta-click.sh` 用 agent-browser 真实 CLI `click '.r32-ai-cta a'`（`data/cta-click.txt`=✓ Done），非 DOM e.click。
- `data/cta-post.json`：hashAfter=#/wiki?tab=skills&anchor=composite，skillStillThere=true，atComposite=true，compositeText=合成技能，scrollY=1700，dashboard=false。
- 截图 `r32p-05-cta-after.png` 亲读：顶部标题「合成技能」+ 合成技能表（ID31-45），确认到达 `#composite`，**不再是旧 r32-04 数据看板**。CTA 缺陷确认闭环。
- WikiView.vue 实现核验：:959 `buildSkillMechanicsAi(cfg?.skills ?? [])`，:961 CTA 已改为 `:href="urlFor('skills','composite')"` + `@click.prevent="goTo(...)"`，非裸 href。

### 门禁3：表内滚动与占位符（PASS）
- `data/scroll-lastcol.json`：before=0 after=829 sw=1153 cw=324 lastHeader=说明 scrolledToEnd=true，可滚到末列。
- 三视口 wrapSw=1153 / wrapCw=324|702|884，表内滚动容器正确。
- 探针 `hasBad=false`（无 NaN/undefined/[object Object]/{total} 占位符）。

### 门禁4：目录/搜索/深链真实浏览器操作（部分未坐实，见第四节）
- 搜索「温压弹」：成功会话 `reg2-clean.log` 显示 CLICK_RC=0/TYPE_RC=0/ENTER_RC=0，真实键盘输入+Enter，SEARCH_POST hash=#/wiki?tab=skills&anchor=skill-table&focus=skill:3，scrollY=170，focused=[skill:3]，inputValue=温压弹，REG2_DONE=YES RC=0（PASS）。
- 深链 tab=enemies：`deep6.log` hash=#/wiki?tab=enemies，navActive=[敌人抗性图鉴]，enemySectionVisible=true，skillSectionVisible=false，enemyRows 含蒸汽爆发/过热/燃烧云三行，docOverflow=0（PASS）。
- 目录/tab 点击选择器未命中（nav-click.log CLICK_RC=1、deep3-click.txt 未找到元素）——不作 PASS，见第四节。

### 门禁5：R31 与既有 Wiki 功能不回退（PASS）
- `data/reg-r31-reaction.json`（390 视口）：反应表 7 行，headers 含层数传播/削甲/击退，腐蚀扩散=传播1层，破甲击退=150‰/4px，燃烧云行仍存在，oldWording=false，badToken=false，page.sw=cw=390 无横溢。
- 定向契约把 R32 与 WikiSkillMechanics / WikiReactionMechanics / WikiReactionDescr / WikiView 一并跑通，R31 契约未回退。

### 门禁6：截图证据（PASS，含一处重复如实标注）
- 11 图 SHA256 亲算。≥4 张两两不同满足：r32p-01-390=9893619f…、r32p-02-768=3e09fd45…、r32p-03-1280=a9497231…、r32p-05-cta-after=1d7849927bd7cd608beaa175904df2f1332f437269563bedd91db5f2e1929836、r32p-07-search=b392a54d8…、r32p-08-deeplink=a07584a5…、r32p-11-deeplink-final=c8ac69a1…。
- 亲读 4 图（r32p-01/02/03/05）全部 390×844/768×1024/1280×800 非登录页，内容为玩家玩法文档站 + 技能表。
- **诚实标注**：r32p-04-cta-before 与 r32p-01-390 SHA 完全相同（9893619f…），二者同图；"CTA 图 ≠ 旧 r32-04 数据看板图（a1c32658…）"成立，但 04 与 01 复用同一张图，不作为独立第 4 图计入。

### 门禁7：审计卫生（PASS）
- 三份复查脚本均 `cd "$(git rev-parse --show-toplevel)"` 发现根目录；mint/prepare 凭据只从 QA_ADMIN_USER / QA_ADMIN_PASS 环境变量读取，无字面口令。
- `zero-secret-scan.sh`→BAD_COUNT=0，SYNTAX_RC=0（三脚本 bash -n 通过）。
- `git check-ignore`：.gitignore:382 忽略 data/、:383 忽略 admin-auth-init.js。
- **残留 token 文件已清理**：复查结束时 data/admin-auth-init.js（含 lyp_admin_token 写入与 JWT 前缀占位串）仍存在，本次复查已用 OPS remove 删除并 stat 确认 exists=false。
- .gitignore 新增行 374-383 覆盖 round32 三个审计目录的 data/ 与 admin-auth-init.js。
- 令牌三锚点 tokens.css:12-14 保留（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6），无 --r32-。
- QA 库 `docker exec lyp-qa-pg psql -U postgres -d laiyipao`→104|477（battle_records=104，max id=477），与基线一致。

### 门禁8：测试/构建/守卫如实执行（PASS，含本机 SKIP 标注）
见第三节。

## 三、独立复跑命令与结果（本会话亲自执行，除注明外均 exit 0）

1. `python reconcile.py 390 768 1280` → VERDICT=PASS，ALL_RECONCILE_OK=True，MISMATCHES=0。
2. 定向 `pnpm exec vitest run`（WikiSkillMechanics + WikiReactionMechanics + WikiReactionDescr + WikiView）→ 4 files / 59 tests passed（7.43s）。
3. admin 全量 `pnpm test -- --testTimeout=90000` → 71 files passed / 795 tests passed（39.41s）。
4. admin `pnpm type-check`（vue-tsc）→ exit 0。
5. admin `pnpm build` → ✓ built in 1.30s，exit 0。
6. Go `TestNoInfraIdentifiersInRepo` → PASS（4.38s）。
7. Go `TestNoRealSecretsInRepo` → **SKIP**：本机读不到 laiyipao/deploy/.local.keys，守卫原文「读不到 …… 真值扫描这条守卫没有执行，不是『通过了』」。**本会话未执行真值扫描，不写 PASS。**
8. `git diff --check` → GIT_DIFF_CHECK_OK。
9. `sha256sum` 11 图、`git check-ignore`、QA 库 psql 复核均完成。

## 四、诚实边界与遗留（不冒充 PASS）

1. **本会话未重驱真浏览器**：服务端口未监听，采信 QA 落盘证据并独立对账；截图为 QA agent-browser 生成，非物理真机、本地回环 127.0.0.1。
2. **agent-browser 部分命令 RC=126 不稳定**：search-clean.log 会话 CLICK/TYPE/ENTER 全 126、focused=[]；reg2-clean.log 会话全 0 且命中。以成功会话为有效证据，失败会话不作 PASS。
3. **目录/tab 点击未坐实**：nav-click.log 与 deep3-click.txt 的选择器（.nav-item:has-text("敌人抗性图鉴") / .el-tabs__item:has-text("敌人图鉴")）均 Element not found，目录点击不作 PASS；深链直达由 deep6 坐实。
4. **深链窄屏差异**：deep3/deep4/deep5 探针出现 enemyTable=false / skillTable=true / bodyHasEnemy=false，与桌面 deep6 的 section 可见性不一致，窄屏下 section 可见性存在不一致，如实记录。
5. **r32p-04 与 r32p-01 同图**：SHA 重复，不作独立第 4 图。
6. **TestNoRealSecretsInRepo 本机 SKIP**：真值扫描守卫未执行，非 PASS。
7. **未改业务代码、未 git**：本次仅新增本报告 + 清理审计临时 token 文件。

## 五、结论

R32 修复后复查核心目标——**AI CTA hash-router 跳转缺陷闭环**——由 cta-pre→cta-post 探针、CLI 真点击日志、r32p-05 截图三方坐实；三视口 24 技能全字段零不符、AI 聚合正确、表内可滚无横溢、R31 不回退、审计零真值、测试/构建/守卫（除本机 SKIP 项）全绿。所有未能亲自重驱或不稳定的操作均已按诚实边界标注，未冒充 PASS。判定 **PASS**。
