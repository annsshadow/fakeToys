# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第62轮: validation验证增强"""
import pytest
from augmentor.validation import DatasetValidator


class TestDatasetValidator:
    def test_create(self):
        v = DatasetValidator()
        assert v is not None

    def test_validate_basic(self):
        v = DatasetValidator()
        items = [{"instruction": "test", "output": "out"}]
        result = v.validate(items)
        assert result is not None

    def test_validate_empty(self):
        v = DatasetValidator()
        result = v.validate([])
        assert result is not None
