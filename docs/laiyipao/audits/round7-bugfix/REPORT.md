# t75 R7-BUG-1 修复后有数据态弹层真浏览器证据 & R7-BUG-2 发现

> 环境：本地回环 QA（H5 http://127.0.0.1:5174、API http://127.0.0.1:8080、PG 容器回环 15440）。截图均为桌面 Chrome 下移动视口 390x844，**非物理真机**。域名/口令/密钥一律占位符 `<YOUR_TEST_DOMAIN>` / `<ADMIN_PASS>`，真值从 gitignore 的 laiyipao/deploy/.local.keys 注入内存，未入库。

## 1. 结论速览

- R7-BUG-1（CTA 缺 id/replace）修复确认 GREEN：t74 新码在后端加载后，skills/equipment 推荐 insight 的 CTA params 均含非零 id/replace，前端自动请求 /me/recommendations/impact 并渲染真实收益对比卡。
- 有数据态两步确认弹层真浏览器打开成功（原生 CDP click，非 eval 假点击），截图读图确认含真实数据。
- **新发现真实跨层缺陷 R7-BUG-2【中】：推荐候选未校验玩家是否拥有，导致两步确认的『确认替换』写入必然失败**（详见 §4）。决定性实验坐实因果。
- 探针 pe/ce/af/fr 全 0；本轮未改任何源码、未 git、未部署。

## 2. 数据来源与会话身份

- 有真实战报的账号 user_id=10（battle_records score=9582/13268/11056，来自 t67 打的 3 场第 1 关）。
- 通过 `guest_token` → `POST /api/v1/auth/guest` 重签 access_token(len=157) 注入浏览器 localStorage；注入模板同时 `removeItem('lyp_token_exp')`（根因：api/client.ts:674 hasUsableToken() 检查残留过期时刻导致重新游客建号），整页 reload 让组件重新 bootstrap。
- DIAG 探针实测：`userId=10 / battles=3 / scores=[11056,13268,9582]`。QA 库为复现候选写入把 user_progress.max_stage 临时设为 60（本地回环 QA，非生产）。

## 3. CTA 闭环与有数据态弹层证据

- CTA 探针真实输出（`cta-params.js`）：
  - skills：`{"context":"skills","id":2,"replace":4}`（燃油弹 id=2 替换干冰弹 id=4）。
  - equipment：`{"id":11,"replace":10,"slot":0}`（磁力手套 id=11 替换劳保手套 id=10）。
- impact 接口自动调用返回 200 / rec-v1 / insights=1，渲染真实收益卡：
  - skills：当前干冰弹 → 推荐燃油弹；每波 1966.33 → 2194.98（+11.63%）；历史三星率 0.0%（stars=3 场数占比，不是胜率）；依据近 7 天 3 场命中/波 19.1。
  - equipment：当前劳保手套 → 推荐磁力手套；每波 1966.33 → 1727.74（-12.13%，负数如实显示）；护甲 5‰ → 10‰。
- 弹层真打开（`click '.r7-btn.primary'`，agent-browser 原生 CDP 自动滚动；此前 eval 坐标 y=1020 超出 844 视口导致 false）。MODAL_STATE 读到 title=「当前 vs 推荐 · 确认替换」、rows/star/basis/actions 齐全，读图确认。
- 截图（sha256 前 8 位）：
  - `r7fix-skills-loaded-final.png` = 6d6b925b（390x844 真实收益卡）
  - `r7fix-skills-modal-final.png` = f0922ec6（390x844 有数据态弹层，已读图确认）
  - `r7fix-skills-applied-final.png` = db9e63dc
  - `r7fix-equipment-loaded-final.png` = ee3a17ea
  - `r7fix-equipment-modal-final.png` = fa8cbaa2（390x844 有数据态弹层）
  - `r7fix-equipment-applied-final.png` = 89d9006a

## 4. R7-BUG-2【中】推荐候选未校验拥有权 → 确认替换必然失败

- 现象（真浏览器点击弹层『确认替换』后页面原文）：
  - equipment：`写入失败：输入非法: 尚未拥有装备 11（磁力手套），原装备配置未改动。请重试`
  - skills：`写入失败，原出战配置未改动。请重试`
- 写库校验：点击前后 DB `user_skill_slots`（10→slot0=1/slot1=4/slot2=7）与 `user_equipment`（gloves:10 equipped=t）均无变化。
- 根因：后端 `laiyipao/server/internal/service/recommendations.go:136-205` 的 `recImpactCTA(build, cfg, maxStage, context)` 只按 GameConfig + max_stage + 数值更优推导候选 id/replace（equipment 分支约 :182-205），**未查询玩家实际拥有哪些 skill/equipment**；而写入端 `laiyipao/server/internal/service/equipment.go:240-244` 有硬校验 `尚未拥有装备 %d（%s）`。QA 实测 user10 仅拥有 skill{1,4,7,10}、equipment{1,10}，候选 skill2/equipment11 均未拥有 → 弹层承诺的替换永远写不进去。
- 决定性实验（`ownership-decisive.sh`，QA 库临时授予后还原）：临时 `insert user_skills(10,2,1)` 与 `insert user_equipment(10,11,'gloves',false)` 后重试确认：
  - equipment 写入 **成功**：DB 变为 `11|gloves|t` / `10|gloves|f`，ERRORS_equipment 为空（无错误）——坐实『未拥有候选装备』即根因。
  - skills 临时授予 skill2 后仍报「写入失败，原出战配置未改动」，说明 skills 侧另有写入约束（待后端排查，不影响 BUG-2 成立）。
  - 实验结束已把 QA 库还原（skill_slots=1/4/7、equipment 10 equipped=t，skill2/user_equipment11 删除），DB 已恢复原状。
- 建议修复（后端写域，本任务不碰）：recImpactCTA 入参加入玩家拥有集合（skill/equipment 拥有表），候选必须落在已拥有集合内；或写入端对推荐路径自动补发拥有项。test-first RED→GREEN。

## 5. 假阳性排除

- `r7fix-skills-modal-data.png` / `r7fix-skills-after-apply.png`（sha b519b148 等）是 token 注入失效时的新游客空态图（『没有战绩：先打一场战斗』『收益数据不可用：本地规则引擎没有真实推荐』），**已废弃，不是本轮证据**。
- 早期『截图与弹层前一致/ME_BEFORE=ME_AFTER』是坐标点击未命中弹层所致（y=1020 > 视口 844），非业务缺陷；改原生 click 后弹层正常打开。
- 探针观测口径：本轮加载后探针差分 pe/ce/af/fr 全 0，无 play() 类噪声。

## 6. 产物清单

- 报告：本文件（`docs/laiyipao/audits/round7-bugfix/REPORT.md`）。
- 截图 6 张：`screenshots/r7fix-{skills,equipment}-{loaded-final,modal-final,applied-final}.png`（均 390x844，非物理真机）。
- 脚本/探针：`probe-init.js`/`set-token2.js.tpl`/`cta-params.js`/`state.js`/`modal-state.js`/`apply-error.js`/`me-build.js`/`diag-token.js`/`capture-final.sh`/`ownership-decisive.sh`/`skills-confirm-diag.sh`/`confirm-diag.sh`。
- 日志：`data/capture-final.log`/`ownership-decisive.log`/`skills-confirm-diag.log`/`confirm-diag.log`。

## 7. 真值纪律

- 报告、脚本、日志无任何真实域名/IP/口令/密钥；本地回环地址仅用于 QA，正式文档一律 `<YOUR_TEST_DOMAIN>` 占位。所有截图二进制不含真值（本批截图只含本地回环页面）。
