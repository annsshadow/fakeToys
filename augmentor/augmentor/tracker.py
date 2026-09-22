# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""微调效果追踪模块"""

import json
import logging
from typing import List, Dict, Optional
from dataclasses import dataclass, asdict
from pathlib import Path
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class ExperimentResult:
    """实验结果"""
    experiment_id: str  # 实验 ID
    dataset_version: str  # 数据集版本
    model_name: str  # 模型名称
    start_time: str  # 开始时间
    end_time: str  # 结束时间
    metrics: Dict[str, List[float]]  # 指标历史
    final_metrics: Dict[str, float]  # 最终指标
    metadata: Dict  # 元数据


class ExperimentTracker:
    """实验追踪器"""
    
    def __init__(self, storage_dir: str = "experiments"):
        """初始化实验追踪器
        
        Args:
            storage_dir: 实验存储目录
        """
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        
        self._current_experiment: Optional[ExperimentResult] = None
    
    def _get_experiment_path(self, experiment_id: str) -> Path:
        """获取实验文件路径
        
        Args:
            experiment_id: 实验 ID
        
        Returns:
            实验文件路径
        """
        return self.storage_dir / f"{experiment_id}.json"
    
    def start_experiment(self,
                        experiment_id: str,
                        dataset_version: str,
                        model_name: str,
                        metadata: Optional[Dict] = None) -> ExperimentResult:
        """开始新实验
        
        Args:
            experiment_id: 实验 ID
            dataset_version: 数据集版本
            model_name: 模型名称
            metadata: 元数据
        
        Returns:
            ExperimentResult 实例
        """
        experiment = ExperimentResult(
            experiment_id=experiment_id,
            dataset_version=dataset_version,
            model_name=model_name,
            start_time=datetime.now().isoformat(),
            end_time="",
            metrics={},
            final_metrics={},
            metadata=metadata or {}
        )
        
        self._current_experiment = experiment
        self._save_experiment(experiment)
        
        logger.info(f"开始实验: {experiment_id}")
        return experiment
    
    def _save_experiment(self, experiment: ExperimentResult):
        """保存实验
        
        Args:
            experiment: 实验结果
        """
        experiment_path = self._get_experiment_path(experiment.experiment_id)
        
        try:
            with open(experiment_path, 'w', encoding='utf-8') as f:
                json.dump(asdict(experiment), f, ensure_ascii=False, indent=2)
        except Exception as e:
            logger.error(f"保存实验失败: {e}")
    
    def log_metric(self, metric_name: str, value: float):
        """记录指标
        
        Args:
            metric_name: 指标名称
            value: 指标值
        """
        if self._current_experiment is None:
            return
        
        if metric_name not in self._current_experiment.metrics:
            self._current_experiment.metrics[metric_name] = []
        
        self._current_experiment.metrics[metric_name].append(value)
        self._save_experiment(self._current_experiment)
    
    def end_experiment(self):
        """结束实验"""
        if self._current_experiment is None:
            return
        
        self._current_experiment.end_time = datetime.now().isoformat()
        
        # 计算最终指标
        for metric_name, values in self._current_experiment.metrics.items():
            if values:
                self._current_experiment.final_metrics[metric_name] = values[-1]
        
        self._save_experiment(self._current_experiment)
        logger.info(f"结束实验: {self._current_experiment.experiment_id}")
        
        self._current_experiment = None
    
    def load_experiment(self, experiment_id: str) -> Optional[ExperimentResult]:
        """加载实验
        
        Args:
            experiment_id: 实验 ID
        
        Returns:
            ExperimentResult 实例
        """
        experiment_path = self._get_experiment_path(experiment_id)
        
        if not experiment_path.exists():
            logger.info(f"实验不存在: {experiment_id}")
            return None
        
        try:
            with open(experiment_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            return ExperimentResult(**data)
        except Exception as e:
            logger.error(f"加载实验失败: {e}")
            return None
    
    def list_experiments(self) -> List[ExperimentResult]:
        """列出所有实验
        
        Returns:
            实验列表
        """
        experiments = []
        
        for file in self.storage_dir.glob("*.json"):
            try:
                with open(file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                experiments.append(ExperimentResult(**data))
            except Exception as e:
                logger.warning(f"加载实验失败: {file}, {e}")
        
        experiments.sort(key=lambda x: x.start_time, reverse=True)
        return experiments
    
    def compare_experiments(self,
                          experiment_ids: List[str]) -> Dict:
        """对比多个实验
        
        Args:
            experiment_ids: 实验 ID 列表
        
        Returns:
            对比结果字典
        """
        experiments = []
        for exp_id in experiment_ids:
            exp = self.load_experiment(exp_id)
            if exp:
                experiments.append(exp)
        
        if not experiments:
            return {"error": "没有找到实验"}
        
        # 收集所有指标
        all_metrics = set()
        for exp in experiments:
            all_metrics.update(exp.final_metrics.keys())
        
        # 对比结果
        comparison = {}
        for metric in all_metrics:
            comparison[metric] = {}
            for exp in experiments:
                comparison[metric][exp.experiment_id] = exp.final_metrics.get(metric)
        
        return {
            "experiments": [exp.experiment_id for exp in experiments],
            "dataset_versions": [exp.dataset_version for exp in experiments],
            "model_names": [exp.model_name for exp in experiments],
            "metric_comparison": comparison
        }
    
    def generate_report(self, experiment_id: str) -> Dict:
        """生成实验报告
        
        Args:
            experiment_id: 实验 ID
        
        Returns:
            实验报告字典
        """
        experiment = self.load_experiment(experiment_id)
        
        if experiment is None:
            return {"error": f"实验不存在: {experiment_id}"}
        
        return {
            "experiment_id": experiment.experiment_id,
            "dataset_version": experiment.dataset_version,
            "model_name": experiment.model_name,
            "start_time": experiment.start_time,
            "end_time": experiment.end_time,
            "duration": self._calculate_duration(experiment),
            "metrics_history": experiment.metrics,
            "final_metrics": experiment.final_metrics,
            "metadata": experiment.metadata
        }
    
    def _calculate_duration(self, experiment: ExperimentResult) -> str:
        """计算实验时长
        
        Args:
            experiment: 实验结果
        
        Returns:
            时长字符串
        """
        if not experiment.end_time:
            return "进行中"
        
        try:
            start = datetime.fromisoformat(experiment.start_time)
            end = datetime.fromisoformat(experiment.end_time)
            duration = (end - start).total_seconds()
            
            if duration < 60:
                return f"{duration:.1f}秒"
            elif duration < 3600:
                return f"{duration / 60:.1f}分钟"
            else:
                return f"{duration / 3600:.1f}小时"
        except Exception:
            return "未知"
