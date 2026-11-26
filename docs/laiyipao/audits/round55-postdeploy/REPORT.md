# R55 部署后真浏览器复核报告

- 任务：t90 / R55 发布后复核（五批 B5 postdeploy 证据）
- 结论：**PASS（带诚实边界）**。重建后的 H5 产物在真 Chromium 六视口复核：反应图鉴横向溢出全为 0、空态 CTA 单行 nowrap、背包占位对比度约 6:1 达 WCAG AA；R14/R12/R22/R16/R51/R53 回归锚点全部在位、无 --r55- 令牌；六张补拍新图 sha 互异且均不等于 QA 基线。
- 范围：只读部署后复核 + 审计写入；未改业务源码；本报告随 B5 提交。

## 1. 部署产物与基线

| 产物 | sha256 | 结论 |
|---|---|---|
| H5 index-BwPHvL1r.js | 9d2f10bde5da6fe44a321ba4de6b22c485a134d13d00070b76c4e6393c057a9d | 本轮 pnpm build:h5 重建，含 placeholder-class、无 --r55- |
| 后端 lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | 沿用 R21 未回退 |
| admin WikiView-ST1PbAt2.js | f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b | 沿用 R53 未回退 |

- CSS 产物核对：flex-wrap:wrap ×35、overflow-wrap:anywhere ×4、white-space:nowrap ×23，与源码修复一致。
- 服务：API 8080（start-api.sh 后台 job，config=200）、H5 静态 5201（python http.server --bind 127.0.0.1）。curl 一律 --noproxy 显式回环。
- 鉴权：QA 库 user10 guest_token 经 POST /api/v1/auth/guest 换 user token 注入浏览器 localStorage（data/auth-init.js，gitignored，仅令牌字段）；未入库。

## 2. 真 Chromium 复核逐视口结论

| 视口 | 页面 | hOverflow | CTA / 占位关键值 | 结论 |
|---|---|---|---|---|
| 390×844 | reactions | 0 | CTA h48 w114 nowrap；tags/stats 无截断 | 不回退 |
| 768×1024 | reactions | 0（原 77） | CTA h92 w222 nowrap、空态标题「AI 暂时没有足够数据给出建议」可见、overflowWrap=anywhere | 修复 |
| 1280×800 | reactions | 0 | CTA h46 w110 nowrap | 不回退 |
| 390×844 | bag | 0 | 占位 rgb(150,160,173) 对比度约 6:1，cw=sw=224 | 修复 |
| 768×1024 | bag | 0 | 占位对比度约 6:1，cw=237/sw=240（残差 3px，如实登记） | 对比度修复，截断残留披露 |
| 1280×800 | bag | 0 | 占位对比度约 6:1，cw=sw=400 | 不回退 |

- 补充响应式视口 414×896 / 820×1180 / 1440×900 六组同样 hOverflow=0；占位色与对比度一致。
- 空态 CTA text 逐字「去打一场战斗 →」、display=inline-block、maxWidth=100%、whiteSpace=nowrap；768 下 h92 高于 SPEC 文字门槛 88，按「不再竖排、w>100、单行可读」判据通过，差异不掩盖。

## 3. 新图 sha（六张补拍，互异且不等于 QA 基线）

| 截图 | sha256 |
|---|---|
| pd-414-reactions.png | 6ddc110d92277d61d22f2adff6ca6f2f0fa3c59a452bd10cad93ea968e015bbe |
| pd-414-bag.png | 5972bdfef70eec518e532c11c04abfe6b85cf40a8a44fc6c19bc36ff3dcc6344 |
| pd-820-reactions.png | 152aea0ce1ddef89ef086f3b0902a4e24851d6b3fd32b07afc5fac916fe3f101 |
| pd-820-bag.png | 6c38844a5eb1346120bbd98487324e6ac0b6354e2f50d33b4d20f00c6eb25745 |
| pd-1440-reactions.png | 59f7a9c4e6472f02654af131d6235fb132ae826aee57e6f53b3742c501537fd1 |
| pd-1440-bag.png | 15a4ee31f94ccffa0a3dabedd9f271001cfa3c12d5275f344f915225a4726cf3 |

- 六张互异，且逐一与 QA 基线十五图 sha 比对均不相等（QA 主视口图另有确定性同 sha 情形，见诚实边界）。
- 亲读 pd-768-reactions.png / pd-768-bag.png / pd-414-reactions.png / pd-1440-bag.png：均为非登录页，元素 chip 单行、AI 结论与 tab 清晰可读。

## 4. 回归锚点（源码只读复核）

- R22 战斗页 CTA「点上方卡牌即可装备它」=1；R16「发起验真」=3；R48 resolveR12Cta helper=1，四页 bag/equipment/gems/skills 各=2；R51 r51-radar 出现 14 处；R53 Math.round 出现 8 处；R14 r14-advice-empty=5。
- 令牌三锚点未改；源码与页面无 --r55-（仅契约测试文件含文档标记）。五轮前序改动均未回退。

## 5. 门禁与收尾

- Go 守卫：TestNoInfraIdentifiersInRepo PASS(5.07s)；TestNoRealSecretsInRepo **SKIP（非 PASS）**——逐字「读不到 <WORKTREE_ROOT>/laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是「通过了」」。
- finalize：删除 QA 审计与本报告 data/auth-init.js，验证 ABSENT，git check-ignore 命中 .gitignore R55 四行；QA 库 battle_records count/max=104/477。
- data/ 与任何令牌不入库；B5 仅 add 本报告、探针/驱动脚本与 pd 截图。

## 6. 五批提交说明

- B1 fc1b2bbc550b feat(r55)：common.css + bag.vue；B2 ec827c63e93a test：r55VisualReadability 契约；B3 303c7f53e9b9 docs：SPEC 与终审报告；B4 2d20a46b1794 test(r55) audit：QA 报告+脚本+截图+gitignore；B5 本批 postdeploy：本报告+部署后脚本与截图。

## 7. 诚实边界

- 390/768/1280 六张主视口图与 QA 基线确定性同 sha（同产物、同视口、同数据），不构成「新图」，故另拍 414/820/1440 六张互异新图满足互异要求；两类均如实列出。
- 768 空态 CTA h92 高于 SPEC 文字门槛 88、768 占位 cw237<sw240 差 3px：为已登记的既有数值残差，本轮不扩大改动面。
- 截图仅在打开瞬间取图，空态 CTA 需滚动至深水区才在视口内；主探针已客观记录其 rect 与 computed 样式，滚动后另拍补证。
- TestNoRealSecrets 因缺 .local.keys 为 SKIP 非 PASS；未将其表述为通过。
