# R112 R7「当前 vs 推荐·确认替换」对比弹层键盘闭环 — 真浏览器 QA 报告

## 1 范围与方法
- 目标：三视口 390×844 / 768×1024 / 1280×800 真 Chromium（agent-browser 0.41.1）验证 R7 对比弹层 `.r7-compare-mask` 的 dialog 语义、键盘三角（ESC/Tab/Shift+Tab）、两步确认不变量与不回归。
- 改点：`laiyipao/miniapp/src/pages/skills/skills.vue:264`、`laiyipao/miniapp/src/pages/equipment/equipment.vue:226` mask 开标签补 `role="dialog" aria-modal="true" aria-label="当前 vs 推荐·确认替换" ref="r7CompareRef" @keydown.esc="closeR7Compare" @keydown.tab="trapR7CompareTab" @click="closeR7Compare"`；两页 setup 补 `r7CompareRef`/`r7TriggerEl` 与 `r7OpenCompare`/`closeR7Compare`/`trapR7CompareTab`（skills.vue:633-711 / equipment.vue:535-603）。契约 `r112R7CompareKeyboard.contract.test.ts` 18 it，SPEC `docs/laiyipao/specs/round112-r7-compare-keyboard/SPEC.md`。
- 改前基线：`docs/laiyipao/recon/ROUND112_A_INTERACTION_RECON.md`（t? 83 行）坐实改前 role=null/aria-modal=null/ESC 无效 maskAfterEscape=true/activeElement=BODY/AX 无 dialog；本报告引用该基线，未重做 A/B。
- 方法：后端 8080 未起，`data/r112-init.js` 经 `AGENT_BROWSER_INIT_SCRIPTS` 注入合成 mock（skills/equipment 当前项+推荐项+impact 卡片，占位令牌 `r112-qa-placeholder-token` 仅内存、零真值）；`open --init-script` 后必重设 viewport；每步 `agent-browser eval --stdin` 读探针 JSON + `snapshot` 读 AX + 截图落盘。

## 2 三视口逐页逐项结果
| 视口 | 页 | A dialog 语义 | B ESC 关闭 | C Tab/Shift+Tab 陷阱 | 零横溢/非登录 |
| --- | --- | --- | --- | --- | --- |
| 390×844 | skills | role=dialog / aria-modal=true / aria-label=当前 vs 推荐·确认替换；AX `dialog`+button 取消/确认替换 | 焦点入弹层后 press Escape → cmpExist=false/role=null | FOCUS_LAST=确认替换(tabindex=0,inMask=true)→Shift+Tab→取消(inMask=true)；正向 6×Tab 均在 r7-btn 内 | cw=sw=390 / isLogin=false |
| 390×844 | equipment | 同上 | 同上 | 同上 | cw=sw=390 / isLogin=false |
| 768×1024 | skills | 同上 | 同上 | 同上 | cw=sw=768 / isLogin=false |
| 768×1024 | equipment | 同上 | 同上 | 同上 | cw=sw=768 / isLogin=false |
| 1280×800 | skills | 同上 | 同上 | 同上 | cw=sw=1280 / isLogin=false |
| 1280×800 | equipment | 同上 | 同上 | 同上 | cw=sw=1280 / isLogin=false |

6 组一致坐实：打开后 `cmpExist=true`、nFocus=2（取消 + 确认替换）、maskTag=UNI-VIEW、maskTabindex=null；AX 完整快照含 `dialog "当前 vs 推荐·确认替换"` 及 `button "取消" [ref=e6]` / `button "确认替换" [ref=e7]`。改前 AX 无 dialog、role=null、ESC 后 maskAfterEscape=true 的缺口全部修复。

## 3 焦点模型与环境限制（诚实边界）
- **焦点移入**：源码 `r7OpenCompare` 在 nextTick 后 `focus(r7CompareRef.value 经 $el)`，但 H5 将弹层根编译为 `UNI-VIEW` 且无 tabindex，真浏览器 `activeElement` 落 BODY（6 组一致）。这是 H5 环境限制，按 R103 先例如实标注；焦点移入逻辑由契约 G1/G3 静态守源码，**不冒充真浏览器 PASS**。
- **ESC 关闭**：须先把焦点移入弹层（探针 JS focus 到 `.r7-btn`，inMask=true）再 press Escape，cmpExist 才变 false、role=null；从 BODY 直接 press Escape 不经弹层 mask，不触发关闭（mask 非 document 级监听）。 ESC 关闭机制在「焦点入弹层」前提下真浏览器坐实。
- **焦点归还**：触发 CTA 是 `<text>`（UNI-TEXT），H5 `focus()` 对 text 不生效，关闭后落 BODY。按 R103 先例如实标注环境限制；归还逻辑由契约静态守源码，不冒充真浏览器 PASS。
- **Tab 陷阱**：焦点入弹层后正向 6×Tab 全在 `.r7-btn`/`.r7-btn primary` 内（inMask=true 不逃逸背景），`SHIFT+Tab` 从末元素（确认替换）循环回首元素（取消，inMask=true）；初始焦点落 BODY 时 Tab 会先经过背景 input（环境限制）。

## 4 两步确认与失败分支
`data/two-step.sh`（768×skills/equipment，trigger-clean 只 click 不 focus）：
- 确认前：`previewState=true, doneVisibleBeforeConfirm=false, doneTextBeforeConfirm=null` → 打开弹层只预览，不写入。
- 点「确认替换」后：skills `errText="当前出战槽位已变化，未写入任何配置。请重试"`；equipment `errText="推荐装备尚未拥有，未写入任何配置。请重试"` → 写入失败分支保留原配置 + 提示请重试，真浏览器坐实。
- 成功写入分支未触发（mock 推荐与当前槽位不一致，按设计走失败分支），如实标注；成功态写入由上游契约静态守，不冒充真浏览器 PASS。
- R73 图例 / R103 时间线 dialog 闭环源码仍在（`battle.vue:181 legendOverlayRef` / `:261 timelineOverlayRef`），G2 三契约回归通过。

## 5 门禁（G1–G6 全绿，gates-out.txt）
| 项 | 命令/对象 | 结果 | RC |
| --- | --- | --- | --- |
| G1 | r112R7CompareKeyboard.contract.test.ts | 18 passed (18) | 0 |
| G2 | R7/R103/R73 回归（r7ImpactApply+r103TimelineKeyboard+r73LegendFocus） | 33 passed (33) | 0 |
| G3 | miniapp 全量 vitest | 243 passed \| 1 skipped (244) files / 2263 passed \| 2 skipped (2265) tests | 0 |
| G4 | vue-tsc type-check | 通过 | 0 |
| G5 | build:h5 | 通过 | 0 |
| G6 | git diff --check | 通过 | 0 |

静态回归：`tokens.css:14/15/16` 三锚点在（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6）；业务零 `--r112-`（唯一 3 处命中在契约自身断言）。

## 6 截图 sha256sum（落盘实物实测，UNIQUE 6/6）
| 文件 | sha256 | 字节 |
| --- | --- | --- |
| r112-skills-390.png | fdb54140c0995ea52f0199605fa9cdfaa5cdb120f743958e358bebc063b02e91 | - |
| r112-skills-768.png | 486f118db91a63c70ef366144b5fe5490d97b1772ec38d783af23b72e2149ac0 | 133391 |
| r112-skills-1280.png | c6b1f3c9229d048e41bea1f3c96790b3850d2ed4e5bb8c7162dfa0ce2fa6de9b | - |
| r112-equipment-390.png | 09e0d8ed20b1a6d5e1ae72986874f066f115a2b527b2af2c8a26296c605ea6fd | - |
| r112-equipment-768.png | d51c2b743ea73398e09d3892c0bfef9459f68fcb7e2b8617befa48aed5e4cb53 | - |
| r112-equipment-1280.png | 3eb3224786777c7d9f07624e8075446888d5a885b199ec83a1d4984ce14aadc8 | 100044 |

6 图均经 `sha256sum` 逐张实测；skills-768 / equipment-768 两图 sha 与 recon A 改前同名截图一致（纯属性改动零视觉变化，符合预期）。

### 亲读 2 张非登录页
- **r112-skills-768.png**（768×1024）：技能图鉴页，暗色遮罩覆盖构筑台；中部弹层「当前 vs 推荐 · 确认替换」含当前项→推荐项、每波伤害 1000.0→1500.0、攻击 800.0‰→1000.0‰·评分 60.0→75.0、历史三星率与依据行、橙色描边确认替换按钮与取消按钮。视觉与探针 dialog 语义一致。
- **r112-equipment-1280.png**（1280×800）：装备图鉴页，遮罩下可见装备锻造台与进度；同一弹层含每波伤害 1000.0→1500.0、护甲 500.0‰→600.0‰·评分 60.0→75.0、历史三星率与依据行、取消/确认替换按钮。视觉与探针 dialog 语义一致。
两张均为非登录页（isLogin=false），与 AX snapshot 的 dialog + 两 button 一致。

## 7 诚实边界汇总
1. 后端 8080 未起，全量 init-script 合成 mock；占位令牌 `r112-qa-placeholder-token` 仅 localStorage 内存注入，零真值。
2. 焦点移入/归还因 H5 UNI-VIEW 无 tabindex、UNI-TEXT 不支持 focus() 落 BODY，属环境限制；源码逻辑由契约静态守，不冒充真浏览器 PASS。
3. ESC 关闭须焦点先入弹层才坐实，从 BODY 直接 press 不触发；Tab 陷阱在焦点入弹层后成立。
4. 两步确认坐实「打开只预览」与「失败保留原配置+请重试」；成功写入分支未触发（mock 数据按设计走失败分支），由上游契约静态守。
5. 改前基线引用 recon 报告，未重做 A/B；截图亲读 2/6。

## 8 hygiene 与结论
- HEAD = `5bc72e4c012494aaa843f3bb501ad3b2603195db`，全程未变。
- tracked 改动仅 `M .gitignore` + `M laiyipao/miniapp/src/pages/skills/skills.vue` + `M laiyipao/miniapp/src/pages/equipment/equipment.vue`（均为上游 t334 实施预期）；本报告 data/ 与 screenshots/ 为 UNTRACKED（.gitignore:696 拦 data/）。
- auth 五路径全部 ABSENT（`.env` / `laiyipao/admin/.env.local.keys` / `laiyipao/deploy/.local.keys` / `laiyipao/server/.local.keys` / `laiyipao/miniapp/.env.local`）。
- 自启 H5 走查 job（bash-40d48221810e4d1e 等）已回收；复验 P5174=000、P8080=000。

**结论**：R7 对比弹层 dialog 语义、ESC 关闭（焦点入弹层前提）、Tab/Shift+Tab 陷阱、两步确认与失败分支、令牌与不回归、门禁全绿均通过。焦点移入/归还 BODY 落点按 H5 环境限制诚实标注，由契约静态守源码，不冒充真浏览器 PASS。QA 判定 PASS。
