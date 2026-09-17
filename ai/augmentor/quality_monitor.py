"""数据集质量监控模块

提供数据集质量监控和告警功能。
"""

import json
import logging
from typing import List, Dict, Optional, Any, Callable
from dataclasses import dataclass, field
from pathlib import Path
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class QualityThreshold:
    """质量阈值"""
    metric_name: str
    min_value: float
    max_value: float
    alert_below: Optional[float] = None
    alert_above: Optional[float] = None
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "metric_name": self.metric_name,
            "min_value": self.min_value,
            "max_value": self.max_value,
            "alert_below": self.alert_below,
            "alert_above": self.alert_above
        }


@dataclass
class QualityAlert:
    """质量告警"""
    alert_id: str
    metric_name: str
    current_value: float
    threshold_value: float
    alert_type: str  # below, above
    severity: str  # warning, critical
    message: str
    timestamp: str
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "alert_id": self.alert_id,
            "metric_name": self.metric_name,
            "current_value": self.current_value,
            "threshold_value": self.threshold_value,
            "alert_type": self.alert_type,
            "severity": self.severity,
            "message": self.message,
            "timestamp": self.timestamp
        }


@dataclass
class QualitySnapshot:
    """质量快照"""
    snapshot_id: str
    timestamp: str
    metrics: Dict[str, float]
    alerts: List[QualityAlert] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "snapshot_id": self.snapshot_id,
            "timestamp": self.timestamp,
            "metrics": self.metrics,
            "alerts": [a.to_dict() for a in self.alerts]
        }


class QualityMonitor:
    """质量监控器
    
    提供数据集质量监控和告警功能。
    """
    
    def __init__(self):
        """初始化监控器"""
        self._thresholds: Dict[str, QualityThreshold] = {}
        self._snapshots: List[QualitySnapshot] = []
        self._alert_callbacks: List[Callable] = []
        
        # 设置默认阈值
        self._setup_default_thresholds()
    
    def _setup_default_thresholds(self):
        """设置默认阈值"""
        self.add_threshold(QualityThreshold(
            metric_name="completeness",
            min_value=0.8,
            max_value=1.0,
            alert_below=0.7
        ))
        
        self.add_threshold(QualityThreshold(
            metric_name="diversity",
            min_value=0.5,
            max_value=1.0,
            alert_below=0.4
        ))
        
        self.add_threshold(QualityThreshold(
            metric_name="consistency",
            min_value=0.8,
            max_value=1.0,
            alert_below=0.7
        ))
    
    def add_threshold(self, threshold: QualityThreshold):
        """添加阈值
        
        Args:
            threshold: 质量阈值
        """
        self._thresholds[threshold.metric_name] = threshold
    
    def add_alert_callback(self, callback: Callable):
        """添加告警回调
        
        Args:
            callback: 回调函数
        """
        self._alert_callbacks.append(callback)
    
    def check_quality(self, items: List[Dict]) -> QualitySnapshot:
        """检查质量
        
        Args:
            items: 数据列表
        
        Returns:
            质量快照
        """
        import hashlib
        
        # 计算指标
        metrics = self._calculate_metrics(items)
        
        # 检查告警
        alerts = self._check_alerts(metrics)
        
        # 创建快照
        snapshot = QualitySnapshot(
            snapshot_id=hashlib.md5(str(datetime.now()).encode()).hexdigest()[:8],
            timestamp=datetime.now().isoformat(),
            metrics=metrics,
            alerts=alerts
        )
        
        self._snapshots.append(snapshot)
        
        # 触发告警回调
        for alert in alerts:
            for callback in self._alert_callbacks:
                try:
                    callback(alert)
                except Exception as e:
                    logger.error(f"告警回调执行失败: {e}")
        
        return snapshot
    
    def _calculate_metrics(self, items: List[Dict]) -> Dict[str, float]:
        """计算指标
        
        Args:
            items: 数据列表
        
        Returns:
            指标字典
        """
        metrics = {}
        
        if not items:
            return metrics
        
        # 完整性
        required_fields = ["instruction", "output"]
        total_required = len(required_fields) * len(items)
        filled_required = sum(
            1 for item in items
            for field in required_fields
            if item.get(field)
        )
        metrics["completeness"] = filled_required / total_required if total_required > 0 else 0
        
        # 多样性
        instructions = [item.get("instruction", "") for item in items if item.get("instruction")]
        if instructions:
            unique_instructions = set(instructions)
            metrics["diversity"] = len(unique_instructions) / len(instructions)
        else:
            metrics["diversity"] = 0
        
        # 一致性
        consistent_count = 0
        for item in items:
            instruction = item.get("instruction", "")
            output = item.get("output", "")
            if instruction and output and instruction != output:
                consistent_count += 1
        metrics["consistency"] = consistent_count / len(items) if items else 0
        
        return metrics
    
    def _check_alerts(self, metrics: Dict[str, float]) -> List[QualityAlert]:
        """检查告警
        
        Args:
            metrics: 指标字典
        
        Returns:
            告警列表
        """
        alerts = []
        
        for metric_name, threshold in self._thresholds.items():
            if metric_name in metrics:
                current_value = metrics[metric_name]
                
                # 检查低于阈值
                if threshold.alert_below is not None and current_value < threshold.alert_below:
                    alert = QualityAlert(
                        alert_id=f"{metric_name}_below",
                        metric_name=metric_name,
                        current_value=current_value,
                        threshold_value=threshold.alert_below,
                        alert_type="below",
                        severity="critical" if current_value < threshold.min_value else "warning",
                        message=f"{metric_name} 低于阈值: {current_value:.2f} < {threshold.alert_below:.2f}",
                        timestamp=datetime.now().isoformat()
                    )
                    alerts.append(alert)
                
                # 检查高于阈值
                if threshold.alert_above is not None and current_value > threshold.alert_above:
                    alert = QualityAlert(
                        alert_id=f"{metric_name}_above",
                        metric_name=metric_name,
                        current_value=current_value,
                        threshold_value=threshold.alert_above,
                        alert_type="above",
                        severity="warning",
                        message=f"{metric_name} 高于阈值: {current_value:.2f} > {threshold.alert_above:.2f}",
                        timestamp=datetime.now().isoformat()
                    )
                    alerts.append(alert)
        
        return alerts
    
    def get_history(self, limit: int = 10) -> List[QualitySnapshot]:
        """获取历史记录
        
        Args:
            limit: 返回数量限制
        
        Returns:
            快照列表
        """
        return self._snapshots[-limit:]
    
    def get_trend(self, metric_name: str, limit: int = 10) -> List[Dict]:
        """获取趋势
        
        Args:
            metric_name: 指标名称
            limit: 返回数量限制
        
        Returns:
            趋势数据
        """
        trend = []
        
        for snapshot in self._snapshots[-limit:]:
            if metric_name in snapshot.metrics:
                trend.append({
                    "timestamp": snapshot.timestamp,
                    "value": snapshot.metrics[metric_name]
                })
        
        return trend
    
    def get_summary(self) -> Dict:
        """获取摘要
        
        Returns:
            摘要信息
        """
        if not self._snapshots:
            return {"status": "no_data"}
        
        latest = self._snapshots[-1]
        
        return {
            "total_snapshots": len(self._snapshots),
            "latest_timestamp": latest.timestamp,
            "latest_metrics": latest.metrics,
            "latest_alerts": [a.to_dict() for a in latest.alerts],
            "alert_count": len(latest.alerts)
        }


def monitor_quality(items: List[Dict]) -> QualitySnapshot:
    """监控质量
    
    Args:
        items: 数据列表
    
    Returns:
        质量快照
    """
    monitor = QualityMonitor()
    return monitor.check_quality(items)


def create_monitor() -> QualityMonitor:
    """创建监控器
    
    Returns:
        监控器实例
    """
    return QualityMonitor()
