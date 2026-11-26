# R69 昵称提交在途反馈与防连点 · 真浏览器 QA 报告

- 任务：t140 / attempt a4b10951-fba8-42f0-9802-d88694dfeda1
- 工作树：dsh-d0bc4721-3248，HEAD=501fa8733
- 方式：真 Chromium（agent-browser 0.41.1），三视口 390×844 / 768×1024 / 1280×800
- 业务源码：只读；本 QA 唯一写域为本审计目录（含 data/，已在 .gitignore:574 忽略）
- 判定：**PASS**

## 一、验证目标与口径

验证 me.vue 昵称提交按 R69 规范采用 submitPassword 范式后，三条件在真浏览器生效：
1. 慢网（PUT 延迟 3000ms）点击保存后 1 秒内：按钮显示「提交中…」+ btn-disabled class + 输入框 disabled=true + 无可点第二保存入口；
2. 正常网速连点 2 次保存：PUT /me/nickname 计数 = 1（防连点）；
3. route abort 失败态：错误文案可见 + 编辑态保留 + nicknameSubmitting 复位、按钮可重试。

构造手段：init-script 注入 window.__qaNet 拦截 XMLHttpRequest，捕获 PUT /me/nickname 的 {t, body}；slow=true 延迟 send 3000ms；失败态用 agent-browser `network route "**/me/nickname" --abort` 真 abort。填值用 HTMLInputElement.prototype.value setter + dispatch input，规避 uni-app 受控输入。

## 二、三条件 × 三视口走查结果

每视口独立会话，探针脚本 b-{mode}-{V}-probe*.txt（慢网另取 -1s 与 -after 两态），截图 r69b-{mode}-{V}.png。

条件① 慢网在途（点击后 1 秒）——三视口全 PASS：

| 视口 | editing | inputDisabled | inputValue | saveLabels | btnDisabled | putCount | putBody | hOverflow |
|---|---|---|---|---|---|---|---|---|
| 390 | true | true | QAprobeR69 | [提交中…] | 1 | 1 | {"nickname":"QAprobeR69"} | false |
| 768 | true | true | QAprobeR69 | [提交中…] | 1 | 1 | {"nickname":"QAprobeR69"} | false |
| 1280 | true | true | QAprobeR69 | [提交中…] | 1 | 1 | {"nickname":"QAprobeR69"} | false |

1 秒内按钮文案为「提交中…」且带 btn-disabled、输入框原生 disabled=true、无第二保存入口（saveBtns 仅 1 项），PUT 已发出 1 次。after 探针（3 秒后、请求完成）三视口 editing=false、saveBtns=[]、putCount 仍为 1，证实在途态随提交完成复位、无重复提交。

条件② 正常网速连点 2 次——三视口全 PASS：

| 视口 | putCount | putBody | 最终态 | hOverflow |
|---|---|---|---|---|
| 390 | 1 | {"nickname":"QAprobeR69"} | editing=false（提交完成） | false |
| 768 | 1 | {"nickname":"QAprobeR69"} | editing=false（提交完成） | false |
| 1280 | 1 | {"nickname":"QAprobeR69"} | editing=false（提交完成） | false |

连续两次点击保存，PUT /me/nickname 计数恒为 1，if (nicknameSubmitting.value) return 入口守卫生效。

条件③ route abort 失败态——三视口全 PASS：

| 视口 | editing | inputDisabled | saveLabels | errTexts | errVisible | putCount | hOverflow |
|---|---|---|---|---|---|---|---|
| 390 | true | false | [保存] | [无法连接服务器：request:fail] | true | 1 | false |
| 768 | true | false | [保存] | [无法连接服务器：request:fail] | true | 1 | false |
| 1280 | true | false | [保存] | [无法连接服务器：request:fail] | true | 1 | false |

network route abort 后错误文案「无法连接服务器：request:fail」可见，编辑态保留（editing=true、输入框值 QAprobeR69 未丢），nicknameSubmitting 经 finally 复位（inputDisabled=false、按钮文案回到「保存」、btnDisabledCount=0），可重试。注：此处 putCount=1 为被 abort 的那次尝试，__qaNet 在 send 阶段即记录，属预期。

## 三、错误与异常收集

另跑 err-driver.sh（三视口×三条件共 9 会话，init-script 注入 window.__qaErr 收集 error/unhandledrejection，并由 agent-browser errors / console 交叉验证）：
- 9 组 __qaErr：errors=[]、rejections=[]（全空）；
- 9 组 agent-browser errors：全空（无 page error）；
- 9 组 console：仅一条既存告警「[来一炮] build.attacker 缺失，正在使用本地占位属性……I-6 验真会误判为伪造」，属本轮无关的既存启动日志，非昵称提交引入的 error/异常。

## 四、回归（不回退）

- R61：me.vue:88 v-aria-native 新昵称标签、:89 aria-label、:93 placeholder-class=ph 均在；编译 DOM 原生 input 带 aria-label="新昵称（≤64 字）"。
- submitPassword 范式：me.vue:580 / :591-592 / :596-599 / :1124 / :1136-1160 完整未动。
- R60 键盘：role=button=48、tabindex="0"=48、@keydown.enter=48。
- 令牌三锚点：tokens.css:14-16 --lyp-token-version:lyp-tokens-v1 / --lyp-r3-version:lyp-tokens-v3-r3 / --lyp-r6-version:lyp-tokens-r6 逐字在；全 src --r69- 仅契约断言 1 处、业务 0 命中。
- 视觉：9 组探针 innerWidth==scrollWidth、hOverflow=false；亲读 r69b-slow-768.png 与 r69b-fail-768.png 为「指挥官档案 / COMMANDER PROFILE」非登录页，昵称已显示 QAprobeR69、165 分、第 60 关、AI 养成诊断雷达图正常。
- 写域：git diff --name-only 仅 .gitignore + laiyipao/miniapp/src/pages/me/me.vue。

## 五、门禁（全部 RC=0）

| 门禁 | 结果 |
|---|---|
| 契约 r69NicknameBusy --reporter=verbose | 5/5 passed |
| 全量 vitest --reporter=verbose | 221 files passed, 1 skipped；2039 tests passed, 2 skipped |
| vue-tsc --noEmit | RC=0 |
| build:h5 | RC=0，Build complete |
| git diff --check | RC=0（仅 .gitignore LF→CRLF warning） |
| Go TestNoInfraIdentifiersInRepo | PASS（8.22s, cached） |
| Go TestNoRealSecretsInRepo | **SKIP 非 PASS**：读不到 .../laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」 |

## 六、截图 sha256（9 张，实际计算，两两互异）

| 文件 | sha256 |
|---|---|
| r69b-slow-390.png | 592fe136fc18f76d7a2cd5d9461d7bef02bfea36185c1bcff271df5ac1c16b1d |
| r69b-slow-768.png | dc623b7a0654b671ff24d14edf5b1662b00a3f7916c588cc6d2e8646051a59aa |
| r69b-slow-1280.png | d483acd255965e45b3a485434ace2da6d6226bb00a9f12f81714495caf4db14a |
| r69b-double-390.png | 000d14cb9d1ae9bfa51899fcaf4c65af3ea923296e6070b817785ee86e2dfe1d |
| r69b-double-768.png | d04106b0342417e85aa442b2b2fd9459a96b934943e4259e108fec59a1e7ee4a |
| r69b-double-1280.png | d136ec4960a98d48093d68a205dcbca1b07f39fd59f9da599ff052f0530288c9 |
| r69b-fail-390.png | 8b45c3ea5b24e94c73750a097b90f83a2723c1773881d4d726fd465f3ab9e56c |
| r69b-fail-768.png | 622499532e5eeb0ec03ffaf898bc2198a11095d5deeb3541c2ceb5556ed1d5a0 |
| r69b-fail-1280.png | 0bd57ab3dae09c87f0c730a9e0c16646fe1822a0a2f6bc08de2ed25232fac458 |

UNIQUE = 9/9。

## 七、诚实边界

- 失败态用 network route abort 模拟「无法连接服务器」，覆盖 catch 文案与复位路径；未覆盖后端返回 4xx/5xx 业务错误的文案分支（本轮范围外）。
- 截图亲读 2 张（768 慢网/失败态）；390/1280 仅 sha 校验与 DOM 探针，未逐张目视。
- 既有 console 告警 build.attacker 缺失与本轮昵称提交无关，如实登记、未算作回归缺陷。
- Go 真值扫描守卫因缺 .local.keys 为 SKIP，不冒充 PASS。
- auth-init.js 由 gitignored data/ 目录凭据生成，审计完成即删除，报告不回显令牌值。

## 八、结论

三条件在三视口真浏览器全部生效，PUT 防连点计数正确，失败态复位可重试，无 page error/异常，回归与门禁全绿（真值扫描 SKIP 如实标注）。判定 **PASS**。
