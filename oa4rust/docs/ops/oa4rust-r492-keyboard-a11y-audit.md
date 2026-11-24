# R492 WCAG 2.1.1 键盘可达性甄别（FileManager 面包屑 / 文件行）

- 轮次：R492，任务 t378，Kind=work（只读甄别，不改业务码）
- 真父提交：c5f4458323a69a963242918f8d70030fa4c7656f（台账 §487 据实立项）
- 判据子集：WCAG 2.1.1 Keyboard——所有功能是否可经键盘操作；本轮按队长派单收敛到「文件管理页面包屑 / 文件列表项」强候选
- 方法：Playwright Chromium headless，PC 1440×900 DPR1，light/dark 双主题，CDP DOM.getDocument + Accessibility.getPartialAXTree + 真实 Tab/Enter/.focus() 逐次实证；非物理真机
- 测试服：从环境变量读取地址与账号（占位 <YOUR_TEST_DOMAIN> / <TEST_HOST>，真值不入库）
- 证据目录：oa4rust/test-results/r492-keyboard/（gitignored，不入提交）

## 1. 甄别对象与硬纪律

强候选（frontend/apps/desktop/src/views/FileManager.vue，注意不是 FileManagerApp.vue）：

- :30-35 路径面包屑 <span class="bc-item" :class="{active: i===breadcrumbs.length-1}" @click="navigateTo(i)">{{ seg }}</span>——非语义 span，无 role / tabindex / @keydown
- :53 列表行 <li class="file-row" @dblclick="openItem(f)">、:69 网格卡 <div class="file-card" @dblclick="openItem(f)">——仅鼠标双击入口
- :820-826 openItem(f) 仅当 f.type==='folder' 才进入目录并追加面包屑；:828-832 navigateTo(i) 截断面包屑 + 重载目录

硬纪律执行：

1. 严禁仅凭源码 @click 命中就预判缺陷；全部结论以真浏览器 CDP + 键盘实证为准。
2. 全 views 共 172 处 @click.self 遮罩关闭属合理模式，modalFocus.ts 已覆盖 Escape/Tab 陷阱/焦点归还，严格排除、不判缺陷。
3. 核查等价键盘替代路径，避免把「有替代」的功能误判为不可达。

## 2. 实测数据（逐项与 report.json / supplement.json 一致）

### 2.1 主轮（report.json，groups=2，errors=0，0 pageerror）

双主题一致：

| 项 | 实测值 |
|---|---|
| 顶层路由面包屑 | 工作台/文件管理（页面顶部导航，非 .breadcrumb 组件） |
| .bc-item | 仅 1 段且 active：text=根目录, tag=span, class=bc-item active, role="", tabindex=null, tabIndexProp=-1, cursor=auto |
| .bc-item AXTree | axRole=generic, axName="", axIgnored=false（CDP getPartialAXTree） |
| 同页 10 个工具栏按钮 | 全部 axRole=button, axFocusable=true（切换主题/展开侧边栏/打开命令面板/🔔/📤上传/🗂️新建/⊞网格/刷新/ℹ/⬇） |
| 文件列表 | 仅 2 个 txt（r315-upload-…txt / R314-Upload-E2E-…txt），folderRows=[]，entered=false |
| 36 步 Tab 序列 | 全部 isBc=false：工具栏 button/input → 侧栏 nav-item → theme-toggle/collapse-btn/search-input，跳过 .bc-item |
| .focus() 尝试 | becameActive=false，activeElement 保持 search-input（span 无 tabindex，不可编程聚焦） |
| 单段点击 | clicked changed=false（navigateTo(0) 对单段 active 不产生面包屑文本变化） |
| 等价路径 | hasBackButton=false，FileManager 自身无侧栏目录树/返回按钮 |

### 2.2 补采（supplement.json，groups=2，errors=0，0 pageerror）

| 项 | light | dark |
|---|---|---|
| bc-item .focus() 后 activeElement | MAIN.content（isBc=false） | MAIN.content（isBc=false） |
| bc-item AX | role=generic, name="", ignored=false | 同 |
| 按 Enter 前后面包屑文本 | 工作台/文件管理 → 不变（changed=false） | 同 |
| 文件行（2 条） | tag=li, tabindex=null, tabIndexProp=-1, role="", hasClick=false | 同 |
| 45 步 Tab 命中行 | tabHitRow=null（序列含 BUTTON.modern-btn/INPUT/BUTTON.icon-action/A.skip-link/DIV.nav-item/theme-toggle/search-input 等，从不命中行） | 同 |
| 对照「刷新」按钮 | .focus() 得 BUTTON「刷新」，按 Enter 触发 listRequestsAfterEnter=1 | 同 |

对照结论：同页真实 <button> 可键盘聚焦且 Enter 生效；.bc-item（span）与文件行均不可聚焦——差异来自语义元素与 tabindex，非测试环境问题。

### 2.3 造数探测（决定可达性，gitignored 证据）

- 前端 ApiClient 自动加 /oa4rust-api 前缀；错误 base /api/... 全 404，纠正后正常。
- POST /api/file/folder/create、POST /api/file/assemble/control/file/create、POST /api/folder 均 status=200，文件夹实体确已创建（如 R492folderY, id=4a2af6ae…）。
- 但 GET /api/file/assemble/control/file/list/root（loadFiles() 实际调用端点）只返回 2 个文件记录，不含任何文件夹实体；刷新后 UI 仍 2 个 txt、bc=[根目录]。
- 另证：/api/file/folder/list/top 与 core/entity top 端点能返回文件夹实体（count=3），说明文件夹存在于服务端但不进入文件管理列表。
- 结论：真实 UI 不渲染 📁 行 → 无法 dblclick 进入子目录 → 多段面包屑（真正的导航功能）在当前真实数据下不可达，不得臆造该场景。

## 3. 三分判定

### 可达真缺陷（P0/P1/P2）：0 项

- 单段 active 面包屑：navigateTo(0) 对单段 active 不产生实质导航变化（实测 clicked changed=false）；即便理解为「重载根目录」，同页「刷新」按钮已提供等价键盘路径（补采 Enter → 1 次 list 请求）。无键盘被阻断的功能，不构成 2.1.1 违反。
- 文件列表项 dblclick：openItem(f) 仅对 folder 生效（源码 :820-826 坐实）。当前真实数据全为 txt，dblclick 无任何功能；行内「详情/预览、下载、分享、删除」为 4 个可聚焦 <button>（补采 tabSeq 坐实 BUTTON.icon-action 全部命中键盘序列）。无功能被键盘阻断。

### 不可达观察（L1）：1 项

- 多段面包屑导航 / 文件夹进入路径：.bc-item 确为无 role/tabindex/@keydown 的 span，AX generic、.focus() 不可聚焦（双主题实证）；文件行打开仅 @dblclick。但该功能的用户路径（进入文件夹 → 生成多段面包屑）因 list 端点不返回文件夹实体而在当前真实 UI 不可达（folderRows=[]），故按铁律记为不可达观察，不计入可达缺陷，不臆造缺陷、不预判。

### 正常 / 规范预期：若干

- 10 个工具栏按钮全部 axRole=button + axFocusable=true；刷新按钮键盘 Enter 触发真实请求。
- 文件行内 4 个 icon-action 按钮键盘可达。
- 172 处 @click.self 遮罩由 modalFocus.ts 覆盖，属队长给定排除区。
- main.ts 已装配 skip-link（Tab 序列出现 A.skip-link）等无障碍增强器。

## 4. 修复建议（供后续轮次，当前不可达故非必改）

1. FileManager.vue:32：若未来开放文件夹列表、启用多段面包屑，应将 .bc-item 改为 <button type="button">，或补 role="link" tabindex="0" @keydown.enter="navigateTo(i)"，并补 aria-current="page" 标识当前段。
2. FileManager.vue:53 / :69：若文件夹进入功能上线，为 .file-row / .file-card 补 tabindex="0" role="button" @keydown.enter="openItem(f)"（当前 openItem 仅 folder 生效，当前数据无 folder）。
3. 数据层：使 loadFiles() 使用的 list 端点返回文件夹实体，或在 UI 单独渲染目录树——这是让上述功能「可达」的前提；未达成前不构成用户可感知的键盘障碍。

## 5. 非阻断观察（据实披露，本轮不判缺陷）

独立反证扫描：全 views 另有 60 处非遮罩 <div>/<span>/<li @click> 真实交互控件（如 FormApp.vue:23/50/87/123、FormDesigner.vue:28/175/567、MeetingApp.vue:51/79、OrgViewer.vue:143/151、PortalApp.vue:80、ProcessDesigner.vue:74-89/350-354 等）。本轮 inScope 收敛到 FileManager 面包屑，未对这些控件逐一做真浏览器键盘实证；按「严禁仅凭源码预判」铁律不预判为缺陷，建议后续独立轮次甄别。

## 6. 截图与来源标注

全部截图为 headless 浏览器真实页面，1440×900，非物理真机；sha256sum 命令行核验与 JSON 一致：

| 截图 | sha256 |
|---|---|
| pc-file-keyboard-light-r492.png | c6846fc06b53a3a5e09a576c308a9a0dee644b3b92fd7126848a89d3641eb3f0 |
| pc-file-keyboard-dark-r492.png | 64f0f417a243210d96c77db2b4637bfc33f686db246a27931f692c800f0186fe |
| pc-file-supplement-light-r492.png | f384d876a3db0bf652faca0978cb2a231c4d1fb85ad500c60033ce6cbce1ec41 |
| pc-file-supplement-dark-r492.png | f9a3e1ca7ee6372552245085c053dbabf1f58c127b63dc2ce41b8c921975b236 |

AX 快照：ax-light.json / ax-dark.json；原始数据：report.json / supplement.json / folder-data-probe.json / probe-create2.json（均 gitignored）。

## 7. 脱敏与合规

- 无密钥、口令、token、私钥；账号由 OA_TEST_USER / OA_TEST_PASS 环境变量注入，真值不入库。
- 测试服地址以 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，0 真值回显。
- 源码零改；唯一写域为本甄别文档；证据目录 gitignored。
- 本地约定：≥1000 轮持续迭代，本结论是 R492 一环，非项目终态。
