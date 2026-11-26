# R31 终审：Wiki 反应机制字段与审计零真值闭环

- 任务：t236 系列的 t244「R31 终审：六门禁独立复核」，reviewedTaskId=t240
- 终审人：design-reviewer-wb；Attempt1 attempt_id=0ba3420e-c34f-4bad-8809-d252f1d274bc
- 基线 HEAD：9f874bdf9eeee0548c06b741d2aa7a4845260844（本轮实测 git rev-parse HEAD）
- 审查对象：docs/laiyipao/specs/round31-wiki-reaction-mechanics/SPEC.md；laiyipao/admin/src/reactionMechanicsText.ts；laiyipao/admin/src/views/WikiView.vue；laiyipao/admin/tests/WikiReactionMechanics.test.ts；docs/laiyipao/audits/round31-wiki-reaction-mechanics/REPORT.md
- 判定：**PASS**。六维门禁全部满足，未发现需新建修复任务的真实产品缺陷。本终审只读业务代码，未改源码、未 git。

## 一、规范与契约（真实导入、单位口径、可失败）
- SPEC 62 行逐字钉死三字段 json tag（spread_stacks / armor_shred_permille / knockback）、真实值（corrosion_spread=1 层、armor_break=150‰/4000 定点）、单位换算、零值诚实空态与 burn_cloud C1 产品阻塞。
- 契约 WikiReactionMechanics.test.ts 148 行**真实 import** `../src/reactionMechanicsText`，无内嵌参考实现；reactionTable() 从 `cfg?.reactions` 切到 `</el-table>`，对新增三列与三个 helper 调用做源码守卫；纯函数对 0/null/undefined/-1/NaN/1.5/±Infinity 逐一断言空态，对 4000→4px、2500→2.5px、150→150‰（且不含 %）断言换算。测试能随业务改动失败，非摆设。
- 独立核验：`spreadStacksText` 正整数走 `传播 N 层`、小数被 `Number.isInteger` 拒；`armorShredText` 原值加 ‰；`knockbackText` `Math.round(n/1000*1000)/1000`——实现与 SPEC、契约三方一致。

## 二、三字段权威真源与 /config 逐字段一致
- 服务端真源：`laiyipao/server/internal/domain/elements.go:125/135/138` 三个 json tag；:235 corrosion_spread AoeRadius120/SpreadStacks1；:241-245 armor_break ArmorShredPermille150/Knockback4000。公开配置装配 `service.go:1601 LoadGameConfig` + :1638 `Reactions: domain.AllReactionSpecs()`（我 OPS grep 独立定位确认）。
- Wiki 消费真实公开端点：`WikiView.vue:620 api.get<ConfigResp>('/config', { auth: false })`、:674 页面自述「本页数据直接来自线上 /api/v1/config，与游戏内实际数值同源」；生成类型 types.gen.ts:602/605/606 已含三字段；轻量端点白名单 reactions_endpoint_test.go:127-128 含三字段。`/admin/reactions` 仅同源旁证，非 Wiki 真源——SPEC 与契约均无回退表述。
- 我独立解析审计快照 config.json（version=6237e0b623219b8b，7 条反应）：corrosion_spread 的 spread_stacks=1、armor_break 的 armor_shred_permille=150/knockback=4000，其余五条三字段全 0。auth 三视口探针 DOM（7 行 × 3 字段：传播 1 层 / 150‰ / 4px，其余 —）与该权威逐字段零不符。

## 三、零值诚实空态与 burn_cloud 边界文案
- 权威为 0 的格子在 DOM 一律显示「—」；探针 `badToken=false`、`has15percent=false`、`has150permille=true`，未把 150‰ 误写成 15%，未编造数值。
- `WikiView.vue:843-845` 已删旧文案「仍**没有特殊效果**的只剩两条」，改为「腐蚀扩散的**层数传播已实现**（传播 1 层、半径 120px、不造成伤害、不触发新反应）」并明确「**燃烧云 burn_cloud 的火区仍未实现**」。探针 `hasImplement=true`、`hasBurnCloud=true`、`oldWording=false`——已实现与未实现边界如实共存，无夸大。

## 四、三视口表格布局与深链/搜索不回退
- 三视口页面级无横向溢出：1440/375/390 的 documentElement.scrollWidth=clientWidth=innerWidth；表格 wrapper（.el-scrollbar__wrap）sw=1090、cw=884/309/324、overflowX=auto，实测 scrollLeft 0→206/781/766（均达 max），末列「说明」可滚达且不出界。
- 既有 8 列（反应/系数/攻击力权重/控制效果/受击增伤/驱散护盾/溅射半径/说明）、`statusText`/`amplifyPctText`/`emptyText`、`SECTIONS`/`targetFromQuery`/`scrollToAnchor`/`scrollToFocus`/`searchIndex`/`searchMatches`/`wiki-body`/`:data-focus` 全部保留（契约源码守卫通过）。
- 深链 `?tab=enemies&anchor=reactions` 落到敌人图鉴（第 4 张截图坐实）；搜索「腐蚀扩散」写回 `?anchor=reactions&focus=reaction:corrosion_spread`、scrollY=708（第 5 张截图坐实末列与说明文本）。诚实边界：焦点行高亮短窗口在截图时已过，探针 `focusRows=0`；该高亮由 `tests/WikiView.test.ts:364` 单测覆盖（随全量 781/781 通过），报告如实记录、未冒充截图直接证据。

## 五、真浏览器有效证据
- 5 张本轮新图 sha256 我亲算两两不同并与 REPORT 一致，逐张 `read_image` 亲读确认均为 Wiki 页面、非登录页：
  - r31a-desktop-wiki.png `9237e65d…f224bf77b5`（1440×900，桌面全表含层数传播/削甲/击退列）
  - r31a-mobile375-wiki.png `de75ce6a…9a51853`（375×390，窄屏首屏与搜索框）
  - r31a-mobile390-wiki.png `e5f10e4d…8407`（390×844，移动目录横条与表首屏）
  - r31a-04-deeplink.png `209f8af5…6a9a1`（390×844，敌人图鉴）
  - r31a-05-search-focus.png `ec8a0d6f…c58208`（390×844，搜索聚焦末列含 150‰/4px 行）
- 非 auth 三视口探针被路由守卫跳 `#/admin/login?redirect=/wiki`、tableFound=false——早期误截图已删除、不作证据；auth 三视口探针 tableFound=true、headers 11 列、rowCount=7、rows 与权威一致。报告对该鉴权流程如实标注。
- 局限：headless Chromium（agent-browser）非物理真机；URL 均为本地回环 127.0.0.1。不冒充线上真机证据。

## 六、零真值审计卫生与 R1-R30 不回退
- 三份审计脚本 `e2e-viewports.sh`/`e2e-viewports-auth.sh`/`mint-admin-token.sh` 均首行 `cd "$(git rev-parse --show-toplevel)" || exit 1`；我 OPS grep 以 `/Users/`/`/home/`/`[A-Za-z]:\\` 扫 `*.sh`/`*.ts` 得 0 命中，三脚本 `bash -n` 全过。含本地令牌的 admin-auth-init.js 已删除，.gitignore:366-367 拦截 data/ 与注入脚本，git check-ignore 两路径均命中。
- t243 更正闭环可复核：契约新增两条确定性守卫（禁本机绝对路径、必须含 git rev-parse），REDD 1file/2tests 失败→GREEN 1file/18tests 通过；报告第十节如实改正原第八节「零真值 0 命中」表述，未在报告回显真实路径或令牌。
- 令牌三锚点 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 保留（tokens.css:12-14），无 `--r31-`；QA 库 battle_records count=104/max_id=477 与基线一致，无战斗写入。
- 工作树仅 `.gitignore` M + `WikiView.vue` M（17 insertions/4 deletions，R31 上游实施）+ 反应机制 helper/契约/审计新文件；未见引擎、replay_hash、score、settle、loot、后端 logic、tokens 的 R31 改动。

## 七、独立复跑（本人 run_code 执行）
| 命令 | 结果 |
| --- | --- |
| `pnpm vitest run tests/WikiReactionMechanics.test.ts tests/WikiReactionDescr.test.ts tests/WikiView.test.ts --reporter=dot`（workdir=admin） | exit 0，3 files / 48 tests passed |
| `pnpm vitest run --reporter=dot`（workdir=admin） | exit 0，69 files / 781 tests passed，46.02s（jsdom router 警告为既有噪声） |
| `pnpm type-check`（vue-tsc --noEmit） | exit 0 |
| `pnpm build`（vue-tsc + vite build） | exit 0，built in 1.73s |
| `go test ./internal/domain -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo' -count=1 -v`（workdir=server） | exit 0；**TestNoInfraIdentifiersInRepo PASS(5.78s)**；**TestNoRealSecretsInRepo SKIP** |
| `for f in …*.sh; do bash -n "$f"; done` | exit 0，BASH_SYNTAX_OK |
| `git diff --check` | exit 0，DIFF_CHECK_EXIT=0 |

**诚实标注（防误读）**：Go 真值守卫 TestNoRealSecretsInRepo 本机因读不到 `laiyipao/deploy/.local.keys` 被 SKIP，输出原文「读不到 … —— 真值扫描这条守卫**没有执行**，不是「通过了」」。该条**未执行、不计 PASS**；同组 TestNoInfraIdentifiersInRepo 真实 PASS。终审不把整体 exit 0 冒充为真值扫描通过。三脚本零真值结论由 OPS 边界 grep 0 命中 + 契约 RED→GREEN 守卫共同支撑，而非这条被跳过的 Go 守卫。

## 八、最终判定
六维门禁全 PASS：SPEC/契约真实可失败、/config 权威逐字段一致、零值空态与 burn_cloud 边界诚实、三视口布局与深链/搜索不回退、5 图亲读非登录页且 auth 探针坐实、零真值清理与 t243 更正闭环完整、测试/类型/构建/守卫全绿（SKIP 已如实标注）。无 needs_revision / reject 项。