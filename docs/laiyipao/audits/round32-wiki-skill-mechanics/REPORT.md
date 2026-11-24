# R32 Wiki 技能机制三视口真浏览器复查报告

## 结论摘要
- 总判定：FAILED（产品缺陷）。三视口 24 条技能逐字段与公开 `/api/v1/config` 权威快照零不符，页面无横向溢出且表格可横向滚到末列；但 AI 结论 CTA“查看合成技能 →”经真实 CLI 点击后跳到 `/admin/dashboard`，未到达 `#composite`，属真实用户路径缺陷。
- 本轮未修改产品源码、未提交 git；QA 只读复查。发现缺陷如实上报，修复需另派实施任务并先补测试。

## 环境与权威快照
- 采样时间：2026-11-25T03:08:36+08:00；`/api/v1/config` 原文落 `data/config.json`；SHA-256：`55791158d8f7f5a32e2dccf62be0b48ee0fc2e30f40e862464fe8c92c0e959fb`。
- 权威统计：version=`6237e0b623219b8b`，技能 24 个，主动 22 个，被动 2 个，多目标 20 个。关键 ID：3=温压弹/主动/2000ms/穿透1/范围180/连锁1；7=电弧弹/主动/900ms/穿透—/范围—/连锁2；19=侦察无人机/被动/四项—。
- 服务：本地 API、Admin（5175）、H5（5174）与本地 QA PostgreSQL 容器均就绪；三视口均由真实 Chromium CLI 驱动。

## 验收逐项
| 项 | 结果 | 依据 |
| --- | --- | --- |
| ① 权威逐行对账 | PASS | `reconcile.py` 对 390/768/1280 三视口各 24 行 × 7 字段及文案逐项比对，`MISMATCHES=0`，`ALL_RECONCILE_OK=True` |
| ② 三视口几何/末列滚动 | PASS | `documentElement.scrollWidth=innerWidth`；表格 wrapper `scrollWidth=1153`，`clientWidth` 324/702/884；实测 `scrollLeft` 到 829，末列表头“说明”与末行可见；无 NaN/undefined/[object Object]/{total}/原始毫秒直出 |
| ③ AI 四判据与 CTA | FAIL | AI 文本与四判据正确，但真实 CLI 点击 CTA 后跳数据看板，`atComposite=false`，未到合成技能锚点 |
| ④ 目录/搜索/深链/R31 回归 | UNPROVEN | 本轮深链/搜索 CLI 回归多次 RC=126，未取得有效新证据；R31+R32 定向单测通过，R31 手工行为未由本轮浏览器重新坐实 |
| ⑤ 报告+原始探针+截图 | PASS（证据完整） | 4 张新图 sha 各异且亲读非登录页，探针原始数据落 `data/`（已被 gitignore） |
| ⑥ 审计脚本零真值 | PASS（已修） | 初稿 `mint-admin-token.sh` 含本地口令字面量；已改为从 `QA_ADMIN_USER`/`QA_ADMIN_PASS` 环境变量读取，复扫 `BAD_COUNT=0`，`bash -n` 通过；含令牌的 `data/admin-auth-init.js` 已删 |
| ⑦ 不改产品源码/不 git | PASS | 仅写审计目录与 `.gitignore` 忽略规则（此前已存在），未改业务源码、未提交 |
| ⑧ 定向/全量/类型/构建 | PASS | R32 定向 11/11；R31+R32 定向 2 files/29 tests；Admin 全量 71 files/795 tests；`type-check` EXIT=0；`build` EXIT=0 |

## CTA 缺陷证据
- 点击前 `data/cta2-pre.json`：`hashBefore=#/wiki?tab=skills`，CTA 文本=“查看合成技能 →”，`href=#composite`，CTA 矩形 `{l:46,t:1628,w:88,h:16}`。
- 使用 `agent-browser click '.r32-ai-cta a'` 真实 CLI 点击，命令返回 Done；点击后 `data/cta2-post.json`：`hashAfter=#/admin/dashboard`，`atComposite=false`，`compositeRect=null`，`skillStillThere=false`。
- 截图 `r32-04-cta-composite.png` 经亲读确认为数据看板首页，非技能页合成技能锚点。
- 根因：`laiyipao/admin/src/router/index.ts:3` 使用 `createWebHashHistory`；`:35` catch-all `/:pathMatch(.*)*` 跳 `/admin/dashboard`。普通 `href="#composite"` 把 hash 改为 `#composite`，被路由视为未知路径后重定向到看板。
- 修复建议（不在本 QA 范围）：test-first 增加 CTA 点击后仍留在技能页并滚动到 `#composite` 的测试；实现可选 `@click.prevent` + `nextTick` 后 `scrollIntoView`，或 `router.push({ path: currentRoute.path, hash: '#composite' })`；避免裸 hash href 触发路由跳转。

## 三视口探针与截图
- 三视口 390×844、768×1024、1280×800 的 `probe-*.json` 均 `rowCount=24`，列头含 ID/名称/元素/系别/伤害/热量/冷却/类型/穿透/范围/连锁/层数/说明。
- 末列滚动：`data/scroll-lastcol.json`：`before=829`、`after=829`、`scrollWidth=1153`、`clientWidth=324`、`lastHeader=说明`、`scrolledToEnd=true`。
| 文件 | 视口 | SHA-256 |
| --- | --- | --- |
| `screenshots/r32-390-skills.png` | 390×844 | `9893619f30818be630ad41346c220c4ae07085cbe148a8d8506e87f9e2ddbce9` |
| `screenshots/r32-768-skills.png` | 768×1024 | `3e09fd450c60d5e0c140004dec0c16eee60a89977c56f68234663d67cd2bd714` |
| `screenshots/r32-1280-skills.png` | 1280×800 | `a9497231f55da34b6f94938e13f064f946a1c697c9026c41e52508ce4a1dd524` |
| `screenshots/r32-04-cta-composite.png` | 390×844 | `a1c326589f7d3d599f6b70e9316b34499688b05617b75bcd4ad83ec9c05c3144` |
- 四图 sha 两两不同，均已亲读；前三张为技能表，第四张坐实 CTA 跳看板缺陷。曾尝试生成末列滚动专属图，CLI 返回 RC=126 未生成，故以 `data/scroll-lastcol.json` 作为滚动证据，不伪造截图。

## 回归与限制
- 已坐实：R31+R32 定向测试 29/29，Admin 全量 795/795，类型检查与构建通过；三视口表格与权威字段零不符。
- 未坐实：本轮深链 `#/wiki?tab=enemies` 与搜索框交互 CLI 多次 RC=126/卡住，未取得有效浏览器证据，不能声称通过；需后续任务复测。
- `TestNoRealSecretsInRepo` 未在本轮单独执行；本轮自行扫描审计脚本 `BAD_COUNT=0`，但不能替代该测试对全仓的守卫结果。

## 清理
- 含临时本地令牌的 `data/admin-auth-init.js` 已删除：`AUTH_BEFORE=YES`，`AUTH_AFTER=NO`。
- `.gitignore` 命中 `docs/laiyipao/audits/round32-wiki-skill-mechanics/data/`；QA 库 `battle_records=104`、`max(id)=477`，与基线一致，本轮无写入。
- `mint-admin-token.sh` 已去除本地口令字面量，改为必填环境变量输入；审计脚本复扫个人绝对路径/令牌模式 `BAD_COUNT=0`，`bash -n` 通过。

## 最终判定
- 复跑确认：审计脚本脱敏后 `pnpm exec vitest run tests/WikiSkillMechanics.test.ts tests/WikiReactionMechanics.test.ts` 为 2 files/29 tests passed（03:26:16）。
- 不通过。仅三视口字段/几何/测试维度通过；CTA 真实用户路径失败且根因明确。请创建修复任务，按 test-first 修复后重新执行本轮三视口与 CTA 门禁。
