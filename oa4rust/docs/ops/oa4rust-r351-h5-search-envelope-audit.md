# R351 H5 搜索页搜索信封甄别（未解包致全来源检索崩溃）

> 审计来源：2026-11-22 测试服真浏览器 E2E（Playwright Chromium 390x844 headless，isMobile/hasTouch/DPR2），**非物理真机**；证据截图均为 headless 浏览器真实渲染，非设备截图。
> 域名/凭据脱敏：仅使用占位符 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，0 真值入库。
> 本轮为只读审计：未改任何源码/契约/后端/SDK/其它页，仅提取缺陷与最小修复方向。

## 1. 范围与甄别

- 页面：`oa4rust/mobile/src/pages/search/search.vue`（209 行，H5 搜索页，路由 `pages/search/search`，`pages.json:18`）。
- H5 入口 `/oa4rust-h5/`，API 前缀 `/oa4rust-api`（`main.ts:25`：VITE_OA_API_BASE || /oa4rust-api）。
- 覆盖甄别：`docs/ops` 对 `search.vue` / H5 全局检索 0 独立命中；R314/R315-f3/R315-search-envelope-fix（桌面 SearchApp）/R344/R345（桌面 QueryViewApp）/R317（H5 六模块）均非本页深审。本轮首次覆盖。
- E2E 共 2 轮：v1（三关键词 UI 流程 + 后端信封探针）、v2（bbs 信封内部结构 + 多词真值探针）。0 pageerror（两轮）。

## 2. 真缺陷（1 个 P1）

### P1：两个搜索来源均为分组信封，UI 用 `(g.data ?? []).map` 直接遍历对象，任何关键词都崩溃为零结果+错误提示

- 源码链路：
  - `oa4rust/mobile/src/pages/search/search.vue:37-38`：`const g = await searchApi.global(kw); out.push(...((g.data ?? []) as Row[]).map((r) => ({ ...r, _from: "全局" })))`。
  - `oa4rust/mobile/src/pages/search/search.vue:43-44`：`const b = await searchApi.bbsSubject(kw); out.push(...((b.data ?? []) as Row[]).map((r) => ({ ...r, _from: "论坛" })))`。
  - `oa4rust/mobile/src/services/index.ts:24-26` `list()` 仅做类型断言不解包；`:546` global = POST `/api/queryview/search`；`:548-550` bbsSubject = PUT `/api/bbs/assemble/control/subject/search/list/page/1/count/20`。
  - `oa4rust/mobile/src/utils/loadList.ts:25-29` `describeLoadError` 把 TypeError.message 拼进 UI 文案。
- 实测证据（v1 probe + v2 probe，页面上下文内带凭据 fetch）：
  - 全局来源 POST `/api/queryview/search` → 200，body `data` 是**对象**非数组（dataIsArray=false），键 `[designList, key, viewList]`；关键词 "a" 时 designList=3、viewList=0，关键词 "e" 时 designList=10、viewList=1（真实结果存在）。
  - 论坛来源 PUT `/api/bbs/.../search/list/page/1/count/20` → 200，`data` 是对象（dataIsArray=false），内部键 `[data, total]`，真实条目在内层 `data` 数组（4 条）。
  - UI 表现（v1 S1/S2/S3，关键词 "a" / 随机不存在的词 / "R344"）：三态一致——卡片 0，页面只显示 `全局检索失败：(e.data ?? []).map is not a function；论坛检索失败：(e.data ?? []).map is not a function 重试`，`未找到结果` 空态从未出现。0 pageerror（异常被各自 try/catch 捕获转为文案）。
- 影响：所有关键词搜索 100% 失败——后端有真实结果（designList 最多 10 条）也无法渲染；用户只看到晦涩的 JS 报错文案，且错误分支掩盖了真空态，搜索功能实质不可用。与 R344（桌面 QueryViewApp 同端点）同型，但本页两个来源同时中招。
- 最小修复方向（本轮不改）：
  1. 全局来源对齐 R345 方案 A / 桌面 SearchApp R315 范式：`const env = (g.data ?? {}) as { designList?: Row[]; viewList?: Row[] }; const rowsG = [...(Array.isArray(env.designList) ? env.designList : []), ...(Array.isArray(env.viewList) ? env.viewList : [])]` 后再 map 加 `_from:"全局"`。
  2. 论坛来源解包内层：`const rowsB = Array.isArray(b.data) ? b.data : Array.isArray((b.data as {data?: Row[]})?.data) ? (b.data as {data:Row[]}).data : []`。
  3. 两来源仍各自 try/catch，失败提示与空态语义保持不变（`listState` 错误优先于空态）。

## 3. 取证限制与产品语义差异

- 取证限制：探针关键词通过真实 UI 输入框 + 搜索按钮触发（含 `@confirm` 路径未单独验，但 `@tap` 路径已坐实），三态覆盖有结果词/无结果词/R344 词。
- 取证限制：未验证论坛结果卡深链跳转（`/pages/bbs/topic?id=...`），因当前缺陷下论坛来源恒 0 行、无从点击；修复后需补测。
- 产品语义差异：v2 探针关键词 "a"/"e" 命中真实数据（designList 3/10），"R344"/"流程"/"admin" 命中 0 条——属数据分布差异，非缺陷；缺陷在任何关键词下均复现，与命中数据无关。
- 产品语义差异：本页异常被 try/catch 捕获为可见错误文案而非 pageerror，符合 `describeLoadError` 设计意图（fail-loud），但错误文案泄露内部 JS 实现细节，属可观测性问题，随 P1 修复自然消失。

## 4. 证据与命令

- 截图（gitignored）：`oa4rust/test-results/r351-h5-search-envelope/` 下 `search-initial.png`、`search-S1.png`、`search-S2-empty.png`、`search-S3.png`，均为 390x844 isMobile/hasTouch DPR2 headless 真浏览器全页截图，**非物理真机**。
- evidence：`evidence.json`（v1 信封探针 + UI 三态）、`evidence-v2.json`（v2 bbs 内层结构 + 多词 designList/viewList 真值）。
- 脚本（gitignored）：`test-results/r351-h5-search-envelope.cjs` / `-v2.cjs` 及对应 `*-run.sh`（凭据运行期经受控 ssh 跳板读取（别名/真值均不入库））。
- 只读约束：本轮未修改任何源码/契约/后端/SDK/组件/台账/其它页；上述修复方向仅为建议。