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
import asyncio
from concurrent.futures import ThreadPoolExecutor, as_completed
from queue import Queue

from .config import (REQUEST_TIMEOUT_RANGE, AppConfig, load_config,
                     get_model_config)
from .validation import require_count, require_ratio, require_seconds
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

logger = logging.getLogger(__name__)


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
                            result_queue: Queue):
        """处理单条数据（线程安全）
        
        Args:
            idx: 索引
            item: 数据项
            use_quality_check: 是否使用质量检查
            result_queue: 结果队列
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
            # 错误恢复增强：记录详细错误信息以便后续分析和重试
            error_info = {
                "index": idx,
                "error_type": type(e).__name__,
                "message": str(e),
                "item_preview": str(item)[:100] if item else ""
            }
            if not hasattr(self, '_process_errors'):
                self._process_errors = []
            self._process_errors.append(error_info)
            result_queue.put((idx, False, []))
    
    def augment_dataset(self,
                       input_file: str,
                       output_file: str,
                       use_checkpoint: bool = True,
                       use_quality_check: bool = True,
                       use_dedup: bool = True,
                       use_parallel: bool = True) -> Dict:
        """增强整个数据集 - 优化版
        
        Args:
            input_file: 输入文件路径
            output_file: 输出文件路径
            use_checkpoint: 是否使用断点续传；任务 ID 由输入文件路径派生，
                因此对同一输入文件重复运行会跳过已完成的条目并复用已有输出。
                如需从头重跑，先调用 checkpoint_manager.delete_checkpoint(task_id)
            use_quality_check: 是否使用质量检查
            use_dedup: 是否使用去重
            use_parallel: 是否使用并行处理
        
        Returns:
            处理报告
        """
        # 加载输入数据
        with open(input_file, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        logger.info(f"加载 {len(items)} 条种子数据")
        
        # 生成任务 ID（基于输入文件，保证跨进程稳定以便续传）
        task_id = self.build_task_id(input_file)
        
        # 创建或恢复断点
        if use_checkpoint:
            resumed = self.checkpoint_manager.load_checkpoint(task_id)
            if resumed is None:
                self.checkpoint_manager.create_checkpoint(task_id, len(items))
            remaining_indices = self.checkpoint_manager.get_remaining_indices()
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
            
            result_queue = Queue()
            
            with ThreadPoolExecutor(max_workers=num_threads) as executor:
                # 提交所有任务
                futures = {}
                for idx in remaining_indices:
                    item = items[idx]
                    future = executor.submit(
                        self._process_single_item,
                        idx, item, use_quality_check, result_queue
                    )
                    futures[future] = idx
                
                # 收集结果
                processed = 0
                for future in as_completed(futures):
                    idx, success, variants = result_queue.get()
                    
                    if success:
                        all_results.extend(variants)
                        existing_generated.extend([v.get("instruction", "") for v in variants])
                        # 滑动窗口，限制内存使用
                        if len(existing_generated) > max_existing_window:
                            existing_generated = existing_generated[-max_existing_window:]
                        
                        if use_checkpoint:
                            self.checkpoint_manager.update_progress(idx, True)
                        
                        processed += 1
                        if processed % 10 == 0:
                            logger.info(f"已处理 {processed}/{len(remaining_indices)}")
                    else:
                        if use_checkpoint:
                            self.checkpoint_manager.update_progress(idx, False)
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
                        self.checkpoint_manager.update_progress(idx, True)
                    
                    logger.info(f"生成 {len(variants)} 个变体")
                    
                except Exception as e:
                    logger.error(f"处理失败: {e}")
                    if use_checkpoint:
                        self.checkpoint_manager.update_progress(idx, False)
        
        # 合并增量并落盘，确保中断后可恢复
        if use_checkpoint:
            self.checkpoint_manager.save_checkpoint()
        
        # 去重
        if use_dedup and all_results:
            logger.info(f"去重前: {len(all_results)} 条")
            all_results = self.deduplicator.deduplicate_and_filter(all_results)
            logger.info(f"去重后: {len(all_results)} 条")
        
        # 保存输出
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(all_results, f, ensure_ascii=False, indent=2)
        
        logger.info(f"保存 {len(all_results)} 条数据到 {output_file}")
        
        # 记录最终内存状态（集成监控）
        memory_monitor.take_snapshot()
        memory_summary = get_memory_summary()
        logger.info(f"增强流程内存监控完成，峰值: {memory_monitor.get_peak_usage_mb():.2f} MB，趋势: {memory_monitor.get_trend()}")
        
        # 创建版本快照
        if self.config.versioning.auto_snapshot:
            self.version_manager.create_version(
                all_results,
                label=f"增强数据_{int(time.time())}",
                description=f"从 {input_file} 增强生成"
            )
        
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
            "progress": self.checkpoint_manager.get_progress() if use_checkpoint else None,
            "memory_monitor": {
                "peak_usage_mb": memory_monitor.get_peak_usage_mb(),
                "trend": memory_monitor.get_trend(),
                "final_usage_mb": memory_summary.get("current_usage_mb", 0.0),
                "monitor_available": memory_summary.get("monitor_available", False)
            }
        }
    
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
                output_path = Path(output_dir) / f"train_data_{name}.json"
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
            图表文件路径字典
        """
        with open(input_file, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        self.visualizer.output_dir = Path(output_dir)
        return self.visualizer.generate_all_visualizations(items)
    
    async def augment_async(self,
                            input_file: str,
                            output_file: str,
                            use_checkpoint: bool = True,
                            use_quality_check: bool = True,
                            use_dedup: bool = True,
                            max_workers: int = 4) -> Dict:
        """异步增强数据集 - 性能优化版，使用 asyncio 并行处理
        
        Args:
            input_file: 输入文件路径
            output_file: 输出文件路径
            use_checkpoint: 是否使用断点续传
            use_quality_check: 是否使用质量检查
            use_dedup: 是否使用去重
            max_workers: 最大并发工作线程数
        
        Returns:
            处理报告
        """
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
        
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(all_results, f, ensure_ascii=False, indent=2)
        
        logger.info(f"异步保存 {len(all_results)} 条数据到 {output_file}")
        
        return {
            "task_id": f"async_{hashlib.md5(str(input_file).encode()).hexdigest()[:8]}",
            "input_file": input_file,
            "output_file": output_file,
            "input_count": len(items),
            "output_count": len(all_results),
            "async_mode": True,
            "quality_check": use_quality_check,
            "dedup": use_dedup
        }
