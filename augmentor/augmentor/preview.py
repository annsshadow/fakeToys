"""导出格式预览模块

在真正导出前，预览数据经过格式转换后的结果，并给出潜在问题告警。
"""

import hashlib
import json
import logging
from dataclasses import dataclass, field
from typing import List, Dict, Any, Optional

from .export import Exporter, ExportFormat, NATIVE_FORMATS

logger = logging.getLogger(__name__)

# 每种格式在预览时的必要字段
REQUIRED_FIELDS: Dict[str, List[str]] = {
    "jsonl": ["instruction", "output"],
    "llama_factory": ["instruction", "output"],
    "alpaca": ["instruction", "output"],
    "sharegpt": ["instruction", "output"],
    "chatml": ["instruction", "output"],
    "csv": ["instruction", "output"]
}

# 预览时截断的文本长度
PREVIEW_TEXT_LIMIT = 200


@dataclass
class ExportPreview:
    """导出预览结果"""
    format: str = ""
    original_data: List[Dict] = field(default_factory=list)
    converted_data: List[Dict] = field(default_factory=list)
    format_info: Dict[str, Any] = field(default_factory=dict)
    warnings: List[str] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        """转换为字典

        Returns:
            预览字典
        """
        return {
            "format": self.format,
            "original_data": self.original_data,
            "converted_data": self.converted_data,
            "format_info": self.format_info,
            "warnings": self.warnings
        }


class PreviewGenerator:
    """导出格式预览生成器"""

    def __init__(self, preview_size: int = 5):
        """初始化预览生成器

        Args:
            preview_size: 预览样本条数
        """
        self.preview_size = preview_size

    def _truncate(self, value: Any) -> Any:
        """截断过长的文本字段

        Args:
            value: 字段值

        Returns:
            截断后的值
        """
        if isinstance(value, str) and len(value) > PREVIEW_TEXT_LIMIT:
            return value[:PREVIEW_TEXT_LIMIT] + "..."
        return value

    def _truncate_record(self, record: Any) -> Any:
        """递归截断记录中的文本

        Args:
            record: 记录（字典 / 列表 / 标量）

        Returns:
            截断后的记录
        """
        if isinstance(record, dict):
            return {k: self._truncate_record(v) for k, v in record.items()}
        if isinstance(record, list):
            return [self._truncate_record(v) for v in record]
        return self._truncate(record)

    def _collect_warnings(self, items: List[Dict], fmt: str) -> List[str]:
        """检查数据并收集告警

        Args:
            items: 数据列表
            fmt: 目标格式

        Returns:
            告警列表
        """
        warnings: List[str] = []

        if not items:
            warnings.append("数据集为空，导出结果将不含任何记录")
            return warnings

        required = REQUIRED_FIELDS.get(fmt, [])
        for field_name in required:
            missing = sum(1 for item in items if not item.get(field_name))
            if missing:
                warnings.append(
                    f"字段 '{field_name}' 在 {missing}/{len(items)} 条记录中为空"
                )

        # 历史对话相关字段仅对 sharegpt / chatml 有意义
        if fmt in ("sharegpt", "chatml"):
            with_history = sum(1 for item in items if item.get("history"))
            if with_history == 0:
                warnings.append("未检测到 history 字段，多轮对话格式将退化为单轮")

        return warnings

    def preview(self,
                items: List[Dict],
                format: Optional[str] = None,
                preview_size: Optional[int] = None) -> ExportPreview:
        """生成导出预览

        Args:
            items: 数据列表
            format: 目标格式，为 None 时使用 jsonl
            preview_size: 预览条数，为 None 时使用默认值

        Returns:
            ExportPreview 实例

        Raises:
            ValueError: 不支持的导出格式
        """
        fmt = format or "jsonl"
        try:
            export_format = ExportFormat(fmt)
        except ValueError:
            raise ValueError(
                f"不支持的导出格式: {fmt}。支持: {[f.value for f in NATIVE_FORMATS]}"
            )

        size = preview_size or self.preview_size
        sample = items[:size]

        exporter = Exporter(default_format=fmt)
        converter = {
            ExportFormat.JSONL: exporter._convert_to_jsonl,
            ExportFormat.LLAMA_FACTORY: exporter._convert_to_llama_factory,
            ExportFormat.ALPACA: exporter._convert_to_alpaca,
            ExportFormat.SHARE_GPT: exporter._convert_to_sharegpt,
            ExportFormat.CHATML: exporter._convert_to_chatml,
            ExportFormat.CSV: exporter._convert_to_csv
        }.get(export_format)

        # ExportFormat 统一后含 13 个成员，而预览只实现了 6 种转换。
        # 必须用 .get() 而非 []：否则传 openai / tsv 等会抛 KeyError，
        # 而不是这里约定的 ValueError。
        if converter is None:
            raise ValueError(
                f"不支持的导出格式: {fmt}。支持: {[f.value for f in NATIVE_FORMATS]}"
            )

        raw_converted = converter(sample)

        # jsonl 转换结果为字符串行，解析回字典以便统一展示
        if export_format == ExportFormat.JSONL:
            converted = [json.loads(line) for line in raw_converted]
        else:
            converted = raw_converted

        # 预览生成优化：缓存预览结果（避免相同数据重复生成预览）
        preview_key = hashlib.md5(str(sample[:min(5, len(sample))]).encode()).hexdigest()[:8]
        if hasattr(self, '_preview_cache') and preview_key in self._preview_cache:
            logger.debug(f"预览缓存命中（优化）: 格式 {fmt}")
            cached = self._preview_cache[preview_key]
            # 更新总数信息（数据可能不同但预览内容相同）
            cached.format_info["total_items"] = len(items)
            cached.format_info["preview_items"] = len(sample)
            return cached
        
        converted = [self._truncate_record(record) for record in converted]
        
        result = ExportPreview(
            format=fmt,
            original_data=[self._truncate_record(item) for item in sample],
            converted_data=converted,
            format_info={
                "format": fmt,
                "total_items": len(items),
                "preview_items": len(sample),
                "fields": list(converted[0].keys()) if converted else [],
                "required_fields": REQUIRED_FIELDS.get(fmt, [])
            },
            warnings=self._collect_warnings(items, fmt)
        )
        
        # 保存预览结果到缓存（优化：避免重复生成相同预览）
        if not hasattr(self, '_preview_cache'):
            self._preview_cache = {}
        self._preview_cache[preview_key] = result
        
        return result
