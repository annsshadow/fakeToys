"""数据集导出增强模块

支持更多导出格式和导出选项。
"""

import json
import csv
import logging
from typing import List, Dict, Optional, Any, TextIO
from pathlib import Path
from dataclasses import dataclass, field
from enum import Enum

logger = logging.getLogger(__name__)


class ExportFormat(Enum):
    """导出格式枚举（全包唯一来源）

    `augmentor/export.py` 曾经**另有一份同名枚举**（6 个成员），而
    `augmentor/__init__.py` 导出的是这一份（13 个成员）。两份枚举的值大部分重叠，
    但 `Enum.__call__` 按**成员身份**匹配（`cls._value2member_map_`），
    于是公开名 `augmentor.ExportFormat` 的成员传给 `pipeline.export_dataset`
    时，即使值完全一致也会被拒：

        >>> ExportFormat.CHATML          # 来自本模块
        >>> Exporter.export(items, p, ExportFormat.CHATML)
        ValueError: <ExportFormat.CHATML: 'chatml'> is not a valid ExportFormat

    现在 `augmentor/export.py` 直接复用本枚举，不再有第二份定义。
    """
    JSON = "json"
    JSONL = "jsonl"
    CSV = "csv"
    TSV = "tsv"
    ALPACA = "alpaca"
    SHAREGPT = "sharegpt"
    # 历史拼写（带下划线）。与 SHAREGPT **同值**，因此是别名而非独立成员：
    # 迭代 ExportFormat 只产出 SHAREGPT，格式清单的长度不受影响。
    # 保留它是因为 augmentor/export.py 曾用这个拼写，外部代码可能仍在引用。
    SHARE_GPT = "sharegpt"
    CHATML = "chatml"
    LLAMA_FACTORY = "llama_factory"
    VICUNA = "vicuna"
    BELLE = "belle"
    OPENAI = "openai"
    HUGGINGFACE = "huggingface"
    RAW = "raw"


@dataclass
class ExportOptions:
    """导出选项"""
    format: ExportFormat = ExportFormat.JSON
    indent: int = 2
    ensure_ascii: bool = False
    include_fields: Optional[List[str]] = None
    exclude_fields: Optional[List[str]] = None
    max_items: Optional[int] = None
    shuffle: bool = False
    seed: int = 42
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "format": self.format.value,
            "indent": self.indent,
            "ensure_ascii": self.ensure_ascii,
            "include_fields": self.include_fields,
            "exclude_fields": self.exclude_fields,
            "max_items": self.max_items,
            "shuffle": self.shuffle,
            "seed": self.seed
        }


class EnhancedExporter:
    """增强导出器
    
    支持更多导出格式和导出选项。
    """
    
    def __init__(self):
        """初始化导出器"""
        self._formatters = {
            ExportFormat.JSON: self._export_json,
            ExportFormat.JSONL: self._export_jsonl,
            ExportFormat.CSV: self._export_csv,
            ExportFormat.TSV: self._export_tsv,
            ExportFormat.ALPACA: self._export_alpaca,
            ExportFormat.SHAREGPT: self._export_sharegpt,
            ExportFormat.CHATML: self._export_chatml,
            ExportFormat.LLAMA_FACTORY: self._export_llama_factory,
            ExportFormat.VICUNA: self._export_vicuna,
            ExportFormat.BELLE: self._export_belle,
            ExportFormat.OPENAI: self._export_openai,
            ExportFormat.HUGGINGFACE: self._export_huggingface,
            ExportFormat.RAW: self._export_raw,
        }
    
    def export(self, items: List[Dict], output_path: str, options: ExportOptions = None) -> Dict:
        """导出数据集
        
        Args:
            items: 数据列表
            output_path: 输出路径
            options: 导出选项
        
        Returns:
            导出结果
        """
        options = options or ExportOptions()
        
        # 处理数据
        processed_items = self._process_items(items, options)
        
        # 获取格式化函数
        formatter = self._formatters.get(options.format)
        if not formatter:
            raise ValueError(f"不支持的格式: {options.format}")
        
        # 创建输出目录
        output = Path(output_path)
        output.parent.mkdir(parents=True, exist_ok=True)
        
        # 导出数据
        formatter(processed_items, output, options)
        
        return {
            "format": options.format.value,
            "output_path": str(output_path),
            "item_count": len(processed_items),
            "options": options.to_dict()
        }
    
    def _process_items(self, items: List[Dict], options: ExportOptions) -> List[Dict]:
        """处理数据
        
        Args:
            items: 原始数据
            options: 导出选项
        
        Returns:
            处理后的数据
        """
        import random
        
        processed = items.copy()
        
        # 随机打乱
        if options.shuffle:
            random.seed(options.seed)
            random.shuffle(processed)
        
        # 限制数量
        if options.max_items and options.max_items > 0:
            processed = processed[:options.max_items]
        
        # 筛选字段
        if options.include_fields:
            processed = [
                {k: v for k, v in item.items() if k in options.include_fields}
                for item in processed
            ]
        elif options.exclude_fields:
            processed = [
                {k: v for k, v in item.items() if k not in options.exclude_fields}
                for item in processed
            ]
        
        return processed
    
    def _export_json(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出JSON格式"""
        with open(output, 'w', encoding='utf-8') as f:
            json.dump(items, f, ensure_ascii=options.ensure_ascii, indent=options.indent)
    
    def _export_jsonl(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出JSONL格式"""
        with open(output, 'w', encoding='utf-8') as f:
            for item in items:
                f.write(json.dumps(item, ensure_ascii=options.ensure_ascii) + '\n')
    
    def _export_csv(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出CSV格式"""
        if not items:
            return
        
        fieldnames = list(items[0].keys())
        
        with open(output, 'w', encoding='utf-8', newline='') as f:
            writer = csv.DictWriter(f, fieldnames=fieldnames)
            writer.writeheader()
            writer.writerows(items)
    
    def _export_tsv(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出TSV格式"""
        if not items:
            return
        
        fieldnames = list(items[0].keys())
        
        with open(output, 'w', encoding='utf-8', newline='') as f:
            writer = csv.DictWriter(f, fieldnames=fieldnames, delimiter='\t')
            writer.writeheader()
            writer.writerows(items)
    
    def _export_alpaca(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出Alpaca格式"""
        alpaca_items = []
        for item in items:
            alpaca_item = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", "")
            }
            alpaca_items.append(alpaca_item)
        
        self._export_json(alpaca_items, output, options)
    
    def _export_sharegpt(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出ShareGPT格式"""
        sharegpt_items = []
        for item in items:
            conversations = []
            
            instruction = item.get("instruction", "")
            output_text = item.get("output", "")
            
            if instruction:
                conversations.append({
                    "from": "human",
                    "value": instruction
                })
            
            if output_text:
                conversations.append({
                    "from": "gpt",
                    "value": output_text
                })
            
            sharegpt_items.append({
                "conversations": conversations
            })
        
        self._export_json(sharegpt_items, output, options)
    
    def _export_chatml(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出ChatML格式"""
        chatml_items = []
        for item in items:
            messages = []
            
            instruction = item.get("instruction", "")
            output_text = item.get("output", "")
            
            if instruction:
                messages.append({
                    "role": "user",
                    "content": instruction
                })
            
            if output_text:
                messages.append({
                    "role": "assistant",
                    "content": output_text
                })
            
            chatml_items.append({
                "messages": messages
            })
        
        self._export_json(chatml_items, output, options)
    
    def _export_llama_factory(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出Llama-Factory格式"""
        llama_items = []
        for item in items:
            llama_item = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", "")
            }
            llama_items.append(llama_item)
        
        self._export_json(llama_items, output, options)
    
    def _export_vicuna(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出Vicuna格式"""
        vicuna_items = []
        for item in items:
            instruction = item.get("instruction", "")
            output_text = item.get("output", "")
            
            vicuna_item = {
                "conversations": [
                    {
                        "from": "human",
                        "value": instruction
                    },
                    {
                        "from": "gpt",
                        "value": output_text
                    }
                ]
            }
            vicuna_items.append(vicuna_item)
        
        self._export_json(vicuna_items, output, options)
    
    def _export_belle(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出BELLE格式"""
        belle_items = []
        for item in items:
            instruction = item.get("instruction", "")
            output_text = item.get("output", "")
            
            belle_item = {
                "instruction": instruction,
                "output": output_text
            }
            belle_items.append(belle_item)
        
        self._export_json(belle_items, output, options)
    
    def _export_openai(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出OpenAI格式"""
        openai_items = []
        for item in items:
            instruction = item.get("instruction", "")
            output_text = item.get("output", "")
            
            messages = []
            if instruction:
                messages.append({"role": "user", "content": instruction})
            if output_text:
                messages.append({"role": "assistant", "content": output_text})
            
            openai_items.append({"messages": messages})
        
        self._export_json(openai_items, output, options)
    
    def _export_huggingface(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出HuggingFace格式"""
        hf_items = []
        for item in items:
            hf_item = {
                "text": f"{item.get('instruction', '')}\n{item.get('output', '')}".strip()
            }
            hf_items.append(hf_item)
        
        self._export_json(hf_items, output, options)
    
    def _export_raw(self, items: List[Dict], output: Path, options: ExportOptions):
        """导出原始格式"""
        self._export_json(items, output, options)


def export_dataset(items: List[Dict], output_path: str, format: str = "json", **kwargs) -> Dict:
    """导出数据集
    
    Args:
        items: 数据列表
        output_path: 输出路径
        format: 导出格式
        **kwargs: 额外选项
    
    Returns:
        导出结果
    """
    exporter = EnhancedExporter()
    options = ExportOptions(format=ExportFormat(format), **kwargs)
    return exporter.export(items, output_path, options)


def get_supported_formats() -> List[str]:
    """获取支持的导出格式
    
    Returns:
        格式列表
    """
    return [fmt.value for fmt in ExportFormat]
