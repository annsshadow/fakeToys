"""数据集质量监控模块测试"""

import pytest
from augmentor.quality_monitor import (
    QualityMonitor, QualityThreshold, QualityAlert, QualitySnapshot,
    monitor_quality, create_monitor
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


@pytest.fixture
def bad_dataset():
    """创建有问题的数据集"""
    return [
        {"instruction": "", "input": "", "output": ""},
        {"instruction": "问", "input": "", "output": "答"},
    ]


class TestQualityMonitor:
    """QualityMonitor 测试"""
    
    def test_init(self):
        """测试初始化"""
        monitor = QualityMonitor()
        assert len(monitor._thresholds) > 0
    
    def test_add_threshold(self):
        """测试添加阈值"""
        monitor = QualityMonitor()
        threshold = QualityThreshold(
            metric_name="test_metric",
            min_value=0.0,
            max_value=1.0,
            alert_below=0.3
        )
        
        monitor.add_threshold(threshold)
        assert "test_metric" in monitor._thresholds
    
    def test_check_quality(self, sample_dataset):
        """测试检查质量"""
        monitor = QualityMonitor()
        snapshot = monitor.check_quality(sample_dataset)
        
        assert isinstance(snapshot, QualitySnapshot)
        assert len(snapshot.metrics) > 0
        assert snapshot.timestamp
    
    def test_check_quality_bad_dataset(self, bad_dataset):
        """测试检查有问题的数据集"""
        monitor = QualityMonitor()
        snapshot = monitor.check_quality(bad_dataset)
        
        assert len(snapshot.alerts) > 0
    
    def test_get_history(self, sample_dataset):
        """测试获取历史记录"""
        monitor = QualityMonitor()
        monitor.check_quality(sample_dataset)
        monitor.check_quality(sample_dataset)
        
        history = monitor.get_history(limit=5)
        assert len(history) == 2
    
    def test_get_trend(self, sample_dataset):
        """测试获取趋势"""
        monitor = QualityMonitor()
        monitor.check_quality(sample_dataset)
        monitor.check_quality(sample_dataset)
        
        trend = monitor.get_trend("completeness")
        assert len(trend) == 2
    
    def test_get_summary(self, sample_dataset):
        """测试获取摘要"""
        monitor = QualityMonitor()
        monitor.check_quality(sample_dataset)
        
        summary = monitor.get_summary()
        assert "total_snapshots" in summary
        assert "latest_metrics" in summary


class TestQualityThreshold:
    """QualityThreshold 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        threshold = QualityThreshold(
            metric_name="test",
            min_value=0.0,
            max_value=1.0,
            alert_below=0.3,
            alert_above=0.9
        )
        
        d = threshold.to_dict()
        
        assert d["metric_name"] == "test"
        assert d["alert_below"] == 0.3
        assert d["alert_above"] == 0.9


class TestQualityAlert:
    """QualityAlert 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        alert = QualityAlert(
            alert_id="test_alert",
            metric_name="test_metric",
            current_value=0.2,
            threshold_value=0.3,
            alert_type="below",
            severity="warning",
            message="测试告警",
            timestamp="2024-01-01T00:00:00"
        )
        
        d = alert.to_dict()
        
        assert d["alert_id"] == "test_alert"
        assert d["current_value"] == 0.2
        assert d["severity"] == "warning"


class TestQualitySnapshot:
    """QualitySnapshot 测试"""
    
    def test_to_dict(self, sample_dataset):
        """测试转换为字典"""
        monitor = QualityMonitor()
        snapshot = monitor.check_quality(sample_dataset)
        d = snapshot.to_dict()
        
        assert isinstance(d, dict)
        assert "snapshot_id" in d
        assert "metrics" in d
        assert "alerts" in d


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_monitor_quality(self, sample_dataset):
        """测试监控质量"""
        snapshot = monitor_quality(sample_dataset)
        
        assert isinstance(snapshot, QualitySnapshot)
        assert len(snapshot.metrics) > 0
    
    def test_create_monitor(self):
        """测试创建监控器"""
        monitor = create_monitor()
        
        assert isinstance(monitor, QualityMonitor)


class TestQualityMonitorExtended:
    """QualityMonitor 扩展测试"""

    def test_check_empty_dataset(self):
        """检查空数据集"""
        monitor = QualityMonitor()
        snapshot = monitor.check_quality([])
        assert snapshot is not None

    def test_history_empty(self):
        """空历史记录"""
        monitor = QualityMonitor()
        history = monitor.get_history()
        assert history == []

    def test_trend_empty(self):
        """空趋势"""
        monitor = QualityMonitor()
        trend = monitor.get_trend("completeness")
        assert trend == []

    def test_summary_empty(self):
        """空摘要"""
        monitor = QualityMonitor()
        summary = monitor.get_summary()
        assert summary["status"] == "no_data"

    def test_quality_threshold_to_dict(self):
        """QualityThreshold.to_dict 完整性"""
        threshold = QualityThreshold(
            metric_name="test", min_value=0.0,
            max_value=1.0, alert_below=0.3
        )
        d = threshold.to_dict()
        assert d["min_value"] == 0.0
        assert d["max_value"] == 1.0

    def test_quality_alert_fields(self):
        """QualityAlert 字段检查"""
        alert = QualityAlert(
            alert_id="a1", metric_name="m",
            current_value=0.5, threshold_value=0.3,
            alert_type="below", severity="warning",
            message="msg", timestamp="2024-01-01T00:00:00"
        )
        assert alert.alert_id == "a1"
        assert alert.severity == "warning"

    def test_check_with_custom_threshold(self):
        """自定义阈值检查"""
        monitor = QualityMonitor()
        monitor.add_threshold(QualityThreshold(
            metric_name="custom_metric",
            min_value=0.0, max_value=1.0,
            alert_below=0.3, alert_above=0.9
        ))
        data = [{"instruction": "test"}]
        snapshot = monitor.check_quality(data)
        assert snapshot is not None

    def test_history_with_limit(self):
        """限制历史记录数量"""
        monitor = QualityMonitor()
        data = [{"instruction": "test"}]
        monitor.check_quality(data)
        monitor.check_quality(data)
        history = monitor.get_history(limit=1)
        assert len(history) == 1

    def test_snapshot_to_dict(self, sample_dataset):
        """Snapshot.to_dict 完整性"""
        monitor = QualityMonitor()
        snapshot = monitor.check_quality(sample_dataset)
        d = snapshot.to_dict()
        assert "snapshot_id" in d
        assert "metrics" in d
        assert "alerts" in d
        assert "timestamp" in d
