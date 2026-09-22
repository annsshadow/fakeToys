# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Quality monitor module"""

import logging
from typing import List, Dict, Optional, Any, Callable
from dataclasses import dataclass, field
from pathlib import Path
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class QualityThreshold:
    metric_name: str
    min_value: float
    max_value: float
    alert_below: Optional[float] = None
    alert_above: Optional[float] = None

    def to_dict(self) -> Dict:
        return {
            "metric_name": self.metric_name,
            "min_value": self.min_value,
            "max_value": self.max_value,
            "alert_below": self.alert_below,
            "alert_above": self.alert_above,
        }


@dataclass
class QualityAlert:
    alert_id: str
    metric_name: str
    current_value: float
    threshold_value: float
    alert_type: str
    severity: str
    message: str
    timestamp: str

    def to_dict(self) -> Dict:
        return {
            "alert_id": self.alert_id,
            "metric_name": self.metric_name,
            "current_value": self.current_value,
            "threshold_value": self.threshold_value,
            "alert_type": self.alert_type,
            "severity": self.severity,
            "message": self.message,
            "timestamp": self.timestamp,
        }


@dataclass
class QualitySnapshot:
    snapshot_id: str
    timestamp: str
    metrics: Dict[str, float]
    alerts: List[QualityAlert] = field(default_factory=list)

    def to_dict(self) -> Dict:
        return {
            "snapshot_id": self.snapshot_id,
            "timestamp": self.timestamp,
            "metrics": self.metrics,
            "alerts": [a.to_dict() for a in self.alerts],
        }


class QualityMonitor:
    def __init__(self):
        self._thresholds: Dict[str, QualityThreshold] = {}
        self._snapshots: List[QualitySnapshot] = []
        self._alert_callbacks: List[Callable] = []
        self._setup_default_thresholds()

    def _setup_default_thresholds(self):
        self.add_threshold(QualityThreshold(
            metric_name="completeness",
            min_value=0.8,
            max_value=1.0,
            alert_below=0.7,
        ))
        self.add_threshold(QualityThreshold(
            metric_name="diversity",
            min_value=0.5,
            max_value=1.0,
            alert_below=0.4,
        ))
        self.add_threshold(QualityThreshold(
            metric_name="consistency",
            min_value=0.8,
            max_value=1.0,
            alert_below=0.7,
        ))

    def add_threshold(self, threshold: QualityThreshold):
        self._thresholds[threshold.metric_name] = threshold

    def add_alert_callback(self, callback: Callable):
        self._alert_callbacks.append(callback)

    def check_quality(self, items: List[Dict]) -> QualitySnapshot:
        import hashlib
        metrics = self._calculate_metrics(items)
        alerts = self._check_alerts(metrics)
        snapshot = QualitySnapshot(
            snapshot_id=hashlib.md5(str(datetime.now()).encode()).hexdigest()[:8],
            timestamp=datetime.now().isoformat(),
            metrics=metrics,
            alerts=alerts,
        )
        self._snapshots.append(snapshot)
        for alert in alerts:
            for callback in self._alert_callbacks:
                try:
                    callback(alert)
                except Exception as e:
                    logger.error(f"Alert callback failed: {e}")
        return snapshot

    def _calculate_metrics(self, items: List[Dict]) -> Dict[str, float]:
        metrics: Dict[str, float] = {}
        if not items:
            return metrics
        required_fields = ["instruction", "output"]
        total_required = len(required_fields) * len(items)
        filled_required = sum(
            1 for item in items
            for field in required_fields
            if item.get(field)
        )
        metrics["completeness"] = filled_required / total_required if total_required > 0 else 0
        instructions = [item.get("instruction", "") for item in items if item.get("instruction")]
        if instructions:
            unique_instructions = set(instructions)
            metrics["diversity"] = len(unique_instructions) / len(instructions)
        else:
            metrics["diversity"] = 0
        consistent_count = 0
        for item in items:
            instruction = item.get("instruction", "")
            output = item.get("output", "")
            if instruction and output and instruction != output:
                consistent_count += 1
        metrics["consistency"] = consistent_count / len(items) if items else 0
        return metrics

    def _check_alerts(self, metrics: Dict[str, float]) -> List[QualityAlert]:
        alerts: List[QualityAlert] = []
        for metric_name, threshold in self._thresholds.items():
            if metric_name in metrics:
                current_value = metrics[metric_name]
                if threshold.alert_below is not None and current_value < threshold.alert_below:
                    alerts.append(QualityAlert(
                        alert_id=f"{metric_name}_below",
                        metric_name=metric_name,
                        current_value=current_value,
                        threshold_value=threshold.alert_below,
                        alert_type="below",
                        severity="critical" if current_value < threshold.min_value else "warning",
                        message=f"{metric_name} below threshold: {current_value:.2f} < {threshold.alert_below:.2f}",
                        timestamp=datetime.now().isoformat(),
                    ))
                if threshold.alert_above is not None and current_value > threshold.alert_above:
                    alerts.append(QualityAlert(
                        alert_id=f"{metric_name}_above",
                        metric_name=metric_name,
                        current_value=current_value,
                        threshold_value=threshold.alert_above,
                        alert_type="above",
                        severity="warning",
                        message=f"{metric_name} above threshold: {current_value:.2f} > {threshold.alert_above:.2f}",
                        timestamp=datetime.now().isoformat(),
                    ))
        return alerts

    def get_history(self, limit: int = 10) -> List[QualitySnapshot]:
        return self._snapshots[-limit:]

    def get_trend(self, metric_name: str, limit: int = 10) -> List[Dict]:
        trend = []
        for snapshot in self._snapshots[-limit:]:
            if metric_name in snapshot.metrics:
                trend.append({
                    "timestamp": snapshot.timestamp,
                    "value": snapshot.metrics[metric_name],
                })
        return trend

    def get_summary(self) -> Dict:
        if not self._snapshots:
            return {"status": "no_data"}
        latest = self._snapshots[-1]
        return {
            "total_snapshots": len(self._snapshots),
            "latest_timestamp": latest.timestamp,
            "latest_metrics": latest.metrics,
            "latest_alerts": [a.to_dict() for a in latest.alerts],
            "alert_count": len(latest.alerts),
        }


def monitor_quality(items: List[Dict]) -> QualitySnapshot:
    monitor = QualityMonitor()
    return monitor.check_quality(items)


def create_monitor() -> QualityMonitor:
    return QualityMonitor()


__all__ = [
    "QualityMonitor",
    "QualityThreshold",
    "QualityAlert",
    "QualitySnapshot",
    "monitor_quality",
    "create_monitor",
]
