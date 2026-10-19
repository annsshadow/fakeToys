---
title: "可选依赖导致的 latent 红：装了 faiss 才暴露的 4 条用例"
date: 2026-10-19
last_refreshed: 2026-10-19
category: testing
module: augmentor/augmentor/vector/faiss.py + augmentor/tests
problem_type: environment_dependent_test_failure
component: python-tests
severity: medium
symptoms:
  - "CI（不装 faiss）全绿；同一份代码在装了 faiss 的机器上 4 条用例红：两条 search(top_k=0)、一条 load 维度不匹配、一条断言 faiss 缺席"
  - "两条后端（faiss 加速 / numpy 精确）对同一输入行为不同：一个被底层 assert 打断、一个正常返回空"
root_cause: delegated_edge_case_semantics_to_optional_lib
resolution_type: code_fix
tags: [faiss, optional-dependency, latent-red, backend-parity, sys-modules-none, test-determinism]
applies_when:
  - "产品里有『可选加速库 + 纯实现回退』两条路径"
  - "要写『没装某库时必须报错』的用例"
  - "同一份代码在不同开发机的 Python 环境里给出不同测试结果"
---

# 可选依赖导致的 latent 红（faiss）

## 现象

`augmentor` 的向量库有两条检索路径：装了 `faiss-cpu` 走 `FAISSDB._index`，否则走 numpy 精确检索。
CI 只装 `requirements-dev.txt`（不含 faiss）⇒ 长期只看得到 numpy 路径。在装了 faiss 的环境里，同一份 main 冒出 4 条红：

| 用例 | 暴露的缺陷 |
|---|---|
| `test_zero_top_k_returns_no_rows`、`test_top_k_zero_is_the_echo_only_mode_the_ui_allows` | `search(top_k=0)` 在 numpy 路径是合法的空结果（UI 的 echo-only 模式），faiss 的 `IndexFlatIP.search` 对 `k<=0` 直接 assert |
| `test_load_with_wrong_dimension` | `load()` 允许加载维度不匹配的数据（判据：返回 True），但随后 `_index.add(vectors)` 被 faiss 的维度断言打断 |
| `test_faiss_not_installed_raises` | 用例隐含假设"运行环境没装 faiss"，装了就跑不到 ImportError 分支 |

## 修法

**产品侧：边界语义留在产品代码，别交给底层库的 assert。**

- `search`：`k = min(top_k, len(ids))` 之后 `k <= 0` 直接 `return []`（两条后端行为对齐）；
- `load` / `delete`：重建索引入口统一判 `vectors.shape[1] == self.dimension`，不匹配则跳过入索引并 `logger.warning`
  （数据仍留在 `_vectors`；`load()` 的判据只是"能加载"，照旧返回 True）；
- 一句话口径：**"后端可换，行为不可换"**——回退路径能答的输入，加速路径也必须能答。

**测试侧：把"环境事实"变成"可控输入"。**

`test_faiss_not_installed_raises` 改 `monkeypatch.setitem(sys.modules, "faiss", None)`：
CPython 的 import 机制遇到 `sys.modules[name] is None` 必然抛 `ImportError` ⇒ 两种环境跑的是同一条分支，
判据不依赖开发机装了什么（也没有放宽断言，仍然是 `pytest.raises(ImportError, match="FAISS")`）。

同形状的既有先例：`tests/unit/test_chunked_matrix.py` 的 `_db_without_faiss()` 用 `db._index = None`
强制 numpy 分块路径，同样是为了"不依赖本机是否装了 faiss"。

## 验证

`python:3.11` 容器 + `requirements-dev.txt` + `faiss-cpu` 全量跑，与卸载 faiss 后的 CI 口径复跑对照（结果见
[main 既有红全量修复](../development-workflow/main-existing-reds-fix-2026-10-19.md) §6 验证矩阵）。

## 可复用教训

1. **可选依赖 = 两套行为**：凡"没装 X 时的回退路径"能处理的输入，装了 X 的路径必须给出同样结果（或显式报同样的错）。
2. **"没装 X"的用例不要靠环境事实**（`importorskip` 的反向同理）：用 `sys.modules["X"] = None` 哨兵把"缺失"变成可控输入。
3. **CI 不装的依赖是 latent 红的温床**：别把这类红当"本机环境问题"压下去——它们往往指向真实边界缺陷。

## 相关

- [main 既有红全量修复（2026-10-19）](../development-workflow/main-existing-reds-fix-2026-10-19.md)（§5 遗留 1，本轮收口）
- **实现**：`augmentor/augmentor/vector/faiss.py`（`dedup.py` 里另有 `index.search(query, k)` 一处，但
  `k = min(n, 100)` 且 `n=0` 时不进循环，无同形缺陷）
- **测试**：`augmentor/tests/unit/test_vector.py`、`test_count_knob_validation.py`、`test_impact_semantic_wiring_l227.py`、`test_dedup.py`
