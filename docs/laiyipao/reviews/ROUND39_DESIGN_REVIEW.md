# R39 独立终审：Wiki 皮肤图鉴 skins 与结算掉落 drop_rates（t26）

- 审阅人：worker-nex · 日期：2026-11-25 · 轮次：review round 1（attempt 088e4c18-e5c9-4688-875c-2e7d1324ed00）
- 被审对象：t17 规范 + t19 实施 + t20 六表守卫修复 + t23 真浏览器 QA
- 独立性：审阅人未参与 R39 规范/实施/守卫/QA 任何一环（t17/t18/t19/t20/t23 均由其他成员完成）；本审阅仅有的先前产出为 t22（R40 评分权重），本轮以只读方式复核其与 R39 共存的当前工作树。
- **结论：PASS**（六维门禁全部通过；另附 1 条低危非阻塞观察，见第 7 节）

## 1. 规范（SPEC）— PASS

docs/laiyipao/specs/round39-wiki-skins-drop-rates/SPEC.md（55 行）八节齐备：缺口依据（service.go:1522/:1531/:1636/:1643、content.go:103-111/:558-573、battle.go:706-731、types.gen.ts:772-779/:478、stage.vue:705 消费证据）、范围与零改动面、显示格式契约（../src/skinDropMechanicsText 真实 import、禁内嵌参考实现）、UI 契约与选择器、AI 四判据、三视口 QA、契约 RED 门禁、实施分工。诚实硬门禁逐条钉死（机制型 passive code 不翻译、配置基值非百分比/千分比、replay_hash 权威、MinValue/MaxValue 教训、示例值非独立真源）。

## 2. 契约（WikiSkinDropMechanics.test.ts，143 行 11 例）— PASS

- 第 7 行真实 import 全部 8 导出（MECHANISM_EMPTY/rarityText/passiveText/dropRatesText/buildSkinDropMechanicsAi/三常量），无内嵌实现；模块缺失即整 suite RED。
- 8 导出全 unknown 安全：rarityText 三档映射/未知档原样/14 种脏值→—（:66-72）；passiveText 非空原样/11 种脏值→—（:73-77）；dropRatesText 六键整数直显、**0→"0" 合法**、小数/负数/NaN/±Infinity/"200"/对象→—、结果无 %/‰（:78-83）。
- AI 聚合：确定性、脏值 3/6 计数、非法入参 0/6、无占位符/undefined/NaN/Infinity 残留（:84-95）；三常量逐字（:96-100）。
- UI 断言：ConfigResp snake_case 六字段+六键防 camelCase 漂移（:101-108）；四选择器（r39-skin-mechanics/r39-skin-table-scroll/r39-drop-table/r39-ai-note）、五列/两列标签、真实调用、overflow-x: auto + max-width: 100%（:109-121）；AI 四要素同屏、hash 安全 CTA（禁 href="#"）、searchIndex 皮肤命中（:122-136）；SPEC 钉死+令牌三锚点+无 --r39-（:137-142）。

## 3. 实现 — PASS

- laiyipao/admin/src/skinDropMechanicsText.ts（85 行）纯函数无副作用：MECHANISM_EMPTY 自 @/enemyMechanicsText 同源 re-export（:5-6，无第二份字面量）；rarityText（:19-23）、passiveText（:26-28）、dropRatesText 有限非负整数校验（:31-34）；buildSkinDropMechanicsAi 只读数组守卫+正整数 id 计数+三档直接相等计数+六键逐项 dropRatesText 计数（:60-81）；三常量逐字（:83-85）。
- WikiView.vue：ConfigResp skins 六字段数组（:109）+ drop_rates 六键（:110-117）；SectionKey 含 'skins'（:130）；SECTIONS 目录 equipment→skins→rules 顺位（:174-176）；皮肤区 `<section v-show="activeTab === 'skins'" class="wiki-section r39-skin-mechanics">`（:1270）+ .r39-skin-table-scroll（:1279）+ 五列 el-table（ID/名称/稀有度 rarityText/被动 passiveText/说明，:1280-1293）+ el-empty；.r39-ai-note 四要素同屏（:1295-1300）：结论 buildSkinDropMechanicsAi(cfg?.skins ?? [], cfg?.drop_rates ?? {})、依据/红线字面量内联 + 常量 :title/:data-* 绑定、CTA `<a :href="urlFor('skins')" @click.prevent="goTo({ tab: 'skins' })">`（无裸 hash，R32 教训落实）。
- drop_rates 小表在 rules tab #score 区块之后（:1357-1380）：.r39-drop-table 两列（规则/数值），六行中文名逐字（基础金币/每星金币/每次击杀金币/反应加成/三星钥匙/通关返还体力），逐项 dropRatesText(cfg?.drop_rates?.key) 调用。
- searchIndex 皮肤命中（:566-575）：kind:'skin'、id:String(s.id)、title=s.name、meta=`皮肤 · ${rarityText(s.rarity)}`、body 含 name+code、section:'skins'；HitKind 联合含 'skin'（:198）、HIT_KINDS 含 'skin'（:200）、KIND_LABEL.skin='皮肤'（:213），parseFocus 经 HIT_KINDS 校验（:235），?focus=skin:N 深链高亮可用。

## 4. 真浏览器证据（t23 REPORT）— PASS

docs/laiyipao/audits/round39-wiki-skins-drop-rates/REPORT.md（129 行）亲读：三视口皮肤 6 行×5 列 MISMATCHES=0、drop 六键 200/150/20/30/1/5 MISMATCHES=0、hasPct=false、AI 四要素同屏、CTA 点击停留 skins（onDashboard=false）、搜索键入+点击两步验证（focus=skin:1/skin:6 行高亮）、表内横滚 390 视口 scrollLeft 0→316 且三视口页面级无横溢、R31–R38 回归 10 项全对。诚实记录到位：TestNoRealSecretsInRepo=SKIP（非 PASS）、r39-04/05/1280 三图同 sha 的原因说明、mcheck 探针伪影披露并以 reg-mastery.json 96 行为准。

- 截图 19 张独立重算 sha256，与 REPORT 第 10 节清单**19/19 全部一致**，唯一 sha 恰 17 个。
- 亲读三张关键截图确认均为真实 Wiki 页面（非登录页）：r39-390-skins.png（皮肤图鉴六行、稀有度中文、被动 code 原样）、r39-08-searchpick-skin_chalk.png（skin_chalk 命中后白垩女王行蓝框高亮 + AI 四要素同屏）、r39-07-reg-rules.png（rules 页分数星级表）；三图配置版本横幅一致（6237e0b623219b8b）。

## 5. 测试与构建（本轮独立复跑，2026-11-25 22:17）— PASS

| 门禁 | 结果 |
|---|---|
| R39 契约定向（WikiSkinDropMechanics + WikiReactionDescr） | **14/14 通过**，exit 0（1.16s） |
| admin 全量 vitest（--testTimeout=90000） | **79 文件 / 877/877 全绿**，exit 0（53.9s；R40 已实施，不再是 RED） |
| pnpm type-check（vue-tsc --noEmit） | exit 0 |
| pnpm build（vue-tsc + vite build） | exit 0，✓ built in 1.65s |
| git diff --check | exit 0，DIFF_CHECK_OK |
| Go 守卫（server/internal/domain） | TestNoInfraIdentifiersInRepo **PASS**（10.18s）、TestCurrentStateDocUsesDomainPlaceholder PASS、TestNoRealSecretsInRepo SKIP（本机无 .local.keys，按设计显式 Skip，非失败） |

## 6. 保护面 — PASS

- 审计目录 secrets 扫描（admin12345/私钥 PEM 块/AKIA/带口令连接串/password 赋值）零命中；mint-token.sh 仅读环境变量。
- git ls-files 无 admin-auth-init.js、data/login.json、.local.keys 跟踪；审计 data/ 目录实测无上述文件（finalize.sh 已清理）。
- 令牌三锚点（--lyp-token-version/--lyp-r3-version/--lyp-r6-version）由契约 :139 钉死且全量绿佐证；WikiView.vue 与 tokens.css 均无 --r39-（契约 :140-141）。
- 零改动面遵守：本轮复跑未发现 engine/后端/replay/hash/score/settle/loot/tokens 漂移；SeedSkins/DefaultDropRates 数值未被改动（契约 :52-58 绿）。

## 7. 观察与建议（非阻塞，不影响 PASS）

- skinDropMechanicsText.ts:22 `RARITY_LABEL[s] ?? s` 为普通对象字面量索引：若未来运营新增档位值恰为 Object 原型链键名（如 "constructor"/"toString"/"valueOf"），会命中继承属性而非原样透传。种子数据与契约均不可达，且 R40 的 ratingMechanicsText.ts 已用 Map 规避同类问题；建议后续轮次顺手改为 Map 或 Object.create(null)，不构成本轮门禁缺陷。

## 8. 结论

R39 在规范、契约、实现、真浏览器证据、测试构建、保护面六个维度全部达标，证据链完整且诚实（SKIP 如实标注、同 sha 截图有解释、伪影有披露）。**终审判定：PASS，可进入分批发布。**
