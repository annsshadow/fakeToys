# R61 form-a11y 部署后真浏览器复核报告

## 1. 发布概要

- 任务：t115「R61 五批发布：v-aria-native 桥接修复」，worker same-route-rel，attempt=a32ea6e7-1ee0-42a6-bd72-001b4012f05c。
- 起点 HEAD=eead131e63ed3edad48d79832843547132426011（R60末），分支 opt1000/laiyipao-ai-refactor，author annsshadow。
- 本轮修复：main.ts 新增 v-aria-native 指令，把 12 页共 17 处外层 aria-label 桥接至原生 input；反馈节点补 role=status/alert。
- 五批提交（精确 git add 逐文件路径，禁 -A/.，每批 LEAK=0 / diff--cached--check=0 / data 入库=0）：

| 批次 | hash | 说明 |
|---|---|---|
| B1 | d27d034d7 | feat(r61) main.ts + 12 页 vue（13 文件） |
| B2 | d2481517d | test(r61) r61FormA11y.contract.test.ts（155 行 5 it） |
| B3 | 3ddf76dab | docs(r61) SPEC.md + ROUND61_DESIGN_REVIEW.md |
| B4 | 657f649b8 | test(r61) audit .gitignore + 40 根脚本 + 73 截图（114 文件） |
| B5 | 见 §6 | postdeploy(r61) 本报告 + 探针/驱动 + 12 截图 |

## 2. 产物 sha（部署后不回退）

| 产物 | 文件 | sha256 |
|---|---|---|
| 后端 | laiyipao/deploy/bin/lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 |
| admin | laiyipao/admin/dist/assets/WikiView-ST1PbAt2.js | f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b |
| H5 | laiyipao/miniapp/dist/build/h5/assets/index-B_1tAtI5.js | 5c8bbd3423c1c09b3bfca71776024159142d4644197e3bf1596d1cc5ae972f52 |

- H5 已 `pnpm run build:h5` 于 laiyipao/miniapp 重建（RC=0），index.html 引用 index-B_1tAtI5.js（QA 阶段已构建该确定性产物，重建 sha 不变）。
- 任务描述猜测的 admin 产物 WikiView-JpQ4MFXA.js 在本工作树不存在，以实际唯一文件 WikiView-ST1PbAt2.js 为准，sha 未回退。

## 3. 部署后真浏览器复核（Chromium，API 8080 + H5 静态 5201）

### 3.1 三视口 × 11 页 × 空/填两态探针（37 组 × 2 = 74 次）

- 视口：390×844 / 768×1024 / 1280×800；页面：announcements/login/verify/signin/me/bag/equipment/enemies/gems/reactions/skills/skins（12 页 × 3 = 36 组）+ me 展开补证 1 组 = 37 组。
- 结果：`nativeAriaAll=true` 空态 37/37、填态 37/37；`hOverflow=0` 37/37；`inputCount>0` 34/37（3 组无可见 input 属条件渲染空页，非失败）。
- 逐字证据：login 输入 nativeAria=「用户名」「口令」，feedback role=status「请填写用户名与口令后可登录」；ann 「搜索公告标题 / 正文」；verify type=number 输入 nativeAria=「battle_id」（AX 归 spinbutton，按终审更正口径判 PASS）；me 展开输入「用户名」「密码（至少 10 位）」。
- me 展开补证：UPG=CLICKED_UPGRADE 后已捕获 2 处带名输入（含密码框）；PWD=NO_BTN（游客态下再次点击升级按钮不可见，未触发二次触发脚本，如实披露；2 处带名已足够坐实 me 桥接）。

### 3.2 交叉视口 12 张新截图（414×896 与 1440×900 × 6 页）

| 截图 | sha256 |
|---|---|
| pd-414-announcements.png | c5907c01481f29c4d8257edd84cab8217e50c2bce308490ea6db878e0d723f75 |
| pd-414-login.png | 223c6a42eafee05781641c287259e217f8283385d7098bb4da489938242c813f |
| pd-414-verify.png | 429f8890ff2c3fd7e4660790a02cf1711b9f7772e57e768338baccadaf668f47 |
| pd-414-me.png | c6cac9ac1caed1c8a75b0b8c22872a482f04ede8a001de3369d7b0bb123a2fd4 |
| pd-414-bag.png | 8527d708015b8c52b80e232f5a84e6ae7a96ef4233447daee3888d45fb19e91d |
| pd-414-reactions.png | ca22fd8f71a091e386e41654981a4f42a5e7d43f156eb40f43cb401aeac4b84e |
| pd-1440-announcements.png | d2334ddff5ec83d893c02bba5714aa396eaad6ff66a0e2f9876e0662a572f599 |
| pd-1440-login.png | 5094f860359fb55529a52c6a19dabb892b2a6835a79442493a6b5a40647d03ae |
| pd-1440-verify.png | 41fc4dd05e79cd7cdea8dc0e98f0e88fcfab1e1ea940c90804f6cb4223ed7f31 |
| pd-1440-me.png | 0f4692170212b34e4e291bb8fd6ffd7dad3533b550879ca4ee57cb57670a53f6 |
| pd-1440-bag.png | d6116eaae32dbfaeae85ce50cf942e028abb35f305e0e58fd0f5c0c21fb65a27 |
| pd-1440-reactions.png | 1edfd19df1dd3b2de82425a5ce064bb56e4f448b5eb4c55801166a0b9421a758 |

- 12/12 sha 互异；与 QA 基线 37 张 pf-*.png（§ 3 QA REPORT / data/pf-sha256.txt）逐一对账，全部不同（414/1440 交叉视口本就不在 QA 基线 390/768/1280 三视口内）。
- 亲读 3 张非登录页：pd-414-login（表单填写态+校验反馈）、pd-1440-me（指挥官档案+R51 雷达+AI 结论）、pd-414-verify（战斗验真+输入 battle_id），均为真实业务页。

### 3.3 回归锚点

- R60 role=button 键盘可达、R51 me 雷达五维、R53 首页 pct 取整、R55 视觉可读性、R43/R44/R46/R47/R48 各锚点在部署后页面均可见、未回退。
- 令牌三锚点（tokens.css:12-14）原样保留；全 src 无 --r61- 业务使用（仅契约否定断言 2 处）。

## 4. 门禁与收尾

- Go：TestNoInfraIdentifiersInRepo PASS(8.22s)；TestNoRealSecretsInRepo SKIP 非 PASS（缺 .local.keys，逐字「真值扫描这条守卫没有执行，不是『通过了』」）；GO_RC=0。
- QA 库容器 lyp-qa-pg：battle_records count|max(id)=104|477。
- .gitignore 561 行末尾无空行；R61 六条规则完整（553/554/555/556/560/561）；两处 data/auth-init.js 已被规则命中，finalize 后删除。
- git ls-files 无本轮真值：form-a11y 与 form-a11y-postdeploy 的 data/ 与 auth 均未入库。

## 5. 诚实边界

1. TestNoRealSecretsInRepo 缺 .local.keys 为 SKIP 非 PASS，不冒充通过。
2. me passwordMsg 真机未二次触发（游客态 PWD=NO_BTN）；依 me-expanded 捕获的 2 处带名输入与 me.vue:601-610 源码 role=status 采信。
3. verify type=number 的 AX 角色为 spinbutton「battle_id」而非 textbox，终审已更正探针口径；本报告按 spinbutton 判 PASS。
4. 原始 RED 日志未留存，依 SPEC 中记录的 2 轮 RED 与最终 GREEN 5/5 采信。
5. 3 组视口页面无可见 input（条件渲染空页），以其余 34 组坐实桥接覆盖。
6. QA 会话侧 guest_token 注入仅在浏览器会话层，不改 DB；QA 库维持 104|477。
7. B4 曾因 POSTFIX_REPORT.md 末尾空行触发 git diff--check RC=2，已精确删除该空行后重提，终态 check=0。

## 6. 结论

本轮 R61 v-aria-native 桥接修复五批发布完成：H5 重建、部署后 37 组双态探针全绿、12 张互异新图、回归锚点不回退、产物 sha 不回退、零真值入库。结论 PASS（带 §5 诚实边界）。
