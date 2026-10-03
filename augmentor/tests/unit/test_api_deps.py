# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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
        f.write_text(json.dumps([{"instruction": "q1"}, {"instruction": "q2"}]),
                     encoding="utf-8")

        async def run():
            return await deps.read_json_file(f)

        assert asyncio.run(run()) == [{"instruction": "q1"}, {"instruction": "q2"}]

    def test_read_json_file_async_applies_shape_gate(self, tmp_path):
        """异步读盘同样受形态校验约束

        原夹具写的是 ``[1, 2]`` 并断言原样返回 —— 那恰好把「非对象数据项也照收」
        这个缺陷语义钉住了：`read_json_file` 只是 `read_items` 的执行器包装，
        两者形态判据必须一致，否则异步路径会重新漏出 500。
        """
        import asyncio

        import api.deps as deps
        from fastapi import HTTPException

        f = tmp_path / "shape.json"
        f.write_text(json.dumps([1, 2]), encoding="utf-8")

        with pytest.raises(HTTPException) as exc:
            asyncio.run(deps.read_json_file(f))
        assert exc.value.status_code == 400
        assert "必须是 JSON 对象" in exc.value.detail

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


class TestConfigDataRootsMissingFile:
    def test_missing_config_file_skips_cache(self):
        """config 文件 stat 不了时走降级键：默认根照常返回，且不落缓存（L102，A202）

        `_config_data_roots` 的 docstring 口径：「文件不存在（stat 不了）时不缓存，
        免得把降级路径一起冻住」——若这一支把 ``(path, None)`` 也写进缓存，配置文件
        修好之前 mtime 永远不会跳变，降级结果会被冻住，缓存失效机制对它整体失明。
        """
        import api.deps as deps

        deps._config_roots_cache.clear()
        missing = Path("l102-definitely-not-present.yaml")
        roots = deps._config_data_roots(missing)
        assert len(roots) == 1 and roots[0].name == "data"
        assert deps._config_roots_cache.get((str(missing), None)) is None


class TestToHttpError500LeakClosed:
    """L179（A155①）：500 档回固定文案、不再转发异常原文（部署路径 / 文件系统状态）。

    判据钉三格：① 未分类异常（非 ValueError）⇒ 500 + 固定 ``INTERNAL_ERROR_DETAIL``，
    且部署绝对路径与 errno 文案**不得**再出现在响应体；② 原文改走服务端日志（ERROR 级）；
    ③ 400 档（ValueError 可行动文案）保持原样，防修过头把客户端可行动信息也抹掉。
    """

    def test_unclassified_error_returns_fixed_detail_not_str(self):
        import api.deps as deps

        exc = OSError(13, "Permission denied", "D:/deploy/sensitive/secret.json")
        err = deps.to_http_error(exc)
        assert err.status_code == 500
        assert err.detail == deps.INTERNAL_ERROR_DETAIL
        for leaked in ("D:/deploy/sensitive", "Permission denied", "Errno 13", "secret.json"):
            assert leaked not in err.detail, leaked

    def test_unclassified_error_goes_to_server_log(self, caplog):
        import api.deps as deps

        with caplog.at_level("ERROR", logger="api.deps"):
            deps.to_http_error(RuntimeError("boom-detail"))
        assert any("未分类异常收敛为 500" in rec.message for rec in caplog.records), \
            "500 档必须 logger.exception 落服务端日志"

    def test_value_error_still_400_with_actionable_detail(self):
        import api.deps as deps

        err = deps.to_http_error(ValueError("比例须在 [0,1] 区间"))
        assert err.status_code == 400
        assert err.detail == "比例须在 [0,1] 区间"


class TestRaiseInternalErrorL180:
    """L180（A155②）：路由裸 500 统一走 raise_internal_error——原文落日志、客户端拿固定文案。

    三格：① 助手 raise 的是 500 + 固定文案、异常原文不进 detail；② 原文进服务端日志；
    ③ **静态形状棘轮**——api/routes 下不得再出现「裸 500 detail=str(异常)」写法
    （谁把某路由的收尾分支改回转发原文 ⇒ 当场红）。
    """

    def test_helper_raises_fixed_500_not_str(self, caplog):
        import api.deps as deps
        from fastapi import HTTPException

        with caplog.at_level("ERROR", logger="api.deps"):
            with pytest.raises(HTTPException) as ei:
                deps.raise_internal_error(OSError(13, "Permission denied", "D:/deploy/x.json"))
        assert ei.value.status_code == 500
        assert ei.value.detail == deps.INTERNAL_ERROR_DETAIL
        for leaked in ("D:/deploy/x.json", "Permission denied"):
            assert leaked not in ei.value.detail
        assert any("未分类异常收敛为 500" in r.message for r in caplog.records)

    def test_no_raw_500_detail_str_leak_in_routes(self):
        import pathlib

        routes = pathlib.Path(__file__).resolve().parents[2] / "api" / "routes"
        offenders = []
        for py in routes.glob("*.py"):
            text = py.read_text(encoding="utf-8")
            for i, line in enumerate(text.splitlines(), 1):
                if "HTTPException(status_code=500, detail=str(" in line:
                    offenders.append(f"{py.name}:{i}")
        assert not offenders, "裸 500 detail=str(...) 回流：\n" + "\n".join(offenders)
