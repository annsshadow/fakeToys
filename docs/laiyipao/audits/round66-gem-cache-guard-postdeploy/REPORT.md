# R66 宝石图鉴缓存 JSON 解析兜底 — 部署后真浏览器复核报告

## 1. 结论

**PASS（带诚实边界）**：B1-B4 已精确提交，H5 已重建，部署后于真 Chromium 三视口（414/820/1440，区别于 QA 的 390/768/1280）×三缓存状态（A 损坏 / B 合法 / C 空）共 9 组复核：A/C 均落诚实空态、`rowsCount=0`、`errors=[]`、无白屏；B 渲染 7 行权威属性差分；`hOverflow=false` 全 9 组。产物 sha 与 QA 基线一致、后端/admin 未回退、回归锚点全绿，Go 真值扫描按 SKIP 非 PASS 如实登记。

## 2. 五批提交链

| 批次 | commit | 主题 |
| --- | --- | --- |
| B1 | `9cbeb4713a4d7296af239ceb35ec162c0bbfe37a` | feat(r66) 宝石图鉴缓存 JSON 解析纳入 try/catch，损坏缓存走诚实空态（gems.vue +4/-2） |
| B2 | `6dbb59b8189c39beb3e72cadc4ebf3385f635ea2` | test(r66) 契约 4 项语义判据（r66GemCacheGuard.contract.test.ts，85 行） |
| B3 | `62007e67e7fdb018ab0af57eebf2b0ccf3acbd68` | docs(r66) SPEC.md(46 行)+ROUND66_DESIGN_REVIEW.md(89 行) |
| B4 | `ac53eedb7fc53e7a01deaf817bb1e55922080d5b` | test(r66) audit 真浏览器缓存兜底证据与 gitignore 真值规则（20 文件） |
| B5 | 本批次 | postdeploy(r66) 部署后真浏览器复核证据（本报告+2 脚本+9 截图） |

提交前起点 HEAD=`4302790d5124e872e117b913ffdce8f909bdaedf`（R64 末）。五批均逐文件路径精确 `git add`（禁 `-A`/点号），每批泄露守卫（`postgres://非零非零@` 口令串、私钥、Bearer、Cookie）零命中、`data/` 暂存数=0、`git diff --cached --check` RC=0。

## 3. 产物 sha（权威基线，逐一复核未回退）

| 产物 | 文件 | sha256 |
| --- | --- | --- |
| 后端 | `laiyipao/deploy/bin/lyp-server-amd64` | `3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6` |
| admin | `laiyipao/admin/dist/assets/WikiView-ST1PbAt2.js` | `f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b` |
| H5 主 bundle | `laiyipao/miniapp/dist/build/h5/assets/index-CC1vr8tW.js` | `6bd41ae00888b1b36a3a63eaad2a4825c5645019d128a9d841513f79aee936d1` |

H5 于 `laiyipao/miniapp` 下 `pnpm run build:h5` 重建（BUILD_RC=0），`index.html` 引用 `./assets/index-CC1vr8tW.js`（与 R64 旧 bundle `index-DcssZHXt.js` 不同）。后端/admin 长期沿用不回退。任务描述曾猜 admin 文件名 `WikiView-JpQ4MFXA.js` 在本工作树不存在，以实际文件 `WikiView-ST1PbAt2.js` 为准。

## 4. 部署后真浏览器 9 组探针

服务：API 8080（`start-api.sh`，`go run ./cmd/api`）+ H5 静态 5201（`python -m http.server 5201`）。auth-init.js 经 `gen-alias-auth-init.sh` 由 users.id=10 guest_token 换 token 生成，仅含 lyp_token/refresh/token_exp/guest/profile，data/ 被 `.gitignore:570` 忽略。缓存三态由 QA 的 `state-{broken,valid,empty}.js` 在 localStorage 人工构造。

| 状态 | 视口 | rowsCount | errors | cacheKeyCount | hOverflow | 判定 |
| --- | --- | --- | --- | --- | --- | --- |
| A 损坏 | 414×896 | 0 | [] | 21 | false | 诚实空态 |
| A 损坏 | 820×1180 | 0 | [] | 21 | false | 诚实空态 |
| A 损坏 | 1440×900 | 0 | [] | 21 | false | 诚实空态 |
| B 合法 | 414×896 | 7 | [] | 21 | false | 7 行差分 |
| B 合法 | 820×1180 | 7 | [] | 21 | false | 7 行差分 |
| B 合法 | 1440×900 | 7 | [] | 21 | false | 7 行差分 |
| C 空 | 414×896 | 0 | [] | 0 | false | 诚实空态 |
| C 空 | 820×1180 | 0 | [] | 0 | false | 诚实空态 |
| C 空 | 1440×900 | 0 | [] | 0 | false | 诚实空态 |

A 损坏三视口 `cacheKeysSample` 为 `lyp:r10:gem-gain:18:9` / `:14:9` = `"{broken"`（半截 JSON，兜底分支确实触发），`emptyText="佩戴收益未上报，先看词条描述"`、`gemText=""`、无 `SyntaxError` 冒泡（`window.__qaErrors` 为空）。B 合法三视口 `gemText="服务端属性差分（佩戴前 → 佩戴后）"`，7 行差分逐字为：攻击 1000.0→1050.0 +50.0、暴击率‰ 100.0→110.0 +10.0、暴击倍率‰/反应倍率‰/元素系数‰/热量上限‰/护甲‰ 均 1000.0→1000.0 +0.0。C 空 `cacheKeys=[]`，空态来自正常无数据路径而非异常吞噬。全 9 组 `innerW==docW==clientW`、`hOverflow=false`。

## 5. 部署后 9 张新截图 sha256

| 文件 | sha256 |
| --- | --- |
| `screenshots/pd-broken-414.png` | `c1e2d09c0cea93361ac9f3ced5c7a1d5e569f13ad4c9a08f8bad5c2108dbf968` |
| `screenshots/pd-broken-820.png` | `a2dc2d5a29dec5199f58dee6f0ebd55e2c323de5a85b70787915843e22543fd8` |
| `screenshots/pd-broken-1440.png` | `20d8f604a0c56663cdc93742d6d30ae94516038e3781b0ba695a63b33bc7124a` |
| `screenshots/pd-valid-414.png` | `952a5286e4e33e1050b60333a8bba734d984ff218a110091665bc77cb001e760` |
| `screenshots/pd-valid-820.png` | `c80f1934b21032e9becaf18195aa02d6ba3f2054f3bad08096fc05e2eee209c0` |
| `screenshots/pd-valid-1440.png` | `1922d2ba9b8d483a17e884c7d8d815c2a20d30bdd1a2484b3506543d968e28c2` |
| `screenshots/pd-empty-414.png` | `c1e2d09c0cea93361ac9f3ced5c7a1d5e569f13ad4c9a08f8bad5c2108dbf968`（同 broken-414，语义等价） |
| `screenshots/pd-empty-820.png` | `a2dc2d5a29dec5199f58dee6f0ebd55e2c323de5a85b70787915843e22543fd8`（同 broken-820，语义等价） |
| `screenshots/pd-empty-1440.png` | `d149a6e2b4ea567d23ef576072c3434a43b54fb09e9b5f0ba76c9cb417c11b05` |

UNIQUE=7/9：`broken` 与 `empty` 在 414/820 同视口同 sha（A 损坏与 C 空走同一诚实空态、页面逐像素一致，属预期语义等价，非截图缺失）。9 张部署后截图视口为 414/820/1440，与 QA 九图（390/768/1280）视口互异，sha 全部不相等。亲读 `pd-valid-414.png`、`pd-broken-1440.png`、`pd-empty-820.png` 三张，均为宝石图鉴非登录页：顶部「宝石图鉴」、宝石收藏雷达 GEM COVERAGE、当前最该强化跨系统优先级、装备契合 0/18、0/8 图鉴种类、收集进度、AI 因果链区块正常渲染；valid 图可见 7 行差分卡，broken/empty 图可见「佩戴收益未上报，先看词条描述」空态卡，无白屏/错误遮罩。

## 6. 回归锚点（static-regression.sh 独立复跑）

- 令牌三锚点 `tokens.css:14-16` 逐字在：`--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`。
- 全 `laiyipao/miniapp/src` 中 `--r66-` 业务 0 命中（仅契约 1 处否定断言）。
- R60 `tabindex="0"`=48、`@keydown.enter`=48；R55 `placeholder-class="ph"`=17；R61 `v-aria-native`=17；均未回退。
- R10 双 CTA 保留：`gems.vue:69`「去商城找宝石 →」、`:70`「去背包镶嵌 →」均 `role="button" tabindex="0" @keydown.enter`；佩戴收益卡标题/依据说明/空态 `role="status"` 完好。
- R64 战报竞态守卫未触碰：`battles.vue` `detailPanelSeq`（:380/497/507/772/777/834）逐字在。
- `git diff --name-only`（本轮 tracked）为空；`git diff --check @{u} HEAD` RC=0。

## 7. 门禁与收尾

- 契约 R66 4/4；全量 vitest 219 files / 2030 passed | 2 skipped；vue-tsc RC=0；`build:h5` RC=0；`git diff --check` RC=0（QA 阶段证据）。
- Go（本轮独立复跑，job `bash-2536263afabf49e3`）：`TestNoInfraIdentifiersInRepo` PASS(21.33s, INFRA_RC=0)；`TestNoRealSecretsInRepo` **SKIP 非 PASS** —— `secrets_leak_test.go:182`「读不到 …laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」，SECRET_RC=0 仅表示测试框架正常退出，不代表真值扫描通过。
- QA 库容器 `lyp-qa-pg`：`select count(*),max(id) from battle_records`=104|477。
- finalize：两处 `data/auth-init.js`（QA 目录与 postdeploy 目录）已删除；`.gitignore:567-571` 六规则完整；`git ls-files` 无真值、tracked `data/`=0。

## 8. 诚实边界

1. 缓存损坏/合法/空三态均为 QA 在 localStorage 人工构造，用于确定性触发三分支；`{broken` 半截 JSON 不声称在真实用户中自然发生。
2. 9 张部署后截图 UNIQUE=7/9：A 损坏与 C 空缓存因走同一诚实空态，在 414/820 同视口逐像素一致，是语义等价预期结果。
3. 合法缓存数据形状为 QA 构造，验证的是「解析→展示」链路，不验证服务端快照真实性。
4. 佩戴收益卡在部分视口首屏以下，差分文案以 DOM 探针（`rowsDetail`）为证据而非仅凭截图。
5. Go `TestNoRealSecretsInRepo` 因缺 `.local.keys` SKIP，未执行不等于通过，绝不记为 PASS。
6. B5 commit 自引用哈希悖论：本报告落盘时 B5 自身 hash 未生成，表中以「本批次」标注，提交后以 `git rev-parse HEAD` 为准。
7. QA/部署后会话侧 guest_token 注入仅改浏览器 localStorage，不改 QA 库 DB（104|477 未变）。

**判定：PASS（带上述诚实边界）**
