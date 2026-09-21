"""微型分支收尾 L60：benchmark/cache/checkpoint/cleaner/active_learning 残留分支

- benchmark: 无基准对比行（baseline=None → no_baseline）
- cache: cached 装饰器 cache_dir 分支（DiskCache 落盘）
- checkpoint: 未建 checkpoint 时增量保存为空操作
- cleaner: 自定义规则在 clean 流程中被应用
- active_learning: 单轮选不出样本时提前终止循环
"""

from pathlib import Path

import pytest

from augmentor.active_learning import ActiveLearningLoop
from augmentor.benchmark import QualityBenchmark
from augmentor.cache import cached
from augmentor.checkpoint import CheckpointManager
from augmentor.cleaner import DatasetCleaner


class TestBenchmarkNoBaselineRow:
    def test_report_renders_no_baseline_row(self):
        bench = QualityBenchmark(baseline_file=None)
        report = bench.generate_report({
            "sample_count": 10,
            "metrics": {"avg_quality": 0.8},
            "comparisons": {
                "avg_quality": {"current": 0.8, "baseline": None},
                "dup_rate": {
                    "current": 0.1, "baseline": 0.2,
                    "delta": -0.1, "status": "improved",
                },
            },
        })
        assert "no_baseline" in report
        assert "improved" in report


class TestCachedDiskBranch:
    def test_cached_with_cache_dir_persists(self, tmp_path):
        calls = {"n": 0}

        @cached(ttl=60, cache_dir=str(tmp_path / "cache"))
        def double(x):
            calls["n"] += 1
            return x * 2

        assert double(21) == 42
        assert double(21) == 42
        assert calls["n"] == 1  # 第二次命中磁盘缓存
        assert any((tmp_path / "cache").iterdir())

    def test_cached_memory_default(self):
        calls = {"n": 0}

        @cached(ttl=60)
        def add(a, b):
            calls["n"] += 1
            return a + b

        assert add(1, 2) == 3
        assert add(1, 2) == 3
        assert calls["n"] == 1


class TestCheckpointDeltaGuard:
    def test_save_delta_without_checkpoint_is_noop(self, tmp_path):
        manager = CheckpointManager(checkpoint_dir=str(tmp_path / "ck"))
        assert manager._current_checkpoint is None
        manager._pending_completed.add(1)
        manager._save_delta()
        # 没有进行中的 checkpoint 时，不应产生增量文件
        assert not any((tmp_path / "ck").glob("*_delta.json"))


class TestCleanerCustomRule:
    def test_custom_rule_applied_in_clean_flow(self):
        cleaner = DatasetCleaner()

        def mark_processed(items):
            for item in items:
                item["processed"] = True
            return items

        cleaner.add_custom_rule(mark_processed)
        out, result = cleaner.clean([
            {"instruction": "如何申请租房补贴？", "output": "携带身份证到政务中心窗口办理"}
        ])
        assert out[0]["processed"] is True
        assert "mark_processed" in result.rules_applied


class TestActiveLearningEarlyStop:
    def test_run_breaks_when_no_samples_selected(self):
        loop = ActiveLearningLoop()

        def empty_select(candidates, strategy=None):
            return []

        loop.select_samples = empty_select
        data = [{"instruction": "q1", "output": "a1"},
                {"instruction": "q2", "output": "a2"}]
        report = loop.run(data, iterations=3)
        # 选不出样本时不再空转后续轮次
        assert report["iterations"] == 0
        assert report["total_selected"] == 0
