# R315 D1 全局搜索有数据不渲染修复

## 来源与范围

- 任务：t87 [R315 D1]，工作台与个人域页面。
- 真 bug：搜索接口返回数据但页面始终显示初始态，用户无法检索到任何内容。
- inScope：frontend/apps/desktop/src/views/SearchApp.vue、新契约 SearchApp.searchEnvelope.R315.contract.test.ts、本文档。
- 后端只读核对，未改后端与 packages/apis。

## 根因

- 后端 /api/queryview/search 返回分组信封 { data: { designList:[{id,name,category}], viewList:[...] } }（字段来源 backend/crates/query_assemble_surface/src/u2_closures.rs:659-660）。
- 旧实现 results.value = (r?.data ?? []) as any[] 把整个信封对象塞进声明为数组的 results；对象无 .length，模板 v-if results.length>0、v-else-if results.length===0&&queried 均不命中，页面停在 v-else 初始态「输入关键词开始搜索」。
- 真响应探针实证：关键词 a 返回 3 条 designList，但页面 items=0、panelText 恒为初始态。

## 修复（test-first RED→GREEN）

- RED 契约 SearchApp.searchEnvelope.R315.contract.test.ts：5 项真断言（不直接赋 r.data、解包 designList/viewList、title 取 name/desc 取 category/type 区分、合并为真正数组、保留端点/Runner/失败提示）。首跑 4 failed / 1 passed。
- GREEN 实现：doSearch 内 const envelope = r?.data ?? {}，designList/viewList 各按 Array.isArray 归一化，映射为含 id、title、type(design/view)、desc 的扁平对象后合并展开赋给 results.value；保留 POST 端点、searchRun.run(signal) 与 describeListError 失败提示。
- 不改端点方法与路径，不动后端，不动 AI 令牌与既有 testid/初始态文案。

## 验证

- 新契约 GREEN：5 passed / 5。
- 现有三契约不回退：SearchApp.business / searchPages / Pages.aiStyle.R305b 共 3 files / 10 tests passed。
- pnpm run typecheck exit=0（Scope 6/7 全 Done）。
- 桌面构建 exit=0；测试服原子部署 Linux web 产物，远端入口哈希核验通过。
- 真浏览器（Playwright Chromium headless，1440×900，非物理真机）登录后 /app/search：关键词 a 渲染 3 条真实结果（S3 查询…）；无匹配词显示「未找到结果，请尝试其它关键词」；pageErrors=[]、apiFailures=[]，5 项断言全 true。

## 证据索引（真值不入仓）

- 截图与 evidence：test-results/r315-search-fix/（gitignored）。
- 测试域名用 <YOUR_TEST_DOMAIN> 占位；口令仅运行期从远端凭据脚本读取，仓库无真值。
