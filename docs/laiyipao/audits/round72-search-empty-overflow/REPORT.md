# R72 搜索空态极端值溢出修复 — 真浏览器 QA 复查报告

- 任务：t154 / attempt 768f36c0-3bf7-4e75-bf7c-079a995fddf7（verification 口径）
- 执行者：same-route-qa｜工作树 HEAD：ea82592a6
- 修复基线：common.css 新增 `.search-empty-text{display:block;max-width:100%;word-break:break-all;}`；7 页空态回显 `<text>` 追加该类（skills/skins/announcements/enemies/equipment/gems/reactions）。
- 侦察对照：`docs/laiyipao/recon/ROUND72_B_EXTREME_CONTENT_RECON.md`（基线 HEAD=501fa8733，6 组合全 hOverflow=true，sw 975/1920/1390）。
- 结论：**PASS**。修复后三视口极端串注入全部 `hOverflow=false`，侦察基线溢出消失。

## 一、环境与方法

- API：8080（`go run ./cmd/api`，回环占位口令，QA 库只读）；静态 H5：5201（`python -m http.server --directory laiyipao/miniapp/dist/build/h5`）；`build:h5` RC0。
- 浏览器：agent-browser 0.41.1 命名会话，真 Chromium，三视口 390×844 / 768×1024 / 1280×800（`open` 空页 + `navigate` 模式，避免带 URL 双 init-script 挂起）。
- 注入串（会话侧合成、非真值）：`「没有匹配-A」×20 + 😀🔥 + <img src=x onerror=alert(1)> + &"引号 + Z×200`，len=356；用 `HTMLInputElement.prototype.value` 原生 setter + `dispatchEvent(input/change)` 写入搜索框。
- auth：data/auth-init.js（517B，gitignored），报告与探针不含任何令牌真值。

## 二、必做① 三视口极端串注入（skills / skins 主测，gems / announcements 抽测）

| 页面 | 视口 | cw | sw(修复后) | 侦察基线sw | hOverflow | 空态文本宽(cw/sw) | display | word-break | max-width |
|---|---|---|---|---|---|---|---|---|---|
| skills | 390×844 | 390 | 390 | 975 | false | 334/334 | block | break-all | 100% |
| skills | 768×1024 | 768 | 768 | 1920 | false | 452/452 | block | break-all | 100% |
| skills | 1280×800 | 1280 | 1280 | 1390 | false | 506/506 | block | break-all | 100% |
| skins | 390×844 | 390 | 390 | 975 | false | 334/334 | block | break-all | 100% |
| skins | 768×1024 | 768 | 768 | 1920 | false | 452/452 | block | break-all | 100% |
| skins | 1280×800 | 1280 | 1280 | 1390 | false | 506/506 | block | break-all | 100% |
| gems | 390×844 | 390 | 390 | — | false | 334/334 | block | break-all | 100% |
| gems | 768×1024 | 768 | 768 | — | false | 452/452 | block | break-all | 100% |
| gems | 1280×800 | 1280 | 1280 | — | false | 506/506 | block | break-all | 100% |
| announcements | 390×844 | 390 | 390 | — | false | 365/365 | block | break-all | 100% |
| announcements | 768×1024 | 768 | 768 | — | false | 511/511 | block | break-all | 100% |
| announcements | 1280×800 | 1280 | 1280 | — | false | 536/536 | block | break-all | 100% |

- 12 组合全部 `sw === cw`、`hOverflow=false`；侦察基线 skills/skins 六组合的 sw 975/1920/1390 全部消失，证明 `.search-empty-text` 宽度约束生效。
- 探针落盘：`data/probe-{skills_skills,skins_skins,gems_gems}-{390,768,1280}.txt`、《通用探针》`data/probe2-ann-{390,768,1280}.txt`。

## 三、必做② 回显完整 / XSS / 清除搜索

- 关键词回显完整：`searchEmpty[0].t` 含完整 356 字符注入串（含 `Z×200` 与 `<img …>` 字面文本），文本 `len=370~371`，`word-break:break-all` 换行不截断；`textCw===textSw` 无内部横向溢出。
- XSS：12 组合 `badTokens=[]`（NaN/Infinity/undefined/[object Object] 均无）；`<img src=x onerror=alert(1)>` 被 Vue 插值转义为字面文本，无脚本执行、无页面错误。
- 清除搜索：skills/skins/gems 三视口 `clearBtns=["清除搜索"]`；announcements `allClear=["清除搜索"]`。点击后探针 `searchEmpty=[] / emptyTextNodes=[] / allClear=[]`、`hOverflow=false`，空态消失、CTA 可点可复位。

## 四、必做③ 正常态回归

- 清空搜索后回到有结果列表（bodyLen 1887→3071），三视口无横向溢出、无 console/page error，表单与列表正常渲染。
- 侦察基线 `causal-base-390/768 hOverflow=false` 与修复后一致。

## 五、必做④ 回归锚点（静态）

- 令牌三锚点：`tokens.css:14 --lyp-token-version: lyp-tokens-v1`、`15 --lyp-r3-version: lyp-tokens-v3-r3`、`16 --lyp-r6-version: lyp-tokens-r6` 逐字在。
- R60 键盘：`role="button"`=48、`tabindex="0"`=48、`@keydown.enter`=48（OPS grep 全量复核）。
- R55 `placeholder-class="ph"`=17；R61 v-aria-native / 新昵称标签在；R71 `fmtVerifyTime(replayInfo.created_at)`、`pretty(replayInfo.expected_hash)`、`buildSnapshotCard` 在；R66 `normalizeGemGain`/诚实空态在；R69 `nicknameSubmitting`/`提交中…` 在。
- 业务源码零 `--r72-`（唯一命中为契约断言 `r72SearchEmpty.contract.test.ts:100`）。
- `git diff --name-only HEAD`：`.gitignore` + 7 个 .vue 页 + `common.css`，与 t153 写域一致，未触及 protected 域。

## 六、必做⑤ 门禁

- G1 契约 `r72SearchEmpty.contract.test.ts`：**5/5 passed**（Test Files 1 passed / Tests 5 passed）。
- G2 全量 vitest：**223 files passed | 1 skipped (224)；2049 tests passed | 2 skipped (2051)**。
- G3 `vue-tsc --noEmit` RC0；G4 `build:h5` RC0（Compiler 5.24 vue3）；G5 `git diff --check` RC0（仅 `.gitignore` LF→CRLF warning，非错误）。
- Go（在 laiyipao/server 下）：`TestNoInfraIdentifiersInRepo` **PASS**（36.67s）；`TestNoRealSecretsInRepo` **SKIP 非 PASS**，逐字：「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」。
- 全部门禁脚本 `gates.sh` 整体 exit code 0。

## 七、必做⑥ auth / 零真值 / QA库

- `data/auth-init.js` 已生成于 gitignored data/（.gitignore:586），结算前删除，复核 ABSENT；报告与所有探针文件无令牌真值。
- QA 库 `battle_records` = **104 | 477**（`docker exec lyp-qa-pg psql -tAc`，只读复核，未写）。

## 八、截图 sha256（12 张，全互异 UNIQUE=12/12）

| 截图 | sha256 |
|---|---|
| r72-skills_skills-390.png | 5e4ca837eb628e34186bebac342dcd1532b35d1d766342f9d707cc0baa9c9dea |
| r72-skills_skills-768.png | 2c8fd9bfc4daa6b4481c043aa0a5e362fe40a880085f86deef336771a08c30e0 |
| r72-skills_skills-1280.png | 913d7900745ac33df9582613aa58e00c45ee6365125c8e506792c8be1103895b |
| r72-skins_skins-390.png | 4a837414af48a85d9ea834b38726e69117cd3c7c3f402502e5a8f0914565d7b3 |
| r72-skins_skins-768.png | 42175cca9a00c06f7350db6af6ad674dddecad6eb77dbb63036310b2eb5be7ea |
| r72-skins_skins-1280.png | dd131375c2260fa051fd372d6eea2508fa5a2defe995c44f4346d6ab59113ebf |
| r72-announcements_announcements-390.png | 556dc364acbe64e94bef2089613a1f89b88e3c8e125d3108d63794eb827180ab |
| r72-announcements_announcements-768.png | 31fbd5ad884ba16f10e2bf3b9d82e0f32c03975fcc50dc6ca22e5271602914b3 |
| r72-announcements_announcements-1280.png | 41e423ec0ab9ab33ecc8ac7073c1ca5cc9f3c9b31aaa2ddfb50da614e75dc6f1 |
| r72-gems_gems-390.png | ab62aa345c0f1458c3b7b7a4a1e377e364e1bb78378a7d3d52675466f607971e |
| r72-gems_gems-768.png | 61c856ecc9183fdfbd6ad4ac93d8da14b31afe7eef98b1697816f79f9cd686df |
| r72-gems_gems-1280.png | e09af982bc576d8d997e2c791b91e9b6c9f191ce81c751af62c76bf4d5ac76e2 |

- 亲读 2 张：`r72-skills_skills-768.png`、`r72-announcements_announcements-768.png`，均为对应功能页（非登录页），极端串空态可见、无横向滚动条；其余 10 张仅列 sha 未逐张目视。
- 公告页重跑 3 张 `r72-ann2-*.png` sha 与上表 announcements 三视口一致（同一稳定页面状态），不重复计数。

## 九、诚实边界

- 注入串为会话侧 mock 合成，非真实用户输入/服务端数据。
- 真浏览器只覆盖 7 页中的 4 页（skills/skins/gems/announcements）；enemies/equipment/reactions 三页为同构改动，由契约 it2 静态读源码确认 class 已追加，未逐一真浏览器点击。
- 公告页首轮用旧探针（只查 `.search-empty` 容器）得到 emptyN=0，系公告页无该容器、选择器未命中所致，非业务态丢失；已用通用探针 `probe2.js`（直接查 `.search-empty-text`）重跑坐实，首轮数据作废如实披露。
- 静态回归 shell 版 grep 因引号问题曾显示 role_button/tabindex0/ph=0，已用 OPS grep 工具复核为 48/48/48/17，shell 结果作废。
- `screenshots/` 未被 .gitignore 忽略，截图以 untracked 证据文件形式保留（不入库）；`data/` 命中 .gitignore:586。

## 十、结论

- 必做①~⑥ 全部满足：修复后三视口极端串注入 `hOverflow=false`、侦察基线溢出消失；关键词完整可见、XSS 转义、清除搜索复位；正常态无回退；回归锚点齐全；门禁全绿（SKIP 如实标注）；auth/QA库/零真值达标。
- 判定：**PASS**。