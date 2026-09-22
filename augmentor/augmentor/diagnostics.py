# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""运行时诊断模块

探测各可选依赖与运行能力，返回结构化诊断报告，便于部署前置检查
与问题定位（哪些增强特性可用、哪些已降级）。
"""

import importlib
import logging
from dataclasses import dataclass, field
from typing import Dict, List

logger = logging.getLogger(__name__)

# 可选依赖：名称 -> (导入模块, 是否核心)
OPTIONAL_DEPENDENCIES: Dict[str, str] = {
    "pandas": "pandas",
    "openpyxl": "openpyxl",
    "faiss": "faiss",
    "psutil": "psutil",
    "sentence_transformers": "sentence_transformers",
    "sklearn": "sklearn",
    "chromadb": "chromadb",
    "yaml": "yaml",
    "PIL": "PIL",
}

REQUIRED_DEPENDENCIES: List[str] = ["yaml"]


@dataclass
class DiagnosticsReport:
    """诊断报告"""

    installed: Dict[str, bool] = field(default_factory=dict)
    missing: List[str] = field(default_factory=list)
    degraded_features: List[str] = field(default_factory=list)

    @property
    def all_required_present(self) -> bool:
        return all(self.installed.get(dep, False) for dep in REQUIRED_DEPENDENCIES)

    @property
    def available_count(self) -> int:
        return sum(1 for v in self.installed.values() if v)

    def to_dict(self) -> Dict:
        return {
            "installed": dict(self.installed),
            "missing": list(self.missing),
            "degraded_features": list(self.degraded_features),
            "all_required_present": self.all_required_present,
            "available_count": self.available_count,
        }


def _importable(module_name: str) -> bool:
    """判断模块是否可导入

    Args:
        module_name: 模块名

    Returns:
        是否可导入
    """
    try:
        importlib.import_module(module_name)
        return True
    except Exception:
        return False


# 各可选依赖缺失时降级的能力（用于提示）
_DEGRADED_HINTS = {
    "pandas": "CSV/Excel 导入导出不可用",
    "openpyxl": "Excel 写入不可用",
    "faiss": "向量检索退化为 numpy 精确检索",
    "psutil": "内存监控使用估算值",
    "sentence_transformers": "语义相似度退化为词元 Jaccard",
    "sklearn": "主题聚类退化为简化分析",
    "chromadb": "ChromaDB 向量后端不可用",
    "PIL": "图像处理退化为文件头解析",
}


def check_dependencies() -> DiagnosticsReport:
    """探测可选依赖可用性并给出降级提示

    Returns:
        DiagnosticsReport 实例
    """
    report = DiagnosticsReport()
    for name, module in OPTIONAL_DEPENDENCIES.items():
        available = _importable(module)
        report.installed[name] = available
        if not available:
            report.missing.append(name)
            hint = _DEGRADED_HINTS.get(name)
            if hint:
                report.degraded_features.append(f"{name} 缺失: {hint}")
    if not report.all_required_present:
        report.degraded_features.append("缺少核心依赖 yaml，配置加载可能失败")
    logger.info(
        "依赖诊断: %d/%d 可选依赖可用，缺失 %s",
        report.available_count, len(OPTIONAL_DEPENDENCIES), report.missing or "无",
    )
    return report


def require_dependency(name: str) -> None:
    """断言某依赖可用，否则抛 ImportError

    Args:
        name: 依赖名

    Raises:
        ImportError: 依赖缺失
    """
    module = OPTIONAL_DEPENDENCIES.get(name)
    if module and not _importable(module):
        raise ImportError(f"缺少依赖: {name}（{module}）")


def summary_line() -> str:
    """单行诊断摘要（供 CLI 打印）

    Returns:
        人类可读摘要字符串
    """
    report = check_dependencies()
    return (
        f"可选依赖 {report.available_count}/{len(OPTIONAL_DEPENDENCIES)} 可用；"
        f"缺失: {', '.join(report.missing) if report.missing else '无'}"
    )
