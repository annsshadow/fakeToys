# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集格式转换模块

提供多种数据集格式之间的转换功能。
"""

import json
import csv
import logging
from typing import List, Dict, Optional, Any
from pathlib import Path
from enum import Enum
from .exceptions import DataFormatError, UnsupportedFormatError

logger = logging.getLogger(__name__)


def csv_fieldnames(items: List[Dict]) -> List[str]:
    """CSV/TSV 的表头：全量键的**并集**，按首次出现顺序排列

    不能只取 `items[0].keys()`。增强与合并的产物天然异构（某一路数据多了
    `category` 字段），用首条的键建表头会让 `DictWriter` 在后续行上抛
    `ValueError: dict contains fields not in fieldnames`，整次导出失败；
    取并集后缺字段的行由 `restval` 落成空单元格。
    """
    keys: List[str] = []
    seen = set()
    for item in items:
        for key in item:
            if key not in seen:
                seen.add(key)
                keys.append(key)
    return keys


class DataFormat(Enum):
    """数据格式"""
    JSON = "json"
    JSONL = "jsonl"
    CSV = "csv"
    TSV = "tsv"
    ALPACA = "alpaca"
    SHAREGPT = "sharegpt"
    CHATML = "chatml"
    LLAMA_FACTORY = "llama_factory"
    VICUNA = "vicuna"
    BELLE = "belle"


class DatasetConverter:
    """数据集格式转换器
    
    支持多种格式之间的转换。
    """
    
    def __init__(self):
        """初始化转换器"""
        self._converters = {
            ("json", "jsonl"): self._json_to_jsonl,
            ("jsonl", "json"): self._jsonl_to_json,
            ("json", "csv"): self._json_to_csv,
            ("csv", "json"): self._csv_to_json,
            ("json", "alpaca"): self._json_to_alpaca,
            ("json", "sharegpt"): self._json_to_sharegpt,
            ("json", "chatml"): self._json_to_chatml,
            ("json", "llama_factory"): self._json_to_llama_factory,
            ("json", "vicuna"): self._json_to_vicuna,
            ("json", "belle"): self._json_to_belle,
        }
    
    def convert(self, 
               data: List[Dict], 
               source_format: str, 
               target_format: str,
               **kwargs) -> Any:
        """转换数据格式
        
        Args:
            data: 数据列表
            source_format: 源格式
            target_format: 目标格式
            **kwargs: 额外参数
        
        Returns:
            转换后的数据
        """
        # 标准化格式名称
        source_format = self._normalize_format(source_format)
        target_format = self._normalize_format(target_format)
        
        # 如果格式相同，直接返回
        if source_format == target_format:
            return data
        
        # 查找转换器
        key = (source_format, target_format)
        if key in self._converters:
            return self._converters[key](data, **kwargs)
        
        # 尝试通过JSON中间格式转换
        if source_format != "json":
            json_data = self._to_json(data, source_format)
            return self.convert(json_data, "json", target_format, **kwargs)
        
        raise UnsupportedFormatError(f"不支持的转换: {source_format} -> {target_format}")
    
    def _normalize_format(self, format_name: str) -> str:
        """标准化格式名称"""
        format_map = {
            "json": "json",
            "jsonl": "jsonl",
            "csv": "csv",
            "tsv": "tsv",
            "alpaca": "alpaca",
            "sharegpt": "sharegpt",
            "chatml": "chatml",
            "llama_factory": "llama_factory",
            "vicuna": "vicuna",
            "belle": "belle",
        }
        return format_map.get(format_name.lower(), format_name.lower())
    
    def _to_json(self, data: Any, source_format: str) -> List[Dict]:
        """转换为JSON格式"""
        if source_format == "jsonl":
            return self._jsonl_to_json(data)
        elif source_format == "csv":
            return self._csv_to_json(data)
        else:
            raise DataFormatError(f"无法从 {source_format} 转换到 JSON")
    
    # ==================== 基础格式转换 ====================
    
    def _json_to_jsonl(self, data: List[Dict], **kwargs) -> List[str]:
        """JSON 转 JSONL"""
        return [json.dumps(item, ensure_ascii=False) for item in data]
    
    def _jsonl_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """JSONL 转 JSON"""
        if isinstance(data, list):
            return [json.loads(line) if isinstance(line, str) else line for line in data]
        return [json.loads(line) for line in data]
    
    def _json_to_csv(self, data: List[Dict], **kwargs) -> List[Dict]:
        """JSON 转 CSV（返回字典列表）"""
        if not data:
            return []
        
        # 收集所有字段
        all_keys = []
        for item in data:
            for key in item.keys():
                if key not in all_keys:
                    all_keys.append(key)
        
        return [item for item in data]
    
    def _csv_to_json(self, data: Any, **kwargs) -> List[Dict]:
        """CSV 转 JSON"""
        if isinstance(data, list):
            return data
        return []
    
    # ==================== 训练格式转换 ====================
    
    def _json_to_alpaca(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 Alpaca 格式"""
        result = []
        for item in data:
            record = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", "")
            }
            result.append(record)
        return result
    
    def _json_to_sharegpt(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 ShareGPT 格式"""
        result = []
        for item in data:
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
            
            result.append({"conversations": conversations})
        return result
    
    def _json_to_chatml(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 ChatML 格式"""
        result = []
        for item in data:
            messages = []
            
            # 添加系统消息
            system = item.get("system", "You are a helpful assistant.")
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
            
            result.append({"messages": messages})
        return result
    
    def _json_to_llama_factory(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 Llama-Factory 格式"""
        result = []
        for item in data:
            record = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", ""),
                "system": item.get("system", "")
            }
            result.append(record)
        return result
    
    def _json_to_vicuna(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 Vicuna 格式"""
        result = []
        for item in data:
            record = {
                "id": item.get("id", ""),
                "conversations": [
                    {
                        "from": "human",
                        "value": item.get("instruction", "")
                    },
                    {
                        "from": "gpt",
                        "value": item.get("output", "")
                    }
                ]
            }
            result.append(record)
        return result
    
    def _json_to_belle(self, data: List[Dict], **kwargs) -> List[Dict]:
        """转换为 BELLE 格式"""
        result = []
        for item in data:
            record = {
                "instruction": item.get("instruction", ""),
                "input": item.get("input", ""),
                "output": item.get("output", "")
            }
            result.append(record)
        return result
    
    # ==================== 文件操作 ====================
    
    def convert_file(self,
                    input_path: str,
                    output_path: str,
                    source_format: Optional[str] = None,
                    target_format: Optional[str] = None,
                    **kwargs) -> Dict:
        """转换文件格式
        
        Args:
            input_path: 输入文件路径
            output_path: 输出文件路径
            source_format: 源格式（为 None 时从扩展名推断）
            target_format: 目标格式（为 None 时从扩展名推断）
        
        Returns:
            转换结果
        """
        input_path = Path(input_path)
        output_path = Path(output_path)
        
        # 推断格式
        if source_format is None:
            source_format = self._infer_format(input_path)
        if target_format is None:
            target_format = self._infer_format(output_path)
        
        # 读取输入
        data = self._read_file(input_path, source_format)
        
        # 转换
        converted = self.convert(data, source_format, target_format, **kwargs)
        
        # 写入输出
        self._write_file(output_path, converted, target_format)
        
        return {
            "input_file": str(input_path),
            "output_file": str(output_path),
            "source_format": source_format,
            "target_format": target_format,
            "input_count": len(data),
            "output_count": len(converted) if isinstance(converted, list) else 1
        }
    
    def _infer_format(self, path: Path) -> str:
        """从文件扩展名推断格式"""
        ext_map = {
            ".json": "json",
            ".jsonl": "jsonl",
            ".csv": "csv",
            ".tsv": "tsv",
        }
        return ext_map.get(path.suffix.lower(), "json")
    
    def _read_file(self, path: Path, format: str) -> Any:
        """读取文件"""
        # `newline=""` 是 csv 模块的硬要求：不传时 Python 会先把 `\r\n` 归一成
        # `\n`，被引号包裹的字段内换行因此错位（写侧同理）。
        with open(path, 'r', encoding='utf-8', newline='') as f:
            if format == "jsonl":
                return [json.loads(line) for line in f if line.strip()]
            elif format == "csv":
                reader = csv.DictReader(f)
                return list(reader)
            else:
                return json.load(f)
    
    def _write_file(self, path: Path, data: Any, format: str) -> None:
        """写入文件

        `newline=""` 是 csv 模块的硬要求：默认文本模式会把 writer 的 `\\r\\n`
        行尾再翻译一次，Windows 上落成 `\\r\\r\\n`，严格解析器会在每条记录之间
        读出一个空行。`fieldnames` 取全量键并集而非 `data[0].keys()`，见
        `csv_fieldnames`。
        """
        path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(path, 'w', encoding='utf-8', newline='') as f:
            if format == "jsonl":
                for item in data:
                    f.write(json.dumps(item, ensure_ascii=False) + '\n')
            elif format == "csv":
                if data:
                    writer = csv.DictWriter(f, fieldnames=csv_fieldnames(data))
                    writer.writeheader()
                    writer.writerows(data)
            else:
                json.dump(data, f, ensure_ascii=False, indent=2)


def convert_dataset(data: List[Dict], 
                   target_format: str,
                   **kwargs) -> Any:
    """转换数据集格式
    
    Args:
        data: 数据列表
        target_format: 目标格式
    
    Returns:
        转换后的数据
    """
    converter = DatasetConverter()
    return converter.convert(data, "json", target_format, **kwargs)


def convert_file(input_path: str,
                output_path: str,
                target_format: Optional[str] = None,
                **kwargs) -> Dict:
    """转换文件格式
    
    Args:
        input_path: 输入文件路径
        output_path: 输出文件路径
        target_format: 目标格式
    
    Returns:
        转换结果
    """
    converter = DatasetConverter()
    return converter.convert_file(input_path, output_path, target_format=target_format, **kwargs)


def get_supported_formats() -> List[str]:
    """列出**转换图真的支持**的目标格式

    不能按 `DataFormat` 成员列：`TSV` 只是 `_infer_format` 认识的扩展名，图里
    没有 `json -> tsv` 这条边，`convert_dataset(items, "tsv")` 会抛
    `UnsupportedFormatError`。以枚举为来源会让这个公开出口虚报能力（CLI 的
    `convert --format` 一直刻意没放 tsv，正是同一事实的另一侧）。
    """
    converters = DatasetConverter()._converters
    supported = {"json"} | {target for source, target in converters if source == "json"}
    return [fmt.value for fmt in DataFormat if fmt.value in supported]
