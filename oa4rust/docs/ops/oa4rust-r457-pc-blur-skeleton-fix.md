# R457 PC 端写域外 8 项 AI 视觉真缺陷修复（4 双重 blur + 4 纯色骨架）

- 任务：t314（page-core，Kind=implementation）
- attempt_id：1ebe4349-29b4-436a-9f33-cf85f38849ee
- 真父：bef690b3d616b3896f6d1a7c8693a7c8fe6feca5（台账 §452），HEAD=UP 同值，AB=0 0，工作树干净
- 依据：t313 甄别（commit 218b29ad，文档 oa4rust-r456-pc-blur-skeleton-audit.md），8 项均 P3 视觉降级轻微，业务零改
- 测试入口：https://<YOUR_TEST_DOMAIN>/oa4rust/app/...（<TEST_HOST> 真值仅存 gitignored runner，不入库）
- 证据目录：oa4rust/test-results/r457-pc-blur-skeleton-fix/（13 PNG + report.json，均 gitignored）

## 1 修复内容（仅视觉 CSS，业务零改）

### 候选 A：双重 blur 4 处

根因：--ai-glass-blur 令牌值=blur(12px)（ai-tokens.css:37 暗/:79 亮），blur(var(--ai-glass-blur)) 展开 blur(blur(12px)) 非法整条丢弃 → computedStyle backdropFilter=none。

| 位置 | 改前 | 改后 |
| --- | --- | --- |
| OrgDirectoryApp.vue:684 | backdrop-filter: blur(var(--ai-glass-blur)) | backdrop-filter: var(--ai-glass-blur) |
| OrgDirectoryApp.vue:689 | 同上 | 同上 |
| OrgDirectoryApp.vue:696 | 同上 | 同上 |
| ProcessWork.vue:3250 | 同上 | 同上 |

-webkit-backdrop-filter 原已是 var(--ai-glass-blur)（无包裹），保留不动。

### 候选 B：纯色骨架无流光 4 页

根因：scoped background shorthand 为纯色 var(--ai-skeleton-base)，ai-shimmer 仅改 background-position，无渐变可位移；FileManager 另叠加 shorthand 把 background-size 重置 auto 双重失效。

修复：background 改三色 linear-gradient(90deg,var(--ai-skeleton-base) 25%,var(--ai-skeleton-shine) 50%,var(--ai-skeleton-base) 75%)，保留 background-size:200% 100% 与 animation:ai-shimmer。

- CommonApp.vue:219 .skel
- FileManager.vue:1506 .skeleton-row、:1507 .sk（两处原本无 size，新增 background-size:200% 100%）
- PortalWidgetDesignerApp.vue:195 .skel
- ServiceInvokeDesignerApp.vue:235 .skel

### 零改范围

- 业务结构/接口/data-testid/端点/payload/后端/SDK/main.ts/adminNav.ts 零改。
- R455 写域（BBSForum/BamApp/FindDesigner/Calendar/Meeting）零改。
- 其它 view、19 页 H5 零改。
- 不可达 L1（IMChat/PdfViewerApp/PortalScriptDesignerApp/QueryViewDesignerApp）据实记录不改码。
- GRAD 5 页（PortalDesigner/PortalDictDesigner/PortalPageDesigner/QueryDesigner/QueryExplorer）渐变定义零回归。

## 2 RED → GREEN 契约

- 契约：oa4rust/frontend/apps/desktop/src/views/R457.aiRenderFix.contract.test.ts（13 it，真断言非桩）。
- 覆盖：①OrgDirectory 3 处 blur ②ProcessWork 1 处 blur ③CommonApp/FileManager(两选择器)/PortalWidget/ServiceInvoke 骨架三色渐变+size200%+shimmer ④零回归守卫（令牌定义存在、GRAD 5 页渐变定义不动）。
- 真父 bef690b3d 定点 RED：7 failed / 6 passed（13）；失败全为真实断言（2 blur + 5 骨架），0 桩错误。
- 修复版 GREEN：13 / 13 全通过。

## 3 三 verify（逐项真实执行）

- V1 `pnpm vitest run frontend/apps/desktop/src/views/R457.aiRenderFix.contract.test.ts` → exit 0，13/13。
- V2 `pnpm test` → exit 0，317 files / 3134 tests，0 failed。
- V3 `pnpm typecheck` → exit 0。

## 4 构建与部署

- 构建：oa4rust 根 `pnpm run build` exit 0，入口 index-Bd4ITG3d.js。
- 部署：整目录 tar → scp → sha256 比对一致 → 远端 /usr/local/oa4rust 建 web.incoming-r457 → grep 入口 → mv 原子替换，DEPLOY_R457_PC_FULL_OK。

## 5 真浏览器实证（1440x900 headless DPR1 非物理真机）

- 来源：Playwright Chromium headless 1440x900 → 测试服真实页面（NOT a physical device，非物理真机）；凭据运行时经 ssh 从 .admin-credentials 提取注入，不入库。
- 结果：R457_DONE total=45 passed=45 failed=0 errors=0，pageerror=0。
- 4 blur 探针 computedStyle backdropFilter=blur(12px)（父版 none）。
- 4 页骨架 computedStyle bgImage=linear-gradient(90deg,rgb(248,250,252) 25%,rgba(15,23,42,0.08) 50%,rgb(248,250,252) 75%)，backgroundSize=200% 100%，animationName 含 ai-shimmer，playState=running。
- GRAD 5 页渐变零回归，0 pageerror。
- 3 张关键截图肉眼确认 1440x900：组织通讯录渐变 header 完整、文件管理加载骨架、查询设计器空态零回归。

## 6 截图 sha256（sha256sum 命令行原始值，13 PNG）

- blur-org-dept-tree-panel.png 2a0328c1a300b1bbd929ad1e2358b5c1b00e164d5414c811c1d431e7258dfd4c
- blur-org-modern-hero.png 3a5c5b703752d7bfb91129525be1a031aa5ec09c64bef6cddc226da24eebca3e
- blur-org-person-section.png 2a0328c1a300b1bbd929ad1e2358b5c1b00e164d5414c811c1d431e7258dfd4c
- blur-process-work-hero.png 0a97d9bedf64ba8e70c433f20f2a71582749352959661e39cf8fcf32007c9c70
- skel-common.png c24be7a71f38d54a7ee2e88d9ac357c486b9ecc3f26bad2a6978e0f938aa53da
- skel-file-manager.png 31ea02563c913acefdbf0a503266cbd30dee2038fde960a3c2e976452dc2faa3
- skel-portal-designer.png 3c3fb8d0dfaec1b8d6a585294e8bd68a01e4e96fbc4072e59f75602a8286c49f
- skel-portal-dict-designer.png b67740bcc7f92bb2df172e4f47fc15fb7296bd33a3d1517fe0a6a13ffe9b04a4
- skel-portal-page-designer.png 8bc472fbd3102d9191a2ed12f42f582cc0b87e1319a155c19228941e17973234
- skel-portal-widget-designer.png 74537e97164b4a8c77909100e7e909140200ffd78125751d65cf7fcea224163d
- skel-query-designer.png 84f7d8314244a90159d8ff0e9cf10d12b2b4cae512fb34877923ecde9f3d847e
- skel-query-explorer.png ba6aa4cab728e52e1fc9efc8f330116ea43725c66a761208d8db44717088744f
- skel-service-invoke-designer.png 42cea809118bfdf6c5addba7ff5e44098a534056797e830dcb7906bb9066cd87

## 7 真值纪律与边界

- 测试域用 <YOUR_TEST_DOMAIN>、<TEST_HOST> 占位，0 真值；无密钥、口令、IP、个人路径。
- 本轮为 ≥1000 轮迭代中的一环，非终态。

