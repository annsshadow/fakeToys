"""AI 训练数据增强主流程 - 优化版"""

import json
import logging
import time
import threading
from typing import List, Dict, Optional
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from queue import Queue

from .config import AppConfig, load_config, get_model_config
from .models import create_model_backend
from .quality import QualityScorer
from .dedup import Deduplicator
from .export import Exporter
from .context import ContextAugmentor
from .checkpoint import CheckpointManager
from .versioning import VersionManager
from .sampler import ActiveSampler
from .expander import DomainExpander
from .tracker import ExperimentTracker
from .visualizer import DataVisualizer

logger = logging.getLogger(__name__)


class AugmentorPipeline:
    """AI 训练数据增强管道 - 优化版"""
    
    def __init__(self, config: Optional[AppConfig] = None, config_path: Optional[str] = None):
        """初始化增强管道
        
        Args:
            config: 应用配置，为 None 时从文件加载
            config_path: 配置文件路径
        """
        self.config = config or load_config(config_path)
        self._lock = threading.Lock()
        
        # 初始化组件
        self._init_components()
    
    def _init_components(self):
        """初始化各组件"""
        # 模型后端
        model_config = get_model_config(self.config, self.config.default_model)
        self.model_backend = create_model_backend(model_config)
        
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
        )
        
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
        self.expander = DomainExpander(self.model_backend)
        
        # 实验追踪器
        self.tracker = ExperimentTracker()
        
        # 可视化器
        self.visualizer = DataVisualizer()
    
    def _generate_variants(self, item: Dict) -> List[Dict]:
        """为单条数据生成变体
        
        Args:
            item: 原始数据
        
        Returns:
            生成的变体列表
        """
        instruction = item.get("instruction", "")
        output = item.get("output", "")
        
        prompt = f"""作为AI领域数据增强专家，你的任务是把原JSON对象中的instruction字段内容在保持相同含义的情况下，扩充为{self.config.augmentation.variants_per_seed}种不同的提问方式，新JSON对象只需包含instruction字段，并将新JSON对象追加到JSON数组里面，注意最终结果只需要JSON数组，不需要任何说明文字，原JSON对象如下：[{{"instruction": "{instruction}"}}]"""
        
        try:
            response = self.model_backend.generate(prompt)
            # 解析响应
            if response.strip().startswith('['):
                variants = json.loads(response.strip())
                if isinstance(variants, list):
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
            use_checkpoint: 是否使用断点续传
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
        
        # 生成任务 ID
        task_id = f"augment_{int(time.time())}"
        
        # 创建断点
        if use_checkpoint:
            checkpoint = self.checkpoint_manager.create_checkpoint(task_id, len(items))
            remaining_indices = self.checkpoint_manager.get_remaining_indices()
        else:
            remaining_indices = list(range(len(items)))
        
        # 处理数据
        all_results = []
        existing_generated = []
        max_existing_window = 500  # 滑动窗口大小，限制内存使用
        
        if use_parallel and len(remaining_indices) > 1:
            # 并行处理
            num_threads = min(
                self.config.augmentation.num_threads,
                len(remaining_indices)
            )
            logger.info(f"使用 {num_threads} 个线程并行处理")
            
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
        
        # 去重
        if use_dedup and all_results:
            logger.info(f"去重前: {len(all_results)} 条")
            all_results = self.deduplicator.deduplicate_and_filter(all_results)
            logger.info(f"去重后: {len(all_results)} 条")
        
        # 保存输出
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(all_results, f, ensure_ascii=False, indent=2)
        
        logger.info(f"保存 {len(all_results)} 条数据到 {output_file}")
        
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
            "progress": self.checkpoint_manager.get_progress() if use_checkpoint else None
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
            formats: 导出格式列表
        
        Returns:
            格式到文件路径的映射
        """
        # 加载数据
        with open(input_file, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        if formats:
            results = {}
            for fmt in formats:
                output_path = Path(output_dir) / f"train_data_{fmt}.json"
                self.exporter.export(items, str(output_path), fmt)
                results[fmt] = str(output_path)
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
