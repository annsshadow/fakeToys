"""数据集自动化测试模块测试"""

import pytest
from augmentor.auto_test import (
    DatasetTestRunner, TestSuite, TestCase, TestResult,
    run_dataset_tests, create_test_suite
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
def bad_dataset():
    """创建有问题的数据集"""
    return [
        {"instruction": "", "input": "", "output": ""},
        {"instruction": "问", "input": "", "output": "答"},
        {"instruction": "如何申请租房？", "input": "", "output": "如何申请租房？"},  # 重复
    ]


class TestDatasetTestRunner:
    """DatasetTestRunner 测试"""
    
    def test_init(self):
        """测试初始化"""
        runner = DatasetTestRunner()
        assert len(runner._builtin_tests) > 0
    
    def test_create_test_suite(self):
        """测试创建测试套件"""
        runner = DatasetTestRunner()
        suite = runner.create_test_suite("test_suite", "测试套件")
        
        assert isinstance(suite, TestSuite)
        assert suite.name == "test_suite"
        assert suite.description == "测试套件"
    
    def test_run_tests(self, sample_dataset):
        """测试运行测试"""
        runner = DatasetTestRunner()
        suite = runner.run_tests(sample_dataset)
        
        assert isinstance(suite, TestSuite)
        assert suite.total_tests > 0
        assert len(suite.results) > 0
    
    def test_run_tests_with_bad_dataset(self, bad_dataset):
        """测试运行有问题的数据集"""
        runner = DatasetTestRunner()
        suite = runner.run_tests(bad_dataset)
        
        assert suite.failed_tests > 0
    
    def test_add_custom_test(self, sample_dataset):
        """测试添加自定义测试"""
        runner = DatasetTestRunner()
        initial_count = len(runner._builtin_tests)
        
        def custom_test(items, expected):
            return len(items) > 0, f"数据量: {len(items)}"
        
        runner.add_custom_test(
            test_id="custom_test",
            name="自定义测试",
            description="测试数据量",
            test_func=custom_test
        )
        
        assert len(runner._builtin_tests) == initial_count + 1
        
        suite = runner.run_tests(sample_dataset)
        test_ids = [tc.test_id for tc in suite.results]
        assert "custom_test" in test_ids
    
    def test_get_test_report(self, sample_dataset):
        """测试生成测试报告"""
        runner = DatasetTestRunner()
        suite = runner.run_tests(sample_dataset)
        report = runner.get_test_report(suite)
        
        assert isinstance(report, str)
        assert "数据集测试报告" in report


class TestTestSuite:
    """TestSuite 测试"""
    
    def test_to_dict(self, sample_dataset):
        """测试转换为字典"""
        runner = DatasetTestRunner()
        suite = runner.run_tests(sample_dataset, "test")
        d = suite.to_dict()
        
        assert isinstance(d, dict)
        assert "name" in d
        assert "total_tests" in d
        assert "passed_tests" in d
        assert "failed_tests" in d
        assert "pass_rate" in d


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_run_dataset_tests(self, sample_dataset):
        """测试运行数据集测试"""
        suite = run_dataset_tests(sample_dataset)
        
        assert isinstance(suite, TestSuite)
        assert suite.total_tests > 0
    
    def test_create_test_suite(self):
        """测试创建测试套件"""
        suite = create_test_suite("test", "测试")
        
        assert isinstance(suite, TestSuite)
        assert suite.name == "test"
