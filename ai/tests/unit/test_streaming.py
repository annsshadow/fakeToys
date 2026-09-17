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
