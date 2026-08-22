# o2server 容器化运行指南（plan002 U9a）

## 镜像与服务

- 镜像：`o2oa/o2server:latest`（O2OA v9.5.2，OpenJDK 11.0.23，~2.6GB）
- compose 服务名 `o2server`（见 `oa4rust/docker-compose.yml`），端口 **18080→80**
- 数据卷：config / custom / dynamic / local / logs / webroot / 内置 MySQL 全部持久化

## 无头初始化（已验证）

v9 首次启动进入 init 向导模式，全程 REST API 可完成：

```powershell
# 1. 查询初始化状态
Invoke-WebRequest http://localhost:18080/jaxrs/secret/check -UseBasicParsing
# 2. 设置 xadmin 密码
Invoke-WebRequest http://localhost:18080/jaxrs/secret/set `
  -Method Post -ContentType "application/json" `
  -Body '{"secret":"o2oa@2022"}' -UseBasicParsing
# 3. 触发部署建表（注意是 GET）
Invoke-WebRequest http://localhost:18080/jaxrs/server/execute -UseBasicParsing
```

- 冷启动/首次建表约 **10 分钟**；重启约 7-8 分钟（JVM 固定 3G 堆）
- 数据卷持久化后重启直接进正常模式，凭据不变

## 行为对比测试接入

确定性凭据：**xadmin / o2oa@2022**

```powershell
$env:JAVA_SERVICE_URL = "http://localhost:18080"
$env:BEHAVIOR_TEST_CREDENTIAL = "xadmin"
$env:BEHAVIOR_TEST_PASSWORD = "o2oa@2022"
$env:BEHAVIOR_COMPARE = "1"
cargo test --test behavior_compare
```

## 已知问题

1. **comparator 登录路径错位**：`tests/behavior_comparison/comparator.rs` 使用 `{base}/jaxrs/authentication/login`，而 O2OA v9 真实路径为 `/x_organization_assemble_authentication/jaxrs/authentication`——裸 `/jaxrs/*` 在 O2OA 上会挂起。需改 comparator 或加路径重写反代
2. 新库中不存在普通测试账户 testadmin；需要普通账户语义时用管理 API 创建
3. CI 等待逻辑需留足冷启动超时（≥10 分钟）
