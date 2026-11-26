# R28 部署后独立回归报告（round28-postdeploy）

- 轮次：round28（实时反应feed波次/敌人归因）
- 证据口径：本报告截图与 DOM 采样均来自**部署完成后**另起的 agent-browser 真 Chromium 无头会话（viewport 390×844，本地回环），与部署前 QA 目录 round28-live-reaction-attribution 的 5 张截图 SHA 完全不同，互不冒充。
- 浏览器口径：真实无头 Chromium 390×844，**不是物理真机**；所有 DOM/几何/截图来自该真实浏览器实例。
- 关联任务：t221 / attempt_id a8c224df-2a71-4df5-b947-846eb9324d9d（release-ops-wb，唯一提交者）。

## 一、部署产物与三端健康

- H5 产物重编：pnpm build:h5 退出 0；dist/build/h5/index.html 引用 assets/index-YVN5vGL0.js；assets/pages-battle-battle.DtmrVfoZ.js 与 assets/battle-C__oYqQg.css 中含 R28 选择器命中：r28-attribution×2、r28-feed-ai×10、r28-feed-cta×2。
- 后端沿用 R21 已部署二进制 lyp-server-amd64（sha256 前缀 3d82635f…），admin 沿用 R20 已部署 index-CGo_zhD3（sha256 前缀 d5e9c483…）；本轮 out of scope，未改、未回退。
- 三端健康：GET /healthz=200、/readyz=200、H5 5174=200、admin 5175/laiyipao-admin/=200。
- git 四批提交：abdac72f2 feat / b80136a05 test / 7453298c6 docs / 6ad6c83a3 audit；逐批 STAGED_LEAK_TOTAL=0、git diff --cached --check 全 0。

## 二、真浏览器部署后独立回归（L5）

- 会话 lyp-r28-postdeploy，--init-script 用占位模板生成的注入脚本（仅本地，最终已删），无业务 shim；真点击 101 帧至 SETTLED，CARD_CLICKS=4，SKIPS=0。
- 状态分布 WAIT=95、CARD_PHASE=5、SETTLED=1；lineCounts 101 帧全 =3（≤3 行门禁满足）。
- 归因：attribOk=303=101×3，全部行 data-r28-wave/data-r28-enemy 与文字「第N波 · #uid」逐字一致；unknownRows=0（不可用分支由部署前契约测试覆盖）。
- AI 四判据：aiTitleOk=101、basisOk=101、honOk=101、ctaOk=101；AI 分母=DOM 行数 ratioOk=101/101（每帧 3/3）。逐字依据行与红线均吻合规范。
- 共存：R24 元素层 101/101、R26 下一波简报 5/5（选卡期）、R27 波次推进 101/101；hasUndefined=0、hasNaN=0；docOverflow=0。
- feedOverflow=true：与部署前 QA 报告一致，是 HEAD 基线既有 6px 反应行几何，非 R28 引入、非本轮阻塞；建议后续轮次单独修基线宽度。
- 结算：result=通关，hash=回放哈希 2e0b4b7f5ecc29fe（可验真），settle=[{status:200,battle_id:478,replay_hash:""}]，hashResponses=[]；replay_hash 字段为接口空串，如实记录，不臆造。
- 探针：consoleErrors=66 全部为 Chrome Autoplay 策略噪声（play() request was interrupted by a call to pause()），按过滤规则 peReal=0，apiFailures=0，failedRequests=0。

## 三、CTA 战斗态真点击（WAIT）

| 探针 | state | helpVisible | lineCount |
|---|---|---|---|
| cta-pre.json | WAIT | false | 3 |
| cta-post.json | WAIT | true | 3 |
| cta-closed.json | WAIT | false | 3 |

即点击 .r28-feed-cta 真打开战场图例覆盖层，再点 .legend-close 真关闭，三态全在战斗 WAIT 态，非结算态冒充。

## 四、部署后截图证据（4 张，sha 两两不同；亲读 390×844 非登录页）

| 文件 | sha256 | 内容 |
|---|---|---|
| screenshots/postdeploy-r28-01-attribution.png | 1fc581dcb64487212c50c6da514b45aa79ce065b6838930206f5657b608eb2b6 | 战斗中三行归因 + AI 归因说明块 |
| screenshots/postdeploy-r28-02-ai.png | 17a42195eb30592c98a6d49055df5b8f61ed51b08988f7848dad505580a622dc | 战斗中反应 feed 与 AI 四判据 |
| screenshots/postdeploy-r28-03-cta-open.png | d45843e09635960a3c8152a36d43769ef5daf235e8b059fe92842477400e9b43 | 战斗态 CTA 打开战场图例覆盖层 |
| screenshots/postdeploy-r28-04-settle.png | a0e8eb10ddc3207dfd36c24910ce7f33955954ecc5c217c792c66a374d0505f8 | 通关结算页，回放哈希 2e0b4b7f… |

与部署前 QA 五图 SHA（9216925c…/dc02d8b6…/b6fd9ccc…/321b2095…/dc6a59cc…）无一相同，满足「不复用部署前证据」。

## 五、零真值与审计卫生

- 注入脚本 auth-init.js 由占位模板生成、仅存在于被 gitignore 的工作目录；finalize 后 AUTH_DELETED=YES、TPL_EXISTS=YES。
- .gitignore 新增 :343-344 拦 round28-postdeploy/data/ 与 auth-init.js；git check-ignore 对 qa-before.dump 与 auth-init.js 均命中。
- 按团队既定真值严格正则（JWT/令牌串、私钥、带口令连接串、敏感环境变量等模式）扫描所有待入库 js/sh 文件，0 命中；正则模式本身不写入仓库，避免自指误报。
- QA 库还原：从 round28-live-reaction-attribution/data/qa-before.dump（sha256 0abe598fe898b410ead7d8f99c518e019f34244cecec9ddc5e29b9d9acaa31d2）pg_restore --clean --if-exists 退出 0；battle_records 由 105/max_id=478 还原为 104/max_id=477，id478=0，与测试前一致。

## 六、结论

部署后独立真浏览器回归满足全部核心门禁：3 行归因、AI 分母=显示行数、CTA 战斗态真点击、settle 200、截图独立且亲读、探针无真实错误、零真值、QA 还原。基线既有 6px 反应行溢出如实保留为后续事项，不属 R28 引入。判定 PASS。
