# R112 独立终审：R7 对比弹层 dialog 键盘闭环

## 总裁定

- 被审任务：t335（same-route-ui 实施），本终审 attempt=276eb7a5-e5a0-4a6d-b4b6-7118f6c13a42。
- 基线 HEAD=5bc72e4c012494aaa843f3bb501ad3b2603195db，全程未变（收尾复核一致）。
- **verdict=pass**：六维全绿，无 blocker / high / medium；8 条 low 均为诚实边界与可读性建议，不阻断分批发布；同意放行。
- 审法：不复用实施/QA 结论，独立读契约源码、RED 原始日志、完整 diff、走查日志；独立重算 6 图 sha；独立复跑全部门禁；独立 stash 基线核验队长契约修复合法性；亲读 2 张异于 QA 选图的截图。

## 维度① RED→GREEN 诚信与契约修复合法性 —— PASS

- 契约 laiyipao/miniapp/src/styles/r112R7CompareKeyboard.contract.test.ts（178 行 / 18 it）真实 fs.readFileSync 读源码，tagBefore 按唯一 class 锚点、blockAfter 按函数名括号配对定位，均不依赖行号。
- RED-raw.log 逐字：Failed Tests 12 / Test Files 1 failed(1) / Tests 12 failed | 6 passed (18) / Duration 327ms；12 失败全为两页各 G1–G6 真实缺口（G1 缺 role=dialog、G2 缺两 ref、G3 缺 document.activeElement+nextTick、G4 缺 closeR7Compare、G5 残留 5 处内联关闭、G6 缺 trapR7CompareTab），6 不变量全绿，无 PARSE_ERROR。首跑 20:45:30 与复跑 20:46:17 失败集合相同。
- 队长就地修复规范轮引入的两处契约缺陷：4 处类型标注（hasRef(s: string, n: string) / tagBefore(src: string, needle: string, tag: string) / blockAfter(text: string, keyword: string)）+ hasRef 正则由双反斜杠改单反斜杠（BS=String.fromCharCode(92)，new RegExp(n+BS+'s*='+BS+'s*ref')）。
- **独立核验修复合法性/实现无关性**：stash 两 vue 回退到基线（skills 08bc5bb1→87bb06cc，equipment c1a7a86f→f6693344），此时修复后的契约下 vue-tsc RC0（证明类型修复与业务实现无关、自洽可编译），契约仍 RED 12 failed | 6 passed (18)（证明契约真实测缺口、非被实现牵动）；stash 已 pop 恢复，两 vue hash 回到修复态。修复合法且与实现无关。
- GREEN：独立复跑 G1 契约 18 passed (18) RC0；与实施轮 GREEN 一致。

## 维度② diff 精确性 —— PASS

- git diff --numstat：.gitignore 4+/0-、equipment.vue 42+/4-、skills.vue 42+/4-（业务对称 42+/4-）；git diff --check RC0。
- 两页改动一字不差对称，逐处吻合 SPEC §2 六处：mask 开标签补 role=dialog + aria-modal=true + aria-label=当前 vs 推荐·确认替换 + ref=r7CompareRef + @keydown.esc=closeR7Compare + @keydown.tab=trapR7CompareTab，遮罩 @click 改调 closeR7Compare；取消/完成两 text 的 @click/@keydown.enter 由内联置 false 改调 closeR7Compare()；import 加 nextTick；setup 增 r7CompareRef/r7TriggerEl 两 ref(null)；r7OpenCompare 记录 document.activeElement 并 nextTick focus 根 view；新增 closeR7Compare()（关弹层 + focus 归还 + 清空触发源）与 trapR7CompareTab(e)（querySelectorAll 查 [tabindex=0]/[role=button]，shiftKey 首尾循环 + preventDefault）。
- 内层 @click.stop、v-if、r7ConfirmApply 两步确认逻辑未动；文案/class/CSS 未改。

## 维度③ QA 证据 —— PASS

- 6 图 sha256 独立重算与 QA 报告逐字一致 UNIQUE 6/6：r112-skills-390=fdb54140…、skills-768=486f118d…、skills-1280=c6b1f3c9…、equipment-390=09e0d8ed…、equipment-768=d51c2b74…、equipment-1280=3eb32247…（均落盘实物实测）。skills-768/equipment-768 与 recon A 改前同名图 sha 一致（纯属性改动零视觉变化，符合预期）。
- 走查日志 w-skills-390.txt / w-equipment-1280.txt 逐字：打开后 cmpExist=true、role=dialog、ariaModal=true、ariaLabel=当前 vs 推荐·确认替换、nFocus=2（取消+确认替换）、maskTag=UNI-VIEW、maskTabindex=null、AX 含 dialog + button 取消[e6] + button 确认替换[e7]、cw=sw、hOverflow=false、isLogin=false；焦点入弹层后 press Escape → cmpExist=false、role=null；WALK_DONE。
- 终审独立亲读 2 张（刻意异于 QA 选的 skills-768/equipment-1280）：r112-skills-390.png（390×844，sha fdb54140…）与 r112-equipment-768.png（768×1024，sha d51c2b74…），均为非空态真实弹层页——暗色遮罩下 dialog「当前 vs 推荐 · 确认替换」含当前项→推荐项、每波伤害 1000.0→1500.0、攻击/护甲千分比与评分 60.0→75.0、历史三星率与依据行、取消/确认替换按钮，与探针 dialog 语义一致，非登录页。

## 维度④ 键盘闭环与焦点归还边界 —— PASS（含诚实边界）

- dialog 语义 / ESC 关闭 / Tab+Shift+Tab 陷阱属 DOM 层，真浏览器坐实：6 组 role=dialog + AX dialog 节点；焦点入弹层后 ESC → cmpExist=false；正向 6×Tab 全在 r7-btn 内、Shift+Tab 从末元素循环回首元素（inMask=true 不逃逸背景）。
- esc-focus-out.txt：768 两页 focus first（inMask=true）→ press esc → cmpExist=false。supp-out.txt：真实 Enter 触发 CTA 后 ACTIVE 仍为 CTA（inMask=false）、从 CTA press esc 弹层仍在（cmpExist=true，如实边界）；SHIFT_DISPATCH before/after 均 r7-btn、prevented=false。two-step-out.txt：两页确认前 previewState=true / doneVisibleBeforeConfirm=false → 点确认后 errText 含「请重试」/ doneVisible=false（失败分支保留原配置）。
- **焦点边界按 R103 先例列为 low，不冒充 PASS、不判回归**：触发 CTA 与弹层按钮均为 text（UNI-TEXT），H5 focus() 不生效；焦点移入优先 focus 根 view（UNI-VIEW），真浏览器 activeElement 仍落 BODY；焦点归还目标 CTA 为 text，关闭后落 BODY。源码含 focus 调用由契约 G3/G4 静态守，非 DOM 层真 PASS。

## 维度⑤ 门禁独立复跑 —— PASS

- 独立后台复跑六项，全部 RC0（.t337-gates.out，GATES_DONE）：
- G1 r112 契约 18 passed (18)；G2 R7/R103/R73 回归 33 passed (33)（15+12+6）；G3 全量 243 passed | 1 skipped files / 2263 passed | 2 skipped tests；G4 vue-tsc type-check RC0；G5 build:h5 RC0；G6 git diff --check RC0。
- 与 QA gates-out.txt 完全一致，复跑无差异。

## 维度⑥ 不变量与卫生 —— PASS

- 两步确认不变量完整：open 只预览不写入、confirm 才写 persistLoadout/applyEquipmentImpact；写入失败保留原配置 + 「请重试」真浏览器坐实；契约 I1/I2 静态守。
- 令牌三锚点 tokens.css:14/15/16（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6）未漂移；业务两页零 --r112-（唯一命中在契约自身断言）；I3 守。
- R73 图例 / R103 时间线 dialog 闭环源码仍在（battle.vue legend/timeline overlay 与 trap 函数），I4 守，G2 三契约回归通过；r7ImpactApply 假守卫保留记录且语义检查由本契约补齐（I5）；mask v-if 与内层 @click.stop 保留（I6）。
- git status 非 ?? 项仅 M .gitignore + M skills.vue + M equipment.vue（均为 t335 实施预期）；auth 五路径（.env / admin/.env.local / deploy/.local.keys / server/.local.keys / miniapp/.env.local）全 ABSENT；三视口 cw=sw、零横溢、isLogin=false；报告零真值。

## 问题分级（均 low，不阻断）

- L1（low）后端 8080 未起，全部浏览器走查用 init-script 合成 mock，占位令牌仅内存、零真值；非真后端数据。
- L2（low）焦点移入/归还因 H5 UNI-VIEW 无 tabindex、UNI-TEXT 不支持 focus() 落 BODY，属环境限制；按 R103 先例由契约静态守源码，不冒充真浏览器 PASS、不判回归。
- L3（low）ESC 关闭须焦点先入弹层才坐实，从 BODY 直接 press 不触发（mask 非 document 级监听）。
- L4（low）Tab 陷阱在焦点入弹层后成立；初始焦点落 BODY 时 Tab 会先经过背景 input。
- L5（low）两步确认成功写入分支未触发（mock 推荐与当前槽位不一致按设计走失败分支），由上游契约静态守。
- L6（low）改前基线引用 recon A 报告，终审未重做 A/B。
- L7（low）截图终审亲读 2/6（刻意异于 QA 选图），其余 4 张以 sha 一致 + QA 报告为据。
- L8（low）hasRef 正则以 String.fromCharCode(92) 规避转义，功能正确且已由 RED 反证有效，但可读性略逊于字面量；仅建议，不阻断。

## 放行裁定

六维验收全部满足，无 blocker/high/medium，契约修复合法且与实现无关，焦点边界按 R103 先例诚实标注未冒充 PASS。**verdict=pass**，同意纳入分批发布。
