# R66 宝石图鉴缓存 JSON 解析兜底 · 真浏览器 QA 报告

- 任务：t130 / attempt=07404edb-c14e-4364-9650-d80db4db5737
- 执行：same-route-qa（真浏览器复查）
- 工作树：`.agents/worktrees/dsh-d0bc4721-3248`，HEAD=`4302790d5`
- 被测修复：R66 对宝石图鉴 `lyp:r10:gem-gain:*` 缓存 JSON 解析加 try/catch 兜底，解析失败/空缓存统一走 R10 诚实空态，不再抛 `SyntaxError` 冒泡或白屏
- 写域：仅本文档与同目录 `data/`、`screenshots/`；业务源码/契约/规范/.gitignore 未改

## 一、被测代码（QA 只读核实）

- `laiyipao/miniapp/src/pages/gems/gems.vue:190` `const gemGain = ref<GemGainView>(normalizeGemGain(null))`；:191-203 `loadGemGain()`：:194 `try {` → :195 前缀 `r10GemGainCacheKey(store.userId, 0).replace(/:0$/, ":")` → :196-199 遍历 `localStorage` 键取首个匹配值 → :200 `parsed = raw ? JSON.parse(raw) : null` → :201 `} catch { parsed = null }` → :202 `gemGain.value = normalizeGemGain(parsed)`；:204 setup 调用，:205 `onShow(() => loadGemGain())`。
- 模板：:48 `class="r10-gem-gain r10-enter"`；:49 标题「AI · 宝石佩戴收益（服务端属性差分）」；:50 `v-if="gemGain.available"` 渲染 7 个差分行；:57 `v-else class="r10-gem-empty" aria-live="polite" role="status"` 文案「佩戴收益未上报，先看词条描述」；:59 `r10-basis-note` 依据说明；:69/:70 双 CTA 保留 `role="button" tabindex="0" @keydown.enter`。
- helper `laiyipao/miniapp/src/pages/_shared/r10GemGain.ts`：:18 `normalizeGemGain(data)` 空态 `{available:false,text:"佩戴收益未上报，先看词条描述",rows:[],basis}`；:27-28 读 `before/after.build.attacker`；:31 缺 `attack` 数值走空；:33-34 `ATTACKER_FIELDS`（attack/crit_permille/crit_multiplier_permille/reaction_mult_permille/element_coef_permille/heat_cap_permille/armor_permille）差分；:35-40 有数据时 `text:"服务端属性差分（佩戴前 → 佩戴后）"`；:44-46 `r10GemGainCacheKey(userId, gemId) = "lyp:r10:gem-gain:" + userId + ":" + gemId`。
- 本轮 diff：`git diff --name-only` 仅 `.gitignore` 与 `laiyipao/miniapp/src/pages/gems/gems.vue`；R10 helper/其他页面未触及。

## 二、真浏览器三状态 × 三视口对照实验

Chromium（agent-browser 0.41.1），静态 H5 5201，API 8080。每视口新鲜会话：`--init-script data/auth-init.js`（gitignored `data/`，游客 token 换自本机 API，仅本会话用，报告不回显）`--init-script data/r66-init.js`（收集 `error`/`unhandledrejection`）`--init-script data/state-{broken,valid,empty}.js`（写 localStorage 前置状态），`set viewport W H` → `navigate http://127.0.0.1:5201/#/pages/gems/gems` → sleep 5s → DOM 探针 → 截图。

**状态构造说明（诚实边界）**：A「损坏缓存」与 B「合法缓存」均为 QA 人工在 localStorage 构造，用于确定性触发三分支；`{broken` 这类半截 JSON 可能源于旧版本写入、浏览器扩展改写或半截写入，本实验不声称它在真实用户中自然发生。C 空缓存为删除全部 `lyp:r10:gem-gain:*` 键。因运行时 `store.userId` 未知，state 脚本对 uid 0..20 全写同值，确保命中实际 uid。页面为游客态 `lyp_guest={id:10,guest:true}`（非登录页）。

| 状态 | 视口 | loggedGuest | 卡片/主文案 | rows | errors | inner=doc=client | hOverflow | bodyLen |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| A 损坏 | 390×844 | true | 佩戴收益未上报，先看词条描述 | 0 | [] | 390/390/390 | false | 948 |
| A 损坏 | 768×1024 | true | 佩戴收益未上报，先看词条描述 | 0 | [] | 768/768/768 | false | 948 |
| A 损坏 | 1280×800 | true | 佩戴收益未上报，先看词条描述 | 0 | [] | 1280/1280/1280 | false | 948 |
| B 合法 | 390×844 | true | 服务端属性差分（佩戴前 → 佩戴后） | 7 | [] | 390/390/390 | false | 1158 |
| B 合法 | 768×1024 | true | 服务端属性差分（佩戴前 → 佩戴后） | 7 | [] | 768/768/768 | false | 1158 |
| B 合法 | 1280×800 | true | 服务端属性差分（佩戴前 → 佩戴后） | 7 | [] | 1280/1280/1280 | false | 1158 |
| C 空 | 390×844 | true | 佩戴收益未上报，先看词条描述 | 0 | [] | 390/390/390 | false | 948 |
| C 空 | 768×1024 | true | 佩戴收益未上报，先看词条描述 | 0 | [] | 768/768/768 | false | 948 |
| C 空 | 1280×800 | true | 佩戴收益未上报，先看词条描述 | 0 | [] | 1280/1280/1280 | false | 948 |

A 损坏三视口探针：`emptyText="佩戴收益未上报，先看词条描述"`、`gemText=""`、`rowsCount=0`、`errors=[]`；21 个 `lyp:r10:gem-gain:<uid>:9` 键均为 `"{broken"`，即兜底分支确实被触发，页面无白屏、无 `SyntaxError` 冒泡（`window.__qaErrors` 为空）。C 空缓存与 A 结果一致但 `cacheKeys=[]`，证明空态来自正常无数据路径而非异常吞噬。

B 合法三视口探针：`gemText="服务端属性差分（佩戴前 → 佩戴后）"`、`rowsCount=7`，逐行：`攻击 1000.0 → 1050.0 +50.0`、`暴击率‰ 100.0 → 110.0 +10.0`、其余 5 字段 `1000.0 → 1000.0 +0.0`（暴击倍率/反应倍率/元素系数/热量上限/护甲）。依据文案含「来源：服务端 /me build.attacker 两次权威快照」。**B 的数据是 QA 构造的权威形状快照，非真实佩戴事件产生的服务端真值**；本 QA 只验证解析与展示链路。

探针原始 JSON：`data/probe-{broken,valid,empty}-{390,768,1280}.txt`（9 份）。

## 三、视觉回归与截图

9 张截图（每状态×视口一张）sha256：

| 文件 | sha256 |
| --- | --- |
| `screenshots/r66-broken-390.png` | `645545fb6f52d4bce0082674a7a64edc956ad195d63d0aac518941493500f752` |
| `screenshots/r66-broken-768.png` | `d1cefcd56deca3211bcd1e8f9f1661112d2c87cda48176468146462e0a13b71a` |
| `screenshots/r66-broken-1280.png` | `590fd1133d54cf8eafdef2387cfa1e4d23fae48d12496287b9b45ba31c582e79` |
| `screenshots/r66-empty-390.png` | `0065a3d65787ebf9b8670dcf0c40b3c0fc8ee60b0f66ff144cb8ecacdca37386` |
| `screenshots/r66-empty-768.png` | `d1cefcd56deca3211bcd1e8f9f1661112d2c87cda48176468146462e0a13b71a` |
| `screenshots/r66-empty-1280.png` | `590fd1133d54cf8eafdef2387cfa1e4d23fae48d12496287b9b45ba31c582e79` |
| `screenshots/r66-valid-390.png` | `266c25c7bcb52d2b8b46938972af8d3276d8bd9283dad01bd7e56dd436f0a391` |
| `screenshots/r66-valid-768.png` | `3a05def62fa86350e481f0fccc101d5c97996d4dbbbf7a958669651046c73c11` |
| `screenshots/r66-valid-1280.png` | `f1b4a4d16ac50da2965c5cbfbbca95edadab8a043af24c4a378979851deafd42` |

**必须如实披露**：9 张截图 UNIQUE=7/9。`broken-768` 与 `empty-768` 同 sha（`d1cefcd5…`）、`broken-1280` 与 `empty-1280` 同 sha（`590fd113…`）——A 损坏与 C 空缓存走同一诚实空态、同一页面状态，视觉逐像素一致是预期结果，不是截图缺失。三视口之间、合法缓存与空态之间互异。协议要求「截图 sha256 全量列出与报告逐字一致」，已逐条列出；若把「全部 9 张两两互异」作为硬要求，则此条未满足，但其原因是业务语义等价，非构造失败。

亲读 `r66-broken-768.png`（sha `d1cefcd5…`）与 `r66-valid-768.png`（sha `3a05def6…`）：均为**宝石图鉴非登录页**，顶部「宝石图鉴」、宝石收藏雷达 GEM COVERAGE/AI、当前最该强化·跨系统优先级、装备契合 0/18、0/8 图鉴种类、收集进度、AI 因果链区块正常渲染；无白屏/无错误遮罩。两张上半屏一致（R66 只改佩戴收益卡），下方佩戴收益卡差异由 DOM 探针坐实（截图视口首屏未滚到该卡，故未在图上直接呈现差分行，**不把截图当作差分文案的视觉证据**，文案以探针为准）。

回归静态确认（三视口 DOM 探针 + diff 范围）：三视口 `innerWidth == scrollWidth == clientWidth`、`hOverflow=false`；`--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6` 三锚点逐字在；全 `laiyipao/miniapp/src` 中 `--r66-` 仅契约断言 1 处（`r66GemCacheGuard.contract.test.ts:76` `not.toMatch(/--r66-/)`），业务 0 命中；`placeholder-class="ph"` 与 R55 视觉未触碰；R61 `v-aria-native` 指令与 R60 `role=button/tabindex/@keydown.enter` 未被本轮 diff 改动（diff 仅 gems.vue + .gitignore）；R64 战报竞态修复相关 `battles.vue` 未被触碰。

## 四、门禁（contract order，逐条状态）

| 门禁 | 结果 | 佐证 |
| --- | --- | --- |
| `cd laiyipao/miniapp && pnpm exec vitest run src/styles/r66GemCacheGuard.contract.test.ts --reporter=verbose` | PASS | 4/4 passed，RC=0 |
| `cd laiyipao/miniapp && pnpm exec vitest run --reporter=verbose` | PASS | 219 files passed / 1 skipped，2030 tests passed / 2 skipped，RC=0 |
| `cd laiyipao/miniapp && pnpm exec vue-tsc --noEmit` | PASS | RC=0，无诊断 |
| `cd laiyipao/miniapp && pnpm run build:h5` | PASS | RC=0，Build complete |
| `git diff --check`（工作树根） | PASS | RC=0；仅 `.gitignore` LF→CRLF warning，非错误 |
| Go `TestNoInfraIdentifiersInRepo`（cd laiyipao/server） | PASS | `ok github.com/laiyipao/server/internal/domain 21.366s`，INFRA_RC=0 |
| Go `TestNoRealSecretsInRepo` | **SKIP 非 PASS** | 逐字：「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」；SECRET_RC=0 仅表示 go test 进程正常退出，不代表守卫执行 |

门禁原始输出：`data/g-*.txt`、`data/go-*.txt`。

## 五、清理与证据 hygiene

- 鉴权：`data/auth-init.js`（517B）由 `gen-alias-auth-init.sh` 从本机 API 游客换 token 生成，落 gitignored `data/`（`.gitignore:568` 命中），仅用于本次浏览器会话；报告未回显任何 token 值。清理阶段按 `ops_stat` 取 revision 后 `ops_remove` 删除，复核 `AUTH_AFTER=ABSENT`（tracked 工作树无鉴权文件）。
- QA 库：`lyp-qa-pg` `battle_records` count|max = `104|477`（未变）。
- 自启后台 job：API `bash-78ae32133159417e`、H5 构建 `bash-3c95d2741b1947cf`、静态 5201 `bash-b21bba17fa194e5a`、walk `bash-b75bf849c00d4d15`、gates/static/go 各 job，收尾全部 `job_kill`。

## 六、结论

**PASS**（按本任务验收口径）：A 损坏缓存三视口全部落到诚实空态，`rows=0`、`errors=[]`、无白屏/SyntaxError；B 合法缓存三视口全部渲染 7 行权威属性差分；C 空缓存回到诚实空态。R10/R61/R64/令牌/R55 与视觉无回退，契约与全量门禁 RC=0，Go 真值扫描按 SKIP 非 PASS 如实登记。证据链完整（9 DOM 探针 JSON + 9 截图 SHA + 门禁输出）。

诚实边界：①缓存损坏/合法是人工构造，非自然事件；②9 张截图 UNIQUE=7/9，A 损坏与 C 空缓存因语义等价视觉逐像素一致；③合法缓存数据形状为 QA 构造，验证的是解析展示链路，不验证服务端快照真实性；④佩戴收益卡在 768 截图首屏以下，差分文案以 DOM 探针为证据而非截图；⑤Go 真值扫描守卫 SKIP（缺少 `.local.keys`），未执行不等于通过。
