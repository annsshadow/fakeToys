"""数据集依赖管理模块测试"""

import pytest
from augmentor.dependency import (
    DependencyManager, Dependency, DatasetInfo,
    register_dataset, add_dependency, get_dependency_graph
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
    ]


@pytest.fixture
def manager(tmp_path):
    """创建管理器"""
    return DependencyManager(str(tmp_path / "registry"))


class TestDependencyManager:
    """DependencyManager 测试"""
    
    def test_init(self, tmp_path):
        """测试初始化"""
        manager = DependencyManager(str(tmp_path / "registry"))
        assert (tmp_path / "registry").exists()
    
    def test_register_dataset(self, manager):
        """测试注册数据集"""
        info = manager.register_dataset("test_dataset", "/path/to/data.json", 100, "测试数据集")
        
        assert isinstance(info, DatasetInfo)
        assert info.name == "test_dataset"
        assert info.item_count == 100
    
    def test_get_dataset(self, manager):
        """测试获取数据集"""
        info = manager.register_dataset("test_dataset", "/path/to/data.json", 100)
        retrieved = manager.get_dataset(info.dataset_id)
        
        assert retrieved is not None
        assert retrieved.name == "test_dataset"
    
    def test_list_datasets(self, manager):
        """测试列出数据集"""
        manager.register_dataset("dataset1", "/path1", 100)
        manager.register_dataset("dataset2", "/path2", 200)
        
        datasets = manager.list_datasets()
        assert len(datasets) == 2
    
    def test_add_dependency(self, manager):
        """测试添加依赖"""
        info1 = manager.register_dataset("dataset1", "/path1", 100)
        info2 = manager.register_dataset("dataset2", "/path2", 200)
        
        dep = manager.add_dependency(info1.dataset_id, info2.dataset_id, "derived", "测试依赖")
        
        assert isinstance(dep, Dependency)
        assert dep.source_dataset == info1.dataset_id
        assert dep.target_dataset == info2.dataset_id
    
    def test_get_dependencies(self, manager):
        """测试获取依赖"""
        info1 = manager.register_dataset("dataset1", "/path1", 100)
        info2 = manager.register_dataset("dataset2", "/path2", 200)
        info3 = manager.register_dataset("dataset3", "/path3", 300)
        
        manager.add_dependency(info1.dataset_id, info2.dataset_id, "derived")
        manager.add_dependency(info2.dataset_id, info3.dataset_id, "derived")
        
        # 获取下游依赖
        deps = manager.get_dependencies(info1.dataset_id, "downstream")
        assert len(deps) == 1
        
        # 获取上游依赖
        deps = manager.get_dependencies(info3.dataset_id, "upstream")
        assert len(deps) == 1
        
        # 获取所有依赖
        deps = manager.get_dependencies(info2.dataset_id, "both")
        assert len(deps) == 2
    
    def test_remove_dependency(self, manager):
        """测试移除依赖"""
        info1 = manager.register_dataset("dataset1", "/path1", 100)
        info2 = manager.register_dataset("dataset2", "/path2", 200)
        
        manager.add_dependency(info1.dataset_id, info2.dataset_id, "derived")
        
        assert manager.remove_dependency(info1.dataset_id, info2.dataset_id) is True
        assert manager.get_dependencies(info1.dataset_id) == []
    
    def test_get_dependency_graph(self, manager):
        """测试获取依赖图"""
        info1 = manager.register_dataset("dataset1", "/path1", 100)
        info2 = manager.register_dataset("dataset2", "/path2", 200)
        
        manager.add_dependency(info1.dataset_id, info2.dataset_id, "derived")
        
        graph = manager.get_dependency_graph()
        
        assert "nodes" in graph
        assert "edges" in graph
        assert len(graph["nodes"]) == 2
        assert len(graph["edges"]) == 1
    
    def test_validate_dependencies(self, manager):
        """测试验证依赖"""
        info1 = manager.register_dataset("dataset1", "/path1", 100)
        info2 = manager.register_dataset("dataset2", "/path2", 200)
        
        manager.add_dependency(info1.dataset_id, info2.dataset_id, "derived")
        
        issues = manager.validate_dependencies()
        assert len(issues) == 0


class TestDatasetInfo:
    """DatasetInfo 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        info = DatasetInfo(
            dataset_id="test_id",
            name="test_dataset",
            description="测试数据集",
            path="/path/to/data.json",
            item_count=100,
            created_at="2024-01-01T00:00:00",
            updated_at="2024-01-01T00:00:00",
            tags=["test"],
            metadata={"author": "test"}
        )
        
        d = info.to_dict()
        
        assert d["dataset_id"] == "test_id"
        assert d["name"] == "test_dataset"
        assert d["item_count"] == 100


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_register_dataset(self, tmp_path):
        """测试注册数据集"""
        info = register_dataset("test", "/path", 100, registry_path=str(tmp_path / "registry"))
        
        assert isinstance(info, DatasetInfo)
        assert info.name == "test"
    
    def test_add_dependency(self, tmp_path):
        """测试添加依赖"""
        info1 = register_dataset("ds1", "/path1", 100, registry_path=str(tmp_path / "registry"))
        info2 = register_dataset("ds2", "/path2", 200, registry_path=str(tmp_path / "registry"))
        
        dep = add_dependency(info1.dataset_id, info2.dataset_id, "derived", 
                           registry_path=str(tmp_path / "registry"))
        
        assert isinstance(dep, Dependency)
    
    def test_get_dependency_graph(self, tmp_path):
        """测试获取依赖图"""
        graph = get_dependency_graph(str(tmp_path / "registry"))
        
        assert isinstance(graph, dict)
        assert "nodes" in graph
        assert "edges" in graph


class TestDependencyExtended:
    """DependencyManager 扩展测试"""

    def test_get_nonexistent_dataset(self, manager):
        """获取不存在的数据集"""
        assert manager.get_dataset("nonexistent") is None

    def test_remove_nonexistent_dependency(self, manager):
        """移除不存在的依赖"""
        result = manager.remove_dependency("a", "b")
        assert result is False

    def test_validate_empty_dependencies(self, manager):
        """验证空依赖"""
        issues = manager.validate_dependencies()
        assert len(issues) == 0

    def test_dependency_to_dict(self):
        """Dependency.to_dict"""
        dep = Dependency(
            source_dataset="s1", target_dataset="t1",
            dependency_type="derived", description="test"
        )
        d = dep.to_dict()
        assert d["source_dataset"] == "s1"
        assert d["target_dataset"] == "t1"

    def test_dataset_info_no_tags(self):
        """DatasetInfo 无 tags"""
        info = DatasetInfo(
            dataset_id="id1", name="n", description="d",
            path="/p", item_count=0, created_at="", updated_at=""
        )
        d = info.to_dict()
        assert d["tags"] == []
