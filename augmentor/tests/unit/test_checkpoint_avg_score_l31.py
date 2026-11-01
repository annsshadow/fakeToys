# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 31 轮：平均质量评分增量维护。

改前 `_calculate_avg_quality_score` 每次轮询都 `sum(quality_scores.values())`
—— 10 万条任务、每 2 秒轮询一次就是每 2 秒 10 万次加法。
改后缓存挂在 CheckpointData 上（非 dataclass 字段，不影响 asdict 序列化面），
写入时增量维护、读取 O(1)；加载的旧断点惰性重建一次。
"""

import pytest

from augmentor.checkpoint import CheckpointData, CheckpointManager


def _ck(task_id="t"):
    return CheckpointData(
        task_id=task_id,
        total_items=10,
        processed_items=0,
        failed_items=0,
        completed_indices=[],
        failed_indices=[],
        start_time=0.0,
        last_update_time=0.0,
        quality_scores={},
    )


def test_avg_score_incremental_and_correct():
    m = CheckpointManager()
    ck = _ck()
    for i, score in enumerate([0.5, 0.7, 0.9], start=1):
        m.update_progress(i - 1, True, quality_score=score, checkpoint=ck)
        assert m._calculate_avg_quality_score(ck) == pytest.approx(sum([0.5, 0.7, 0.9][:i]) / i)

    # 缓存命中：不重新 sum（用 _score_agg 存在 + 值与 quality_scores 一致证明）
    assert getattr(ck, "_score_agg", None) == (2.1, 3)
    assert m._calculate_avg_quality_score(ck) == pytest.approx(0.7)


def test_avg_score_rebuilds_on_loaded_checkpoint():
    m = CheckpointManager()
    ck = _ck()
    ck.quality_scores = {0: 1.0, 1: 3.0}  # 模拟加载的旧断点（无缓存）
    assert m._calculate_avg_quality_score(ck) == pytest.approx(2.0)
    # 重建后缓存已挂上
    assert getattr(ck, "_score_agg", None) == (4.0, 2)
    # 后续写入走增量
    m.update_progress(2, True, quality_score=5.0, checkpoint=ck)
    assert m._calculate_avg_quality_score(ck) == pytest.approx(3.0)


def test_avg_score_overwrite_same_index():
    m = CheckpointManager()
    ck = _ck()
    m.update_progress(0, True, quality_score=1.0, checkpoint=ck)
    # 同一 index 被覆盖（幂等重试等）：均值应反映新值，不重复计数。
    m.update_progress(0, True, quality_score=4.0, checkpoint=ck)
    assert m._calculate_avg_quality_score(ck) == pytest.approx(4.0)
    assert getattr(ck, "_score_agg", None) == (4.0, 1)
