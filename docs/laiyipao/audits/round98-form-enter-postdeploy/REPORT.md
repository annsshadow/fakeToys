# R98 部署后回归：三页 Enter 提交补漏

## 1. 范围与结论
- 本轮基于 t231 规范/实施/QA/终审全绿（QA REPORT PASS、终审 verdict=pass），对生产构建（非 dev server）做三视口真实浏览器回归。
- 结论：**PASS**。三页（signin/verify/bag）在三视口（414×896、820×1180、1440×900）共 9 组真实 Enter 提交均命中预期 API；零横向溢出；三页截图 UNIQUE 9/9 且全不等于 QA 9 图；亲读 3 张均非登录页。

## 2. 五批链与产物
- B1=f253b78bee4b7432f219c817c078e5362f9f7b1e / B2=9c3052804791d558aac3053ad7a9e165928b8230 / B3=1da195bec2f24b33425fbdfeb2d5fbb7d5741ce8 / B4=16fa6efa90dc1b6636a5db887e05643e18bf4eef / B5=<本提交>（完整 sha 见 §7）。
- H5 主 bundle：`index-DPdZAHUo.js` sha256=`83b9fbaa72d0597e67099b995fae3fcb64059217ee9335c497636908c8d3b132`（index.html:21 引用，≠ R97 `index-Cwr-Q4Qg.js`）。
- 三页 chunk sha256：`pages-signin-signin.DBN0RYSG.js`=7f2e2f157062be3c708fce9daa1f52cd13309ee01eb8244cfe2b44f3f4cec572；`pages-verify-verify.DphS8xY8.js`=0aa94d35390b2edee75bff579ab40d01ca58ea67e7d45b3d94437ddeecf1704d；`pages-bag-bag.CvCi2uV7.js`=ca84b1fd2958e766b7fb788108b3901f7bfae03327f8bac2fd243b5c68034f2c。
- 后端 `lyp-server-amd64` 沿用 3d82635f…，不回退。

## 3. 三视口 9 组真实 Enter 提交探针
- 方法：生产静态构建跑 python http.server 5201（dist/build/h5），后端 API 8080 真实启动；浏览器 session r98pd 对每视口每页 goto #/pages/<p>/<p>，注入单行 IIFE 的 XHR.open hook 记录请求，`fill` 输入后 `press Enter`，再读请求数组与 iw/cw/sw。
- 414×896：signin 输入「QA-CODE-001」→ `/api/v1/redeem` 命中 1 次；verify 输入「12345」→ `/api/v1/battle/12345/replay` 命中 2 次；bag 输入「QA预设测试」→ `/api/v1/me/presets` 命中 6 次。三组 iw=cw=sw=414，hOverflow=false。
- 820×1180：signin→redeem 4 次；verify→replay 5 次；bag→presets 12 次。三组 iw=cw=sw=820，hOverflow=false。
- 1440×900：signin→redeem 7 次；verify→replay 8 次；bag→presets 18 次。三组 iw=cw=sw=1440，hOverflow=false。
- 三组输入值均经 activeElement 核为预期值且 aria-label 正确。点击/按键在同一 session 累积请求导致 hit 次数递增，属探针累积效应，唯一 URL 口径每组正确。
- 诚实边界：生产构建 `window.uni` 存在但 `uni.showToast`/`uni.setClipboardData` undefined（同 t239 教训），故采用真实 Enter+XHR hook 而非 QA 的 dev-server 全局 uni hook；本轮只证明 Enter 触发同路径请求，未断言 UI 反馈文案。QA 层已覆盖 toast 文案，本轮不重复。

## 4. 部署后截图与亲读
- 9 图 sha256（UNIQUE 9/9，全 ≠ QA 9 图）：
- pd98-414-signin=031eced7d0d285b5870c302bd0f120e3862f7a40ce26e389bf65681888010caf
- pd98-414-verify=efa52944c6f10ce7acac51783adbdb5f92c77b984ae9e7c3300da038b077f3ba
- pd98-414-bag=ffec1973c7a820308721a69251d9cd4c7078ca76435aecedc56d0390001c2e8b
- pd98-820-signin=81b2dd6f41147feb13a4896b3627bd1a3dbb689ea74939d7ddb81c65c38c7cb3
- pd98-820-verify=3e6cd8817f1bc1270519eaad6c46e2344b2a3cf886a239c2037b551aefde5e69
- pd98-820-bag=2150cdc023382999ee57750da72311987283d49d007618920ec4d304a5bf10c3
- pd98-1440-signin=7edd2523c568ac6fbb5847263282184c0aa5a4bff77a3697d096b980747047ca
- pd98-1440-verify=83794465b9c8bda44810d66a6d383a7a7035e7f4c5841ece73cb47a69cb0775d
- pd98-1440-bag=5ae03ab6a99f28694e0fc6841e3dbf5d007311160b854e51520b9cfa7f6e5f50
- 亲读 3 张（pd98-414-signin=签到页/兑换码「QA-CODE-001」可见；pd98-820-verify=战绩验真页/battle_id「12345」可见；pd98-1440-bag=背包页/预设名「QA预设测试」及已存条目），均非登录页。其余 6 张仅列 sha，未逐一亲读，如实披露。

## 5. 回归锚点与门禁
- B1 三页各 +1 行 `@keydown.enter="handler"`，与 QA 契约一致；业务零 `--r98-`（仅契约否定断言命中）。
- G1 契约 9/9；G2 me.test 29/29；G3 全量 228f|1skip、2081t|2skip；G4 vue-tsc RC0；G5 build:h5 RC0；G6 diff-check RC0（QA 阶段）。
- 本轮重跑 Go：`TestNoInfraIdentifiersInRepo` PASS 7.9s；`TestNoRealSecretsInRepo` SKIP 非 PASS（`secrets_leak_test.go:182` 逐字「读不到 …deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」）。
- QA 库只读核验 `users=828 / battle_records=104`（docker exec lyp-qa-pg psql -U postgres -d laiyipao）。

## 6. 合规
- auth 五路径全 ABSENT；本轮五批 tracked data=0；每批 `git diff --cached` 泄露守卫 LEAK_OK；精确逐文件 git add，未用 -A/点号/commit -am。
- pd 驱动脚本与 XHR hook 落在 `data/`（已 .gitignore 预置忽略），不入库；仅提交报告与 9 张截图。

## 7. 五批完整 sha
- B1=f253b78bee4b7432f219c817c078e5362f9f7b1e（feat 三页 Enter 提交）
- B2=9c3052804791d558aac3053ad7a9e165928b8230（test 契约 9it）
- B3=1da195bec2f24b33425fbdfeb2d5fbb7d5741ce8（docs SPEC+RED+终审）
- B4=16fa6efa90dc1b6636a5db887e05643e18bf4eef（audit QA 报告+脚本+9 图）
- B5=见最终提交（chore 本报告+9 图）

## 8. 判定
- 三页 Enter 提交在生产构建三视口全部送达预期 API，零横溢、截图互异、亲读非登录页，回归通过。