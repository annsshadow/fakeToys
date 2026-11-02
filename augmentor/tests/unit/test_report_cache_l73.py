# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 73 轮：增强报告缓存必须有界（LRU）

`_REPORTS` 此前无上限：每跑一个增强任务就多留一份报告（含逐条错误明细），
长期运行的进程内存随任务数线性增长。改为 OrderedDict + 容量上限后，
超限逐出最久未访问的一份；读取同样刷新访问顺序，正在被轮询的任务不会被挤掉。
"""

import pytest

from api.routes import augment


@pytest.fixture(autouse=True)
def clear_reports():
    """每个用例从空缓存开始（模块级全局，用例之间不能互相污染）"""
    with augment._REPORTS_LOCK:
        augment._REPORTS.clear()
    yield
    with augment._REPORTS_LOCK:
        augment._REPORTS.clear()


def report(task_id: str) -> dict:
    return {"task_id": task_id, "errors": []}


class TestReportCacheBound:
    def test_capacity_is_bounded_and_oldest_evicted(self):
        cap = augment._MAX_REPORTS
        for i in range(cap + 5):
            augment._remember_report(report(f"task-{i}"))

        with augment._REPORTS_LOCK:
            assert len(augment._REPORTS) == cap
            # 最旧的 5 份被逐出，最新的全在
            assert "task-0" not in augment._REPORTS
            assert "task-4" not in augment._REPORTS
            assert f"task-{cap + 4}" in augment._REPORTS

    def test_read_refreshes_lru_order(self):
        cap = augment._MAX_REPORTS
        for i in range(cap):
            augment._remember_report(report(f"task-{i}"))

        # 访问最旧的一份 → 它变成「最近使用」，下一次逐出应轮到 task-1
        assert augment._get_report("task-0") is not None
        augment._remember_report(report("task-new"))

        with augment._REPORTS_LOCK:
            assert len(augment._REPORTS) == cap
            assert "task-0" in augment._REPORTS, "被读取过的报告不得被逐出"
            assert "task-1" not in augment._REPORTS, "应逐出最久未访问的 task-1"

    def test_reinserting_same_task_does_not_grow_cache(self):
        augment._remember_report(report("task-1"))
        augment._remember_report({"task_id": "task-1", "errors": ["updated"]})

        with augment._REPORTS_LOCK:
            assert len(augment._REPORTS) == 1
            assert augment._REPORTS["task-1"]["errors"] == ["updated"]

    def test_get_unknown_task_returns_none(self):
        assert augment._get_report("no-such-task") is None
