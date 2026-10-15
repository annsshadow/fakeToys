# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""AI 训练数据增强主流程 - 优化版"""

import json
import logging
import os
import time
import hashlib
import threading
from typing import List, Dict, Optional
from pathlib import Path

from .atomic_write import atomic_write_json
import asyncio
from concurrent.futures import ThreadPoolExecutor, as_completed
from queue import Empty, Queue

from .config import (REQUEST_TIMEOUT_RANGE, AppConfig, load_config,
                     get_model_config)
from .validation import require_count, require_ratio, require_seconds, require_within_dir
from .retry import MAX_RETRY_AFTER
from .models import create_model_backend, extract_json_array
from .quality import QualityScorer
from .dedup import Deduplicator
from .export import Exporter, ExportFormat
from .context import ContextAugmentor
from .checkpoint import CheckpointManager
from .memory_monitor import MemoryMonitor, get_memory_summary
from .versioning import VersionManager
from .sampler import ActiveSampler
from .expander import DomainExpander
from .tracker import ExperimentTracker
from .visualizer import DataVisualizer
from .semantic import persist_index

logger = logging.getLogger(__name__)

# A365：并行增强里「这一笔的结果」的有界等待秒数。消费侧等到这一步时对应的工作
# 函数已经结束（`as_completed` 只交付完成的 future），所以信箱要么有货要么永远
# 不会有货——这一格给的是「永远不会有货」那一档的上界，不是单条处理的耗时预算。
RESULT_WAIT_SECONDS = 60


class AugmentorPipeline:
    """AI 训练数据增强管道 - 优化版"""
    
    def __init__(self,
                 config: Optional[AppConfig] = None,
                 config_path: Optional[str] = None,
                 response_cache_dir: Optional[str] = None,
                 response_cache_ttl: Optional[float] = None,
                 response_cache_max_bytes: Optional[int] = None):
        """初始化增强管道

        Args:
            config: 应用配置，为 None 时从文件加载
            config_path: 配置文件路径
            response_cache_dir: 模型响应磁盘缓存目录。**默认 None 即不启用**——
                启用意味着把 prompt 与模型响应写入磁盘，属于需要显式选择的行为
            response_cache_ttl: 磁盘缓存生存时间（秒），None 表示不按时间过期
            response_cache_max_bytes: 磁盘缓存容量上限（字节）
        """
        self.config = config or load_config(config_path)
        self._lock = threading.Lock()
        # L206：失败明细的出口。改前它是失败分支里惰性首建的，而**全仓零读者**
        # ——整批里哪些条目失败了、为什么失败，除了 logger.error 之外没有任何
        # 可编程出口，而那段注释明明写着「以便后续分析和重试」。
        self._process_errors: List[Dict] = []
        self._response_cache_dir = response_cache_dir
        self._response_cache_ttl = response_cache_ttl
        self._response_cache_max_bytes = response_cache_max_bytes

        # 初始化组件
        self._init_components()
    
    def _init_components(self):
        """初始化各组件"""
        # 重试与超时旋钮先判后建：下面那个 try/except 的既定语义是「模型没配好就降级」，
        # 配置文件里写坏的 max_retries / retry_delay / max_retry_wait / retry_jitter /
        # request_timeout 若让它去抛，症状会被读成
        # 「后端不可用」，而真实原因是参数越界——所以判据必须在 try 之外。
        require_count("augmentation.max_retries", self.config.augmentation.max_retries)
        require_seconds("augmentation.retry_delay", self.config.augmentation.retry_delay)
        require_seconds("augmentation.max_retry_wait",
                        self.config.augmentation.max_retry_wait,
                        minimum=0.0, maximum=MAX_RETRY_AFTER)
        require_ratio("augmentation.retry_jitter",
                      self.config.augmentation.retry_jitter)
        lo, hi = REQUEST_TIMEOUT_RANGE
        require_seconds("augmentation.request_timeout",
                        self.config.augmentation.request_timeout,
                        minimum=lo, maximum=hi)

        # 模型后端（可选：未配置模型/密钥时降级，避免阻断断点续传等只读能力）
        try:
            model_config = get_model_config(self.config, self.config.default_model)
            self.model_backend = create_model_backend(
                model_config,
                response_cache_dir=self._response_cache_dir,
                response_cache_ttl=self._response_cache_ttl,
                response_cache_max_bytes=self._response_cache_max_bytes,
                default_attempts=self.config.augmentation.max_retries,
                default_retry_delay=self.config.augmentation.retry_delay,
                default_max_retry_wait=self.config.augmentation.max_retry_wait,
                default_retry_jitter=self.config.augmentation.retry_jitter,
                default_request_timeout=self.config.augmentation.request_timeout,
            )
        except Exception as exc:
            logger.warning("模型后端初始化失败，增强功能将不可用：%s", exc)
            self.model_backend = None
        
        # 质量评分器
        self.quality_scorer = QualityScorer(
            threshold=self.config.quality.threshold,
            weights=self.config.quality.weights
        )
        
        # 去重器
        self.deduplicator = Deduplicator(
            threshold=self.config.dedup.threshold
        )
        
        # 导出器
        self.exporter = Exporter(
            default_format=self.config.export.default_format
        )
        
        # 对话上下文增强器
        self.context_augmentor = ContextAugmentor(
            model_backend=self.model_backend,
            num_turns=self.config.context.num_turns
        ) if self.model_backend else None
        
        # 断点管理器
        self.checkpoint_manager = CheckpointManager(
            checkpoint_dir="checkpoints",
            auto_save_interval=self.config.augmentation.auto_save_interval
        )
        
        # 版本管理器
        self.version_manager = VersionManager(
            storage_dir=self.config.versioning.storage_dir
        )
        
        # 主动学习选样器
        self.sampler = ActiveSampler()
        
        # 领域扩展器
        self.expander = DomainExpander(self.model_backend) if self.model_backend else None
        
        # 实验追踪器
        self.tracker = ExperimentTracker()
        
        # 可视化器
        self.visualizer = DataVisualizer()
    
    @staticmethod
    def build_task_id(input_file: str) -> str:
        """基于输入文件生成稳定的任务 ID
        
        任务 ID 必须跨进程稳定，断点续传才能识别出「同一次任务」。
        使用时间戳会导致每次运行都是新任务，从而永远无法恢复。
        
        Args:
            input_file: 输入文件路径
        
        Returns:
            任务 ID
        """
        resolved = str(Path(input_file).resolve())
        digest = hashlib.md5(resolved.encode('utf-8')).hexdigest()[:12]
        return f"augment_{digest}"
    
    def _generate_variants(self, item: Dict) -> List[Dict]:
        """为单条数据生成变体
        
        Args:
            item: 原始数据
        
        Returns:
            生成的变体列表
        """
        if self.model_backend is None:
            return []
        
        instruction = item.get("instruction", "")
        output = item.get("output", "")

        # 原 JSON 用 json.dumps 生成而不是手拼字符串字面量：instruction 里带引号或
        # 换行时，手拼的 `"instruction": "{instruction}"` 会把发给模型的 prompt 里的
        # JSON 结构撑破（A109）。
        seed_json = json.dumps([{"instruction": instruction}], ensure_ascii=False)
        prompt = f"""作为AI领域数据增强专家，你的任务是把原JSON对象中的instruction字段内容在保持相同含义的情况下，扩充为{self.config.augmentation.variants_per_seed}种不同的提问方式，新JSON对象只需包含instruction字段，并将新JSON对象追加到JSON数组里面，注意最终结果只需要JSON数组，不需要任何说明文字，原JSON对象如下：{seed_json}"""

        try:
            response = self.model_backend.generate(prompt)
            # 复用仓库里已带自测的 robust 提取器（直接解析 / 代码围栏 / 首个 '[' 到
            # 末个 ']' 的括号切片），而不是「首个字符必须是 [」的手搓判据 —— 后者会让
            # ```json 围栏或前置一句说明的正常响应静默变成 0 个变体（A108）。
            # extract_json_array 是流水线唯一正确的解析器：这里的产物恒为 JSON 数组，
            # 而它无法解析时抛 ModelResponseError，由下面的 except 收成一行 warning。
            variants = extract_json_array(response)
            result = []
            for v in variants[:self.config.augmentation.variants_per_seed]:
                if isinstance(v, dict) and "instruction" in v:
                    result.append({
                        "instruction": v["instruction"],
                        "input": "",
                        "output": output
                    })
            return result
        except Exception as e:
            logger.warning(f"生成变体失败: {e}")

        return []
    
    def _process_single_item(self, 
                            idx: int, 
                            item: Dict, 
                            use_quality_check: bool,
                            result_queue: Queue,
                            errors: Optional[List[Dict]] = None):
        """处理单条数据（线程安全）
        
        Args:
            idx: 索引
            item: 数据项
            use_quality_check: 是否使用质量检查
            result_queue: 本笔结果的信箱（A365 后由 `augment_dataset` 按条目各建一只；
                直接调用方仍可传任意 `Queue`，本方法只往里 `put` 一次）
            errors: 失败明细的落点。`augment_dataset` 传的是**本批局部清单**（L275）；
                `None` 留给直接调用本方的旧调用方，那时仍落实例属性
        """
        try:
            variants = self._generate_variants(item)
            
            # 质量检查
            if use_quality_check and variants:
                items_to_score = []
                for v in variants:
                    items_to_score.append({
                        "original": item.get("instruction", ""),
                        "generated": v.get("instruction", ""),
                        "output": v.get("output", "")
                    })
                
                # 简化的质量检查（不使用已有生成文本，避免锁竞争）
                filtered = []
                for score_item in items_to_score:
                    score = self.quality_scorer.score(
                        score_item["original"],
                        score_item["generated"],
                        score_item["output"]
                    )
                    if score.passed:
                        filtered.append({
                            "instruction": score_item["generated"],
                            "input": "",
                            "output": score_item["output"]
                        })
                variants = filtered
            
            result_queue.put((idx, True, variants))
            
        except Exception as e:
            logger.error(f"处理第 {idx} 条数据失败: {e}")
            # 错误恢复增强：记录详细错误信息以便后续分析和重试。
            # 本方法会由 ThreadPoolExecutor 里多个工作线程同时调用（augment_dataset
            # 的并行路径），而 `_process_errors` 是惰性首用才建的共享 list——
            # 「没有就建」+「append」两步不是原子的，须持锁包住，否则并发下会
            # 重复建 list 丢掉错误记录。__init__ 里的 self._lock 就是为此留的。
            error_info = {
                "index": idx,
                "error_type": type(e).__name__,
                "message": str(e),
                "item_preview": str(item)[:100] if item else ""
            }
            with self._lock:
                # L206：列表在 __init__ 就建好，这里只 append（仍持锁：
                # 多工作线程并发 append 到同一个 list）。
                # L275：落点优先是 `augment_dataset` 传下来的本批清单。改前两个任务
                # 共用这一个 pipeline 实例时都往 `self._process_errors` 写，实测 A 的
                # 报告里 139 笔明细**全部**来自对方。不传才落实例属性。
                (errors if errors is not None else self._process_errors).append(error_info)
            result_queue.put((idx, False, []))
    
    def augment_dataset(self,
                       input_file: str,
                       output_file: str,
                       use_checkpoint: bool = True,
                       use_quality_check: Optional[bool] = None,
                       use_dedup: Optional[bool] = None,
                       use_parallel: bool = True) -> Dict:
        """增强整个数据集 - 优化版
        
        Args:
            input_file: 输入文件路径
            output_file: 输出文件路径
            use_checkpoint: 是否使用断点续传；任务 ID 由输入文件路径派生，
                因此对同一输入文件重复运行会跳过已完成的条目并复用已有输出。
                如需从头重跑，先调用 checkpoint_manager.delete_checkpoint(task_id)
            use_quality_check: 是否使用质量检查。`None`（默认）= 调用方没点，
                交给 `config.quality.enabled`；显式 True/False = 调用方点了，配置让位
            use_dedup: 是否使用去重，同上，交给 `config.dedup.enabled`
            use_parallel: 是否使用并行处理
        
        Returns:
            处理报告。含 `errors` 键（L206）：本批失败条目的明细清单
            （下标 / 异常类型 / 消息 / 条目预览）；无失败时是空列表。
            报告里的 `quality_check` / `dedup` 是**解析后的**布尔值，不是 `None`
            （L215：加了配置臂之后，报告若回显 `None` 就等于说「这一阶段没定」，
            而实际是定了的——由配置定的）。
        """
        # L215（A138/B5 的 enabled 族第一批：出厂四只 true 键里落在本阶段的三只）：
        # 权威序 调用参数 > 配置键 > 内置默认。`None` 是「调用方没点」的哨兵，
        # 不是「关掉」——所以这两只键接上之后，随包配置（两只都 true）下行为零漂移。
        if use_quality_check is None:
            use_quality_check = self.config.quality.enabled
        if use_dedup is None:
            use_dedup = self.config.dedup.enabled

        # L206：每次运行前清空。改前不清 ⇒ 同一个实例跑第二批时，第一批的
        # 错误记录还留在里面，报告于是混进与本次无关的错。
        # L275：本批明细住在局部清单里，`self._process_errors` 只留作直接调用方
        # （不传 `errors=` 的旧写法）的默认出口。改前「同时跑两批」时这一格是共享的：
        # B 的 start 会把 A 已记录的错抹掉，A 的报告反过来全是 B 的错。
        errors: List[Dict] = []
        ckpt = None
        with self._lock:
            self._process_errors = []

        # 加载输入数据
        with open(input_file, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        logger.info(f"加载 {len(items)} 条种子数据")
        
        # 生成任务 ID（基于输入文件，保证跨进程稳定以便续传）
        task_id = self.build_task_id(input_file)
        
        # 创建或恢复断点
        # L275：把**本运行自己那一份**断点对象握在手里，之后每一笔进度与收尾都显式
        # 交给它。改前这些调用不带断点，落到 `CheckpointManager._current_checkpoint`
        # 这一格共享环境态上 ⇒ 两个任务同时在跑时，后加载的一方决定所有进度写进谁的
        # 文件（实测 230 笔里 170 笔写错对象）。
        if use_checkpoint:
            ckpt = self.checkpoint_manager.load_checkpoint(task_id)
            if ckpt is None:
                ckpt = self.checkpoint_manager.create_checkpoint(task_id, len(items))
            remaining_indices = self.checkpoint_manager.get_remaining_indices(ckpt)
            skipped = len(items) - len(remaining_indices)
            if skipped > 0:
                logger.info(f"检测到断点，跳过 {skipped} 条已完成数据")
        else:
            remaining_indices = list(range(len(items)))
        
        # 初始化内存监控（集成到增强流程）
        memory_monitor = MemoryMonitor(interval_mb=50)
        memory_monitor.take_snapshot()
        
        # 处理数据
        all_results = []
        existing_generated = []
        max_existing_window = 500  # 滑动窗口大小，限制内存使用
        
        # 续传时载入上一轮已生成的输出，否则会把已完成的结果覆盖为空
        if use_checkpoint and skipped > 0:
            output_path = Path(output_file)
            if output_path.exists():
                with open(output_path, 'r', encoding='utf-8') as f:
                    all_results = json.load(f)
                logger.info(f"载入上一轮已生成的 {len(all_results)} 条数据")
            else:
                logger.warning(f"断点显示已处理 {skipped} 条，但输出文件不存在，仅能保留本轮结果")
        
        if use_parallel and len(remaining_indices) > 1:
            # 优化线程池：根据任务规模动态调整线程数，避免过度并发
            available_cores = max(1, (os.cpu_count() or 1) - 1)
            num_threads = min(
                self.config.augmentation.num_threads,
                len(remaining_indices),
                available_cores * 2  # 限制为核心数的2倍，避免过度上下文切换
            )
            logger.info(f"优化线程池: 使用 {num_threads} 个线程 (核心: {available_cores}, 任务: {len(remaining_indices)})")
            
            # A365 修法①（用户拍板「条目失败继续」）：结果按**这一笔**配对。
            # 改前一个共享队列配一条不带超时的 `get()`，三格同族缺陷：
            # ① 任一 worker 少投一次 ⇒ 整批零 CPU 永久挂死（L275 门禁实测停在 62%）；
            # ② `future` 的返回值一次也不读 ⇒ worker 里逃出 `_process_single_item`
            #    的异常无声吞掉，日志里连那笔 `TypeError` 都没有；
            # ③ 配对纯靠顺序（`futures[future] = idx` 建了从不读）⇒ worker 多投一条时
            #    进度与变体记到别的条目名下。
            # 每条目一只信箱之后，「这一笔的结果」与「下一条结果」不再可能是两回事。
            mailboxes = {idx: Queue() for idx in remaining_indices}
            
            with ThreadPoolExecutor(max_workers=num_threads) as executor:
                # 提交所有任务
                futures = {}
                for idx in remaining_indices:
                    item = items[idx]
                    future = executor.submit(
                        self._process_single_item,
                        idx, item, use_quality_check, mailboxes[idx], errors
                    )
                    futures[future] = idx
                
                # 收集结果
                processed = 0
                for future in as_completed(futures):
                    idx = futures[future]
                    # ②出声：worker 里逃出本方法的异常不再被吞掉（本方法自己 catch
                    # 不到的那一格，例如 except 支内部再抛），此处读到就记进日志。
                    escaped = future.exception()
                    if escaped is not None:
                        logger.error(f"第 {idx} 条的工作线程逃出异常: {escaped}")
                    
                    try:
                        # 走到这里说明这一笔的工作函数已经返回或抛出 ⇒ 信箱里要么
                        # 已经有货，要么永远不会有货。有界等待取代无限等待（①）。
                        _, success, variants = mailboxes[idx].get(
                            timeout=RESULT_WAIT_SECONDS
                        )
                    except Empty:
                        # ③少投那一笔算**条目失败并继续**，整批不再挂死。
                        errors.append({
                            "index": idx,
                            "error_type": "ResultNotDelivered",
                            "message": (f"工作线程结束但未投递本笔结果"
                                        f"（等待超过 {RESULT_WAIT_SECONDS}s）"),
                            "item_preview": str(items[idx])[:100] if items[idx] else ""
                        })
                        success, variants = False, []
                    
                    if success:
                        all_results.extend(variants)
                        existing_generated.extend([v.get("instruction", "") for v in variants])
                        # 滑动窗口，限制内存使用
                        if len(existing_generated) > max_existing_window:
                            existing_generated = existing_generated[-max_existing_window:]
                        
                        if use_checkpoint:
                            self.checkpoint_manager.update_progress(idx, True,
                                                                    checkpoint=ckpt)
                        
                        processed += 1
                        if processed % 10 == 0:
                            logger.info(f"已处理 {processed}/{len(remaining_indices)}")
                    else:
                        if use_checkpoint:
                            self.checkpoint_manager.update_progress(idx, False,
                                                                    checkpoint=ckpt)
        else:
            # 串行处理
            for idx in remaining_indices:
                item = items[idx]
                logger.info(f"处理 {idx + 1}/{len(items)}: {item.get('instruction', '')[:50]}...")
                
                try:
                    variants = self.augment_seed(
                        item,
                        use_quality_check=use_quality_check,
                        existing_generated=existing_generated
                    )
                    
                    all_results.extend(variants)
                    existing_generated.extend([v.get("instruction", "") for v in variants])
                    # 滑动窗口，限制内存使用
                    if len(existing_generated) > max_existing_window:
                        existing_generated = existing_generated[-max_existing_window:]
                    
                    # 更新断点
                    if use_checkpoint:
                        self.checkpoint_manager.update_progress(idx, True,
                                                                checkpoint=ckpt)
                    
                    logger.info(f"生成 {len(variants)} 个变体")
                    
                except Exception as e:
                    logger.error(f"处理失败: {e}")
                    # L206：串行支路此前只打日志、**不记 `_process_errors`** ⇒
                    # `use_parallel=False`（或只剩一条数据）时 `errors` 恒为空，
                    # 「失败明细有出口」这件事只在并行档成立。两支出同一份明细。
                    # L275 之后这一支不持锁：写的是本运行自己的 `errors`，而一次运行
                    # 只走串行或并行其中一支，串行这一支只有调用方那一个线程在 append。
                    errors.append({
                        "index": idx,
                        "error_type": type(e).__name__,
                        "message": str(e),
                        "item_preview": str(item)[:100] if item else "",
                    })
                    if use_checkpoint:
                        self.checkpoint_manager.update_progress(idx, False,
                                                                checkpoint=ckpt)
        
        # 合并增量并落盘，确保中断后可恢复
        if use_checkpoint:
            self.checkpoint_manager.save_checkpoint(ckpt)
        
        # 去重
        if use_dedup and all_results:
            logger.info(f"去重前: {len(all_results)} 条")
            all_results = self.deduplicator.deduplicate_and_filter(all_results)
            logger.info(f"去重后: {len(all_results)} 条")
        
        # 保存输出
        # L168 收原子写：本方法是管线的最终交付落盘口（API 链落数据根、
        # list_data_files 扫描读者在写窗口内拿半份）
        atomic_write_json(output_file, all_results)
        
        logger.info(f"保存 {len(all_results)} 条数据到 {output_file}")
        
        # 记录最终内存状态（集成监控）
        memory_monitor.take_snapshot()
        memory_summary = get_memory_summary()
        logger.info(f"增强流程内存监控完成，峰值: {memory_monitor.get_peak_usage_mb():.2f} MB，趋势: {memory_monitor.get_trend()}")
        
        # 创建版本快照
        # L215（enabled 族）：`versioning.enabled` 是这一族的总闸，`auto_snapshot`
        # 是「增强结束时自动打点」这一路的闸，两只串联而不是互相覆盖——总闸关掉之后
        # 仍然自动打点，等于用户说「版本管理别跑」而代码回答「但自动打点你没关」。
        if self.config.versioning.enabled and self.config.versioning.auto_snapshot:
            self.version_manager.create_version(
                all_results,
                label=f"增强数据_{int(time.time())}",
                description=f"从 {input_file} 增强生成"
            )

        # L216（vector 族）：`vector.enabled` 是「增强结束时自动建语义索引并落盘」的闸。
        # 出厂值是 `false` ⇒ 默认完全不建，与本轮之前的行为一字不差；建与不建的差别
        # 只在 `data/vectors` 里有没有那两份 `{collection}_vectors.npy` / `_meta.json`。
        vector_index = None
        if self.config.vector.enabled:
            vector_index = self.build_vector_index(all_results)

        # 返回报告
        return {
            "task_id": task_id,
            "input_file": input_file,
            "output_file": output_file,
            "input_count": len(items),
            "output_count": len(all_results),
            "quality_check": use_quality_check,
            "dedup": use_dedup,
            "parallel": use_parallel,
            # L216：语义索引的落点与规模；`None` 是「配置没让建」，不是「建了但空」。
            "vector_index": vector_index,
            # L206：失败明细出得来了。放在 progress 之前——报告形键集中在
            # 一起，`progress` / `memory_monitor` 是运行期诊断，错误是结果。
            # L275：取本批局部清单，不再取共享的实例属性。
            "errors": list(errors),
            "progress": self.checkpoint_manager.get_progress(ckpt) if use_checkpoint else None,
            "memory_monitor": {
                "peak_usage_mb": memory_monitor.get_peak_usage_mb(),
                "trend": memory_monitor.get_trend(),
                "final_usage_mb": memory_summary.get("current_usage_mb", 0.0),
                "monitor_available": memory_summary.get("monitor_available", False)
            }
        }
    
    def build_vector_index(self, items: List[Dict],
                           text_key: str = "instruction") -> Dict:
        """按 `config.vector.*` 的生效值建语义索引并落盘（L216）

        这是 `vector` 节五只键在增强侧的消费点：`backend` 选后端类、`dimension` 定
        嵌入宽度、`collection` 定文件名与 ID 前缀、`storage_dir` 定落盘目录。
        返回体就是这些生效值的回执，调用方（报告 / 状态端点）据此可核对「配了什么、建了什么」。

        Args:
            items: 待入库的数据条目
            text_key: 参与嵌入的字段名

        Returns:
            `persist_index` 的回执：`count` / `backend` / `dimension` / `collection` / `storage_dir`
        """
        vector = self.config.vector
        return persist_index(
            items,
            backend=vector.backend,
            dimension=vector.dimension,
            collection=vector.collection,
            storage_dir=vector.storage_dir,
            text_key=text_key
        )

    def augment_seed(self, 
                    seed_item: Dict,
                    use_quality_check: bool = True,
                    existing_generated: Optional[List[str]] = None) -> List[Dict]:
        """增强单条种子数据
        
        Args:
            seed_item: 种子数据
            use_quality_check: 是否使用质量检查
            existing_generated: 已生成的文本列表
        
        Returns:
            增强后的数据列表
        """
        variants = self._generate_variants(seed_item)
        
        if use_quality_check and variants:
            # 添加原始数据用于质量评估
            items_to_score = []
            for v in variants:
                items_to_score.append({
                    "original": seed_item.get("instruction", ""),
                    "generated": v.get("instruction", ""),
                    "output": v.get("output", "")
                })
            
            # 过滤低质量样本
            filtered = self.quality_scorer.filter_by_quality(
                items_to_score,
                existing_generated
            )
            
            # 重建数据格式
            result = []
            for item in filtered:
                result.append({
                    "instruction": item["generated"],
                    "input": "",
                    "output": item["output"]
                })
            return result
        
        return variants
    
    def export_dataset(self,
                      input_file: str,
                      output_dir: str,
                      formats: Optional[List[str]] = None) -> Dict[str, str]:
        """导出数据集为多种格式
        
        Args:
            input_file: 输入文件路径
            output_dir: 输出目录
            formats: 导出格式列表，元素可传字符串（如 "jsonl"）或 `ExportFormat` 成员
        
        Returns:
            格式到文件路径的映射
        """
        # 加载数据
        with open(input_file, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        if formats:
            results = {}
            for fmt in formats:
                # 归一化为字符串：ExportFormat 成员直接进 f-string 会渲染成
                # "ExportFormat.OPENAI"，把枚举类名写进文件名与返回值的键。
                name = fmt.value if isinstance(fmt, ExportFormat) else fmt
                output_path = require_within_dir("导出格式名", f"train_data_{name}.json", Path(output_dir))
                self.exporter.export(items, str(output_path), name)
                results[name] = str(output_path)
            return results
        else:
            return self.exporter.export_all_formats(items, output_dir)
    
    def analyze_dataset(self, input_file: str) -> Dict:
        """分析数据集
        
        Args:
            input_file: 输入文件路径
        
        Returns:
            分析报告
        """
        with open(input_file, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        # 覆盖分析
        coverage = self.sampler.analyze_coverage(items)
        
        # 统计信息
        stats = self.visualizer.generate_statistics(items)
        
        # 去重报告
        dedup_report = self.deduplicator.generate_report(items)
        
        return {
            "coverage_analysis": coverage,
            "statistics": stats,
            "dedup_report": dedup_report
        }
    
    def visualize_dataset(self,
                         input_file: str,
                         output_dir: str = "visualizations") -> Dict[str, str]:
        """可视化数据集

        Args:
            input_file: 输入文件路径
            output_dir: 输出目录

        Returns:
            图表文件路径字典。`config.visualization.enabled` 为 False 时是**空字典**
            而不是抛错（L215）：这一族键是阶段开关，关掉的意思是「这一步别做」，
            不是「这个能力不存在」。调用方（含 `GET /api/visualize/{filename}`）
            因此拿到 200 与空清单，形状不变。

        出厂配置下这一节是 `true` ⇒ 接上之后随包行为零漂移。
        """
        # L215（enabled 族第一批第四只）
        if not self.config.visualization.enabled:
            logger.info("config.visualization.enabled=false，跳过可视化阶段")
            return {}

        with open(input_file, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        self.visualizer.output_dir = Path(output_dir)
        return self.visualizer.generate_all_visualizations(items)
    
    async def augment_async(self,
                            input_file: str,
                            output_file: str,
                            use_checkpoint: bool = True,
                            use_quality_check: Optional[bool] = None,
                            use_dedup: Optional[bool] = None,
                            max_workers: int = 4) -> Dict:
        """异步增强数据集 - 性能优化版，使用 asyncio 并行处理
        
        Args:
            input_file: 输入文件路径
            output_file: 输出文件路径
            use_checkpoint: 是否使用断点续传
            use_quality_check: 是否使用质量检查。`None`（默认）交给
                `config.quality.enabled`，与同步入口 `augment_dataset` 同一道判
            use_dedup: 是否使用去重，同上，交给 `config.dedup.enabled`
            max_workers: 最大并发工作线程数
        
        Returns:
            处理报告
        """
        # L215：两个入口同一权威序，否则「同步读配置、异步不读」会变成同一份配置
        # 下两条路径出两种结果——这类分歧在报告里看不出来，只在产物里看得出来。
        if use_quality_check is None:
            use_quality_check = self.config.quality.enabled
        if use_dedup is None:
            use_dedup = self.config.dedup.enabled

        loop = asyncio.get_running_loop()
        with open(input_file, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        logger.info(f"异步增强启动: {len(items)} 条种子数据")
        
        # 使用线程池执行同步增强逻辑，避免阻塞事件循环
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            tasks = []
            for idx, item in enumerate(items):
                # 提交到线程池
                future = loop.run_in_executor(
                    executor,
                    self.augment_seed,
                    item,
                    use_quality_check,
                    []
                )
                tasks.append(future)
            
            results = await asyncio.gather(*tasks, return_exceptions=True)
        
        all_results = []
        for result in results:
            if isinstance(result, Exception):
                logger.error(f"异步处理单条数据失败: {result}")
                continue
            all_results.extend(result)
        
        # 去重
        if use_dedup and all_results:
            logger.info(f"异步去重前: {len(all_results)} 条")
            all_results = self.deduplicator.deduplicate_and_filter(all_results)
            logger.info(f"异步去重后: {len(all_results)} 条")
        
        # L168 收原子写：本方法是管线的最终交付落盘口（API 链落数据根、
        # list_data_files 扫描读者在写窗口内拿半份）
        atomic_write_json(output_file, all_results)
        
        logger.info(f"异步保存 {len(all_results)} 条数据到 {output_file}")

        # L216：与同步入口同一道判（「只接一个入口」在报告里看不出来，只在产物里看得出来）
        vector_index = None
        if self.config.vector.enabled:
            vector_index = self.build_vector_index(all_results)

        return {
            "task_id": f"async_{hashlib.md5(str(input_file).encode()).hexdigest()[:8]}",
            "input_file": input_file,
            "output_file": output_file,
            "input_count": len(items),
            "output_count": len(all_results),
            "async_mode": True,
            "quality_check": use_quality_check,
            "dedup": use_dedup,
            "vector_index": vector_index
        }
