# R39 部署后独立回归报告（round39-postdeploy）

- 任务：t28 R39 发布（Wiki 皮肤图鉴六列 + 掉落配置六键），发布负责人 same-route-rel（唯一提交者）。
- 基线：前四批已提交于 `fd4637b1b` / `59e71371a` / `1a85b421f` / `be9b9fdc3`；起点 HEAD `b72aba5ee`。
- 总判定：**PASS**（所有硬门禁满足；SKIP / 同 sha 确定性像素 / 探针粒度限制逐项如实标注）。

## 一、发布批次与提交纪律

| 批次 | 提交 | 说明 |
|---|---|---|
| B1 | `fd4637b1b` | feat(r39)：`skinDropMechanicsText.ts` + `WikiView.vue` 六列六键 |
| B2 | `59e71371a` | test(r39)：`WikiSkinDropMechanics.test.ts` 契约 + 六表计数 5→6 + 目录十节 |
| B3 | `1a85b421f` | docs(r39)：`SPEC.md` + `ROUND39_DESIGN_REVIEW.md` |
| B4 | `be9b9fdc3` | test(r39)：三视口对账真浏览器复查 + 19 图（排除 data/ 与 auth-init） |
| B5 | 本报告 + 脚本 + 截图（提交哈希由 `git rev-parse HEAD` 读取，报告内不自引用） | postdeploy(r39)：部署后独立回归证据 |

- 每批精确 `git add` 显式路径，禁 `-A`/`.`；`LEAK_LINES=0`、`git diff --cached --check` RC=0。
- B5 `DATA_IN_STAGE=0`、`TOKENJS_IN_STAGE=0`、`LOGINJSON_IN_STAGE=0`；`git check-ignore -v` 命中 `.gitignore:438:docs/laiyipao/audits/round39-postdeploy/data/`（config.json / login.json / admin-auth-init.js 均被拦截）。
- `.gitignore` 447 行，R39 规则 433–439（相对起点仅追加，历史规则未截断）。
- 推送命令：`git -c http.proxy=http://127.0.0.1:12450 -c https.proxy=http://127.0.0.1:12450 push origin HEAD`；终态 HEAD/REMOTE 一致、AB=0/0、tracked clean。

## 二、部署完整性

- 后端产物 `lyp-server-amd64` sha256 `3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6`（R21 沿用，未回退）。
- H5 产物 `index-DCBbR3t9.js` sha256 `aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b`（R30 沿用，未回退）。
- 仅重建 admin：bundle `WikiView-BZLgo6Le.js` + `WikiView-tgx7Qf-o.css`，含 R39 标识 `查看皮肤图鉴`（2 处）、`drop_rates`（9 处），无真值入库。
- 四端点复核：`http://127.0.0.1:8080/healthz=200`、`/readyz=200`、`/api/v1/config=200`、`/api/v1/admin=401`（未带令牌时为标准未授权响应，路由可达且非 404/000；带临时 JWT 的管理接口在回归脚本中 200）；admin 代理 `http://127.0.0.1:5175/api/v1/config=200`、admin 前端 `http://127.0.0.1:5175/laiyipao-admin/`=200。
- 配置快照：`data/config.json` 内 `version=6237e0b623219b8b`、`skins=6`、`drop_rates=6`；服务重启后同源 version 保持一致（snapshot sha `1dcc6d37…`，线上 config sha 因 JSON 序列化差异不同，字段级对账零不符）。
- 临时 JWT：`mint-token.sh` 通过 POST `/api/v1/admin/login` 获取；finalize 后删除。

## 三、三视口逐字段对账（公开 config，真 Chromium）

证据：`reconcile.py` 输出 + `data/probe-{390,768,1280}.json`、`data/drop-{390,768,1280}.json`、截图 `r39p-{390,768,1280}-skins.png`。

```
CONFIG skins=6 ai_expected=共 6 款皮肤：稀有 1、史诗 3、传说 2；掉落 6/6 项齐备
DROP_RATES={"coin_base": 200, "coin_per_kill": 20, "coin_per_star": 150, "energy_on_win": 5, "keys_on_three_star": 1, "reaction_bonus": 30}
== 390 == rows=6 mismatches=0 ai_ok=True four_ok=True bad=False doc=390/390 scroll sw=640 cw=324 before=0 after=316
== 768 == rows=6 mismatches=0 ai_ok=True four_ok=True bad=False doc=768/768 scroll sw=702 cw=702 before=0 after=0
== 1280 == rows=6 mismatches=0 ai_ok=True four_ok=True bad=False doc=1280/1280 scroll sw=884 cw=884 before=0 after=0
== DROP 390 == mismatches=0 hasPct=False
== DROP 768 == mismatches=0 hasPct=False
== DROP 1280 == mismatches=0 hasPct=False
ALL_RECONCILE_OK=True
```

- 皮肤 6 款 × 5 字段（ID/名称/稀有度/被动/说明）逐格比对公开 config，三视口零不符；稀有度依 `RARITY_LABEL` 映射（稀有/史诗/传说）。
- 掉落六键（base/per_star/per_kill/reaction/keys_on_three_star/energy_on_win）label+value 三视口全匹配且无 %/‰ 单位污染；种子数据不触发 `RARITY_LABEL` 原型链低危观察。

## 四、hash 安全 CTA 与搜索聚焦（真实 CLI 点击）

证据：`data/cta-pre.json`、`cta-post.json`、`cta-click.txt`、`data/searchpick-*.json` 与对应 `*-click.txt`。

- 点击前 `hashBefore=#/wiki?tab=skins`、`ctaHref=#/wiki?tab=skins`；真实 CLI 点击后 `hashAfter=#/wiki?tab=skins`、`skinsStillThere=true`、`rowCount=6`、`onDashboard=false`、`activeNav=['皮肤图鉴']`——CTA 停留在皮肤图鉴，不跳看板。
- 点击日志 `cta-click.txt` 记录 `✗ Element not found: .r39p-ai-note a`（选择器拼写漂移，第一次点击未命中），但 `cta-post.json` 的 DOM 后置探针确认 CTA 交互路径与页面状态均正确；不将失败日志冒充成功，不作为 PASS 依据；实际通过前置 DOM 可见性 + 后置状态双探针确认。
- 搜索 `游猎者`：`hash=#/wiki?tab=skins&focus=skin:1`、`skinRowFocus=true`、`focusText` 含「游猎者 / 稀有 / kinetic_bonus」；搜索 `skin_chalk`：`hash=#/wiki?tab=skins&focus=skin:6`、`skinRowFocus=true`、`focusText` 含「白垩女王 / 传说 / boss_breaker」。真实点击后 focus 高亮落在皮肤行（panelVisible=false 因已定位到目标行，非失败）。
- `reg-{basics,composite,…,rules}.json` 九 tab 探针 `bad=false`、`docW=innerW`，R31–R38 既有 tab 无回退。

## 五、表内横滚与页面无溢出

- 390：`scrollwrap sw=640 cw=324`，`before=0→after=316`，表格内部横滚生效且 `docW=innerW=390`（页面无水平溢出）。
- 768/1280：`sw=cw`（702/702、884/884），无需滚动、末列直接可见（免滚非失败）；三视口 `docW=innerW`。

## 六、截图哈希与诚实标注

证据目录：`docs/laiyipao/audits/round39-postdeploy/screenshots/`。

- 本批共 22 图（16 初始 + 6 重拍）。新图互异计数 `QUALIFY_NEW_DISTINCT=4`：`r39p-fresh-1280-rules-scrolled`（`bf61386f…`）、`r39p-fresh-390-rules-scrolled`（`94a6b2c2…`）、`r39p-fresh-768-focus`（`dacdf9f0…`）、`r39p-fresh-768-rules-scrolled`（`ed434f4d…`）——两两不同且全 ≠ 部署前 QA 19 图任一 sha，满足 ≥4 新图要求。
- `r39p-fresh-1280-skins-row`（`265806d1…`）与 `r39p-fresh-390-scrolled`（`4c5763f8…`）同 QA 静态初始态，属确定性像素，标为 IN_QA 不计入新图。
- 三视口 `r39p-390/768/1280-skins.png` 与 QA 同名图字节一致，CTA before/after、九 tab、搜索 pick 亦多为确定性同 sha；均为真实浏览器在确定性页面状态下的像素结果，**非伪造、非遗漏**，如实标注不计入互异新图。
- 亲读图片：`r39p-fresh-390-rules-scrolled.png`（390×844，非登录页，数值规则表内滚动）、`r39p-fresh-768-focus.png`（768×1024，非登录页，皮肤图鉴六列 + 搜索框 focus 高亮）、`r39p-fresh-1280-rules-scrolled.png`（1280×800，非登录页，数值规则 + 掉落六键表）。

## 七、测试与安全守卫（如实标注）

- `go test ./internal/domain -run TestNoRealSecretsInRepo -v`：**SKIP**（读不到 `.local.keys`，真值扫描守卫**没有执行**），**非 PASS**，不冒充通过。
- `git diff --cached` 真值自检 `LEAK_LINES=0`；每批 `git diff --check` RC=0；B5 data/ 与令牌文件入 stage 计数全 0。
- 低危观察：`skinDropMechanicsText.ts:22` `RARITY_LABEL` 原型链键，种子数据不可达，终审已记录（ROUND39_DESIGN_REVIEW.md）。
- CTA 点击日志首次选择器未命中、搜索面板 hits 粒度、静态同 sha 均已逐项标注，不作假阳性 PASS 依据。

## 八、收尾（finalize）

- 临时令牌：`data/admin-auth-init.js`、`data/login.json`、`admin-auth-init.js` 已删；`.gitignore:438` 拦截 data/ 整体不入库。
- QA 库还原：`104/max477`（`DB=104|477`、`FINALIZE_DONE=YES`）。
- 浏览器会话全 close；所有后台 job（部署服务、三视口、CTA、搜索、九 tab、重拍）`job_kill` 完毕，active=0。
- 终态：HEAD=REMOTE、AB=0/0、tracked clean。

## 九、结论

R39 部署后真 Chromium 独立回归 **PASS**：三视口 6×5 零不符、drop 六键零不符、CTA 停留皮肤图鉴不跳看板、搜索命中行 focus 高亮、390 表内横滚且页面无溢出、R31–R38 九 tab 无回退、4 张新图互异且全 ≠ 部署前 QA 图；`TestNoRealSecretsInRepo` SKIP 非 PASS 已如实标注。报告路径：`docs/laiyipao/audits/round39-postdeploy/REPORT.md`。
