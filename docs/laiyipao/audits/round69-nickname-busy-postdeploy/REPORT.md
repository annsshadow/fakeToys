# R69 部署后真浏览器复核报告：昵称提交在途反馈与防连点

- 任务：t142 / attempt 1af2b96a-c4ee-406b-8ca0-a6be777f39a6
- 工作树：dsh-d0bc4721-3248，起点 HEAD=501fa87335b6b9afa5f81e6d2d67bd18fcd83efc（R66 末）
- 方式：重建 H5 后真 Chromium（agent-browser），三视口 414×896 / 820×1180 / 1440×900，三条件慢网/连点/失败态
- 判定：**PASS**（带诚实边界）

## 一、五批提交链（精确 git add 逐路径，禁 -A/点号）

| 批次 | commit sha | 内容 |
| --- | --- | --- |
| B1 feat | 6350782f4 | me.vue +7/-1：busy ref/入口守卫/finally/按钮文案/disabled |
| B2 test | 62f8464e1 | r69NicknameBusy.contract.test.ts 5it |
| B3 docs | e28ae0b7f | SPEC.md + ROUND69_DESIGN_REVIEW.md |
| B4 test audit | 51fa71fdd | .gitignore +6行 + REPORT.md + 审计脚本 + 9张QA截图（24文件371+） |
| B5 postdeploy | 本报告提交时生成 | REPORT.md + pd-probe.js + pd-driver.sh + 9张部署后截图 |

每批 LEAK_OK / DATA_IN_STAGE=0 / CHECK_RC=0，warning 仅 .gitignore LF→CRLF 既有提示。

## 二、产物 sha 核验（重建后逐字实测，与 QA/终审一致）

| 产物 | 文件 | sha256 |
| --- | --- | --- |
| H5 主 bundle | index-2A8Oxcaj.js | cef41aa95fff702a90ef6ab915d51b01f0ac365ff7fdc2712639d5b585f638bd |
| 后端 | lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 |
| admin | WikiView-ST1PbAt2.js | f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b |

在 laiyipao/miniapp 下 pnpm run build:h5 RC=0，index.html 引用 assets/index-2A8Oxcaj.js；后端与 admin 沿用长期基线未回退。服务：API :8080（go run ./cmd/api）与 H5 静态 :5201 均 200。

## 三、部署后真浏览器复核（三视口 × 三条件）

注入 guest token 鉴权（仅会话侧，不改 DB）+ QA 的 r69-init.js（拦截 PUT /me/nickname 计数、慢网延迟 3000ms）与 r69-err-init.js（收集 error/unhandledrejection）；用 r69-open-edit.js/r69-fill.js/r69-click.js 真实点击。慢网取点击后 1 秒探针，连点取两次点击后探针，失败态用 network route abort。

| 视口 | 模式 | editing | inputDisabled | saveLabels | btnDisabled | putCount | errTexts | hOverflow |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 414 | 慢网1s | true | true | [提交中…] | 1 | 1 | [] | false |
| 820 | 慢网1s | true | true | [提交中…] | 1 | 1 | [] | false |
| 1440 | 慢网1s | true | true | [提交中…] | 1 | 1 | [] | false |
| 414 | 连点 | false（提交完成） | null | [] | 0 | 1 | [] | false |
| 820 | 连点 | false（提交完成） | null | [] | 0 | 1 | [] | false |
| 1440 | 连点 | false（提交完成） | null | [] | 0 | 1 | [] | false |
| 414 | 失败 | true | false | [保存] | 0 | 1 | [无法连接服务器：request:fail] | false |
| 820 | 失败 | true | false | [保存] | 0 | 1 | [无法连接服务器：request:fail] | false |
| 1440 | 失败 | true | false | [保存] | 0 | 1 | [无法连接服务器：request:fail] | false |

三条件全符合：慢网在途可见「提交中…」且按钮/输入框禁用、PUT=1；连点两次 PUT 恒为 1（入口守卫生效）；失败态编辑态保留、错误可见、按钮复位可重试（finally 生效）。九组 qaErrors/qaRejections 全空，innerW==docW、hOverflow=false。

## 四、部署后截图 sha256（9 张，视口 414/820/1440 与 QA 的 390/768/1280 互异；全 ≠ QA 九图）

| 文件 | sha256 |
| --- | --- |
| pd-slow-414.png | 3cc09f937bfbc19c00b0fa4c6c6c838abafc2fc4607068258ee692e022c70230 |
| pd-slow-820.png | 9a4e666774f42a5fab56d14e3ad5f5ed175b16f40fe880c8eff7c7c182e78a2b |
| pd-slow-1440.png | 5d7e442029247fe1b340b576b5c070b89499e6e63d4b133531f9f58f771dc79b |
| pd-double-414.png | 8e373ceb1970bc8f5aead8be041b1b72c79d1ae1b9be86f62067d990fc4db5ba |
| pd-double-820.png | de3de35951e242ee737b52567b0c0179a40619df67943844f260898982d970e0 |
| pd-double-1440.png | e5db4b1227a071d8d66f23b6d02f5d03dd84a773c6f27722081166ce45999ee7 |
| pd-fail-414.png | 960840008060fd4bf91dacd9b39ffb5275d001cad03f82d8e848043a9eb70838 |
| pd-fail-820.png | 04e0e14a01a41a35c8c821994c432ebc9291937fb8c66fa5b037aff704b819c3 |
| pd-fail-1440.png | dbc01a3cfd808ec9f006bbb9a4616d4d86e03c7d8cc6ca3e98e462f6fdf76091 |

UNIQUE=9/9。亲读 pd-slow-414.png（慢网「提交中…」+输入框值 QAprobeR69 的指挥官档案）、pd-double-820.png（提交完成后的指挥官档案）、pd-fail-1440.png（失败态「无法连接服务器：request:fail」+保存按钮复位）三张，均非登录页，165 分/第 60 关/AI 雷达图正常。

## 五、回归锚点（不回退）

- R61 无障碍：me.vue v-aria-native / aria-label「新昵称（≤64 字）」保持，编译 DOM 原生 input 带 aria-label。
- R60 键盘：role=button=48 / tabindex=48 / @keydown.enter=48 保持。
- submitPassword 正面对照范式（passwordSubmitting 守卫 + :class/:disabled/「提交中…」）未动。
- 令牌三锚点 tokens.css:14-16 逐字在（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6）；全 src --r69- 仅契约断言 1 处，业务 0 命中。
- R64 detailPanelSeq 守卫、R66 缓存兜底产物未回退。

## 六、门禁与收尾

- QA 门禁（终审独立复跑）：R69 契约 5/5；全量 vitest 221 files passed 1 skipped / 2039 tests passed 2 skipped；vue-tsc RC=0；build:h5 RC=0；git diff --check RC=0（仅 LF→CRLF 既有 warning）。
- 部署后 Go 门禁：TestNoInfraIdentifiersInRepo PASS（21.33s，INFRA_RC=0）；TestNoRealSecretsInRepo **SKIP 非 PASS**（逐字「读不到 .../laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是『通过了』」，SECRETS_RC=0）。
- QA 库 docker lyp-qa-pg：battle_records count|max(id)=104|477，只读未写。auth-init.js 仅会话侧 guest token 注入，两处 data/auth-init.js 于 finalize 删除；data/ 全目录被 .gitignore:574/:576 忽略，不入库。

## 七、诚实边界

1. 失败态用 network route abort 模拟「无法连接服务器」，未覆盖后端 4xx/5xx 业务错误响应分支（QA 已披露，范围外）。
2. 三张截图亲读，其余六张以 sha256 亲算与探针 DOM 佐证；九张互异且全 ≠ QA 基线九图。
3. Go 真值扫描因缺 .local.keys 为 SKIP，不冒充 PASS。
4. 本报告的 B5 自身 sha 无法自引用哈希，提交后由 HEAD 固定，如实标注。
5. guest token 仅会话侧注入，不改 QA 库真实昵称（探针值 QAprobeR69 为会话输入）。
6. build:h5 期间既存 build.attacker console 告警与本轮无关（QA 已登记）。

## 八、结论

重建 H5 后三视口三条件真浏览器复验全部通过：慢网在途反馈、防连点单 PUT、失败态复位可重试；产物 sha 与 QA/终审一致，零真值入库，回归锚点零回退。判定 PASS 可发布。
