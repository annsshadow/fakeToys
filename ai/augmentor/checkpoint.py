"""断点续传模块 - 优化版"""

import json
import logging
import time
import threading
from typing import List, Dict, Optional, Set
from dataclasses import dataclass, asdict
from pathlib import Path
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class CheckpointData:
    """断点数据"""
    task_id: str  # 任务 ID
    total_items: int  # 总条目数
    processed_items: int  # 已处理条目数
    failed_items: int  # 失败条目数
    completed_indices: List[int]  # 已完成的索引
    failed_indices: List[int]  # 失败的索引
    start_time: float  # 开始时间
    last_update_time: float  # 最后更新时间
    quality_scores: Dict[int, float]  # 质量评分


class CheckpointManager:
    """断点管理器 - 优化版"""
    
    def __init__(self, 
                 checkpoint_dir: str = "checkpoints",
                 auto_save_interval: int = 10):
        """初始化断点管理器
        
        Args:
            checkpoint_dir: 断点存储目录
            auto_save_interval: 自动保存间隔（每 N 条）
        """
        self.checkpoint_dir = Path(checkpoint_dir)
        self.checkpoint_dir.mkdir(parents=True, exist_ok=True)
        self.auto_save_interval = auto_save_interval
        
        self._current_checkpoint: Optional[CheckpointData] = None
        self._last_save_count = 0
        self._lock = threading.Lock()
        self._pending_completed: Set[int] = set()  # 待保存的完成索引
        self._pending_failed: Set[int] = set()  # 待保存的失败索引
    
    def _get_checkpoint_path(self, task_id: str) -> Path:
        """获取断点文件路径
        
        Args:
            task_id: 任务 ID
        
        Returns:
            断点文件路径
        """
        return self.checkpoint_dir / f"{task_id}_checkpoint.json"
    
    def _get_delta_path(self, task_id: str) -> Path:
        """获取增量文件路径
        
        Args:
            task_id: 任务 ID
        
        Returns:
            增量文件路径
        """
        return self.checkpoint_dir / f"{task_id}_delta.json"
    
    def create_checkpoint(self, 
                         task_id: str,
                         total_items: int) -> CheckpointData:
        """创建新的断点
        
        Args:
            task_id: 任务 ID
            total_items: 总条目数
        
        Returns:
            CheckpointData 实例
        """
        checkpoint = CheckpointData(
            task_id=task_id,
            total_items=total_items,
            processed_items=0,
            failed_items=0,
            completed_indices=[],
            failed_indices=[],
            start_time=time.time(),
            last_update_time=time.time(),
            quality_scores={}
        )
        
        self._current_checkpoint = checkpoint
        self._last_save_count = 0
        self._pending_completed = set()
        self._pending_failed = set()
        
        # 保存初始断点
        self._save_checkpoint(checkpoint)
        
        logger.info(f"创建断点: {task_id}, 总条目: {total_items}")
        return checkpoint
    
    def load_checkpoint(self, task_id: str) -> Optional[CheckpointData]:
        """加载断点
        
        Args:
            task_id: 任务 ID
        
        Returns:
            CheckpointData 实例，如果不存在则返回 None
        """
        checkpoint_path = self._get_checkpoint_path(task_id)
        
        if not checkpoint_path.exists():
            logger.info(f"断点不存在: {task_id}")
            return None
        
        try:
            with open(checkpoint_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
            
            # quality_scores 以整数索引为键，JSON 往返后会变成字符串，这里还原
            data['quality_scores'] = {
                int(k): v for k, v in data.get('quality_scores', {}).items()
            }
            
            # 尝试加载增量
            delta_path = self._get_delta_path(task_id)
            if delta_path.exists():
                delta = self._read_delta(delta_path)
                
                # 合并增量
                data['completed_indices'].extend(delta['completed'])
                data['failed_indices'].extend(delta['failed'])
                data['processed_items'] += len(delta['completed'])
                data['failed_items'] += len(delta['failed'])
                
                # 合并质量评分
                data['quality_scores'].update({
                    int(k): v for k, v in delta['quality_scores'].items()
                })
            
            checkpoint = CheckpointData(**data)
            self._current_checkpoint = checkpoint
            self._pending_completed = set()
            self._pending_failed = set()
            
            if delta_path.exists():
                # 合并结果立即落盘并清除增量，避免下次恢复重复累加
                self._save_checkpoint(checkpoint)
                self._remove_delta(task_id)
            
            logger.info(f"加载断点: {task_id}, 已处理: {checkpoint.processed_items}/{checkpoint.total_items}")
            return checkpoint
        except Exception as e:
            logger.error(f"加载断点失败: {e}")
            return None
    
    def _save_checkpoint(self, checkpoint: CheckpointData):
        """保存断点（紧凑格式）
        
        Args:
            checkpoint: 断点数据
        """
        checkpoint_path = self._get_checkpoint_path(checkpoint.task_id)
        checkpoint.last_update_time = time.time()
        
        try:
            # 使用紧凑 JSON（无缩进）
            data = asdict(checkpoint)
            with open(checkpoint_path, 'w', encoding='utf-8') as f:
                json.dump(data, f, ensure_ascii=False, separators=(',', ':'))
        except Exception as e:
            logger.error(f"保存断点失败: {e}")
    
    def _save_delta(self):
        """保存增量到临时文件

        增量文件是累加写入的：每次自动保存都会把新完成的索引并入已有增量，
        而不是覆盖，否则进程崩溃后只能恢复最后一批进度。
        """
        if self._current_checkpoint is None:
            return
        
        if not self._pending_completed and not self._pending_failed:
            return
        
        delta_path = self._get_delta_path(self._current_checkpoint.task_id)
        
        try:
            delta = self._read_delta(delta_path)
            delta['completed'].extend(sorted(self._pending_completed))
            delta['failed'].extend(sorted(self._pending_failed))
            delta['quality_scores'].update({
                str(k): self._current_checkpoint.quality_scores[k]
                for k in self._pending_completed
                if k in self._current_checkpoint.quality_scores
            })
            
            with open(delta_path, 'w', encoding='utf-8') as f:
                json.dump(delta, f, ensure_ascii=False, separators=(',', ':'))
            
            # 清空待保存集合
            self._pending_completed.clear()
            self._pending_failed.clear()
            
        except Exception as e:
            logger.error(f"保存增量失败: {e}")
    
    def _read_delta(self, delta_path: Path) -> Dict:
        """读取已有增量文件
        
        Args:
            delta_path: 增量文件路径
        
        Returns:
            增量字典，文件不存在或损坏时返回空增量
        """
        if not delta_path.exists():
            return {'completed': [], 'failed': [], 'quality_scores': {}}
        
        try:
            with open(delta_path, 'r', encoding='utf-8') as f:
                delta = json.load(f)
        except Exception as e:
            logger.warning(f"增量文件损坏，将重建: {delta_path}, {e}")
            return {'completed': [], 'failed': [], 'quality_scores': {}}
        
        return {
            'completed': list(delta.get('completed', [])),
            'failed': list(delta.get('failed', [])),
            'quality_scores': dict(delta.get('quality_scores', {}))
        }
    
    def _remove_delta(self, task_id: str):
        """删除增量文件（增量已并入主文件时调用）
        
        Args:
            task_id: 任务 ID
        """
        delta_path = self._get_delta_path(task_id)
        if delta_path.exists():
            delta_path.unlink()
    
    def update_progress(self,
                       index: int,
                       success: bool,
                       quality_score: Optional[float] = None):
        """更新进度
        
        Args:
            index: 当前处理的索引
            success: 是否成功
            quality_score: 质量评分
        """
        if self._current_checkpoint is None:
            return
        
        with self._lock:
            checkpoint = self._current_checkpoint
            
            if success:
                checkpoint.processed_items += 1
                checkpoint.completed_indices.append(index)
                self._pending_completed.add(index)
                if quality_score is not None:
                    checkpoint.quality_scores[index] = quality_score
            else:
                checkpoint.failed_items += 1
                checkpoint.failed_indices.append(index)
                self._pending_failed.add(index)
            
            # 定期保存增量
            if checkpoint.processed_items - self._last_save_count >= self.auto_save_interval:
                self._save_delta()
                self._last_save_count = checkpoint.processed_items
                logger.debug(f"保存增量: {checkpoint.processed_items}/{checkpoint.total_items}")
    
    def save_checkpoint(self):
        """手动保存断点（合并增量）"""
        with self._lock:
            if self._current_checkpoint:
                self._save_delta()
                self._save_checkpoint(self._current_checkpoint)
                # 增量已并入主文件，删除以免下次恢复时重复累加
                self._remove_delta(self._current_checkpoint.task_id)
                logger.info(f"手动保存断点: {self._current_checkpoint.task_id}")
    
    def get_progress(self) -> Dict:
        """获取进度信息
        
        Returns:
            进度信息字典
        """
        if self._current_checkpoint is None:
            return {"status": "no_checkpoint"}
        
        checkpoint = self._current_checkpoint
        elapsed_time = time.time() - checkpoint.start_time
        
        progress = checkpoint.processed_items / checkpoint.total_items if checkpoint.total_items > 0 else 0
        
        # 预计剩余时间
        if progress > 0:
            estimated_total = elapsed_time / progress
            remaining_time = estimated_total - elapsed_time
        else:
            remaining_time = 0
        
        return {
            "task_id": checkpoint.task_id,
            "total_items": checkpoint.total_items,
            "processed_items": checkpoint.processed_items,
            "failed_items": checkpoint.failed_items,
            "progress": progress,
            "progress_percent": f"{progress * 100:.1f}%",
            "elapsed_time": self._format_time(elapsed_time),
            "remaining_time": self._format_time(remaining_time),
            "start_time": datetime.fromtimestamp(checkpoint.start_time).isoformat(),
            "last_update": datetime.fromtimestamp(checkpoint.last_update_time).isoformat(),
            "avg_quality_score": self._calculate_avg_quality_score()
        }
    
    def _format_time(self, seconds: float) -> str:
        """格式化时间
        
        Args:
            seconds: 秒数
        
        Returns:
            格式化的时间字符串
        """
        if seconds < 60:
            return f"{seconds:.1f}秒"
        elif seconds < 3600:
            minutes = seconds / 60
            return f"{minutes:.1f}分钟"
        else:
            hours = seconds / 3600
            return f"{hours:.1f}小时"
    
    def _calculate_avg_quality_score(self) -> float:
        """计算平均质量评分
        
        Returns:
            平均质量评分
        """
        if self._current_checkpoint is None:
            return 0.0
        
        scores = list(self._current_checkpoint.quality_scores.values())
        return sum(scores) / len(scores) if scores else 0.0
    
    def get_remaining_indices(self) -> List[int]:
        """获取未处理的索引
        
        Returns:
            未处理的索引列表
        """
        if self._current_checkpoint is None:
            return []
        
        checkpoint = self._current_checkpoint
        all_indices = set(range(checkpoint.total_items))
        processed = set(checkpoint.completed_indices + checkpoint.failed_indices)
        
        return sorted(list(all_indices - processed))
    
    def delete_checkpoint(self, task_id: str):
        """删除断点
        
        Args:
            task_id: 任务 ID
        """
        checkpoint_path = self._get_checkpoint_path(task_id)
        
        if checkpoint_path.exists():
            checkpoint_path.unlink()
        self._remove_delta(task_id)
        
        logger.info(f"删除断点: {task_id}")
    
    def list_checkpoints(self) -> List[str]:
        """列出所有断点
        
        Returns:
            任务 ID 列表
        """
        checkpoints = []
        for file in self.checkpoint_dir.glob("*_checkpoint.json"):
            task_id = file.stem.replace("_checkpoint", "")
            checkpoints.append(task_id)
        return checkpoints
