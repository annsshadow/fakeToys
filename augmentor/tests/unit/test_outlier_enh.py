# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""异常检测增强测试"""
from augmentor.outlier import detect_outliers

def test_outlier_exists():
    assert callable(detect_outliers)
