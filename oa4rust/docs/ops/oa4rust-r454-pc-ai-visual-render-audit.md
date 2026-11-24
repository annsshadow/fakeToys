# R454 PC 端 AI 视觉高风险渲染点真浏览器实证核查

- 任务：t310（design-lead，Kind=work，只读甄别、不改源码）
- attempt_id：9a846220-03e6-4dab-9a1b-89eb4a9ded7d
- 基线：分支 opt1000/oa4rust-pc-ai-refactor，HEAD=UP=b6adea4ab90957a01a585490bb00ef6fd15cb60a（任务描述的 c60ad3cdb 已过时，以 git 实况为准），工作树干净
- 测试入口：https://<YOUR_TEST_DOMAIN>/oa4rust/app/...（<TEST_HOST> 真值仅存在于 gitignored 运行脚本，不入库）
- 证据目录：oa4rust/test-results/r454-pc-ai-render/（11 PNG + report.json + deep.json + deep2.json，均 gitignored）

## 1 方法与真实性声明

- 真浏览器：Playwright Chromium headless，viewport 1440x900、DPR1、非物理真机。
- 来源标注：测试服真实页面（NOT a physical device，非物理真机），全程未注入伪造业务数据、未改任何源码。
- 凭据：ssh oa4rust-test 运行时从 /usr/local/oa4rust/.admin-credentials 正则提取，注入 OA_TEST_USER/OA_TEST_PASS；域名真值只在 gitignored runner，仓库内 0 真值。
- 采集：逐点 computedStyle（backgroundImage/backgroundColor/backgroundClip/color/webkitTextFillColor/backdropFilter/boxShadow/borderColor/transition/animationName/Duration）、CDP CSS.getMatchedStylesForNode（DOM.enable + CSS.enable 后取匹配规则与选择器文本）、getAnimations（playState/currentTime）、截图 sha256。
- 骨架屏触发：对真实业务 API 用 page.route 延迟 5000ms 再 continue，使 loading 态真实出现，无伪造数据。
- 结果：10 路由全加载成功，errors=[]，pageerror=0；deep 6 probes / deep2 5 probes 均 0 errors。

## 2 逐点实证结果

### ① 渐变文字 .ai-gradient-text（8 页）

判定链：background-clip:text 需同时 color:transparent（webkitTextFillColor 透明）才能让渐变透出；若 color 被实色覆盖，渐变被实色文字遮住=效果丢失；若渐变未解析且 color 透明=文字不可见（P 级）。

| 页 | bgImage | color | webkitTextFillColor | bgClip | visibility | 判定 |
| --- | --- | --- | --- | --- | --- | --- |
| dashboard | linear-gradient(120deg,rgb(67,56,202),rgb(14,116,144)) | rgba(0,0,0,0) | rgba(0,0,0,0) | text | visible | 正常 |
| calendar | linear-gradient(120deg,...) | rgba(0,0,0,0) | rgba(0,0,0,0) | text | visible | 正常 |
| file | linear-gradient(120deg,...) | rgba(0,0,0,0) | rgba(0,0,0,0) | text | visible | 正常 |
| meeting | linear-gradient(120deg,...) | rgba(0,0,0,0) | rgba(0,0,0,0) | text | visible | 正常 |
| cms-index | linear-gradient(120deg,...) | rgba(0,0,0,0) | rgba(0,0,0,0) | text | visible | 正常 |
| deployment | linear-gradient(120deg,...) | rgba(0,0,0,0) | rgba(0,0,0,0) | text | visible | 正常 |
| bbs | linear-gradient(120deg,...) | rgb(37,99,235) | rgb(37,99,235) | text | visible | 渐变效果丢失（色被覆盖） |
| bam | linear-gradient(120deg,...) | rgb(37,99,235) | rgb(37,99,235) | text | visible | 渐变效果丢失（色被覆盖） |

- CDP 坐实 bbs：全局/局部 .ai-gradient-text[data-v-3772873b]{background:var(--ai-gradient-text);color:#0000;background-clip:text} 之后 .header-left h1[data-v-3772873b]{color:var(--color-primary);text-shadow:...} 特异性更高（类+标签+属性 vs 类+属性），color 被业务 CSS 覆盖为 --color-primary=rgb(37,99,235)。
- CDP 坐实 bam：同理 .view-header h1[data-v-eae1b21c]{color:var(--color-primary);...} 覆盖 .ai-gradient-text[data-v-eae1b21c]{color:#0000}。
- 无 P 级"渐变未解析且透明不可见"：8 页 bgImage 全部真实解析为 linear-gradient，无文字消失。

### ② 骨架屏动画

- find-designer（FindDesignerApp）：page.route 延迟 /api/query/assemble/designer/list/all 5000ms 后 observed=true；实测 animationName=ai-shimmer-85d73030、animationDuration=1.6s、getAnimations playState=running/duration=1600ms；但 bgImage=none、bgColor=rgb(248,250,252)（纯色）。
- CDP 坐实根因：全局 .ai-skeleton-line{background:linear-gradient(90deg,--ai-skeleton-base 25%,--ai-skeleton-shine 50%,--ai-skeleton-base 75%);animation:ai-shimmer...} 被同页 scoped .ai-skeleton-line[data-v-85d73030]{background:var(--ai-skeleton-base);height:16px;animation:ai-shimmer-85d73030...} 以更高特异性覆盖；@keyframes ai-shimmer 只改 background-position，纯色背景下位移无视觉流光 → 动画在跑但视觉不可见。
- attendance（AttendanceApp）：模板 :156 v-if="loading" 渲染 .ls > .sk×8，样式 :1839 渐变、:1901 兜底纯色；但全文件 grep "loading" 仅 :336 声明 ref(false) 与 :156 模板引用，无任何置 true 代码 → 骨架分支死代码、不可达 L1，用户永不触发。
- bam：源码 :652 有 .ai-page .skeleton-row 渐变定义，本轮加载期未捕获到骨架 DOM（正常内容已渲染），非重点页据实记录。

### ③ .ai-panel 玻璃卡片 backdrop-filter

| 元素 | backdropFilter | bgColor/bgImage | boxShadow | 判定 |
| --- | --- | --- | --- | --- |
| attendance .view-header | blur(12px) | gradient-surface | 正常阴影 | 正常（scoped 用正确 var(--ai-glass-blur)） |
| bbs .bbs-header | blur(12px) | rgba(255,255,255,0.72) | 正常 | 模糊正常，但渐变背景被 ai-panel 覆盖（见④） |
| dashboard .welcome-bar | none | gradient-surface | 正常 | 无 backdrop 模糊；背景/阴影完整，玻璃观感由渐变+阴影承载 |
| file .ai-panel | none | rgba(255,255,255,0.72) | 正常 | 同上 |
| bam .view-header | none | rgba(255,255,255,0.72) | 正常 | 同上 |
| calendar .view-header | none | rgba(255,255,255,0.72) | 正常 | 双重 blur 语法错误致丢弃（见下） |
| meeting .meeting-hero | none | gradient-surface | 正常 | 双重 blur 语法错误致丢弃（见下） |

- CDP 坐实确定性根因：CalendarApp.vue:800 与 MeetingApp.vue:1125 写 backdrop-filter:blur(var(--ai-glass-blur))，而 --ai-glass-blur 本身已是 blur(12px)，展开为 blur(blur(12px)) 非法值，声明被浏览器丢弃 → backdropFilter=none。反例 attendance :1836 写 backdrop-filter:var(--ai-glass-blur)（无外层 blur）→ 正确解析 blur(12px)。
- 所有页玻璃卡片的背景色、边框、阴影均正常解析（bgColor rgba(255,255,255,0.72) 或 gradient-surface、border rgba(15,23,42,0.1)、boxShadow 正常），视觉外观完整，仅 backdrop 毛模糊在部分页缺失。

### ④ .ai-page view-header 渐变背景与 transition

- calendar .view-header：bgImage=none / bgColor=rgba(255,255,255,0.72)（背景 var(--ai-glass-bg) 正常解析），transition on .ai-page.calendar-view=background 0.14s cubic-bezier(0.16,1,0.3,1) ✓；CDP 见 .ai-page .view-header[data-v-c3018281] 规则存在（其 backdrop-filter 因双重 blur 失效见③）。
- meeting .meeting-hero：bgImage=linear-gradient(160deg,...) ✓（var(--ai-gradient-surface) 正常），transition on .ai-page.meeting-view=background 0.14s...,color 0.14s... ✓。
- bbs .bbs-header：期望 bgImage=linear-gradient（源码 .bbs-header{background:var(--ai-gradient-surface)}），实测 bgImage=none/bgColor=rgba(255,255,255,0.72)。CDP 坐实同特异性冲突：.bbs-header[data-v-3772873b]{background:var(--ai-gradient-surface)} 与后置 .ai-panel[data-v-3772873b]{background:var(--ai-glass-bg)} 特异性相同（均 0,2,0），后置 ai-panel 覆盖 → 渐变 header 退化为玻璃纯色。可达视觉降级（轻微）。
- attendance/find-designer .view-header：bgImage=linear-gradient(160deg,...) ✓ 正常。

### ⑤ 未定义令牌导致的声明丢弃

逐页 computedStyle 读取 8 个令牌，结果一致：--ai-gradient-text=linear-gradient(120deg,#4338ca,#0e7490)、--ai-glass-bg=#ffffffb8、--ai-glass-blur=blur(12px)、--ai-shadow-2=0 4px 16px #0f172a1a、--ai-gradient-surface=linear-gradient(160deg,#4f46e50f,#0891b205)、--ai-skeleton-base=#f8fafc、--ai-skeleton-shine=#0f172a14 均有有效值；--ai-surface-gradient 为空（已知废弃令牌，真实代码引用 0，属预期）。

结论：无任何页面实际引用未定义令牌导致声明变 none/空/继承的情况；队长静态前提"PC 十五种 ai- 令牌差集 0、错误令牌 --ai-surface-gradient 真实残留 0"获运行时印证。

## 3 三分判定

### 可达真缺陷（有 computedStyle + CDP 证据）

1. BBSForum/BamApp 渐变标题退化：color 被业务 h1 规则覆盖为 rgb(37,99,235)，background-clip:text 失效，渐变文字显示为纯蓝实色。文字可读（非 P 级不可见），属视觉降级 P2。建议后续独立任务提升 .ai-gradient-text color:transparent 特异性（或业务 h1 避免覆盖 color）。
2. FindDesigner 骨架屏流光失效：scoped 纯色背景覆盖全局渐变，ai-shimmer 仅位移 background-position，纯色下无流光视觉。占位块仍可见，属动效失效 P3。建议 scoped 骨架背景改用 linear-gradient 或移除颜色覆盖。
3. CalendarApp/MeetingApp 玻璃模糊失效：backdrop-filter:blur(var(--ai-glass-blur)) 双重包裹为非法值致丢弃，backdropFilter=none。背景/边框/阴影完整，属轻微 P3。建议改为 backdrop-filter:var(--ai-glass-blur)。
4. BBSForum header 渐变背景被 ai-panel 同特异性覆盖：bgImage 由预期 linear-gradient 退化为 none/纯色玻璃。轻微 P3。建议调整 scoped 规则顺序或提升 .bbs-header 背景特异性。

### 不可达 L1

- AttendanceApp 骨架屏：loading 永不为 true，模板分支死代码，用户不可见，非缺陷；不建议为其投入修复，若需激活应另立任务接通真实 loading 状态。

### 正常

- 6/8 页渐变文字完全正常（渐变解析+透明色+clip:text+可见）；dashboard/meeting/attendance/find-designer 的 header 背景与 transition 正常；所有令牌解析正常；10 路由 0 pageerror。

## 4 截图 sha256（sha256sum 命令行原始值）

- dashboard.png 90e39cb8daa2ec874107087a4e83f3e6e0de1b639f84a960f8fad94a6e2f6f80
- calendar.png 88e87f3762f036b3455ef4b03ddc97a8ee22044ff41e99a8d886f0038a20da0d
- file.png 4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf
- bbs.png 5e8bc6643b495db0ab5254541bd34a241ab5ed88e85b53a3ee5e38364e0a467a
- meeting.png d447f1fe24c4843722d5b4b21f6a20ba5c7cbcdd0c717f383ed10d52a91c1809
- attendance.png e43740aea4ae43b571b94a334e33176a1325107f7936c7dc09cb7e2bb4b7c145
- bam.png 38ba72a0ef72c4a0ec1119ab36443ff05527c949e013c1f595cda1e49d1c4af9
- cms-index.png 2e95be7a5863fb9d592c91b60417ce321dad9bf1156e8f4bf161a132ac4db115
- deployment.png 612f84e58105254d62bde5be7e0db5d94ef7dcd4e856fd2239650ce934baba34
- find-designer.png fae59b310dc056febec395863bf51583bda18b013cea20b8c16e141b12875070
- find-designer-skeleton.png d47feb903ab4af586af2722036ad2745a79bc3f2224b38ca8f856057d7aec6c1

目视抽样（1440x900）：dashboard 渐变标题「早安，admin」清晰可见、卡片层次完整；calendar 工作日历 header 与日期网格完整；find-designer 骨架截图为加载完成后空态（骨架在加载期一闪而过，其失效由 computedStyle + CDP 坐实，不靠截图判定）。

## 5 真值纪律与边界

- 文档内测试域用 <YOUR_TEST_DOMAIN>、<TEST_HOST> 占位，0 真值；无密钥、口令、IP、个人路径。
- 全程只读甄别，未修改任何源码/样式/契约/配置；未执行真实写操作；截图全部来自测试服真实页面并标注非物理真机。
- 本轮为 ≥1000 轮迭代中的一环，非终态；4 项可达真缺陷均仅给修复建议，留待后续独立 RED→GREEN 任务处理。
