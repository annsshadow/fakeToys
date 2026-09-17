"""数据集自动化测试模块

提供数据集的自动化测试功能。
"""

import json
import logging
from typing import List, Dict, Optional, Any, Callable
from dataclasses import dataclass, field
from pathlib import Path
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class TestCase:
    """测试用例"""
    test_id: str
    name: str
    description: str
    test_func: Callable
    expected_result: Any = None
    tags: List[str] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "test_id": self.test_id,
            "name": self.name,
            "description": self.description,
            "expected_result": self.expected_result,
            "tags": self.tags
        }


@dataclass
class TestResult:
    """测试结果"""
    test_id: str
    test_name: str
    passed: bool
    message: str
    execution_time_ms: float
    timestamp: str
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "test_id": self.test_id,
            "test_name": self.test_name,
            "passed": self.passed,
            "message": self.message,
            "execution_time_ms": self.execution_time_ms,
            "timestamp": self.timestamp
        }


@dataclass
class TestSuite:
    """测试套件"""
    name: str
    description: str
    test_cases: List[TestCase] = field(default_factory=list)
    results: List[TestResult] = field(default_factory=list)
    
    @property
    def total_tests(self) -> int:
        return len(self.test_cases)
    
    @property
    def passed_tests(self) -> int:
        return sum(1 for r in self.results if r.passed)
    
    @property
    def failed_tests(self) -> int:
        return sum(1 for r in self.results if not r.passed)
    
    @property
    def pass_rate(self) -> float:
        return self.passed_tests / len(self.results) if self.results else 0
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "name": self.name,
            "description": self.description,
            "total_tests": self.total_tests,
            "passed_tests": self.passed_tests,
            "failed_tests": self.failed_tests,
            "pass_rate": self.pass_rate,
            "results": [r.to_dict() for r in self.results]
        }


class DatasetTestRunner:
    """数据集测试运行器
    
    提供数据集的自动化测试功能。
    """
    
    def __init__(self):
        """初始化测试运行器"""
        self._test_suites: Dict[str, TestSuite] = {}
        self._builtin_tests = self._create_builtin_tests()
    
    def _create_builtin_tests(self) -> List[TestCase]:
        """创建内置测试用例
        
        Returns:
            测试用例列表
        """
        return [
            TestCase(
                test_id="format_check",
                name="格式检查",
                description="检查数据集格式是否正确",
                test_func=self._test_format,
                tags=["format", "basic"]
            ),
            TestCase(
                test_id="field_check",
                name="字段检查",
                description="检查必填字段是否存在",
                test_func=self._test_fields,
                tags=["fields", "basic"]
            ),
            TestCase(
                test_id="quality_check",
                name="质量检查",
                description="检查数据质量是否达标",
                test_func=self._test_quality,
                tags=["quality", "basic"]
            ),
            TestCase(
                test_id="diversity_check",
                name="多样性检查",
                description="检查数据多样性是否足够",
                test_func=self._test_diversity,
                tags=["diversity", "basic"]
            ),
            TestCase(
                test_id="duplicate_check",
                name="重复检查",
                description="检查是否存在重复数据",
                test_func=self._test_duplicates,
                tags=["duplicates", "basic"]
            ),
        ]
    
    def create_test_suite(self, name: str, description: str = "",
                         test_cases: List[TestCase] = None) -> TestSuite:
        """创建测试套件
        
        Args:
            name: 套件名称
            description: 套件描述
            test_cases: 测试用例列表
        
        Returns:
            测试套件
        """
        suite = TestSuite(
            name=name,
            description=description,
            test_cases=test_cases or self._builtin_tests.copy()
        )
        self._test_suites[name] = suite
        return suite
    
    def run_tests(self, items: List[Dict], suite_name: str = None) -> TestSuite:
        """运行测试
        
        Args:
            items: 数据列表
            suite_name: 测试套件名称
        
        Returns:
            测试套件结果
        """
        if suite_name and suite_name in self._test_suites:
            suite = self._test_suites[suite_name]
        else:
            suite = self.create_test_suite("default", "默认测试套件")
        
        suite.results = []
        
        for test_case in suite.test_cases:
            import time
            start_time = time.time()
            
            try:
                passed, message = test_case.test_func(items, test_case.expected_result)
            except Exception as e:
                passed = False
                message = f"测试异常: {str(e)}"
            
            execution_time = (time.time() - start_time) * 1000
            
            result = TestResult(
                test_id=test_case.test_id,
                test_name=test_case.name,
                passed=passed,
                message=message,
                execution_time_ms=execution_time,
                timestamp=datetime.now().isoformat()
            )
            
            suite.results.append(result)
        
        return suite
    
    def _test_format(self, items: List[Dict], expected: Any = None) -> tuple:
        """测试格式
        
        Args:
            items: 数据列表
            expected: 期望结果
        
        Returns:
            (是否通过, 消息)
        """
        if not isinstance(items, list):
            return False, "数据不是列表格式"
        
        for i, item in enumerate(items):
            if not isinstance(item, dict):
                return False, f"第 {i+1} 条数据不是字典格式"
        
        return True, f"格式检查通过，共 {len(items)} 条数据"
    
    def _test_fields(self, items: List[Dict], expected: Any = None) -> tuple:
        """测试字段
        
        Args:
            items: 数据列表
            expected: 期望结果
        
        Returns:
            (是否通过, 消息)
        """
        required_fields = ["instruction", "output"]
        missing_fields = []
        
        for i, item in enumerate(items):
            for field in required_fields:
                if not item.get(field):
                    missing_fields.append(f"第 {i+1} 条数据缺少字段: {field}")
        
        if missing_fields:
            return False, f"发现 {len(missing_fields)} 个缺失字段: {missing_fields[:5]}"
        
        return True, f"字段检查通过，所有数据都包含必填字段"
    
    def _test_quality(self, items: List[Dict], expected: Any = None) -> tuple:
        """测试质量
        
        Args:
            items: 数据列表
            expected: 期望结果
        
        Returns:
            (是否通过, 消息)
        """
        if not items:
            return False, "数据为空"
        
        # 检查问题长度
        short_instructions = []
        for item in items:
            instruction = item.get("instruction", "")
            if len(instruction) < 5:
                short_instructions.append(instruction)
        
        if short_instructions:
            return False, f"发现 {len(short_instructions)} 个过短的问题"
        
        # 检查回答长度
        short_outputs = []
        for item in items:
            output = item.get("output", "")
            if len(output) < 10:
                short_outputs.append(output)
        
        if short_outputs:
            return False, f"发现 {len(short_outputs)} 个过短的回答"
        
        return True, "质量检查通过"
    
    def _test_diversity(self, items: List[Dict], expected: Any = None) -> tuple:
        """测试多样性
        
        Args:
            items: 数据列表
            expected: 期望结果
        
        Returns:
            (是否通过, 消息)
        """
        if not items:
            return False, "数据为空"
        
        # 收集所有问题
        instructions = [item.get("instruction", "") for item in items if item.get("instruction")]
        
        if not instructions:
            return False, "没有有效问题"
        
        # 计算唯一问题比例
        unique_instructions = set(instructions)
        diversity_ratio = len(unique_instructions) / len(instructions)
        
        if diversity_ratio < 0.5:
            return False, f"多样性不足: {diversity_ratio:.2%}"
        
        return True, f"多样性检查通过: {diversity_ratio:.2%}"
    
    def _test_duplicates(self, items: List[Dict], expected: Any = None) -> tuple:
        """测试重复
        
        Args:
            items: 数据列表
            expected: 期望结果
        
        Returns:
            (是否通过, 消息)
        """
        if not items:
            return False, "数据为空"
        
        # 收集所有问题
        instructions = [item.get("instruction", "") for item in items if item.get("instruction")]
        
        # 计算重复
        seen = set()
        duplicates = []
        for inst in instructions:
            if inst in seen:
                duplicates.append(inst)
            seen.add(inst)
        
        if duplicates:
            return False, f"发现 {len(duplicates)} 个重复问题"
        
        return True, "重复检查通过"
    
    def add_custom_test(self, test_id: str, name: str, description: str,
                       test_func: Callable, tags: List[str] = None):
        """添加自定义测试
        
        Args:
            test_id: 测试ID
            name: 测试名称
            description: 测试描述
            test_func: 测试函数
            tags: 标签列表
        """
        test_case = TestCase(
            test_id=test_id,
            name=name,
            description=description,
            test_func=test_func,
            tags=tags or []
        )
        self._builtin_tests.append(test_case)
    
    def get_test_report(self, suite: TestSuite) -> str:
        """生成测试报告
        
        Args:
            suite: 测试套件
        
        Returns:
            测试报告
        """
        lines = [
            "=" * 60,
            f"数据集测试报告: {suite.name}",
            "=" * 60,
            f"描述: {suite.description}",
            f"总测试数: {suite.total_tests}",
            f"通过: {suite.passed_tests}",
            f"失败: {suite.failed_tests}",
            f"通过率: {suite.pass_rate:.2%}",
            "",
            "测试详情:",
            "-" * 60
        ]
        
        for result in suite.results:
            status = "✅ 通过" if result.passed else "❌ 失败"
            lines.append(f"{status} {result.test_name}: {result.message}")
            lines.append(f"  执行时间: {result.execution_time_ms:.2f}ms")
        
        lines.append("=" * 60)
        
        return "\n".join(lines)


def run_dataset_tests(items: List[Dict], suite_name: str = None) -> TestSuite:
    """运行数据集测试
    
    Args:
        items: 数据列表
        suite_name: 测试套件名称
    
    Returns:
        测试套件结果
    """
    runner = DatasetTestRunner()
    return runner.run_tests(items, suite_name)


def create_test_suite(name: str, description: str = "",
                     test_cases: List[TestCase] = None) -> TestSuite:
    """创建测试套件
    
    Args:
        name: 套件名称
        description: 套件描述
        test_cases: 测试用例列表
    
    Returns:
        测试套件
    """
    runner = DatasetTestRunner()
    return runner.create_test_suite(name, description, test_cases)
