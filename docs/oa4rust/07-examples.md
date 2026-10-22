# 07 · 典型使用示例

> 以下示例基于当前代码已注册路由（`crates/auth/src/lib.rs` 等）。响应信封字段见 [03-data-and-api.md](03-data-and-api.md)。

## 1. 登录 → 会话 → 业务调用（curl 全流程）

```bash
BASE=http://localhost:3000

# 1) 登录（成功后 Set-Cookie: HttpOnly 会话）
curl -i -c cookies.txt -X POST "$BASE/api/authentication" \
  -H 'Content-Type: application/json' \
  -d '{"credential":"xadmin","password":"your-password"}'
# → {"type":"success","data":{...用户信息...},"message":"",...}

# 2) 携带 Cookie 调业务接口（示例：组织单位列表）
curl -b cookies.txt "$BASE/api/authentication/unit/list"

# 3) 查询当前用户
curl -b cookies.txt "$BASE/api/authentication/who"

# 4) 会话续期（前端 SDK 在 401 时自动做同样的事）
curl -b cookies.txt -X POST "$BASE/api/authentication/refresh"

# 5) 登出（服务端吊销会话）
curl -b cookies.txt -X POST "$BASE/api/authentication/logout"
```

**失败样例**（密码错误）：

```json
{ "type": "error", "message": "......", "prompt": "...", "...": "o2server 对齐信封" }
```

## 2. 流程审批（通过 / 驳回）

```bash
# 待办列表 → 取 task id 后：
curl -b cookies.txt -X POST "$BASE/api/task/{id}/complete" \
  -H 'Content-Type: application/json' \
  -d '{"opinion":"同意"}'
curl -b cookies.txt -X POST "$BASE/api/task/{id}/reject" \
  -H 'Content-Type: application/json' \
  -d '{"opinion":"材料不全，退回"}'
```

桌面端对应视图 `frontend/apps/desktop/src/views/`（流程中心/表单）；移动端 `process` 页（待办/已办/我发起 + 通过/驳回）。端点与后端注册路由由契约守卫对账。

## 3. WebSocket 实时消息（浏览器端）

```javascript
const ws = new WebSocket("ws://localhost:3000/ws/realtime");
ws.onopen = () => ws.send(JSON.stringify({ /* 协议消息：RealtimeMessage 结构 */ }));
ws.onmessage = (e) => console.log("realtime:", e.data);

// 房间通道与统计
// ws://localhost:3000/ws/realtime/room/{room_id}
// ws://localhost:3000/ws/realtime/room/{room_id}/stats
```

生产建议：桌面端直接用 `@oa4rust/sdk` 的 `websocket.ts` 封装（自动重连/会话整合）。

## 4. MCP 工具调用（JSON-RPC 2.0）

> 需以 `cargo run -- --http` 启动；`Authorization` 头透传复用会话认证。

```bash
# 1) initialize
curl -s -X POST http://localhost:3000/mcp -H 'Content-Type: application/json' -d '{
  "jsonrpc":"2.0","id":1,"method":"initialize","params":{}
}'
# → {"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05",
#     "capabilities":{"tools":{"listChanged":false}},
#     "serverInfo":{"name":"oa4rust-mcp","version":"0.1.0"}}}

# 2) 列出工具
curl -s -X POST http://localhost:3000/mcp -H 'Content-Type: application/json' -d '{
  "jsonrpc":"2.0","id":2,"method":"tools/list"
}'

# 3) 调用工具
curl -s -X POST http://localhost:3000/mcp -H 'Content-Type: application/json' -d '{
  "jsonrpc":"2.0","id":3,"method":"tools/call",
  "params":{ "name":"<工具名>", "arguments":{ } }
}'
# 未知 method → {"error":{"code":-32601,"message":"method not found: ..."}}
```

## 5. OpenAPI 获取

```bash
curl http://localhost:3000/openapi.json -o openapi.json
# 可导入 Postman / Apifox / Swagger Editor 生成客户端与文档
```

## 6. 新增一个业务 API 的最小路径（后端开发示例）

以既有 crate 追加端点为例（推荐模式，参考 `bbs_assemble_control`）：

1. `crates/<module>/src/routes.rs`（或 `u2.rs`）注册路由：`.route("/api/<resource>", get(list))`。
2. handler 直接用 `pool` 执行 SQL；行转 JSON 复用 `shared::response::row_to_json`；返回 `ActionResult::success(data)` / `AppError`。
3. 写操作走 `shared::crud`（`CrudSpec` + `crud_create/save/delete`）或手写带校验的 handler（`input_validation`）。
4. 在 `crates/<module>/src/tests.rs` 补手写单测（生成测试走 `scripts/generate_handler_tests.py`）。
5. 更新三端契约 fixture（`tests/contracts/`）与前端 `@oa4rust/apis`；移动端按需在 `services/index.ts` 暴露。
6. 门禁：`cargo check --workspace --locked && cargo test --workspace --lib --locked`；workspace 根 `pnpm test && pnpm typecheck && pnpm lint:check`。
7. 重新运行 `backend/scripts/regen_endpoints.py` 刷新迁移状态文档（单一信息源）（待确认：该脚本当前实际产出 `tests/behavior_comparison/endpoints.rs`；清单文档自述的生成器 `scripts/gen_inventory.py` 不在 `backend/scripts/`）。

## 7. 数据迁移演练（本地全量）

```bash
cd oa4rust/backend
docker compose up -d postgres
cargo run -- --migrate-only
# → migrations: ~111 applied（全量后约 379 表）
# parity 全量验证：
cargo test -p parity --lib --locked      # 期望 4143/4143
```

## 8. 前端接入新接口（TypeScript 示例）

```typescript
// 桌面端：经 @oa4rust/sdk 的 ApiClient（自动 401 refresh 重试、统一 ApiError）
import { api } from "@oa4rust/sdk";

const res = await api.get("/api/authentication/who");
if (res.type === "success") {
  console.log(res.data); // 业务数据
}

// 移动端：services/http.ts 的 uni.request 传输层，语义与桌面一致
import { http } from "./services/http";
const who = await http.get("/api/authentication/who");
```
