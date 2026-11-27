# R97 两处复制失败反馈 · 真浏览器 QA 报告

- 任务：t237（attempt 40958ca3-506e-4b90-8085-5a55c07905d9）
- 对象：承接 t235 规范 RED（3fail|3pass）→ t236 实施 GREEN（契约 6/6，队长复核 diff 精确 me.vue+battles.vue 各 +1/-1，diff-check RC0）
- 验证基线 HEAD：586f77c227642bce4f69385d1f9ec7be64053e06（QA 期间工作树含 t236 未提交改动，属预期）
- 方法：agent-browser 0.41.1 真 Chromium，三视口 390×844 / 768×1024 / 1280×800；两页各注入 success/fail 两态，hook uni.showToast 记录 title+icon 并逐字比对。

## §1 验证目标与判定口径

- me.vue copyUserId 与 battles.vue copyHash 两处补的 fail 回调在真浏览器生效：剪贴板失败出现 toast「复制失败，请手动选择复制」(icon:none)，成功文案不变(「已复制用户 ID」/「已复制回放哈希」)。
- 判定：hook 记录 CLIP:hasSuccess=true:hasFail=true（两回调都在），每态点击后捕获唯一 TOAST 行；iw=cw=sw 零横溢；截图 sha 逐字一致；门禁全绿。

## §2 静态复核（只读源码 + 权威口径）

- laiyipao/miniapp/src/pages/me/me.vue:110-111 复制入口 <text class="link" @click="copyUserId">复制</text>；:714-721 copyUserId() 调 uni.setClipboardData，success→uni.showToast({title:'已复制用户 ID',icon:'success'})，fail→uni.showToast({title:'复制失败，请手动选择复制',icon:'none'})。
- laiyipao/miniapp/src/pages/battles/battles.vue:313 复制入口 <view class="btn copy-hash-btn" @click="copyHash(detail.replay_hash)">复制回放哈希</view>；:896-904 copyHash(hash) 空 hash 提前 return，success→「已复制回放哈希」icon:success，fail→「复制失败，请手动选择复制」icon:none。
- 权威口径对照：laiyipao/admin/src/views/UsersView.vue:462-469 用 try/catch→ElMessage.success(已复制用户 ID) / ElMessage.error('复制失败，请手动选择复制')（:467）。失败文案逐字一致。
- 契约 laiyipao/miniapp/src/styles/r97CopyFailure.contract.test.ts 6 it（readFileSync 真实读源码 + blockAfter 大括号配对解析 setClipboardData 参数）：it1 copyUserId 含 fail / it2 copyHash 含 fail / it3 两处文案含权威口径 / it4 success 文案图标不变量 / it5 令牌三锚点 / it6 业务零 --r97- + share.ts 三函数不动。
- 零真值：laiyipao/miniapp/src/api/share.ts grep setClipboardData|已复制|复制失败 无命中（未改动）；业务源码 --r97- 仅契约断言文件自身命中，页面/样式零命中；令牌三锚点 tokens.css:14/15/16 在。

## §3 真浏览器三视口 · 两态数据表

### 3.1 me 页（非登录态，复制入口自然可见 copyCount=1）

| 视口 | iw/cw/sw | 成功态 hook 记录 | 失败态 hook 记录 | 横溢 |
|---|---|---|---|---|
| 390×844 | 390/390/390 | CLIP:hasSuccess=true:hasFail=true；TOAST:已复制用户 ID\|ICON:success | CLIP:hasSuccess=true:hasFail=true；TOAST:复制失败，请手动选择复制\|ICON:none | 否 |
| 768×1024 | 768/768/768 | 同上（逐字一致） | 同上（逐字一致） | 否 |
| 1280×800 | 1280/1280/1280 | 同上（逐字一致） | 同上（逐字一致） | 否 |

- 三视口探针 bodyLen=621、copyCount=1、hashCount=0（me 页无回放哈希入口，符合预期）。每态点击 text.link「复制」后即刻捕获唯一 TOAST 行。
- 证据：data/w-me-390.log、data/w-me-768.log、data/w-me-1280.log。

### 3.2 battles 页（XHR mock 注入 1 条合成战报 + 详情，展开后复制入口自然可见）

- 空态边界：guest 态无战报数据（bodyLen≈195、copyLinks=[]、hashEls=[]），复制入口在 v-if 详情展开内不可见。为覆盖两态，用 init-script 注册 XHR mock：拦截 /me/battles? 返回 1 条合成战报，拦截 /me/battles/:id 返回合成详情；点开战报行展开后 hashBtns=1，复制入口可见（此为 mock 合成数据，非真值）。

| 视口 | iw/cw/sw | 展开前→后 hashBtns | 成功态 hook 记录 | 失败态 hook 记录 | 横溢 |
|---|---|---|---|---|---|
| 390×844 | 390/390/390 | 0→1 | CLIP:双回调；TOAST:已复制回放哈希\|ICON:success | CLIP:双回调；TOAST:复制失败，请手动选择复制\|ICON:none | 否 |
| 768×1024 | 768/768/768 | 0→1 | 同上（逐字一致） | 同上（逐字一致） | 否 |
| 1280×800 | 1280/1280/1280 | 0→1 | 同上（逐字一致） | 同上（逐字一致） | 否 |

- 证据：data/w-battles-390.log、data/w-battles-768.log、data/w-battles-1280.log；mock 命中 URL 记录 window.__qaHits 见 trial-battles2-out.txt（/me/battles?limit=20&offset=0）。

### 3.3 成功/失败文案逐字对照

| 场景 | 成功 toast（title/icon） | 失败 toast（title/icon） | 权威口径 |
|---|---|---|---|
| me 复制用户 ID | 已复制用户 ID / success | 复制失败，请手动选择复制 / none | UsersView.vue:467 逐字一致 |
| battles 复制回放哈希 | 已复制回放哈希 / success | 复制失败，请手动选择复制 / none | UsersView.vue:467 逐字一致 |

## §4 截图 SHA-256 表与亲读

12 张正式证据（每视口每页成功/失败各 1 张），UNIQUE=12/12：

| 截图 | SHA-256 |
|---|---|
| r97-me-390-success.png | b95801a70aea310833b085200ee4cbeaa986109b5b30dbb45d2f45e2a567c7c4 |
| r97-me-390-fail.png | 4f3d790cc03de890e171cc975defe35dc11aa40dd32d1e03789634c3da8d21ae |
| r97-me-768-success.png | 135506916f389d364900a603e92f260548f3bfacd50fea63f587b46c10312346 |
| r97-me-768-fail.png | e899d7ea8b59990249f461b6c351a0094c90310f5e0d8a21d7099c15b7c7bb20 |
| r97-me-1280-success.png | ff9362eab0f3ef5cc4b4033340d968bb0491ff2193747c00f46d7f9b1aaa6f6a |
| r97-me-1280-fail.png | 727525db7a0a8110ded83162d2d1afca3b35a757a3ee1e82422a0cc75547d591 |
| r97-battles-390-success.png | 572134945933d8d08b71b4f26de5e9ef26b995bd34b1078fc14d9e369be66166 |
| r97-battles-390-fail.png | 954329585a72344b42b37667d757ea0ba6a1a1d86e67bd062c538263975c6ab1 |
| r97-battles-768-success.png | 6dd6e45b6f8397705eb9de73bdf0e178c1c70ee1242be2d4401dda2ecc2eed6d |
| r97-battles-768-fail.png | b0a71e3c92fb5f1a3641d4165438e54ac5480ce83d18a1350cda80bc5c0fa007 |
| r97-battles-1280-success.png | f342bfd2ff3991f1d21e212348566ba5d099a0177bd3169ee6f3a5eb97accc23 |
| r97-battles-1280-fail.png | 06cc6d4d5903d380ac6b498b6b06f4b3b8ccff12451d6939fbf64d3f9227739d |

- 亲读 2 张（均非登录页，且恰好显示 toast）：r97-me-768-fail.png = 指挥官档案页，浮层「复制失败，请手动选择复制」+ 玩家 ID 0 复制入口；r97-battles-768-success.png = 我的战报页，浮层「已复制回放哈希」+ 战绩总账。
- 另有 4 张 trial 过程截图（trial-success-768 / trial-fail-768 / trial-battles-success-768 / trial-battles-fail-768）作额外证据，不列入正式 12 张。
- screenshots/ 未被 .gitignore 忽略，作 untracked 证据保留，不入库。

## §5 门禁与静态回归

| 门禁 | 命令 | 结果 |
|---|---|---|
| G1 R97 契约 | vitest r97CopyFailure.contract.test.ts | 1 passed / 6 tests passed，RC0 |
| G2 定向 | vitest me.test.ts + battles.test.ts | 2 passed / 43 tests passed，RC0 |
| G3 全量 vitest | vitest run | 229 passed \| 1 skipped (230)；2087 passed \| 2 skipped (2089)，RC0 |
| G4 vue-tsc | pnpm run type-check | RC0 |
| G5 build:h5 | pnpm run build:h5 | RC0 |
| G6 diff-check | git diff --check | RC0（仅 .gitignore LF→CRLF warning） |

- 证据：data/gates-out.txt，G1-G6_RC=0，GATES_ALL_DONE。
- 静态：令牌三锚点 tokens.css:14/15/16；失败文案两处 battles.vue:903 / me.vue:720；业务零 --r97-（仅契约断言文件）。
- 本轮无 Go 守卫要求（纯 miniapp 复制回调，不涉及后端真值扫描）。

## §6 诚实边界、Hygiene 与判定

**诚实边界**
- battles 页 guest 态无战报，复制入口不可见；为覆盖两态使用 XHR mock 注入合成战报/详情（非真值），如实披露。me 页为自然空态无 mock。成功/失败态均由覆盖 uni.setClipboardData 直接触发 success/fail 回调模拟，非真实系统剪贴板写入/拒绝；捕获的是回调链产生的 toast 文案，坐实的是「回调接线正确」。
- 首轮 trial 用 about:blank eval 注入 mock 在 navigate 后丢失（rows=0），作废；改用 open --init-script 注册生效。trial3 成功态点击后按钮因冒泡收起详情导致失败态 NO_HASH_BTN，作废；正式走查每态前重新展开坐实。截图仅亲读 2/12。
- CLIP 记录 hasSuccess=true:hasFail=true 系真实源码两回调均注册（改前 recon 基线 hasFail=false），非 mock 置位。

**Hygiene**
- HEAD=586f77c227642bce4f69385d1f9ec7be64053e06 未变；非 ?? 写域仅 .gitignore（队长预置）+ laiyipao/miniapp/src/pages/battles/battles.vue + laiyipao/miniapp/src/pages/me/me.vue（t236 实施，预期）；本轮未碰实施写域。
- auth 四路径 ABSENT：laiyipao/miniapp/.env.local、laiyipao/miniapp/.env.local.keys、laiyipao/admin/.env.local.keys、laiyipao/deploy/.local.keys（另根 .env.local.keys ABSENT）。
- QA 库 lya_qa 只读：users id=477 存在（username 空）、104 无记录；本轮未写库。
- 自启 H5 job bash-81bc67284ffa41be 收尾 kill；零真值入库；未 git 提交。

**判定：PASS**。两处复制失败反馈在真 Chromium 三视口两态全部坐实，成功文案未变，失败文案逐字对齐 admin 权威口径，零横溢，门禁 G1-G6 全绿，hygiene 全绿。
