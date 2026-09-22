# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第77轮: data_pipeline数据管道"""
import pytest
from augmentor.data_pipeline import DataPipeline


class TestDataPipeline:
    def test_create(self):
        p = DataPipeline()
        assert p is not None

    def test_has_core_methods(self):
        p = DataPipeline()
        assert hasattr(p, 'process') or hasattr(p, 'run')
