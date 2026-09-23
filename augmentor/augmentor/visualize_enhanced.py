# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集可视化增强模块

提供数据集的可视化功能。

分工边界（与 `augmentor.visualizer`）
    本模块提供 `visualize_dataset(items, output, format)` 与 `VisualizationConfig`，
    CLI 的 `visualize --output`（报告模式）走这里；不带 `--output` 时走
    `augmentor.visualizer.DataVisualizer` 生成图表文件。
    走 pipeline 的可视化流程，用 `visualizer.DataVisualizer`。
"""

import json
import logging
from typing import List, Dict, Optional, Any
from pathlib import Path
from dataclasses import dataclass
from collections import Counter

logger = logging.getLogger(__name__)


@dataclass
class VisualizationConfig:
    """可视化配置"""
    width: int = 800
    height: int = 600
    theme: str = "default"
    color_palette: List[str] = None
    
    def __post_init__(self):
        if self.color_palette is None:
            self.color_palette = [
                "#4e79a7", "#f28e2b", "#e15759", "#76b7b2",
                "#59a14f", "#edc948", "#b07aa1", "#ff9da7",
                "#9c755f", "#bab0ac"
            ]


class EnhancedVisualizer:
    """增强可视化器
    
    提供数据集的可视化功能。
    """
    
    def __init__(self, config: VisualizationConfig = None):
        """初始化可视化器
        
        Args:
            config: 可视化配置
        """
        self._config = config or VisualizationConfig()
    
    def generate_text_report(self, items: List[Dict]) -> str:
        """生成文本报告
        
        Args:
            items: 数据列表
        
        Returns:
            文本报告
        """
        lines = [
            "=" * 60,
            "数据集可视化报告",
            "=" * 60,
            "",
            f"数据总量: {len(items)}",
            ""
        ]
        
        # 字段统计
        lines.append("字段统计:")
        lines.append("-" * 40)
        
        all_fields = set()
        for item in items:
            all_fields.update(item.keys())
        
        for field in sorted(all_fields):
            count = sum(1 for item in items if item.get(field))
            lines.append(f"  {field}: {count} 条 ({count/len(items)*100:.1f}%)")
        
        lines.append("")
        
        # 问题长度分布
        lines.append("问题长度分布:")
        lines.append("-" * 40)
        
        instructions = [item.get("instruction", "") for item in items if item.get("instruction")]
        if instructions:
            lengths = [len(inst) for inst in instructions]
            avg_length = sum(lengths) / len(lengths)
            min_length = min(lengths)
            max_length = max(lengths)
            
            lines.append(f"  平均长度: {avg_length:.1f} 字符")
            lines.append(f"  最短长度: {min_length} 字符")
            lines.append(f"  最长长度: {max_length} 字符")
            
            # 长度分布直方图
            bins = [0, 10, 20, 50, 100, 200, 500]
            counts = []
            for i in range(len(bins) - 1):
                count = sum(1 for l in lengths if bins[i] <= l < bins[i+1])
                counts.append((f"{bins[i]}-{bins[i+1]}", count))
            
            # 超过500的
            count_500_plus = sum(1 for l in lengths if l >= 500)
            if count_500_plus > 0:
                counts.append(("500+", count_500_plus))
            
            lines.append("  长度分布:")
            max_count = max(c for _, c in counts) if counts else 1
            for label, count in counts:
                bar_len = int(count / max_count * 30) if max_count > 0 else 0
                lines.append(f"    {label:>10}: {'#' * bar_len} ({count})")
        
        lines.append("")
        
        # 词汇统计
        lines.append("词汇统计:")
        lines.append("-" * 40)
        
        if instructions:
            # 简单分词
            all_words = []
            for inst in instructions:
                # 按字符分割（中文）
                words = [c for c in inst if '\u4e00' <= c <= '\u9fff']
                all_words.extend(words)
            
            word_freq = Counter(all_words)
            top_words = word_freq.most_common(20)
            
            if top_words:
                lines.append("  高频词汇 (前20):")
                max_freq = top_words[0][1] if top_words else 1
                for word, freq in top_words:
                    bar_len = int(freq / max_freq * 20)
                    lines.append(f"    {word}: {'#' * bar_len} ({freq})")
        
        lines.append("")
        lines.append("=" * 60)
        
        return "\n".join(lines)
    
    def generate_json_report(self, items: List[Dict]) -> Dict:
        """生成JSON报告
        
        Args:
            items: 数据列表
        
        Returns:
            JSON报告
        """
        report = {
            "total_items": len(items),
            "fields": {},
            "instruction_stats": {},
            "output_stats": {}
        }
        
        # 字段统计
        all_fields = set()
        for item in items:
            all_fields.update(item.keys())
        
        for field in sorted(all_fields):
            count = sum(1 for item in items if item.get(field))
            report["fields"][field] = {
                "count": count,
                "percentage": count / len(items) * 100 if items else 0
            }
        
        # 问题统计
        instructions = [item.get("instruction", "") for item in items if item.get("instruction")]
        if instructions:
            lengths = [len(inst) for inst in instructions]
            report["instruction_stats"] = {
                "count": len(instructions),
                "avg_length": sum(lengths) / len(lengths),
                "min_length": min(lengths),
                "max_length": max(lengths)
            }
        
        # 回答统计
        outputs = [item.get("output", "") for item in items if item.get("output")]
        if outputs:
            lengths = [len(out) for out in outputs]
            report["output_stats"] = {
                "count": len(outputs),
                "avg_length": sum(lengths) / len(lengths),
                "min_length": min(lengths),
                "max_length": max(lengths)
            }
        
        return report
    
    def save_text_report(self, items: List[Dict], output_path: str):
        """保存文本报告
        
        Args:
            items: 数据列表
            output_path: 输出路径
        """
        report = self.generate_text_report(items)
        
        output = Path(output_path)
        output.parent.mkdir(parents=True, exist_ok=True)
        
        with open(output, 'w', encoding='utf-8') as f:
            f.write(report)
    
    def save_json_report(self, items: List[Dict], output_path: str):
        """保存JSON报告
        
        Args:
            items: 数据列表
            output_path: 输出路径
        """
        report = self.generate_json_report(items)
        
        output = Path(output_path)
        output.parent.mkdir(parents=True, exist_ok=True)
        
        with open(output, 'w', encoding='utf-8') as f:
            json.dump(report, f, ensure_ascii=False, indent=2)


def visualize_dataset(items: List[Dict], output_path: str = None, format: str = "text") -> str:
    """可视化数据集
    
    Args:
        items: 数据列表
        output_path: 输出路径
        format: 输出格式 (text/json)
    
    Returns:
        可视化结果
    """
    visualizer = EnhancedVisualizer()
    
    if format == "json":
        report = visualizer.generate_json_report(items)
        if output_path:
            visualizer.save_json_report(items, output_path)
        return json.dumps(report, ensure_ascii=False, indent=2)
    else:
        report = visualizer.generate_text_report(items)
        if output_path:
            visualizer.save_text_report(items, output_path)
        return report
