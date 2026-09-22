"""api.deps 依赖层单元测试

覆盖惰性 get_pipeline、reset_pipeline、同步/异步 JSON 读写、
require_file 校验与 run_in_thread 线程池执行。
"""

import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture(autouse=True)
def _isolate_pipeline_state():
    """每个测试前后重置全局管道单例，避免状态泄漏

    Yields:
        None
    """
    import api.deps as deps
    saved = deps._pipeline
    deps.reset_pipeline()
    yield
    deps._pipeline = saved


class TestLazyPipeline:
    def test_get_pipeline_lazy_init(self, tmp_path, monkeypatch):
        """_pipeline 为空时 get_pipeline 需惰性构造（离线可降级）"""
        import api.deps as deps

        monkeypatch.chdir(tmp_path)
        # CWD 无 config.yaml 时 load_config 走默认配置，不得抛异常
        pipeline = deps.get_pipeline()
        assert pipeline is not None
        assert pipeline is deps._pipeline

    def test_reset_pipeline(self):
        import api.deps as deps

        deps._pipeline = object()
        deps.reset_pipeline()
        assert deps._pipeline is None


class TestFileHelpers:
    def test_require_file_missing_raises_404(self, tmp_path):
        from fastapi import HTTPException

        import api.deps as deps

        with pytest.raises(HTTPException) as exc_info:
            deps.require_file(str(tmp_path / "nope.json"))
        assert exc_info.value.status_code == 404

    def test_require_file_existing_returns_path(self, tmp_path):
        import api.deps as deps

        f = tmp_path / "ok.json"
        f.write_text("[]", encoding="utf-8")
        assert deps.require_file(str(f)) == f

    def test_save_items_roundtrip(self, tmp_path):
        import api.deps as deps

        target = tmp_path / "sub" / "items.json"
        items = [{"instruction": "q", "input": "", "output": "a"}]
        deps.save_items(str(target), items)
        assert json.loads(target.read_text(encoding="utf-8")) == items

    def test_read_json_file_async(self, tmp_path):
        import asyncio

        import api.deps as deps

        f = tmp_path / "a.json"
        f.write_text(json.dumps([1, 2]), encoding="utf-8")

        async def run():
            return await deps.read_json_file(f)

        assert asyncio.run(run()) == [1, 2]

    def test_write_json_file_async(self, tmp_path):
        import asyncio

        import api.deps as deps

        f = tmp_path / "b.json"

        async def run():
            await deps.write_json_file(f, [{"x": 1}])

        asyncio.run(run())
        assert json.loads(f.read_text(encoding="utf-8")) == [{"x": 1}]

    def test_run_in_thread_executes_func(self):
        import asyncio

        import api.deps as deps

        result = asyncio.run(deps.run_in_thread(lambda: 40 + 2))
        assert result == 42
