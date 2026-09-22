# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据管道编排模块

将画像、清洗、去重、异常检测、质量评分等独立模块编排为声明式管道，
按声明顺序执行并收集各阶段报告，支持单步执行与结果追溯。
"""

import logging
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional

logger = logging.getLogger(__name__)

# 阶段类型别名
StageFunc = Callable[[List[Dict], Dict[str, Any]], List[Dict]]


@dataclass
class StageResult:
    """单个阶段执行结果"""
    name: str
    input_count: int
    output_count: int
    duration_ms: float = 0.0
    report: Dict[str, Any] = field(default_factory=dict)
    error: str = ""

    @property
    def success(self) -> bool:
        return not self.error

    def to_dict(self) -> Dict[str, Any]:
        return {
            "name": self.name,
            "input_count": self.input_count,
            "output_count": self.output_count,
            "duration_ms": self.duration_ms,
            "success": self.success,
            "report": self.report,
            "error": self.error,
        }


class DataPipeline:
    """声明式数据管道

    阶段按注册顺序执行；某阶段标记 stop_on_failure=True 时，
    其失败会终止整个管道。
    """

    def __init__(self, name: str = "default"):
        """初始化管道

        Args:
            name: 管道名称
        """
        self.name = name
        self._stages: List[Dict[str, Any]] = []
        self.results: List[StageResult] = []

    def add_stage(self,
                  name: str,
                  func: StageFunc,
                  stop_on_failure: bool = False) -> "DataPipeline":
        """注册一个处理阶段

        Args:
            name: 阶段名
            func: 处理函数 (items, context) -> items
            stop_on_failure: 失败时是否终止管道

        Returns:
            自身（支持链式调用）
        """
        if any(stage["name"] == name for stage in self._stages):
            raise ValueError(f"阶段名重复: {name}")
        self._stages.append({
            "name": name,
            "func": func,
            "stop_on_failure": stop_on_failure,
        })
        return self

    def run(self,
            items: List[Dict],
            context: Optional[Dict[str, Any]] = None) -> List[Dict]:
        """执行全部阶段

        Args:
            items: 输入数据
            context: 跨阶段共享上下文

        Returns:
            最终数据列表
        """
        import time

        context = context if context is not None else {}
        current = items
        self.results = []
        stopped = False

        for stage in self._stages:
            start = time.time()
            result = StageResult(
                name=stage["name"],
                input_count=len(current),
                output_count=0,
                duration_ms=0.0,
            )
            try:
                current = stage["func"](current, context)
                if not isinstance(current, list):
                    raise TypeError(
                        f"阶段 {stage['name']} 必须返回列表，"
                        f"实际返回 {type(current).__name__}"
                    )
                result.output_count = len(current)
            except Exception as e:
                result.error = str(e)
                logger.error(f"阶段 {stage['name']} 执行失败: {e}")
                if stage["stop_on_failure"]:
                    stopped = True
                    self.results.append(result)
                    break
            result.duration_ms = (time.time() - start) * 1000
            self.results.append(result)

        if stopped:
            logger.warning(f"管道 {self.name} 因阶段失败而终止")

        return current

    def run_stage(self,
                  name: str,
                  items: List[Dict],
                  context: Optional[Dict[str, Any]] = None) -> List[Dict]:
        """仅执行指定阶段（不执行其他阶段）

        Args:
            name: 阶段名
            items: 输入数据
            context: 共享上下文

        Returns:
            阶段输出
        """
        import time

        stage = self._find_stage(name)
        start = time.time()
        result = StageResult(name=name, input_count=len(items), output_count=0, duration_ms=0.0)
        try:
            current = stage["func"](items, context or {})
            result.output_count = len(current)
        except Exception as e:
            result.error = str(e)
            raise
        result.duration_ms = (time.time() - start) * 1000
        self.results = [result]
        return current

    def _find_stage(self, name: str) -> Dict[str, Any]:
        for stage in self._stages:
            if stage["name"] == name:
                return stage
        raise ValueError(f"未注册的阶段: {name}")

    def get_report(self) -> Dict[str, Any]:
        """生成管道执行报告

        Returns:
            报告字典
        """
        total_in = self.results[0].input_count if self.results else 0
        total_out = self.results[-1].output_count if self.results else 0
        failed = [r.name for r in self.results if not r.success]

        return {
            "pipeline": self.name,
            "stage_count": len(self._stages),
            "executed_stages": len(self.results),
            "initial_count": total_in,
            "final_count": total_out,
            "failed_stages": failed,
            "stages": [r.to_dict() for r in self.results],
        }

    @property
    def stage_names(self) -> List[str]:
        """已注册阶段名列表"""
        return [stage["name"] for stage in self._stages]

    def clear_results(self) -> None:
        """清空执行结果"""
        self.results = []


def identity(items: List[Dict], context: Dict[str, Any]) -> List[Dict]:
    """透传阶段（测试与占位用）"""
    return items


def report_stage(report_key: str, report_value: Any):
    """创建「写入报告值后透传」的阶段工厂

    返回的 stage 函数签名为 (items, context) -> items，会把
    report_value 写入 context[report_key] 供后续阶段与 get_report 使用。

    Args:
        report_key: 上下文键
        report_value: 要写入的报告值

    Returns:
        可作为 add_stage 注册的阶段函数
    """
    def stage(items: List[Dict], context: Dict[str, Any]) -> List[Dict]:
        context[report_key] = report_value
        return items

    stage.__name__ = f"report_stage:{report_key}"
    return stage


__all__ = [
    "DataPipeline",
    "StageResult",
    "identity",
    "report_stage",
]