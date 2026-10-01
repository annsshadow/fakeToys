# 浏览器 Session Cookie 安全契约

## 服务端配置

生产环境必须显式配置以下变量，缺失或不安全时服务拒绝启动：

```dotenv
RUST_ENV=production
APP_PUBLIC_ORIGIN=https://oa.example.com
AUTH_COOKIE_SECURE=true
SESSION_TTL_SECONDS=7200
```

`APP_PUBLIC_ORIGIN` 只能包含 scheme、host 和可选 port，不能包含路径、查询串或尾部 `/`。它同时是凭据型 CORS 的唯一允许来源和 Cookie 写请求的 CSRF Origin 基准。测试/开发可显式使用 HTTP origin 与 `AUTH_COOKIE_SECURE=false`。

## Cookie 与接口契约

唯一 Cookie 名称为 `oa4rust_session`。登录和刷新响应设置 host-only Cookie（不设置 `Domain`），属性为 `HttpOnly; SameSite=Lax; Path=/`；生产设置 `Secure`。同时设置 `Max-Age` 与 `Expires`，二者均与 `SESSION_TTL_SECONDS` 及服务端 Session/Redis TTL 一致。

- 登录：成功后设置新 Cookie；响应体不再回显正式 session token（token-free body），
  浏览器与 CLI 均以 Cookie 为权威会话载体。
- 刷新：请求不携带 body token，从 Cookie 或 Bearer 认证结果轮换会话并重设 Cookie。
- 登出：允许空 body、重复调用始终成功；清理对应会话并返回 `Max-Age=0` 和过期 `Expires` 的 Cookie。
- 登录、刷新和登出响应均带 `Cache-Control: no-store`。

认证提取严格采用 Cookie 优先：请求一旦存在 `oa4rust_session`，就不会回退到 Bearer；空值、伪造值或过期 Cookie 均按 Cookie 认证失败处理。只有 Cookie 不存在时才接受 `Authorization: Bearer ...`。

## CSRF、CORS 与代理

Cookie 认证的 `POST`、`PUT`、`PATCH`、`DELETE` 请求必须携带与 `APP_PUBLIC_ORIGIN` 字符串完全相同的 `Origin`；缺失、带尾部 `/` 或其他来源返回 403。Bearer-only 请求不依赖浏览器自动附带凭据，故豁免此 Origin 检查。CORS 允许 GET、POST、PUT、PATCH、DELETE、HEAD、OPTIONS，并允许 credentials。

生产必须在 nginx 等可信入口终止 TLS，HTTP 入口只做 HTTPS 重定向，并传递原始 `Host`、`X-Forwarded-For` 和 `X-Forwarded-Proto`。`deploy/nginx-auth-routes.conf` 应包含在 TLS server 块内；不得通过代理改写或拼接客户端 Origin。应用不接受 `X-Forwarded-Host` 作为 CSRF 判断依据。

## 运维验证

部署后至少验证：

1. HTTPS 登录响应 Cookie 包含 `HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=...; Expires=...`，且不含 `Domain`。
2. 使用 Cookie 对写接口发起无 Origin 或错误 Origin 请求返回 403；精确 Origin 成功。
3. 仅 Bearer 的写请求无需 Origin；同时携带 Cookie 与 Bearer 时始终按 Cookie 处理。
4. 刷新可用空 body，旧 Session 立即失效；登出重复调用均成功并清除 Cookie。
