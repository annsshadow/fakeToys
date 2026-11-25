# R43 独立终审：战斗 HUD AI 解读区 CTA 文案—行为一致性闭环

- 审阅人：same-route-reviewer（R43 独立终审，继承队长路由）
- 被审对象：t43 实施（battle.vue 四处最小删改，切断 R27/R24 AI 解读区 CTA 到 openHelp 的错误绑定）与 t45 真浏览器复查
- 依赖链：t42 规范与真实 RED → t43 实施 → t45 真浏览器复查 PASS（带诚实边界）→ 本轮 t47 终审
- 裁决：**PASS（可发布）**

## 独立性声明

本轮仅做只读复核与独立复跑，未修改任何业务源码、未做任何 git 写操作；唯一落盘文件为本报告。全部结论基于亲自阅读 SPEC（75 行）、两份契约（221 / 216 行）、battle.vue 与 .gitignore 的只读 diff、审计 REPORT（102 行）与 9 份关键 QA 证据 JSON/TXT，独立重跑契约、battle.test、全量测试、type-check、build:h5、Go 守卫，独立重算 13 张截图 sha256，并亲读 4 张关键截图。

## 维度一：SPEC 与契约逐条复核 —— PASS

| 检查项 | 证据 | 判定 |
| --- | --- | --- |
| SPEC 9 节齐备 | docs/laiyipao/specs/round43-battle-hud-cta-honesty/SPEC.md 75 行，§0 缺口依据到 §8 红线零真值完整 | PASS |
| 缺口依据真实 | battle.vue:53/80 注释宣称只读不拦截，:58/:95 却绑 @click=openHelp；:982-1000 openHelp() 调 doPause() 暂停战斗；:2804/:2778 CSS 含 pointer-events:auto；盲区 :158/:167 只查文案 | PASS |
| 契约真实源码守卫、无内嵌实现 | r27 契约第 4 行真实 import buildWaveProgress from ./waveProgressHud、第 10 行 readSrc('...battle.vue')；r24 契约第 124 行 readFileSync(join(__dirname,'./battle.vue'),'utf-8')；两者各含 R43 describe，用 ctaLine/cssRule 定位真实源码文本断言 | PASS |
| RED 断言集合齐全 | 两 CTA 行 not.toContain('openHelp') / not.toContain('@click')；两 CSS 规则体 not.toContain('pointer-events: auto') / not.toContain('pointer-events:auto')；R26 锚点 not.toContain('@click') | PASS |
| 令牌三锚点原样、无 --r43- | tokens.css:14-16 原样 lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6；battle.vue 与审计目录 grep --r43- 零命中 | PASS |
| 硬禁改面声明 | SPEC §1 明列禁改 engine/replay/replay_hash/score/settle/loot/后端/tokens.css/.gitignore（实施轮仅动 battle.vue；.gitignore 为审计规则非业务配置） | PASS |

## 维度二：修复判据逐条核对 —— PASS

本轮修复为纯模板/CSS 删改（无新 helper），对照 SPEC §4 六条判据逐条核对 battle.vue 只读 diff（精确 4 处，8+/4-）：

1. r27-ai-cta 模板行删 @click=openHelp，逐字文案『继续清理场上敌人 →』保留 —— 符合判据 1、4。
2. r24-ai-cta 模板行删 @click=openHelp，逐字文案『去释放对应元素技能 →』保留 —— 符合判据 2、4。
3. .r24-ai-cta / .r27-ai-cta CSS 各删 pointer-events: auto，只留 color/font-size/font-weight —— 符合判据 3。
4. .r26-ai-cta 仍无 @click（对齐目标锁定）；openHelp() 函数体、.help-chip、r28-feed-cta、r29 均未进 diff —— 符合判据 5、6。
5. 只读派生不进 replay_hash，红线文案原样 —— 符合判据 6 与 §5。

diff 未触任何 engine/replay/hash/score/settle/loot/tokens 代码，改动面与 SPEC §1 完全一致。

## 维度三：真浏览器 QA 证据独立判断 —— PASS

亲读 data/v2-r27-before/after.json、v2-r24-before/after.json 与两份坐标点击日志（各含 MOVE 坐标 + 3 行 Done），独立核对：

| 项目 | r27 点击前→后 | r24 点击前→后 |
| --- | --- | --- |
| 倒计时（距满分） | 138s → 135s（推进 3s） | 133s → 130s（推进 3s） |
| helpOverlay / hasTimeline | false→false / false→false | false→false / false→false |
| hasPause / hasResume | 0/0 → 0/0 | 0/0 → 0/0 |
| pauseLabels | [] → [] | [] → [] |
| href / hpNow / waveNow | 战斗页 level=5 / 1100 / 1 不变 | 同左不变 |

坐标来自真实探针（r27 中心 196,168；r24 中心 196,651），采用真实底层鼠标 move/down/up 事件而非 DOM eval 派发。正对照 hc-before/hc-open-state/hc-closed-state 显示：点 .help-chip 后 helpOverlay=false→true、bodyLen 1062→2123、计时冻结 138s；点 .legend-close 后 overlay 关闭、bodyLen 回 1062、计时恢复 137s。证明帮助层正路径完好，CTA 静默是定向修复而非全局禁用点击。页面始终处于战斗态（非结算态）。

## 维度四：截图 sha256 与亲读 —— PASS

独立 sha256sum 重算 REPORT 第七节 11 张有效图，逐行与报告清单完全一致；11 张两两互异。另单独重算两张弃用图 r43-v2-help-open/closed，sha 同为 da0586…，与报告『同 sha 无效空白图』披露一致。

亲读 4 张关键图均为 390×844、非登录页：r43-v2-r27-after.png（真实战斗态，倒计时 135s，r27 CTA 为纯文本，无图例遮罩）；r43-v2-r24-after.png（真实战斗态，倒计时 130s，r24 CTA 为纯文本，无图例遮罩）；r43-hc-help-open.png（『战场图例』弹层可见，正对照成立）；r43-initial.png（未登录深链的第 5 关未解锁态，与报告 §三预期披露一致）。

## 维度五：独立复跑门禁 —— PASS

| 命令 | 结果 | 退出码 |
| --- | --- | --- |
| vitest run 两 R43 契约（--reporter=dot） | 2 文件 / 33 通过 | 0 |
| vitest run battle.test.ts | 1 文件 / 61 通过 | 0 |
| pnpm test（miniapp 全量） | 212 文件通过 / 1 跳过；1988 通过 / 2 跳过 | 0 |
| pnpm run type-check（vue-tsc） | 无报错 | 0 |
| pnpm run build:h5（uni build） | DONE Build complete | 0 |
| git diff --check | 无 whitespace 错误 | 0 |
| Go TestNoInfraIdentifiersInRepo | PASS（5.07s） | 0 |
| Go TestNoRealSecretsInRepo | **SKIP（非 PASS）**：缺 .local.keys，真值扫描未执行 | 0（整体 ok） |

stderr 中『build.attacker 缺失』为测试占位告警，非失败，与 REPORT 一致。

## 维度六：保护面与证据完整性 —— PASS

- tracked diff 仅 4 文件：.gitignore（R43 审计 ignore 规则）、battle.vue、两份 R43 契约；无 engine/replay/replay_hash/score/settle/loot/tokens/后端 改动。R43 写域与 R42 admin WikiView.vue 写域不同，未混淆。
- 零真值：四个临时 auth 路径全 ABSENT；git check-ignore -v 命中 .gitignore:466（审计 data/ 目录）；审计目录文本 SECRET_HITS=0；QA 库 docker exec lyp-qa-pg 查 battle_records = count 104 / max(id) 477。

## 诚实标注（不作为 PASS 依据，但须随报告披露）

1. TestNoRealSecretsInRepo 为 SKIP 而非 PASS：缺 laiyipao/deploy/.local.keys，真值扫描守卫未执行，不计入『守卫全绿』。
2. r27/r24 选择器点击被画布遮挡（agent-browser 报 covered by a div inside uni-canvas），系 CTA pointer-events:none 的设计使然、非产品缺陷；最终证据改用真实坐标鼠标事件。
3. r43-v2-help-open/closed 两图字节相同（同 sha da0586…），为视口重置后的空白无效图，已弃用；正对照改用 hc 系列图。
4. hasResume=0 / pauseLabels=[] 在图例打开时仍为假阴性，可靠暂停判据为 helpOverlay + 计时冻结/恢复 + 截图。
5. R28/R29 真实点击开时间线本轮未单跑浏览器用例，仅源码只读确认 handler 保留。
6. 首次 coord-verify.sh 有 bash 变量拼接 bug（MOVE=9140{stage}X），已由 coord-verify2.sh 修正，最终证据只采用 v2。
7. r43-initial.png 为未登录深链锁定态，非登录页但亦非战斗态，仅作进入流程披露，不作 CTA 证据。

## 结论

R43 战斗 HUD CTA 诚实性修复满足六维门禁：规范与契约真实守卫、四处最小 diff 精准命中判定据、真浏览器坐标点击证明不再误暂停、截图与 QA 证据自洽、全部可自动化门禁独立复跑通过、保护面与零真值检查干净。所有诚实边界均已披露且不影响主结论。裁决 **PASS（可发布）**。
