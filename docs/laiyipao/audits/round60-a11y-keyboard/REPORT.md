# R60 真浏览器 QA 报告：role=button 键盘可达性（Tab 聚焦 + Enter 激活）

- 任务：t103 / same-route-qa；attempt_id=`19436879-9e36-4b8d-933f-0bf530578833`
- 工作目录：`.agents/worktrees/dsh-d0bc4721-3248`；起点 HEAD `fe93c86e5`
- 性质：只读 QA 走查，不改业务源码、不 git 写；审计产物写入 `docs/laiyipao/audits/round60-a11y-keyboard/`
- 规格：`docs/laiyipao/specs/round60-a11y-keyboard/SPEC.md`；契约 `laiyipao/miniapp/src/styles/r60A11yKeyboard.contract.test.ts`（6 it）

## 0. 结论摘要（诚实版）

核心目标 **PASS**：改前 R60 侦察 12/12 次 Tab 只在 BODY/INPUT 循环、48 处 role=button 键盘不可达；本次真 Chromium 在 6 代表页 × 3 视口共 18 组、每组 25 次真实 Tab，activeElement 全部能落到 role=button，Enter 激活与原鼠标 @click 行为一致，focus-visible outline 非 none、cursor 为 pointer。

但以下四项必须如实披露，**不得声称全绿**：
1. **git diff --check RC=2**：`.gitignore:553: new blank line at EOF.`（业务源码 `git diff --check -- laiyipao/miniapp/src` RC=0）。硬门禁要求全绿，此项未满足；QA 无权改 .gitignore（队长资产），不代为修复。根因是 .gitignore 累积 R58-B/R59/R60 审计目录规则后末尾多一空行。
2. **截图非全互异**：25 张中 24 唯一，`r60-enter-announcements.png` 与 `r60-mouse-announcements.png` 同 sha（两者同为 Enter/点击后抵达的战报页，预期合理）。
3. **TestNoRealSecretsInRepo SKIP 非 PASS**：因读不到 `laiyipao/deploy/.local.keys`，真值扫描守卫未执行；Go 进程 RC=0 不代表该用例通过。
4. **CTA 键盘覆盖残留（R60 范围外，非回退）**：R12 自指 CTA「在本页强化装备契合 →」与 battles「看战报详情 →」仍是纯 `view`/`text`，无 `role=button`/`tabindex`/`cursor:pointer`。R60 契约仅覆盖 48 个已标 role=button 的节点，这些历史 CTA 不在其内；其 @click 言行完好（鼠标可达），不构成 R43/R44/R46/R47/R48 回退，但键盘不可达的残留缺口须披露。

## 1. 环境与基线

| 项 | 值 |
| --- | --- |
| API | `start-api.sh` 后台 job，`/api/v1/config`=200，version=`6237e0b623219b8b` |
| H5 | 在 `laiyipao/miniapp` 下 `pnpm run build:h5` RC=0；bundled python `http.server 5201 --bind 127.0.0.1 --directory dist/build/h5`，根=200 |
| 端口 | 5174 为旧 302 服务，弃用；仅用 5201 |
| QA 库 | `laiyipao`（docker `lyp-qa-pg`），`battle_records` count|max = 104|477 |
| 认证 | users.id=10 guest_token 经 `POST /api/v1/auth/guest` 换 token，`data/auth-init.js` 516B 会话侧注入；不改 DB/服务端；`git check-ignore` 命中 `.gitignore:549` |

## 2. 走查方法

- 浏览器：真 Chromium（agent-browser）。wrapper 必须 `function AB(){ agent-browser --session "$1" "${@:2}"; }` 转发全部参数。
- 探针：`r60-probe.js`（href/hash/innerW/docW/hOverflow/rbCount/visCount/visAllTabindex/visAllCursorPointer/visEls[tag,cls,txt,tabIndex,hasAttr,cursor,outline,rect]/active/bodyLen）；`r60-focus.js`（每次 Tab 后 activeElement 的 computed outline/cursor/role/txt）；`r60-state.js`（Enter 前后 hash/bodyLen/scrollY/可见文本）；`r60-reg-probe.js`（回归）；`r60-cta-probe.js`（CTA 不变量补证）。
- 驱动：`r60-driver.sh`（6 页 × 3 视口：open → set viewport → probe → screenshot → 25× press Tab，每 Tab 后 eval focus 追加 `tabs-NAME.txt`）；`enter-driver.sh`（6 页 768：focus CTA → before → press Enter → after → screenshot）；`reg-driver.sh`；`cta-driver.sh`。

## 3. Tab 聚焦序列（18 组，每组 25 次真实 Tab）

改前基线：R60 侦察 12/12 次 Tab 只在 BODY/INPUT 循环。改后结果（job `bash-b250087946b94ab1` RC=0，18 组全有效）：

| 页 | 390 rbHits/firstHit | 768 rbHits/firstHit | 1280 rbHits/firstHit |
| --- | --- | --- | --- |
| announcements | 9 / 1 | 9 / 2 | 9 / 2 |
| reactions | 20 / 1 | 22 / 1 | 21 / 1 |
| equipment | 14 / 1 | 10 / 4 | 14 / 2 |
| skills | 16 / 1 | 16 / 1 | 16 / 1 |
| shop | 14 / 1 | 9 / 3 | 14 / 2 |
| mastery | 9 / 1 | 9 / 1 | 14 / 2 |

全部 18 组：`innerW == docW`、`hOverflow=0`、`hrefOk=true`、`visAllTabindex=true`、`visAllCursorPointer=true`。焦点 outline：390/1280 为 `solid 1px rgb(255, 107, 53)`，768 为 `solid 3px rgb(255, 107, 53)`（=3rpx accent），全部非 none。

## 4. Enter 激活（job `bash-fe9ccf25fc034df0` RC=0，768 六页）

| 页 | 行为 | before → after | 断言 |
| --- | --- | --- | --- |
| announcements | 导航到战报 | hash #/pages/announcements → #/pages/battles；bodyLen 692→1125 | 真导航，与原 @click 同效 |
| reactions | 展开反应详情 | bodyLen 2897→3055（+158）；scrollY 42→1212 | 可见副作用 |
| equipment | 展开装备详情 | bodyLen 2320→2440（+120）；scrollY 47→848 | 可见副作用 |
| skills | why-toggle 展开 | bodyLen 3620→3733（+113）；scrollY 92→1104 | 三个 why-toggle Enter 展开坐实 |
| shop | 聚焦商品 | bodyLen 1261→1261；scrollY 29→3803 | pageScrollTo #shop-id 真滚到目标 |
| mastery | 聚焦概览 | bodyLen 821→821；scrollY 0→1355 | pageScrollTo .overview-card |

## 5. focus-visible / cursor computed

- `[role="button"]` 节点 computed `cursor: pointer`（common.css:907 `[role="button"] { cursor: pointer; }`）。
- focus-visible（common.css:908 `:where(a, button, input, textarea, select, [tabindex]):focus-visible` + `outline: 3rpx solid var(--accent)`）：真实 Tab 聚焦时 outline 为 `solid 1px`（390/1280）或 `solid 3px`（768）accent 色，非 none。

## 6. 回归与正对照（768，job `bash-20e6309f32d84536` RC=0）

- R51：`#/pages/me/me` 雷达 `viewBox="0 0 200 180"`；R53 结论「第 60 关 · 评分 165，优先补「装备契合」（当前 0%）」；index「装备契合仅 0%（权重 21%）：优先补它，收益 378」。
- R55：reactions CTA「去打一场战斗 →」`white-space=nowrap`（修复保持）；搜索占位色正常。
- 鼠标正对照：announcements `mouse-before=#/pages/announcements/announcements` → `click ".r3-ai-cta"`（✓ Done）→ `mouse-after=#/pages/battles/battles`，与 Enter 同效。
- CTA 不变量补证（job `bash-b0f24be509d24142` RC=0，768）：battles `r20-ai-cta` 源码 `battles.vue:151` 带 `role=button tabindex=0 @keydown.enter=goStage()` @click=goStage()；index `index.vue:672` 装备契合 label「去配装备」；equipment/bag/gems/skills `r12-priority-cta @click="r12Go()"` 完好。
- **残留缺口（披露）**：CTA 探针在真浏览器发现 equipment「在本页强化装备契合 →」（class `r12-priority-cta`，visible=true，role=null，tab=-1，hasTab=false，cursor=auto）与 battles「看战报详情 →」（class `r3-ai-cta`，role=null，tab=-1，cursor=auto）仍是普通元素，键盘不可达。二者不在 R60 的 48 个 role=button 契约范围，@click 言行无回退，但属整体 a11y 残留，建议后续轮次评估。

## 7. 门禁结果

| 门禁 | 结果 | RC |
| --- | --- | --- |
| 契约 r60A11yKeyboard | 6/6 passed | 0 |
| miniapp 全量 | 217 files passed / 1 skipped；2020 tests passed / 2 skipped | 0 |
| type-check（vue-tsc） | passed | 0 |
| build:h5 | DONE Build complete | 0 |
| git diff --check（业务源码 laiyipao/miniapp/src） | 干净 | 0 |
| git diff --check（全仓） | **FAILED**：`.gitignore:553 new blank line at EOF.` + LF→CRLF warning | **2** |
| Go TestNoInfraIdentifiersInRepo | PASS（8.22s） | 0 |
| Go TestNoRealSecretsInRepo | **SKIP 非 PASS**（读不到 .local.keys，真值扫描未执行） | 0（进程） |

令牌三锚点：`tokens.css:14-16` 逐字在 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`；全仓 `--r60-` 仅契约断言 2 处（`r60A11yKeyboard.contract.test.ts:83,88` `expect(tokens).not.toContain("--r60-")`），业务 0 命中。

## 8. 截图 sha256 表（25 张，24 唯一）

| 文件 | sha256 | 字节 |
| --- | --- | --- |
| `r60-1280-announcements.png` | `a11d93bb750cee8f18577c7feae24ceed95ca6d128b7eb96ece8d428ae14f2f7` | 51547 |
| `r60-1280-equipment.png` | `130d6dbd97d1fb0c27b1943e9d255a538a6011fbff8d4ce17bac74a845d96499` | 65176 |
| `r60-1280-mastery.png` | `3654b441d879c9494a1410587e2fc2d2a95b94e13e4f663d269a82b0c8ddf1fb` | 96685 |
| `r60-1280-reactions.png` | `9f5bc1d7e18c2c765a1c78e6f9e66b7993e57385acfb73dead26ffcb15dc83ed` | 110358 |
| `r60-1280-shop.png` | `7efaebd9fa6fcbc2df80a0e98e45a0f8ffb9ac09fd5aabe5555db7674b16d243` | 137793 |
| `r60-1280-skills.png` | `f9dfa52124515a077b6bee721305e2a8c1c31d745e2515edae62c4c85f79ee56` | 82065 |
| `r60-390-announcements.png` | `6ab29fc32066bc9bf24d89331da6513646ad7db00a10aff955d316353b7fb10c` | 111909 |
| `r60-390-equipment.png` | `02a102fb22b321b44d31507fc23b11fd711404ae962d64a54956737fcd258b43` | 125271 |
| `r60-390-mastery.png` | `6eb8befddcd1aa2b6e880c1ce64cdef9f9db1ae00e3f0e1c2d87b06e0d276870` | 116447 |
| `r60-390-reactions.png` | `46ba749089e8ceb67d161824426138295de70a8c79a051bad9b213028e96e64e` | 101487 |
| `r60-390-shop.png` | `20b7c2c6126783119303d3ad2b415776c89c130bccda39f9deb91980578ed872` | 127486 |
| `r60-390-skills.png` | `deda2d38ffe617db7b7acf9bffd37a04efbf8e1a65d4fe8330fae5224d4ed947` | 125688 |
| `r60-768-announcements.png` | `5542faa78e5f89abec8bd4f95e9e997940d5e850f01f54f6761ae6e98900774d` | 251260 |
| `r60-768-equipment.png` | `8d4a7c1089f96f03cae2a2faa235229e03d82b257e7ae764a9ca648000ea33ba` | 178821 |
| `r60-768-mastery.png` | `85d610bcb73c963a7b7c0d3b37e064226dc4a20245d9ae95a1d6eb0f833eaaeb` | 256238 |
| `r60-768-reactions.png` | `92e2f8ac6b49be03833668ab97179b08d6a10731c5c7b83803cbb07fa7698c6c` | 106071 |
| `r60-768-shop.png` | `47e21468bf0849ef4400599f965b1cea385a19ed3565ce200899d44c29fc5a29` | 260750 |
| `r60-768-skills.png` | `ab3fb7d863bf2350a668efbc63741979ed4d20c8c164f3d7fdae2819d497df5e` | 207389 |
| `r60-enter-announcements.png` | `d5482858d5c63f031479bd29bddb6f28b312434cee16775919ed6da0852832b1` | 217466 |
| `r60-enter-equipment.png` | `a4af9b0f78b49c28e7edd9c2192460d6eb0e2738d7607a4c72dc6db99ec00429` | 151425 |
| `r60-enter-mastery.png` | `9d6c1a1a88ea09bcebd4279404f599f024701cc70c233715986eb448128166fa` | 82891 |
| `r60-enter-reactions.png` | `43302b33cb26ef109bb5cb92a4c305b06459c17fe93db9e7d4b098fe3b2229fd` | 122532 |
| `r60-enter-shop.png` | `f6a9a86886746a0e00c0d8b6dc21b9610dab564fe8f56dd931d4a594eb1cffa3` | 97450 |
| `r60-enter-skills.png` | `84fae5b2364a3423dafccd2ca02648922b869769e81d78214f59921dd40cfcd6` | 147826 |
| `r60-mouse-announcements.png` | `d5482858d5c63f031479bd29bddb6f28b312434cee16775919ed6da0852832b1` | 217466 |

唯一重复：`r60-enter-announcements.png` = `r60-mouse-announcements.png`（sha `d5482858…`，同为抵达战报页，预期合理）。

亲读非登录页截图（read_image）：`r60-768-announcements`（公告页 AI 摘要 + 去看变更影响 CTA）、`r60-768-reactions`（反应图谱 + 机制对照）、`r60-enter-announcements`（我的战报，Enter 导航后）、`r60-mouse-announcements`（同战报页）。

## 9. 诚实边界

- 会话侧 mock：全部探针经 auth-init.js 注入 guest token，未改 DB/服务端；回归使用 user10 真实存档（0% 为其存档差异，非回退）。
- 弃用 driver：首版 detail 探针曾因未定义变量报错，改用 window.innerWidth 重跑；失败输出不采信。本轮 cta eval 合成点击未导航，改用 agent-browser `click <selector>` 坐实鼠标正对照。
- 端口漂移：5174 旧 302 服务全程弃用，仅取 5201。
- git diff --check 全仓 RC=2：.gitignore EOF 空行，属队长资产，QA 未修；业务源码 RC=0。
- 截图 24/25 唯一，1 组预期重复，已披露。
- TestNoRealSecretsInRepo SKIP 非 PASS，不得计为通过。
- CTA 残留键盘缺口（R12 自指 / battles 详情）在 R60 范围外，已披露，不计入本轮 pass 判据。

## 10. 结论

R60 核心验收（6 代表页 × 3 视口 Tab 落 role=button、Enter 激活、focus-visible、cursor pointer、鼠标正对照、R43/R44/R46/R47/R48 言行及 R53/R55/R57 视觉不回退）**通过**。全仓 git diff --check 因 .gitignore EOF 空行 **未全绿（RC=2）**，截图 **非全互异（24/25）**，TestNoRealSecrets **SKIP 非 PASS**，另有 CTA 键盘残留缺口披露。QA 未改业务源码、未 git 写。
