"""流式处理模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.streaming import StreamReader, StreamWriter, StreamProcessor, StreamAugmentor, StreamConfig, create_stream_processor


@pytest.fixture
def sample_data():
    """创建测试数据"""
    return [
        {"instruction": "问题1", "input": "", "output": "回答1"},
        {"instruction": "问题2", "input": "", "output": "回答2"},
        {"instruction": "问题3", "input": "", "output": "回答3"},
        {"instruction": "问题4", "input": "", "output": "回答4"},
        {"instruction": "问题5", "input": "", "output": "回答5"},
    ]


@pytest.fixture
def sample_jsonl_data():
    """创建JSONL格式测试数据"""
    return [
        {"instruction": "问题A", "input": "", "output": "回答A"},
        {"instruction": "问题B", "input": "", "output": "回答B"},
        {"instruction": "问题C", "input": "", "output": "回答C"},
    ]


@pytest.fixture
def temp_json_file(tmp_path, sample_data):
    """创建临时JSON文件"""
    file_path = tmp_path / "test_data.json"
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(sample_data, f, ensure_ascii=False)
    return str(file_path)


@pytest.fixture
def temp_jsonl_file(tmp_path, sample_jsonl_data):
    """创建临时JSONL文件"""
    file_path = tmp_path / "test_data.jsonl"
    with open(file_path, 'w', encoding='utf-8') as f:
        for item in sample_jsonl_data:
            f.write(json.dumps(item, ensure_ascii=False) + '\n')
    return str(file_path)


class TestStreamConfig:
    """StreamConfig 测试"""
    
    def test_default_values(self):
        """测试默认值"""
        config = StreamConfig()
        assert config.chunk_size == 1000
        assert config.buffer_size == 10000
        assert config.max_memory_mb == 512
    
    def test_custom_values(self):
        """测试自定义值"""
        config = StreamConfig(chunk_size=500, buffer_size=5000, max_memory_mb=256)
        assert config.chunk_size == 500
        assert config.buffer_size == 5000
        assert config.max_memory_mb == 256


class TestStreamReaderCountJSONL:
    """StreamReader._count_items JSONL 路径测试"""

    def test_count_jsonl_fallback(self, temp_jsonl_file):
        """JSONL 文件计数应走 fallback 分支"""
        reader = StreamReader(temp_jsonl_file, chunk_size=2)
        count = reader._count_items()
        assert count == 3

    def test_count_caches_result(self, temp_jsonl_file):
        """计数结果应被缓存，重复调用不重算"""
        reader = StreamReader(temp_jsonl_file, chunk_size=2)
        first = reader._count_items()
        reader._total_count = -1
        second = reader._count_items()
        # 第二次调用使用缓存值（-1 证明不再重算）
        assert second == -1

    def test_count_non_list_json_returns_zero(self, tmp_path):
        """JSON 对象（非数组）格式应返回 0"""
        file_path = tmp_path / "obj.json"
        file_path.write_text('{"a": 1}', encoding='utf-8')
        reader = StreamReader(str(file_path), chunk_size=2)
        assert reader._count_items() == 0

    def test_count_blank_lines_ignored(self, tmp_path):
        """JSONL 中空白行不应计数"""
        file_path = tmp_path / "blank.jsonl"
        file_path.write_text('{"a":1}\n\n  \n{"b":2}\n\n', encoding='utf-8')
        reader = StreamReader(str(file_path), chunk_size=2)
        assert reader._count_items() == 2


class TestStreamWriterCloseEdges:
    """StreamWriter 关闭边界测试"""

    def test_close_json_writes_closing_bracket(self, tmp_path):
        """JSON 格式写入模式关闭时应补上 ]"""
        out = tmp_path / "out.json"
        with StreamWriter(str(out), format="json", mode="w") as writer:
            writer.write_chunk([{"a": 1}])
        content = out.read_text(encoding="utf-8")
        assert content.endswith("]")
        assert json.loads(content) == [{"a": 1}]

    def test_close_twice_is_safe(self, tmp_path):
        """重复关闭不应抛异常"""
        out = tmp_path / "out.jsonl"
        writer = StreamWriter(str(out), format="jsonl", mode="w")
        with writer:
            writer.write_chunk([{"a": 1}])
        writer.close()
        writer.close()
        assert writer._file is None

    def test_write_chunk_json_separator(self, tmp_path):
        """JSON 格式第二个 chunk 前应写入逗号分隔符"""
        out = tmp_path / "out.json"
        with StreamWriter(str(out), format="json", mode="w") as writer:
            writer.write_chunk([{"a": 1}])
            writer.write_chunk([{"b": 2}])
        assert json.loads(out.read_text(encoding="utf-8")) == [{"a": 1}, {"b": 2}]

    def test_write_empty_chunk(self, tmp_path):
        """空 chunk 不应影响文件内容"""
        out = tmp_path / "out.jsonl"
        with StreamWriter(str(out), format="jsonl", mode="w") as writer:
            writer.write_chunk([])
        assert out.read_text(encoding="utf-8").strip() == ""

    def test_write_chunk_unopened_raises(self, tmp_path):
        """未打开时写入应报错"""
        writer = StreamWriter(str(tmp_path / "x.jsonl"))
        with pytest.raises(RuntimeError):
            writer.write_chunk([{"a": 1}])


class TestStreamReader:
    """StreamReader 测试"""
    
    def test_init(self, temp_json_file):
        """测试初始化"""
        reader = StreamReader(temp_json_file, chunk_size=2)
        assert reader.file_path == Path(temp_json_file)
        assert reader.chunk_size == 2
    
    def test_read_chunks_json(self, temp_json_file, sample_data):
        """测试JSON格式分块读取"""
        reader = StreamReader(temp_json_file, chunk_size=2)
        chunks = list(reader.read_chunks())
        
        assert len(chunks) == 3  # 5条数据，每块2条，共3块
        assert chunks[0] == sample_data[:2]
        assert chunks[1] == sample_data[2:4]
        assert chunks[2] == sample_data[4:]
    
    def test_read_chunks_jsonl(self, temp_jsonl_file, sample_jsonl_data):
        """测试JSONL格式分块读取"""
        reader = StreamReader(temp_jsonl_file, chunk_size=2)
        chunks = list(reader.read_chunks())
        
        assert len(chunks) == 2  # 3条数据，每块2条，共2块
        assert chunks[0] == sample_jsonl_data[:2]
        assert chunks[1] == sample_jsonl_data[2:]
    
    def test_read_all(self, temp_json_file, sample_data):
        """测试读取全部数据"""
        reader = StreamReader(temp_json_file)
        data = reader.read_all()
        
        assert data == sample_data
    
    def test_total_count(self, temp_json_file, sample_data):
        """测试数据总数统计"""
        reader = StreamReader(temp_json_file)
        assert reader.total_count == len(sample_data)
    
    def test_chunk_size_larger_than_data(self, tmp_path, sample_data):
        """测试分块大小大于数据总量"""
        file_path = tmp_path / "small_data.json"
        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(sample_data[:2], f, ensure_ascii=False)
        
        reader = StreamReader(str(file_path), chunk_size=100)
        chunks = list(reader.read_chunks())
        
        assert len(chunks) == 1
        assert len(chunks[0]) == 2
    
    def test_count_items_caching(self, temp_json_file):
        """测试数据条数缓存"""
        reader = StreamReader(temp_json_file)
        
        # 第一次调用
        count1 = reader.total_count
        # 第二次调用应该使用缓存
        count2 = reader.total_count
        
        assert count1 == count2
        assert reader._total_count is not None


class TestStreamWriter:
    """StreamWriter 测试"""
    
    def test_write_jsonl(self, tmp_path, sample_data):
        """测试写入JSONL格式"""
        output_path = tmp_path / "output.jsonl"
        
        with StreamWriter(str(output_path), format='jsonl') as writer:
            writer.write_chunk(sample_data[:2])
            writer.write_chunk(sample_data[2:])
        
        # 验证写入内容
        with open(output_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        assert len(lines) == 5
        for i, line in enumerate(lines):
            item = json.loads(line)
            assert item == sample_data[i]
    
    def test_write_json(self, tmp_path, sample_data):
        """测试写入JSON格式"""
        output_path = tmp_path / "output.json"
        
        with StreamWriter(str(output_path), format='json') as writer:
            writer.write_chunk(sample_data)
        
        # 验证写入内容
        with open(output_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # JSON格式会有外层括号
        assert content.startswith('[')
        assert content.endswith(']')
    
    def test_write_json_multiple_chunks(self, tmp_path, sample_data):
        """测试JSON格式多次写入"""
        output_path = tmp_path / "output.json"
        
        with StreamWriter(str(output_path), format='json') as writer:
            writer.write_chunk(sample_data[:2])
            writer.write_chunk(sample_data[2:4])
        
        # 验证写入内容
        with open(output_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # 检查是否有逗号分隔
        assert ',' in content
    
    def test_write_chunk_without_open(self, tmp_path):
        """测试未打开写入器时写入"""
        output_path = tmp_path / "output.json"
        writer = StreamWriter(str(output_path))
        
        with pytest.raises(RuntimeError):
            writer.write_chunk([{"test": "data"}])
    
    def test_close_without_open(self, tmp_path):
        """测试关闭未打开的写入器"""
        output_path = tmp_path / "output.json"
        writer = StreamWriter(str(output_path))
        
        # 不应该抛出异常
        writer.close()


class TestStreamProcessor:
    """StreamProcessor 测试"""
    
    def test_process(self, temp_json_file, sample_data):
        """测试数据处理"""
        reader = StreamReader(temp_json_file, chunk_size=2)
        
        def process_func(items):
            return [{"processed": True, **item} for item in items]
        
        processor = StreamProcessor(reader, process_func)
        result = processor.process()
        
        assert result["total_input"] == 5
        assert result["total_output"] == 5
    
    def test_process_with_writer(self, tmp_path, temp_json_file, sample_data):
        """测试带写入器的数据处理"""
        reader = StreamReader(temp_json_file, chunk_size=2)
        output_path = tmp_path / "processed.jsonl"
        writer = StreamWriter(str(output_path), format='jsonl')
        
        def process_func(items):
            return [{"processed": True, **item} for item in items]
        
        processor = StreamProcessor(reader, process_func, writer)
        
        with writer:
            result = processor.process()
        
        assert result["total_input"] == 5
        assert result["total_output"] == 5
        
        # 验证写入内容
        with open(output_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        assert len(lines) == 5
    
    def test_process_empty_file(self, tmp_path):
        """测试处理空文件"""
        file_path = tmp_path / "empty.json"
        file_path.write_text('[]', encoding='utf-8')
        
        reader = StreamReader(str(file_path))
        
        def process_func(items):
            return [{"processed": True, **item} for item in items]
        
        processor = StreamProcessor(reader, process_func)
        result = processor.process()
        
        assert result["total_input"] == 0
        assert result["total_output"] == 0


class TestStreamAugmentor:
    """StreamAugmentor 测试"""
    
    def test_augment(self, tmp_path, temp_json_file, sample_data):
        """测试流式增强"""
        output_path = tmp_path / "augmented.jsonl"
        
        def process_func(items):
            return [{"augmented": True, **item} for item in items]
        
        augmentor = StreamAugmentor(
            input_file=temp_json_file,
            output_file=str(output_path),
            processor=process_func,
            chunk_size=2
        )
        
        result = augmentor.augment()
        
        assert result["total_input"] == 5
        assert result["total_output"] == 5
        
        # 验证输出文件
        with open(output_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        assert len(lines) == 5
        for line in lines:
            item = json.loads(line)
            assert item["augmented"] is True
    
    def test_augment_json_format(self, tmp_path, temp_json_file, sample_data):
        """测试JSON格式流式增强"""
        output_path = tmp_path / "augmented.json"
        
        def process_func(items):
            return [{"augmented": True, **item} for item in items]
        
        augmentor = StreamAugmentor(
            input_file=temp_json_file,
            output_file=str(output_path),
            processor=process_func,
            chunk_size=2,
            output_format='json'
        )
        
        result = augmentor.augment()
        
        assert result["total_input"] == 5
        assert result["total_output"] == 5
        
        # 验证输出文件
        with open(output_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        assert len(data) == 5


class TestStreamEdgeCases:
    """边界情况测试"""
    
    def test_empty_file(self, tmp_path):
        """测试空文件"""
        file_path = tmp_path / "empty.json"
        file_path.write_text('[]', encoding='utf-8')
        
        reader = StreamReader(str(file_path))
        chunks = list(reader.read_chunks())
        
        assert len(chunks) == 0
        assert reader.total_count == 0
    
    def test_single_item(self, tmp_path):
        """测试单条数据"""
        data = [{"instruction": "问题", "input": "", "output": "回答"}]
        file_path = tmp_path / "single.json"
        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(data, f, ensure_ascii=False)
        
        reader = StreamReader(str(file_path), chunk_size=10)
        chunks = list(reader.read_chunks())
        
        assert len(chunks) == 1
        assert len(chunks[0]) == 1
    
    def test_invalid_json_line(self, tmp_path):
        """测试无效JSON行"""
        file_path = tmp_path / "invalid.jsonl"
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('{"instruction": "问题1", "input": "", "output": "回答1"}\n')
            f.write('invalid json line\n')
            f.write('{"instruction": "问题2", "input": "", "output": "回答2"}\n')
        
        reader = StreamReader(str(file_path))
        chunks = list(reader.read_chunks())
        
        # 应该跳过无效行
        assert len(chunks) == 1
        assert len(chunks[0]) == 2


class TestCreateStreamProcessor:
    """create_stream_processor 工厂函数测试"""
    
    def test_create_stream_processor(self):
        """测试创建流式处理器"""
        def process_item(item):
            return {"processed": True, **item}
        
        stream_process = create_stream_processor(process_item, chunk_size=10)
        
        items = [{"instruction": "问题1"}, {"instruction": "问题2"}]
        result = stream_process(items)
        
        assert len(result) == 2
        assert all(item["processed"] is True for item in result)
    
    def test_create_stream_processor_with_error(self):
        """测试创建带错误处理的流式处理器"""
        def process_item(item):
            if "error" in item.get("instruction", ""):
                raise ValueError("处理错误")
            return {"processed": True, **item}
        
        stream_process = create_stream_processor(process_item)
        
        items = [
            {"instruction": "正常问题"},
            {"instruction": "error问题"},
            {"instruction": "另一个正常问题"}
        ]
        result = stream_process(items)
        
        # 应该跳过错误项
        assert len(result) == 2
        assert all(item["processed"] is True for item in result)
    
    def test_create_stream_processor_returns_none(self):
        """测试处理函数返回None的情况"""
        def process_item(item):
            if "skip" in item.get("instruction", ""):
                return None
            return {"processed": True, **item}
        
        stream_process = create_stream_processor(process_item)
        
        items = [
            {"instruction": "正常问题"},
            {"instruction": "skip问题"},
            {"instruction": "另一个正常问题"}
        ]
        result = stream_process(items)
        
        # 应该跳过返回None的项
        assert len(result) == 2


class TestStreamingExtended:
    """流式处理扩展测试"""

    def test_read_chunks_empty_file(self, tmp_path):
        """空文件分块读取"""
        file_path = tmp_path / "empty.json"
        file_path.write_text("[]", encoding="utf-8")
        reader = StreamReader(str(file_path), chunk_size=10)
        chunks = list(reader.read_chunks())
        assert chunks == []

    def test_read_chunks_single_item(self, tmp_path):
        """单条数据分块读取"""
        file_path = tmp_path / "single.json"
        file_path.write_text('[{"instruction": "q"}]', encoding="utf-8")
        reader = StreamReader(str(file_path), chunk_size=10)
        chunks = list(reader.read_chunks())
        assert len(chunks) == 1
        assert chunks[0] == [{"instruction": "q"}]

    def test_write_json_multiple_chunks(self, tmp_path):
        """多次写入 JSON 格式"""
        output_path = tmp_path / "output.json"
        with StreamWriter(str(output_path), format='json') as writer:
            writer.write_chunk([{"instruction": "q1"}])
            writer.write_chunk([{"instruction": "q2"}])
        with open(output_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        assert len(data) == 2

    def test_stream_processor_empty_input(self):
        """空输入流式处理"""
        def process_item(item):
            return item
        stream_process = create_stream_processor(process_item)
        result = stream_process([])
        assert result == []

    def test_stream_processor_all_fail(self):
        """全部处理失败"""
        def process_item(item):
            raise RuntimeError("always fail")
        stream_process = create_stream_processor(process_item)
        result = stream_process([{"instruction": "q1"}, {"instruction": "q2"}])
        assert result == []

    def test_stream_config_defaults(self):
        """StreamConfig 默认值"""
        config = StreamConfig()
        assert config.chunk_size > 0
        assert config.buffer_size > 0
        assert config.max_memory_mb > 0

    def test_read_all_jsonl(self, temp_jsonl_file, sample_jsonl_data):
        """读取全部 JSONL 数据"""
        reader = StreamReader(temp_jsonl_file)
        chunks = list(reader.read_chunks())
        total = sum(len(chunk) for chunk in chunks)
        assert total == len(sample_jsonl_data)

    def test_read_chunks_jsonl_format(self, tmp_path):
        """读取 JSONL 格式文件"""
        file_path = tmp_path / "test.jsonl"
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('{"instruction": "q1"}\n')
            f.write('{"instruction": "q2"}\n')
        reader = StreamReader(str(file_path), chunk_size=10)
        chunks = list(reader.read_chunks())
        assert len(chunks) == 1
        assert len(chunks[0]) == 2

    def test_stream_processor_partial_failure(self):
        """部分处理失败"""
        call_count = [0]
        def process_item(item):
            call_count[0] += 1
            if call_count[0] == 2:
                raise RuntimeError("fail")
            return item
        stream_process = create_stream_processor(process_item)
        result = stream_process([{"instruction": "q1"}, {"instruction": "q2"}, {"instruction": "q3"}])
        assert len(result) == 2

    def test_stream_writer_jsonl_format(self, tmp_path):
        """JSONL 格式写入"""
        output_path = tmp_path / "output.jsonl"
        with StreamWriter(str(output_path), format='jsonl') as writer:
            writer.write_chunk([{"instruction": "q1"}])
            writer.write_chunk([{"instruction": "q2"}])
        with open(output_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        assert len(lines) == 2

    def test_stream_config_chunk_size(self):
        """StreamConfig chunk_size"""
        config = StreamConfig(chunk_size=50)
        assert config.chunk_size == 50

    def test_stream_config_buffer_size(self):
        """StreamConfig buffer_size"""
        config = StreamConfig(buffer_size=1024)
        assert config.buffer_size == 1024
