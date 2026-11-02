# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 82 轮：MemoryMonitor 快照列表有界。

`monitor_memory_usage` 的 monitor 随被装饰函数长期存活，每次调用追加 2 条
快照——旧实现 `_snapshots` 是普通 list，跑得越久条目越多。改为 deque 上限 200
后仍必须保住「历史峰值」语义（不能退化成最近 200 条的峰值）。
"""

import sys
from pathlib import Path

AI_DIR = Path(__file__).resolve().parents[2]
if str(AI_DIR) not in sys.path:
    sys.path.insert(0, str(AI_DIR))

from augmentor.memory_monitor import MemoryMonitor, MAX_SNAPSHOTS  # noqa: E402


def _driven(values):
    monitor = MemoryMonitor()
    it = iter(values)
    monitor.get_memory_usage_mb = lambda: next(it)
    return monitor


def test_snapshots_are_bounded():
    monitor = MemoryMonitor()
    monitor.get_memory_usage_mb = lambda: 1.0
    for _ in range(MAX_SNAPSHOTS * 3):
        monitor.take_snapshot()
    assert len(monitor._snapshots) == MAX_SNAPSHOTS


def test_peak_survives_eviction():
    # 先冲一个高峰，再用大量低值把它挤出滚动窗口
    values = [500.0] + [1.0] * (MAX_SNAPSHOTS * 2)
    monitor = _driven(values)
    for _ in values:
        monitor.take_snapshot()
    assert all(s["usage_mb"] == 1.0 for s in monitor._snapshots), "高峰应已被挤出窗口"
    assert monitor.get_peak_usage_mb() == 500.0, "历史峰值不能被滚动窗口丢掉"


def test_peak_is_max_of_all_recorded():
    values = [10.0, 300.0, 5.0, 20.0]
    monitor = _driven(values)
    for _ in values:
        monitor.take_snapshot()
    assert monitor.get_peak_usage_mb() == 300.0


def test_trend_uses_recent_window_only():
    values = [1.0, 2.0, 3.0, 4.0, 5.0]
    monitor = _driven(values)
    for _ in values:
        monitor.take_snapshot()
    assert monitor.get_trend() == "increasing"
    assert len(monitor._snapshots) == 5


def test_reset_clears_peak_and_snapshots():
    monitor = _driven([100.0])
    monitor.take_snapshot()
    assert monitor.get_peak_usage_mb() == 100.0
    monitor.reset()
    assert len(monitor._snapshots) == 0
    assert monitor.get_peak_usage_mb() == 0.0
    assert monitor.get_trend() == "stable"
