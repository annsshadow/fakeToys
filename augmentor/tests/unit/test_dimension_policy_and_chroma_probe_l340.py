# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L340 行为面：A268 维度冲突「拒配置」策略 / A269 chromadb 只读状态探查。

零 parametrize（l97 / l98 把全仓 parametrize 三面钉成现量数）。
"""

from pathlib import Path

import numpy as np
import pytest

from augmentor.exceptions import VectorError
from augmentor.semantic import (
    _chroma_status,
    assert_model_dimension,
    embed_texts,
    index_status,
    persist_index,
)

ROOT = Path(__file__).resolve().parents[2]
SEMANTIC_PY = (ROOT / "augmentor" / "semantic.py").read_text(encoding="utf-8")


# ---------------------------------------------------------------------------
# A268：模型输出宽度与 config.vector.dimension 冲突即拒
# ---------------------------------------------------------------------------


def test_conflicting_widths_are_rejected_with_the_policy_named():
    with pytest.raises(VectorError) as exc:
        assert_model_dimension(384, 768)
    assert "拒配置" in str(exc.value)
    assert "384" in str(exc.value) and "768" in str(exc.value)


def test_matching_widths_pass():
    assert assert_model_dimension(384, 384) is None


def test_hash_embedding_really_calls_the_policy():
    """判据不是给死路径预留的装饰：哈希臂今天就用写出的矩阵宽度调它"""
    assert "assert_model_dimension(rows.shape[1], dimension)" in SEMANTIC_PY


def test_hash_embedding_is_unchanged_by_the_policy():
    """零漂移反证：宽度自检只判不改写，形状与确定性都和改前一致"""
    a = embed_texts(["如何申请？", "怎么办理"], 16)
    b = embed_texts(["如何申请？", "怎么办理"], 16)
    assert len(a) == 2 and all(v.shape == (16,) for v in a)
    assert np.array_equal(a[0], b[0]) and np.array_equal(a[1], b[1])


# ---------------------------------------------------------------------------
# A269：chromadb 只读状态探查（GET 不落盘）
# ---------------------------------------------------------------------------


def test_probe_of_a_missing_dir_reports_absent_and_creates_nothing(tmp_path):
    target = tmp_path / "store"
    out = index_status(backend="chromadb", dimension=16,
                       collection="default", storage_dir=str(target))
    assert (out["exists"], out["count"]) == (False, 0)
    assert out["backend"] == "chromadb" and out["dimension"] == 16
    assert not target.exists()  # 构造从未发生 ⇒ 一条 GET 没写任何东西


def test_probe_recognises_an_existing_persistence_marker(tmp_path):
    target = tmp_path / "store"
    target.mkdir()
    (target / "chroma.sqlite3").write_bytes(b"sqlite")
    out = _chroma_status("chromadb", 16, "default", str(target))
    assert out["exists"] is True
    assert out["count"] == 0  # 条数不可得的能力边界，不是「索引为空」


def test_probe_never_imports_or_constructs_a_client(tmp_path):
    """本机没装 chromadb；若实现去构造 PersistentClient，这里会直接 ImportError"""
    import sys

    assert sys.modules.get("chromadb") is None
    assert index_status(backend="chromadb", dimension=8,
                        collection="default", storage_dir=str(tmp_path / "x"))["backend"] == "chromadb"


def test_persist_still_refuses_chromadb(tmp_path):
    """状态面放开不等于落盘闸放松：persist/load 面它确实没有"""
    target = tmp_path / "store"
    with pytest.raises(VectorError, match="chromadb"):
        persist_index([{"instruction": "问题"}], backend="chromadb",
                      dimension=8, collection="default", storage_dir=str(target))
    assert not target.exists()
