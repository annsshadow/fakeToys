"""多格式导出模块 - 优化版"""

import json
import csv
import logging
from typing import List, Dict, Optional
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed

from .export_enhanced import EnhancedExporter, ExportFormat, ExportOptions

logger = logging.getLogger(__name__)

# Exporter **原生实现**的格式。其余成员（json / tsv / vicuna / belle / openai /
# huggingface / raw）由 `Exporter.export` 委托给 EnhancedExporter 完成。
#
# 这个元组刻意**不等于**整个 ExportFormat：
#   * `export_all_formats` 迭代它，从而保持「一次导出 6 个文件」的既有行为。
#     若改为迭代 13 个成员的枚举，`pipeline.export_dataset(formats=None)` 与
#     CLI 的 export 命令会突然产出 13 个文件，属破坏性变更；
#   * `preview` 也以它为支持边界（预览只实现了这 6 种转换）。
NATIVE_FORMATS: tuple = (
    ExportFormat.JSONL,
    ExportFormat.LLAMA_FACTORY,
    ExportFormat.ALPACA,
    ExportFormat.SHARE_GPT,
    ExportFormat.CHATML,
    ExportFormat.CSV,
)

# 导出文件的扩展名。TSV 在枚举统一后变得可达，不能再落进 ".json" 兜底。
_FORMAT_EXTENSIONS: Dict[str, str] = {
    ExportFormat.CSV.value: ".csv",
    ExportFormat.TSV.value: ".tsv",
}


class Exporter:
    """数据导出器 - 优化版"""
    
    def __init__(self, default_format: str = "jsonl", max_workers: int = 4):
        """初始化导出器（带格式转换缓存优化）
        
        Args:
            default_format: 默认导出格式
            max_workers: 最大并发数
        """
        self.default_format = default_format
        self.max_workers = max_workers
        # 导出格式转换缓存：避免重复转换相同数据到相同格式
        self._export_cache: Dict[str, List[Dict]] = {}
        self.default_format = ExportFormat(default_format)
        self.max_workers = max_workers
    
    def _convert_to_jsonl(self, items: List[Dict]) -> List[str]:
        """转换为 JSONL 格式
        
        Args:
            items: 数据列表
        
        Returns:
            JSONL 行列表
        """
        lines = []
        for item in items:
            record = {
                "input": item.get("input", ""),
                "instruction": item.get("instruction", ""),
                "output": item.get("output", "")
            }
            lines.append(json.dumps(record, ensure_ascii=False))
        return lines
    
    def _convert_to_llama_factory(self, items: List[Dict]) -> List[Dict]:
        """转换为 Llama-Factory 格式
        
        Args:
            items: 数据列表
        
        Returns:
            Llama-Factory 格式数据列表
        """
        result = []
        for item in items:
            record = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", ""),
                "system": item.get("system", "你是安居乐寓的智能客服")
            }
            result.append(record)
        return result
    
    def _convert_to_alpaca(self, items: List[Dict]) -> List[Dict]:
        """转换为 Alpaca 格式
        
        Args:
            items: 数据列表
        
        Returns:
            Alpaca 格式数据列表
        """
        result = []
        for item in items:
            record = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", "")
            }
            result.append(record)
        return result
    
    def _convert_to_sharegpt(self, items: List[Dict]) -> List[Dict]:
        """转换为 ShareGPT 格式
        
        Args:
            items: 数据列表
        
        Returns:
            ShareGPT 格式数据列表
        """
        result = []
        for item in items:
            conversations = []
            
            # 添加历史对话
            history = item.get("history", [])
            for turn in history:
                conversations.append({
                    "from": turn.get("role", "user"),
                    "value": turn.get("content", "")
                })
            
            # 添加当前问答
            conversations.append({
                "from": "human",
                "value": item.get("instruction", "")
            })
            conversations.append({
                "from": "gpt",
                "value": item.get("output", "")
            })
            
            record = {"conversations": conversations}
            result.append(record)
        return result
    
    def _convert_to_chatml(self, items: List[Dict]) -> List[Dict]:
        """转换为 ChatML 格式
        
        Args:
            items: 数据列表
        
        Returns:
            ChatML 格式数据列表
        """
        result = []
        for item in items:
            messages = []
            
            # 添加系统消息
            system = item.get("system", "你是安居乐寓的智能客服")
            if system:
                messages.append({
                    "role": "system",
                    "content": system
                })
            
            # 添加历史对话
            history = item.get("history", [])
            for turn in history:
                messages.append({
                    "role": turn.get("role", "user"),
                    "content": turn.get("content", "")
                })
            
            # 添加当前问答
            messages.append({
                "role": "user",
                "content": item.get("instruction", "")
            })
            messages.append({
                "role": "assistant",
                "content": item.get("output", "")
            })
            
            record = {"messages": messages}
            result.append(record)
        return result
    
    def _convert_to_csv(self, items: List[Dict]) -> List[Dict]:
        """转换为 CSV 格式（返回字典列表）
        
        Args:
            items: 数据列表
        
        Returns:
            CSV 数据列表
        """
        result = []
        for item in items:
            record = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", "")
            }
            result.append(record)
        return result
    
    def export(self, 
               items: List[Dict], 
               output_path: str,
               format: Optional[str] = None) -> str:
        """导出数据
        
        Args:
            items: 数据列表
            output_path: 输出文件路径
            format: 导出格式，可传字符串（如 "jsonl"）或 `ExportFormat` 成员；
                    为 None 时使用默认格式
        
        Returns:
            实际使用的格式（字符串）
        """
        export_format = ExportFormat(format) if format else self.default_format
        
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        if export_format not in NATIVE_FORMATS:
            # 没有原生实现的格式委托给 EnhancedExporter（它覆盖全部 13 种）。
            # 不能只靠下面的 if/elif 链兜底：那些分支全不命中时会**静默地不写
            # 任何文件**然后返回成功，比直接报错更难排查。
            EnhancedExporter().export(
                items, str(output_path), ExportOptions(format=export_format)
            )
            logger.info(
                f"导出 {len(items)} 条数据到 {output_path}，格式: {export_format.value}"
            )
            return export_format.value
        
        if export_format == ExportFormat.JSONL:
            lines = self._convert_to_jsonl(items)
            with open(output_path, 'w', encoding='utf-8') as f:
                f.write('\n'.join(lines))
        
        elif export_format == ExportFormat.LLAMA_FACTORY:
            data = self._convert_to_llama_factory(items)
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(data, f, ensure_ascii=False, indent=2)
        
        elif export_format == ExportFormat.ALPACA:
            data = self._convert_to_alpaca(items)
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(data, f, ensure_ascii=False, indent=2)
        
        elif export_format == ExportFormat.SHARE_GPT:
            data = self._convert_to_sharegpt(items)
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(data, f, ensure_ascii=False, indent=2)
        
        elif export_format == ExportFormat.CHATML:
            data = self._convert_to_chatml(items)
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(data, f, ensure_ascii=False, indent=2)
        
        elif export_format == ExportFormat.CSV:
            data = self._convert_to_csv(items)
            if data:
                with open(output_path, 'w', encoding='utf-8', newline='') as f:
                    writer = csv.DictWriter(f, fieldnames=data[0].keys())
                    writer.writeheader()
                    writer.writerows(data)
        
        # ExportFormat 六个成员均已在上方分支处理，无其他可能值
        logger.info(f"导出 {len(items)} 条数据到 {output_path}，格式: {export_format.value}")
        return export_format.value
    
    def export_all_formats(self, 
                          items: List[Dict],
                          output_dir: str,
                          base_name: str = "train_data",
                          use_parallel: bool = True) -> Dict[str, str]:
        """导出所有格式（优化版）
        
        导出的是 `NATIVE_FORMATS`（6 种），**不是**整个 `ExportFormat` 枚举——
        枚举统一后含 13 个成员，全量导出会让调用方的产物数量翻倍。
        
        Args:
            items: 数据列表
            output_dir: 输出目录
            base_name: 基础文件名
            use_parallel: 是否使用并行处理
        
        Returns:
            格式到文件路径的映射
        """
        output_dir = Path(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)
        
        results = {}
        
        def export_single(fmt):
            ext = _FORMAT_EXTENSIONS.get(fmt.value, ".json")
            output_path = output_dir / f"{base_name}_{fmt.value}{ext}"
            self.export(items, str(output_path), fmt.value)
            return fmt.value, str(output_path)
        
        if not use_parallel:
            # 串行处理
            for fmt in NATIVE_FORMATS:
                fmt_name, path = export_single(fmt)
                results[fmt_name] = path
        else:
            # 并行处理
            with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
                futures = [executor.submit(export_single, fmt) for fmt in NATIVE_FORMATS]
                
                for future in as_completed(futures):
                    try:
                        fmt_name, path = future.result()
                        results[fmt_name] = path
                        logger.info(f"导出格式 {fmt_name} 完成")
                    except Exception as e:
                        logger.error(f"导出失败: {e}")
        
        return results
    
    def export_batch(self,
                     datasets: Dict[str, List[Dict]],
                     output_dir: str,
                     formats: Optional[List[str]] = None,
                     use_parallel: bool = True) -> Dict[str, Dict[str, str]]:
        """批量导出多个数据集（每个数据集可导出多种格式）

        Args:
            datasets: 数据集名称到数据列表的映射
            output_dir: 输出目录
            formats: 导出格式列表，为 None 时使用默认格式
            use_parallel: 是否使用并行处理

        Returns:
            数据集名称到 {格式: 文件路径} 的映射
        """
        output_dir_path = Path(output_dir)
        output_dir_path.mkdir(parents=True, exist_ok=True)

        format_names = formats or [self.default_format.value]
        # 校验格式合法性，避免部分任务失败后才发现
        for fmt in format_names:
            ExportFormat(fmt)

        def export_single(name: str, items: List[Dict], fmt: str):
            ext = _FORMAT_EXTENSIONS.get(fmt, ".json")
            output_path = output_dir_path / f"{name}_{fmt}{ext}"
            self.export(items, str(output_path), fmt)
            return name, fmt, str(output_path)

        results: Dict[str, Dict[str, str]] = {name: {} for name in datasets}

        if not use_parallel:
            for name, items in datasets.items():
                for fmt in format_names:
                    name_, fmt_, path = export_single(name, items, fmt)
                    results[name_][fmt_] = path
            return results

        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            futures = [
                executor.submit(export_single, name, items, fmt)
                for name, items in datasets.items()
                for fmt in format_names
            ]

            for future in as_completed(futures):
                try:
                    name_, fmt_, path = future.result()
                    results[name_][fmt_] = path
                except Exception as e:
                    logger.error(f"批量导出失败: {e}")

        return results

    def get_supported_formats(self) -> List[str]:
        """获取支持的导出格式
        
        Returns:
            格式名称列表
        """
        return [fmt.value for fmt in ExportFormat]
