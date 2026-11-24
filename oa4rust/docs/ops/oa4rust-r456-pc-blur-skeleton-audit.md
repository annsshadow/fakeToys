# R456 PC 端写域外双重 blur 残留与纯色骨架流光意图可达性实测核查

- 任务：t313（design-lead，Kind=work，只读甄别、不改源码）
- attempt_id：00590402-6c32-43b2-8b0b-f80f9b15d5d4
- 真父：9a28f183f357e52d9c21ffd52cdfdc18bbd04863（台账 §451），HEAD=UP 同值，AB=00，工作树干净
- 测试入口：https://<YOUR_TEST_DOMAIN>/oa4rust/app/...（<TEST_HOST> 真值仅存 gitignored runner，不入库）
- 证据目录：oa4rust/test-results/r456-pc-blur-skeleton-audit/（13 PNG + report-blur.json + report-skeleton.json，均 gitignored）

## 1 方法与真实性声明

- 真浏览器：Playwright Chromium headless 1440x900、DPR1、非物理真机；来源标注「测试服真实页面（NOT a physical device，非物理真机）」。
- 凭据：ssh oa4rust-test 运行时从 /usr/local/oa4rust/.admin-credentials 正则提取注入 OA_TEST_USER/OA_TEST_PASS；域名真值只在 gitignored runner，仓库 0 真值。
- 骨架触发：page.route 对真实业务 API 延迟 5500ms（放行 auth/who/login/logout/permission/menu），令 loading 态真实出现，无伪造数据；本轮 11 条业务 API 被延迟。
- CDP：DOM.enable + CSS.enable 后 CSS.getMatchedStylesForNode 坐实规则来源与选择器文本。
- 结果：blur 4 探针 + skeleton 13 页，errors=[]，pageerror=0；9 页骨架真实 observed=true。

## 2 候选 A：写域外同型双重 blur 4 处

判定依据：--ai-glass-blur 令牌真实值为 blur(12px)（ai-tokens.css:37 暗色 / :79 亮色）。规则写 backdrop-filter:blur(var(--ai-glass-blur)) 展开为 blur(blur(12px))，CSS 非法值被整条丢弃 → computedStyle backdropFilter=none。与 R455 已修 Calendar/Meeting 完全同型。

| 位置 | 选择器 | backdropFilter | bgImage | 判定 |
| --- | --- | --- | --- | --- |
| OrgDirectoryApp.vue:684 | .ai-page .modern-hero | none | linear-gradient(160deg,...) | 可达真缺陷 P3 |
| OrgDirectoryApp.vue:689 | .ai-page .person-section | none | none | 可达真缺陷 P3 |
| OrgDirectoryApp.vue:696 | .ai-page .dept-tree-panel | none | none | 可达真缺陷 P3 |
| ProcessWork.vue:3250 | .ai-page .work-hero | none | linear-gradient(160deg,...) | 可达真缺陷 P3 |

- CDP 坐实（OrgDirectory 页 data-v-f666bbbd）：`.ai-page .modern-hero[data-v-f666bbbd]{background:var(--ai-gradient-surface);...;backdrop-filter:blur(var(--ai-glass-blur))}`；`.ai-page .search-section[data-v-f666bbbd], .ai-page .tree-section[data-v-f666bbbd], .ai-page .person-section[data-v-f666bbbd]{...backdrop-filter:blur(var(--ai-glass-blur))}`；`.ai-page .dept-tree-panel[data-v-f666bbbd]{...backdrop-filter:blur(var(--ai-glass-blur))}`。
- CDP 坐实（ProcessWork 页 data-v-616066e3）：`.ai-page .work-hero[data-v-616066e3]{...backdrop-filter:blur(var(--ai-glass-blur))}`。
- 对照：全局 `.ai-panel{...-webkit-backdrop-filter:var(--ai-glass-blur);...}` 写法正确（无外层 blur），但被 scoped 双重 blur 规则按特异性/顺序盖过后仍为 none（非法值胜出即丢弃）。
- 4 处均为写域外（R455 写域仅 BBSForum/BamApp/FindDesigner/Calendar/Meeting），未改码；建议后续独立 RED→GREEN 修复：改 backdrop-filter:var(--ai-glass-blur)（去外层 blur）。
- 视觉影响轻微：header/卡片渐变背景、边框、圆角、阴影均正常，仅毛玻璃模糊缺失；目视 org 页整体现代风格完整。

## 3 候选 B：纯色背景 + ai-shimmer 意图交集 13 文件

方法：逐页延迟业务 API 触发 loading，实测骨架元素 computedStyle backgroundImage / backgroundColor / backgroundSize / animationName / animationDuration / getAnimations playState，并以 CDP 坐实规则来源。关键判据：ai-shimmer 仅改 background-position；若背景为纯色（bgImage=none）则位移无视觉流光，等同 R455 FindDesigner 已修缺陷。

### 3.1 可达真缺陷（纯色无流光，4 页）

| 页 | 选择器 | bgImage | bgColor | backgroundSize | animation | 判定 |
| --- | --- | --- | --- | --- | --- | --- |
| CommonApp | .loading-state .skel | none | rgb(248,250,252) | 200% 100% | ai-shimmer-6ece6c55 1.6s running | 可达真缺陷 P3 |
| FileManager | .skeleton-row | none | rgb(248,250,252) | auto | ai-shimmer 1.6s running | 可达真缺陷 P3（双重：纯色+size 被 shorthand 重置 auto） |
| PortalWidgetDesignerApp | .loading-state .skel | none | rgb(248,250,252) | 200% 100% | ai-shimmer-c315d3f6 1.6s running | 可达真缺陷 P3 |
| ServiceInvokeDesignerApp | .loading-state .skel | none | rgb(248,250,252) | 200% 100% | ai-shimmer-0c7ede59 1.6s running | 可达真缺陷 P3 |

- CDP 坐实 CommonApp：`.skel[data-v-6ece6c55]{border-radius:var(--radius-sm);background:var(--ai-skeleton-base);height:16px;animation:ai-shimmer-6ece6c55 1.6s var(--ai-ease-spring) infinite;background-size:200% 100%;margin-bottom:8px}` —— background 纯色 shorthand，无 background-image。
- CDP 坐实 FileManager：全局 `.ai-skeleton-line{...background:linear-gradient(90deg, var(--ai-skeleton-base) 25%, var(--ai-skeleton-shine) 50%, var(--ai-skeleton-base) 75%);background-size:200% 100%}` 存在，但 scoped `.skeleton-row[data-v-2b23cdd9], .sk[data-v-2b23cdd9]{...background:var(--ai-skeleton-base);height:40px;animation:ai-shimmer 1.6s...}` 以更高特异性用 background shorthand 覆盖为纯色，且 shorthand 把 background-size 重置为 auto（实测 backgroundSize=auto）——双重失效。
- 目视 skel-common.png：加载态为纯色灰块文字「加载中...」，无可见流光，与 computedStyle 结论一致。
- 4 页均建议后续独立 RED→GREEN：scoped 骨架改三色 linear-gradient(90deg,base 25%,shine 50%,base 75%) + background-size:200% 100%（参照 R455 FindDesigner 修法）或移除纯色 shorthand 覆盖。

### 3.2 正常（真实渐变流光，5 页）

| 页 | bgImage | animation | 判定 |
| --- | --- | --- | --- |
| PortalDesigner | linear-gradient(90deg,rgba(0,0,0,0),rgba(15,23,42,0.08),rgba(0,0,0,0)) | ai-shimmer-222a1824 0.22s running | 正常（流光可见） |
| PortalDictDesignerApp | 同三色渐变 | ai-shimmer-963dca66 0.22s running | 正常 |
| PortalPageDesignerApp | 同三色渐变 | ai-shimmer-292e7f22 0.22s running | 正常 |
| QueryDesigner | 同三色渐变 | ai-shimmer-39dc5187 0.22s running | 正常 |
| QueryExplorerApp | 同三色渐变 | ai-shimmer-dca68ddb 0.22s running | 正常 |

- 5 页源码均为 `.sk/.skel{background:var(--ai-skeleton-base);background-image:linear-gradient(90deg,transparent,var(--ai-skeleton-shine),transparent);background-size:200% 100%;animation:ai-shimmer var(--ai-motion-normal) linear infinite}`，background-image 独立声明未被 shorthand 覆盖，computedStyle 坐实真实渐变。
- animationDuration=0.22s 来自 --ai-motion-normal=220ms（ai-tokens.css:48/:88），为设计令牌原意（快速 shimmer），非缺陷；与纯色类 1.6s 不同但均有 background-position 动画 + 真实渐变，视觉流光有效。
- 目视 skel-query-designer.png：骨架区与空态「选择或创建查询」一致，无视觉破损。

### 3.3 不可达 / 正常业务态（未触发 loading，4 页）

| 页 | observed | 源码根因 | 判定 |
| --- | --- | --- | --- |
| IMChat | false | 骨架 v-if=msgLoading（:67），msgLoading 仅 loadFirstPage()（:352/:361）在选中会话时置 true；本轮无会话 id，未触发。骨架定义 :1249 .message-list .skeleton-row{background:var(--ai-skeleton-base)} 同型缺陷但加载态不可达 | 不可达 L1（需选会话交互） |
| PdfViewerApp | false | 骨架 v-else-if=loading（模板），loading 仅 loadPdf()（:45）在输入文件 ID 并点「加载」后置 true；本轮无输入 | 不可达 L1（需交互） |
| PortalScriptDesignerApp | false | 模板无 v-if loading 骨架（:5-11 仅 ScriptWorkbench 子组件）；.ai-skeleton-line 样式 :87 存在但本页模板未渲染该元素，加载由子组件承载 | 不可达 L1（本页模板零消费） |
| QueryViewDesignerApp | false | .sk 样式 :175（三色渐变）存在，但模板骨架由条件加载/交互（输入 Query Flag 加载视图）触发；本轮未加载 | 不可达 L1（需交互） |

- 4 页均按 R453 判据：无参数/无交互不置 loading 属正常业务态，严禁臆造为缺陷；其中 IMChat 骨架若未来进入真实加载态，存在与本次 PURE 类同型纯色风险，据实记录为「潜在、当前不可达」。

## 4 三分判定汇总

- ① 可达真缺陷：8 项 —— 候选 A 双重 blur 4 处（P3 视觉降级）+ 候选 B 纯色骨架无流光 4 页（P3 动效失效，其中 FileManager 双重失效）。均仅建议，不改码，留后续独立 RED→GREEN。
- ② 不可达 L1：4 项 —— IMChat / PdfViewerApp / PortalScriptDesignerApp / QueryViewDesignerApp（需交互或模板零消费）。
- ③ 正常：5 项 —— PortalDesigner / PortalDictDesigner / PortalPageDesigner / QueryDesigner / QueryExplorerApp 真实渐变 + 流光动画生效。
- 0 项 P 级（渐变未解析致文字不可见 / 对比度崩坏）；10 页原正常渲染零回归。

## 5 截图 sha256（sha256sum 命令行原始值，13 PNG）

- blur-org-modern-hero.png c203b2ac6677b110ffff8323128d702b96ef3bd62cca1209a0e279d480f4a9ef
- blur-org-person-section.png c203b2ac6677b110ffff8323128d702b96ef3bd62cca1209a0e279d480f4a9ef
- blur-org-dept-tree-panel.png c203b2ac6677b110ffff8323128d702b96ef3bd62cca1209a0e279d480f4a9ef
- blur-process-work-hero.png 0eccc409bd96d5829f6c311552d139ea33f1ad2bbea205e1cb8e9da4a55acbe9
- skel-common.png ff5e1d021fbe4e9f961c53c0bafdab8ba70dbcf74177fa53cd73734129a1eef4
- skel-file-manager.png 31ea02563c913acefdbf0a503266cbd30dee2038fde960a3c2e976452dc2faa3
- skel-portal-widget-designer.png 946eb513d4e656f7fb32aa1f1b13a1b61fd5d4cf6b6a768c3553ff4baa7f47a8
- skel-service-invoke-designer.png f4616d0072c0fdbd10756b89c8c7c2eeb4e662f8438cc70563e99ea457918ccf
- skel-portal-designer.png 3c3fb8d0dfaec1b8d6a585294e8bd68a01e4e96fbc4072e59f75602a8286c49f
- skel-portal-dict-designer.png 7cc778c85d1acebb2034d0617c932a1812c46cf3baab03a5af854bf1f3221c5b
- skel-portal-page-designer.png 5d072dc1a12bfc597dc4c13a670fac6f8d14ce959337ca44dcdcc7d4388950f1
- skel-query-designer.png 84f7d8314244a90159d8ff0e9cf10d12b2b4cae512fb34877923ecde9f3d847e
- skel-query-explorer.png d827d179eeba738cefb346702d9962f45e3fdf947706b2733bab81e1010d1ba1

目视抽样（1440x900）：org 组织通讯录 header 渐变标题清晰、卡片完整（模糊缺失肉眼轻微）；common 加载态纯色灰块无流光；query-designer 加载骨架与空态完整。

## 6 真值纪律与边界

- 文档内测试域用 <YOUR_TEST_DOMAIN>、<TEST_HOST> 占位，0 真值；无密钥、口令、IP、个人路径。
- 全程只读甄别，未修改任何源码/样式/契约/配置；未执行真实写操作；截图全部来自测试服真实页面并标注非物理真机。
- 本轮为 ≥1000 轮迭代中的一环，非终态；8 项可达真缺陷均仅给修复建议，留后续独立 RED→GREEN 任务。
