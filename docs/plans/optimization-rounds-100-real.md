# 100 轮实质优化 · 轮次计数台账（第二批）

- **目标**：全面优化 fakeToys monorepo 的性能与功能，**每轮必须有性能或功能的
  实质性代码进展**（含测试验证），累计完成 100 轮。
- **口径（用户 2026-10-30 重申）**：验证轮、纯文档轮、单纯编译通过**不计入轮次**。
  每轮 = 一个真实的性能改进（异步化 / 缓存 / 查询优化 / 前端体积等）或功能改进
  （新端点 / 新能力 / 修复真实缺陷），并附带测试或基准证明其效果。
- **验证基线**：全仓门禁 `node scripts/check-warnings.mjs`（13 通过 / 0 失败 / 1 跳过）。

| 轮次 | 日期 | 主题 | 关键改动 | 验证 |
|---|---|---|---|---|
| 1 | 2026-10-30 | **Ollama 后端原生异步**（AU-P1 扩展）：httpx.AsyncClient、请求体同步/异步同源、可选依赖回退 | `augmentor/models/ollama.py` 新增 `_build_request`/`_acall_api`/`_async_client_for`/`aclose`；新增 `tests/unit/test_ollama_async_au_p1.py`（3 用例） | Ollama 异步 3 passed；相关回归 55 passed |
| 2 | 2026-10-30 | **Claude 后端原生异步**（AU-P1 扩展）：httpx.AsyncClient + 响应解析同步/异步共用 | `augmentor/models/claude.py` 新增 `_build_request`/`_parse_response`/`_acall_api`/`_async_client_for`/`aclose`；新增 `tests/unit/test_claude_async_au_p1.py`（3 用例） | Claude 异步 3 passed；相关回归 55 passed |
| 3 | 2026-10-30 | **ERNIE 后端原生异步**（AU-P1 扩展）：httpx.AsyncClient + asyncio.Lock 异步 token 刷新 | `augmentor/models/ernie.py` 新增 `_aget_access_token`/`_acall_api`/`_async_client_for`/`aclose` 等；新增 `tests/unit/test_ernie_async_au_p1.py`（3 用例） | ERNIE 异步 3 passed；相关回归 64 passed |
| 4 | 2026-10-30 | **Gemini 后端原生异步**（AU-P1 完成五后端全覆盖） | `augmentor/models/gemini.py` 新增异步方法；新增 `tests/unit/test_gemini_async_au_p1.py`（3 用例） | Gemini 异步 3 passed；相关回归 67 passed |
| 5 | 2026-10-30 | **异步重试取消传播保护**：CancelledError 显式不进重试循环，不被包装成生成错误 | `augmentor/retry.py` 循环开头显式重抛；新增 3 个取消传播测试 | 相关回归 70 passed |
| 6 | 2026-10-30 | **augment_async 入口改走原生异步批处理**（消除线程池占用，与 use_async=True 路径一致） | `augmentor/pipeline.py` 复用 `_run_batch_async`；新增入口测试（并发重叠断言） | 相关回归 119 passed |
| 7 | 2026-10-30 | **laiyipao 玩家「修改密码」功能**：此前账号只有引导创建、无改密入口，口令只能运维改环境变量重启轮换 | 后端新增 `POST /api/v1/me/password`（`ChangePassword`：验旧密 → bcrypt 更新 → 同事务吊销全部 refresh token）+ 限流守卫 + 前端 `miniapp` 接入改密入口（`client.ts changePassword` + me.vue 表单） | 服务层 8 用例、e2e 5 用例全绿；`internal/service` 227s 全绿、`internal/httpapi` 全绿、miniapp 1167 passed、type-check 通过 |
| 8 | 2026-10-30 | **laiyipao 游客账号转正**：游客数据只靠 localStorage 的 guest_token 保护，清缓存/换设备即永久丢失 | 后端新增 `POST /api/v1/me/upgrade`（`UpgradeGuestAccount`：用户名唯一性先查后写、bcrypt 密码、guest_token 保留不变、is_guest 翻转）+ 限流守卫 + 前端游客「设置账号密码」入口（`client.ts upgradeAccount` + me.vue 表单，成功后自动刷新身份） | 服务层 7 用例、e2e 4 用例全绿；`internal/service` 227s 全绿、`internal/httpapi` 全绿、miniapp 1167 passed、type-check 通过 |
| 9 | 2026-10-30 | **laiyipao 玩家修改昵称**：昵称只在登录/引导时设置一次，之后永远改不了（排行榜/防线/战报全是默认名） | 后端新增 `PUT /api/v1/me/nickname`（`UpdateNickname`：TrimSpace、上限 MaxNicknameLen=64、主键更新）+ 限流守卫 + 前端昵称行内编辑（`client.ts updateNickname` + me.vue 改名入口，保存后刷新资料） | 服务层 5 用例、e2e 3 用例全绿；`internal/service` 240s 全绿、`internal/httpapi` 全绿、miniapp 1167 passed、type-check 通过 |
| ... | | 持续至 100 轮 | | |

> 状态：进行中。当前累计 **9 轮实质优化**，距 100 轮还差 91 轮。