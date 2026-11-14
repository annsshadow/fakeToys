# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L342 / A346：点号主张（路径冒号后接符号名）有了机械通道。

改前 a184 的 REF 只认 `:行号`，这类主张被当成普通文件引用 ⇒ 符号换名、拼错
两把尺都不响。现并入 symbol_live / symbol_dead：名字是全仓真符号才算好引用，
示例形状逐格钉住数量。零 parametrize。
"""

import pytest

from tests.unit.test_comment_refs_a184 import census, probe, tick

#: 现量按真身普查钉死；往账本里新写一条真点号主张就要按现量重钉（题 7 接受的成本）
MEASURED_DOT_CLAIM_LIVE = 45
MEASURED_DOT_EXAMPLE = 30


@pytest.fixture(scope="module")
def face():
    return census()[0]


def test_the_dot_claim_face_is_the_measured_one(face):
    assert face["dot_claim_live"] == MEASURED_DOT_CLAIM_LIVE, face["dot_claim_live"]
    assert face["dot_example"] == MEASURED_DOT_EXAMPLE, face["dot_example"]


def test_no_dot_claim_points_at_an_absent_symbol(face):
    """硬 0 档：点号主张点名了全仓不存在的符号就当场红"""
    assert face["dead_symbol"] == 0
    assert face["symbol_dead"] == 0


def test_dot_claims_are_counted_separately_from_paren_claims(face):
    """反空转：内容维的括弧主张一格未动，新增的是另一条通道"""
    assert face["anchor_claims"] == 36


def test_a_fabricated_dot_claim_lands_in_the_dead_bucket():
    """注入臂：编一个不存在的符号名 ⇒ 硬 0 档立刻 +1（判据真的会红）"""
    items = (("tests/unit/_a184_dot_probe.py", (
        (1, tick("augmentor/config.py", ":", "no_such_symbol_l342")),
        (2, tick("file.py", ":", "NNN")),
    )),)
    counts, _rows, _sym, real, _rr, _rs = probe(items)
    assert counts["dead_symbol"] - real["dead_symbol"] == 1, counts["dead_symbol"]
    assert counts["symbol_dead"] - real["symbol_dead"] == 1
    assert counts["dot_example"] - real["dot_example"] == 1


def test_example_shapes_never_become_symbol_claims():
    """反面：形状示例只进 dot_example，不许混进好引用去把比例洗白"""
    items = (("tests/unit/_a184_dot_example_probe.py", (
        (1, tick("file.py", ":", "NNN")),
    )),)
    counts, _rows, _sym, real, _rr, _rs = probe(items)
    assert counts["dot_example"] - real["dot_example"] == 1
    assert counts["symbol_live"] == real["symbol_live"]
    assert counts["dead_symbol"] == real["dead_symbol"]
