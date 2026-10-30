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
| 3 | | | | |
| ... | | 持续至 100 轮 | | |

> 状态：进行中。当前累计 **2 轮实质优化**，距 100 轮还差 98 轮。