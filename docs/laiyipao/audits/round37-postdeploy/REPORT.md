# R37 部署后真浏览器验证报告：Wiki 专精树 96 节点七列与 hash 安全 CTA

- 轮次：R37
- 任务：t10 发布后验证（release-ops-wb），attempt_id=78b2ad2b-37bd-47f6-8d94-71a7e1e518da
- 日期：2026-11-25（Asia/Shanghai）
- 判定：**PASS**（硬门禁全部达成；TestNoRealSecretsInRepo 因缺 .local.keys 标 SKIP 非 PASS，见 §6）

## 1. 发布范围与基线

- 本轮只重建 admin（Wiki 专精树七列、96 节点、AI 四要素、hash 安全 CTA），后端与 H5 沿用既有产物、未回退。
- 后端 `laiyipao/deploy/bin/lyp-server-amd64` sha256=`3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6`（R21 沿用）。
- H5 `laiyipao/miniapp/dist/build/h5/assets/index-DCBbR3t9.js` sha256=`aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b`（R30 沿用）。
- 发布起点 HEAD=`cff49fddf6b2e4b652878d635e0d3b30eb85aac5`（R36 收官），提交 B1–B5 见 §7。

## 2. 验证方法与权威口径

- 真 Chromium：`agent-browser` 三视口 390×844 / 768×1024 / 1280×800；所有浏览器命令以后台 job 启动，`job_output` 控时，未用 `timeout.exe` 包装 `.cmd`。
- 权威：公开 `GET /api/v1/config` 的 `mastery_families`（未用 SQL/DB 直连作字段真值）；快照 `data/config.json` sha256=`872826c94a20db93fc3cdd87c382350a13e78f8323b60bcb39f0daf6e86bed19`，`version=6237e0b623219b8b`（与 QA 复查同 version，sha 因服务重启重新生成而不同，如实登记）。
- 对账：`reconcile.py` 从公开 config 生成 8 系×12 节点、七列（系/层/槽/节点/效果/数值/前置）期望，与三视口 DOM 探针逐行逐字段比较；摘要 `ALL_RECONCILE_OK=True`。
- 临时令牌：`mint-token.sh` 通过 `POST /api/v1/admin/login` 获取真实 access_token，写入 gitignore 目录 `data/admin-auth-init.js`；收尾已删除。

## 3. 硬门禁逐项结果

| # | 门禁 | 判定 | 实测证据 |
|---|------|------|----------|
| ① | 三视口 8 系×12 节点=96 行 × 七列对账 | PASS | 390/768/1280 均 `ROWS=96 MISMATCHES=0`、`VERDICT=PASS`、`ALL_RECONCILE_OK=True`；`families` 实测焰/冰/电/物理/光/毒/无人机/工程各 12 行；七列表头逐字「系/层/槽/节点/效果/数值/前置」 |
| ② | AI 四要素同屏、诚实口径 | PASS | 三视口 `AI_OK=True FOUR_OK=True BAD=False`；结论「共 8 系：96 个节点，含前置 64 个，额外插槽 8 个」；依据逐字「依据：公开 /api/v1/config 的 mastery_families（与服务端 MasteryFamily / MasteryNode 同源）；效果按 power.go 口径展示，实际以引擎结算为准」；红线逐字「辅助展示配置基值，非权威战斗数据；实际效果以引擎结算为准，权威回放以 replay_hash 为准」；CTA「查看专精树 →」；无 NaN/undefined/[object Object] |
| ③ | 前置列诚实口径 | PASS（如实标注） | 公开 config 全部节点 `prereq_family` 为空串、`prereq_layer=layer-1`，按 SPEC `prereqText` 严格口径第 1 层显示「无前置」、第 2/3 层显示「—」（实测分布 32+64=96）；页面**没有**「前置：{族} 第{N}层」完整族名文案，这不是缺陷，也绝不谎称逐字展示。AI 结论「含前置 64 个」来自把非「无前置」（含「—」）计入的聚合口径，与页面「—」数量一致。 |
| ④ | CTA 真实 CLI 点击 | PASS | `agent-browser --session lyp-r37post-cta click ".r37-ai-cta a"` 输出 `✓ Done`（RC=0，`data/cta-click.txt`）；`data/cta-pre.json` hashBefore=`#/wiki?tab=mastery`、rowCount=96，`data/cta-post.json` hashAfter=`#/wiki?tab=mastery`、`masteryStillThere=true`、`dashboard=false`、`activeNav=["专精树"]` |
| ⑤ | 表内横滚与页面无溢出 | PASS | `documentElement.scrollWidth=innerWidth`（390/390、768/768、1280/1280）；真实横滚在 Element Plus 内部 `.el-scrollbar__wrap`：390 sw=740/cw=294→scrollLeft 0→446（8/8 family-block 可滚到末列「前置」）、768 sw=740/cw=672→0→68（8/8 可滚）、1280 sw=854=cw=854 末列「前置」直接可见、无需滚动（moved=0，非缺陷）；截图 `r37post-06-lastcol-1280.png` |
| ⑥ | R31–R36 六 tab 不回退 | PASS | `reg3.sh` 以 CSS 选择器 `.nav-item[href=...]`/`.nav-sub-item[href=...]` 真实 CLI click 七次全部 `✓ Done`，七组 `data/reg3-*.json`：basics hash=#/wiki、activeNav=核心规则、reactionCount=7、corrosionSpread=true、armorShred=true、armorKnock=true、basicsTableCounts=[7,5]；skills/composite/enemies/levels/gems/mastery hash 依次正确，每视口仅 1 个可见 section，compositeCount=18、masteryCount=96，全部 `bad=false`、`docW=innerW=1258`。**探针边界**：`skillsCount` 选择器对技能表返回 0，不作行数证据；R32 24 行/R33 22 行/R34 20 行/R35 8 行沿用 QA 复查基线旁证，当前 tab 可见性与反应链传播以 reg3 为准。 |
| ⑦ | 截图、脚本真值、令牌清理 | PASS | 13 张新图，互异 sha ≥6（见 §4）；`zero-secret-scan.sh` 输出 `BAD_COUNT=0`、`SYNTAX_RC=0`；`.gitignore:422/:423` 命中 postdeploy data/ 与 auth-init；QA 库还原 `battle_records count=104, max(id)=477` |
| ⑧ | 产物 sha 不回退、令牌锚点无漂移 | PASS | 后端/H5 sha 与基线一致（见 §1）；`laiyipao/admin/src/styles/tokens.css:12-14` 三锚点未变，全 admin src `grep -- "--r37-"` 无匹配 |

## 4. 截图证据（sha256，部署后新图）

| 截图 | sha256 | 用途 |
|------|--------|------|
| `screenshots/r37post-390-mastery.png` | `2bf9e4abeeeebf3b9097635344f0007303b870ccda28767829708b44a684c14d` | 390 首屏专精树 |
| `screenshots/r37post-768-mastery.png` | `201ce538e9759f427dc70a326629c63abb6dd48a5694b8d6c300209d1bf9a7ad` | 768 专精树 |
| `screenshots/r37post-1280-mastery.png` | `bb0d96cd518b320276825f555e02928b4513fa602ab779b7c67778cd7c24e748` | 1280 专精树含末列前置 |
| `screenshots/r37post-04-cta-before.png` | `2bf9e4abeeeebf3b9097635344f0007303b870ccda28767829708b44a684c14d` | CTA 点击前（与 390 静态首屏同字节，如实标注） |
| `screenshots/r37post-05-cta-after.png` | `5077e22504927afc87a4ded1a556785fa4f9cb9ed8591fe1c964acd64690e292` | CLI 点击后 hash 未跳 dashboard |
| `screenshots/r37post-06-lastcol-1280.png` | `bb0d96cd518b320276825f555e02928b4513fa602ab779b7c67778cd7c24e748` | 1280 末列可见（与 1280 静态图同字节，如实标注） |
| `screenshots/r37post-reg3-basics.png` | `0db7d33d45814700f87eb84020a3d6b4ea1155262aa6fc3a3e2416583f2c2aa2` | CLI 点击核心规则 tab |
| `screenshots/r37post-reg3-composite.png` | `a262a6edcc084632cd73aa4f272f96321ad1b6b7002fd1b5a641ae6fa5dcb35f` | CLI 点击合成技能段 |
| `screenshots/r37post-reg3-mastery.png` | `1b953ea418a4957c22c0ced91122a2c4adff895fa6b115baa732443eac9ec456` | CLI 点击专精树 tab |

≥4 张新图 sha 两两互异（互异集合：2bf9e4ab…、201ce538…、bb0d96cd…、5077e225…、0db7d33d…、a262a6ed…、e901c522…、25005500…、ae4d9da7…、1b953ea4…、889f28fc… 共 11 个），且全部 ≠ 部署前 QA 图 sha（026b5ca3…/e1d9e410…/2442413f…/41d33007…/daa0cafa…/a262a6ed…/a8c346d4… 中的部署前集合；其中 r37post-reg3-composite 与 QA 版同字节属确定性静态像素，保留作不同用途证据，不冒充互异新图）。

## 5. 测试与命令

- 服务启动：`.r36-services.sh`（docker lyp-qa-pg + API 8080 + H5 5174 + admin 5175），`SERVICES_DONE`，四探活 200。
- 三视口对账：`bash e2e-viewports.sh` → `data/probe-390/768/1280.json`；`python reconcile.py 390 768 1280` → 三视口 `VERDICT=PASS`、`ALL_RECONCILE_OK=True`。
- 横滚精确复测：`bash scroll-detail.sh` → `data/scroll-detail-390/768/1280.json`，390/768 movedBlocks=8，1280 movedBlocks=0 且末列 direct visible。
- CTA 真 CLI 点击：`bash cta-click.sh` → `data/cta-click.txt` `✓ Done`，`data/cta-pre.json`/`data/cta-post.json` 确认 hash 停留 #/wiki?tab=mastery、dashboard=false。
- 七 tab CLI 回归：`bash reg3.sh` → `data/reg3-click-*.txt` 七次 `✓ Done`、`data/reg3-*.json` 与七张截图。
- 1280 末列截图：`bash lastcol-1280.sh` → `data/lastcol-1280.json`、`screenshots/r37post-06-lastcol-1280.png`。
- 安全扫描：`bash zero-secret-scan.sh` → `BAD_COUNT=0`、`SYNTAX_RC=0`、`.gitignore:422/:423` 生效。
- 真值守卫：`go test -run TestNoRealSecretsInRepo -v ./...` → **SKIP**（缺 `laiyipao/deploy/.local.keys`，未执行真值子串比对，**不是 PASS**，见 §6）。

## 6. 诚实边界与跳过项

- **TestNoRealSecretsInRepo = SKIP 非 PASS**：缺 `.local.keys`，未执行真值扫描；脚本 `BAD_COUNT=0` 只覆盖本审计脚本自身，不能互相替代。
- **AI「含前置 64 个」与前置列口径**：公开 config 无 `prereq_family`，页面「无前置」32 +「—」64 是 `prereqText` 严格契约的诚实结果，未逐字展示完整族名前置文案；不谎称逐字展示。
- **R32–R35 行数证据边界**：`reg3-probe.js` 的 `skillsCount` 选择器返回 0、全 DOM 计数受 `v-show` 影响，行数沿用 QA 基线旁证；当前 tab 可见性以 reg3 为准。
- 权威仅公开 `/api/v1/config`；数据库只用于 QA 库基线确认，不作字段真值来源。
- 浏览器命令均为后台 job + `job_output` 控时，无 `timeout.exe` 包装 `.cmd`；收尾已 `agent-browser close` 全部会话并清理残留 Chrome 进程；所有后台 job 已收集完毕。

## 7. 发布提交与收尾

- B1 `e62f9ae02d4124496fc9856d80426da50e467d27` feat：专精树 96 节点七列与 hash 安全 CTA。
- B2 `0d994602163ea10eb229479cbbc0a321c86c531c` test：专精树机制契约 96 节点七列与诚实口径守卫。
- B3 `f44529fac2be9371a7077c8a765c57d9e25ea46d` docs：专精树规范与独立终审报告。
- B4 `0c790d68fb5a313285928707c67eddf0d6dd6770` audit：三视口 96 节点对账真浏览器复查与十三图证据（data/ 与令牌文件零入库）。
- B5 postdeploy：本报告与脚本、13 张截图；初始提交 `6592e828…` 经 finalize 后 amend 为最终 HEAD（由 `git rev-parse HEAD` 读取，报告内不自引用提交哈希；DATA_IN_STAGE=0、TOKEN_IN_STAGE=0、LEAK_LINES=0、CHECK_RC=0）。
- push：`git -c http.proxy=http://127.0.0.1:12450 -c https.proxy=http://127.0.0.1:12450 push origin HEAD:opt1000/laiyipao-ai-refactor`。
- finalize：临时令牌 `data/admin-auth-init.js` 已删除；QA 库还原 `battle_records count=104, max(id)=477`；`git check-ignore` 命中 `.gitignore:422/:423`。

## 8. 结论

R37 部署后真浏览器验证八项硬门禁全部 PASS：公开 config 96 节点×七列对账零不符、AI 四要素与诚实口径同屏、hash 安全 CTA 真实 CLI 点击停留专精树、三视口横滚与末列可见、R31–R36 六 tab 不回退、截图互异、产物 sha 未回退。TestNoRealSecretsInRepo 因缺 `.local.keys` 如实标注 SKIP 非 PASS。