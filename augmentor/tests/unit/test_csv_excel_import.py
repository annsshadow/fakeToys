"""CSV/Excel 导入测试

原实现的问题（本次重写的原因）：

- `test_import_raises_unsupported_format` 只有 `assert True`；
- `test_import_dataset_auto_detect_csv` 断言函数签名的参数个数，而不是行为；
- `test_import_raises_without_pandas` 用 `try/except: pass` 吞掉所有异常，
  无论被测代码做什么都会通过——**一个不可能失败的测试**；
- `mock_pandas_available` fixture 定义后从未被任何用例使用。

现改为断言可观测行为。格式校验在读取文件之前完成，因此下面两条不依赖 pandas、
也不产生真实文件 IO。
"""

import pytest

from augmentor.csv_excel_import import import_dataset


class TestFormatValidation:
    """格式识别与拒绝"""

    def test_unknown_suffix_rejected(self, tmp_path):
        """无法识别的后缀必须抛 ValueError，而不是静默返回空列表"""
        with pytest.raises(ValueError, match="无法识别文件格式"):
            import_dataset(str(tmp_path / "data.txt"))

    def test_unsupported_format_argument_rejected(self, tmp_path):
        """显式传入不支持的 format 必须抛 ValueError"""
        with pytest.raises(ValueError, match="不支持的导入格式"):
            import_dataset(str(tmp_path / "data.txt"), format="bogus")
