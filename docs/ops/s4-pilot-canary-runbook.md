# S4 金丝雀（项 4）试点部署 + 数据采集 Runbook

> **定位**：本 runbook 是给「具备部署权限方」的**操作交接件**，用于最终签核计划 §七 项 4（S4 金丝雀真实流量）。
> **明确非**：它本身**不是**项 4 的达成证据，也不是代理信号。项 4 只有在**真实试点**的一个观察窗口内 5xx/错误预算达标、且经 `oa4rust/scripts/pilot_gate.py` 评估通过后才算签核。在此之前 §七 项 1/2/3/5 视为 gate 达成、项 4 保留为待部署外部 gate，**不宣布完全替代**。

## 0. 目标门禁
`§七 项 4`：pilot 真实使用一个观察窗口内 5xx/错误预算达标。
评估器：`oa4rust/scripts/pilot_gate.py`（离线读取 Nginx 访问日志，逐窗口判 5xx 预算）。
CI 载体：`.github/workflows/oa4rust-pilot-gate.yml`（下载 `access_log_url` → 调 `pilot_gate.py` → 上传 `report.json` + `manual-checklist.md`）。

## 1. 前置条件（外部，本仓库不提供）
1. 在**真实/staging** 部署 oa4rust 新栈（Rust `oa4rust` 服务 + 前端 dist + 依赖 PG/Redis），供小范围真实用户使用。
2. 新栈前挂一个**专属 Nginx**（只代理试点流量，日志独立于其它服务）。
3. 有一批**真实试点用户**在一个**观察窗口**内产生真实流量（勿用本地合成流量冒充）。

## 2. 专属 Nginx 访问日志格式（`pilot_gate.py` 解析契约）
`pilot_gate.py` 用如下正则逐行解析（`scripts/pilot_gate.py:18`）：
```
^\S+\s+\S+\s+\S+\s+\[(?P<timestamp>[^]]+)]\s+"[^"]*"\s+(?P<status>[1-5][0-9]{2})\s+(?:[0-9]+|-)(?:\s+.*)?$
```
即行形如：`<ip> <user> <x> [DD/Mon/YYYY:HH:MM:SS ±ZZZZ] "请求行" 状态码 字节数`（时间戳为 Nginx `$time_local` 格式）。
配套的 `log_format`（`$http_x_forwarded_for` 作为第 3 token；无则填 `-`）：
```nginx
log_format oa4rust_pilot '$remote_addr $remote_user $http_x_forwarded_for '
                         '[$time_local] "$request" $status $body_bytes_sent';
access_log /var/log/nginx/oa4rust-pilot.access.log oa4rust_pilot;
```
示例行：`203.0.113.7 - 10.0.0.9 [14/Sep/2026:09:30:12 +0000] "GET /jaxrs/calendar/setting/list/all HTTP/1.1" 200 512`
- 状态 500–599 计为 5xx；`5xx 预算 = floor(窗口请求数 × max_5xx_rate_percent / 100)`；`5xx 数 ≤ 预算` 且满足 `min_requests` 等条件才通过。
- 畸形行会被记入 `malformed_line_numbers`，请确保日志只含本服务行、无其它 Nginx 默认行混入。

## 3. 发布 access_log_url
- 把观察窗口的试点访问日志（纯文本）发布到一个 **HTTPS** URL（对象存储/内部文件服务皆可），作为 `access_log_url`。
- 如需鉴权：设置 workflow 的 `PILOT_ACCESS_LOG_TOKEN`（Bearer），URL 需接受该 token。
- 文件须非空（workflow `test -s` 校验），且时间戳落在 `[window_start, window_end)` 内的行才会计入。

## 4. `.github/workflows/oa4rust-pilot-gate.yml` 输入（逐项）
| 输入 | 含义 | 建议取值 |
|------|------|----------|
| `access_log_url` | 专属 Nginx 试点访问日志的 HTTPS 地址 | 第 3 步发布的 URL |
| `service_url` | 部署后的新栈绝对 http(s) URL | 试点入口 |
| `window_start` / `window_end` | 观察窗口（ISO-8601，含起不含止） | 试点实际观察期 |
| `max_5xx_rate_percent` | 5xx 错误预算（0–100） | 按 SLO 定（如 1） |
| `min_requests` | 窗口内最少真实请求数（样本量下限） | 保证统计意义的阈值 |
| `pilot_users` | 真实试点用户标识（逗号分隔） | 实际试点人员 |
| `version` | 被观测的新栈版本 | 本次部署版本 |

## 5. 触发 + 读取结果
1. 以 第 4 节 参数手动触发 `oa4rust-pilot-gate`（或由其 CI 编排触发）。
2. 产物：`oa4rust/target/pilot-gate/report.json`（机读评估）+ `manual-checklist.md`（人工核对清单），随 artifact `oa4rust-pilot-gate-<run_id>` 上传。
3. **签核口径**：`pilot_gate.py` 通过（5xx 数 ≤ 预算、请求数 ≥ min_requests、窗口合法、无未解释畸形行）→ 项 4 签核 → 结合已达成项 1/2/3/5，方可宣布完全替代。

## 6. 与 §七 的关系
- 项 1（behavior_compare ≤170）/项 2（S3 E2E 5/5）/项 3（W13 无伪装实装）/项 5（§五 边界声明）：已 gate 达成并留证（提交 `69dc35d6`/`81eba12c`/`6fde0ec6`）。
- 项 4（本 runbook 对应）：需按 §1–§5 完成真实试点后签核。**在真实流量达标前不宣布完全替代。**
