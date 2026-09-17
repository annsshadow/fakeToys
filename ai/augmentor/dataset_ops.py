"""数据集操作工具模块

提供数据集的合并、采样、分割等常用操作。
"""

import json
import random
import logging
from typing import List, Dict, Optional, Tuple, Union
from pathlib import Path
from dataclasses import dataclass
import hashlib

logger = logging.getLogger(__name__)


@dataclass
class MergeConfig:
    """合并配置"""
    deduplicate: bool = True  # 合并时是否去重
    dedup_threshold: float = 0.9  # 去重阈值
    preserve_order: bool = True  # 保持原始顺序
    max_items: Optional[int] = None  # 最大保留条数


@dataclass
class SampleConfig:
    """采样配置"""
    method: str = "random"  # random, stratified, systematic
    size: Optional[int] = None  # 采样数量
    ratio: Optional[float] = None  # 采样比例
    seed: Optional[int] = None  # 随机种子
    stratify_key: str = "instruction"  # 分层采样的字段


@dataclass
class SplitConfig:
    """分割配置"""
    ratios: Tuple[float, float, float] = (0.8, 0.1, 0.1)  # 训练/验证/测试比例
    shuffle: bool = True  # 是否打乱
    seed: Optional[int] = None  # 随机种子
    stratify: bool = False  # 是否分层分割
    stratify_key: str = "instruction"  # 分层字段


class DatasetOperations:
    """数据集操作工具类"""
    
    def __init__(self):
        """初始化操作工具"""
        pass
    
    # ==================== 合并操作 ====================
    
    def merge(self,
             datasets: List[List[Dict]],
             config: Optional[MergeConfig] = None) -> List[Dict]:
        """合并多个数据集
        
        Args:
            datasets: 数据集列表
            config: 合并配置
        
        Returns:
            合并后的数据集
        """
        config = config or MergeConfig()
        
        # 合并数据
        if config.preserve_order:
            merged = []
            for dataset in datasets:
                merged.extend(dataset)
        else:
            merged = []
            for dataset in datasets:
                merged.extend(dataset)
            random.shuffle(merged)
        
        # 去重
        if config.deduplicate:
            merged = self._deduplicate(merged, config.dedup_threshold)
        
        # 限制最大条数
        if config.max_items and len(merged) > config.max_items:
            merged = merged[:config.max_items]
        
        logger.info(f"合并完成: {len(datasets)} 个数据集, 共 {len(merged)} 条数据")
        return merged
    
    def merge_files(self,
                   file_paths: List[str],
                   output_path: str,
                   config: Optional[MergeConfig] = None) -> Dict:
        """合并多个数据集文件
        
        Args:
            file_paths: 文件路径列表
            output_path: 输出文件路径
            config: 合并配置
        
        Returns:
            合并结果统计
        """
        datasets = []
        for file_path in file_paths:
            with open(file_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
                datasets.append(data)
                logger.info(f"加载 {file_path}: {len(data)} 条数据")
        
        merged = self.merge(datasets, config)
        
        # 保存结果
        output = Path(output_path)
        output.parent.mkdir(parents=True, exist_ok=True)
        with open(output, 'w', encoding='utf-8') as f:
            json.dump(merged, f, ensure_ascii=False, indent=2)
        
        return {
            "input_files": len(file_paths),
            "output_file": output_path,
            "total_input": sum(len(d) for d in datasets),
            "total_output": len(merged),
            "removed_duplicates": sum(len(d) for d in datasets) - len(merged)
        }
    
    def _deduplicate(self, items: List[Dict], threshold: float = 0.9) -> List[Dict]:
        """简单的基于哈希的去重
        
        Args:
            items: 数据列表
            threshold: 阈值（此处未使用，保持接口一致）
        
        Returns:
            去重后的数据列表
        """
        seen_hashes = set()
        unique_items = []
        
        for item in items:
            # 使用instruction字段生成哈希
            text = item.get("instruction", "")
            item_hash = hashlib.md5(text.encode('utf-8')).hexdigest()
            
            if item_hash not in seen_hashes:
                seen_hashes.add(item_hash)
                unique_items.append(item)
        
        return unique_items
    
    # ==================== 采样操作 ====================
    
    def sample(self,
              items: List[Dict],
              config: Optional[SampleConfig] = None) -> List[Dict]:
        """对数据集进行采样
        
        Args:
            items: 数据列表
            config: 采样配置
        
        Returns:
            采样后的数据列表
        """
        config = config or SampleConfig()
        
        # 确定采样数量
        if config.size:
            sample_size = min(config.size, len(items))
        elif config.ratio:
            sample_size = int(len(items) * config.ratio)
        else:
            sample_size = len(items)
        
        # 设置随机种子
        if config.seed is not None:
            random.seed(config.seed)
        
        # 根据方法采样
        if config.method == "random":
            sampled = random.sample(items, sample_size)
        elif config.method == "systematic":
            step = max(1, len(items) // sample_size)
            sampled = items[::step][:sample_size]
        elif config.method == "stratified":
            sampled = self._stratified_sample(items, sample_size, config.stratify_key)
        else:
            raise ValueError(f"不支持的采样方法: {config.method}")
        
        logger.info(f"采样完成: {len(items)} -> {len(sampled)} 条数据")
        return sampled
    
    def _stratified_sample(self,
                          items: List[Dict],
                          sample_size: int,
                          key: str) -> List[Dict]:
        """分层采样
        
        Args:
            items: 数据列表
            sample_size: 采样数量
            key: 分层字段
        
        Returns:
            采样后的数据列表
        """
        # 按字段值分组
        groups = {}
        for item in items:
            value = item.get(key, "")
            # 简化：使用前10个字符作为分组键
            group_key = value[:10] if value else "empty"
            if group_key not in groups:
                groups[group_key] = []
            groups[group_key].append(item)
        
        # 按比例从每组采样
        sampled = []
        total = len(items)
        
        for group_key, group_items in groups.items():
            group_ratio = len(group_items) / total
            group_sample_size = max(1, int(sample_size * group_ratio))
            group_sampled = random.sample(group_items, min(group_sample_size, len(group_items)))
            sampled.extend(group_sampled)
        
        # 调整到目标数量
        if len(sampled) > sample_size:
            sampled = random.sample(sampled, sample_size)
        
        return sampled
    
    def sample_file(self,
                   input_path: str,
                   output_path: str,
                   config: Optional[SampleConfig] = None) -> Dict:
        """对数据集文件进行采样
        
        Args:
            input_path: 输入文件路径
            output_path: 输出文件路径
            config: 采样配置
        
        Returns:
            采样结果统计
        """
        with open(input_path, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        sampled = self.sample(items, config)
        
        # 保存结果
        output = Path(output_path)
        output.parent.mkdir(parents=True, exist_ok=True)
        with open(output, 'w', encoding='utf-8') as f:
            json.dump(sampled, f, ensure_ascii=False, indent=2)
        
        return {
            "input_file": input_path,
            "output_file": output_path,
            "input_count": len(items),
            "output_count": len(sampled)
        }
    
    # ==================== 分割操作 ====================
    
    def split(self,
             items: List[Dict],
             config: Optional[SplitConfig] = None) -> Tuple[List[Dict], List[Dict], List[Dict]]:
        """分割数据集
        
        Args:
            items: 数据列表
            config: 分割配置
        
        Returns:
            (训练集, 验证集, 测试集)
        """
        config = config or SplitConfig()
        
        # 验证比例
        total_ratio = sum(config.ratios)
        if abs(total_ratio - 1.0) > 0.01:
            raise ValueError(f"分割比例之和必须为1.0，当前为 {total_ratio}")
        
        # 打乱数据
        data = items.copy()
        if config.shuffle:
            if config.seed is not None:
                random.seed(config.seed)
            random.shuffle(data)
        
        # 计算各部分大小
        n = len(data)
        train_size = int(n * config.ratios[0])
        val_size = int(n * config.ratios[1])
        
        # 分割
        train = data[:train_size]
        val = data[train_size:train_size + val_size]
        test = data[train_size + val_size:]
        
        logger.info(f"分割完成: 训练集 {len(train)}, 验证集 {len(val)}, 测试集 {len(test)}")
        return train, val, test
    
    def split_file(self,
                  input_path: str,
                  output_dir: str,
                  config: Optional[SplitConfig] = None,
                  prefixes: Tuple[str, str, str] = ("train", "val", "test")) -> Dict:
        """分割数据集文件
        
        Args:
            input_path: 输入文件路径
            output_dir: 输出目录
            config: 分割配置
            prefixes: 输出文件前缀
        
        Returns:
            分割结果统计
        """
        with open(input_path, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        train, val, test = self.split(items, config)
        
        # 创建输出目录
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)
        
        # 保存各部分
        results = {}
        for prefix, data in zip(prefixes, [train, val, test]):
            file_path = output_path / f"{prefix}.json"
            with open(file_path, 'w', encoding='utf-8') as f:
                json.dump(data, f, ensure_ascii=False, indent=2)
            results[prefix] = {
                "file": str(file_path),
                "count": len(data)
            }
        
        return {
            "input_file": input_path,
            "output_dir": output_dir,
            "splits": results
        }
    
    # ==================== 其他操作 ====================
    
    def shuffle(self,
               items: List[Dict],
               seed: Optional[int] = None) -> List[Dict]:
        """打乱数据集顺序
        
        Args:
            items: 数据列表
            seed: 随机种子
        
        Returns:
            打乱后的数据列表
        """
        shuffled = items.copy()
        if seed is not None:
            random.seed(seed)
        random.shuffle(shuffled)
        return shuffled
    
    def head(self, items: List[Dict], n: int = 10) -> List[Dict]:
        """获取前N条数据
        
        Args:
            items: 数据列表
            n: 数量
        
        Returns:
            前N条数据
        """
        return items[:n]
    
    def tail(self, items: List[Dict], n: int = 10) -> List[Dict]:
        """获取后N条数据
        
        Args:
            items: 数据列表
            n: 数量
        
        Returns:
            后N条数据
        """
        return items[-n:] if len(items) >= n else items
    
    def filter_by_length(self,
                        items: List[Dict],
                        min_length: int = 0,
                        max_length: int = float('inf'),
                        key: str = "instruction") -> List[Dict]:
        """按文本长度过滤
        
        Args:
            items: 数据列表
            min_length: 最小长度
            max_length: 最大长度
            key: 过滤字段
        
        Returns:
            过滤后的数据列表
        """
        return [
            item for item in items
            if min_length <= len(item.get(key, "")) <= max_length
        ]
    
    def filter_by_keyword(self,
                         items: List[Dict],
                         keywords: List[str],
                         key: str = "instruction",
                         mode: str = "include") -> List[Dict]:
        """按关键词过滤
        
        Args:
            items: 数据列表
            keywords: 关键词列表
            key: 过滤字段
            mode: include（包含）或 exclude（排除）
        
        Returns:
            过滤后的数据列表
        """
        result = []
        for item in items:
            text = item.get(key, "")
            has_keyword = any(kw in text for kw in keywords)
            
            if mode == "include" and has_keyword:
                result.append(item)
            elif mode == "exclude" and not has_keyword:
                result.append(item)
        
        return result
    
    def get_statistics(self, items: List[Dict]) -> Dict:
        """获取数据集统计信息
        
        Args:
            items: 数据列表
        
        Returns:
            统计信息
        """
        if not items:
            return {"total": 0}
        
        lengths = [len(item.get("instruction", "")) for item in items]
        
        return {
            "total": len(items),
            "avg_length": sum(lengths) / len(lengths),
            "min_length": min(lengths),
            "max_length": max(lengths),
            "unique_instructions": len(set(item.get("instruction", "") for item in items))
        }


# ==================== 便捷函数 ====================

def merge_datasets(file_paths: List[str], output_path: str, **kwargs) -> Dict:
    """合并数据集文件
    
    Args:
        file_paths: 文件路径列表
        output_path: 输出文件路径
        **kwargs: 传递给MergeConfig的参数
    
    Returns:
        合并结果统计
    """
    ops = DatasetOperations()
    config = MergeConfig(**kwargs)
    return ops.merge_files(file_paths, output_path, config)


def sample_dataset(input_path: str, output_path: str, **kwargs) -> Dict:
    """对数据集文件进行采样
    
    Args:
        input_path: 输入文件路径
        output_path: 输出文件路径
        **kwargs: 传递给SampleConfig的参数
    
    Returns:
        采样结果统计
    """
    ops = DatasetOperations()
    config = SampleConfig(**kwargs)
    return ops.sample_file(input_path, output_path, config)


def split_dataset(input_path: str, output_dir: str, **kwargs) -> Dict:
    """分割数据集文件
    
    Args:
        input_path: 输入文件路径
        output_dir: 输出目录
        **kwargs: 传递给SplitConfig的参数
    
    Returns:
        分割结果统计
    """
    ops = DatasetOperations()
    config = SplitConfig(**kwargs)
    return ops.split_file(input_path, output_dir, config)
