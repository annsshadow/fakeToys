# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L127：preview 缓存键缺陷回归（L123 记档的真缺陷：键只哈希数据样本）

旧键 `md5(str(sample[:5]))` 不含格式与条数 ⇒ 同数据换格式 / 换条数会命中旧缓存，
返回旧格式、旧条数的残缺结果。修复后键 = (格式, 条数, 样本) 三因子。

**L202 订正**：上面那句里的「样本」一度仍被截断在前 5 条，于是「前 5 条相同、
第 6 条起不同」的两个数据集照样撞键，第二个调用方拿到第一个的产物。现在键覆盖
**整份样本**（外加 len(sample)），命中也不再返回同一个可变对象。
本文件钉住：
- 换格式不串味（旧键下必红：第二次拿到的是 jsonl 缓存）
- 换条数不串味（旧键下必红：前 5 条相同 ⇒ 同键，第二次拿到 5 条预览当 8 条用）
- 同三因子仍命中（优化没被修掉：命中支的总数更新路径照常工作）
"""

from augmentor.preview import PreviewGenerator


def _items(n):
    return [
        {"instruction": "问%d" % i, "input": "", "output": "答%d" % i} for i in range(n)
    ]


class TestPreviewCacheKeyCoversFormat:
    def test_same_data_different_format_does_not_leak(self):
        previewer = PreviewGenerator()
        items = _items(4)
        r_jsonl = previewer.preview(items, "jsonl")
        r_csv = previewer.preview(items, "csv")
        assert r_jsonl.format_info["format"] == "jsonl"
        # 旧键下 r_csv 是 jsonl 缓存（format_info["format"] == "jsonl"）⇒ 此断言先红
        assert r_csv.format_info["format"] == "csv"
        # 旧键下第二次调用直接 return 缓存对象（r_csv is r_jsonl）；新键下各算各的
        assert r_csv is not r_jsonl


class TestPreviewCacheKeyCoversSize:
    def test_same_first_five_different_size_does_not_leak(self):
        previewer = PreviewGenerator()
        items = _items(12)
        # 前 5 条相同、条数不同：旧键（只哈希前 5 条）会撞上 5 条那份缓存
        r5 = previewer.preview(items, "jsonl", preview_size=5)
        r8 = previewer.preview(items, "jsonl", preview_size=8)
        assert r5.format_info["preview_items"] == 5
        # 旧键下 r8 拿到 5 条预览（preview_items 被刷新成 8、converted_data 还是 5 条）
        assert r8.format_info["preview_items"] == 8
        assert len(r8.converted_data) == 8


class TestPreviewCacheStillHitsOnSameKey:
    def test_same_triple_hits_and_updates_totals(self):
        previewer = PreviewGenerator()
        small = _items(5)
        r1 = previewer.preview(small, "jsonl", preview_size=5)
        assert len(previewer._preview_cache) == 1
        # 同样三因子、数据加长：命中缓存、总数信息被刷新（既有优化行为不变）
        r2 = previewer.preview(_items(9), "jsonl", preview_size=5)
        # L202 订正：命中仍发生（下面三格照旧），但**不再返回同一个对象**。
        # 旧断言 `r2 is r1` 钉的是「命中时把缓存对象就地改完按引用返回」——那意味着
        # 同实例的所有调用方共享同一个可变 ExportPreview，先来的调用方改一次、后来的
        # 看到的就被改（L202 修的正是这条）。判据换成「内容相同、对象不同、已刷新」。
        assert r2 is not r1
        assert r2.original_data == r1.original_data
        assert r2.converted_data == r1.converted_data
        assert r2.format_info["total_items"] == 9
        assert len(previewer._preview_cache) == 1
