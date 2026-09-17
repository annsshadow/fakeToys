"""数据集版本控制模块测试"""

import pytest
from augmentor.version_control import (
    DatasetVersionManager, DatasetVersion,
    create_version, load_version, list_versions
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


@pytest.fixture
def sample_dataset_v2():
    """创建测试数据集v2"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
        {"instruction": "如何申请买房？", "input": "", "output": "请咨询销售顾问"},
    ]


class TestDatasetVersionManager:
    """DatasetVersionManager 测试"""
    
    def test_init(self, tmp_path):
        """测试初始化"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        assert versions_dir.exists()
        assert manager._index_file.exists()
    
    def test_create_version(self, tmp_path, sample_dataset):
        """测试创建版本"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        version = manager.create_version(sample_dataset, "初始版本")
        
        assert isinstance(version, DatasetVersion)
        assert version.version_number == "1.0.0"
        assert version.item_count == 3
        assert version.description == "初始版本"
    
    def test_create_multiple_versions(self, tmp_path, sample_dataset, sample_dataset_v2):
        """测试创建多个版本"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        v1 = manager.create_version(sample_dataset, "v1")
        v2 = manager.create_version(sample_dataset_v2, "v2")
        
        assert v1.version_number == "1.0.0"
        assert v2.version_number == "1.0.1"
    
    def test_get_version(self, tmp_path, sample_dataset):
        """测试获取版本"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        version = manager.create_version(sample_dataset, "test")
        retrieved = manager.get_version(version.version_id)
        
        assert retrieved is not None
        assert retrieved.version_id == version.version_id
    
    def test_load_version(self, tmp_path, sample_dataset):
        """测试加载版本数据"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        version = manager.create_version(sample_dataset, "test")
        loaded = manager.load_version(version.version_id)
        
        assert len(loaded) == len(sample_dataset)
        assert loaded[0]["instruction"] == sample_dataset[0]["instruction"]
    
    def test_list_versions(self, tmp_path, sample_dataset):
        """测试列出版本"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        manager.create_version(sample_dataset, "v1")
        manager.create_version(sample_dataset, "v2")
        
        versions = manager.list_versions()
        
        assert len(versions) == 2
    
    def test_get_current_version(self, tmp_path, sample_dataset):
        """测试获取当前版本"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        version = manager.create_version(sample_dataset, "test")
        current = manager.get_current_version()
        
        assert current == version.version_id
    
    def test_set_current_version(self, tmp_path, sample_dataset):
        """测试设置当前版本"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        v1 = manager.create_version(sample_dataset, "v1")
        v2 = manager.create_version(sample_dataset, "v2")
        
        manager.set_current_version(v1.version_id)
        current = manager.get_current_version()
        
        assert current == v1.version_id
    
    def test_delete_version(self, tmp_path, sample_dataset):
        """测试删除版本"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        version = manager.create_version(sample_dataset, "test")
        
        assert manager.delete_version(version.version_id) is True
        assert manager.get_version(version.version_id) is None
    
    def test_compare_versions(self, tmp_path, sample_dataset, sample_dataset_v2):
        """测试比较版本"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        v1 = manager.create_version(sample_dataset, "v1")
        v2 = manager.create_version(sample_dataset_v2, "v2")
        
        result = manager.compare_versions(v1.version_id, v2.version_id)
        
        assert result["size_a"] == 3
        assert result["size_b"] == 4
        assert result["stats"]["only_in_b_count"] == 1


class TestDatasetVersion:
    """DatasetVersion 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        version = DatasetVersion(
            version_id="v1.0.0",
            version_number="1.0.0",
            timestamp="2024-01-01T00:00:00",
            description="test version",
            item_count=100,
            checksum="abc123",
            tags=["test"],
            metadata={"author": "test"}
        )
        
        d = version.to_dict()
        
        assert d["version_id"] == "v1.0.0"
        assert d["version_number"] == "1.0.0"
        assert d["item_count"] == 100
        assert d["tags"] == ["test"]


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_create_version(self, tmp_path, sample_dataset):
        """测试创建版本"""
        versions_dir = tmp_path / "versions"
        version = create_version(sample_dataset, str(versions_dir), "test")
        
        assert isinstance(version, DatasetVersion)
        assert version.item_count == 3
    
    def test_load_version(self, tmp_path, sample_dataset):
        """测试加载版本"""
        versions_dir = tmp_path / "versions"
        version = create_version(sample_dataset, str(versions_dir), "test")
        loaded = load_version(version.version_id, str(versions_dir))
        
        assert len(loaded) == 3
    
    def test_list_versions(self, tmp_path, sample_dataset):
        """测试列出版本"""
        versions_dir = tmp_path / "versions"
        create_version(sample_dataset, str(versions_dir), "v1")
        create_version(sample_dataset, str(versions_dir), "v2")
        
        versions = list_versions(str(versions_dir))
        
        assert len(versions) == 2


class TestVersionControlEdgeCases:
    """版本控制边界测试"""

    def test_calculate_checksum(self, tmp_path, sample_dataset):
        """校验和应一致"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        version = manager.create_version(sample_dataset, "test")
        data_file = versions_dir / version.version_id / "data.json"
        
        checksum1 = manager._calculate_checksum(str(data_file))
        checksum2 = manager._calculate_checksum(str(data_file))
        
        assert checksum1 == checksum2
        assert checksum1 == version.checksum

    def test_generate_version_number_carry(self, tmp_path, sample_dataset):
        """版本号进位测试"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        # 手动设置版本号为 1.0.9
        manager._index["versions"] = [{"version_number": "1.0.9", "version_id": "v1.0.9"}]
        next_ver = manager._generate_version_number()
        assert next_ver == "1.1.0"

    def test_generate_version_number_major_carry(self, tmp_path, sample_dataset):
        """主版本号进位测试"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        # 手动设置版本号为 1.9.9
        manager._index["versions"] = [{"version_number": "1.9.9", "version_id": "v1.9.9"}]
        next_ver = manager._generate_version_number()
        assert next_ver == "2.0.0"

    def test_generate_version_number_first(self, tmp_path):
        """首次创建版本号应为 1.0.0"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        ver = manager._generate_version_number()
        assert ver == "1.0.0"

    def test_set_current_version_invalid(self, tmp_path, sample_dataset):
        """设置不存在的版本为当前版本应返回 False"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        assert manager.set_current_version("nonexistent") is False

    def test_delete_current_version(self, tmp_path, sample_dataset):
        """删除当前版本后应更新当前版本"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        v1 = manager.create_version(sample_dataset, "v1")
        v2 = manager.create_version(sample_dataset, "v2")
        
        manager.delete_version(v2.version_id)
        assert manager.get_current_version() == v1.version_id

    def test_delete_last_version(self, tmp_path, sample_dataset):
        """删除最后一个版本后当前版本应为 None"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        v1 = manager.create_version(sample_dataset, "v1")
        manager.delete_version(v1.version_id)
        
        assert manager.get_current_version() is None

    def test_delete_nonexistent_version(self, tmp_path):
        """删除不存在的版本应返回 False"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        assert manager.delete_version("nonexistent") is False

    def test_load_version_nonexistent(self, tmp_path):
        """加载不存在的版本数据应抛出异常"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        with pytest.raises(FileNotFoundError):
            manager.load_version("nonexistent")

    def test_get_version_nonexistent(self, tmp_path):
        """获取不存在的版本信息应返回 None"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        assert manager.get_version("nonexistent") is None

    def test_compare_identical_versions(self, tmp_path, sample_dataset):
        """比较相同版本应返回空差异"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        v1 = manager.create_version(sample_dataset, "v1")
        v2 = manager.create_version(sample_dataset, "v2")
        
        result = manager.compare_versions(v1.version_id, v2.version_id)
        
        assert result["stats"]["only_in_a_count"] == 0
        assert result["stats"]["only_in_b_count"] == 0
        assert result["stats"]["in_both_count"] == 3

    def test_create_version_with_tags_and_metadata(self, tmp_path, sample_dataset):
        """创建版本时应保存标签和元数据"""
        versions_dir = tmp_path / "versions"
        manager = DatasetVersionManager(str(versions_dir))
        
        version = manager.create_version(
            sample_dataset, "test",
            tags=["release", "stable"],
            metadata={"author": "test_user"}
        )
        
        assert version.tags == ["release", "stable"]
        assert version.metadata == {"author": "test_user"}


class TestVersionControlExtended:
    """DatasetVersionManager 第二轮扩展测试（覆盖剩余分支）"""

    def test_version_numbers_increment(self, tmp_path, sample_dataset):
        """连续创建版本应递增 patch 号"""
        manager = DatasetVersionManager(str(tmp_path / "versions"))
        numbers = [
            manager.create_version(sample_dataset).version_number
            for _ in range(3)
        ]
        assert numbers == ["1.0.0", "1.0.1", "1.0.2"]

    def test_current_version_set_to_latest(self, tmp_path, sample_dataset):
        """最新版本应成为 current_version"""
        manager = DatasetVersionManager(str(tmp_path / "versions"))
        manager.create_version(sample_dataset, "old")
        latest = manager.create_version(sample_dataset, "new")
        assert manager.get_current_version() == latest.version_id

    def test_load_version_missing_raises(self, tmp_path, sample_dataset):
        """加载不存在的版本应报错"""
        manager = DatasetVersionManager(str(tmp_path / "versions"))
        with pytest.raises(FileNotFoundError):
            manager.load_version("v9.9.9")

    def test_get_version_data_missing_file(self, tmp_path):
        """版本目录缺失 data.json 时应报错"""
        manager = DatasetVersionManager(str(tmp_path / "versions"))
        with pytest.raises(FileNotFoundError):
            manager.load_version("v1.0.0")

    def test_list_versions_empty(self, tmp_path):
        """无版本时列表应为空"""
        manager = DatasetVersionManager(str(tmp_path / "versions"))
        assert manager.list_versions() == []

    def test_list_versions_returns_all(self, tmp_path, sample_dataset):
        """列表应包含全部已创建版本"""
        manager = DatasetVersionManager(str(tmp_path / "versions"))
        manager.create_version(sample_dataset, "1")
        manager.create_version(sample_dataset, "2")
        versions = manager.list_versions()
        assert len(versions) == 2
        # list_versions 返回字典列表
        assert [v["version_id"] for v in versions] == ["v1.0.0", "v1.0.1"]

    def test_version_to_dict_fields(self):
        """DatasetVersion.to_dict 应包含全部字段"""
        version = DatasetVersion(
            version_id="v1", version_number="1.0.0",
            timestamp="t", description="d", item_count=3,
            checksum="c", tags=["x"], metadata={"k": "v"}
        )
        d = version.to_dict()
        assert d["version_id"] == "v1"
        assert d["tags"] == ["x"]
        assert d["metadata"] == {"k": "v"}

    def test_create_version_convenience_function(self, tmp_path, sample_dataset):
        """便捷函数 create_version 应创建版本"""
        version = create_version(sample_dataset, str(tmp_path / "versions"), "desc")
        assert version.version_number == "1.0.0"

    def test_load_version_convenience_function(self, tmp_path, sample_dataset):
        """便捷函数 load_version 应加载数据"""
        manager = DatasetVersionManager(str(tmp_path / "versions"))
        version = manager.create_version(sample_dataset)
        data = load_version(version.version_id, str(tmp_path / "versions"))
        assert data == sample_dataset

    def test_list_versions_convenience_function(self, tmp_path, sample_dataset):
        """便捷函数 list_versions 应列出版本"""
        manager = DatasetVersionManager(str(tmp_path / "versions"))
        manager.create_version(sample_dataset)
        versions = list_versions(str(tmp_path / "versions"))
        assert len(versions) == 1

    def test_index_persistence_across_instances(self, tmp_path, sample_dataset):
        """索引应跨实例持久化"""
        dir_path = str(tmp_path / "versions")
        m1 = DatasetVersionManager(dir_path)
        m1.create_version(sample_dataset, "a")
        m2 = DatasetVersionManager(dir_path)
        assert len(m2.list_versions()) == 1
