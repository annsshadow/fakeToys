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


class TestQualityMonitorExtended2:
    """QualityMonitor 扩展测试 - 第二轮"""

    def test_add_alert_callback(self):
        """添加告警回调"""
        monitor = QualityMonitor()
        callback = lambda alert: None
        monitor.add_alert_callback(callback)
        assert len(monitor._alert_callbacks) == 1

    def test_alert_callback_triggered(self):
        """告警回调被触发"""
        monitor = QualityMonitor()
        triggered = [False]
        def callback(alert):
            triggered[0] = True
        monitor.add_alert_callback(callback)
        bad_data = [{"instruction": "", "output": ""}]
        monitor.check_quality(bad_data)
        assert triggered[0] is True

    def test_alert_callback_exception_handled(self):
        """告警回调异常被处理"""
        monitor = QualityMonitor()
        def bad_callback(alert):
            raise RuntimeError("callback error")
        monitor.add_alert_callback(bad_callback)
        bad_data = [{"instruction": "", "output": ""}]
        snapshot = monitor.check_quality(bad_data)
        assert snapshot is not None

    def test_calculate_metrics_empty(self):
        """空数据集计算指标"""
        monitor = QualityMonitor()
        metrics = monitor._calculate_metrics([])
        assert isinstance(metrics, dict)

    def test_calculate_metrics_with_data(self):
        """有数据集计算指标"""
        monitor = QualityMonitor()
        data = [{"instruction": "q1", "output": "a1"}]
        metrics = monitor._calculate_metrics(data)
        assert "completeness" in metrics
        assert metrics["completeness"] == 1.0

    def test_check_alerts_with_breach(self):
        """检查告警触发"""
        monitor = QualityMonitor()
        monitor.add_threshold(QualityThreshold(
            metric_name="test_metric", min_value=0.0,
            max_value=1.0, alert_below=0.5
        ))
        alerts = monitor._check_alerts({"test_metric": 0.2})
        assert len(alerts) > 0

    def test_check_alerts_no_breach(self):
        """检查告警未触发"""
        monitor = QualityMonitor()
        monitor.add_threshold(QualityThreshold(
            metric_name="test_metric", min_value=0.0,
            max_value=1.0, alert_below=0.5
        ))
        alerts = monitor._check_alerts({"test_metric": 0.8})
        assert len(alerts) == 0

    def test_check_alerts_above_breach(self):
        """高于 alert_above 应产生 above 告警"""
        monitor = QualityMonitor()
        monitor.add_threshold(QualityThreshold(
            metric_name="sim", min_value=0.0, max_value=1.0,
            alert_above=0.9
        ))
        alerts = monitor._check_alerts({"sim": 0.95})
        assert len(alerts) == 1
        assert alerts[0].alert_type == "above"
        assert alerts[0].severity == "warning"
        assert alerts[0].metric_name == "sim"

    def test_check_alerts_above_and_below_together(self):
        """同时设置上下限时，仅触发越界的一侧"""
        monitor = QualityMonitor()
        monitor.add_threshold(QualityThreshold(
            metric_name="m", min_value=0.0, max_value=1.0,
            alert_below=0.2, alert_above=0.8
        ))
        assert monitor._check_alerts({"m": 0.99})  # 仅 above
        assert monitor._check_alerts({"m": 0.01})  # 仅 below
        assert monitor._check_alerts({"m": 0.5}) == []  # 正常区间


class TestQualityMonitorExtended3:
    """QualityMonitor 第三轮扩展测试"""

    def test_check_quality_triggers_above_alert_callback(self):
        """高于上限触发告警回调"""
        monitor = QualityMonitor()
        monitor.add_threshold(QualityThreshold(
            metric_name="completeness", min_value=0.0, max_value=1.0,
            alert_above=0.9
        ))
        triggered = []
        monitor.add_alert_callback(triggered.append)
        # 全满数据 completeness=1.0 > 0.9
        snapshot = monitor.check_quality(
            [{"instruction": "这是一个足够长的问题", "output": "这是一个足够长的回答内容"}]
        )
        assert len(triggered) >= 1
        assert triggered[0].alert_type == "above"

    def test_get_trend_empty(self):
        """无数据时趋势应为空"""
        monitor = QualityMonitor()
        assert monitor.get_trend("completeness") == []

    def test_get_trend_with_snapshots(self, sample_dataset):
        """多次检查后趋势应按时间有序"""
        monitor = QualityMonitor()
        monitor.check_quality(sample_dataset)
        monitor.check_quality(sample_dataset)
        trend = monitor.get_trend("completeness")
        assert len(trend) == 2
        assert "value" in trend[0]

    def test_get_history_limit_zero(self, sample_dataset):
        """limit=0 时历史应返回全部快照（切片行为）"""
        monitor = QualityMonitor()
        monitor.check_quality(sample_dataset)
        history = monitor.get_history(limit=0)
        # 切片 [-0:] 等价于全部，这是现有实现语义
        assert len(history) == 1

    def test_get_summary_with_data(self, sample_dataset):
        """有数据时摘要包含最新快照指标与告警数"""
        monitor = QualityMonitor()
        monitor.check_quality(sample_dataset)
        summary = monitor.get_summary()
        assert "total_snapshots" in summary
        assert summary["total_snapshots"] == 1
        assert "latest_metrics" in summary
        assert summary["alert_count"] == 0
