# R37 独立终审报告：Wiki 专精树节点字段与诚实效果口径

- 轮次：R37
- 任务：t9 独立终审（design-reviewer-wb），覆盖 t6 规范、t7 实施、t8 真浏览器复查
- 日期：2026-11-25（Asia/Shanghai）
- 判定：**verdict = pass**（六维门禁全绿；失败/跳过/偏差项如实登记见 §5，未以 SKIP 冒充 PASS）
- 审阅对象：ReviewedTaskId=t7；依赖 t8 报告 docs/laiyipao/audits/round37-wiki-mastery-nodes/REPORT.md
- 独立性声明：不采信成员自报，全部源码/文档亲读、全部门禁亲自复跑，只读终审；唯一落盘为本报告，未改业务源码、未执行任何 git 写操作。

## 一、审阅范围与方法

本轮目标是把 Wiki 专精树节点从「只显示 n.name」的漂移状态，对齐权威 MasteryNode 九字段（id/family/layer/slot/name/kind/value/prereq_family/prereq_layer），并以诚实口径展示效果（配置基值、非结算结果）。终审按六维逐项裁决：

1. 规范与契约（t6）：SPEC 口径正确、契约真实导入实施模块无内嵌实现；
2. 实施源码（t7）：helper 口径逻辑正确、WikiView.vue 最小改动；
3. 真浏览器证据（t8）：三视口逐字段对账、CTA 安全、横滚/末列可见、七 tab 不回退；
4. 独立门禁复跑：契约/全量/类型/构建/Go 守卫/diff 全部亲自执行；
5. 诚实边界：SKIP 非 PASS、前置列口径、探针缺陷如实记录；
6. 保护面：engine/后端/replay/hash/score/settle/loot/tokens/令牌零改动。

## 二、亲读证据摘要

### ① SPEC 与契约（维度 1）
- docs/laiyipao/specs/round37-wiki-mastery-nodes/SPEC.md（51 行）逐字钉死：权威节点结构来自 power.go:11-25 与 mastery_json_shape_test.go:67-71；效果语义来自 power.go:175-206 的 EvaluateMastery（element_cap→v、reaction_mult→v 千分比、heat_cap→v*10、extra_slot→+1、skill_damage/crit/armor/mechanic 千分比）；下发路径 service.go:1523/:1637 与公开 /api/v1/config；零改动面；九字段本地类型；七列；AI 四判据；三视口 QA 门禁；RED 门禁。
- laiyipao/admin/tests/WikiMasteryMechanics.test.ts（141 行）第 6 行真实 import { ... } from "../src/masteryMechanicsText，无内嵌参考实现；11 个 it 覆盖 MECHANISM_EMPTY 同源、八种 kind、heat_cap 不二次换算、extra_slot +1 槽、千分比口径、prereqText 边界、AI 确定性聚合与逐字常量、权威源码有据、本地类型九字段、七列与真实调用、AI 四要素与 hash 安全 CTA、令牌三锚点。

### ② helper 口径（维度 2）
- laiyipao/admin/src/masteryMechanicsText.ts（113 行）亲读确认：
  - re-export MECHANISM_EMPTY from @/enemyMechanicsText 并同源 import，无第二份字面量；
  - kindText：八种权威映射，未知合法字符串原样、非字符串/空串 → —；
  - valueText：isPositiveInteger 守卫 0/负/小数/NaN/Infinity；heat_cap 直接 `+${value}‰` 取服务端 v*10 配置值（不二次乘 10、无百分比）；extra_slot 恒 `+1 槽`；element_cap `+${value}层`；其余权威 kind `+${value}‰`；未知合法 kind 原值 +「单位以游戏内结算为准」；
  - prereqText：空族 + 层 0 → 无前置；非空族 + 正整数层 → 「前置：{族} 第{N}层」；其余脏组合 → —；
  - buildMasteryMechanicsAi 只读数组确定性聚合，统计合法 family/有效 name 节点/非「无前置」节点/extra_slot 节点，两次同输入严格相等，空输入诚实零态；
  - 三常量 MASTERY_AI_BASIS / HONESTY / CTA 与 SPEC 逐字一致。

### ③ WikiView.vue 差异（维度 2）
- `git diff -- laiyipao/admin/src/views/WikiView.vue` 亲读：ConfigResp.mastery_families 类型从 {name,layer} 补齐为九字段；专精 section 加 .r37-mastery-mechanics、节点 el-table 七列（系/层/槽/节点/效果/数值/前置）外包 .r37-mastery-table-scroll（overflow-x:auto;max-width:100%）；.r37-ai-note 四要素同屏；CTA 走 urlFor('mastery') + goTo({ tab:'mastery' })，无裸 href="#mastery"；CSS 新增 R37 样式块。改动范围最小，未触碰其它 section 与 rowFocusClass。

### ④ QA 报告与截图（维度 3）
- docs/laiyipao/audits/round37-wiki-mastery-nodes/REPORT.md（73 行）8 项门禁逐项有实测证据；reconcile-summary.txt 显示 390/768/1280 三视口 ROWS=96 MISMATCHES=0、AI_OK/FOUR_OK=True、BAD=False、DOC=innerWidth、SCROLL=True、ALL_RECONCILE_OK=True，CFG_SHA=ff87a8db…、VERSION=6237e0b623219b8b。
- 亲读 5 张关键截图（均为真实专精树页面、非登录页）：
  - r37-390-mastery.png（390×844）：可见「专精树」选中、焰系表格前四列（系/层/槽/节点）；
  - r37-768-mastery.png（768×1024）：七列齐全，元素上限 +10层、反应倍率 +10‰、热量上限 +10‰、额外插槽 +1 槽、前置列「无前置」/「—」逐行可见；
  - r37-1280-mastery.png（1280×800）：左目录树高亮专精树，七列含末列「前置」直接可见；
  - r37-05-cta-after.png（390×844）：CTA 点击后仍停留在专精树段，未跳 dashboard 或登录页；
  - r37-reg3-mastery.png（1258×588）：CLI 回归点击后专精树 tab 有效。
- 独立 sha256sum：config.json 与 REPORT 记录一致（ff87a8db…）；8 张引用截图哈希全部与 REPORT §3 逐行一致；互异 sha ≥6 个，满足 ≥4 张互异新图。注 r37-06-lastcol-1280 与 r37-1280-mastery 同字节、r37-04-cta-before 与 r37-390-mastery 同字节，REPORT 已声明不冒充互异新图。

## 三、独立复跑门禁表（维度 4，全部亲自执行）

| # | 命令（摘要） | 结果 | exit |
|---|--------------|------|------|
| 1 | pnpm exec vitest run tests/WikiMasteryMechanics.test.ts --reporter=dot --testTimeout=90000 | 1 file / **11 passed (11)** | 0 |
| 2 | R31–R36 相关契约 6 文件（Composite/Gem/Level/Enemy/Reaction/Skill） | 6 files / **69 passed (69)** | 0 |
| 3 | pnpm exec vitest run --testTimeout=90000 --reporter=dot（全量） | **76 files / 846 tests passed**，Duration 55.43s | 0 |
| 4 | pnpm run type-check（vue-tsc --noEmit） | 无类型错误 | 0 |
| 5 | pnpm run build | built in 11.94s，WikiView chunk 43.93 kB | 0 |
| 6 | go test -run TestNoInfraIdentifiersInRepo -v ./internal/domain -count=1 | **PASS (74.20s)** | 0 |
| 7 | go test -run TestNoRealSecretsInRepo -v ./internal/domain -count=1 | **--- SKIP**（secrets_leak_test.go:182 读不到 .local.keys）| 0（非 PASS）|
| 8 | git diff --check | DIFF_CHECK_RC=0 | 0 |
| 9 | grep -rn -- '--r37-' laiyipao/admin/src | 无匹配（R37_TOKEN_GREP_RC=1）| 1（无匹配）|
| 10 | python reconcile.py 390 768 1280（亲读摘要并独立 sha 复核） | 三视口 96×7 MISMATCHES=0，ALL_RECONCILE_OK=True，CFG_SHA=ff87a8db… | — |
| 11 | sha256sum data/config.json screenshots/*.png | 与 REPORT §3 逐行一致 | 0 |
| 12 | docker exec lyp-qa-pg psql ... battle_records | **104｜477** | 0 |
| 13 | auth-init 检查 | AUTH_INIT=ABSENT、ADMIN_AUTH_INIT=DELETED | 0 |
| 14 | git diff --name-only + 保护面 grep | 仅 .gitignore、WikiView.vue；**NO_PROTECTED_CHANGES** | 0 |

说明：全量 vitest 的 stderr 含既有 jsdom / Vue Router warn（/admin/users 无匹配、router-link 未解析）等噪声，属历史既有、非本轮回退；76/846 全绿。R36 终审记录的全量为 835 tests，本轮 846 tests 含 R37 新增 11 例，数量自洽。

## 四、六维门禁裁决

| 维度 | 结论 | 依据 |
|------|------|------|
| 1 规范与契约 | PASS | SPEC 权威有据、零改动面明确；契约真实 import 无内嵌实现；RED 门禁由 t6 保留证据；本轮 GREEN 11/11 |
| 2 实施源码 | PASS | helper 五口径正确（不二次乘10 / +1槽 / 千分比 / 同源空态 / 确定性聚合）；WikiView 九字段七列 + 安全 CTA 最小改动 |
| 3 真浏览器证据 | PASS | 三视口 96×7 零不符、CTA CLI 停留 mastery tab、390/768 表内真横滚、1280 末列直接可见、七 tab 不回退、截图非登录页 |
| 4 独立门禁复跑 | PASS | 上表 14 项全部亲跑，测试/类型/构建/Go 守卫/diff 全通过（SKIP 项单独登记） |
| 5 诚实边界 | PASS | SKIP 非 PASS、前置列契约结果、AI 含前置64 口径、探针缺陷与三处 SPEC 字面偏差均如实记录（§5） |
| 6 保护面 | PASS | 仅 .gitignore/WikiView.vue 两处跟踪修改 + R37 预期新增；禁改面零触碰；令牌三锚点无漂移、无 --r37-；NO_PROTECTED_CHANGES |

## 五、诚实边界与偏差登记

1. **TestNoRealSecretsInRepo = SKIP，非 PASS**：secrets_leak_test.go:182 读不到 laiyipao/deploy/.local.keys，真值子串比对未执行；零真值脚本 zero-secret-scan.sh 的 BAD_COUNT=0 只覆盖审计脚本自身，两者不可互相替代。终审不宣称真值扫描通过。
2. **前置列「无前置」32 +「—」64 是严格契约结果，非缺陷**：公开 config 全部节点 prereq_family 空串、prereq_layer=layer-1，依 SPEC prereqText 第 1 层显示「无前置」、第 2/3 层显示「—」。页面**没有**逐字展示「前置：{族} 第{N}层」文案，本报告不谎称展示。AI 结论「含前置 64 个」来自把非「无前置」（含「—」）计入的聚合口径，与页面「—」数量一致。若需完整族名前置需改规范/种子，不属本轮实施缺陷。
3. **reg3 skillsCount 探针缺陷已标注**：reg3-probe.js 的 skillsCount 选择器对技能表返回 0，全 DOM 计数受 v-show 影响；七 tab 回归以 CLI click 后 hash/activeNav/可见 section/反应链传播为准，R32–R35 行数沿用旧基线旁证，不作为当前可见行数独立强证据。早期 text= 选择器脚本 reg2.sh 全部失败已废弃，不纳入证据。
4. **SPEC §3 字面导入形式与实施有偏差（非阻断）**：SPEC 文字要求具名 import { ... } from @/masteryMechanicsText，实施为化解 R32/R36 同名 kindText 冲突改用 import * as masteryMechanics from '@/masteryMechanicsText' 命名空间导入，契约断言全部通过；功能等价、无内嵌实现、三视口零不符，属正当工程取舍。
5. **MECHANISM_EMPTY 来源差异（非阻断）**：WikiView.vue 复用顶部既有 @/enemyMechanicsText 的 MECHANISM_EMPTY（与 helper re-export 同源同值），未从 @/masteryMechanicsText 再导入；无第二份字面量，契约通过，语义一致。
6. **element_cap 展示值与 SPEC 示例措辞差异（非阻断）**：SPEC §5 示例写 1/2/3 → +1层/+2层/+3层，实际公开 config 的 element_cap 值为 10/20，页面据实显示 +10层/+20层；契约 valueText("element_cap",2)="+2层" 通过，helper 逻辑正确，属配置种子值与示例措辞差异，不是口径缺陷。
7. **QA 只复查展示与导航，不验证战斗结算**：表格为配置基值，实际效果以引擎结算为准，权威回放以 replay_hash 为准；数据库仅用于还原基线确认（104/477），不作字段真值来源。
8. **本轮终审为只读**：未改任何业务源码、未执行 git 写操作，唯一写入为本报告；未产生新的真值或环境特化信息。

## 六、保护面结论

- git diff --name-only：仅 .gitignore 与 laiyipao/admin/src/views/WikiView.vue；新增文件均为 R37 预期（SPEC.md、WikiMasteryMechanics.test.ts、masteryMechanicsText.ts、审计目录）。
- engine / 后端 / replay / hash / score / settle / loot / tokens 目录零改动（保护面 grep 无命中）：**NO_PROTECTED_CHANGES**。
- laiyipao/admin/src/styles/tokens.css 三锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6 未漂移；全 admin src 无 --r37-。
- 临时 auth-init 已删除（DELETED），工作树无 laiyipao/deploy/.local.keys（ABSENT）；QA 库回到 104/477 基线；无遗留后台 job。

## 七、最终判定

**verdict = pass**。R37 六维门禁全绿，产品实现正确、诚实边界如实记录、无受保护面改动。可进入分批发布流程。唯一非 PASS 项为 TestNoRealSecretsInRepo 的 SKIP，已按事实标注、未冒充通过。
