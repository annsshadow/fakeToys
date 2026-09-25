# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集操作工具模块测试"""

import json
import random
from collections import Counter
import pytest
from pathlib import Path
from augmentor.allocation import largest_remainder
from augmentor.data_splitter import DataSplitter
from augmentor.exceptions import DataValidationError
from augmentor.dataset_ops import (
    DatasetOperations, MergeConfig, SampleConfig, SplitConfig,
    merge_datasets, sample_dataset as sample_dataset_func, split_dataset
)


@pytest.fixture
def test_data():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
        {"instruction": "如何支付房租？", "input": "", "output": "支持多种支付方式"},
        {"instruction": "租房合同怎么签？", "input": "", "output": "在线签署电子合同"},
    ]


@pytest.fixture
def temp_dataset_file(tmp_path, test_data):
    """创建临时数据集文件"""
    file_path = tmp_path / "dataset.json"
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(test_data, f, ensure_ascii=False)
    return str(file_path)


class TestDatasetOperations:
    """DatasetOperations 测试"""
    
    def test_merge_basic(self, test_data):
        """测试基本合并功能"""
        ops = DatasetOperations()
        dataset_b = [
            {"instruction": "问题6", "input": "", "output": "回答6"},
            {"instruction": "问题7", "input": "", "output": "回答7"},
        ]
        
        merged = ops.merge([test_data, dataset_b])
        assert len(merged) == 7
    
    def test_merge_with_dedup(self, test_data):
        """测试带去重的合并"""
        ops = DatasetOperations()
        dataset_b = [
            {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},  # 重复
            {"instruction": "问题7", "input": "", "output": "回答7"},
        ]
        
        config = MergeConfig(deduplicate=True)
        merged = ops.merge([test_data, dataset_b], config)
        
        # 去重后应该只有6条（原5条 + 1条新数据）
        assert len(merged) == 6
    
    def test_merge_max_items(self, test_data):
        """测试限制最大条数"""
        ops = DatasetOperations()
        dataset_b = [
            {"instruction": "问题6", "input": "", "output": "回答6"},
            {"instruction": "问题7", "input": "", "output": "回答7"},
        ]
        
        config = MergeConfig(max_items=5)
        merged = ops.merge([test_data, dataset_b], config)
        
        assert len(merged) == 5
    
    def test_sample_random(self, test_data):
        """测试随机采样"""
        ops = DatasetOperations()
        config = SampleConfig(method="random", size=3, seed=42)
        
        sampled = ops.sample(test_data, config)
        assert len(sampled) == 3
    
    def test_sample_by_ratio(self, test_data):
        """测试按比例采样"""
        ops = DatasetOperations()
        config = SampleConfig(ratio=0.6, seed=42)
        
        sampled = ops.sample(test_data, config)
        assert len(sampled) == 3  # 5 * 0.6 = 3
    
    def test_sample_systematic(self, test_data):
        """测试系统采样"""
        ops = DatasetOperations()
        config = SampleConfig(method="systematic", size=3)
        
        sampled = ops.sample(test_data, config)
        assert len(sampled) == 3
    
    def test_split_basic(self, test_data):
        """测试基本分割功能"""
        ops = DatasetOperations()
        config = SplitConfig(ratios=(0.6, 0.2, 0.2), shuffle=False)
        
        train, val, test = ops.split(test_data, config)
        
        assert len(train) == 3
        assert len(val) == 1
        assert len(test) == 1
    
    def test_split_with_shuffle(self, test_data):
        """测试打乱分割"""
        ops = DatasetOperations()
        config = SplitConfig(ratios=(0.6, 0.2, 0.2), shuffle=True, seed=42)
        
        train, val, test = ops.split(test_data, config)
        
        assert len(train) + len(val) + len(test) == len(test_data)
    
    def test_split_invalid_ratios(self, test_data):
        """测试无效分割比例"""
        ops = DatasetOperations()
        config = SplitConfig(ratios=(0.5, 0.3, 0.3))  # 总和 > 1
        
        with pytest.raises(ValueError):
            ops.split(test_data, config)
    
    def test_shuffle(self, test_data):
        """测试打乱顺序"""
        ops = DatasetOperations()
        shuffled = ops.shuffle(test_data, seed=42)
        
        assert len(shuffled) == len(test_data)
        # 内容相同但顺序可能不同
        assert set(item["instruction"] for item in shuffled) == set(item["instruction"] for item in test_data)
    
    def test_head(self, test_data):
        """测试获取前N条"""
        ops = DatasetOperations()
        head = ops.head(test_data, n=3)
        
        assert len(head) == 3
        assert head == test_data[:3]
    
    def test_tail(self, test_data):
        """测试获取后N条"""
        ops = DatasetOperations()
        tail = ops.tail(test_data, n=3)
        
        assert len(tail) == 3
        assert tail == test_data[-3:]
    
    def test_filter_by_length(self, test_data):
        """测试按长度过滤"""
        ops = DatasetOperations()
        filtered = ops.filter_by_length(test_data, min_length=5, max_length=10)
        
        for item in filtered:
            assert 5 <= len(item["instruction"]) <= 10
    
    def test_filter_by_keyword(self, test_data):
        """测试按关键词过滤"""
        ops = DatasetOperations()
        filtered = ops.filter_by_keyword(test_data, ["租房"], mode="include")
        
        for item in filtered:
            assert "租房" in item["instruction"]
    
    def test_get_statistics(self, test_data):
        """测试获取统计信息"""
        ops = DatasetOperations()
        stats = ops.get_statistics(test_data)
        
        assert stats["total"] == 5
        assert "avg_length" in stats
        assert "min_length" in stats
        assert "max_length" in stats
        assert "unique_instructions" in stats


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_merge_files(self, tmp_path, test_data):
        """测试合并文件"""
        # 创建两个数据集文件
        file_a = tmp_path / "a.json"
        file_b = tmp_path / "b.json"
        
        with open(file_a, 'w', encoding='utf-8') as f:
            json.dump(test_data[:3], f, ensure_ascii=False)
        
        with open(file_b, 'w', encoding='utf-8') as f:
            json.dump(test_data[3:], f, ensure_ascii=False)
        
        output = tmp_path / "merged.json"
        result = merge_datasets([str(file_a), str(file_b)], str(output))
        
        assert result["input_files"] == 2
        assert result["total_output"] == 5
        assert output.exists()
    
    def test_sample_file(self, tmp_path, test_data):
        """测试采样文件"""
        input_file = tmp_path / "input.json"
        output_file = tmp_path / "output.json"
        
        with open(input_file, 'w', encoding='utf-8') as f:
            json.dump(test_data, f, ensure_ascii=False)
        
        result = sample_dataset_func(str(input_file), str(output_file), size=3, seed=42)
        
        assert result["input_count"] == 5
        assert result["output_count"] == 3
        assert output_file.exists()
    
    def test_split_file(self, tmp_path, test_data):
        """测试分割文件"""
        input_file = tmp_path / "input.json"
        output_dir = tmp_path / "splits"
        
        with open(input_file, 'w', encoding='utf-8') as f:
            json.dump(test_data, f, ensure_ascii=False)
        
        result = split_dataset(str(input_file), str(output_dir), seed=42)
        
        assert "splits" in result
        assert (output_dir / "train.json").exists()
        assert (output_dir / "val.json").exists()
        assert (output_dir / "test.json").exists()


class TestDatasetOperationsEdgeCases:
    """边界情况测试"""
    
    def test_merge_empty_datasets(self):
        """测试合并空数据集"""
        ops = DatasetOperations()
        merged = ops.merge([[], []])
        assert len(merged) == 0
    
    def test_sample_larger_than_dataset(self, test_data):
        """测试采样数量大于数据集"""
        ops = DatasetOperations()
        config = SampleConfig(size=100)
        
        sampled = ops.sample(test_data, config)
        assert len(sampled) == len(test_data)
    
    def test_get_statistics_empty(self):
        """测试空数据集统计"""
        ops = DatasetOperations()
        stats = ops.get_statistics([])
        
        assert stats["total"] == 0


class TestDatasetOperationsExtended:
    """DatasetOperations 扩展测试"""

    def test_merge_single_dataset(self):
        """合并单个数据集"""
        ops = DatasetOperations()
        data = [{"instruction": "q1"}, {"instruction": "q2"}]
        merged = ops.merge([data])
        assert len(merged) == 2

    def test_sample_random_with_seed(self, test_data):
        """随机采样可复现性"""
        ops = DatasetOperations()
        config = SampleConfig(method="random", size=3, seed=42)
        sampled1 = ops.sample(test_data, config)
        sampled2 = ops.sample(test_data, config)
        assert [item["instruction"] for item in sampled1] == [item["instruction"] for item in sampled2]

    def test_sample_stratified(self, test_data):
        """分层采样"""
        ops = DatasetOperations()
        config = SampleConfig(method="stratified", size=3, stratify_key="instruction")
        sampled = ops.sample(test_data, config)
        assert len(sampled) == 3

    def test_split_all_to_train(self, test_data):
        """全部分到训练集"""
        ops = DatasetOperations()
        config = SplitConfig(ratios=(1.0, 0.0, 0.0))
        train, val, test = ops.split(test_data, config)
        assert len(train) == len(test_data)
        assert len(val) == 0
        assert len(test) == 0

    def test_filter_by_keyword_exclude(self, test_data):
        """排除关键词过滤"""
        ops = DatasetOperations()
        filtered = ops.filter_by_keyword(test_data, ["租房"], mode="exclude")
        for item in filtered:
            assert "租房" not in item["instruction"]

    def test_filter_by_keyword_empty(self, test_data):
        """空关键词列表过滤应返回全部"""
        ops = DatasetOperations()
        filtered = ops.filter_by_keyword(test_data, [], mode="include")
        assert len(filtered) == 0

    def test_merge_with_max_items_and_dedup(self, test_data):
        """同时使用 max_items 和 dedup"""
        ops = DatasetOperations()
        dataset_b = [{"instruction": "如何申请租房？"}]  # duplicate
        config = MergeConfig(max_items=3, deduplicate=True)
        merged = ops.merge([test_data, dataset_b], config)
        assert len(merged) <= 3

    def test_merge_files_empty(self, tmp_path):
        """合并空文件"""
        file_a = tmp_path / "empty.json"
        file_a.write_text("[]", encoding="utf-8")
        output = tmp_path / "merged.json"
        result = merge_datasets([str(file_a)], str(output))
        assert result["total_output"] == 0


class TestDatasetOpsExtended:
    """DatasetOperations 第二轮扩展测试（覆盖剩余分支）"""

    def test_merge_preserve_order_false_shuffles(self, test_data):
        """preserve_order=False 应打乱顺序（设种子验证）"""
        ops = DatasetOperations()
        b = [{"instruction": f"x{i}"} for i in range(5)]
        merged = ops.merge([test_data, b], MergeConfig(preserve_order=False))
        assert len(merged) == 10

    def test_merge_max_items_truncates(self, test_data):
        """max_items 应限制合并后数量"""
        ops = DatasetOperations()
        b = [{"instruction": "y"}] * 5
        merged = ops.merge([test_data, b], MergeConfig(max_items=3))
        assert len(merged) == 3

    def test_merge_dedup_same_instruction(self, test_data):
        """相同 instruction 去重"""
        ops = DatasetOperations()
        dup = [{"instruction": test_data[0]["instruction"]}]
        merged = ops.merge([test_data, dup], MergeConfig(deduplicate=True))
        assert len(merged) == len(test_data)

    def test_sample_by_size(self, test_data):
        """按数量采样"""
        ops = DatasetOperations()
        sampled = ops.sample(test_data, SampleConfig(size=2, seed=1))
        assert len(sampled) == 2

    def test_sample_by_ratio(self, test_data):
        """按比例采样"""
        ops = DatasetOperations()
        sampled = ops.sample(test_data, SampleConfig(ratio=0.4, seed=1))
        assert len(sampled) == 2  # 5 * 0.4 = 2

    def test_sample_default_no_config(self, test_data):
        """无采样配置应返回全部（默认 random 方法）"""
        ops = DatasetOperations()
        sampled = ops.sample(test_data)
        assert sorted(i["instruction"] for i in sampled) == \
               sorted(i["instruction"] for i in test_data)

    def test_split_three_ways(self, test_data):
        """三向分割数量应正确"""
        ops = DatasetOperations()
        train, val, test = ops.split(
            test_data, SplitConfig(ratios=(0.6, 0.2, 0.2))
        )
        assert len(train) + len(val) + len(test) == len(test_data)

    def test_filter_by_keyword_include(self, test_data):
        """关键词包含过滤"""
        ops = DatasetOperations()
        filtered = ops.filter_by_keyword(test_data, ["租房"], mode="include")
        assert all("租房" in item["instruction"] for item in filtered)

    def test_merge_files_writes_output(self, tmp_path, test_data):
        """merge_files 应写出结果文件"""
        ops = DatasetOperations()
        f1 = tmp_path / "a.json"
        f1.write_text(json.dumps(test_data), encoding="utf-8")
        f2 = tmp_path / "b.json"
        f2.write_text(json.dumps([{"instruction": "z"}]), encoding="utf-8")
        output = tmp_path / "out.json"
        result = ops.merge_files([str(f1), str(f2)], str(output))
        assert output.exists()
        assert result["total_input"] == 6

    def test_deduplicate_empty(self):
        """空列表去重"""
        ops = DatasetOperations()
        assert ops._deduplicate([]) == []

    def test_deduplicate_uses_instruction_hash(self):
        """去重应基于 instruction 哈希"""
        ops = DatasetOperations()
        items = [
            {"instruction": "same", "output": "a"},
            {"instruction": "same", "output": "b"},
        ]
        result = ops._deduplicate(items)
        assert len(result) == 1

    def test_convenience_split_dataset(self, test_data, tmp_path):
        """便捷函数 split_dataset 应写出分割文件"""
        import json as _json
        src = tmp_path / "src.json"
        src.write_text(_json.dumps(test_data), encoding="utf-8")
        outdir = str(tmp_path / "split")
        result = split_dataset(str(src), outdir)
        assert result["output_dir"] == outdir
        assert (tmp_path / "split" / "train.json").exists()

    def test_convenience_sample_dataset(self, test_data, tmp_path):
        """便捷函数 sample_dataset 应采样并写出文件"""
        import json as _json
        src = tmp_path / "src.json"
        src.write_text(_json.dumps(test_data), encoding="utf-8")
        out = str(tmp_path / "sampled.json")
        result = sample_dataset_func(str(src), out, size=2, seed=0)
        assert result["output_count"] == 2
        assert Path(out).exists()


def _stratify_corpus():
    """三类不等大小（6 / 10 / 14 条，共 30 条）的语料

    故意让三类都吃不下整份比例：`ratios=(0.6, 0.2, 0.2)` 的名义值是 c0 `3.6/1.2/1.2`、
    c1 `6/2/2`、c2 `8.4/2.8/2.8`，不分层的随机切在 8 个种子上最大单元偏差 1.0–2.0 条
    （其中 seed 0、4 的验证集整类漏掉 c0 / c1），分层后恒为 0.4 条 —— 这个差值是语料
    本身的性质，用例把它当作前置断言，免得「语料根本区分不出两种口径」时用例白拿绿。
    """
    items = []
    for class_id in range(3):
        for _ in range(6 + 4 * class_id):
            items.append({"instruction": f"问题{len(items)}", "intent": f"c{class_id}",
                          "pos": len(items)})
    return items


CORPUS = _stratify_corpus()
RATIOS = (0.6, 0.2, 0.2)


def _worst_cell_deviation(segments, corpus, ratios):
    """`max |段内某类条数 - 该类总条数 × 比例|`，与 `test_data_splitter` 的口径同"""
    sizes = Counter(item["intent"] for item in corpus)
    worst = 0.0
    for segment, ratio in zip(segments, ratios):
        got = Counter(item["intent"] for item in segment)
        for key, size in sizes.items():
            worst = max(worst, abs(got.get(key, 0) - size * ratio))
    return worst


def _legacy_plain_split(items, config):
    """L40 提交态的 `DatasetOperations.split`（无分层支路）复刻，只作同答 oracle

    钉的是「默认路径（`stratify=False`）一个字都没动」，不参与本轮成效。
    """
    data = list(items)
    if config.shuffle:
        random.Random(config.seed).shuffle(data)
    train_size, val_size, _ = largest_remainder(len(data), config.ratios)
    return (data[:train_size], data[train_size:train_size + val_size],
            data[train_size + val_size:])


class TestStratifiedSplitWiring:
    """A54：`SplitConfig.stratify` / `stratify_key` 必须真的参与分割

    L38 修好的分层口径在 `DataSplitter` 里，而 `DatasetOperations.split`（CLI `split`、
    API `/api/dataset/split`、便捷函数 `split_dataset` 都走它）自带一套三段切分，
    以前**从不读**这两个字段 —— 传 `stratify=True` 与不传逐条同答（实测真实 6,902 条
    语料三个种子、缩例八个种子都同答），所以那项能力在产品面上是静默空转的。
    """

    def test_stratify_switch_is_not_inert(self):
        """开关必须改变答案：同一 `seed` 下 stratify=True 与 False 不得逐条同答

        这就是本轮的缺陷本体 —— 旧实现里开关对所有取值都是死的。
        """
        ops = DatasetOperations()
        for field in ("intent", "instruction"):
            plain = ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=0))
            strat = ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=0,
                                                  stratify=True, stratify_key=field))
            assert strat != plain, f"stratify_key={field} 时开关仍是死的"

    def test_stratify_key_selects_the_grouping(self):
        """`stratify_key` 必须是分组字段：换字段就换答案

        `instruction` 在这份语料里逐行唯一（每组 1 条），`intent` 只有 3 组，
        两种分组给出的成员分配不可能相同。
        """
        ops = DatasetOperations()
        by_intent = ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=0, stratify=True,
                                                  stratify_key="intent"))
        by_instruction = ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=0, stratify=True,
                                                      stratify_key="instruction"))
        assert by_intent != by_instruction

    def test_stratified_answer_matches_data_splitter(self):
        """口径只有一份实现：分层支路必须与 `DataSplitter` 逐条同答

        接线而不是复刻算法，所以这里拿 `DataSplitter` 当 oracle —— 一旦本轮在
        `dataset_ops` 里另写一套摊派，成员顺序或组序纪律稍有不同就会红。
        """
        ops = DatasetOperations()
        for seed in (0, 3, 17):
            mine = ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=seed, stratify=True,
                                                 stratify_key="intent"))
            theirs = DataSplitter(*RATIOS, seed=seed, stratify_field="intent").split(CORPUS)
            assert mine == (theirs.train, theirs.val, theirs.test)

    @pytest.mark.parametrize("seed", [0, 1, 2, 3, 4, 5, 6, 7])
    def test_stratify_moves_each_class_its_nominal_share(self, seed):
        """分层的能力主张：每类在每段里的条数压到名义值 ±0.5 条以内

        语料的三类是 6 / 10 / 14 条，名义值带小数（3.6 / 1.2 / 2.8 / 8.4），所以 0.5 是
        「整数条数能做到的贴近程度」这一档；实测分层后恒为 0.400，不分层在 8 个种子上是
        1.000–2.000。三段大小不因分层而变（分层只搬成员，不改配额）。
        """
        ops = DatasetOperations()
        plain = ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=seed))
        strat = ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=seed, stratify=True,
                                              stratify_key="intent"))
        assert _worst_cell_deviation(strat, CORPUS, RATIOS) < 0.5
        assert _worst_cell_deviation(plain, CORPUS, RATIOS) >= 1.0
        assert {it["intent"] for it in strat[1]} == {"c0", "c1", "c2"}
        assert [len(s) for s in strat] == [len(s) for s in plain] == [18, 6, 6]

    @pytest.mark.parametrize("seed", [0, 3, 17])
    def test_shuffle_false_keeps_original_relative_order(self, seed):
        """`shuffle` 在分层支路上也有活：False 时三段内部按原始相对顺序排

        分工是「`seed` 管可复现，`shuffle` 管段内顺序」：成员集合与条数与 `shuffle=True`
        相同，只有顺序不同。旧实现里分层支路不存在，`stratify=True, shuffle=False` 交出
        的是「原序连续切三段」，成员与打乱档相同这一条就不成立。
        """
        ops = DatasetOperations()
        shuffled = ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=seed, stratify=True,
                                                 stratify_key="intent"))
        ordered = ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=seed, shuffle=False,
                                                stratify=True, stratify_key="intent"))
        for seg_shuffled, seg_ordered in zip(shuffled, ordered):
            assert Counter(id(item) for item in seg_ordered) == \
                Counter(id(item) for item in seg_shuffled)
            positions = [item["pos"] for item in seg_ordered]
            assert positions == sorted(positions)
        assert sum(len(s) for s in ordered) == len(CORPUS)

    def test_stratified_with_same_seed_is_reproducible(self):
        """同种子两次调用逐条同答（对照组：缺陷态也绿，钉的是「接线没引入新随机源」）"""
        ops = DatasetOperations()
        config = SplitConfig(ratios=RATIOS, seed=11, stratify=True, stratify_key="intent")
        assert ops.split(CORPUS, config) == ops.split(CORPUS, config)

    @pytest.mark.parametrize("key", ["", None])
    def test_blank_stratify_key_is_rejected(self, key):
        """开了分层却没字段：报错，不静默退化成不分层

        静默退化正是本轮修掉的缺陷形状（旧实现对任何 `stratify_key` 都不响应）。
        """
        ops = DatasetOperations()
        with pytest.raises(DataValidationError, match="分层分割需要非空"):
            ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=1, stratify=True,
                                          stratify_key=key))

    @pytest.mark.parametrize("ratios", [
        (1.5, -0.5, 0.0),
        (0.8, 0.1, 0.105),
    ], ids=["negative", "sum_within_loose_tolerance"])
    def test_strict_ratio_rules_apply_on_the_stratified_path(self, ratios):
        """分层支路沿用 `DataSplitter` 的严口径（±1e-6 且不许负数）

        默认路径只要求比例和落在 ±0.01 内、也不查负数，所以同一份配置在两条支路上
        的**可接受域**不同：这是接线方式带来的既有分歧（两处校验本就不一致，另立
        Backlog 统一），本轮先把它钉成事实，免得日后有人当成新引入的回归。
        """
        ops = DatasetOperations()
        plain = ops.split(CORPUS, SplitConfig(ratios=ratios, seed=1))
        assert len(plain[0]) + len(plain[1]) + len(plain[2]) == len(CORPUS)
        with pytest.raises(DataValidationError):
            ops.split(CORPUS, SplitConfig(ratios=ratios, seed=1, stratify=True,
                                          stratify_key="intent"))

    @pytest.mark.parametrize("config", [
        SplitConfig(ratios=(0.8, 0.1, 0.1), seed=7),
        SplitConfig(ratios=(0.6, 0.2, 0.2), shuffle=False),
        SplitConfig(ratios=(1.0, 0.0, 0.0), seed=3),
        SplitConfig(ratios=(0.34, 0.33, 0.33), seed=5, stratify_key="intent"),
    ], ids=["seeded", "ordered", "one_segment", "stratify_key_only"])
    def test_unstratified_default_path_unchanged(self, config):
        """对照组：`stratify=False` 的默认路径与 L40 提交态复刻逐条同答

        钉「本轮没动默认路径」。缺陷态同样绿（那时所有请求都走这条），所以它是
        白名单里的护栏，不计入成效。
        """
        ops = DatasetOperations()
        assert ops.split(CORPUS, config) == _legacy_plain_split(CORPUS, config)

    def test_split_dataset_helper_writes_stratified_files(self, tmp_path):
        """端到端：便捷函数 `split_dataset(...)` 的分层参数要落到写盘产物上

        缺陷态对任何 `stratify` 取值都走不分层那条路，实测 seed=0 的验证集只含 c1、c2
        两类（c0 整类缺席）；接上分层后三段都覆盖三类。这条挂在文件级入口上，因为
        CLI `split` 与 API `/api/dataset/split` 走的是同一个 `split_file`。
        """
        src = tmp_path / "src.json"
        src.write_text(json.dumps(CORPUS, ensure_ascii=False), encoding="utf-8")
        out_dir = tmp_path / "split"
        result = split_dataset(str(src), str(out_dir), ratios=RATIOS, seed=0,
                               stratify=True, stratify_key="intent")
        assert [result["splits"][name]["count"] for name in ("train", "val", "test")] == \
            [18, 6, 6]
        classes = {}
        for name in ("train", "val", "test"):
            data = json.loads((out_dir / f"{name}.json").read_text(encoding="utf-8"))
            classes[name] = {item["intent"] for item in data}
        assert classes["val"] == {"c0", "c1", "c2"}
        assert classes["train"] == classes["test"] == {"c0", "c1", "c2"}

    @pytest.mark.parametrize("config", [
        SplitConfig(ratios=(0.8, 0.1, 0.1), seed=7),
        SplitConfig(ratios=(0.6, 0.2, 0.2), shuffle=False),
        SplitConfig(ratios=(0.34, 0.33, 0.33), stratify_key="intent"),
    ], ids=["seeded", "ordered", "key_only"])
    def test_unstratified_path_never_constructs_data_splitter(self, monkeypatch, config):
        """接线不许越界：`stratify=False` 时连 `DataSplitter` 都不该被构造

        钉的是本轮的范围决定。两条支路的可复现性口径本就不同（`stratify=False` 且
        `shuffle=False` 时全确定，分层时组序必随机、要靠 `seed` 才复现），哪天有人
        「统一走一层」即便答案侥幸相同也算改口径 ⇒ 这条要红。
        """
        from augmentor import dataset_ops as dataset_ops_module

        def boom(*args, **kwargs):
            raise AssertionError("default path must not touch DataSplitter")

        # 缺陷态（HEAD）的模块里根本没有这个属性，`raising=False` 让装上桩这一步不报错
        monkeypatch.setattr(dataset_ops_module, "DataSplitter", boom, raising=False)
        ops = DatasetOperations()
        train, val, test = ops.split(CORPUS, config)
        assert len(train) + len(val) + len(test) == len(CORPUS)

    @pytest.mark.parametrize("shuffle", [True, False], ids=["shuffled", "ordered"])
    def test_order_restore_runs_only_when_shuffle_is_off(self, monkeypatch, shuffle):
        """代价护栏：保序重排只在 `shuffle=False` 时发生，且三段各一次

        双向都要钉。打乱档一次都不许调用（那是纯白付的 O(n) 开销，实测真实 6,902 条
        语料上要多花 3.2–3.9 ms）；保序档必须恰好三次 —— 少一次就说明某一段没排回去。
        缺陷态里「打乱档 0 次」这侧天然成立（还没有分层支路），红在保序那侧。
        """
        from augmentor import dataset_ops as dataset_ops_module

        seen = []
        real = getattr(dataset_ops_module, "_in_original_order", None)
        if real is None:  # 缺陷态还没有这个符号：让它一旦被调用就崩，计数保持 0
            def real(*args, **kwargs):  # noqa: ANN001
                raise AssertionError("restore helper is missing")

        def spy(*args, **kwargs):
            seen.append(args)
            return real(*args, **kwargs)

        # 缺陷态（HEAD）的模块里还没有这个符号，`raising=False` 让装上桩这一步不报错：
        # 否则该用例红在「装不上桩」，而不是它真正要钉的调用次数上。
        monkeypatch.setattr(dataset_ops_module, "_in_original_order", spy, raising=False)
        ops = DatasetOperations()
        ops.split(CORPUS, SplitConfig(ratios=RATIOS, seed=5, shuffle=shuffle,
                                      stratify=True, stratify_key="intent"))
        assert len(seen) == (0 if shuffle else 3)


def _sample_corpus():
    """4 类 × 5 条（共 20 条）的分层采样语料

    `instruction` 在同类内完全相同 —— 分组键取它的前 10 字（见 `_stratified_sample`），
    类内加序号会把 20 条拆成 20 组，就测不出「类与类之间怎么分余数」。`output` 每条唯一，
    于是按 `instruction` 分层与按 `output` 分层交出两种形状，`stratify_key` 有没有真的
    传到分组那一侧，一条断言就能分辨。
    """
    return [{"instruction": f"类别{cat}", "output": f"{cat}答案{i}"}
            for cat in ("甲", "乙", "丙", "丁") for i in range(1, 6)]


SAMPLE_CORPUS = _sample_corpus()


def _class_counts(sampled):
    """按 `instruction` 类计数（只保留出现过的类）

    Args:
        sampled: 采样结果

    Returns:
        类名 -> 条数
    """
    return dict(Counter(item["instruction"] for item in sampled))


class TestSampleStratifyKey:
    """A56 同族站点：`SampleConfig.stratify_key` 的判据与真接线

    L41 给分割支路定了「空 `stratify_key` 报错，不静默退化」的口径，但采样支路是另一份
    分组代码：实测（Temp `l42b.py`）空键时 4 类 × 5 条取 8 条交出 `5/2/1` 的随机形状，
    而正常键恒为 `2/2/2/2` —— 与 A54「旋钮不被读就等于没有」同形。本轮把两条口径对齐。
    """

    @pytest.mark.parametrize("key", ["", None], ids=["empty", "none"])
    def test_blank_stratify_key_is_rejected(self, key):
        """开了分层采样却没字段：报错，不静默退化成随机挑条

        缺陷态里 `item.get("", "")` 对每条都交出 `""`，20 条全落进同一个 `empty` 组，
        配额等于对整份语料做一次 `rng.sample` —— 用户拿到的是随机样本却以为是分层样本。
        """
        ops = DatasetOperations()
        with pytest.raises(DataValidationError, match="分层采样需要非空"):
            ops.sample(SAMPLE_CORPUS, SampleConfig(method="stratified", size=8,
                                                   seed=7, stratify_key=key))

    @pytest.mark.parametrize("method", ["random", "systematic"],
                             ids=["random", "systematic"])
    def test_blank_key_still_allowed_for_other_methods(self, method):
        """判据只管分层支路：另外两种方法不读键，不许被顺手拦下"""
        ops = DatasetOperations()
        sampled = ops.sample(SAMPLE_CORPUS, SampleConfig(method=method, size=8,
                                                         seed=7, stratify_key=""))
        assert len(sampled) == 8

    @pytest.mark.parametrize("key,expected", [
        ("instruction", {"类别甲": 2, "类别乙": 2, "类别丙": 2, "类别丁": 2}),
        ("output", {"类别甲": 5, "类别乙": 3}),
    ], ids=["by_class", "by_unique_field"])
    def test_stratify_key_selects_the_grouping(self, key, expected):
        """键必须是分组字段：换字段就换形状

        `size=8, seed=7`。按 `instruction` 分层是 4 类各 2 条（在 seed 0/1/7 上恒等）；
        按 `output` 分层时 20 条各自成组、每组配额 1，于是整份样本只覆盖到前两类的
        头 8 条 —— 形状完全不同。缺陷态（把键硬编码成 `instruction`）在第二行上必红。
        """
        ops = DatasetOperations()
        sampled = ops.sample(SAMPLE_CORPUS, SampleConfig(method="stratified", size=8,
                                                         seed=7, stratify_key=key))
        assert len(sampled) == 8
        assert _class_counts(sampled) == expected
