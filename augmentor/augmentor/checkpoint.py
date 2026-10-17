# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""断点续传模块 - 优化版"""

import json
import logging
import time
import threading
from typing import List, Dict, Optional, Set
from dataclasses import dataclass, asdict
from pathlib import Path
from datetime import datetime

from augmentor.atomic_write import atomic_write_json
from augmentor.validation import require_within_dir

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



class _RunState:
    """一份断点**自己**的运行态：待落盘的增量与上一次自动保存时的已处理数。

    L275 之前这几格可变量住在 `CheckpointManager` 实例上，而
    `update_progress(index, success)` 不带任务身份 ⇒ 一次运行的每一笔进度写的都是
    「此刻 `_current_checkpoint` 指向的那一份」。改前实测（同一进程、同一个 manager，
    A=200 条与 B=50 条两次 `augment_dataset` 并发）：230 笔进度里 **170 笔落在对方的断点上**，
    于是 B 的 `completed_indices` 拿到 150 个越界下标（值域 0..199 而 `total_items=50`），
    A 自己那份在盘上停在 `processed_items=0`，另留一枚没人认领的增量文件。
    L273 的 `path_lock` 治的是「同一份文件别写坏」，治不到「这次更新属于哪个任务」。

    状态跟着断点对象走之后，归属由调用方手里的那份对象决定，与「谁最后加载」无关。
    它不是 dataclass 字段，所以 `asdict()` 的序列化面一字不变——盘上格式与既有断点文件
    全兼容（把它写成字段等于改盘上格式，旧文件立刻读不回来）。
    """

    __slots__ = ("pending_completed", "pending_failed", "last_save_count")

    def __init__(self):
        self.pending_completed = set()
        self.pending_failed = set()
        self.last_save_count = 0


# 挂在 CheckpointData 实例上的私有属性名
_RUN_STATE_ATTR = "_run_state"


def _state_of(checkpoint: "CheckpointData") -> _RunState:
    """取这份断点自己的运行态，第一次用时建一个挂在对象上

    Args:
        checkpoint: 断点数据对象

    Returns:
        该对象专属的 `_RunState`
    """
    state = getattr(checkpoint, _RUN_STATE_ATTR, None)
    if state is None:
        state = _RunState()
        setattr(checkpoint, _RUN_STATE_ATTR, state)
    return state



class CheckpointManager:
    """断点管理器 - 优化版"""
    
    def __init__(self, 
                 checkpoint_dir: str = "checkpoints",
                 auto_save_interval: int = 10):
        """初始化断点管理器
        
        Args:
            checkpoint_dir: 断点存储目录；**相对路径在构造这一刻就定基**（按构造时的当前目录
                解析成绝对路径），但**不建目录** —— 建目录推迟到第一次真正写断点/增量时。
                不定基的话落点会跟着写盘时的当前目录漂移：这里建好目录、之后任何一次
                `os.chdir` 都会让下一个断点写到别的目录去，于是同一个任务的两半进度分家。
                绝对路径原样保留，不做规范化。
            auto_save_interval: 自动保存间隔（每 N 条）
        """
        self.checkpoint_dir = Path(checkpoint_dir)
        if not self.checkpoint_dir.is_absolute():
            self.checkpoint_dir = self.checkpoint_dir.resolve()
        self.auto_save_interval = auto_save_interval
        
        self._current_checkpoint: Optional[CheckpointData] = None
        # 「还没有当前断点」时环境态的读写落点。产品路径拿不到它（`update_progress`
        # 在此之前就 return 了），它存在是为了让环境态任一刻都有对象可用，
        # 也让既有测试里 `manager._pending_completed` 这类直读直写继续成立。
        self._orphan_state = _RunState()
        self._lock = threading.Lock()
    
    @property
    def _active_state(self) -> _RunState:
        """环境态读法的落点：当前断点自己的那一份，没有当前断点时用孤儿态

        `_pending_completed` / `_pending_failed` / `_last_save_count` 三格的名字与写法
        一律保留，为的是**不改动任何既有读者**：`create_checkpoint` 与 `load_checkpoint`
        里那三行「换当前断点时把待保存集合清零」照原样成立，而它们清的是新那份对象
        自己的状态。带 `checkpoint` 参的路径不走这里，直接 `_state_of(ck)`。
        """
        if self._current_checkpoint is None:
            return self._orphan_state
        return _state_of(self._current_checkpoint)

    @property
    def _pending_completed(self) -> Set[int]:
        """当前断点的待保存完成索引（环境态代理，落点见 `_active_state`）"""
        return self._active_state.pending_completed

    @_pending_completed.setter
    def _pending_completed(self, value: Set[int]) -> None:
        self._active_state.pending_completed = set(value)

    @property
    def _pending_failed(self) -> Set[int]:
        """当前断点的待保存失败索引（环境态代理，落点见 `_active_state`）"""
        return self._active_state.pending_failed

    @_pending_failed.setter
    def _pending_failed(self, value: Set[int]) -> None:
        self._active_state.pending_failed = set(value)

    @property
    def _last_save_count(self) -> int:
        """当前断点上一次自动保存时的已处理数（环境态代理）"""
        return self._active_state.last_save_count

    @_last_save_count.setter
    def _last_save_count(self, value: int) -> None:
        self._active_state.last_save_count = value


    def _get_checkpoint_path(self, task_id: str) -> Path:
        """获取断点文件路径
        
        Args:
            task_id: 任务 ID
        
        Returns:
            断点文件路径
        """
        return require_within_dir("任务 ID", f"{task_id}_checkpoint.json", self.checkpoint_dir)
    
    def _get_delta_path(self, task_id: str) -> Path:
        """获取增量文件路径（append-only JSONL，一行为一批）

        Args:
            task_id: 任务 ID

        Returns:
            增量文件路径
        """
        return require_within_dir("任务 ID", f"{task_id}_delta.jsonl", self.checkpoint_dir)

    def _get_legacy_delta_path(self, task_id: str) -> Path:
        """旧格式增量文件路径：整份是**一个** JSON 文档的 `_delta.json`

        写侧不再产出它（A144：每次自动保存整读整写 ⇒ 总代价 O(N²/interval)），
        但读侧必须继续认，否则升级会把用户已有任务的中途进度判成不存在。
        """
        return require_within_dir("任务 ID", f"{task_id}_delta.json", self.checkpoint_dir)
    
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
            
            # 尝试加载增量（新旧两种格式一并合并）
            delta_files = self._delta_files(task_id)
            if delta_files:
                delta = {'completed': [], 'failed': [], 'quality_scores': {}}
                for delta_path in delta_files:
                    part = self._read_delta(delta_path)
                    delta['completed'].extend(part['completed'])
                    delta['failed'].extend(part['failed'])
                    delta['quality_scores'].update(part['quality_scores'])
                
                # 合并增量按「这一格记过没有」判，两侧都去重。两种重复各有来路：① 崩溃窗
                # （主文件写成之后、`_remove_delta` 之前被杀）让同一批增量下次再并一次；
                # ② `_delta_files` 把 legacy 与 JSONL 两份一并读回，同一下标在两份里各
                # 出现一次时纯 extend 就记两笔。实测①：6 条已完成读成 `processed_items=12`、
                # `progress_percent=200.0%` 并随合并落盘固化（`test_checkpoint_recovery_l100.py`
                # 只钉了写**失败**那一支，这两序当时都无守卫）。
                already_completed = set(data['completed_indices'])
                fresh_completed = [
                    idx for idx in dict.fromkeys(delta['completed']) if idx not in already_completed
                ]
                already_failed = set(data['failed_indices'])
                fresh_failed = [idx for idx in dict.fromkeys(delta['failed'])
                                if idx not in already_failed]

                data['completed_indices'].extend(fresh_completed)
                data['failed_indices'].extend(fresh_failed)
                data['processed_items'] += len(fresh_completed)
                data['failed_items'] += len(fresh_failed)
                
                # 合并质量评分
                data['quality_scores'].update({
                    int(k): v for k, v in delta['quality_scores'].items()
                })
            
            checkpoint = CheckpointData(**data)
            self._current_checkpoint = checkpoint
            self._pending_completed = set()
            self._pending_failed = set()
            
            if delta_files:
                # 合并结果立即落盘并清除增量，避免下次恢复重复累加。
                # 顺序不能反、也不能不看判决：主文件没写成功时删增量＝把新进度删没了。
                if self._save_checkpoint(checkpoint):
                    self._remove_delta(task_id)
            
            logger.info(f"加载断点: {task_id}, 已处理: {checkpoint.processed_items}/{checkpoint.total_items}")
            return checkpoint
        except Exception as e:
            logger.error(f"加载断点失败: {e}")
            return None
    
    def _save_checkpoint(self, checkpoint: CheckpointData) -> bool:
        """保存断点（紧凑格式，原子替换）

        返回判决而不是静默吞掉：调用方在写完之后会**删除增量文件**（`_remove_delta`），
        那是不可逆的一步。旧写法 `except: logger.error(...)` 之后照样往下删，于是「主文件
        没写成功」+「增量已删」＝ 那一批进度在盘上彻底消失，而且全程只有一行 error。
        现在两处删除都以本函数的返回值为条件，写失败时增量留在盘上，下次恢复照样合并。

        原子替换（`augmentor/atomic_write.py`）治的是另一半：旧写法 `open(path,'w')` 的
        写窗口里盘上是一份截断 JSON，而本模块的读侧 `load_checkpoint()` 把它连同其它异常
        一并 `except Exception → return None`，于是「一次坏读取」被翻译成「这个任务没有
        断点」—— 已完成的整段工作凭空消失。

        Args:
            checkpoint: 断点数据

        Returns:
            True 表示新内容已经落盘；False 表示写失败、目标文件仍是旧内容
        """
        checkpoint_path = self._get_checkpoint_path(checkpoint.task_id)
        checkpoint.last_update_time = time.time()
        
        try:
            # 使用紧凑 JSON（无缩进）
            data = asdict(checkpoint)
            atomic_write_json(checkpoint_path, data)
            return True
        except Exception as e:
            logger.error(f"保存断点失败: {e}")
            return False
    
    def _save_delta(self, checkpoint: Optional[CheckpointData] = None):
        """追加一批增量（A144：写侧 O(1)，不再整读整写）

        每次自动保存只把**新完成的那一批**写成一行，因此第 k 次保存的代价与已保存
        条数无关。以前是把已有增量整个读进内存、extend、再把全量序列化写回，
        于是总代价 O(N²/interval)，长任务里断点保存自己变成主要耗时。

        崩溃安全由读侧负责：一行写完一次 `write`，尾行若被截断只丢那一批，
        `_read_delta` 会跳过坏行而不是把整份增量作废。

        追加失败（目录不在、盘满、路径被占）时的口径是**不清空待保存集合**：那一批留在内存里，
        下一次保存连同新批次一起补写，代价是延迟落盘而不是丢失。因此调用方不必看返回值，但也
        不能把「本函数返回」读成「这一批已经在盘上」。

        Args:
            checkpoint: 这一批增量属于哪一份断点（决定落哪个 delta 文件、读哪份评分表）。
                `None` 沿用当前断点；并发运行的调用方要传自己那一份，见 `_RunState`。
        """
        ck = checkpoint if checkpoint is not None else self._current_checkpoint
        if ck is None:
            return

        state = _state_of(ck)
        if not state.pending_completed and not state.pending_failed:
            return

        delta_path = self._get_delta_path(ck.task_id)

        record = {
            'completed': sorted(state.pending_completed),
            'failed': sorted(state.pending_failed),
            'quality_scores': {
                str(k): ck.quality_scores[k]
                for k in state.pending_completed
                if k in ck.quality_scores
            },
        }

        try:
            delta_path.parent.mkdir(parents=True, exist_ok=True)
            with open(delta_path, 'a', encoding='utf-8') as f:
                f.write(json.dumps(record, ensure_ascii=False, separators=(',', ':')) + "\n")

            # 清空待保存集合
            state.pending_completed.clear()
            state.pending_failed.clear()

        except Exception as e:
            logger.error(f"保存增量失败: {e}")


    def _read_delta(self, delta_path: Path) -> Dict:
        """读取增量文件（JSONL 逐行合并；旧格式的单个 JSON 文档同样认）

        Args:
            delta_path: 增量文件路径

        Returns:
            增量字典，文件不存在或读不出时返回空增量
        """
        empty = {'completed': [], 'failed': [], 'quality_scores': {}}
        if not delta_path.exists():
            return empty
        
        try:
            text = delta_path.read_text(encoding='utf-8')
        except Exception as e:
            logger.warning(f"增量文件读不出，将重建: {delta_path}, {e}")
            return empty
        
        records: List[Dict] = []
        try:
            whole = json.loads(text)
        except Exception:
            whole = None
        if isinstance(whole, dict):
            # 旧格式（或只写了一行的新格式）：整份就是一个批次记录，形状与新格式逐键相同
            records.append(whole)
        else:
            for line in text.splitlines():
                line = line.strip()
                if not line:
                    continue
                try:
                    record = json.loads(line)
                except Exception as e:
                    # 崩溃时最可能截断的就是尾行：跳它，不跳整份
                    logger.warning(f"跳过增量坏行: {delta_path}, {e}")
                    continue
                if isinstance(record, dict):
                    records.append(record)
                else:
                    logger.warning(f"跳过增量坏行（不是 JSON 对象）: {delta_path}")
        
        delta = {'completed': [], 'failed': [], 'quality_scores': {}}
        for record in records:
            delta['completed'].extend(record.get('completed', []))
            delta['failed'].extend(record.get('failed', []))
            delta['quality_scores'].update(record.get('quality_scores', {}))
        if not records and text.strip():
            logger.warning(f"增量文件无可用记录: {delta_path}")
        return delta
    
    def _delta_files(self, task_id: str) -> List[Path]:
        """本次恢复要合并的增量文件，按时间顺序（旧格式在前，新格式在后）"""
        return [
            path for path in (self._get_legacy_delta_path(task_id),
                              self._get_delta_path(task_id))
            if path.exists()
        ]
    
    def _remove_delta(self, task_id: str):
        """删除增量文件（增量已并入主文件时调用，新旧两种格式一并清掉）
        
        Args:
            task_id: 任务 ID
        """
        for delta_path in (self._get_delta_path(task_id),
                           self._get_legacy_delta_path(task_id)):
            # 增量已并入主文件，缺档即目标达成：exists 预检挡不住并发删除里裸 unlink 抛的 FileNotFoundError。
            delta_path.unlink(missing_ok=True)
    
    def update_progress(self,
                       index: int,
                       success: bool,
                       quality_score: Optional[float] = None,
                       checkpoint: Optional[CheckpointData] = None):
        """更新进度

        Args:
            index: 当前处理的索引
            success: 是否成功
            quality_score: 质量评分
            checkpoint: 这一笔进度属于哪一份断点。默认 `None` 是环境态读法（写进
                `_current_checkpoint` 当下指的那一份）；并发运行的调用方**必须**传自己
                手里那份，否则就是 L275 量到的那一形——A 的 200 笔进度落进 B 的断点文件，
                B 的续传集合从此多出 150 个越界下标，而 A 在盘上一条都没做过。
        """
        ck = checkpoint if checkpoint is not None else self._current_checkpoint
        if ck is None:
            return

        with self._lock:
            checkpoint = ck
            state = _state_of(checkpoint)

            if success:
                checkpoint.processed_items += 1
                checkpoint.completed_indices.append(index)
                state.pending_completed.add(index)
                if quality_score is not None:
                    checkpoint.quality_scores[index] = quality_score
            else:
                checkpoint.failed_items += 1
                checkpoint.failed_indices.append(index)
                state.pending_failed.add(index)

            # 定期保存增量
            if checkpoint.processed_items - state.last_save_count >= self.auto_save_interval:
                self._save_delta(checkpoint)
                state.last_save_count = checkpoint.processed_items
                logger.debug(f"保存增量: {checkpoint.processed_items}/{checkpoint.total_items}")


    def save_checkpoint(self, checkpoint: Optional[CheckpointData] = None):
        """手动保存断点（合并增量）

        Args:
            checkpoint: 要收尾的那一份断点；`None` 沿用当前断点
        """
        with self._lock:
            ck = checkpoint if checkpoint is not None else self._current_checkpoint
            if ck:
                self._save_delta(ck)
                if self._save_checkpoint(ck):
                    # 增量已并入主文件，删除以免下次恢复时重复累加；主文件没写成就不删
                    self._remove_delta(ck.task_id)
                    logger.info(f"手动保存断点: {ck.task_id}")


    def get_progress(self, checkpoint: Optional[CheckpointData] = None) -> Dict:
        """获取进度信息

        Args:
            checkpoint: 读哪一份断点。`None` 是环境态读法（当下 `_current_checkpoint`
                指谁就读谁）——`GET /api/augment/progress` 走的正是这一支。
                一次运行自己要进度应传手里那份，否则并发下读到的是别人的数（L275）。

        Returns:
            进度信息字典
        """
        ck = checkpoint if checkpoint is not None else self._current_checkpoint
        if ck is None:
            return {"status": "no_checkpoint"}
        
        elapsed_time = time.time() - ck.start_time
        
        progress = ck.processed_items / ck.total_items if ck.total_items > 0 else 0
        
        # 预计剩余时间
        if progress > 0:
            estimated_total = elapsed_time / progress
            remaining_time = estimated_total - elapsed_time
        else:
            remaining_time = 0
        
        return {
            "task_id": ck.task_id,
            "total_items": ck.total_items,
            "processed_items": ck.processed_items,
            "failed_items": ck.failed_items,
            "progress": progress,
            "progress_percent": f"{progress * 100:.1f}%",
            "elapsed_time": self._format_time(elapsed_time),
            "remaining_time": self._format_time(remaining_time),
            "start_time": datetime.fromtimestamp(ck.start_time).isoformat(),
            "last_update": datetime.fromtimestamp(ck.last_update_time).isoformat(),
            "avg_quality_score": self._calculate_avg_quality_score(ck)
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
    
    def _calculate_avg_quality_score(self,
                                    checkpoint: Optional[CheckpointData] = None) -> float:
        """计算平均质量评分

        Args:
            checkpoint: 算哪一份断点的评分表；`None` 沿用当前断点

        Returns:
            平均质量评分
        """
        ck = checkpoint if checkpoint is not None else self._current_checkpoint
        if ck is None:
            return 0.0
        
        scores = list(ck.quality_scores.values())
        return sum(scores) / len(scores) if scores else 0.0
    
    def get_remaining_indices(self,
                             checkpoint: Optional[CheckpointData] = None) -> List[int]:
        """获取未处理的索引

        Args:
            checkpoint: 要问的那一份断点；`None` 沿用当前断点（并发运行里传自己那份，
                否则问到的可能是别人的总条目数）

        Returns:
            未处理的索引列表
        """
        ck = checkpoint if checkpoint is not None else self._current_checkpoint
        if ck is None:
            return []

        checkpoint = ck
        all_indices = set(range(checkpoint.total_items))
        processed = set(checkpoint.completed_indices + checkpoint.failed_indices)

        return sorted(list(all_indices - processed))


    def delete_checkpoint(self, task_id: str):
        """删除断点
        
        Args:
            task_id: 任务 ID
        """
        checkpoint_path = self._get_checkpoint_path(task_id)
        
        checkpoint_path.unlink(missing_ok=True)
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
