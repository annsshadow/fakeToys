# R480 WCAG 1.3.5 输入字段用途（autocomplete）甄别报告

## 1. 概况
- 任务：t351（attempt 6049a808-6c53-45bc-9fd6-55642d630e21），真父 8a4283e80f084582128e08f2b3e5d47db5339e19；只读甄别，唯一交付本文。
- 测试来源：Playwright Chromium headless，PC 1440×900 DPR1，light/dark 双主题，非物理真机；双主题 7 页（登录/个人中心/三方成员/日历/表单/配置设计器/论坛）+ 三方成员新建弹窗补采。
- 判据：WCAG 1.3.5 Identify Input Purpose——收集用户个人信息（用户名/密码/姓名/手机/邮箱等）的输入字段须有正确 autocomplete token；业务搜索/配置/系统字段不适用。
- 证据：gitignored oa4rust/test-results/r480-input-purpose/（16 PNG + report.json + detail.json + analysis.txt + e2e/detail 脚本），真域名仅存在于 gitignored runner，口令运行时从受控凭据源提取注入，均未入库。

## 2. 方法
1. 静态盘点：全库 desktop views 显式 autocomplete 仅 Personal.vue 三密码框、LoginScreen.vue 用户名/密码框；ThreeMemberApp.vue 新建弹窗姓名/手机/邮箱 input 无 autocomplete。
2. E2E 14 组：登录 + personal/three-member/calendar/form/config-designer/bbs 各双主题。采集每个 input 的 type/id/autocomplete/placeholder/ariaLabel/关联 label/visible/value，CDP Accessibility.getPartialAXTree 取 axRole/axName/axAutocomplete（须先 DOM.enable + Accessibility.enable）。
3. 关键补采：three-member 新建弹窗视口首采未打开，写 r480-detail.cjs 经真实 UI 点击「新建」打开弹窗后采集姓名/手机/邮箱/所属单位字段（light/dark 两组）。
4. 截图 sha256 命令行 sha256sum 原始值入第 7 节，与 report.json/detail.json 三方交叉核验。
5. 方法学发现：AXTree 不暴露 autocomplete 属性（AX_AUTOCOMPLETE_DIST 全 30 项为空），因此 autocomplete 声明以 DOM getAttribute 为准，AXTree 仅用于核对字段标签与 role。

## 3. 逐字段实测分类（light/dark 对称，以下每字段双主题各计 1 项）

| 页面 | 字段 | 类型 | 分类 | autocomplete (DOM) | AX 名称 | 判定 |
| --- | --- | --- | --- | --- | --- | --- |
| 登录 | 用户名 (login-username) | text | PII | username | 用户名 | 正常：token 正确 |
| 登录 | 密码 (login-password) | password | PII | current-password | 密码 | 正常：token 正确 |
| 个人中心 | 当前密码 (oldPwd) | password | PII | current-password | 当前密码 | 正常：token 正确 |
| 个人中心 | 新密码 (newPwd) | password | PII | new-password | 新密码 | 正常：token 正确 |
| 个人中心 | 确认密码 (confirmPwd) | password | PII | new-password | 确认密码 | 正常：token 正确 |
| 三方成员新建弹窗 | 姓名 | text | PII | 空 | 姓名 | 可达真缺陷：缺 name token |
| 三方成员新建弹窗 | 手机 | text | PII | 空 | 手机 | 可达真缺陷：缺 tel token |
| 三方成员新建弹窗 | 邮箱 | text | PII | 空 | 邮箱 | 可达真缺陷：缺 email token |
| 三方成员新建弹窗 | 所属单位 | text | 业务 | 空 | 所属单位 | 正常：业务字段不适用（单位编号非个人 PII） |
| 三方成员列表 | 搜索姓名/手机/单位 | search | 业务 | 空 | 搜索姓名 / 手机 / 单位... | 正常：搜索框不适用，纠正首轮正则误判 |
| 全局 topbar | 搜索页面或内容 | text | 业务 | 空 | 全局搜索 | 正常：站内搜索不适用 |
| 个人中心 | 更换头像 | file | 业务 | 空 | 更换头像 | 正常：文件上传非文本 PII 字段 |
| 配置设计器 | 搜索配置 | text | 业务 | 空 | 搜索配置... | 正常：业务搜索不适用 |
| 论坛 | 搜索帖子 | text | 业务 | 空 | 搜索帖子 | 正常：业务搜索不适用 |

统计：PII 字段 16 项（双主题各 8）——正确 10 项（登录 2 + 个人中心 3，双主题），缺失 6 项（三方成员新建弹窗姓名/手机/邮箱 3，双主题）；业务字段 32 项（双主题各 16）全部不适用。合计 48 项输入。
说明：首轮 r480-analyze.cjs 将三方成员列表搜索框（placeholder 含姓名/手机）误判为 PII 2 项，经真实 UI 语义核对纠正为业务搜索字段，据实不把它算作缺陷；three-member 顶部搜索框在其后页面采集时正确归类为业务字段。

## 4. 根因与精确修复建议
- 缺失位置：oa4rust/frontend/apps/desktop/src/views/ThreeMemberApp.vue:39（姓名 input）、:40（手机 input）、:41（邮箱 input）。三处 input 均有可见 label（AX 名称正确），但 v-model 绑定表单字段且无 autocomplete 属性，浏览器/密码管理器无法识别字段用途，无法自动填充姓名/电话/邮箱。
- 建议（后续 RED→GREEN 轮执行，本轮只读不改码）：
  1. :39 姓名 input 增加 autocomplete="name"
  2. :40 手机 input 增加 autocomplete="tel"
  3. :41 邮箱 input 增加 autocomplete="email"
- LoginScreen.vue:62-67 验证码框无 autocomplete 属边界情况：验证码为一次性挑战码非持久 PII，本轮未显示（showCaptcha=false），未纳入候选；如需覆盖可用 one-time-code（须评估业务验证码输入方式是否由人工识读，避免误填），本轮不判缺陷。
- Settings.vue:26-53 均为系统配置项（台账号/URL/阈值等），非个人信息，不适用。

## 5. CDP 与截图目视
- CDP Accessibility.getPartialAXTree：弹窗姓名/手机/邮箱 axName 分别为「姓名/手机/邮箱」、axRole=textbox、axIgnored=false，说明无障碍名称维度正常，缺陷仅在 autocomplete 用途声明维度。
- 目视 4 图均名实相符：pc-login-light（登录卡用户名/密码框占位清晰）；pc-personal-light（修改密码三框+资料展示卡完整）；pc-three-member-modal-light（新建弹窗姓名/手机/邮箱/所属单位字段可见）；pc-three-member-modal-dark（同结构深色）。无布局崩坏。

## 6. 三分判定
- 可达真缺陷：1 项 P2——三方成员新建/编辑弹窗姓名/手机/邮箱三个 PII 输入字段缺 autocomplete（双主题 6 项），浏览器/密码管理器无法自动填充；影响有限（管理端成员录入低频场景）故定 P2，修复成本极低。
- 不可达 L1：0——所有候选均真实可达（弹窗经真实 UI 点击打开，字段 visible=true），无 aria-hidden/display:none/R306C 硬排除页。
- 正常规范预期：登录用户名/密码、个人中心三密码框 token 正确（10 项）；全部业务搜索/配置/文件/单位字段不适用（32 项）；搜索框首轮误判已据实纠正。
- AX autocomplete 属性全空属 CDP AXTree 不暴露该属性，非缺陷；以 DOM getAttribute 为实测依据。

## 7. 截图 sha256（命令行原始值，与 report.json/detail.json 一致）
- pc-login-light-input-purpose.png sha256=24f77b06e86a32cfefd8f51fb60418fc079e517945714c4f726944d58c9bf957
- pc-login-dark-input-purpose.png sha256=b533a22715c26c8d7dc8d379c216896aa0ff1e63b9c41c4ef4914389fd17aa1b
- pc-personal-light-input-purpose.png sha256=1459c1f14638d3f96959341cd21901859dfe643b1816ef0677649a541f4f7cc8
- pc-personal-dark-input-purpose.png sha256=ab251a567519c4ec0fce414eafb392b8b986ba926ffec46fb05865f0de5d24dc
- pc-three-member-light-input-purpose.png sha256=2a12c2b0649cfad3e9f115fd3c135c6f5436eed94df710750f87da9f45ad3c5f
- pc-three-member-dark-input-purpose.png sha256=9a5f4cff1bd6328806e387dcca7bc2296d28aaf9b68237404270b102b615fc3a
- pc-three-member-modal-light-input-purpose.png sha256=ac7005f8bcb090a419636961d350c7186aa50ec3fdcb29c7cec542f9c7267b35
- pc-three-member-modal-dark-input-purpose.png sha256=2154b1c753a8046a28471eba7237cb852e2abf01add9eaad0753a32fe1de8403
- pc-calendar-light-input-purpose.png sha256=8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad
- pc-calendar-dark-input-purpose.png sha256=0075d87d57dc86a5bc3963fa095ecbabaa06562542fbaea5922b2234fe9f6c4b
- pc-form-light-input-purpose.png sha256=577b73c3de9a05e9ab28d20d1551e3de8af93c0f607d2b23c904997db8c24f80
- pc-form-dark-input-purpose.png sha256=7c675548961afd8d9226d6b166b641d1a21e13dc20b5afc30a2bd9d47275d7d2
- pc-config-designer-light-input-purpose.png sha256=5605435ed5fc32e592d8d141330682c47a64e65cb25046a0d66b0079b8109380
- pc-config-designer-dark-input-purpose.png sha256=a26259e69c09d9ae3700ace6639bdab22494827a59cc0e2535eac913639aa90f
- pc-bbs-light-input-purpose.png sha256=888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105
- pc-bbs-dark-input-purpose.png sha256=6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c

## 8. 真值纪律
- 测试域名/主机用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，真值仅在 gitignored runner；口令运行时注入，不入库。
- 本文 0 pageerror；所有截图为真实浏览器渲染，注明非物理真机；未使用 / 未提交任何密钥、真实域名或个人信息。
- 只读甄别：未修改任何入库源码，修复建议留给后续 RED→GREEN 轮。
