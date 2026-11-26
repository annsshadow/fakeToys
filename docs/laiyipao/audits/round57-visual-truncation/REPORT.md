# R57 真浏览器 QA 报告：公告溢出与占位截断三视口复查

- 任务：t94 / attempt a725ac3b-ae32-455d-83b7-39025f314303
- 工作树：dsh-d0bc4721-3248，HEAD e53610dcc
- 规范：docs/laiyipao/specs/round57-visual-truncation/SPEC.md
- 契约：laiyipao/miniapp/src/styles/r57VisualTruncation.contract.test.ts
- 结论：**PASS（带诚实边界）**。R56 两候选均修复确认，R14/R12/R51/R53/R55/R48 锚点无回归。

---

## 1. 门禁逐条判定

| # | 门禁 | 判定 | 证据 |
|---|------|------|------|
| ① | announcements 768 docW回768/hOverflow=0；.ann-date不越界；日期文本与「最新」徽标仍在；390/1280保持0 | PASS | 768探针 innerW=768/docW=768/hOverflow=0（R56基线795/27）；.ann-date right=610<768，文本「2026-11-24 01:10」；detail探针 .ann-new「最新」display=block/visibility=visible/right=229<768；390/1280 hOverflow均0，rightOver=[] |
| ② | enemies 768占位sw≤cw或可见省略号，trunc归零；对比度≥4.5；390/1280保持；placeholder-class=ph与文案不回退 | PASS | 精确探针768：占位sw466>cw409但 text-overflow:ellipsis + white-space:nowrap + overflow:hidden + min-width:0 全在 → ellipsisOKCount=1、hardTruncCount=0；色rgb(150,160,173)对bg rgb(26,34,51)比值6.0≥4.5；390 cw=sw=342/311、1280 cw=sw=514/484；文案与.ph保留 |
| ③ | 回归R14/R12/R22/R16/R51/R53/R55不回退；令牌三锚点保留、无 --r57- | PASS | 768回归：reactions .r14-advice-empty-cta×7全nowrap；equipment .r12-priority-text + R48自指CTA「在本页强化装备契合 →」在；me .r51-radar viewBox=0 0 200 180/rings=3/axes=5 + honesty「非权威战斗数据；权威以 replay_hash 为准」；index/me无长浮点；四页innerW=docW=768；tokens.css:14-16三锚点在；全仓 --r57- 仅契约断言无业务使用 |
| ④ | 契约4/4+miniapp全量+type-check+build:h5+git diff --check全绿；Go TestNoInfra PASS、TestNoRealSecrets SKIP非PASS | PASS（SKIP如实标注） | 契约4 tests passed RC=0；全量216 files passed（1 skipped）、2014 tests passed（2 skipped）RC=0；type-check RC=0；build:h5 DONE RC=0；git diff --check RC=0（仅.gitignore LF→CRLF warning）；Go TestNoInfraIdentifiersInRepo PASS(5.07s)、TestNoRealSecretsInRepo SKIP非PASS（原文见§5） |
| ⑤ | 截图sha互异、亲读非登录；诚实标注mock会话侧/弃用driver/端口漂移错误页 | PASS | 6张截图sha全互异且均已亲读（公告页/敌人图鉴页，非登录、非chrome-error）；详见§3、§5 |
| ⑥ | finalize删auth-init.js、QA库回104/477、data/不入库 | PASS | 见§6；git check-ignore命中.gitignore:530；QA库count/max=104/477 |

---

## 2. 三视口走查结果表（主探针 + 精确探针）

| 视口 | 页 | docW-innerW | hardTrunc | 低对比度 | 说明 |
|------|----|-------------|-----------|----------|------|
| 390x844 | announcements | 0 | 0 | 0 | 10条日期在，「最新」徽标在 |
| 390x844 | enemies | 0 | 0 | 0 | 占位 cw=sw=342 |
| 768x1024 | announcements | 0（基线27） | 0 | 0 | .ann-date right=610，.ann-title min-width:0/ellipsis/nowrap |
| 768x1024 | enemies | 0 | 0（ellipsisOK=1） | 0 | 占位省略号守卫生效，视觉见尾部省略号 |
| 1280x800 | announcements | 0 | 0 | 0 | 占位 cw=sw=514 |
| 1280x800 | enemies | 0 | 0 | 0 | 占位 cw=sw=484 |

### 2.1 候选1（公告768溢出）修复确认
- 实施diff（QA只读）：announcements.vue 的 .ann-title 增 min-width:0/overflow:hidden/text-overflow:ellipsis/white-space:nowrap；.ann-date 的 flex-shrink:0 与 formatDate(a.published_at) 绑定逐字保留（SPEC §1.3 方案A）。
- 768主探针：docW=768/hOverflow=0、rightOver=[]；detail探针 head 三子项 badge(66px)/title(151px,sw506→ellipsis)/date(197px,right610) 全部≤768。长标题经 ellipsis 收缩属预期，非R56文档级溢出。

### 2.2 候选2（敌人图鉴768占位截断）修复确认
- 实施diff（QA只读）：common.css 的 .ph 增 min-width:0/overflow:hidden/text-overflow:ellipsis/white-space:nowrap；enemies.vue 未改，placeholder-class=ph 与占位文案逐字保留；.ph 色仍 var(--dim)。
- 768精确探针：sw466>cw409 但四守卫齐全 → ellipsisOK、hardTrunc=0；截图目视占位显示「搜索敌人名称 / 类别（如 Boss、…」尾部省略号，符合 SPEC §4「sw≤cw 或可见省略号，trunc归零」。

---

## 3. 截图 SHA-256 表（6张互异，均已亲读非登录页）

| 截图 | 视口 | 字节 | sha256 |
|------|------|------|--------|
| r57-390-announcements-announcements.png | 390x844 | 111909 | 6ab29fc32066bc9bf24d89331da6513646ad7db00a10aff955d316353b7fb10c |
| r57-390-enemies-enemies.png | 390x844 | 93802 | 7f67bdba3b801d0ec17d8c2be670ad89742c03f82254d5cb3f2eb428ce944aca |
| r57-768-announcements-announcements.png | 768x1024 | 251362 | 62bcfb33c05ceec984b79effcb06272299102c822d06665d6e5ec9ca03a996c4 |
| r57-768-enemies-enemies.png | 768x1024 | 241394 | 035f2b863c60b025f44acbcdc960b0f38078715e9e54f132e48a0f28767ac1cc |
| r57-1280-announcements-announcements.png | 1280x800 | — | 2a5315d21fc2ed29992634d7b22394ebec7d0b8022db2522157f96aad5ee07ec |
| r57-1280-enemies-enemies.png | 1280x800 | — | 5ea020f1361d5c7c1b24cd6bdfa83b397fe2643b78984380f619c6eb67667b84 |

（sha 由 sha256sum 实算，6值互异。）

---

## 4. 命令与结果

| 命令 | 结果 |
|------|------|
| vitest run src/styles/r57VisualTruncation.contract.test.ts | 4 tests passed，RC=0 |
| pnpm test（miniapp全量） | 216 files passed / 1 skipped，2014 tests passed / 2 skipped，RC=0 |
| pnpm run type-check | RC=0 |
| pnpm run build:h5 | DONE Build complete，RC=0 |
| git diff --check | RC=0（仅.gitignore LF→CRLF warning） |
| go test ./internal/domain -run TestNoInfra..\|TestNoRealSecrets -v | TestNoInfra PASS(5.07s)；TestNoRealSecrets SKIP；GO_RC=0 |
| git check-ignore -v .../data/auth-init.js | 命中 .gitignore:530，RC=0 |
| docker exec ... select count(*),max(id) from battle_records | 104/477 |

---

## 5. 诚实边界（逐条披露）

1. TestNoRealSecretsInRepo 为 SKIP 非 PASS：原文「读不到 WORKTREE_ROOT/laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是「通过了」」。Go整体RC=0不代表该守卫通过。
2. 浏览器注入为会话侧：auth-init.js 由QA库 users.id=10 的 guest_token 经 POST /api/v1/auth/guest 换token写localStorage，仅作用于agent-browser会话；未改DB/服务端。R56基线JSON仅作test-first RED与前后对比，本轮探针为修复后实采。
3. 首版detail探针已弃用：初版用未定义变量 innerW 报 ReferenceError: innerW is not defined（两份 detail-err.txt 各108B），已改 window.innerWidth 重跑成功；采信重跑后detail JSON。
4. 主探针truncCount=1属假阳性、精确探针坐实：通用trunc检测器把带ellipsis守卫的占位溢出也计入，768 enemies报truncCount=1。按SPEC §4「可见省略号即可」另写精确探针区分hardTrunc（无守卫）与ellipsisOK（守卫齐全），三视口复核：hardTrunc全0、768 enemies ellipsisOK=1。768公告标题sw506>cw151同属ellipsis守卫（预期省略，非缺陷）。
5. 端口与错误页：只用API 8080（/api/v1/config=200，version=6237e0b623219b8b）与静态5201（根200）；端口5174旧302服务未用；所有探针href均为对应deep link，无chrome-error/BAD_JSON/eval错误页。
6. 回归基于真实存档：回归探针未注入mock，me/index用user10真实存档（装备契合0%），结论显示0%而非R53 mock案例的44%——锚点（整数百分比、雷达、R12/R14/R48 CTA、无长浮点）均在，属数据差异非功能回退。
7. 服务与构建：build:h5在仓库根pnpm报Missing script，须在 laiyipao/miniapp 下执行（RC=0）；已重建并静态托管于5201。

---

## 6. 收尾

- data/auth-init.js 删除并验证ABSENT。
- QA库 battle_records count/max=104/477，与基线一致。
- docs/laiyipao/audits/round57-visual-truncation/data/ 已被 .gitignore:530 忽略，不入库。
- 自启后台job（API c2e82e5c21ff4ed1、H5 6c7307a5972f4d7c、walk 28a220982ece4901、gate 1093e4bd956c4b7c、refine f00fa1e78c16451a、reg 359b44d456794872）全部收集或终止。