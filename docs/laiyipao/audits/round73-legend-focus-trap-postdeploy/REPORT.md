# R73 战场图例弹层焦点管理 — 部署后真浏览器回归报告

- 任务：t161 / attempt cf1758b8-1912-49a3-9812-1b2ab1ec21e9
- 轮次起点：HEAD `def111dd8a579ecaa297ff65537c38c8998da32d`（R72 收官），AB=0/0
- 口径：唯一发布者 release-ops；五批精确 `git add` 逐文件（禁 -A/点号/目录）；零真值入库；真 Chromium 三视口点击 + 真键盘。
- 报告路径：`docs/laiyipao/audits/round73-legend-focus-trap-postdeploy/REPORT.md`

## 一、五批链表

| 批次 | commit | 说明 |
| --- | --- | --- |
| B1 feat | `acd768ce730f18d23e647adcc8653583ee665271` | battle.vue +39/-3：help-chip 键盘可达、legend-overlay role=dialog/aria-modal、焦点移入恢复、Tab/Shift+Tab 陷阱、开暂停关恢复 |
| B2 test | `23be437f4508a8bdb8c5fac5c2f6562c38f5104e` | 契约 r73LegendFocus.contract.test.ts 101 行 6it |
| B3 docs | `4e28521dff6cd05c8f3c619b5b648b84acd84463` | SPEC.md + ROUND73_DESIGN_REVIEW.md（终审 verdict=pass 六维全绿） |
| B4 audit | `7cc06c605e2dbc6370d4bc638ceb3fcf591690d9` | .gitignore+6 行、QA REPORT.md、10 脚本、7 截图（19 文件 285+） |
| B5 postdeploy | 本报告提交后生成 | REPORT.md + pd-probe.js + pd-driver.sh + 9 截图（12 文件） |

每批 DATA_IN_STAGE=0 / LEAK_OK / CHECK_RC=0，warning 仅 LF→CRLF。

## 二、产物基线 sha256

- H5 重建：laiyipao/miniapp 下 `pnpm run build:h5` RC=0（job bash-3cca16cf28504443）；index.html 引用主 bundle `index-B1_dpZ3t.js`，sha256=`e078636779c9b9caf1f522d9dd27cc988d2e246a4760b0a2fb3039ed246f0550`（≠R72 `index-Cr-I6ffw.js`）。
- 后端 `laiyipao/deploy/bin/lyp-server-amd64` sha256=`3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6`，沿用不回退。
- admin `laiyipao/admin/dist/assets/WikiView-ST1PbAt2.js` sha256=`f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b`，沿用不回退（本树实测唯一 admin 文件）。

## 三、部署后真浏览器三视口探针

- 环境：API 本地回环 8080（`/api/v1/config`=200，go run ./cmd/api）；H5 静态本地回环 5201（python http.server on dist/build/h5）。
- 登录：`gen-alias-auth-init.sh` 从 QA 库 guest 账号换 token 注入（仅本地回环，脚本/token 均落 gitignored data/，收官删除）。
- 视口：414×896 / 820×1180 / 1440×900（区别 QA 的 390/768/1280，保证截图异 sha）。
- 探针口径：`hOverflow = document.documentElement.scrollWidth <= document.documentElement.clientWidth + 1`，**true 表示无横向溢出**（历史命名勿反读）；三视口实测 docW==clientW==innerW。

| 视口 | 阶段 | 焦点 | 弹层 | role/aria-modal/aria-label | 暂停 | Tab×6 / ShiftTab×6 | JS 错误 | hOverflow |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 414 | BEFORE | BODY | 关 | null/null/null | false | — | [] | true（sw==cw） |
| 414 | OPEN | legend-close（UNI-VIEW 文本×, role=button） | 开 | dialog/true/战场图例 | true | inOverlay 恒 true 6/6、6/6 | [] | true |
| 414 | CLOSED | help-chip（UNI-VIEW 文本?, role=button） | 关 | null/null/null | false | — | [] | true |
| 820 | BEFORE | BODY | 关 | null/null/null | false | — | [] | true |
| 820 | OPEN | legend-close（role=button） | 开 | dialog/true/战场图例 | true | inOverlay 恒 true 6/6、6/6 | [] | true |
| 820 | CLOSED | help-chip（role=button） | 关 | null/null/null | false | — | [] | true |
| 1440 | BEFORE | BODY | 关 | null/null/null | false | — | [] | true |
| 1440 | OPEN | legend-close（role=button） | 开 | dialog/true/战场图例 | true | inOverlay 恒 true 6/6、6/6 | [] | true |
| 1440 | CLOSED | help-chip（role=button） | 关 | null/null/null | false | — | [] | true |

- OPEN 态 chipRole=button、chipTab=0、closeRole=button、closeTab=0，说明帮助 chip 与关闭按钮均可键盘到达。
- MANUAL_PAUSE（手动先暂停再开图例）：焦点落 legend-close，关闭后 paused 仍保持 true —— 页面未误恢复用户手动暂停。
- MUTEX（R29 互斥）：点 `.r28-feed-cta` 后图例关闭（overlayOpen=false, drawerOpen=false 诚实边界，详见第七节）。

## 四、部署后截图 sha256（9 张）

| 截图 | sha256 |
| --- | --- |
| pd-414-before.png | `43fa9da991cab549e8dbe63199e139726e2904afbb1608612183180ceef37019` |
| pd-414-open.png | `72a80716f2712c1dac84d5bb79282ad17c5f490dc945b19fc242785d1b1a5035` |
| pd-414-closed.png | `ecd19d9d4af0ee0c8cabc5db3b398c74aa495538d46c544ad449fc92bca774f1` |
| pd-820-before.png | `ae3aa94ef6487ec14ebead68078d75004c9d4557327f049aec75196974d6777b` |
| pd-820-open.png | `579a8b8722dfde3a2512f772582b58cea84ff9cde53f291cdc99f9dd3cde4d21` |
| pd-820-closed.png | `96be91b487592d22a48e2b0ecbad495ec486dfcaa53b76758757f4af3b603f6d` |
| pd-1440-before.png | `9fa6039e2e0dcd4da2726dea32d1119854c9c22a1f5b7dd73c9d5f730ce0dc0d` |
| pd-1440-open.png | `0295d4f7b52f916323a0b4ab6e78cb83699d63fa16426564c2bd59b344d8ecf7` |
| pd-1440-closed.png | `36936ef984f1b00e95b8eac725508f33dce5e396b08b9c03b65c3b587aa9a180` |

- UNIQUE=9/9；全部 ≠ QA 七图（r73-390-open=3d9c053f…/390-closed=ebe1606a…/768-open=0093c0eb…/768-closed=bea370f6…/1280-open=61b386cb…/1280-closed=752421e2…/trial-open-768=dcdd3a5d…），因视口互异。
- 亲读 3 张：pd-414-open（战场图例弹层全屏覆盖，标题「战场图例」+ 热量条/技能槽/元素反应/蒸汽爆发等条目，非登录页）；pd-820-closed（战场 AI 归因面板与帮助 chip「?」，非登录页）；pd-1440-open（居中图例弹层，内容与 414 一致，非登录页）。

## 五、回归锚点（不回退）

- 令牌三锚点：tokens.css:14-16 `--lyp-token-version` / `--lyp-r3-version` / `--lyp-r6-version` 原样在位。
- R60 `role="button"` 键盘可达：全 src 51 处；battle.vue @keydown/tabindex 组合在位。
- R61 `v-aria-native` 桥接：全 src 23 处。
- R55 `placeholder=` 表单提示：17 处。
- R71 `fmtVerifyTime`（验真页记录时间本地化）：verify.vue 2 处在位。
- R72 `.search-empty-text`（搜索空态极端值）：common.css 1 处在位。
- R73 业务源码无 `--r73-` 令牌：仅契约 r73LegendFocus.contract.test.ts:99 否定断言命中，业务 0。

## 六、门禁

- QA 门禁（t160）：契约 6/6、battle.test 61/61、全量 224 files passed|1 skipped / 2055 tests passed|2 skipped、vue-tsc RC0、build:h5 RC0、git diff --check RC0（仅 LF→CRLF）。
- Go 门禁（job bash-01f289cefad64b83 EXIT=0，结果落 gitignored data/go-gates-out.txt）：
  - `TestNoInfraIdentifiersInRepo` PASS。
  - `TestNoRealSecretsInRepo` **SKIP 非 PASS**（逐字：`读不到 …laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是「通过了」`；GO2_RC=0）。
- QA 库容器只读核对：`battle_records` = 104 行 / max(id)=477（未写库）。

## 七、诚实边界

1. guest token 来自本地回环 QA 账号，仅用于登录态注入，未修改数据库记录。
2. 部署后 9 组由真 Chromium 真点击 + 真键盘产生；battle 页 level=1 自动开局数据来自本地 mock 服务，非真值。
3. 9 张截图 sha 全互异且 ≠ QA 七图，但仅亲读其中 3 张（414-open / 820-closed / 1440-open），其余 6 张由 sha + DOM 探针佐证。
4. R29 互斥：部署后仅坐实「点 CTA 后图例关闭」；QA 报告已披露「抽屉展开态未捕获」为已知边界，本轮不夸大。
5. Go `TestNoRealSecretsInRepo` 因缺 `.local.keys` 为 SKIP，**不是 PASS**，不作「真值扫描通过」表述。
6. B5 自引用哈希悖论：本报告随 B5 提交，其自身 commit hash 在提交后生成，未逐字写入正文。

## 八、结论

部署后三视口 ×（打开/焦点陷阱/归还/暂停保持/互斥）全链路符合预期，图例弹层 role=dialog + aria-modal + Tab/Shift+Tab 焦点陷阱 + ESC 归还均坐实，零 JS 错误、零横向溢出；产物 sha 与重建一致，历史锚点不回退。结论：**可发布**（Go 真值扫描 SKIP 如实标注）。