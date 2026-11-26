# R27 部署后独立证据补采（EVIDENCE）

任务：t212。目的：修复 round27-postdeploy/REPORT.md:40 所述「E2E 重拍后已 git checkout HEAD 恢复为入库版」导致仓库内 5 图 SHA 与部署前 QA 批 2 完全相同、无法证明部署后真实状态的问题。本次从零重跑真实浏览器 E2E，独立采集 5 图与 DOM 采样，不复用、不复制、不 checkout 任何旧图。域名统一占位 <YOUR_TEST_DOMAIN>；真值零入库零回显。

## 1. 部署对象与采样时间（验收①）
- 采样起点：2026-11-24T20:18:42+08:00；工作 HEAD=28ccc2559。
- H5 bundle：laiyipao/miniapp/dist/build/h5/assets/index-BLjjQbiy.js sha256=08b98caf6335f5f3cd73c277f9402fb83ba4966932d57ab97129894e61f3e0c0。
- H5 index.html：sha256=0f5700f9f2a005369defc820729e8b9ec66edca89b3dcb0bb2f9d810b8040ebe。
- 后端产物：laiyipao/deploy/bin/lyp-server-amd64 sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R27 无后端改动，沿用 R21）。
- 采样时服务：API 本地回环 :8080（/api/v1/config=200、/healthz=200），H5 本地回环 :5174（=200）。
- 记录文件：data/deploy-env.txt（入 gitignore，不入库）。

## 2. 真实 L5 通关（无 shim，验收①）
- agent-browser 无头 Chromium 390×844（非物理真机）；session=lyp-r27-postdeploy，经本地回环访问，document-start 仅注入 guest token（data/auth-init.js 用完即删）与只读错误/响应探针，无业务代码补丁、无 shim。
- 真实 click：#e2e-full.sh 循环 86 帧到 SETTLED；CARD_CLICKS=4、SKIPS=0；四时点截图触发标记 W1=1/MID=1/CLEAR=1/NEXT=1。
- settle：status=200、battle_id=478；结算 DOM result=通关、hash="回放哈希 7750f3577ce041be（可验真）"；hashResponses=[]（HUD 未额外触发 hash 端点）。
- 结算战报 id=478 为本次真实 E2E 产生，收尾已 pg_restore 还原清除（见 §7）。

## 3. 截图 SHA256 独立性与亲读（验收②⑤）
本次 5 图（screenshots/postdeploy-r27-*.png），两两 SHA 不同，且与部署前 QA 批 2 五图 SHA 全部不同：

| 图 | 本次 SHA256 | 部署前 QA | 是否相同 |
|---|---|---|---|
| 01-first-wave | b1f82c45fafdb5df95f7f73c068e78d3442187f4499a1cc8b10b040665b32024 | 1747bcd7… | 否 |
| 02-mid-wave | b1dffd321871037e38fd4fa57ef79cef07d8b12f5989b6a6916eb389be5c11d0 | 0816cb98… | 否 |
| 03-clear-100 | a092e1601206f729926b5f520859e38b3f7bad67066afa713dc49be4e5ec36d2 | 73ca5f4c… | 否 |
| 04-next-wave | 7e0cf3f75a31905e2361e92af4ccea22f1ca5b95708e6e6cc48a9b3c6270a88c | 2ab01640… | 否 |
| 05-settle | ad439fe5f365444a7cd3d4b670c04ff59a227818c758c701b21f85a1f31d6f91 | 06ea3c75… | 否 |

- 5 图均已亲读：390×844，战斗/结算界面非登录页。01 首波 1/3·33.3% + AI·波次推进四判据 + R24 元素层 + R26 下一波简报；02 中途 3/5·60.0%；03 清波 3/3·100.0% + 选卡 panel；04 下一波 0/5·0.0%；05 通关结算分数 8634、击杀 36、反应 70、回放哈希、AI 赛后卡。
- 仓库内现存 5 图即本次新拍，未做 checkout HEAD 覆盖，REPORT.md 未改（out of scope）。

## 4. DOM 逐字段比对（验收③）
采样 data/postdeploy-samples.jsonl 共 86 帧（双层 JSON.parse）。按 waveIdx 聚合：

| 波 | domTotal | resolved 取值 | pct 范围 | 100.0% |
|---|---|---|---|---|
| 0 | 3 | 1,3 | 33.3–100.0 | 是 |
| 1 | 5 | 0,1,3,4,5 | 0.0–100.0 | 是 |
| 2 | 6 | 0,4,6 | 0.0–100.0 | 是 |
| 3 | 8 | 0,1,3,5,6,8 | 0.0–100.0 | 是 |
| 4 | 14 | 0,5,6,7,9,11,12,14 | 0.0–100.0 | 是 |

- total 3/5/6/8/14 与权威 /api/v1/levels/5 spawns.count 聚合逐字段一致（config 不下发 waves，权威波次端点需 Bearer，仅用于比对，不在浏览器侧伪造）。
- 清波 FULL 帧 7 帧（如「本波 3/3 · 100.0%」）；下一波归零 ZERO 帧 25 帧（如 idx=1 total=5 pct=0.0）。
- progText 严格 x.x% 一位小数：BAD_PCT_FORMAT=0。fillW 实测为文本百分比（如 33.3%），与 progText 百分比一致。
- AI 四判据逐帧齐（86/86）：①aiTitle 含「AI·波次推进」②aiConcl 真实数据结论「本波共 N 只，已处理 M 只，场上 X 只、待出场 Y 只，清理进度 Z%」随 DOM 变化 ③aiBasis 逐字「依据：引擎 waveProgressHud() 只读快照（当前波 spawns 总数 − 场上存活 − 待出场）；只读派生不进 replay_hash」④aiCta 逐字「继续清理场上敌人 →」。诚实红线逐字「辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准」（86/86）。
- R24 元素层 86/86 帧可见、R26 简报在选卡态 5/5 帧可见；hudWrap 实测全帧为 "wrap"（flex-wrap:wrap 保留，R9 窄屏布局未回退）；docOverflow 全帧 false（无横向溢出）；hasUndefined/hasNaN 全帧 false。
- 空态逐字「本波暂无敌人进度数据」在 battle_r27_wave_progress.contract.test.ts:113,146 守卫；L5 每波均有敌人，浏览器侧不可达，如实标注不伪造空态截图。

## 5. settle / hash / 探针原始采样（验收④）
- data/postdeploy-final.json：result=通关、settle=[{status:200,battle_id:478,replay_hash:""}]、hash="回放哈希 7750f3577ce041be（可验真）"、hashResponses=[]、pe=71、ce=0、af=0、fr=0。
- data/postdeploy-allpe.json：pageErrors total=71、noise=71、real=0；71 条全为 Chrome Autoplay 限制噪声「The play() request was interrupted by a call to pause() …goo.gl/LdLk22」，与 R24/R25/R26/R27 同口径，过滤后真实错误 peReal=0。
- apiFailures=0、failedRequests=0、consoleErrors=0。hash 只读派生不污染：R27 引擎 waveProgressHud() 仅读 spawns/enemies/spawnQueue 返回新对象，不写 step/record/score/replay_hash（契约源码守卫 + 结算 hash 可验真）。

## 6. 零真值、临时脚本清理（验收⑤）
- data/auth-init.js（含本次临时 guest token）已删除，仅留 auth-init.tpl.js（__LYP_TOKEN__ 占位符）；AUTH_EXISTS=NO、TPL_EXISTS=YES。
- .gitignore 追加两行：docs/laiyipao/audits/round27-postdeploy/data/ 与 docs/laiyipao/audits/round27-postdeploy/auth-init.js（紧跟 R27 两行后）；git check-ignore 确认 qa-before.dump、postdeploy-final.json 被忽略，screenshots 可入库。
- 零真值正则扫描（JWT 片段 / Authorization 头 / 带口令连接串 / IP 模式）0 命中。部署环境记录仅回环地址与 <YOUR_TEST_DOMAIN> 占位，无公网域名/IP/口令/个人路径。

## 7. QA 库还原（验收⑤）
- 快照：data/qa-before.dump 289612B sha256=a215db237025cb07b433c91b74e1a55e475a0eb8832d7ab80a1abab1f1ce9dd3（采样前 pg_dump）。
- 采样后 pg_restore --clean --if-exists 还原；验证 battle_records=104、id=478 计数=0、max_id=477，与 baseline 一致。

## 8. 范围与结论
- 未改任何业务源码：git diff --stat -- laiyipao/ = 0 行；未提交未推送；仅新增本目录 EVIDENCE.md + 5 图 + 审计脚本/采样（data/ 被忽略）。REPORT.md 保持原样（out of scope）。
- 六项验收全部满足：部署 bundle 哈希与采样时间可追溯、真实 L5 无 shim 通关、5 图 SHA 两两不同且全≠部署前、DOM 四时点逐字段一致且 AI 四判据齐、settle/hash 与探针 peReal=0 有原始采样佐证、EVIDENCE 完整且零真值/token 清理/QA 还原。
- 判定：PASS。局限：agent-browser 为无头 Chromium 非物理真机；空态浏览器不可达以契约补证如实标注。
