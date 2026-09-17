"""流式处理模块

提供大数据集的流式处理能力，支持分块读取、处理和写入。
"""

import json
import logging
from typing import List, Dict, Iterator, Callable, Optional, Generator
from pathlib import Path
from dataclasses import dataclass

logger = logging.getLogger(__name__)


@dataclass
class StreamConfig:
    """流式处理配置"""
    chunk_size: int = 1000  # 每次处理的数据条数
    buffer_size: int = 10000  # 写入缓冲区大小
    max_memory_mb: int = 512  # 最大内存使用（MB）


class StreamReader:
    """数据流读取器
    
    支持分块读取JSON文件，避免一次性加载全部数据到内存。
    """
    
    def __init__(self, file_path: str, chunk_size: int = 1000):
        """初始化读取器
        
        Args:
            file_path: 文件路径
            chunk_size: 每次读取的数据条数
        """
        self.file_path = Path(file_path)
        self.chunk_size = chunk_size
        self._total_count: Optional[int] = None
    
    def _count_items(self) -> int:
        """统计文件中的数据条数
        
        Returns:
            数据条数
        """
        if self._total_count is not None:
            return self._total_count
        
        count = 0
        with open(self.file_path, 'r', encoding='utf-8') as f:
            # 尝试解析JSON数组
            content = f.read()
            try:
                data = json.loads(content)
                if isinstance(data, list):
                    count = len(data)
            except json.JSONDecodeError:
                # 尝试JSONL格式
                for line in content.strip().split('\n'):
                    if line.strip():
                        count += 1
        
        self._total_count = count
        return count
    
    def read_chunks(self) -> Generator[List[Dict], None, None]:
        """分块读取数据
        
        Yields:
            数据块列表
        """
        with open(self.file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        try:
            data = json.loads(content)
            if isinstance(data, list):
                # JSON数组格式
                for i in range(0, len(data), self.chunk_size):
                    yield data[i:i + self.chunk_size]
                return
        except json.JSONDecodeError:
            pass
        
        # JSONL格式
        chunk = []
        with open(self.file_path, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line:
                    try:
                        item = json.loads(line)
                        chunk.append(item)
                        if len(chunk) >= self.chunk_size:
                            yield chunk
                            chunk = []
                    except json.JSONDecodeError:
                        logger.warning(f"跳过无效行: {line[:100]}")
        
        if chunk:
            yield chunk
    
    def read_all(self) -> List[Dict]:
        """读取全部数据（仅适用于小数据集）
        
        Returns:
            数据列表
        """
        with open(self.file_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    
    @property
    def total_count(self) -> int:
        """获取数据总数"""
        return self._count_items()


class StreamProcessor:
    """数据流处理器
    
    支持对数据进行流式处理，边读取边处理边写入。
    """
    
    def __init__(self, 
                 reader: StreamReader,
                 processor: Callable[[List[Dict]], List[Dict]],
                 writer: Optional['StreamWriter'] = None):
        """初始化处理器
        
        Args:
            reader: 数据读取器
            processor: 处理函数，接收数据块，返回处理后的数据块
            writer: 数据写入器，为 None 时不写入
        """
        self.reader = reader
        self.processor = processor
        self.writer = writer
        self._processed_count = 0
        self._total_count = reader.total_count
    
    def process(self) -> Dict:
        """执行流式处理
        
        Returns:
            处理报告
        """
        self._processed_count = 0
        results = []
        
        for chunk in self.reader.read_chunks():
            # 处理数据块
            processed_chunk = self.processor(chunk)
            results.extend(processed_chunk)
            
            # 写入数据
            if self.writer:
                self.writer.write_chunk(processed_chunk)
            
            self._processed_count += len(chunk)
            logger.info(f"已处理 {self._processed_count}/{self._total_count}")
        
        return {
            "total_input": self._total_count,
            "total_output": len(results),
            "processed": self._processed_count
        }


class StreamWriter:
    """数据流写入器
    
    支持分块写入数据到文件。
    """
    
    def __init__(self, 
                 file_path: str,
                 mode: str = 'w',
                 format: str = 'json'):
        """初始化写入器
        
        Args:
            file_path: 文件路径
            mode: 文件模式 ('w' 写入, 'a' 追加)
            format: 输出格式 ('json' 或 'jsonl')
        """
        self.file_path = Path(file_path)
        self.mode = mode
        self.format = format
        self._file = None
        self._first_chunk = True
    
    def __enter__(self):
        self._file = open(self.file_path, self.mode, encoding='utf-8')
        if self.format == 'json' and self.mode == 'w':
            self._file.write('[')
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        if self._file:
            if self.format == 'json' and self.mode == 'w':
                self._file.write(']')
            self._file.close()
            self._file = None
    
    def write_chunk(self, chunk: List[Dict]):
        """写入数据块
        
        Args:
            chunk: 数据块列表
        """
        if not self._file:
            raise RuntimeError("写入器未打开")
        
        for item in chunk:
            if self.format == 'jsonl':
                line = json.dumps(item, ensure_ascii=False)
                self._file.write(line + '\n')
            else:  # json
                if not self._first_chunk:
                    self._file.write(',')
                self._file.write(json.dumps(item, ensure_ascii=False))
                self._first_chunk = False
        
        self._file.flush()
    
    def close(self):
        """关闭写入器"""
        if self._file:
            if self.format == 'json' and self.mode == 'w':
                self._file.write(']')
            self._file.close()
            self._file = None


class StreamAugmentor:
    """流式增强器
    
    结合读取器、处理器和写入器的高级接口。
    """
    
    def __init__(self, 
                 input_file: str,
                 output_file: str,
                 processor: Callable[[List[Dict]], List[Dict]],
                 chunk_size: int = 1000,
                 output_format: str = 'jsonl'):
        """初始化流式增强器
        
        Args:
            input_file: 输入文件路径
            output_file: 输出文件路径
            processor: 处理函数
            chunk_size: 分块大小
            output_format: 输出格式
        """
        self.reader = StreamReader(input_file, chunk_size)
        self.writer = StreamWriter(output_file, format=output_format)
        self.processor = StreamProcessor(self.reader, processor, self.writer)
    
    def augment(self) -> Dict:
        """执行流式增强
        
        Returns:
            处理报告
        """
        with self.writer:
            return self.processor.process()


def create_stream_processor(processor_func: Callable,
                           chunk_size: int = 1000) -> Callable:
    """创建流式处理器工厂
    
    Args:
        processor_func: 单条数据处理函数
        chunk_size: 分块大小
    
    Returns:
        流式处理函数
    """
    def stream_process(items: List[Dict]) -> List[Dict]:
        results = []
        for item in items:
            try:
                result = processor_func(item)
                if result is not None:
                    results.append(result)
            except Exception as e:
                logger.warning(f"处理数据失败: {e}")
        return results
    
    return stream_process
