# R467 WCAG 2.2.1 Timing Adjustable 时间限制可调整甄别报告

- 任务：t327 [R467]（attempt 67939afc-bfa5-41c8-a035-365dc4153f08）
- 类型：只读甄别，不改任何入库源码/契约/台账
- 标准：WCAG 2.2.1 Timing Adjustable（A 级）——凡有时间限制的内容，须至少提供关闭、调整（放宽至 10 倍）或延长（在到期前警告并允许至少 20 秒延长、可重复）三者之一；**认证会话超时（Essential 例外）与超过 20 小时的时限不适用本条**。
- 测试来源：Playwright Chromium headless 真浏览器实测（非物理真机）；PC 1440x900 DPR1 双主题、H5 390x844 DPR2 isMobile/hasTouch
- 证据目录（gitignored）：oa4rust/test-results/r467-timing-adjustable/；域名/凭据真值仅存在于 gitignored runner 与运行时环境变量，不入库、不回显；文中测试服地址一律占位。

## 1. 审计范围与判据

WCAG 2.2.1 针对的是**用户正在阅读/操作的内容上的时间限制**（如限时答题、限时编辑、自动登出前的倒计时、会话即将过期的告警）。判据三分：

- **认证必需豁免（Essential）**：会话到期后要求重新登录，属安全实践，本条明确豁免；只要重新登录可达，就不是缺陷。
- **>20 小时豁免**：时限超过 20 小时的内容不受本条约束。
- **正常-规范预期**：无时间限制、或有时间限制但提供关闭/调整/延长机制、或时限属上述两类豁免。
- **真实退化（可达缺陷，标 P 级）**：非认证类、<20 小时的时限，且无关闭/调整/延长机制、承载信息、用户在真浏览器可实际遭遇。

**严禁把认证会话超时误判为缺陷**——这是本轮最关键的定性红线。已覆盖维度严禁重复：对比度 R458–R466 系列、骨架 R453/R456、焦点/标题层级/表格/响应式、动画暂停 R466、悬停内容 R465。

## 2. 静态盘点：时间限制来源全库搜索

### 2.1 倒计时/限时操作类——零命中

对以下关键词做全库源码搜索（ts/vue/js/jsx）：

| 关键词组 | 命中 | 结论 |
| --- | --- | --- |
| 倒计时 / remainingSeconds / countdownSeconds / countdown | 0 | 无倒计时限时操作 |
| expiresIn / expires_at / tokenExpires / refreshToken / refresh_token | 0 | 前端无令牌到期倒计时 |
| timeLimit / deadline / expiration | 2 | 均为 ProcessDesigner.vue 的字段名（condFields/nodeVars），非限时机制 |
| 即将过期 / 即将失效 / expiring / about to expire / 延长 / 续期 / 保持登录 | 1（仅测试注释） | 无"会话即将过期"提示、无延长按钮组件 |

结论：本库**不存在任何面向用户的倒计时、限时答题、限时编辑、即将过期告警或延长/关闭/调整控件**。2.2.1 的"三选一机制"在本库无适用场景。

### 2.2 会话超时配置——双重口径，均属豁免

| 配置 | 位置 | 值 | 单位与判定 |
| --- | --- | --- | --- |
| 后端会话 TTL | oa4rust/backend/crates/shared/src/config.rs:9 | DEFAULT_SESSION_TTL_SECONDS = 7200 | 7200 秒 = 2 小时；会话 cookie max-age 同值（auth/src/lib.rs:27） |
| 后端会话到期计算 | oa4rust/backend/crates/shared/src/session.rs:209 | expires_at = now + Duration::seconds(session_ttl_seconds) | 与上方同一 2h 常量 |
| 前端安全配置默认值 | oa4rust/frontend/apps/desktop/src/views/Settings.vue:41/:101 | sessionTimeout: 7200 | **7200 分钟 = 120 小时**，远超 20h 豁免线 |
| 单次 HTTP 请求超时 | oa4rust/frontend/packages/sdk/src/api.ts:25 | DEFAULT_TIMEOUT_MS = 30_000 | 30 秒网络超时，非用户时限 |
| 无关命中 | vite.config.ts:45 | max_age: 86400 | CSP Report-To 头参数，非会话 |

后端 2h 会话到期属**认证必需豁免**（安全实践），且前端配置口径为 120h（超 20h 豁免），两种口径都不构成 2.2.1 缺陷。

### 2.3 会话失效处理——具备自动续期延长机制

- oa4rust/frontend/packages/sdk/src/api.ts:278-290：业务请求收到 401 时，先调用 refreshSession()（POST /api/authentication/refresh），成功则 retried 重试原请求（会话无缝续期）；refresh 也失败才走 authenticationFailed()。这就是 2.2.1 意义上的"延长"机制。
- api.ts:215-225 authenticationFailed()：认证失败唯一出口；cause instanceof AuthenticationError 透传避免重复通知；isLoginRequest() 区分登录接口 401（凭据错误）与业务 401（会话过期）。
- oa4rust/frontend/packages/sdk/src/session.ts:48-51（桌面）/ oa4rust/mobile/src/store/session.ts:27-30（移动端）：setAuthenticationFailureHandler 在最终失败时清本地 user，视图回退到登录态（重新登录可达）。
- oa4rust/mobile/src/services/http.ts:140：移动端同为 401→refresh→retried→失败才清会话。
- 测试钉死：oa4rust/frontend/packages/sdk/src/api.login-auth-failure.test.ts 与 api.error-mapping.test.ts 断言"401 不得误判为普通错误，否则自动续期永久失效"。

## 3. 真浏览器实测

### 3.1 时间限制探针（21 组）

PC 双主题 8 页（工作台/组织架构/文件/论坛/日历/会议/考勤/公共组件库）× light+dark = 16 组，H5 5 页（工作台/我的/消息/日历/收藏）锁浅色 = 5 组。逐页扫描正文时间相关文本（含倒计时/过期/超时/延长/会话等中英文关键词）与时间类元素（[class*=countdown]/[class*=timer]/[class*=expire]/[role=dialog]）：

- **时间相关文本命中：0/21**。唯一元素命中是路由切换无障碍播报 [aria-live]"已切换到：工作台/组织架构/…"（16 组 PC 各 1 条），这属于路由播报无障碍维度，与时间限制无关。
- 0 pageerror。

### 3.2 会话超时（401）强制失效模拟（2 组）

通过 Playwright route 强制业务 API 与 refresh 均返回 401（业务 401 命中 PC 7 次 / H5 3 次），观察 2.5 秒：

| 观察项 | PC | H5 |
| --- | --- | --- |
| 会话被清空（头像 admin → 用户） | 是 | 是 |
| 业务区提示 | "待办加载失败：Session expired, please login again" + "重新加载待办"按钮 | 统计数字降级为"-"，页面骨架完整 |
| 自动跳转登录页 | 否（原地降级，重新登录路径可达） | 否（原地降级） |
| 即将过期倒计时/延长按钮 | 无 | 无 |
| pageerror | 0 | 0 |

目视 session401-pc.png：会话失效后头像降级为"用户"、待审批数字变"-"、业务区红字提示"Session expired, please login again"并提供"重新加载待办"按钮——这是会话清空后的正确降级，用户可重新登录恢复，符合认证必需豁免。session401-h5.png：统计卡数字变"-"、九宫格入口与底部 tabbar 完整，无崩坏。

该模拟同时坐实：会话失效路径是"401 → 续期尝试 → 失败 → 清会话 → 降级/重登录"，并非静默丢失内容；续期成功时（正常登录态）用户无感知。

## 4. CDP 根因坐实

对每页执行 CDP DOM.enable + CSS.enable + DOM.querySelector + CSS.getMatchedStylesForNode：

- PC：[class*=countdown]/[class*=expire]/[role=dialog] 全部 found=false（0 规则）；[aria-live] found=true（3 规则，路由播报）；.ai-gradient-text found=true、.ai-panel found=true（视觉类，无关时限）。
- H5：[class*=countdown]/[class*=expire]/[role=dialog] 全部 found=false；.ai-brand-fill found=true（6 规则）；.ai-gradient-text found=false（H5 零消费，与 R451/R453 一致）。

CDP 与静态搜索、E2E 探针三方一致：本库无任何时间限制 UI 或倒计时机制。

## 5. 三分判定

- **可达真缺陷（P 级）：0**。全库无倒计时/限时操作；会话 2h TTL 属认证必需豁免且有 401 自动续期延长机制；前端配置 120h 超 20h 豁免；会话失效后重新登录可达（真浏览器实测降级 UI）。据实判 0，不臆造。
- **不可达 L1：0**。本轮无"源码存在但业务条件恒不满足"的时间限制候选（"即将过期"提示/延长按钮本就不存在，属无适用场景，非存在但不可达）。
- **正常-规范预期：若干**。会话 TTL 认证豁免 / 401 自动续期延长 / 重登录可达 / 配置 120h 豁免 / 无倒计时组件 / 路由播报 aria-live 属其他无障碍维度。

## 6. 截图 sha256（命令行原始值，23 张）

会话失效模拟：

    1f78dc6111304d0bf44abe76645eac11ba320ebfb49ab24a5ce1a3c64fb46dd0  session401-pc.png
    7b4129897a8967e98cff8475673d20916cd3c8285f8a5469f7253036982d9c66  session401-h5.png

PC 双主题（16 张）：

    f501cd4fae3995409104053d54b9756ead34c9ebd892f3213ad0b9345d9e0d9c  pc-dashboard-light.png
    8e7ad6a70d82e4ee2aaf9c1f3a6129872a2eac3e9e383f6c9e281484c63f7745  pc-dashboard-dark.png
    e6660e9702b990795f1c5a852f7505011642073e3c429ec7ce6b63727dfb1e7b  pc-org-light.png
    887da0a8cbdb5e02ad1b6aebbbaa7b849f087bd14426b2a46b6ef05040f11738  pc-org-dark.png
    066053df18c5f1c676bd606d31b2a046745444692e2e06ce106958f71fe7547f  pc-file-light.png
    d7ebe750e0bf26efbca0896ab619ae7dae03e2af697ff92616fa43b1c35b9875  pc-file-dark.png
    888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105  pc-bbs-light.png
    6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c  pc-bbs-dark.png
    8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad  pc-calendar-light.png
    0075d87d57dc86a5bc3963fa095ecbabaa06562542fbaea5922b2234fe9f6c4b  pc-calendar-dark.png
    94c4840fbd266eeaae6c3287abe09f1d0e3b7e874f6e35ef8a358d73fd891b26  pc-meeting-light.png
    9ff0653f9e93a36a3273b637fd88bbc16839dad876290b1025a8a564849ea774  pc-meeting-dark.png
    678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b  pc-attendance-light.png
    85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7  pc-attendance-dark.png
    33f73040fdde480b96841e417ee4fee1d06884845ba16e4303365e3d9b0db073  pc-common-light.png
    afe79b29a25873e32fabc99ddcb7729ec6aab8faffcec317d88c0adfaca33424  pc-common-dark.png

H5 浅色（5 张）：

    c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96  h5-index.png
    cf939600d8b263fc01f5d09a73d2f24fa3f3877929c019aaaf5072c3ad025385  h5-mine.png
    170482f3818cb89f3b3fd21adf4971b65a72ef834b2b270a600c1e736800b3d4  h5-message.png
    95af2b84a08af75e603a927b7e757e5cf132f6a848c2cd26b82b9fd1e6f6f1fc  h5-calendar.png
    cbae5e3d08c81c9b2276a9f3c103d6b0094494f6dbda229b8a1f66a2276f0ed6  h5-collect.png

截图来源：Playwright Chromium headless 真浏览器，非物理真机；sha256 由命令行 sha256sum 生成并与 report.json 内运行时 sha 逐一核对一致。

## 7. 真值纪律

本文件不含任何真实测试域名、IP、主机名、SSH 端口、口令或凭据；测试服地址统一以 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位；凭据由部署环境的 .local.keys / 环境变量在运行时注入，仅存在于 gitignored runner 与进程环境，不入库、不回显。证据截图与 report.json 位于 gitignored 的 oa4rust/test-results/r467-timing-adjustable/，其中的 urlTail 字段为运行时环境真值，不纳入本交付文档。
