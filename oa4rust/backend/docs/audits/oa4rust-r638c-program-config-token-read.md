# R638c config_token 系统令牌配置裸读甄别报告

- 对象：`program_center` 服务 `config_token` 接口
- 性质：验证型甄别，零源码改动、零测试入库；本报告为唯一产出

## 一、结论

判级 **P2**：任意已登录用户可通过 `config_token` 读取系统令牌配置整行（含 `value`），而写同项配置需管理员权限，构成认证后的垂直越权读敏感配置。不判 P1（生产匿名被全局鉴权中间件拦截，且仓库内令牌值为占位夹具，非真凭据泄露）；不判 P3（敏感配置读取危害高于前端值域错配）。

## 二、锚点与根因

- 审计对象：`crates/program_center/src/lib.rs`（13665 行，只读未改）。
- `config_token`（约 3842 行）：签名仅 `pool: Extension<Pool>`，无会话/角色参数；SQL `SELECT id,key,value,creator,create_time FROM x_program_config WHERE key = system.token`，空绑定；回传整行含 `value`。
- 写侧对照：`config_save`（约 3752 行）、`u2_config_domain_put`（约 9777 行）首行均 `require_admin`；`u2_config_token_put`（约 9846 行）经 `u2_config_domain_put(system.token)` 写入，需管理员。读无门、写有门，不对称。
- 路由：`crates/program_center/src/routes.rs` 约 422 行 GET 直挂 `config_token`、423 行 PUT 挂 `u2_config_token_put`。
- 根因：读接口未施加与写接口一致的 admin 归属门，任意会话可直取敏感配置值。

## 三、真执行证据

- 方法：oneshot 真测试（独占临时 PostgreSQL，测试值用明确非真的占位夹具字符串），通过 `router` 注入会话层调用。
- 结果：`running 1 test / 1 passed; 0 failed / EXIT 0`。四臂全部成立：
  1. 普通登录用户读取返回 200 且可读 `value`；
  2. 无会话层直调 handler 仍返回 200 且可读 `value`（handler 自身不区分身份）；
  3. 源码守卫确认 `config_token` 为裸 `pool` 签名、无 `Session`/`require_admin`/`is_admin`，且命中 `system.token`；
  4. 写侧 `u2_config_token_put` 经 admin 门，读/写门控不对称。

## 四、三分判级

- P1 排除：生产路由经全局 `auth_middleware`，无认证头返回 401，外部匿名在中间件层即被拦；仓库内 `system.token` 真值由部署环境注入，未见明文，测试值为占位夹具，不夸大真凭据泄露。
- P3 排除：认证后垂直越权读敏感配置，危害高于前端值域错配（后者默认视图可绕过、无越权）。
- 取 P2：认证后越权读取系统令牌配置整行，需登录但非管理员即可读，危害介于两者之间。

## 五、行为现状与披露

- 现状：任何持有有效会话的用户 GET `config_token` 均可拿到 `system.token` 的 `value`；写该项仍需 admin。
- 据实披露：oneshot 直调 `router` 不经全局 `auth_middleware`，故"匿名臂 200"反映的是 handler 自身无身份校验，而非生产环境匿名可用；生产匿名被中间件 401 拦截。
- 复现前提：测试库 `<TEST_HOST>`（本地回环占位端口）注入占位令牌值；真值由部署环境注入，不入仓。
- 未实施修复，仅输出甄别结论；修复可参考写侧 admin 门保持一致。

## 六、SHA 分列

- 审计对象 `crates/program_center/src/lib.rs` SHA-256：`21985a04b5147b487cf1edabce965616f838967ec92b84d7e82ecf6ea8b6c94b`（只读未改）。
- runnerSha（临时测试脚本 SHA-256）：`3149777c92d148c3ee9e99085d2454affd482d45839c8c593973a96d0b3908b6`。
- docSha（本报告全文 SHA-256）：以提交时文件内容为准，见提交记录。
