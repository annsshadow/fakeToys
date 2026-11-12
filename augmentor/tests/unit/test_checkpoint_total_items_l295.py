# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L295: checkpoint total_items 与实际输入校平守卫

A406: `CheckpointManager.get_remaining_indices`把盘上断点的 `total_items`当全集权威，
从不与本轮实际读入的`len(items)`校平 ⇒ 输入文件在两次运行之间变短就越界、
变长就漏尾巴，同步与异步两入口同形。

## 判据

### 改前形状（必现缺陷）
1. **断点总条目 > 实际输入**：`total_items=10`, `completed=[0,1,2]`, 本轮只 6 条 → 
   `get_remaining_indices()` 返回 `[3,4,5,6,7,8,9]`，随后`items[idx]` 抛裸`IndexError`
2. **断点总条目 < 实际输入**：`total_items=6`, `completed=[0,1]`, 本轮 10 条 → 
   `get_remaining_indices()` 返回`[2,3,4,5]`，新增尾部 **6..9 四条永不进入处理集合**

### 改后形状（期望行为）
以本轮 `actual_length` 为全集权威（断点 `total_items` 只是上一轮记录）：
1. 变短时：返回剩余有效下标，不越界
2. 变长时：补齐新增尾巴的下标

## 三面

- **AST 名册面**：检查 `get_remaining_indices`签名是否增加`actual_length` 可选参
- **行为面对账**：注入臂复现改前形状必红，修法后必绿
- **反证臂**：去掉`actual_length` 参数旧形回归必红

"""

from pathlib import Path
from typing import List, Optional
import pytest
import sys
import os

# 添加 augmentor 到路径
sys.path.insert(0, str(Path(__file__).parent.parent / "augmentor"))

from augmentor.checkpoint import CheckpointManager, CheckpointData


def make_manager(tmp_path: Path) -> CheckpointManager:
    """构造器工厂"""
    return CheckpointManager(str(tmp_path))


class TestActualLengthParameterL295:
    """AST 名册面：签名字段检查"""
    
    def test_signature_has_actual_length_optional_param(self):
        """get_remaining_indices 签名增加 actual_length 可选参"""
        import inspect
        sig = inspect.signature(CheckpointManager.get_remaining_indices)
        params = list(sig.parameters.keys())
        
        # 必须有 actual_length 参数
        assert "actual_length" in params, \
            f"expected 'actual_length' param in signature, got {params}"
        
        # 必须是可选的（有默认值 None）
        actual_length_param = sig.parameters["actual_length"]
        assert actual_length_param.default is None, \
            "actual_length must be optional with default None"


class TestInputShorterThanCheckpointL295:
    """改前①：输入变短时防止 IndexErrors"""
    
    @pytest.fixture
    def tmp_path(self, tmp_path_factory):
        return tmp_path_factory.mktemp("l295_short")
    
    def test_no_index_error_when_input_shorter(self, tmp_path):
        """输入 6 条 vs 断点 10 条 → 只返回 [3,4,5] 而非越界的 [3..9]"""
        mgr = make_manager(tmp_path)
        
        # 模拟旧断点：总共 10 条，已完成 3 条
        ckpt = CheckpointData(
            task_id="test",
            total_items=10,
            completed_indices=[0, 1, 2],
            failed_indices=[],
            quality_scores={},
            processed_items=3,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        # 今轮只 6 条输入
        actual_length = 6
        
        # 修法后应只返回有效范围 [3,4,5]
        remaining = mgr.get_remaining_indices(ckpt, actual_length=actual_length)
        
        assert remaining == [3, 4, 5], \
            f"expected [3,4,5] for 6 items vs checkpoint 10, got {remaining}"
        assert len(remaining) == 3, "should have 3 remaining items"
        
        # 验证不会越界
        for idx in remaining:
            assert idx < actual_length, \
                f"index {idx} exceeds actual_length {actual_length}"
    
    def test_remapped_indices_with_actual_length(self, tmp_path):
        """断点变短时，下标按实际输入重映射"""
        mgr = make_manager(tmp_path)
        
        # 断点：总共 10 条，已完成全部 [0..9]
        ckpt = CheckpointData(
            task_id="test",
            total_items=10,
            completed_indices=list(range(10)),
            failed_indices=[],
            quality_scores={},
            processed_items=10,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        # 今轮只有 4 条输入
        actual_length = 4
        
        # 所有有效项都已完成 → 无剩余
        remaining = mgr.get_remaining_indices(ckpt, actual_length=actual_length)
        assert remaining == [], \
            f"expected empty when all effective items completed, got {remaining}"
    
    def test_failed_indices_excluded_too(self, tmp_path):
        """失败项同样排除，且受实际长度限制"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=10,
            completed_indices=[0, 1],
            failed_indices=[2, 3],
            quality_scores={},
            processed_items=4,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=2
        )
        
        actual_length = 5
        
        # 有效范围 0..4，已完成/失败：0,1,2,3 → 只剩 4
        remaining = mgr.get_remaining_indices(ckpt, actual_length=actual_length)
        assert remaining == [4], \
            f"expected [4], got {remaining}"


class TestInputLongerThanCheckpointL295:
    """改前②：输入变长时补齐新增尾巴"""
    
    @pytest.fixture
    def tmp_path(self, tmp_path_factory):
        return tmp_path_factory.mktemp("l295_long")
    
    def test_new_tail_items_included(self, tmp_path):
        """断点 6 条 vs 实际 10 条 → 补齐 6..9 四条"""
        mgr = make_manager(tmp_path)
        
        # 旧断点：总共 6 条，已完成 2 条
        ckpt = CheckpointData(
            task_id="test",
            total_items=6,
            completed_indices=[0, 1],
            failed_indices=[],
            quality_scores={},
            processed_items=2,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        # 今轮新增到 10 条
        actual_length = 10
        
        # 应以实际长度为全集：0..9，扣除已完成的 0,1 → 返回 [2..9]
        remaining = mgr.get_remaining_indices(ckpt, actual_length=actual_length)
        
        expected = list(range(2, 10))
        assert remaining == expected, \
            f"expected {expected}, got {remaining}. New tail should be included."
    
    def test_tail_after_failed_items(self, tmp_path):
        """失败项后的新增尾巴也应包含"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=6,
            completed_indices=[0],
            failed_indices=[1, 2, 3],
            quality_scores={},
            processed_items=4,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        actual_length = 10
        
        # 有效范围 0..9，已完成/失败：0,1,2,3 → 返回 [4..9]
        remaining = mgr.get_remaining_indices(ckpt, actual_length=actual_length)
        expected = list(range(4, 10))
        
        assert remaining == expected, \
            f"expected {expected}, got {remaining}"


class TestNoActualLengthArgumentL295:
    """反证臂：不传 actual_length 时应回退到旧行为（仅用于证明修法有效）"""
    
    @pytest.fixture
    def tmp_path(self, tmp_path_factory):
        return tmp_path_factory.mktemp("l295_noarg")
    
    def test_fallback_to_total_items_without_actual_length(self, tmp_path):
        """不传 actual_length 时使用 total_items（旧行为，用于向后兼容）"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=10,
            completed_indices=[0, 1, 2],
            failed_indices=[],
            quality_scores={},
            processed_items=3,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        # 不传 actual_length → 使用 total_items=10
        remaining = mgr.get_remaining_indices(ckpt)
        
        assert remaining == [3, 4, 5, 6, 7, 8, 9], \
            "without actual_length, should fall back to total_items behavior"
    
    def test_behavior_change_requires_actual_length(self, tmp_path):
        """修法的有效性：只有传入 actual_length 才触发校平逻辑"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=10,
            completed_indices=[0, 1, 2],
            failed_indices=[],
            quality_scores={},
            processed_items=3,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        # 带 actual_length=6 → [3,4,5]
        with_actual = mgr.get_remaining_indices(ckpt, actual_length=6)
        
        # 不带 actual_length → [3,4,5,6,7,8,9]
        without_actual = mgr.get_remaining_indices(ckpt)
        
        assert with_actual != without_actual, \
            "actual_length parameter should change behavior when input shorter than checkpoint"
        assert with_actual == [3, 4, 5]
        assert without_actual == [3, 4, 5, 6, 7, 8, 9]


class TestEdgeCasesL295:
    """边界情况测试"""
    
    @pytest.fixture
    def tmp_path(self, tmp_path_factory):
        return tmp_path_factory.mktemp("l295_edge")
    
    def test_equal_lengths(self, tmp_path):
        """断点与实际长度相等 → 正常计算"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=5,
            completed_indices=[0, 1],
            failed_indices=[],
            quality_scores={},
            processed_items=2,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        remaining = mgr.get_remaining_indices(ckpt, actual_length=5)
        assert remaining == [2, 3, 4]
    
    def test_actual_length_zero(self, tmp_path):
        """实际长度为 0 → 无剩余项"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=10,
            completed_indices=[],
            failed_indices=[],
            quality_scores={},
            processed_items=0,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        remaining = mgr.get_remaining_indices(ckpt, actual_length=0)
        assert remaining == []
    
    def test_actual_length_extremely_long(self, tmp_path):
        """实际长度远大于断点 → 以本轮实际输入为准，新增尾巴全部纳入"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=5,
            completed_indices=[],
            failed_indices=[],
            quality_scores={},
            processed_items=0,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        # 断点只有 5 条，本轮实际 100 条 → 全集按 100 条算
        remaining = mgr.get_remaining_indices(ckpt, actual_length=100)
        assert remaining == list(range(100))
        assert max(remaining) == 99
    
    def test_all_items_completed(self, tmp_path):
        """全部完成 → 空列表"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=5,
            completed_indices=list(range(5)),
            failed_indices=[],
            quality_scores={},
            processed_items=5,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        remaining = mgr.get_remaining_indices(ckpt, actual_length=5)
        assert remaining == []
    
    def test_mixed_completed_and_failed(self, tmp_path):
        """混合完成与失败 → 两者都排除"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=10,
            completed_indices=[0, 2, 4],
            failed_indices=[1, 3, 5],
            quality_scores={},
            processed_items=6,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        remaining = mgr.get_remaining_indices(ckpt, actual_length=10)
        # 0..9 中除去 [0,1,2,3,4,5] → 剩 [6,7,8,9]
        assert remaining == [6, 7, 8, 9]


class TestBackwardsCompatibilityL295:
    """向后兼容性：旧调用方式不受影响"""
    
    @pytest.fixture
    def tmp_path(self, tmp_path_factory):
        return tmp_path_factory.mktemp("l295_compat")
    
    def test_old_calls_still_work(self, tmp_path):
        """不调用 actual_length 参数的旧调用仍可用"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=10,
            completed_indices=[0, 1, 2],
            failed_indices=[3],
            quality_scores={},
            processed_items=4,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        # 旧风格调用
        remaining = mgr.get_remaining_indices(ckpt)
        assert remaining == [4, 5, 6, 7, 8, 9]
    
    def test_none_explicit_argument(self, tmp_path):
        """显式传 None 等价于不传"""
        mgr = make_manager(tmp_path)
        
        ckpt = CheckpointData(
            task_id="test",
            total_items=10,
            completed_indices=[0, 1],
            failed_indices=[],
            quality_scores={},
            processed_items=2,
            start_time=0.0,
            last_update_time=0.0,
            failed_items=0
        )
        
        with_none = mgr.get_remaining_indices(ckpt, actual_length=None)
        without_arg = mgr.get_remaining_indices(ckpt)
        
        assert with_none == without_arg, \
            "actual_length=None should behave like not passing the argument"


class TestPipelineWiringL295:
    """接线面：同步与异步两个入口都必须把本轮 len(items) 交给校平函数

    只测 `get_remaining_indices` 本身，抓不到「调用点忘了传 actual_length」
    这一类回归；A406 的病灶正是两个入口同形，因此把接线也钉住。
    """

    def test_both_entries_pass_actual_length(self):
        import ast

        src = Path(__file__).parent.parent.parent / "augmentor" / "pipeline.py"
        tree = ast.parse(src.read_text(encoding="utf-8"))

        calls = [
            node for node in ast.walk(tree)
            if isinstance(node, ast.Call)
            and isinstance(node.func, ast.Attribute)
            and node.func.attr == "get_remaining_indices"
        ]
        assert len(calls) == 2, f"expected 2 call sites, got {len(calls)}"

        for call in calls:
            kwargs = {kw.arg: kw.value for kw in call.keywords}
            assert "actual_length" in kwargs, ast.dump(call)
            value = kwargs["actual_length"]
            assert (
                isinstance(value, ast.Call)
                and isinstance(value.func, ast.Name)
                and value.func.id == "len"
            ), ast.dump(value)
            assert len(value.args) == 1, ast.dump(value)
            arg = value.args[0]
            assert isinstance(arg, ast.Name) and arg.id == "items", ast.dump(arg)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
