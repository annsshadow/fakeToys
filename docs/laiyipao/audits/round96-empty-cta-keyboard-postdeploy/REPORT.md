# R96 六页空态 CTA 键盘可达 —— 部署后真浏览器回归报告

## 1 范围与方法

- 任务：t232 R96 五批发布 B5 部署后回归，验证重建 H5 产物在真 Chromium 下六页空态 CTA 键盘可达（Tab 命中 / AX button / 焦点环 / Enter 送达 / 零横溢）未回退。
- 基线：起点 `85d5b0d91f384fdb0bda1625d9893680a9d3dfbf`（R95 收官），B1–B4 本地领先 4 提交；QA（t231）报告 `docs/laiyipao/audits/round96-empty-cta-keyboard/REPORT.md` 结论 PASS，终审 `docs/laiyipao/reviews/ROUND96_DESIGN_REVIEW.md` verdict=pass。
- 环境：agent-browser 真 Chromium；三视口 414×896 / 820×1180 / 1440×900；H5 静态服务 `http://127.0.0.1:5201`（`python -m http.server 5201 --bind 127.0.0.1` on `laiyipao/miniapp/dist/build/h5`，后台作业 bash-1c6b239573694a3c，收尾已 kill）。
- 样本：每视口 `battles` 与 `me` 两页，共 6 组。生产构建下直接 hash 导航自然渲染空态 CTA（`ctaCount=1`/`emptyExists=true`），无需后端或 `uni.navigateTo`（生产构建缺该 API，R95 教训）。`battles` Enter 跳 stage 坐实导航，`me` Tab 需 2 次（先命中 R95 返回按钮）、Enter 因 `r4GoMe()` 空态 return 为 no-op（既有业务守卫，非绑定失效）。
- 方法：`open` 空页预热 → `set viewport` → `navigate` 目标 hash → `sleep 4` → 注入 `enter-observer.js` → `eval --stdin` 六探针（probe-cta / probe-hit / probe-active / probe-focusring / after-probe）→ AX snapshot → `screenshot <路径>` → `press Tab` 命中 → `press Enter` 前后对比。日志见 postdeploy `data/pd-{V}-{P}.log`（被 gitignore 排除）。

## 2 五批提交链表

| 批次 | 类型 | commit | 内容 |
|---|---|---|---|
| B1 | feat | `8148da102f1719c7615cd398e36e0f96f3805543` | 六页空态 CTA 补 role/tabindex/@keydown.enter（各 +1/-1） |
| B2 | test | `a5d611dc5921705a8aa2de3e603cc01d92016105` | 契约 `r96EmptyCtaKeyboard.contract.test.ts` 135 行 8 it |
| B3 | docs | `f1860dfc26a161d32b92fe3dde409590a30cd8f0` | SPEC + RED 证据 + RED.txt/RED-rerun.log + 终审 |
| B4 | test | `98da5b437efe8302aa9f8f2ca9f453dc4d163002` | .gitignore + QA REPORT + 5 脚本 + review-gates + 18 截图（26 文件） |
| B5 | chore | 本提交 | 部署后回归报告 + pd 驱动脚本 + 6 截图 |

每批 B1–B4 精确 git add 逐路径（RED.txt/RED-rerun.log/review-gates*.txt 用 `git add -f` 绕通用忽略），泄露守卫 LEAK_OK、`DATA_IN_STAGE=0`、diff-check CHECK_RC=0，warning 仅 LF→CRLF。

## 3 产物 sha256

| 产物 | sha256 | 说明 |
|---|---|---|
| `laiyipao/miniapp/dist/build/h5/assets/index-BMmuAvKT.js` | `001fc5966c42769a3416a7e24565192450ec57e32583a7659360153c547c4c8c` | H5 主 bundle，`index.html:21` 引用，≠R95 `index-D99R8srB.js` |
| `laiyipao/dist/build/h5/index.html` | `7d27a920ca5969721bbb69b4845e0d86bbb66bdecc3f6f4d223674e50a535931` | 重建入口 |
| `laiyipao/deploy/bin/lyp-server-amd64` | `3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6` | 后端沿用不回退 |
| `laiyipao/admin/dist/assets/WikiView-1EWc_XRk.js` | `3fb8d0870678d661e1967f66fd2e7bfeab3d825fc0318465924b56f93512f0a5` | admin 唯一 WikiView js，沿用不回退 |

## 4 三视口 × 两页实测

| 视口 | 页 | CTA role/tab/tabIdx/cursor | AX button | Tab 命中 | 焦点环 | Enter 送达 | iw=cw=sw / hOverflow | Enter 后 |
|---|---|---|---|---|---|---|---|---|
| 414 | battles | button/0/0/pointer | 去关卡页挑一关开打 → | 1 | solid/1px/rgb(255,107,53)/offset1px | isCta=true | 414=414=414 / false | hash→#/pages/stage/stage |
| 414 | me | button/0/0/pointer | ‹ 返回 + 去专精页 → | 2 | solid/1px/rgb(255,107,53)/offset1px | isCta=true | 414=414=414 / false | no-op（r4GoMe 空态 return） |
| 820 | battles | button/0/0/pointer | 去关卡页挑一关开打 → | 1 | solid/3px/rgb(255,107,53)/offset2px | isCta=true | 820=820=820 / false | hash→#/pages/stage/stage |
| 820 | me | button/0/0/pointer | ‹ 返回 + 去专精页 → | 2 | solid/3px/rgb(255,107,53)/offset2px | isCta=true | 820=820=820 / false | no-op |
| 1440 | battles | button/0/0/pointer | 去关卡页挑一关开打 → | 1 | solid/1px/rgb(255,107,53)/offset1px | isCta=true | 1440=1440=1440 / false | hash→#/pages/stage/stage |
| 1440 | me | button/0/0/pointer | ‹ 返回 + 去专精页 → | 2 | solid/1px/rgb(255,107,53)/offset1px | isCta=true | 1440=1440=1440 / false | no-op |

六组全部 `ctaCount=1` / `emptyExists=true` / `role=button` / `tabindex="0"` / `tabIndex=0` / `cursor=pointer` / `activeIsCta=true`，AX 树可见 button 角色，焦点环 solid 橙 `rgb(255,107,53)`，零 JS 错、零横溢。

## 5 部署后截图证据

6 张 `screenshots/pd96-{V}-{P}.png`，SHA-256 如下（UNIQUE=6/6，全 ≠ QA 18 图）：

| 文件 | SHA-256 |
|---|---|
| pd96-414-battles.png | `c39628a9e4fbe6949dc8682c406bfc502d608e33e3716b825c6d25e0268d6121` |
| pd96-414-me.png | `db43c60d6b3cc100ecba82e7c45f203a030a1eda20ca2e16804cb8312c782df7` |
| pd96-820-battles.png | `869d54aa6b3255cadf9eb3f21612633422472778a76d1ecf00582c24521667ac` |
| pd96-820-me.png | `3d2b72f5680b59e7930205450e83f76913e8b799200ba2c4192a4abbb3df0a92` |
| pd96-1440-battles.png | `c9fae87b76f294b09a86adbcb060ab3c8a830e390261a4cd1e04eb54a4fb645b` |
| pd96-1440-me.png | `7a24a5f0c76c6d00cf55b82f19982a5a0347a52e61d2f767f26327b06121088c` |

亲读 3 张（414-battles / 820-me / 1440-battles）均为业务页非登录页：414 与 1440 为「我的战报」空态、CTA「去关卡页挑一关开打 →」带橙色焦点环；820 为「指挥官档案」、CTA「去专精页 →」带橙色焦点环。其余 3 张仅列 SHA-256，未逐张亲读，如实披露。

## 6 回归锚点与门禁

- 静态锚点：`tokens.css:14/15/16` 三令牌在（`--lyp-token-version` / `--lyp-r3-version` / `--lyp-r6-version`）；业务源码零 `--r96-`（仅契约否定断言）；六页 CTA 四属性顺序逐行核对在；`role="button"` 58 处、`v-aria-native` 17 处、`placeholder-class="ph"` 17 处、`fmtVerifyTime` 2 处、`search-empty-text` 8 处，均不回退。
- 前端门禁（QA 期已过，B5 未改业务源码）：契约 8/8、me.test 29/29、全量 228 files passed | 1 skipped / 2081 tests passed | 2 skipped、vue-tsc RC0、build:h5 RC0、diff-check RC0。
- Go 门禁（本轮 `bash-59ec43043070407e`，`cd laiyipao/server`，`go test ./internal/domain -run "TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo" -v -count=1`）：`TestNoInfraIdentifiersInRepo` PASS 8.48s RC0；`TestNoRealSecretsInRepo` **SKIP 非 PASS**，逐字「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」（`secrets_leak_test.go:182`），GO_RC=0。不得表述为「真值扫描通过」。
- postdeploy `data/` 与 `admin-auth-init.js` 被 `.gitignore:624/:625` 命中，探针日志与 Go 输出不入库。

## 7 诚实边界

- `me` 三视口 Enter 事件 `target.isCta=true` 已送达 CTA handler，但 `r4GoMe()` 因空态 `r4AiMe=null` 提前 return，无 DOM/导航变化；属既有业务守卫，非键盘绑定失效。
- 生产静态构建无 `uni.navigateTo`，`__qaNav` 记录为空；`battles` 以 `hash` 从 `#/pages/battles/battles` 变为 `#/pages/stage/stage` 坐实 Enter 导航。
- 空态来自六页本地兜底（后端未启动），非 API 返回空列表；未使用 mock 或人为构造。
- 6 图仅亲读 3 张，其余只列 SHA-256。
- Go `TestNoRealSecretsInRepo` SKIP 非 PASS，真值扫描守卫未执行。
- QA 库 `lya_qa` 只读复核：`users=579` / `battle_records=104`（与 QA 期 477 不同系并行活动写入；本轮未写库）。

## 8 结论

**可发布。** 重建 H5 产物在真 Chromium 三视口下，六页空态 CTA 键盘可达（role/tabindex/Enter/AX button/焦点环/零横溢）未回退；五批链完整、产物 sha 与后/Admin 基线一致、Go NoInfra PASS、零真值入库。B5 报告与截图据此归档。
