"""流式处理模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.streaming import StreamReader, StreamWriter, StreamProcessor, StreamAugmentor


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
