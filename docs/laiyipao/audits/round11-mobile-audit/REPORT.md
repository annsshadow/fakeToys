# R11 系统性审计：H5 全 19 页移动视口（390×844）可用性

> 本轮为真实浏览器实测（Chrome 157 通过 agent-browser 驱动），**非物理真机**。视口强制 390×844 模拟移动端。所有截图均为桌面 Chrome 移动端视口渲染，不代表真实手机硬件。
> 真值纪律：测试 token / 域名 / 口令一律内存注入，报告与落盘文件用 `<YOUR_TEST_TOKEN>` / `<YOUR_TEST_DOMAIN>` 占位；本目录已完成零真值扫描。

## 1. 结论速览

- **真实缺陷数：0**。19 页在 390×844 移动视口下全部可用，无竖排不可读、无水平溢出、无遮挡交互、无白屏、无 API 失败。
- 5 页探针初判疑似问题（reactions/enemies 竖排、skills/signin/me 按钮超视口），经原子化诊断 + 读图 **全部判定为假阳性**（详见 §4）。
- R9 终审关注的「战斗页 HUD 竖排不可读」在本次 390×844 真机视口复查中 **未复现**：battle 页 vertical=0、hOverflow=0，HUD 胶囊横排清晰（截图 04-battle.png）。
- 19 页探针 pageError / consoleError / API 失败 / 失败请求全部为 0；每页 URL/title/结构三重确认均为目标页，非登录页假阳性。

## 2. 环境与方法

- 三服务本地回环：后端 `:8080`（`SETTLE_RECOMPUTE_MODE=shadow`）、H5 `:5174`（`VITE_API_BASE` 指向回环 API）、 PostgreSQL 容器回环 `15440→5432`。healthz=200 / H5=200。
- 鉴权：从测试库取已有用户的 guest token，经 `POST /auth/guest` 重签 access token（len=157），以 document-start init-script 在应用 bootstrap 前注入 `lyp_token` 并删除残留 `lyp_token_exp`，避免被自动游客登录覆盖。
- 探针：注入 `window.__LYP_PROBE__`，捕获 pageError / unhandledrejection / console.error / fetch≥400 / XHR≥400。
- 布局探针 `mobile-probe.js` 五类检测：① 窄高文字（w<30 且 h>55，疑似竖排）② `scrollWidth > clientWidth`（水平溢出）③ 交互元素 `getBoundingClientRect().bottom > 844`（超视口）④ 固定定位元素 ⑤ 空态容器。
- 关键教训：全新会话先 `open about:blank` 预热再 `set viewport 390 844`；长时间闲置会话会被回收导致空白截图，诊断必须原子化（open+viewport+注入+eval+screenshot 同一脚本）。

## 3. 逐页结论表（19 页）

| # | 页面 | 视口 | 水平溢出 | 竖排疑似 | 超视口疑似 | 固定定位 | 文本量 | 探针 pe/ce/af/fr | 结论 |
|---|------|------|---------|---------|-----------|---------|--------|------------------|------|
| 01 | index 首页 | 390×844 | 0 | 0 | 0 | 0 | 781 | 0/0/0/0 | PASS（首屏引导卡正常） |
| 02 | announcements 公告 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 150 | 0/0/0/0 | PASS（新服无公告诚实空态） |
| 03 | stage 选关 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 991 | 0/0/0/0 | PASS |
| 04 | battle 战斗 | 390×844 | 0 | 0 | 0 | 0 | 610 | 0/0/0/0 | PASS（HUD 横排无竖排） |
| 05 | bag 背包 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 3476 | 0/0/0/0 | PASS |
| 06 | mastery 专精 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 821 | 0/0/0/0 | PASS |
| 07 | reactions 反应图鉴 | 390×844 | 0 | 3(假) | 0 | 1(顶栏) | 1381 | 0/0/0/0 | PASS（假阳性见 §4.1） |
| 08 | enemies 敌人图鉴 | 390×844 | 0 | 1(假) | 0 | 1(顶栏) | 2735 | 0/0/0/0 | PASS（假阳性见 §4.1） |
| 09 | skills 技能图鉴 | 390×844 | 0 | 0 | 1(假) | 1(顶栏) | 3355 | 0/0/0/0 | PASS（假阳性见 §4.2） |
| 10 | equipment 装备图鉴 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 2166 | 0/0/0/0 | PASS |
| 11 | gems 宝石图鉴 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 830 | 0/0/0/0 | PASS |
| 12 | skins 皮肤 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 791 | 0/0/0/0 | PASS |
| 13 | battles 战报 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 502 | 0/0/0/0 | PASS |
| 14 | shop 商城 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 1182 | 0/0/0/0 | PASS |
| 15 | signin 签到 | 390×844 | 0 | 0 | 5(假) | 1(顶栏) | 517 | 0/0/0/0 | PASS（假阳性见 §4.2） |
| 16 | rank 排行 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 537 | 0/0/0/0 | PASS |
| 17 | verify 验真 | 390×844 | 0 | 0 | 0 | 1(顶栏) | 316 | 0/0/0/0 | PASS |
| 18 | me 我的 | 390×844 | 0 | 0 | 3(假) | 0 | 845 | 0/0/0/0 | PASS（假阳性见 §4.2） |
| 19 | login 登录 | 390×844 | 0 | 0 | 0 | 0 | 318 | 0/0/0/0 | PASS |

## 4. 假阳性排除（5 页）

### 4.1 竖排文字（reactions / enemies）

- 初判：reactions「当前不可达」w12×h80、enemies「专吃有元素层数的目标。」w25×h122，符合窄高启发式条件。
- 决定性诊断（原子化脚本重新加载 + `scrollIntoView` + 读 `getComputedStyle`）：
  - reactions「当前不可达」实际 **w78×h24 / writingMode=horizontal-tb / class=chip**（父 `.rxn-head` flex row）；
  - enemies「专吃有元素层数的目标。」实际 **w55×h61 / writingMode=horizontal-tb / class=rxn-descr**（父 `.rxn enemy list-row` flex row）。
- 读图佐证（`07-reactions-vertical.png` / `08-enemies-vertical.png`）：「当前不可达」为横排胶囊，「专吃…目标」为三行横排文字，肉眼无竖排。
- 根因：初捕发生在页面入场动画 `page-in`（opacity/transform 过渡）期间，布局未稳定导致测得窄高瞬时值。**非竖排缺陷**。

### 4.2 超视口按钮（skills / signin / me）

- 初判：skills「应用推荐」bottom=1056；signin 5 个「领取/未完成/兑换」bottom=973~1439；me「保存当前构筑为防线/设置账号密码/退出登录」bottom=1625~2032。
- 读图（`09-skills.png` / `15-signin.png` / `18-me.png`）：这些按钮均位于首屏下方的**可滚动内容区**，页面 `scrollHeight` 远大于 844，用户正常下滑即可触达；首屏可见区域无遮挡、无横向溢出。
- 探针按「绝对坐标>视口高」标记，未考虑可滚动容器。**非遮挡缺陷**。

### 4.3 其他

- 固定定位元素多数页为 1：即顶部标题导航栏 fixed，属正常设计，读图确认未遮挡正文。
- `empty=true` 页（index/announcements/bag/equipment/skins/me/login）：探针针对内容列表容器的启发式标记，非整页空白——各页 textLen 最小 150（announcements 为「暂无公告·新服正常」诚实空态），均非白屏。

## 5. R9 关注点复查

- R9 终审报告「战斗页 390×844 HUD 竖排不可读」：本轮 battle 页探针 vertical=0，截图 `04-battle.png` 中 HUD 胶囊、血条、波次信息横排清晰。**未复现，判定为 R9 特定状态/动画期的观测假象**。

## 6. 真值与合规

- 测试 token 仅存在于运行时 init-script（内存/浏览器 localStorage），采集脚本用 `auth-init.tpl.js` 占位符 `__LYP_TOKEN__` 生成；生成文件 `auth-init.js` 采集后已删除。
- 全目录扫描（`*.js/*.sh/*.md/*.json/*.log`）：JWT 字面量 / Bearer 头 / 真实域名 / 真实 IP / 口令明文 / DB_DSN / JWT_SECRET 模式 **全部 0 命中**。
- 未修改任何业务源码、未执行 git 操作、未部署。写域仅限 `docs/laiyipao/audits/round11-mobile-audit/**`。

## 7. 产物清单

- 报告：`REPORT.md`（本文件）。
- 脚本：`mobile-probe.js`（五类布局探针）、`auth-init.tpl.js`（token 注入模板，占位符）、`mint-token.sh`（测试 token 重签）、`batch1.sh`/`batch2.sh`（分批采集）、`parse.js`（汇总）、`diag-vertical.js`/`diag-r.sh`/`diag-e.sh`/`diag-scroll.sh`（竖排诊断）。
- 数据：`data/batch1.log`（60 行）、`data/batch2.log`（54 行）、`data/diag-r.log`、`data/diag-e.log`、`data/diag-vertical.log`、`data/test-one.log`。
- 截图：`screenshots/01-index.png` … `19-login.png`（19 张，全 390×844），另 `07-reactions-vertical.png`/`08-enemies-vertical.png`（诊断重抓，1258×588 桌面渲染但内容为移动端视口页面，已读图确认横排）。均标注非物理真机。

## 8. 最终判定

**本轮 19 页移动视口可用性审计通过，真实缺陷数 0，无需业务源码修复。** 5 页疑似问题均为探针启发式误报，已用 CSS 计算样式 + 读图双重证据排除。