"""多格式导出模块 - 优化版"""

import json
import csv
import logging
from typing import List, Dict, Optional
from pathlib import Path
from enum import Enum
from concurrent.futures import ThreadPoolExecutor, as_completed

logger = logging.getLogger(__name__)


class ExportFormat(Enum):
    """导出格式枚举"""
    JSONL = "jsonl"
    LLAMA_FACTORY = "llama_factory"
    ALPACA = "alpaca"
    SHARE_GPT = "sharegpt"
    CHATML = "chatml"
    CSV = "csv"


class Exporter:
    """数据导出器 - 优化版"""
    
    def __init__(self, default_format: str = "jsonl", max_workers: int = 4):
        """初始化导出器
        
        Args:
            default_format: 默认导出格式
            max_workers: 最大并发数
        """
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
            format: 导出格式，为 None 时使用默认格式
        
        Returns:
            实际使用的格式
        """
        export_format = ExportFormat(format) if format else self.default_format
        
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
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
        
        elif export_format == ExportFormat.SHAREGPT:
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
        
        else:
            raise ValueError(f"不支持的导出格式: {export_format}")
        
        logger.info(f"导出 {len(items)} 条数据到 {output_path}，格式: {export_format.value}")
        return export_format.value
    
    def export_all_formats(self, 
                          items: List[Dict],
                          output_dir: str,
                          base_name: str = "train_data",
                          use_parallel: bool = True) -> Dict[str, str]:
        """导出所有格式（优化版）
        
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
            if fmt == ExportFormat.CSV:
                ext = ".csv"
            else:
                ext = ".json"
            
            output_path = output_dir / f"{base_name}_{fmt.value}{ext}"
            self.export(items, str(output_path), fmt.value)
            return fmt.value, str(output_path)
        
        if not use_parallel:
            # 串行处理
            for fmt in ExportFormat:
                fmt_name, path = export_single(fmt)
                results[fmt_name] = path
        else:
            # 并行处理
            with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
                futures = [executor.submit(export_single, fmt) for fmt in ExportFormat]
                
                for future in as_completed(futures):
                    try:
                        fmt_name, path = future.result()
                        results[fmt_name] = path
                        logger.info(f"导出格式 {fmt_name} 完成")
                    except Exception as e:
                        logger.error(f"导出失败: {e}")
        
        return results
    
    def get_supported_formats(self) -> List[str]:
        """获取支持的导出格式
        
        Returns:
            格式名称列表
        """
        return [fmt.value for fmt in ExportFormat]
