# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第72轮: preview预览生成"""
import pytest
from augmentor.preview import PreviewGenerator


class TestPreview:
    def test_create(self):
        p = PreviewGenerator()
        assert p is not None

    def test_has_core_methods(self):
        p = PreviewGenerator()
        assert hasattr(p, 'preview') or hasattr(p, 'generate_preview')
