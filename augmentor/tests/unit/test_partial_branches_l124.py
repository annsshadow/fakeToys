# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L124（A205）：偏支逐支审计——cli 家族 7 条 + vector/faiss 3 条

**可达、补测（本文件）**
- cli/commands/data_ops.py 66->73      `validate` 不带 `--output` ⇒ `if args.output:` 假支
- cli/commands/ops.py 118->exit       `dependency --action graph` 不带 `--output` ⇒
                             `if args.output:` 假支（graph 出完图就结束，不进 validate）
- cli/commands/profiling.py 35->37    `outliers --field score`（非 length）⇒
                             `if args.field == "length":` 假支（不挂长度字段）
- vector/faiss.py 192->199            查询缓存命中后 `_ids` 被**绕过写接口**直接改动 ⇒
                             失效条目被滤掉、整条作废重算（:189 注释明说的兜底）
- vector/faiss.py 268->271           faiss 可用时删空全部向量 ⇒ `if len(self._vectors):`
                             假支（不往空索引里 add）
- vector/faiss.py 334->337           faiss 可用时 load 一份**零向量**持久化 ⇒
                             `if len(vectors):` 假支

**判定为结构性不可达、记档不测**
- cli/commands/version.py 98->exit   `run_version` 的 action 由 parser 限定
                             choices（list/create/load/compare/rollback/delete），
                             全被 if/elif 链覆盖 ⇒ 链尾 `elif action == "delete":`
                             假支（穿到函数尾）不可达（封闭词表，同 export 277->286 一族）。
- cli/commands/version.py 142->exit  `run_backup` 的 action 同样封闭
                             （create/restore/list/delete 全在 parser choices 里）⇒
                             链尾假支不可达。
- cli/commands/ops.py 125->exit     `dependency` 的 action choices
                             （register/list/graph/validate）全被覆盖 ⇒ 链尾假支不可达。
- cli/commands/quality.py 86->91    `generate_quality_report` 的 `_generate_recommendations`
                             在**零改进建议时兜底追加**「数据集质量良好」一条
                             （quality_report.py :396-397）⇒ `report.recommendations`
                             恒非空，`if report.recommendations:` 假支不可达。
"""

import io
import json
import sys
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path

import numpy as np
import pytest

from augmentor.vector.faiss import FAISSDB


def _run(argv, cwd=None):
    """走真 argparse 流程跑一条 CLI（L71 的 `_run` 同式）"""
    from cli import main

    old_argv, old_cwd = sys.argv, None
    if cwd:
        import os

        old_cwd = os.getcwd()
        os.chdir(cwd)
    sys.argv = ["cli"] + argv
    out, err = io.StringIO(), io.StringIO()
    code = None
    try:
        with redirect_stdout(out), redirect_stderr(err):
            try:
                main()
            except SystemExit as e:
                code = e.code
    finally:
        sys.argv = old_argv
        if old_cwd is not None:
            import os

            os.chdir(old_cwd)
    return out.getvalue(), code


# ---------------------------------------------------------------------------
# cli 家族
# ---------------------------------------------------------------------------

class TestCliValidateWithoutOutput:
    """data_ops.py 66->73：validate 不带 --output 不落盘、只出判决"""

    def test_validate_without_output_flag(self, tmp_path):
        data = tmp_path / "d.jsonl"
        data.write_text(
            json.dumps([{"instruction": "问", "input": "", "output": "答"}], ensure_ascii=False),
            encoding="utf-8",
        )
        out, code = _run(["validate", "--input", str(data)], cwd=tmp_path)
        assert code in (0, None)
        assert "验证结果已保存" not in out
        assert "验证结果" in out


class TestCliDependencyGraphWithoutOutput:
    """ops.py 118->exit：graph 不带 --output 只打印节点/边数，不进 validate 分支"""

    def test_graph_without_output_flag(self, tmp_path):
        out, code = _run(["dependency", "--action", "graph"], cwd=tmp_path)
        assert "依赖图" in out
        assert "已保存" not in out
        assert code in (0, None)


class TestCliOutliersNonLengthField:
    """profiling.py 35->37：--field score（非 length）不挂长度字段、直接按该列检测"""

    def test_outliers_with_score_field(self, tmp_path):
        rows = [{"instruction": "q%d" % i, "output": "a", "score": i} for i in range(20)]
        data = tmp_path / "d.json"
        # CLI 的 `_load_items` 是整文件 `json.load` ⇒ 写 JSON 数组而不是 NDJSON
        data.write_text(json.dumps(rows, ensure_ascii=False), encoding="utf-8")
        out, code = _run(["outliers", "--input", str(data), "--field", "score"], cwd=tmp_path)
        assert code in (0, None)
        # 报表 `field` 回显成 score ⇒ 确实走了「非 length、不挂长度字段」那一支
        assert '"field": "score"' in out
        assert '"outlier_count": 0' in out


# ---------------------------------------------------------------------------
# vector/faiss.py（faiss 缺失环境用测试树既有的伪 faiss 注入）
# ---------------------------------------------------------------------------

class TestFaissStaleQueryCache:
    """faiss.py 192->199：缓存命中但 `_ids` 被绕过写接口改动 ⇒ 滤掉失效条目、作废重算"""

    def test_stale_cache_entry_falls_back_to_recompute(self, monkeypatch):
        from tests.unit.test_faiss_and_benchmark import _install_fake_faiss

        _install_fake_faiss(monkeypatch)
        db = FAISSDB(dimension=4)
        # 四个不同方向（`add_vectors` 先归一化再存）：与查询 [1,0,0,0] 的内积
        # 依次 1.0 / 0.707 / 0.577 / 0.5，互不相等 ⇒ top-k 排序无平局
        db.add_vectors(
            [np.array([4.0, 0, 0, 0]), np.array([1.0, 1.0, 0, 0]),
             np.array([1.0, 1.0, 1.0, 0]), np.array([1.0, 1.0, 1.0, 1.0])],
            [{}, {}, {}, {}],
            ["w", "x", "y", "z"],
        )
        # top_k 必须**小于删除后的条数**：缓存键是 `(digest, min(top_k, len(_ids)))`，
        # 删一条后 len 4→3、top_k=5 时 k 从 4 变 3，键直接 miss，永远到不了
        # 「命中但内容已失效」那条兜底支。top_k=2：两次 k 都等于 2 ⇒ 同键命中
        first = db.search(np.array([1.0, 0, 0, 0]), 2)
        assert {r["id"] for r in first} == {"w", "x"}
        # 绕过 delete() 直接改 `_ids`：缓存里还站着 2 条旧结果、有效只剩 1 条
        db._ids.remove("w")
        db._ids.remove("x")
        second = db.search(np.array([1.0, 0, 0, 0]), 2)
        assert "w" not in {r["id"] for r in second}
        assert "x" not in {r["id"] for r in second}


class TestFaissDeleteAllVectors:
    """faiss.py 268->271：删空 ⇒ 不往（faiss）空索引里 add"""

    def test_delete_all_skips_index_add(self, monkeypatch):
        from tests.unit.test_faiss_and_benchmark import _install_fake_faiss

        _install_fake_faiss(monkeypatch)
        db = FAISSDB(dimension=4)
        db.add_vectors(
            [np.array([1.0, 0, 0, 0]), np.array([0, 1.0, 0, 0])],
            [{}, {}],
            ["x", "y"],
        )
        assert db.delete(["x", "y"]) == 2
        assert len(db._ids) == 0
        assert db.search(np.array([1.0, 0, 0, 0]), 3) == []


class TestFaissLoadEmptyVectors:
    """faiss.py 334->337：load 零向量持久化 ⇒ 跳过 `index.add`"""

    def test_load_zero_vector_store_skips_index_add(self, monkeypatch, tmp_path):
        from tests.unit.test_faiss_and_benchmark import _install_fake_faiss

        _install_fake_faiss(monkeypatch)
        storage = str(tmp_path / "db")
        db1 = FAISSDB(dimension=4, storage_dir=storage)
        db1.add_vectors([np.array([1.0, 0, 0, 0])], [{}], ["x"])
        db1.delete(["x"])
        db1.persist()

        db2 = FAISSDB(dimension=4, storage_dir=storage)
        assert db2.load() is True
        assert db2._ids == []
        assert db2.search(np.array([1.0, 0, 0, 0]), 3) == []
