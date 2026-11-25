# R518 WCAG 2.2 SC 3.3.8 Accessible Authentication（验证码转录）甄别报告

## 1. 概况
- 任务：t1（attempt 633a777c-9d7f-40f6-aafb-338d4aa0a571），真父 447ce21d2dbc425ecc5efbb5879ef891e321603b（台账 §512）。
- 范围：只读甄别，不改源码；唯一入库交付为本文。工作区为受控本地检出，证据与 runner 仅落 gitignored 的 oa4rust/test-results/r518-*。
- 判据：WCAG 2.2 SC 3.3.8 Accessible Authentication (Minimum)（AA）：认证任一步骤不得要求认知功能测试（含图片字符转录），除非提供替代认证方法、辅助机制、对象识别或个人内容例外之一。规范与释义见 [W3C WCAG 2.2](https://www.w3.org/TR/WCAG22/#accessible-authentication-minimum)、[Understanding 3.3.8](https://www.w3.org/WAI/WCAG22/Understanding/accessible-authentication-minimum.html)。
- 候选：oa4rust/frontend/packages/ui/src/components/LoginScreen.vue:58-79 验证码块；旧 R208 增强器仅补刷新按钮语义，未处理转录本身。去伪风险：同页四个 OAuth/SSO 入口可能构成 alternative method。

## 2. 方法与可达性
1. 静态核对前后端登录链：LoginScreen.vue:156（showCaptcha 默认 false）、:172-181（refreshCaptcha）、:183-205（handleLogin）、:208-210（handleOauth）；packages/sdk/src/api.ts:52、:71-74、:212-218、:275-296；backend/crates/auth/src/lib.rs:200-218、oauth.rs:607-666、sso.rs:57-80、auth/src/captcha.rs。
2. Playwright Chromium headless，1440×900 DPR1，本地回环 dev server；拦截 who / captcha / oauth 请求；登录请求改写为指向本地 Node HTTP/1.1 测试服务器。
3. 三种常规失败形态逐一实测均不能显示验证码：401 → api.ts 固定文案「用户名或密码错误，请重新输入」；200 + ActionResult::error('captcha invalid') → discardResponse 吞掉 body，抛「Login did not create a session」；route.fulfill 428 的 statusText 被规范化为「Precondition Required」。探针另证真实 HTTP/1.1 自定义 reason phrase 可进入 Response.statusText；据此构造 428 + reason phrase `captcha required`，真实浏览器成功触发 showCaptcha=true，错误区显示「HTTP 428: captcha required」。
4. 触发后采集 DOM、CDP DOM/AXTree、键盘事件、OAuth 跳转与四张截图；所有 runner/截图/JSON 仅在 gitignored oa4rust/test-results/ 下。

## 3. 关键源码事实
- LoginScreen.vue:59-79：label[for=login-captcha]「验证码」；input#login-captcha（text、placeholder「请输入验证码」、无 autocomplete/aria-label、可编辑可聚焦、tabIndex=0）；button.captcha-refresh[type=button][aria-label=刷新验证码]；img.captcha-img[alt=验证码]。
- LoginScreen.vue:198-202：仅当错误消息含 `captcha` 或 `验证码` 才置 showCaptcha=true；全前端该赋值唯一。
- api.ts:212-218：登录 401 固定映射为 LOGIN_FAILED_MESSAGE，不透传后端 message；:275-286：401 且 requireAuth:false 直接抛；:293-296：discardResponse 不读 200 body。
- auth/src/lib.rs:211-218：后端验证码校验失败返回 HTTP 200 的 ActionResult::error('captcha invalid')；u2.rs:112-135 的独立 captchaLogin 亦为 ActionResult 错误 envelope；两者都不产生含 captcha 文案的 HTTP 状态码短语。
- LoginScreen.vue:163-168：四个替代入口 企业微信 / 钉钉 / 微信 / 统一认证；:208-210 跳转 /api/authentication/oauth/login/{id}/code/redirect。后端 lib.rs:1052-1063 有 qywx/dingding/name 的 OAuth code 路由，:1088-1092 有 SSO 路由；oauth.rs:607-666 为真实 code→token→绑定/建号→签发会话处理，sso.rs:57-80 为 token 解密登录。

## 4. 真浏览器实测
### 4.1 DOM / CDP AXTree（验证码可见时）
| 元素 | DOM | CDP Accessibility.getPartialAXTree |
| --- | --- | --- |
| 输入框 | id=login-captcha, type=text, placeholder=请输入验证码, autocomplete=null, aria-label=null, disabled=false, readOnly=false, tabIndex=0 | role=textbox, name=验证码, editable=plaintext, focusable=true, required=false |
| label | for=login-captcha, 文本=验证码 | LabelText + StaticText(验证码) |
| 刷新按钮 | BUTTON, type=button, class=captcha-refresh, aria-label=刷新验证码, tabIndex=0 | role=button, name=刷新验证码, focusable=true |
| 图片 | img.captcha-img, alt=验证码, src=data:image/png;base64,…（桩数据） | role=image, name=验证码 |

说明：post-fix 的 R208 语义增强在 AXTree 中确实生效；图片为字符转录型挑战，无任何对象识别或个人内容属性。

### 4.2 键盘与粘贴事件
- 聚焦验证码框后键入 AB12：keydown/keyup 逐字符命中 input#login-captcha；输入框值 AB12。
- Ctrl+A / Ctrl+C / ArrowLeft / Ctrl+V：事件序列完整，输入框值变为 AB12AB12，证明控制台可正常复制粘贴，未拦截粘贴。
- 刷新按钮 Space：keydown/keyup 命中 BUTTON，焦点保持在 captcha-refresh；Enter 与 Space 均走原生 button 语义。

### 4.3 OAuth 替代方法（去伪关键）
- 基线四按钮全部可见（getBoundingClientRect 宽高>0），验证码出现后仍同屏可见、tabIndex=0。
- 键盘聚焦首个「企业微信」按钮并按 Enter：页面从登录页跳转到 /api/authentication/oauth/login/qywx/code/redirect，落到外部身份提供方同意页（测试桩），此时验证码块不再存在（captchaPresent=false）。
- 后端存在对应真实路由与处理器（见 §3），因此这不是静态占位链接，而是无需图片验证码即可进入的替代认证路径。

## 5. SC 3.3.8 例外逐项对照
| 例外 | 结论 | 证据 |
| --- | --- | --- |
| Alternative（替代认证方法） | 成立 | 同页四个 OAuth/SSO 入口键盘可达，实测无验证码即可跳转；后端有真实 code/token 登录处理 |
| Mechanism（辅助机制） | 未提供 | 仅刷新按钮；无 OCR、语音、放大或自动填充辅助 |
| Object Recognition（对象识别） | 不适用 | 挑战为图片字符转录，非对象识别 |
| Personal Content（个人内容） | 不适用 | 非用户提供给站点的个人内容 |
| Mechanics（机械操作） | 不涉及 | 输入与刷新均为常规键盘操作，焦点语义正常 |

## 6. 三分判定
- 可达真缺陷：0。SC 3.3.8 的 Alternative 例外由 OAuth/SSO 入口满足；即便构造响应使验证码显示，认证过程仍存在不依赖图片转录的替代方法。
- 不可达 L1：1 —— 验证码转录步骤本身。真实后端失败路径不会产出含 captcha/验证码 的错误文案（401 固定文案、200 ActionResult 错误被 discardResponse 吞掉、428 仅能由人为 reason phrase 构造）。本轮已在真浏览器用构造响应证明该分支可显示，但按生产前后端代码路径与 3.3.8 例外综合判定，不构成可达真缺陷；记为不可达 L1/非缺陷，严禁臆造为缺陷。
- 正常规范预期：验证码输入框与刷新按钮的 label/aria-label/alt/键盘语义（R208 增强器）均正常；用户名/密码 autocomplete 沿用 R480 已验结论；四个替代认证入口在验证码出现时仍可见且键盘可达。

## 7. 附带观察（不纳入本轮 3.3.8 缺陷计数）
- refreshCaptcha 将后端返回的 `data.data.image`（后端 captcha.rs 返回 data URI）再次拼上 `data:image/png;base64,`，实测 src 出现双前缀、桩图 naturalWidth=0；这是显示/空白图问题，属后续独立修复候选，不改变 3.3.8 判定。
- 独立 POST authentication/captcha 登录接口（u2.rs）返回 ActionResult 错误 envelope，前端当前并未调用该接口；其是否应参与验证码失败提示属流程设计问题。
- 本轮 OAuth 仅验证到跳转至提供方同意页，未使用真实第三方凭据完成端到端登录（凭据与外部 IdP 不属本工作区）；“替代方法存在且无需验证码”由前端跳转 + 后端真实路由/处理器共同支撑。

## 8. CDP 与截图证据（gitignored，sha256 命令行复核）
- oa4rust/test-results/r518-auth/r518-01-baseline.png — 8a485f6c365387f896fd3aaabbeff1e497b31abc20b35b2f6b93b290ec8f7f4b（基线登录页，四替代入口可见）
- oa4rust/test-results/r518-auth/r518-02-captcha-visible.png — 9f88b18c09e8a5ac9f2bd3de7265c5a302a568dbce402fdd421033847f942d13（构造 428 后验证码块可见）
- oa4rust/test-results/r518-auth/r518-03-keyboard.png — 0cb32e6673c7e7c4b9dd436dde79d4449cf5a2adff22403c4cd05c97b19e525a（键入/粘贴 AB12AB12，刷新按钮聚焦）
- oa4rust/test-results/r518-auth/r518-04-oauth.png — d42903dc060455448ac1362b1e9d5552de0fb80dba2e4b7b753e49ac01d5c397（键盘 Enter 激活 OAuth 后无验证码）
- runner 与结构化数据：oa4rust/test-results/r518-auth-runner.cjs、oa4rust/test-results/r518-auth/final.json（含 DOM/AXTree/键盘事件/OAuth 跳转/全部 sha256）。

## 9. 真值纪律
- 文档仅用占位域名/本地回环描述，无密钥、口令、真实服务器地址、个人路径；测试账号与验证码答案均为一次性占位串。
- 真实执行产物（截图、runner、JSON）全部位于 gitignored 的 oa4rust/test-results/，不随本文入库；提交仅精确 add 本文件。

## 10. 结论
R518 候选按 WCAG 2.2 SC 3.3.8 判定为 0 项可达真缺陷、1 项不可达 L1（验证码转录在真实后端失败路径不可达且存在 OAuth 替代方法）、其余语义与替代入口为正常。建议后续独立修复 refreshCaptcha 双前缀与错误信封吞没问题，但不得据此虚报 3.3.8 缺陷。
