# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L277 安全审计轮：本轮收口的八族缺陷各留一把守卫

每族都带一支「摘掉修复就红」的取向，能纯函数判的不走 HTTP：

1. `WriteTransportGate`（A347）：未配密钥时非回环监听地址上的匿名写被 403。
2. `save_config` 的占位符回写名单含 `base_url`（改前占位符被解析后的真值顶替落盘）。
3. `load_config` 顶层非映射 ⇒ 可行动的 `ConfigError`，不是 `TypeError` / `AttributeError`。
4. `logging.file` 含 NUL 的路径 ⇒ `DataValidationError`，不是裸 `ValueError`。
5. `require_*` 对超大整数不抛 `OverflowError`（经 API 就是 500）。
6. `sample` 的 `ratio` 判在 0-1。
7. `AugmentProgressResponse` 两形都能过校验（改前任务态缺 `status` ⇒ 恒 500）。
8. `DiskCache` 多线程、空数据集导出、空格式清单语义、清洗跨批等价。
"""

import asyncio
import json
import threading
from concurrent.futures import ThreadPoolExecutor

import pytest

from api import deps
from api.middleware.write_auth_gate import (
    WRITE_METHODS,
    WriteTransportGate,
    is_trusted_transport,
)
from augmentor.cache import DiskCache
from augmentor.cleaner import clean_batch_optimized, clean_dataset
from augmentor.export import Exporter
from augmentor.config import AppConfig, load_config, save_config
from augmentor.dataset_ops import DatasetOperations, SampleConfig
from augmentor.exceptions import ConfigError, DataValidationError
from augmentor.logging_setup import _open_file_handler
from augmentor.validation import require_positive, require_ratio, require_seconds


async def _send_json(send, status, payload):
    body = json.dumps(payload).encode()
    await send({"type": "http.response.start", "status": status,
                "headers": [(b"content-type", b"application/json")]})
    await send({"type": "http.response.body", "body": body})


def _drive(gate_cls, method, path, server, app_status=200):
    """用纯 ASGI 消息驱动一次闸：TestClient 给不出任意的 `scope["server"]`"""
    seen = {"down": False, "status": None}

    async def downstream(scope, receive, send):
        seen["down"] = True
        await _send_json(send, app_status, {"ok": True})

    async def receive():
        return {"type": "http.request", "body": b"", "more_body": False}

    async def send(message):
        if message["type"] == "http.response.start":
            seen["status"] = message["status"]

    scope = {"type": "http", "method": method, "path": path, "headers": [],
             "server": server, "query_string": b"", "root_path": ""}
    asyncio.run(gate_cls(downstream)(scope, receive, send))
    return seen["down"], seen["status"]


class TestWriteTransportGate:
    REMOTE = ("10.0.0.5", 8000)
    LOOPBACK = ("127.0.0.1", 8000)

    def test_anonymous_remote_write_is_refused_before_the_app(self, monkeypatch):
        monkeypatch.delenv(deps.API_KEY_ENV, raising=False)
        for method in sorted(WRITE_METHODS):
            down, status = _drive(WriteTransportGate, method, "/api/config", self.REMOTE)
            assert (down, status) == (False, 403), method

    def test_local_write_still_works_with_zero_config(self, monkeypatch):
        """零配置可用是既有契约：本闸只把它的范围收回本机，不取消它"""
        monkeypatch.delenv(deps.API_KEY_ENV, raising=False)
        for server in [self.LOOPBACK, ("127.0.0.2", 80), ("::1", 8000), None,
                       ("", 80), ("testserver", 80)]:
            assert _drive(WriteTransportGate, "POST", "/api/config", server) == (True, 200)

    def test_reads_are_never_touched(self, monkeypatch):
        monkeypatch.delenv(deps.API_KEY_ENV, raising=False)
        assert _drive(WriteTransportGate, "GET", "/api/data/list", self.REMOTE) == (True, 200)

    def test_non_api_paths_are_out_of_scope(self, monkeypatch):
        monkeypatch.delenv(deps.API_KEY_ENV, raising=False)
        assert _drive(WriteTransportGate, "POST", "/docs", self.REMOTE) == (True, 200)

    def test_with_a_key_the_gate_stays_out_of_the_way(self, monkeypatch):
        """设了密钥 ⇒ 契约仍是路由那层的 401，本闸一个字都不管"""
        monkeypatch.setenv(deps.API_KEY_ENV, "s3cret")
        assert _drive(WriteTransportGate, "POST", "/api/config", self.REMOTE) == (True, 200)

    def test_removal_reopens_the_hole(self, monkeypatch):
        """把闸换成直通 ⇒ 同一句请求打到应用；证明 403 是本闸发的，不是别人"""
        monkeypatch.delenv(deps.API_KEY_ENV, raising=False)

        class Passthrough:
            def __init__(self, app):
                self.app = app

            async def __call__(self, scope, receive, send):
                await self.app(scope, receive, send)

        assert _drive(Passthrough, "POST", "/api/config", self.REMOTE) == (True, 200)

    def test_trusted_transport_table(self):
        assert is_trusted_transport(("127.0.0.1", 80)) is True
        assert is_trusted_transport(("127.0.0.2", 80)) is True
        assert is_trusted_transport(("::1", 80)) is True
        assert is_trusted_transport(("10.0.0.5", 80)) is False
        assert is_trusted_transport(("0.0.0.0", 80)) is False
        assert is_trusted_transport(("unix.socket", 80)) is False
        assert is_trusted_transport(None) is True

    def test_the_real_app_has_the_gate_installed(self):
        from api.main import app
        assert any(m.cls is WriteTransportGate for m in app.user_middleware)


class TestConfigFamily:
    def test_base_url_placeholder_survives_save(self, tmp_path, monkeypatch):
        monkeypatch.setenv("AUDIT_BASE_URL", "https://internal.example/v1")
        monkeypatch.setenv("AUDIT_API_KEY", "REALKEY")
        p = tmp_path / "config.yaml"
        p.write_text(
            "models:\n  default: b1\n  b1:\n    type: baidu\n"
            "    base_url: ${AUDIT_BASE_URL}\n    api_key: ${AUDIT_API_KEY}\n",
            encoding="utf-8")
        save_config(load_config(str(p)), str(p))
        text = p.read_text(encoding="utf-8")
        assert "${AUDIT_BASE_URL}" in text
        assert "${AUDIT_API_KEY}" in text
        # 解析后的真值不许落盘（改前 `base_url` 那一格就是把内网 URL 写回磁盘）
        assert "https://internal.example/v1" not in text
        assert "REALKEY" not in text

    def test_plaintext_secret_still_dropped(self, tmp_path):
        """既有的「明文密钥不落盘」不许因为名单换成常量而被放宽"""
        p = tmp_path / "config.yaml"
        p.write_text("models:\n  default: b1\n  b1:\n    type: baidu\n"
                     "    api_key: hard-coded-secret\n", encoding="utf-8")
        save_config(load_config(str(p)), str(p))
        assert "hard-coded-secret" not in p.read_text(encoding="utf-8")

    @pytest.mark.parametrize("body", ["- a\n- b\n", "42\n", "hello\n", "true\n"])
    def test_top_level_non_mapping_is_actionable(self, tmp_path, body):
        p = tmp_path / "config.yaml"
        p.write_text(body, encoding="utf-8")
        with pytest.raises(ConfigError) as exc:
            load_config(str(p))
        assert "映射" in str(exc.value)

    def test_comment_only_config_still_means_defaults(self, tmp_path):
        """「什么都没写」= 全默认，这条既有语义不许被新判据误伤"""
        p = tmp_path / "config.yaml"
        p.write_text("# 只有注释\n", encoding="utf-8")
        assert load_config(str(p)).web.port == AppConfig().web.port

    def test_nul_in_logging_path_is_a_validation_error(self):
        with pytest.raises(DataValidationError):
            _open_file_handler("a" + chr(0) + "b.log")

    @pytest.mark.parametrize("fn, kwargs", [
        (require_seconds, {"name": "s", "value": 10 ** 400, "maximum": 300}),
        (require_seconds, {"name": "s", "value": 10 ** 400}),
        (require_positive, {"name": "p", "value": 10 ** 400}),
        (require_ratio, {"name": "r", "value": 10 ** 400}),
        (require_ratio, {"name": "r", "value": -(10 ** 400)}),
    ])
    def test_huge_int_never_raises_overflow(self, fn, kwargs):
        try:
            fn(**kwargs)
        except DataValidationError:
            pass  # 判负是对的，500 族的裸异常不是
        except OverflowError as exc:
            pytest.fail("裸 OverflowError 经 API 就是 500：" + str(exc))

    @pytest.mark.parametrize("ratio", [2.0, -1.0, 1.5])
    def test_sample_ratio_out_of_zero_one_is_refused(self, ratio):
        items = [{"instruction": "i" + str(i), "output": "o" + str(i)}
                 for i in range(20)]
        with pytest.raises(DataValidationError):
            DatasetOperations().sample(items, SampleConfig(ratio=ratio))

    def test_sample_ratio_in_range_unchanged(self):
        items = [{"instruction": "i" + str(i), "output": "o" + str(i)}
                 for i in range(20)]
        assert len(DatasetOperations().sample(items, SampleConfig(ratio=0.5))) == 10
        assert DatasetOperations().sample(items, SampleConfig(ratio=0.0)) == []


class TestProgressContract:
    EMPTY = {"status": "no_checkpoint"}
    TASK = {"task_id": "t1", "total_items": 10, "processed_items": 4,
            "failed_items": 0, "progress": 0.4, "progress_percent": "40.0%",
            "elapsed_time": "1s", "remaining_time": "2s", "start_time": "now",
            "last_update": "now", "avg_quality_score": 0.8}

    def test_both_shapes_validate(self):
        """任务态不含 `status`：从前该键声明成必填 ⇒ 响应校验当场失败 ⇒ 恒 500"""
        from api.routes.augment import AugmentProgressResponse

        assert AugmentProgressResponse(**self.EMPTY).model_dump(
            exclude_none=True) == self.EMPTY
        parsed = AugmentProgressResponse(**self.TASK).model_dump(exclude_none=True)
        assert parsed == self.TASK
        assert "status" not in parsed

    def test_status_is_optional_on_the_model(self):
        """一个键都不给也必须建得出来：`status` 是可选，不是必填"""
        from api.routes.augment import AugmentProgressResponse

        assert AugmentProgressResponse().model_dump(exclude_none=True) == {}


class TestDiskCacheConcurrency:
    def test_parallel_sets_do_not_raise(self, tmp_path):
        """8 线程共写同一目录：从前淘汰旧键时后到的那次 `del` 当场 KeyError，
        而它不是 `OSError`，穿透 `models/base.py` 的降级捕获，把成功的生成记成失败"""
        cache = DiskCache(cache_dir=str(tmp_path), max_bytes=4096)
        errors = []

        def worker(n):
            try:
                for i in range(60):
                    cache.set("k" + str(n) + "_" + str(i), {"blob": "x" * 200})
                    cache.get("k" + str(n) + "_" + str(i))
            except Exception as exc:
                errors.append(type(exc).__name__ + ": " + str(exc))

        with ThreadPoolExecutor(max_workers=8) as pool:
            for n in range(8):
                pool.submit(worker, n)
        assert errors == []

    def test_lock_is_reentrant_for_nested_paths(self, tmp_path):
        """`get` 的过期分支会调 `delete`：非重入锁会自己死锁"""
        cache = DiskCache(cache_dir=str(tmp_path))
        cache.set("k", 1, ttl=-1)
        assert cache.get("k") is None
        assert isinstance(cache._lock, type(threading.RLock()))


class TestEmptyOutputs:
    def test_empty_csv_still_creates_the_file(self, tmp_path):
        out = tmp_path / "empty.csv"
        Exporter().export([], str(out), format="csv")
        assert out.exists()

    def test_empty_format_list_exports_nothing(self, tmp_path):
        """空清单 = 一个都不导；只有不表态才走默认全格式"""
        from augmentor.pipeline import AugmentorPipeline

        src = tmp_path / "in.json"
        src.write_text(json.dumps([{"instruction": "a", "output": "b"}]),
                       encoding="utf-8")
        out = tmp_path / "out"

        class Stub:
            exporter = Exporter()

        assert AugmentorPipeline.export_dataset(Stub(), str(src), str(out), []) == {}
        assert not out.exists() or list(out.iterdir()) == []
        # 不表态那一支照旧要写出文件（否则这一格是恒真的）
        written = AugmentorPipeline.export_dataset(Stub(), str(src), str(out), None)
        assert written

    def test_cleaner_reports_rules_actually_applied(self):
        items = [{"instruction": "same", "output": "x"},
                 {"instruction": "same", "output": "x"},
                 {"instruction": "other", "output": "y"}]
        _, result = clean_batch_optimized(items, batch_size=2)
        assert "remove_duplicates" in result.rules_applied

    def test_cleaner_is_batch_size_invariant(self):
        items = [{"instruction": "i" + str(i % 4), "output": "o" + str(i)}
                 for i in range(12)]
        single, r1 = clean_dataset(list(items))
        batched, r2 = clean_batch_optimized(list(items), batch_size=3)
        assert len(batched) == len(single)
        assert r2.removed_count == r1.removed_count
        assert r2.rules_applied == r1.rules_applied
