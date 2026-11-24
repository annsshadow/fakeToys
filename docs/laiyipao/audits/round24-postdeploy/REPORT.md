# R24 线上回归报告——战斗实时元素层与反应触发预判 HUD

## 1. 结论

PASS —— R24 已完成三批精确提交、H5 生产包重建、三端集成验证与真浏览器决定性回归；本轮为纯 H5 只读展示派生，无后端/admin 业务源码变更。全程未发现真值入库风险。

## 2. 部署产物（测试环境）

| 端 | 产物 | sha256 | 说明 |
| --- | --- | --- | --- |
| 后端 | laiyipao/deploy/bin/lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | 沿用 R21，R24 无后端接口改动 |
| H5 | laiyipao/miniapp/dist/build/h5/assets/index-Btu5rrLb.js | 05521f3c4c9603bd28f0e937205f096fc12d63305f7286222926e131e682cf0a | 本轮 VITE_API_BASE 指向回环 API 重建 |
| H5 业务块 | pages-battle-battle.CWW3P6QO.js / battle-E8jqnkPe.css | grep -rl 命中 r24-element-hud | 确认改动进入生产 bundle |
| admin | laiyipao/admin/dist/assets/index-CGo_zhD3.js | d5e9c483992f2a370b5def55d7418da958b695deda4db9c3446e26bd04d58c70 | 沿用 R20，R24 无 admin 业务改动 |

> 测试环境域名统一用 <YOUR_TEST_DOMAIN> 占位；报告不含真实域名、IP、token、口令或带口令连接串。

## 3. 集成验证

- 后端 /healthz = 200；/readyz = 200。
- H5 根路径 = 200；admin /laiyipao-admin/ = 200（hash 路由，非 hash 直链 404 属预期）。
- QA 库 Docker 容器 lyp-qa-pg Up，端口映射回环 15440→5432；后端、H5 dev、admin dev 三服务均启动并复验通过。
- H5 构建 exit 0，bundle 内 r24-element-hud 标记在 JS/CSS 均命中。

## 4. R24 功能核心决定性证据

- 引擎 elementLayerHud() 只读快照：filter(!dead) 后深拷贝 stacks，层数 clamp，不触碰 step()/replayHash()/score/结算；静态测试 17 例覆盖深拷贝/clamp/确定性/hash 不污染，目标测试 3 文件 17 例全绿。
- H5 .r24-element-hud：40 帧逐秒采样，敌人 uid 21 个随波次变化；元素层文案从 电1 → 焰1·冰2·电2 → 无元素层 等实时变化；预判提示数 0~38 波动，非静态假数据。
- 有数据态逐字提示：『已附着 电 + 再补 焰 → 过热 反应』『已附着 冰 + 再补 焰 → 蒸汽爆发 反应』等；空态逐字：『当前场上无可预判的元素反应』，AI 结论不编造建议。
- AI 四判据全程齐备：标题『AI · 实时元素层预判』、真实数据结论、依据行『依据：敌人 stacks 元素层 × REACTION_PAIRS(源自引擎 lookupReaction) 派生；只读快照不进 replay_hash』、CTA『去释放对应元素技能 →』、诚实行『辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准』。
- hash 不污染动态实测：整局结算 settle status=200 battle_id=477；回放哈希 632ab67f900b9861 与结算页权威 replay_hash 一致，HUD 刷新未影响结算权威。
- 探针：consoleErrors=0、apiFailures=0、failedRequests=0；pageErrors 原始 294 均为无头浏览器自动播放限制噪声（无用户手势），按模式过滤真实业务错误 0。全部 40 帧 hasUndefined=false、hasNaN=false。
- 三端不回退：契约 8 用例 5 守卫覆盖选择器/逐字文案/AI 判据/诚实红线/三端不回退，全量测试 1860 通过、type-check exit 0（依赖 t190 复查证据）。

## 5. 真浏览器验证

- agent-browser 无头 Chromium 390×844（非物理真机），会话 lyp-r24-e2e，URL <YOUR_TEST_DOMAIN>/#/pages/battle/battle?level=5，真实自动选卡打到结算。
- 截图 4 张亲读均为非登录页：r24-01 战斗中数据态（游荡者 电1 + 4 条预判 + AI 卡四判据齐）、r24-02 战斗中空态（无可预判元素反应，不编造）、r24-03 过渡帧（HUD+AI 卡与选卡弹层同屏）、r24-04 通关结算页（权威 replay_hash=632ab67f900b9861、AI 复盘卡、查看本局战报）。
- 页面错误过滤口径已明确记录，未掩盖任何业务异常。

## 6. 截图 sha256（入库版，两两不同）

| 截图 | sha256 |
| --- | --- |
| r24-01-hud-live-a.png | 3f9560ac52da963997066c42b16dd4109428afa725368497302907ac9defbe40 |
| r24-02-hud-live-b.png | 56981216f3a07c127d40c7d5ceda65767ebe2cf30dfc2ab792df5936dae9feee |
| r24-03-settle.png | c6e9a0aea239f0ad12fbe1416d6428ca7b3215cd41e1630522b71939b2212de1 |
| r24-04-settled.png | b440258a683132d5a2aa676080d04ac5dd9d666b96890c5b492d7f706a0fd0a7 |

## 7. 真值与收尾自检

- 三批暂存逐批 git diff --cached 严格正则自检：批1 feat 2 文件 125 行 LEAK=0；批2 docs 3 文件 138 行 LEAK=0；批3 test 14 文件 720 行 LEAK=0 DATA=0 AUTHJS=0。
- 严格正则覆盖：JWT 形态、guest_token 字面值、Windows 用户目录形态、root 加 IP 登录形态、私钥头形态、带口令连接串、引导口令赋值形态、真实域名/IP。
- 含真 token 的 auth-init.js 与 data/ 快照/样本均被 .gitignore 第 319–320 行排除且从未入库；仅 __LYP_TOKEN__ 占位模板入库。报告本身经严格正则扫描 0 命中。
- 本地分支前三批 commit eff483549 / da848aca9 / 2ef7a6738，AB=3/0（push 前），工作树 tracked 干净。
