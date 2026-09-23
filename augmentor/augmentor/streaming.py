# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""流式处理模块

提供大数据集的流式处理能力，支持分块读取、处理和写入。
"""

import json
import logging
from typing import List, Dict, Iterator, Callable, Optional, Generator, Any
from pathlib import Path
from dataclasses import dataclass
from .exceptions import StreamError, DataFormatError

try:
    from .memory_monitor import MemoryMonitor
    HAS_MEMORY_MONITOR = True
except ImportError:
    HAS_MEMORY_MONITOR = False

logger = logging.getLogger(__name__)

_WS = " \t\r\n"
_DEFAULT_READ_SIZE = 65536


def _peek_first_non_space(stream, read_size: int = 4096) -> Optional[str]:
    """返回流中第一个非空白字符

    .. note::
       本函数会**消费**流内容，调用方需自行 `seek(0)` 复位。

    Args:
        stream: 已打开的文本流
        read_size: 每次读取的字符数

    Returns:
        第一个非空白字符；流为空时返回 None
    """
    while True:
        chunk = stream.read(read_size)
        if not chunk:
            return None
        for ch in chunk:
            if ch not in _WS:
                return ch


def _iter_json_array_items(stream, read_size: int = _DEFAULT_READ_SIZE) -> Iterator[Any]:
    """增量解析 JSON 数组 `[...]`，逐条产出元素

    这是本模块「真流式」的核心：任何时刻内存里只有约一个 `read_size` 的缓冲
    加上当前元素，**不会把整个数组载入内存**。原实现用 `f.read()` + `json.loads`
    先整体载入再切片，10GB 的数组必然 OOM。

    元素边界交给 `json.JSONDecoder.raw_decode` 判定，因此字符串与嵌套结构内的
    逗号/括号不会被误当成分隔符。

    Args:
        stream: 已打开并定位到数组起始处的文本流
        read_size: 每次从流中读取的字符数

    Yields:
        数组中的每个元素

    Raises:
        DataFormatError: 内容不是合法 JSON 数组（如元素间缺少逗号、非法 JSON）
    """
    decoder = json.JSONDecoder()
    buf = ""
    pos = 0
    eof = False

    def fill() -> None:
        nonlocal buf, eof
        chunk = stream.read(read_size)
        if chunk:
            buf += chunk
        else:
            eof = True

    def compact() -> None:
        nonlocal buf, pos
        if pos:
            buf = buf[pos:]
            pos = 0

    def skip_ws() -> bool:
        """跳过空白（必要时补充缓冲）；返回是否仍有内容可读"""
        nonlocal buf, pos
        while True:
            while pos < len(buf) and buf[pos] in _WS:
                pos += 1
            if pos < len(buf):
                return True
            if eof:
                return False
            compact()
            fill()

    if not skip_ws() or buf[pos] != "[":
        raise DataFormatError("流式数组读取要求文件以 '[' 开头")

    pos += 1
    first = True

    while True:
        if not skip_ws():
            return  # 数组未闭合（文件被截断）：按已读到的元素结束
        if buf[pos] == "]":
            return
        if not first:
            if buf[pos] != ",":
                raise DataFormatError(
                    f"JSON 数组元素之间缺少逗号，实际遇到 {buf[pos]!r}"
                )
            pos += 1
            if not skip_ws():
                raise DataFormatError("JSON 数组在逗号之后意外结束")
            if buf[pos] == "]":
                return  # 容忍尾随逗号

        while True:
            try:
                value, end = decoder.raw_decode(buf, pos)
                break
            except json.JSONDecodeError:
                if eof:
                    raise
                fill()

        yield value
        pos = end
        first = False
        compact()


def _chunked(items: Iterator[Any], size: int) -> Iterator[List[Any]]:
    """把迭代器按 size 切分为列表块"""
    chunk: List[Any] = []
    for item in items:
        chunk.append(item)
        if len(chunk) >= size:
            yield chunk
            chunk = []
    if chunk:
        yield chunk


def _is_single_json_value(stream, read_size: int = _DEFAULT_READ_SIZE) -> bool:
    """判断整个流是否为「单个 JSON 值」（例如 `{"a": 1}`）

    用于保持 `_count_items` 的既有语义：单个 JSON 值计 0 条。
    对 JSONL 文件会在读到第二个值后立刻返回 False，代价很低。

    .. note::
       本函数会消费流内容，调用方需自行 `seek(0)` 复位。

    Args:
        stream: 已打开的文本流
        read_size: 每次读取的字符数

    Returns:
        整个流是否恰好包含一个 JSON 值
    """
    decoder = json.JSONDecoder()
    buf = ""
    pos = 0
    eof = False

    while True:
        while pos < len(buf) and buf[pos] in _WS:
            pos += 1
        if pos < len(buf):
            break
        if eof:
            return False
        more = stream.read(read_size)
        if not more:
            eof = True
        else:
            buf += more

    while True:
        try:
            _, end = decoder.raw_decode(buf, pos)
            break
        except json.JSONDecodeError:
            if eof:
                return False
            more = stream.read(read_size)
            if not more:
                eof = True
            else:
                buf += more

    pos = end
    while True:
        while pos < len(buf) and buf[pos] in _WS:
            pos += 1
        if pos < len(buf):
            return False  # 首个值之后仍有内容 → 不是单个值
        if eof:
            return True
        more = stream.read(read_size)
        if not more:
            eof = True
        else:
            buf += more


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
        """统计文件中的数据条数（流式，不整体载入内存）

        Returns:
            数据条数
        """
        if self._total_count is not None:
            return self._total_count

        with open(self.file_path, 'r', encoding='utf-8') as f:
            first = _peek_first_non_space(f)
            f.seek(0)

            if first == "[":
                # JSON 数组：增量解析计数（原实现会先 f.read() 整个文件）
                count = sum(1 for _ in _iter_json_array_items(f))
            elif first is None:
                count = 0
            elif _is_single_json_value(f):
                # 整个文件是单个 JSON 值（如 {"a": 1}）：保持既有语义，计 0 条
                count = 0
            else:
                # JSONL：逐行统计非空行
                f.seek(0)
                count = sum(1 for line in f if line.strip())

        self._total_count = count
        return count

    def read_chunks(self) -> Generator[List[Dict], None, None]:
        """分块读取数据（真流式：内存占用与文件大小无关）

        原实现先 `f.read()` 整个文件再 `json.loads`，最后才切片——名为流式，
        实际峰值内存等于整个数据集。现在数组走增量解析，JSONL 走逐行读取。

        Yields:
            数据块列表
        """
        with open(self.file_path, 'r', encoding='utf-8') as f:
            first = _peek_first_non_space(f)
            f.seek(0)

            if first == "[":
                # JSON 数组格式：增量解析，逐块产出
                yield from _chunked(_iter_json_array_items(f), self.chunk_size)
                return

            # JSONL 格式：逐行流式读取，跳过无效行
            chunk: List[Dict] = []
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    item = json.loads(line)
                except json.JSONDecodeError:
                    logger.warning(f"跳过无效行: {line[:100]}")
                    continue
                chunk.append(item)
                if len(chunk) >= self.chunk_size:
                    yield chunk
                    chunk = []

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
        # 只累计计数，不保留结果本身——原实现用 results.extend() 把全部输出
        # 留在内存里，与其自身「不累积全部结果到内存」的注释相矛盾，
        # 大数据集下等于把内存问题从读取端搬到了处理端。
        total_output = 0
        memory_monitor = MemoryMonitor(interval_mb=512) if HAS_MEMORY_MONITOR else None
        
        for chunk in self.reader.read_chunks():
            # 处理数据块
            processed_chunk = self.processor(chunk)
            
            # 内存优化：直接写入而非全部累积到内存
            if self.writer:
                self.writer.write_chunk(processed_chunk)
            
            total_output += len(processed_chunk)
            
            # 内存监控检查（集成优化）
            if memory_monitor is not None and len(chunk) > 100:
                memory_monitor.take_snapshot()
            
            self._processed_count += len(chunk)
            logger.info(f"已处理 {self._processed_count}/{self._total_count}")
        
        # 最终内存状态记录
        if memory_monitor is not None:
            memory_summary = memory_monitor.get_trend()
            logger.info(f"流式处理内存监控完成，峰值: {memory_monitor.get_peak_usage_mb():.2f} MB，趋势: {memory_summary}")
        
        return {
            "total_input": self._total_count,
            "total_output": total_output,
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
            raise StreamError("写入器未打开")
        
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
