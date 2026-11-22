# t77 R7-BUG-2 修复审计：推荐候选未校验拥有权

> 环境：本地回环 QA 的 PostgreSQL 测试库（真值从 gitignore 的 `laiyipao/deploy/.local.keys` 仅注入进程内存）。本文只写占位符，不写真实域名/IP/口令/密钥。截图证据来自 qa-e2e t75，均为桌面 Chrome 移动视口 390x844，**非物理真机**。

## 1. 结论

推荐 CTA 的 `id/replace` 现在必须同时满足：配置存在、进度解锁、数值更优、**玩家已拥有**、且候选未真正占槽。未拥有时不下发 id，前端不展示不可达收益卡；拥有但未上槽时也能正确下发并真实写入。

修复覆盖两层真实缺陷：

1. `recImpactCTA` 只读 GameConfig + 当前构筑，不查 `user_skills` / `user_equipment`，导致 t75 真浏览器点「确认替换」必失败（equipment 原文：`尚未拥有装备 11（磁力手套）`）。
2. 加上拥有权过滤后暴露第二层：`build["skills"]` 来自 `buildSkillsSQL`（`COALESCE(sl.slot, -1)`），包含**所有已拥有技能**；旧代码用 map key 判「已装备」，会把刚授予、slot=-1 的候选误判为已装备而永远推不出来。这正是 t75 报告 §4 中「临时授予 skill2 后仍写入失败」的剩余根因。

## 2. 依据

- qa-e2e t75 报告 `docs/laiyipao/audits/round7-bugfix/REPORT.md:35-46`：真点击失败原文、DB 前后无变化、决定性实验（授予装备 11 后写入成功）、建议修复方向。
- 写入端事实源：`laiyipao/server/internal/service/equipment.go:240-244`（`尚未拥有装备 %d（%s）`）与 `laiyipao/server/internal/service/loadout.go:47`（JOIN `user_skills`）。
- 构筑事实源：`laiyipao/server/internal/service/build_parts.go:59-65`，未上槽技能 `slot=-1`。

## 3. test-first RED

新增 `laiyipao/server/internal/httpapi/routes_recommendation_cta_ownership_e2e_test.go`：先断言新号未拥有 skill 2 / equipment 11 时 CTA 无 id/replace；再授予归属，断言下发 2/4 与 11/10、basis 含「已拥有」、收益接口 200，并真实 PUT `/api/v1/me/loadout` 与 `/api/v1/me/equipment` 后查库落库。

命令：`go test ./internal/httpapi -run "TestE2ERecommendationCTASkipsUnowned" -count=1 -v`

RED 输出（逐字）：

- `routes_recommendation_cta_ownership_e2e_test.go:56: 未拥有技能却被下发为 CTA：id=2 replace=4（应不下发，前端才能不展示不可达收益卡）`
- `routes_recommendation_cta_ownership_e2e_test.go:106: 未拥有装备却被下发为 CTA：id=11 replace=10（应不下发）`

第二层 RED（授予 skill2 之后）：`routes_recommendation_cta_ownership_e2e_test.go:70: 拥有技能 2 后应推荐 2 替换 4，实际 id=0 replace=0；basis=... 已拥有技能里没有同为主战且基础伤害高于当前 干冰弹(id=4) 的候选，未给出替换 id`。

## 4. GREEN 实现

- `laiyipao/server/internal/service/recommendations.go`：新增 `(s *Service) recOwnedIDs(ctx, userID, kind)`（读 `user_skills` / `user_equipment` 全部行；查库失败 fail closed，不下发 id）；`recImpactCTA` 改为服务方法并接收 ctx/userID，候选增加 `!owned[id]` 过滤；basis 改为「已在你的已拥有技能/装备中」，无候选时返回诚实说明；两个调用点即使无 params 也保留 impactBasis。
- 同文件技能候选判重改为 `impactInt(m,"slot") >= 0`：只排除真正占槽的技能，slot=-1 的已拥有候选可被推荐。
- `laiyipao/server/internal/httpapi/routes_recommendation_cta_e2e_test.go`：t74 旧 fixture 补授予 skill2 / equipment11 归属，与新契约一致。

GREEN 输出（逐字摘要）：

- `go test ./internal/httpapi -run "TestE2ERecommendationCTASkipsUnowned|TestE2ERecommendationImpactCTA" -count=1 -v` → 4 条 PASS，`ok github.com/laiyipao/server/internal/httpapi 0.576s`。
- 新测试同时断言真实写入：`PUT /me/loadout` 后 skill2 占槽、skill4 清槽；`PUT /me/equipment` 后 equipment 11 equipped=t。
- `gofmt -l internal/service internal/httpapi` 无输出；`go vet ./internal/service ./internal/httpapi` exit 0；路由守卫 `TestEveryRouteHasClientCaller` ok 0.164s。
- 全量串行：`go test ./internal/httpapi -count=1 -parallel 1 -timeout 20m` exit 0（job bash-5a73945839be4726）；`go test ./internal/service -count=1 -parallel 1 -timeout 30m` → `ok github.com/laiyipao/server/internal/service 928.161s`，exit 0（job bash-b3335cc201ea48b0）。

## 5. 影响面与纪律

- 不放宽任何写入校验、不改防作弊语义；只让推荐读取与写入端相同的事实源。
- 未执行任何 git 操作；未写入真实域名/IP/口令/密钥；报告中的回环地址仅作本地 QA 术语，无真值。

---

# 第二部分：t78 真浏览器闭环证据（R7-BUG-2 修复后写入成功）

> 补证执行：qa-e2e t78（attempt_id=1a0fe15a）。截图均为**桌面 Chrome 浏览器**本地回环 QA 页面（实际尺寸 1258x588，**非物理真机**；uni-app H5 移动 UI 居中渲染，两侧留黑边）。所有真值从 gitignore 的 laiyipao/deploy/.local.keys 及 QA 库仅注入进程内存，落盘脚本/日志/报告用 `<YOUR_TEST_DOMAIN>` / `<TOKEN>` 占位符。

## A. 结论

R7-BUG-2 修复在真浏览器完全闭环：拥有候选时推荐 CTA 下发合法 id/replace → 有数据态两步确认弹层真打开 → 真点「确认替换」写入成功 → reload 后保持；未拥有候选时不下发 id/replace、不展示不可达收益卡（诚实空态）。

## B. 环境与数据准备（如实记录）

- 后端 **go run 重新加载 t77 新代码**：`go run ./cmd/api`（:8080，DATABASE_URL 本地回环 15440，SETTLE_RECOMPUTE_MODE=shadow），实测 `/healthz=200` / `/readyz=200` / H5=200。旧二进制不含拥有权过滤会误下发未拥有 id，故必须重启加载。
- 账号 user10：3 场真实战报（score 9582/13268/11056，t67 所打），max_stage=60；原始拥有 skill{1,4,7,10} / equipment{1(weapon,t),10(gloves,t)}，skill2/equipment11 未拥有。
- **认证注入方式（关键）**：旧 t75 的「open 后 eval 注入 token」会失败——页面首次加载时应用已自动 guest login 建新号（实测注入后 userId=33/battles=0），因为 `hasUsableToken()`（miniapp/src/api/client.ts:674）检查 `lyp_token_exp` 残留。改用 **document-start init-script**（`--init-script auth-init.js`，在应用 bootstrap 前 `localStorage.setItem('lyp_token', <TOKEN>)` + `removeItem('lyp_token_exp')`）+ 全新会话，首次 open 即注入。DIAG 实测 `userId=10/battles=3/scores=[11056,13268,9582]`。
- **数据准备（临时授予，测完还原）**：user10 原始态无更优候选，故阶段 B/C 临时 `insert user_skills(10,2,1)` 与 `insert user_equipment(10,11,'gloves',false)`，让 CTA 推导 skill2→skill4、equipment11→equipment10；全部截图采集完成后 `update ... set skill_id=4 where slot=1` + `update ... set equipped=(equipment_id=10) where slot='gloves'` + `delete` 两条拥有记录，**已还原 QA 库原始态**（复验：skill_slots=1/4/7、equipment=1(t)/10(t)，skill2/equipment11 拥有记录已删除）。

## C. 阶段 A：未拥有候选的诚实空态（GREEN）

- **equipment 页（user10 未拥有 equipment11）**：`cta-params.js` 输出 equipment CTA `{"slot":0}` —— **无 id/replace**；页面 impact 卡显示「真实收益预估·当前 vs 推荐 / 当前 — → 推荐 — / 收益数据不可用：接口未提供真实预估。不展示虚构数值。」，`hasRealImpact=false`、`impactResponses=0`、探针 pe/ce/af/fr 全 0。截图 `honest-empty-equipment.png`（sha256:2581e74ea513，1258x588），读图确认无可疑虚构数值。
- **新号33（无历史）skills 页**：推荐卡「数据不足·NO_HISTORY」，CTA 无 id，诚实空态「近 7 天还没有战斗记录：先完成一场对局」，探针全 0。截图 `honest-empty-skills-newuser.png`（sha256:4fa31e2a8e94）。

## D. 阶段 B：skills 写入闭环（GREEN）

- CTA：`{"context":"skills","id":2,"replace":4}`（燃油弹2 替换 干冰弹4）。impact 自动调用 200 / rec-v1，真实收益卡「干冰弹 → 燃油弹，每波 +5.26%（2085.23→2194.98），历史三星率 0.0%（stars=3 场数占比，不是胜率）」。
- 四态截图（均 1258x588，读图确认）：
  - `write-skills-1-before.png`（sha256:68ea771c273f）—— 写入前 impact 卡。
  - `write-skills-2-modal.png`（sha256:591c272dc2da）—— 有数据态两步确认弹层，含「当前 vs 推荐·确认替换 / 干冰弹→燃油弹 / 每波 2085.2272→2194.976 / 历史三星率 0.0% / 取消·确认替换」。
  - `write-skills-3-after-apply.png`（sha256:cce8e6deb2ee）—— 点确认后（APPLY msgs 无错误）。
  - `write-skills-4-after-reload.png`（sha256:965ab4cbb4c7）—— reload 后构筑快照与推荐刷新为下一候选（重炮弹），证明写入持久。
- **UI 构筑快照**：ME_BEFORE `skillSlots={1:0,2:-1,4:1,7:2,10:-1}` → ME_AFTER 与 ME_AFTER_RELOAD 均 `{1:0,2:1,4:-1,7:2,10:-1}`（**skill2 占槽 1、skill4 清槽 -1，reload 后保持**）。
- **DB 证据**：写入后 `user_skill_slots` slot1 由 4 → 2；还原后复验回到 1/4/7。

## E. 阶段 C：equipment 写入闭环（GREEN）

- CTA：`{"id":11,"replace":10,"slot":0}`（磁力手套11 替换 劳保手套10）。impact 真实收益卡「劳保手套 → 磁力手套，每波 -12.13%（2085.23→1832.21，负数如实显示），护甲 5‰→10‰」。
- 四态截图（均 1258x588，读图确认）：
  - `write-equipment-1-before.png`（sha256:566ac3dbf252）
  - `write-equipment-2-modal.png`（sha256:1832e83148d9）—— 有数据态弹层「劳保手套→磁力手套 / 2085.2272→1832.208 / 护甲 5‰→10‰ / 取消·确认替换」。
  - `write-equipment-3-after-apply.png`（sha256:67718e1ac9bd）—— APPLY msgs 无错误。
  - `write-equipment-4-after-reload.png`（sha256:c3d82e187af6）—— reload 后推荐回到「构筑短板」诚实空态（无更优已拥有候选）。
- **UI 构筑快照**：ME_BEFORE `equipment=[10,1]` → ME_AFTER 与 ME_AFTER_RELOAD 均 `equipment=[11,1]`（**equipment11 已装备，reload 后保持**）。
- **DB 证据**：写入后 `user_equipment` equipment11 `equipped=t` / equipment10 `equipped=f`；还原后复验 1(t)/10(t)。

## F. 工程达标项不回退

- 探针：阶段 A 两页、阶段 B/C 两页全部 `pe=0/ce=0/af=0/fr=0`（`probe-init.js` 捕获 pageerror/unhandledrejection/console.error/fetch+XHR>=400）。
- 弹层交互用 agent-browser **原生 CDP click**（`.r7-btn.primary` / `.r7-compare .r7-actions .r7-btn.primary`），非 eval 假点击；此前 t75 坐标 click 超视口问题已规避。
- 诚实文案保持：历史三星率标注「stars=3 场数占比，不是胜率」、负数收益如实显示 -12.13%、不可达收益显示「不展示虚构数值」。

## G. 假阳性排除

- 阶段 A 初版（phase-a-empty.sh）open 后 eval 注入 token → userId=33 新号，属采集注入时机假阳性（已弃用），非产品缺陷；改 document-start 注入后正确命中 user10。
- 截图尺寸 1258x588 而非 390x844：uni-app H5 在桌面宽视口下 UI 居中渲染（两侧黑边），属**非物理真机**桌面 Chrome 环境，不影响 DOM/交互证据；已如实标注。

## H. 真值纪律与产物

- 落盘前扫描发现 3 个采集临时脚本（auth-init.js / auth-init-33.js / set-token2.js）残留 JWT 字面量，已用模板占位符覆盖，复扫 `grep -rlE 'eyJ[A-Za-z0-9_-]{40,}|g_[0-9a-f]{40,}'` 零命中。日志中的 `http://127.0.0.1` 为本地回环 QA 地址（允许），无真实域名/IP/口令/密钥。
- 产物：`docs/laiyipao/audits/round7-bug2fix/` 下 REPORT.md（本文件，两部分）、12 张截图（screenshots/）、`data/{auth-test,phase-a-final,phase-bc}.log`、脚本 / 探针（phase-a-final.sh / phase-bc-write.sh / auth-test.sh / probe-init.js / auth-init.js.tpl / cta-params.js / state.js / modal-state.js / me-build.js / diag-token.js / apply-error.js）。
- 未改任何源码、未执行 git、未部署。