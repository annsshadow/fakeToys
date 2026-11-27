# R88 部署后真浏览器回归报告 — battles 趋势图图例修正

## 1. 结论

部署后真 Chromium 三视口（414/820/1440）复验 PASS：战绩趋势卡图例逐字为「━ 分数（蓝·越高越好）　┄ 漏怪率（红虚线·越低越好）　● 绿胜 ● 红负」，与蓝实线/红虚线/绿胜点/红负点三编码一一对应，旧「● 分数（蓝）　● 漏怪率（红）　越低越好」折线语义已消除；三视口零横向溢出、零 JS 错误。可发布。

## 2. 五批发布链

- 起点 HEAD = UP = 6a18e7e9b89f7bfa9327239534f125dc47eebb93（R81 收官），AB=0/0
- B1 = 2e489003b779cb6b0e2f7a6fa8f70e2f6c — feat(laiyipao): R88 战绩趋势图图例修正（battles.vue +1/-1，:75 一行）
- B2 = 0325b0740 — test(laiyipao): R88 契约 6it（r88TrendLegend.contract.test.ts 78 行）
- B3 = 71bef7bf1 — docs(laiyipao): R88 规范与终审（SPEC.md 78 行 + ROUND88_DESIGN_REVIEW.md 78 行）
- B4 = e674bc94f — test(laiyipao): R88 QA 审计与截图（.gitignore + REPORT.md 89 行 + 4 脚本 + 3 截图，9 文件）
- B5 = （本报告提交后生成）— chore(laiyipao): R88 部署后回归报告

## 3. 产物 SHA 与不回退基线

- H5 主 bundle = index-Dt9iRg7g.js（index.html:assets 引用），sha256 = aae0c7fe8525eb2a597e80936c9c2f43213ebf20f6c0d085cf0d8933e2a8bd3c（≠ R81 index-iiibg12t.js）
- 后端 lyp-server-amd64 sha256 = 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（沿用不回退）
- admin WikiView-ST1PbAt2.js sha256 = f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b（沿用不回退，实测唯一 WikiView js）
- QA 库容器 lyp-qa-pg battle_records 只读核验：104|477

## 4. 部署后三视口 DOM 探针

注入 guest token（仅 lyp_token/refresh/token_exp/guest/profile）与 6 条 mock 战报（4 胜 2 负），真浏览器 navigate battles 页，三视口结果完全一致：

| 视口 | legendText | trendExists | svgAria | 蓝实线 stroke | 红虚线 stroke/dash | 绿胜 fill/数 | 红负 fill/数 | innerW=docW=clientW | overflowPx | hOverflow | errText |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 414×896 | 逐字正确 | true | 战绩分数趋势 | rgb(88,166,255) | rgb(255,123,114) / 4px,3px | rgb(126,231,135) / 4 | rgb(255,123,114) / 2 | 414 | 0 | false | null |
| 820×1180 | 逐字正确 | true | 战绩分数趋势 | rgb(88,166,255) | rgb(255,123,114) / 4px,3px | rgb(126,231,135) / 4 | rgb(255,123,114) / 2 | 820 | 0 | false | null |
| 1440×900 | 逐字正确 | true | 战绩分数趋势 | rgb(88,166,255) | rgb(255,123,114) / 4px,3px | rgb(126,231,135) / 4 | rgb(255,123,114) / 2 | 1440 | 0 | false | null |

图例逐字（三视口一致）：━ 分数（蓝·越高越好）　┄ 漏怪率（红虚线·越低越好）　● 绿胜 ● 红负。lineCount=1 / leakCount=1 / winCount=4 / loseCount=2，6 点匹配 mock 4 胜 2 负；bodyLen=991；hits=[/me/battles?limit=20&offset=0]（真实只读接口命中）。旧模式「● 分数（蓝）　● 漏怪率（红）　越低越好」在三视口均不再出现。

## 5. 部署后截图 SHA256

| 文件 | sha256 |
| --- | --- |
| pd-r88-trend-414.png | 3d2e6051af0307bf909f2e20802730b48c783d493e1f1a1baeae9590a397cdbf |
| pd-r88-trend-820.png | 778942c969c3c0d44c02099f4574f3d3c5200dcdc5ea381f5ee9040ff9bbd291 |
| pd-r88-trend-1440.png | e2f434a3ffd4bd45d8d152904791dbb2b53572da2988249e2bb0a740c84d4e45 |

三图 UNIQUE 3/3，全部 ≠ QA 三图（r88-trend-390=7cc419762d7bc3169db978a840ea397b75eb75901f9fe15bd1e6fba1a05ab79a / 768=5af48fde271f7954952463d421f3eb4b84607ef7ad340ea25cc4ee7aff95ebef / 1280=6122d0c100a5ae2713b5a8f9e87001f3cea9bdb576bfd5feb17480e30ffd4997）。亲读三张：均为「我的战报」页非登录页，414 与 1440 清晰可见战绩趋势卡蓝实线 + 红虚线 + 绿红圆点 + 完整图例不截断；820 首屏止于战绩总账（趋势卡在首屏以下，以 DOM 探针坐实）。

## 6. 回归锚点（不变量）

- tokens.css:14 --lyp-token-version / :15 --lyp-r3-version / :16 --lyp-r6-version 三锚点在；业务源码零 --r88- 令牌
- R60 role="button" 51 处；R61 v-aria-native 24 处；R55 placeholder-class="ph" 17 处
- 不变量保持：verify fmtVerifyTime 2 处 / gems normalizeGemGain 3 处 / me nicknameSubmitting 6 处 / skills search-empty-text 1 处

## 7. 门禁

- Go TestNoInfraIdentifiersInRepo PASS（3.963s，RC=0）
- Go TestNoRealSecretsInRepo SKIP 非 PASS（secrets_leak_test.go:182：读不到 .local.keys —— 真值扫描这条守卫没有执行，不是「通过了」；RC=0）
- 依赖 QA：契约 6/6、全量 226 files passed|1 skipped / 2067 tests passed|2 skipped、vue-tsc/build:h5/diff-check RC0

## 8. 诚实边界

- guest token 由 users.id=10 换得，只读接口命中，不修改 QA 库
- 6 条战报为 mock 构造（4 胜 2 负），非真实玩家数据
- 三视口 820 首屏未含趋势卡，以 DOM 探针为差异证据；三图仅亲读判定，非逐像素 OCR
- TestNoRealSecretsInRepo 因缺 .local.keys SKIP，真值扫描守卫未执行，非通过
- 本报告自引用 B5 哈希存在自指悖论，以实际提交哈希为准

## 9. 结论

部署后真浏览器回归 PASS，五批可发布。
