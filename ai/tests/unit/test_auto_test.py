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


class TestAutoTestExtended:
    """DatasetTestRunner 扩展测试"""

    def test_builtin_test_format_non_list(self):
        """_test_format 非列表输入"""
        runner = DatasetTestRunner()
        passed, msg = runner._test_format("not-a-list")
        assert passed is False
        assert "不是列表" in msg

    def test_builtin_test_format_non_dict_item(self):
        """_test_format 非字典项"""
        runner = DatasetTestRunner()
        passed, msg = runner._test_format([{"instruction": "q"} , "not-dict"])
        assert passed is False
        assert "不是字典" in msg

    def test_builtin_test_format_pass(self):
        """_test_format 正常通过"""
        runner = DatasetTestRunner()
        passed, msg = runner._test_format([{"instruction": "q", "output": "a"}])
        assert passed is True
        assert "1" in msg

    def test_builtin_test_fields_missing(self):
        """_test_fields 缺字段"""
        runner = DatasetTestRunner()
        passed, msg = runner._test_fields([{"instruction": "q"}])
        assert passed is False
        assert "output" in msg

    def test_builtin_test_fields_empty_field(self):
        """_test_fields 字段为空"""
        runner = DatasetTestRunner()
        passed, _ = runner._test_fields([{"instruction": "", "output": "a"}])
        assert passed is False

    def test_builtin_test_fields_pass(self):
        """_test_fields 正常通过"""
        runner = DatasetTestRunner()
        passed, _ = runner._test_fields([{"instruction": "q", "output": "a"}])
        assert passed is True

    def test_builtin_test_quality_empty(self):
        """_test_quality 空数据"""
        runner = DatasetTestRunner()
        passed, msg = runner._test_quality([])
        assert passed is False
        assert "为空" in msg

    def test_builtin_test_quality_short_instruction(self):
        """_test_quality 问题过短"""
        runner = DatasetTestRunner()
        passed, msg = runner._test_quality([{"instruction": "短", "output": "足够长的回答内容哦呀哈"}])
        assert passed is False
        assert "过短" in msg

    def test_builtin_test_quality_short_output(self):
        """_test_quality 回答过短"""
        runner = DatasetTestRunner()
        passed, msg = runner._test_quality([{"instruction": "足够长的问题", "output": "短"}])
        assert passed is False

    def test_builtin_test_quality_pass(self):
        """_test_quality 通过"""
        runner = DatasetTestRunner()
        passed, _ = runner._test_quality([{"instruction": "足够长的问题", "output": "足够长的回答内容哦呀哈"}])
        assert passed is True

    def test_builtin_test_diversity_empty(self):
        """_test_diversity 空数据"""
        runner = DatasetTestRunner()
        passed, _ = runner._test_diversity([])
        assert passed is False

    def test_builtin_test_diversity_no_questions(self):
        """_test_diversity 无有效问题"""
        runner = DatasetTestRunner()
        passed, msg = runner._test_diversity([{"instruction": ""}])
        assert passed is False
        assert "没有有效问题" in msg

    def test_builtin_test_diversity_low(self):
        """_test_diversity 重复过多"""
        runner = DatasetTestRunner()
        items = [{"instruction": "同一个问题啊"}] * 4
        passed, msg = runner._test_diversity(items)
        assert passed is False
        assert "多样性不足" in msg

    def test_builtin_test_diversity_pass(self):
        """_test_diversity 通过"""
        runner = DatasetTestRunner()
        items = [{"instruction": f"不同的问题{i}啊"} for i in range(4)]
        passed, _ = runner._test_diversity(items)
        assert passed is True

    def test_builtin_test_duplicates_empty(self):
        """_test_duplicates 空数据"""
        runner = DatasetTestRunner()
        passed, _ = runner._test_duplicates([])
        assert passed is False

    def test_builtin_test_duplicates_found(self):
        """_test_duplicates 发现重复"""
        runner = DatasetTestRunner()
        items = [{"instruction": "同个问题呀"}] * 2
        passed, msg = runner._test_duplicates(items)
        assert passed is False

    def test_builtin_test_duplicates_pass(self):
        """_test_duplicates 通过"""
        runner = DatasetTestRunner()
        items = [{"instruction": f"问题{i}呀"} for i in range(3)]
        passed, _ = runner._test_duplicates(items)
        assert passed is True

    def test_run_tests_with_named_suite(self, sample_dataset):
        """run_tests 指定已有套件名"""
        runner = DatasetTestRunner()
        runner.create_test_suite("my_suite", "自定义")
        suite = runner.run_tests(sample_dataset, "my_suite")
        assert suite.name == "my_suite"

    def test_run_tests_exception_caught(self):
        """测试函数抛异常时应记录失败而不是崩溃"""
        runner = DatasetTestRunner()
        runner.add_custom_test(
            test_id="boom", name="爆炸", description="",
            test_func=lambda items, expected: (_ for _ in ()).throw(RuntimeError("boom"))
        )
        suite = runner.run_tests([{"instruction": "q", "output": "a"}])
        results = [r for r in suite.results if r.test_id == "boom"]
        assert len(results) == 1
        assert results[0].passed is False
        assert "异常" in results[0].message

    def test_report_contains_pass_rate(self, sample_dataset):
        """报告应包含通过率"""
        runner = DatasetTestRunner()
        suite = runner.run_tests(sample_dataset)
        report = runner.get_test_report(suite)
        assert "通过率" in report or "pass" in report.lower()
