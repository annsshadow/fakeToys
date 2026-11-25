# R509 WCAG 2.1 AA 4.1.2 文件输入控件可访问名称甄别

## 1. 依据与判据

- 轮次 R509，真父 7b420b83c49c0241fac4074359e518ff8dbd480f（台账 §503 已 push，HEAD=UP AB=0/0）。
- WCAG 2.1 AA 4.1.2 Name, Role, Value：用户界面组件的名称与角色须可被程序化确定。文件选择 `<input type=file>` 在 Chromium AXTree 中稳定暴露 role=button，但其名称若仅为浏览器 UA 默认文案（Choose Files），并非开发者提供的业务可访问名称；当周边视觉提示未被程序化关联时，读屏用户只能听到与页面语言无关的英文默认名，无法感知该控件在本界面的语义。
- 反预判铁律：本轮严禁仅凭源码无 `aria-label` 即预判缺陷；必须以真浏览器 CDP `Accessibility.getPartialAXTree` 坐实 computed axName、role、父级提示是否被程序化关联；并以显式注入 aria-label 的正面对照验证名称来源。
- 证据来源：Playwright Chromium headless，PC 1440x900 DPR1，light/dark 双主题，非物理真机；CDP DOM + Accessibility.getPartialAXTree；0 pageerror。
- 测试服域名与凭据仅存在于 gitignored 本地 runner，文档仅用占位符 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>`，0 真值入库。

## 2. 方法与自洽

- 静态锚点（源码 sha256：FileManager.vue=832fc7d5f22637d383325d9bfafd189f22b7d68d6f384e486111bbe2bf38020b；Personal.vue=8a323a15ea9e3c66f8114dede3fbb38f403502a131c23c7fe75bdfe58088d9b1）：
  - `oa4rust/frontend/apps/desktop/src/views/FileManager.vue:149-157`：上传弹窗 `<div v-if="showUpload" class="upload-overlay">` 内 `:153 <div class="upload-area @drop.prevent=handleDrop">`，`:155 <p>拖放文件到此处，或点击选择</p>`，`:156 <input type="file" multiple class="file-input" @change="handleFileSelect" />`；该 input 无 id、无 label for、无 aria-label/aria-labelledby，父级 p 提示不在 label 内，源码上不构成程序化关联。
  - `oa4rust/frontend/apps/desktop/src/views/Personal.vue:19-22` 正面对照：`<label class="upload-btn modern-btn primary">` 内 `<input type="file" ... aria-label="更换头像" />`，显式名称 + label 包裹。
- 真浏览器流程：登录 → `/oa4rust/app/file/` → 设置主题 → 点击 `.file-toolbar button.primary`（📤 上传）真实打开上传弹窗 → 对 `.upload-dialog input[type=file]` 打标 → CDP 采 DOM 属性 + AXTree；补采对 Personal 头像 input 作正面对照，并在 FileManager 对同一 input 注入 `aria-label="上传文件"` 后即时重采 AXTree，验证名称来源。
- 主轮报告 `test-results/r509-file-input/report.json`（sha256=106da9f0389d39d490a45209336a32d125b853ef540a3e2f841ceaad07803b98）；补采 `.../supplement.json`（sha256=b50c7e87d1162e973853825c1d113e99bad1a8f3c1be168292bb51b4a0d2980b）。两轮各 2 组（双主题），均 0 fatal / 0 pageerror，上传弹窗真实出现（overlayPresent=true，actionError 为空）。

## 3. 逐项程序化证据（与 report.json / supplement.json 一致）

| 主题 | 目标 | DOM 属性 | CDP AXTree | 截图 |
|---|---|---|---|---|
| light | FileManager 上传 input | type=file multiple class=file-input id="" role="" aria-label="" aria-labelledby="" title="" tabindex="" tabIndex=0 visible=true rect=(301,801,253,21) outerHTML 无名称属性 | axRole=button, axName="Choose Files", axDescription="", axIgnored=false, focusable=true | pc-file-upload-light-r509.png |
| dark | FileManager 上传 input | 同 light（双主题一致） | axRole=button, axName="Choose Files", axDescription="", axIgnored=false, focusable=true | pc-file-upload-dark-r509.png |
| light | 同页关闭按钮 | `.upload-actions .btn-cancel` 可见 | axRole=button, axName="关闭", axIgnored=false | 同上 |
| dark | 同页关闭按钮 | 同上 | axRole=button, axName="关闭", axIgnored=false | 同上 |
| light | 同页 `.upload-area p` | 文本"拖放文件到此处，或点击选择"，parent 内与 input 并列；labelsFor=[]，pInLabel=false，inputInLabel=false | axRole=paragraph, axName="", axIgnored=false，未作为 input 的标签 | 同上 |
| dark | 同页 `.upload-area p` | 同 light | axRole=paragraph, axName="", axIgnored=false | 同上 |
| light | Personal 正面对照 input | aria-label="更换头像", inLabel=true, `<label>` 文本"📷 更换头像" | axRole=button, axName="更换头像", axIgnored=false | pc-sup-light-r509.png（同页同态同 sha） |
| dark | Personal 正面对照 input | 同 light | axRole=button, axName="更换头像", axIgnored=false | pc-sup-dark-r509.png（同页同态同 sha） |
| light | FileManager 注入对照 | 对同一 input 注入 `aria-label="上传文件"` 后即时重采 | axRole=button, axName="上传文件", axIgnored=false | 同上 |
| dark | FileManager 注入对照 | 同 light | axRole=button, axName="上传文件", axIgnored=false | 同上 |

程序化自洽链：DOM 层无任何名称属性（id/aria-label/aria-labelledby/title 均空、无 label for）→ AXTree 名称回落到 UA 默认 "Choose Files" → 父 p 作为独立 paragraph 未被程序化关联（labelsFor=[]）→ 正面对照 Personal 有显式 aria-label 且 AX 名为业务中文 → 对 FileManager input 注入 aria-label 后 AX 名即时改变，证明 "Choose Files" 是 UA 默认而非业务可访问名称。

## 4. 三分判定

- 可达真缺陷：1 项（P2）。FileManager.vue:156 上传弹窗 file input 的 computed AX 名为浏览器默认 "Choose Files"，无中文业务可访问名；父 p 提示"拖放文件到此处，或点击选择"未被程序化关联（labelsFor=[]，p 与 input 并列无 label）。读屏用户获知的是与界面语言不符的通用英文默认名，无法从控件自身确定"上传文件"语义，命中 WCAG 4.1.2 Name 要求。双主题一致，实证充分。
- 不可达 / L1：0 类。上传弹窗真实打开（overlayPresent=true，file input 可见），非数据不可达；主轮与补采均 0 pageerror / 0 fatal。
- 正常规范预期：3 类。① 角色暴露正确：file input 在 AXTree 中 role=button、focusable=true、axIgnored=false，符合 4.1.2 Role；② 同弹窗关闭按钮有正确名称"关闭"；③ 正面对照 Personal 的 file input 显式 aria-label="更换头像" 且 inLabel=true，名称程序化可用——证明工程内已有合规范式，本项是单点遗漏而非系统性缺失。
- 反预判边界：本轮仅抽测 FileManager 上传弹窗与 Personal 头像对照两处 file input，其余文件输入控件不预判；"Choose Files"非空故不按"名称完全为空"误判，但它是 UA 默认而非业务名称且无程序化关联，据实判 Name 缺失，不夸大至 Role/Value。

## 5. 修复建议（本轮只读不改源码）

- 在 `oa4rust/frontend/apps/desktop/src/views/FileManager.vue:156` 为 file input 增加显式可访问名，例如 `aria-label="上传文件"`（与注入对照一致，AX 名即时生效），或用 `<label for>` 将 `:155` 的"拖放文件到此处，或点击选择"程序化关联；零业务校验逻辑改动。
- 回归验证：真浏览器打开上传弹窗后 CDP AXTree 应显示 axName 为业务中文（如"上传文件"）而非 "Choose Files"；双主题均通过。

## 6. 证据与 sha256

| 证据 | sha256 |
|---|---|
| test-results/r509-file-input/report.json | 106da9f0389d39d490a45209336a32d125b853ef540a3e2f841ceaad07803b98 |
| test-results/r509-file-input/supplement.json | b50c7e87d1162e973853825c1d113e99bad1a8f3c1be168292bb51b4a0d2980b |
| pc-file-upload-light-r509.png | c72eaf9ce8ed2f58823b1dca1ee9652511fcd228d604e65dbac0a4b3b6de7d4d |
| pc-file-upload-dark-r509.png | d2d7d6d62d3a93a222da9231758701dc0936dcbe6b9f3a170c67d2335efcc794 |
| pc-sup-light-r509.png（同页同态同 sha） | c72eaf9ce8ed2f58823b1dca1ee9652511fcd228d604e65dbac0a4b3b6de7d4d |
| pc-sup-dark-r509.png（同页同态同 sha） | d2d7d6d62d3a93a222da9231758701dc0936dcbe6b9f3a170c67d2335efcc794 |
| frontend/apps/desktop/src/views/FileManager.vue（源码锚点） | 832fc7d5f22637d383325d9bfafd189f22b7d68d6f384e486111bbe2bf38020b |
| frontend/apps/desktop/src/views/Personal.vue（正面对照） | 8a323a15ea9e3c66f8114dede3fbb38f403502a131c23c7fe75bdfe58088d9b1 |

全部 sha 由命令行 sha256sum 对磁盘实存文件核验；补采截图与主轮同页同态同 sha，据实披露不重复计数。截图均为 headless 非物理真机，1440x900 DPR1；双主题实际生效（DOM theme=light/dark 已坐实）。

## 7. 脱敏边界

- 测试服真实域名、SSH 目标、登录凭据只存在于 gitignored 本地 runner 与环境变量，未写入本文档或任何入库文件；域名占位 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>`。
- 截图为测试服真实页面但不含凭据；证据目录 `test-results/r509-file-input/` 已 gitignored。

## 8. 结论

R509 据实甄别：可达真缺陷 1 项（P2，FileManager 上传 file input 仅有 UA 默认名、无业务可访问名且父提示未程序化关联）；不可达 / L1 0 类；正常规范预期 3 类（Role 正确、关闭按钮名称、Personal 正面对照）。两轮真浏览器 4 组 0 pageerror，反预判链完整，不泛化其他 file input。