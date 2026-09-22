"""异常检测增强测试"""
from augmentor.outlier import detect_outliers

def test_outlier_exists():
    assert callable(detect_outliers)
