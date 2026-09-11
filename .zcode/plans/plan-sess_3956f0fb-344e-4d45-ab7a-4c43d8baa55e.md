## 目标与完成标准

把“全面优化 oa4rust + oa4rust-web”落实为五项可验收交付物，而不是以单一测试绿灯代替完成：

1. **接口完备**：前端 SDK 不再生成重复 `/jaxrs/jaxrs`；认证选项和响应类型符合约定；OpenAPI/parity 生成结果可确定性复核；生产与测试使用同一 Router 装配。
2. **安全加固**：消除已确认的 DOM XSS 和主页面动态代码执行入口；管理命令不经过 shell；CORS 覆盖真实生产栈；补供应链门禁。Token 改为 HttpOnly Cookie、严格 CSP 和富文本净化因涉及协议/部署架构，先形成经测试的独立后续批次，不伪装成已完成。
3. **性能优化**：API 客户端具备可恢复的超时/响应解析；性能基线拒绝错误响应并输出可信数据；不在不稳定共享 runner 上贸然设置绝对延迟门槛。
4. **前后端质量门禁**：前端 test/typecheck/只读 lint/build 纳入 CI；后端 fmt/check/test/integration/parity/OpenAPI 和安全检查可执行且覆盖目标。
5. **分批提交**：每次开始修改前先检查工作区；若有上一批修改，先验证并按批提交。当前工作区干净，因此首批开始前无需制造空提交。每一批完成验证后创建独立本地提交，再进入下一批；不推送远端。

## 实施顺序

### 批次 1：修复前端接口契约（最高优先级）

- 在 `oa4rust-web/packages/sdk/src/api.ts` 统一 URL 解析，默认同源且保留调用方完整 `/jaxrs/...` 路径；普通请求与上传共用规则，兼容完整 origin、部署子路径和 base 已含 `/jaxrs`，不破坏 `https://`。
- 让 `requireAuth: false` 真正禁止 Bearer 头，同时保留 cookie credentials；不新增“无 token 即抛错”的破坏性语义。
- 导出或提供最小可配置客户端入口；修正 `session.ts`、`router.ts`、`packages/apis` 与 OAuth 回调中重复包装的响应泛型和公开端点认证选项。
- 新增 Node 环境 Vitest：覆盖双前缀、base/path 组合、query、认证头、401/403、JSON 响应及 upload；用 mock `window/localStorage/fetch`，不为此引入 jsdom。
- 验证：定向 Vitest、全量 `pnpm test`、`pnpm typecheck`、`pnpm exec biome check .`、`pnpm build`。
- 通过后提交本批，再开始下一批。

### 批次 2：消除直接前端 XSS 与伪沙盒

- 将 `apps/desktop/src/utils/toast.ts` 的动态 HTML 改为 `createElement` + `textContent`，复用同文件已有安全 DOM 模式，不引入 sanitizer。
- 将 8 份重复 `confirmMsg` 迁移到共享安全实现；修正共享实现不可达的 overlay/fallback 判断，但不改变确认语义。
- 增加共享异步复制 helper，并迁移已确认的 clipboard 调用，只有真实写入成功才提示成功，失败被捕获并提示。
- 删除 `ProcessDesigner.vue` 中不可达且错误的 `new Function` 沙盒路径；可见“运行测试”明确标注为校验/模拟，不再暗示真实执行；移除对 `window/document/localStorage/fetch` 的危险自动补全推荐。不会在浏览器主 realm 中实现脚本执行。
- 默认关闭生产 sourcemap；开发模式保持调试能力。
- 增加静态安全回归测试，禁止共享 toast 回归 `innerHTML`、禁止 ProcessDesigner 出现 `new Function/eval`；若现有依赖无法做 DOM 级测试，则用可测试的安全 DOM 构造函数加最小 mock，避免新增大依赖。
- 验证前端全套门禁并提交。

### 批次 3：统一后端生产/测试 Router 与 CORS

- 先逐项对照 `src/main.rs` 和 `src/lib.rs` 的 route、Extension、middleware 顺序，提取唯一共享 API Router builder；`main.rs` 仅保留进程级 OpenAPI/MCP/静态文件/监听装配。
- 把生产独有的 organization、WebSocket、preview、signature、数据库 Extension 和行为比较能力纳入可配置的共享 builder，避免测试继续覆盖另一套应用。
- 在统一栈上挂载现有 CORS；按实际前端方法允许 GET/POST/PUT/PATCH/DELETE/HEAD/OPTIONS。非法 origin 配置显式报错或告警，不静默形成空策略。
- 补合法/非法 origin、带凭据预检、生产独有 route 非 404 的回归测试，并验证预检不会被认证或限流提前拒绝。
- 运行 shared/目标测试、workspace lib、integration runner；数据库不可用时明确记录，不把 skip 当通过。验证后提交。

### 批次 4：加固 Console 管理命令

- 在 `crates/console` 提取可单测的命令规范；命令名严格映射到固定 executable，参数逐命令允许列表校验。
- 完全移除 `sh -c`，使用 `tokio::process::Command` 直接 argv；加入超时、进程终止和 stdout/stderr 上限，保留 handler 内 Admin RBAC。
- 测试非管理员、未知命令、换行/重定向/命令替换/反引号等输入、参数不做 shell 展开、超时和输出截断；真实系统命令测试限定 Unix，Windows 运行纯校验测试。
- 运行 console、workspace lib 与集成测试，验证后提交。

### 批次 5：OpenAPI/parity 确定性与接口漂移门禁

- 让两个生成器去掉 wall-clock 时间，增加只读 `--check` 模式，在临时内容中生成并比较，不修改工作树。
- 修正 parity 元数据：实际生成测试数与跳过数分开，非法/无法表示的 route 必须进入显式 allowlist，不能虚报覆盖。
- 将 OpenAPI 安全检查改为结构化验证 `ApiDoc::openapi()`；先报告实际 `(method,path)` 缺失/多余集合，再修正生成器扫描范围。不会机械接受数千行生成差异。
- CI 加生成器 freshness、OpenAPI 结构测试和 parity 门禁；行为对比现有大量历史失败先产出明确 baseline/聚类，只有确认无既有失败或设定受控 allowlist 后才取消 `continue-on-error`。
- 验证生成器 check、openapi/parity tests、workspace tests 后提交。

### 批次 6：可信性能基线与 API 韧性

- SDK 请求加入可配置 `AbortController` 超时，支持调用方 signal；正确处理 204、JSON、文本/文件 content-type，保持现有 `ApiResponse<T>` 约定并给错误提供清晰上下文。
- 修复 `tests/perf_baseline.rs`：warm-up 和采样必须得到预期成功状态/结构，错误请求不计样本且令测试失败；端点分开采集，恢复进程环境变量，输出稳定 JSON（样本、avg/p50/p95/p99/max、commit）。
- 首先作为手动/scheduled artifact 收集基线；得到稳定历史数据后再设回归阈值，避免共享 runner 假阳性。
- 验证 SDK 测试、前端全套门禁及后端性能测试可运行性，提交。

### 批次 7：CI 与供应链基线

- 为 `oa4rust-web` 增加冻结 lockfile 安装、test、typecheck、只读 Biome、build；将本地 `lint --write` 与 CI `lint:check` 分开。
- 审计 tracked `.env` 是否只有安全模板内容；若含真实凭据，只停止继续跟踪并报告需人工轮换，不自动改写 Git 历史。
- 引入固定版本的 `cargo-audit`/`cargo-deny` 与最小 `deny.toml`，分类处理现存 advisory、许可证和 source；增加前端生产依赖 audit。依赖安装不会上传代码或秘密。
- Pin Actions 与行为比较镜像到确定版本/摘要；若无法可靠确定 digest，保留为明确未完成项而不猜测。
- 验证 workflow 语法、本地对应命令和两端完整构建，提交。

## 完成审计清单

最终逐项提供实际证据：

- SDK URL、认证、响应、超时的测试名称与结果。
- 前端 XSS/动态执行扫描结果，Vitest/typecheck/Biome/build 结果。
- 单一 Router 的生产调用路径及 CORS/生产独有 route 测试。
- Console 无 shell 执行的代码证据及攻击载荷测试。
- OpenAPI/parity `--check`、集合差异和测试计数证据。
- 性能采样成功率及机器可读报告；没有稳定数据则明确“不宣称性能达标”。
- CI 每个门禁与它覆盖的目标映射；audit 发现的问题逐项分类。
- 每批本地提交哈希、修改文件和验证状态；最后确认工作区干净。
- 尚需部署/协议决策的 HttpOnly Cookie、CSRF、严格 CSP、富文本净化、真正脚本沙盒单独列为未完成架构项，不以本轮局部修复掩盖。

## 边界与假设

- 用户已明确要求分批提交，因此会创建本地提交；不会 push、改写历史或 force 操作。
- 当前分支是 `main` 且领先远端 2 个提交。为避免继续直接堆叠主分支，实现开始时会先创建独立工作分支，再按批提交。
- 不自动轮换凭据、删除环境文件、改生产基础设施或切换认证协议；这些属于需要真实部署信息的高影响动作。
- 若某批暴露大规模历史漂移或现有测试失败，将先隔离根因并如实记录，不把无关重构混入该批。