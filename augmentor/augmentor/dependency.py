# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集依赖管理模块

提供数据集依赖关系管理功能。
"""

import json
import logging
from typing import List, Dict, Optional, Any, Set
from dataclasses import dataclass, field
from pathlib import Path
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class Dependency:
    """依赖关系"""
    source_dataset: str
    target_dataset: str
    dependency_type: str  # derived, merged, filtered, transformed
    description: str = ""
    created_at: str = ""
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "source_dataset": self.source_dataset,
            "target_dataset": self.target_dataset,
            "dependency_type": self.dependency_type,
            "description": self.description,
            "created_at": self.created_at
        }


@dataclass
class DatasetInfo:
    """数据集信息"""
    dataset_id: str
    name: str
    description: str
    path: str
    item_count: int
    created_at: str
    updated_at: str
    tags: List[str] = field(default_factory=list)
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "dataset_id": self.dataset_id,
            "name": self.name,
            "description": self.description,
            "path": self.path,
            "item_count": self.item_count,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
            "tags": self.tags,
            "metadata": self.metadata
        }


class DependencyManager:
    """依赖管理器
    
    提供数据集依赖关系管理功能。
    """
    
    def __init__(self, registry_path: str = ".dependency_registry"):
        """初始化依赖管理器
        
        Args:
            registry_path: 注册表路径
        """
        self._registry_path = Path(registry_path)
        self._registry_path.mkdir(parents=True, exist_ok=True)
        
        self._datasets_file = self._registry_path / "datasets.json"
        self._dependencies_file = self._registry_path / "dependencies.json"
        
        loaded_datasets = self._load_file(self._datasets_file, {})
        self._datasets: Dict[str, DatasetInfo] = {
            k: DatasetInfo(**v) if isinstance(v, dict) else v
            for k, v in loaded_datasets.items()
        }
        self._dependencies: List[Dependency] = self._load_dependencies()
    
    def _load_file(self, path: Path, default: Any) -> Any:
        """加载文件
        
        Args:
            path: 文件路径
            default: 默认值
        
        Returns:
            文件内容
        """
        if path.exists():
            with open(path, 'r', encoding='utf-8') as f:
                return json.load(f)
        return default
    
    def _load_dependencies(self) -> List[Dependency]:
        """加载依赖关系
        
        Returns:
            依赖列表
        """
        data = self._load_file(self._dependencies_file, [])
        return [Dependency(**dep) for dep in data]
    
    def _save_datasets(self):
        """保存数据集信息"""
        data = {k: v.to_dict() for k, v in self._datasets.items()}
        with open(self._datasets_file, 'w', encoding='utf-8') as f:
            json.dump(data, f, ensure_ascii=False, indent=2)
    
    def _save_dependencies(self):
        """保存依赖关系"""
        data = [dep.to_dict() for dep in self._dependencies]
        with open(self._dependencies_file, 'w', encoding='utf-8') as f:
            json.dump(data, f, ensure_ascii=False, indent=2)
    
    def register_dataset(self, name: str, path: str, item_count: int,
                        description: str = "", tags: List[str] = None,
                        metadata: Dict[str, Any] = None) -> DatasetInfo:
        """注册数据集
        
        Args:
            name: 数据集名称
            path: 数据集路径
            item_count: 数据数量
            description: 描述
            tags: 标签
            metadata: 元数据
        
        Returns:
            数据集信息
        """
        import hashlib
        
        dataset_id = hashlib.md5(name.encode()).hexdigest()[:8]
        now = datetime.now().isoformat()
        
        info = DatasetInfo(
            dataset_id=dataset_id,
            name=name,
            description=description,
            path=path,
            item_count=item_count,
            created_at=now,
            updated_at=now,
            tags=tags or [],
            metadata=metadata or {}
        )
        
        self._datasets[dataset_id] = info
        self._save_datasets()
        
        logger.info(f"数据集注册成功: {name} ({dataset_id})")
        return info
    
    def get_dataset(self, dataset_id: str) -> Optional[DatasetInfo]:
        """获取数据集信息
        
        Args:
            dataset_id: 数据集ID
        
        Returns:
            数据集信息
        """
        return self._datasets.get(dataset_id)
    
    def list_datasets(self) -> List[DatasetInfo]:
        """列出所有数据集
        
        Returns:
            数据集列表
        """
        return list(self._datasets.values())
    
    def add_dependency(self, source_dataset: str, target_dataset: str,
                      dependency_type: str, description: str = "") -> Dependency:
        """添加依赖关系
        
        Args:
            source_dataset: 源数据集
            target_dataset: 目标数据集
            dependency_type: 依赖类型
            description: 描述
        
        Returns:
            依赖关系
        """
        dep = Dependency(
            source_dataset=source_dataset,
            target_dataset=target_dataset,
            dependency_type=dependency_type,
            description=description,
            created_at=datetime.now().isoformat()
        )
        
        self._dependencies.append(dep)
        self._save_dependencies()
        
        logger.info(f"依赖关系添加成功: {source_dataset} -> {target_dataset}")
        return dep
    
    def get_dependencies(self, dataset_id: str, direction: str = "both") -> List[Dependency]:
        """获取依赖关系
        
        Args:
            dataset_id: 数据集ID
            direction: 方向 (upstream, downstream, both)
        
        Returns:
            依赖列表
        """
        result = []
        
        for dep in self._dependencies:
            if direction == "upstream" and dep.target_dataset == dataset_id:
                result.append(dep)
            elif direction == "downstream" and dep.source_dataset == dataset_id:
                result.append(dep)
            elif direction == "both" and (dep.source_dataset == dataset_id or dep.target_dataset == dataset_id):
                result.append(dep)
        
        return result
    
    def remove_dependency(self, source_dataset: str, target_dataset: str) -> bool:
        """移除依赖关系
        
        Args:
            source_dataset: 源数据集
            target_dataset: 目标数据集
        
        Returns:
            是否成功移除
        """
        original_count = len(self._dependencies)
        self._dependencies = [
            dep for dep in self._dependencies
            if not (dep.source_dataset == source_dataset and dep.target_dataset == target_dataset)
        ]
        
        if len(self._dependencies) < original_count:
            self._save_dependencies()
            logger.info(f"依赖关系移除成功: {source_dataset} -> {target_dataset}")
            return True
        
        return False
    
    def get_dependency_graph(self) -> Dict:
        """获取依赖图
        
        Returns:
            依赖图
        """
        graph = {
            "nodes": [],
            "edges": []
        }
        
        # 添加节点
        for dataset_id, info in self._datasets.items():
            graph["nodes"].append({
                "id": dataset_id,
                "name": info.name,
                "item_count": info.item_count
            })
        
        # 添加边
        for dep in self._dependencies:
            graph["edges"].append({
                "source": dep.source_dataset,
                "target": dep.target_dataset,
                "type": dep.dependency_type
            })
        
        return graph
    
    def validate_dependencies(self) -> List[Dict]:
        """验证依赖关系
        
        Returns:
            验证结果
        """
        issues = []
        
        for dep in self._dependencies:
            # 检查源数据集是否存在
            if dep.source_dataset not in self._datasets:
                issues.append({
                    "type": "missing_source",
                    "dependency": dep.to_dict(),
                    "message": f"源数据集不存在: {dep.source_dataset}"
                })
            
            # 检查目标数据集是否存在
            if dep.target_dataset not in self._datasets:
                issues.append({
                    "type": "missing_target",
                    "dependency": dep.to_dict(),
                    "message": f"目标数据集不存在: {dep.target_dataset}"
                })
        
        return issues


def register_dataset(name: str, path: str, item_count: int,
                    description: str = "", tags: List[str] = None,
                    registry_path: str = ".dependency_registry") -> DatasetInfo:
    """注册数据集
    
    Args:
        name: 数据集名称
        path: 数据集路径
        item_count: 数据数量
        description: 描述
        tags: 标签
        registry_path: 注册表路径
    
    Returns:
        数据集信息
    """
    manager = DependencyManager(registry_path)
    return manager.register_dataset(name, path, item_count, description, tags)


def add_dependency(source_dataset: str, target_dataset: str,
                  dependency_type: str, description: str = "",
                  registry_path: str = ".dependency_registry") -> Dependency:
    """添加依赖关系
    
    Args:
        source_dataset: 源数据集
        target_dataset: 目标数据集
        dependency_type: 依赖类型
        description: 描述
        registry_path: 注册表路径
    
    Returns:
        依赖关系
    """
    manager = DependencyManager(registry_path)
    return manager.add_dependency(source_dataset, target_dataset, dependency_type, description)


def get_dependency_graph(registry_path: str = ".dependency_registry") -> Dict:
    """获取依赖图
    
    Args:
        registry_path: 注册表路径
    
    Returns:
        依赖图
    """
    manager = DependencyManager(registry_path)
    return manager.get_dependency_graph()
