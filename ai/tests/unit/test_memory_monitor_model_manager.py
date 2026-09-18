"""memory_monitor 与 model_manager 剩余分支测试

memory_monitor：注入伪 psutil 覆盖「可用」分支、趋势 increasing/
decreasing/fluctuating、告警、装饰器与摘要；
model_manager：单例、sentence-transformers 缺失回退、缓存复用、清理。
"""

import importlib
import sys
import types

import pytest

import augmentor.memory_monitor as mm_module


class _FakeMemoryInfo:
    rss = 200 * 1024 * 1024  # 200 MB


class _FakeProcess:
    def memory_info(self):
        return _FakeMemoryInfo()


def _install_fake_psutil(monkeypatch):
    fake = types.ModuleType("psutil")

    def Process(*a, **kw):
        return _FakeProcess()

    fake.Process = Process
    monkeypatch.setitem(sys.modules, "psutil", fake)


class TestMemoryMonitorPsutilPath:
    def test_psutil_available_uses_rss(self, monkeypatch):
        _install_fake_psutil(monkeypatch)
        importlib.reload(mm_module)
        try:
            assert mm_module.HAS_PSUTIL is True
            monitor = mm_module.MemoryMonitor()
            usage = monitor.get_memory_usage_mb()
            assert usage == pytest.approx(200.0)
        finally:
            # 恢复真实（无 psutil）状态
            monkeypatch.delitem(sys.modules, "psutil", raising=False)
            importlib.reload(mm_module)
            assert mm_module.HAS_PSUTIL is False

    def test_get_memory_summary_reflects_flag(self, monkeypatch):
        _install_fake_psutil(monkeypatch)
        importlib.reload(mm_module)
        try:
            summary = mm_module.get_memory_summary()
            assert summary["monitor_available"] is True
            assert summary["current_usage_mb"] > 0
        finally:
            monkeypatch.delitem(sys.modules, "psutil", raising=False)
            importlib.reload(mm_module)


class TestMemoryTrends:
    def _monitor_with_usages(self, usages):
        monitor = mm_module.MemoryMonitor()
        for u in usages:
            monitor._current_usage_mb = u
            monitor.take_snapshot()
        return monitor

    def test_trend_increasing(self, monkeypatch):
        monitor = self._monitor_with_usages([1, 2, 3, 4])
        assert monitor.get_trend() == "increasing"

    def test_trend_decreasing(self, monkeypatch):
        monitor = self._monitor_with_usages([4, 3, 2, 1])
        assert monitor.get_trend() == "decreasing"

    def test_trend_fluctuating(self, monkeypatch):
        monitor = self._monitor_with_usages([1, 3, 2, 4])
        assert monitor.get_trend() == "fluctuating"

    def test_trend_stable_single(self, monkeypatch):
        monitor = self._monitor_with_usages([1])
        assert monitor.get_trend() == "stable"

    def test_peak_usage(self, monkeypatch):
        monitor = self._monitor_with_usages([5, 9, 3])
        assert monitor.get_peak_usage_mb() == 9

    def test_peak_empty_zero(self, monkeypatch):
        monitor = mm_module.MemoryMonitor()
        assert monitor.get_peak_usage_mb() == 0.0

    def test_alert_triggered_on_high_usage(self, monkeypatch, caplog):
        monitor = mm_module.MemoryMonitor(interval_mb=10, alert_threshold_percent=50.0)
        monitor._current_usage_mb = 100.0
        with caplog.at_level("WARNING"):
            snapshot = monitor.take_snapshot()
        assert snapshot["alert"] is True
        assert any("内存使用警告" in r.message for r in caplog.records)

    def test_reset_clears_state(self, monkeypatch):
        monitor = self._monitor_with_usages([1, 2, 3])
        monitor.reset()
        assert monitor._snapshots == []
        assert monitor._current_usage_mb == 0.0


class TestMonitorMemoryUsageDecorator:
    def test_wrapper_records_snapshots_and_returns(self, monkeypatch):
        calls = []

        @mm_module.monitor_memory_usage
        def work(x):
            calls.append(x)
            return x * 2

        assert work(3) == 6
        assert calls == [3]
        monitor = work.memory_monitor
        assert len(monitor._snapshots) == 2


class TestModelManager:
    @pytest.fixture(autouse=True)
    def _reset_singleton(self):
        from augmentor.model_manager import ModelManager

        original = ModelManager._instance
        ModelManager._instance = None
        yield
        # 恢复原全局单例，避免影响其他依赖它的测试
        ModelManager._instance = original

    def test_singleton_identity(self):
        from augmentor.model_manager import ModelManager

        a = ModelManager()
        b = ModelManager()
        assert a is b

    def test_sentence_model_fallback_when_missing(self):
        from augmentor.model_manager import ModelManager

        model = ModelManager()
        # sentence-transformers 未安装 → 回退哨兵
        result = model.get_sentence_model()
        assert result == "fallback"

    def test_cache_reuse_returns_same_instance(self):
        from augmentor.model_manager import ModelManager

        model = ModelManager()
        model._sentence_model = "cached_obj"
        assert model.get_sentence_model() is model._sentence_model

    def test_clear_resets_cache(self):
        from augmentor.model_manager import ModelManager

        model = ModelManager()
        model._sentence_model = "cached_obj"
        model.clear()
        assert model._sentence_model is None

    def test_global_instance_available(self):
        import augmentor.model_manager as mm

        from augmentor.model_manager import ModelManager

        assert isinstance(mm.model_manager, ModelManager)
