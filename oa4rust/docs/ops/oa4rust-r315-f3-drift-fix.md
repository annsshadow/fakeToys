# R315 F3 五处封装方法/路径漂移修复（t86）

## 背景与依据

t80 静态审计（docs/ops/oa4rust-r314-contract-health-audit.md §F3，第 61-70 行）坐实 packages/apis/src/index.ts 五处封装方法/路径与后端真实注册不符；t81 纳入 route-methods 守卫并写入 F3_KNOWN_PENDING 冻结快照显式标注待修。五处均无 @dead 标注、当前无视图消费方，属静态坐实潜伏缺陷，非线上已触发故障。

本轮以**后端真实路由为准（方法+路径双维度）**逐处核对 .route 源码 + backend-route-methods.fixture，不臆造后端不存在的路径。队长就唯一无对应方法的 jpush 一处裁定：补 @dead 标注保留原路径（不补后端、不改 GET）。

## 五处逐处处置

| 封装 | 原声明 | 修复/处置 | 后端依据 |
| --- | --- | --- | --- |
| generalApi.dictItemCreate | POST /api/general/dict/item/{dictId} | POST /api/general/dict/item/create，dictId 移入 body | general_core_entity/src/lib.rs:1078，handler 从 body.dictId 读取 |
| queryViewApi.search | GET /api/queryview/search | POST /api/queryview/search，{key} | query_assemble_surface/src/lib.rs:617 u2::search_post |
| queryViewApi.importModelList | GET /api/queryview/importmodel/list/{query}/{flag} | POST 同路径，空 body | query_assemble_surface/src/lib.rs:404 |
| programCenterApi.moduleInvoke | POST /api/program_center/invoke/{flag} | POST /api/program_center/invoke/{flag}/execute | program_center/src/routes.rs:468 该路径仅 GET/PUT/DELETE；:632 execute 为 POST |
| jpushApi.templateUpdate | PUT /api/jpush/core/entity/template/{id} | 补 /** @dead */ 保留原路径 | jpush_core_entity/src/lib.rs:273-274 仅 GET list/template/{id}，全仓无 update/save 路由 |

前四处为方法/路径真修；jpush 一处后端确无更新方法，补后端属臆造产品语义、改 GET 会把「更新」偷换成「查询」，故按队长裁定标 @dead 明示不可用。

## RED → GREEN 与守卫同步

新增契约 apiWrapperDrift.R315.contract.test.ts（vi.mock + toHaveBeenCalledWith 真断言方法+URL，同 R313 范式）：原始源码下 5 failed（四处方法/路径不符 + jpush 无 @dead）；修复后 5 passed。第 5 条为 @dead-known 防回退守护（断言源码含 @dead 且 fixture 对该路径仅 GET，无 PUT/POST）。

route-methods.test.ts 冻结快照同步：四处旧漂移签名从 API_LEGACY_MISMATCHES 移除，并以 F3_FIXED 集合逐条真断言 classify()===ok（非静默删除、非 EXEMPT 放行）；F3_KNOWN_PENDING 收敛为 jpush 一条（@dead-known，仍在快照内）。守卫 2 tests 绿；R313 契约 4 tests 绿；覆盖率测试 651 tests 绿。

## 真浏览器封装路径探针（非业务 UI）

遵从本轮部署口径：不重建/不替换测试服 web 目录（t85 独占部署），在现有测试服上下文用 Playwright Chromium headless + CDP fetch 直调五个后端真实端点。来源标注：封装路径探针，Playwright Chromium headless against deployed test server; not a physical device；非业务 UI 流程（五处无视图消费方）。

- 四处修复端点：general/dict/item/create、queryview/search、queryview/importmodel/list/{q}/{f}、program_center/invoke/{flag}/execute —— 全部 status=200、contentType=application/json、isJson=true。
- jpush @dead-known：GET /api/jpush/core/entity/template/probe 返回 404 + application/json（probe 模板不存在；非 405 证明 GET 路由存在，佐证该路径无 update 方法）。
- pageerror=0；evidence：oa4rust/test-results/r315-f3-probe/evidence.json（gitignored）。

服务地址与凭据：测试服域名、SSH 连接与登录凭据不入库，运行时从 <YOUR_TEST_DOMAIN> 对应环境变量/本地凭据文件读取；本轮未改后端、未部署前端。

## 验证

- cd oa4rust && pnpm exec vitest run frontend/packages/apis/src/apiWrapperDrift.R315.contract.test.ts → 5 passed
- cd oa4rust && pnpm exec vitest run tests/contracts/route-methods.test.ts → 2 passed
- cd oa4rust && pnpm run typecheck → 6/6 包 Done，EXIT=0
- 项目 ≥1000 轮持续迭代，本轮为其中一环，非终态。
