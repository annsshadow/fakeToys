# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第90轮: indexer索引增强"""
import pytest
from augmentor.indexer import DatasetIndexer, DatasetView, QueryResult, IndexType


class TestIndexer:
    def test_create(self):
        items = [{"instruction": "test", "output": "out"}]
        idx = DatasetIndexer(items)
        assert idx is not None

    def test_index_type_enum(self):
        assert hasattr(IndexType, 'HASH')
        assert hasattr(IndexType, 'INVERTED')
        assert hasattr(IndexType, 'ENHANCED')

    def test_has_core_methods(self):
        items = [{"instruction": "test", "output": "out"}]
        idx = DatasetIndexer(items)
        assert hasattr(idx, 'search') or hasattr(idx, 'query')

    def test_create_view(self):
        items = [{"instruction": "test", "output": "out"}]
        view = DatasetView(items, "test_view")
        assert view is not None
