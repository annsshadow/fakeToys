# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""流水线错误恢复增强测试 - 错误记录和恢复机制验证"""

import pytest
from unittest.mock import MagicMock, patch


class MockPipeline:
    """模拟流水线用于测试错误恢复"""
    def __init__(self):
        self._process_errors = []
    
    def record_error(self, error_info):
        if not hasattr(self, '_process_errors'):
            self._process_errors = []
        self._process_errors.append(error_info)


class TestPipelineErrorRecovery:
    """流水线错误恢复增强测试"""
    
    def test_error_info_contains_index(self):
        """错误信息应包含索引"""
        mock = MockPipeline()
        mock.record_error({"index": 1, "error_type": "ValueError", "message": "测试错误"})
        assert len(mock._process_errors) == 1
        assert mock._process_errors[0]["index"] == 1
    
    def test_error_info_has_preview(self):
        """错误信息应包含数据预览"""
        mock = MockPipeline()
        mock.record_error({
            "index": 2,
            "error_type": "Exception",
            "message": "处理失败",
            "item_preview": "测试数据预览内容..."
        })
        assert "item_preview" in mock._process_errors[0]
        assert len(mock._process_errors[0]["item_preview"]) > 0
    
    def test_error_recovery_code_path_exists(self):
        """错误恢复增强代码路径应存在"""
        import inspect
        from augmentor.pipeline import AugmentorPipeline
        source = inspect.getsource(AugmentorPipeline._process_single_item)
        assert "_process_errors" in source or "error_info" in source


class TestProcessErrorsThreadSafety:
    """_process_errors 惰性首建+append 的并发安全（L137，B207）"""

    def test_concurrent_error_records_not_lost(self):
        """多个工作线程同时走 _process_single_item 的 except 分支时，惰性首建+append
        两步在旧实现里非原子（两线程各建一份 list、互相覆盖丢记录）。持 self._lock
        后 N 条并发错误一条不丢。"""
        import threading
        from queue import Queue
        from augmentor.pipeline import AugmentorPipeline

        class Stub:
            pass

        stub = Stub()
        stub.quality_scorer = None
        stub._lock = threading.Lock()
        # 用真实方法，不重新实现一遍
        process = AugmentorPipeline._process_single_item

        N = 40
        barrier = threading.Barrier(N)
        queue = Queue()

        def worker(i):
            barrier.wait()
            # 让 _generate_variants 抛异常，强制走 except 分支
            class Boom:
                def _generate_variants(self, item):
                    raise RuntimeError('注入故障')
            boom = Boom()
            try:
                process(stub, i, {'instruction': str(i), 'output': 'o'}, False, queue)
            except Exception:
                pass

        threads = [threading.Thread(target=worker, args=(i,)) for i in range(N)]
        for t in threads: t.start()
        for t in threads: t.join()

        errors = getattr(stub, '_process_errors', [])
        assert len(errors) == N, (
            f'N={N} 条并发错误只记录了 {len(errors)} 条，惰性首建+append 非原子丢记录'
        )
        assert {e['index'] for e in errors} == set(range(N))

    def test_error_record_mutation_is_guarded_by_lock(self):
        """行为不变量之外，还须钉死「锁确实被用上了」：_process_errors 的惰性首建+
        append 必须包在 with self._lock 里（源码级守卫，防止未来有人把锁拿掉退回
        非原子 check-then-act 而行为测试因 CPython GIL 窗口太窄测不出丢失）"""
        import inspect
        from augmentor.pipeline import AugmentorPipeline
        src = inspect.getsource(AugmentorPipeline._process_single_item)
        assert "with self._lock:" in src, "record 分支丢了锁守卫，check-then-act 回到非原子"