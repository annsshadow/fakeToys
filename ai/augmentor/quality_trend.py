"""数据质量趋势追踪模块

功能：记录数据集质量指标随时间变化，支持趋势方向分析、
报告生成和数据集间趋势比较，支持持久化到 JSON 文件。

使用示例：
    from augmentor.quality_trend import QualityTrendTracker
    tracker = QualityTrendTracker(storage_path="trends.json")
    tracker.record_quality_metrics("v1", {"completeness": 0.85})
    report = tracker.generate_trend_report()
"""

import logging
from typing import List, Dict, Optional
from datetime import datetime
from pathlib import Path
import json

logger = logging.getLogger(__name__)


class QualityTrendTracker:
    """数据质量趋势追踪器"""
    
    def __init__(self, storage_path: Optional[str] = None):
        self.storage_path = Path(storage_path) if storage_path else None
        self._trend_history: List[Dict] = []
        if self.storage_path and self.storage_path.exists():
            self._load_history()
    
    def _load_history(self):
        """从文件加载历史记录"""
        try:
            with open(self.storage_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
                self._trend_history = data.get("trends", [])
        except Exception as e:
            logger.warning(f"加载趋势历史失败: {e}")
    
    def _save_history(self):
        """保存历史记录到文件"""
        if not self.storage_path:
            return
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        try:
            with open(self.storage_path, 'w', encoding='utf-8') as f:
                json.dump({"trends": self._trend_history}, f, ensure_ascii=False)
        except Exception as e:
            logger.error(f"保存趋势历史失败: {e}")
    
    def record_quality_metrics(self,
                                dataset_name: str,
                                metrics: Dict[str, float],
                                sample_count: int = 0) -> Dict:
        """记录质量指标
        
        Args:
            dataset_name: 数据集名称
            metrics: 质量指标字典
            sample_count: 样本数量
        
        Returns:
            记录结果
        """
        entry = {
            "dataset_name": dataset_name,
            "timestamp": datetime.now().isoformat(),
            "metrics": metrics,
            "sample_count": sample_count
        }
        self._trend_history.append(entry)
        self._save_history()
        logger.info(f"记录质量趋势: {dataset_name} (样本: {sample_count})")
        return entry
    
    def get_trend_for_metric(self, metric_name: str, dataset_name: Optional[str] = None) -> List[Dict]:
        """获取指定指标的趋势数据
        
        Args:
            metric_name: 指标名称
            dataset_name: 数据集名称（可选过滤）
        
        Returns:
            趋势数据列表
        """
        trend = []
        for entry in self._trend_history:
            if dataset_name is not None and entry.get("dataset_name") != dataset_name:
                continue
            if metric_name in entry.get("metrics", {}):
                trend.append({
                    "timestamp": entry["timestamp"],
                    "value": entry["metrics"][metric_name],
                    "dataset_name": entry.get("dataset_name", "unknown"),
                    "sample_count": entry.get("sample_count", 0)
                })
        return trend
    
    def calculate_trend_direction(self, metric_name: str, dataset_name: Optional[str] = None) -> str:
        """计算趋势方向
        
        Args:
            metric_name: 指标名称
            dataset_name: 数据集名称
        
        Returns:
            趋势方向（"improving", "declining", "stable", "insufficient_data"）
        """
        values = [t["value"] for t in self.get_trend_for_metric(metric_name, dataset_name)]
        if len(values) < 2:
            return "insufficient_data"

        # values 长度已保证 ≥2，最近窗口长度随之 ≥2，无需二次检查
        recent_values = values[-min(5, len(values)):]

        first = recent_values[0]
        last = recent_values[-1]
        
        # 计算变化率
        change_rate = (last - first) / (abs(first) + 1e-6)
        
        if abs(change_rate) < 0.05:
            return "stable"
        return "improving" if change_rate > 0 else "declining"
    
    def generate_trend_report(self, dataset_name: Optional[str] = None) -> Dict:
        """生成趋势报告
        
        Args:
            dataset_name: 数据集名称（可选）
        
        Returns:
            趋势报告字典
        """
        if dataset_name:
            entries = [e for e in self._trend_history if e.get("dataset_name") == dataset_name]
        else:
            entries = self._trend_history
        
        if not entries:
            return {"status": "no_data", "message": "无趋势数据"}
        
        # 提取所有指标
        all_metrics = set()
        for entry in entries:
            all_metrics.update(entry.get("metrics", {}).keys())
        
        trends = {}
        for metric in all_metrics:
            direction = self.calculate_trend_direction(metric, dataset_name)
            values = [t["value"] for t in self.get_trend_for_metric(metric, dataset_name)]
            trends[metric] = {
                "direction": direction,
                "latest_value": values[-1] if values else None,
                "value_count": len(values),
                "min_value": min(values) if values else None,
                "max_value": max(values) if values else None
            }
        
        return {
            "status": "ok",
            "dataset_filter": dataset_name,
            "total_entries": len(entries),
            "metrics_trends": trends,
            "latest_timestamp": entries[-1].get("timestamp") if entries else None
        }
    
    def compare_trends(self, dataset_a: str, dataset_b: str, metric_name: str) -> Dict:
        """比较两个数据集的趋势
        
        Args:
            dataset_a: 数据集 A 名称
            dataset_b: 数据集 B 名称
            metric_name: 指标名称
        
        Returns:
            比较结果
        """
        trend_a = self.get_trend_for_metric(metric_name, dataset_a)
        trend_b = self.get_trend_for_metric(metric_name, dataset_b)
        
        return {
            "dataset_a": dataset_a,
            "dataset_b": dataset_b,
            "metric_name": metric_name,
            "dataset_a_values": len(trend_a),
            "dataset_b_values": len(trend_b),
            "dataset_a_latest": trend_a[-1]["value"] if trend_a else None,
            "dataset_b_latest": trend_b[-1]["value"] if trend_b else None,
            "comparison": "dataset_a_higher" if (trend_a[-1]["value"] if trend_a else 0) > (trend_b[-1]["value"] if trend_b else 0) else "dataset_b_higher"
        }
