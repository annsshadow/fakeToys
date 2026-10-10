# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L202（B267）：预览缓存键截断——第二个调用方曾拿到第一份数据集的产物

**缺陷**：`PreviewGenerator.preview` 的缓存键只哈希 `sample[:5]`，而 `sample` 是
`items[:preview_size]` 的**全部**内容。于是「前 5 条相同、第 6 条起不同」的两个
数据集撞键：第二个调用方拿到**第一个数据集的 `original_data` / `converted_data`**。

这是 L123 修过的那一半的另一半——L123 把「只哈希样本」补成了「格式 + 条数 +
样本」三因子，但样本那个因子本身被截断了，等于「数据」这个因子又弄丢一次。
同族还有命中时把缓存对象**就地改完按引用返回**：同实例的所有调用方共享同一个
可变 `ExportPreview`，先来的改一次、后来的看到的就被改。

**修法**：键覆盖整份样本（外加 `len(sample)`）；命中时返回 `dataclasses.replace`
出的副本，只刷可能不同的 `total_items` / `preview_items`（键覆盖整份样本 ⇒ 命中
即数据相同，`fields` 不可能不同，无需刷）。
"""

import pytest

from augmentor.preview import PreviewGenerator

SHARED = [{"instruction": f"共{i}", "output": f"同{i}"} for i in range(5)]
#: 前 5 条与 SHARED 逐字节相同，第 6 条起不同
A = SHARED + [{"instruction": "A独有", "output": "甲"}]
B = SHARED + [{"instruction": "B独有", "output": "乙"}]


class TestKeyCoversTheWholeSample:
    """键必须覆盖整份样本，不能只哈希它的一角"""

    def test_same_prefix_different_tail_do_not_collide(self):
        p = PreviewGenerator()
        first = p.preview(A, format="alpaca", preview_size=6)
        second = p.preview(B, format="alpaca", preview_size=6)
        assert second.original_data[-1]["instruction"] == "B独有", (
            "第二个调用方拿到的是第一个数据集的产物——缓存键被截断了"
        )
        assert second.converted_data[-1]["instruction"] == "B独有"
        assert second is not first

    def test_identical_data_still_hits_the_cache(self):
        """防修过头：真相同的数据仍要命中（否则这条优化白做）"""
        p = PreviewGenerator()
        first = p.preview(A, format="alpaca", preview_size=6)
        second = p.preview(list(A), format="alpaca", preview_size=6)
        assert second.original_data == first.original_data
        assert second.converted_data == first.converted_data

    def test_length_differences_do_not_collide(self):
        """前若干条相同但条数不同，也不许撞键"""
        p = PreviewGenerator()
        short = p.preview(SHARED, format="alpaca", preview_size=5)
        long_ = p.preview(A, format="alpaca", preview_size=6)
        assert long_.original_data[-1]["instruction"] == "A独有"
        assert short.original_data[-1]["instruction"] == "共4"

    def test_format_still_breaks_the_key(self):
        """L123 修的那一半不许被本轮改回去"""
        p = PreviewGenerator()
        p.preview(A, format="alpaca", preview_size=6)
        as_jsonl = p.preview(A, format="jsonl", preview_size=6)
        assert as_jsonl.format == "jsonl"


class TestHitReturnsACopy:
    """命中不许把共享的可变对象交出去"""

    def test_hit_does_not_mutate_the_cached_object(self):
        p = PreviewGenerator()
        first = p.preview(A, format="alpaca", preview_size=6)
        second = p.preview(A, format="alpaca", preview_size=6)
        assert second is not first, "命中返回了缓存对象本身"
        second.format_info["total_items"] = 999
        assert first.format_info["total_items"] == len(A), (
            "改命中返回值把第一份的 format_info 也改了——共享可变对象"
        )

    def test_total_items_tracks_the_caller_dataset_on_a_hit(self):
        """同一前缀、不同总长的两份数据：命中后计数要按**本次**的算"""
        p = PreviewGenerator()
        p.preview(A, format="alpaca", preview_size=5)          # total=6
        longer = SHARED + [{"instruction": "B独有", "output": "乙"}] * 20
        hit = p.preview(longer, format="alpaca", preview_size=5)
        assert hit.format_info["total_items"] == len(longer), (
            "命中后 total_items 没按本次数据集刷新"
        )
        assert hit.format_info["preview_items"] == 5

    def test_fields_survive_a_hit(self):
        """键覆盖整份样本 ⇒ 命中即数据相同 ⇒ fields 必须还是对的"""
        p = PreviewGenerator()
        first = p.preview(A, format="alpaca", preview_size=6)
        second = p.preview(list(A), format="alpaca", preview_size=6)
        assert second.format_info["fields"] == first.format_info["fields"]
        assert set(second.format_info["fields"]) >= {"instruction", "output"}


class TestPreviewIsStillCorrectOnItsOwn:
    """防修过头：预览本身的功能不许变"""

    def test_truncation_of_long_records_still_works(self):
        p = PreviewGenerator()
        items = [{"instruction": "x" * 5000, "output": "y" * 5000}]
        result = p.preview(items, format="alpaca")
        assert len(result.original_data[0]["instruction"]) < 5000

    def test_preview_size_zero_is_honoured(self):
        """0 是合法值：一条都不预览（不许被读成「没传参数」）"""
        p = PreviewGenerator()
        result = p.preview(A, format="alpaca", preview_size=0)
        assert result.original_data == []
        assert result.converted_data == []

    def test_unsupported_format_still_raises(self):
        from augmentor.exceptions import DataValidationError

        p = PreviewGenerator()
        with pytest.raises(DataValidationError):
            p.preview(A, format="tsv")
