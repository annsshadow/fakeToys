# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L123（A205）：偏支逐支审计——九条单偏支的可达/不可达判定

**可达、补测（本文件）**
- performance_benchmark.py 59->63  内存监控不可用档（`HAS_MEMORY_MONITOR` 假支）：
                             模块级开关可被降级环境（memory_monitor 导入失败）置 False，
                             测试直接翻开关 ⇒ `measure_memory_peak` 跳过采样、返回原值
- preview.py 216->218            同实例第二次 `preview` ⇒ `_preview_cache` 已存在，
                             `if not hasattr(self, '_preview_cache'):` 假支
- tracker.py 126->125            指标登记了**但从未记值**（`metrics[name] == []`）⇒
                             `end_experiment` 里 `if values:` 假支（该指标不进 final_metrics）
- logging_setup.py 133->135     撤装时 handler 已被外部从目标 root 摘走 ⇒
                             `if handler in root.handlers:` 假支（跳过 remove、仍 close）
- models/base.py 223->258       双检锁内层假支：并发写者在我方拿到锁之前已把 `_session`
                             置好 ⇒ 内层 `if self._session is None:` 假支（DCL 的**本意
                             场景**，用两个线程确定性触发，不是记档项）

**判定为结构性不可达、记档不测**
- report.py 145->138           `metrics` 是写死的三元组（semantic_similarity/relevance/
                             diversity），走到 :145 时 `metric` 必为 "diversity"（前两个
                             elif 已排除）⇒ `elif metric == "diversity":` 恒真支，假支
                             要有人往 `metrics` 加第四项才见。
- retry.py 197->237           `for attempt in range(max_retries+1)` 的自然耗尽出边：
                             最后一轮必然在 :218 `if attempt >= max_retries: break` 出循环，
                             成功则 :202 已 return，不可重试则 :216 break ⇒ 自然耗尽
                             不可达（:237 的 `raise last_exc` 只由两个 break 喂到）。
- cache.py 291->297           淘汰循环的「零轮」出边：进入循环的前提是
                             `total > _max_bytes`（:289），而 total 由 entries 累加 ⇒
                             entries 必非空 ⇒ `for` 至少一轮，回边不可达。
- api/main.py 193->207       **模块导入期**环境分支：`if not _api_key_required():` 的假支
                             要「设了 `AUGMENTOR_API_KEY` 时重新执行一遍模块顶层」才见。
                             用例在**子进程**里设键后 `import api.main` 并断言 stderr
                             无「未设置」告警——不在测试进程里 reload 生产模块（那会替换
                             全套 TestClient 拿到的 `app` 对象，炸面比一条偏支大）。
"""

import logging
import os
import subprocess
import sys
import threading
from pathlib import Path

from augmentor.config import ModelConfig
from augmentor.logging_setup import _INSTALLED, _detach_installed
from augmentor.models.factory import create_model_backend
from augmentor.performance_benchmark import PerformanceBenchmark
from augmentor.preview import PreviewGenerator
from augmentor.tracker import ExperimentTracker


class TestPerfBenchNoMemoryMonitor:
    """performance_benchmark.py 59->63：内存监控缺失档跳过采样、原样返回"""

    def test_flag_off_returns_stored_peak(self, monkeypatch):
        import augmentor.performance_benchmark as pb

        bench = PerformanceBenchmark()
        bench.memory_peak_mb = 5.0
        monkeypatch.setattr(pb, "HAS_MEMORY_MONITOR", False)
        assert bench.measure_memory_peak() == 5.0


class TestPreviewSecondCallSkipsCacheInit:
    """preview.py 216->218：第二次预览 ⇒ 缓存属性已存在，走 hasattr 假支"""

    def test_second_preview_reuses_lazily_initialised_cache(self):
        # 缓存键是**前 5 条样本数据**的 md5（不含格式，:190），同数据换格式会走
        # :191 命中早退、到不了 :216。要见 :216 的 hasattr 假支，第二次调用必须**换
        # 数据**（键缺失、重建结果、且属性已被第一次调用创建过）。
        previewer = PreviewGenerator()
        items_a = [{"instruction": "问甲", "input": "", "output": "答甲"}]
        items_b = [{"instruction": "问乙", "input": "", "output": "答乙"}]
        r1 = previewer.preview(items_a, "jsonl")
        r2 = previewer.preview(items_b, "csv")
        assert r1.format_info["format"] == "jsonl"
        assert r2.format_info["format"] == "csv"
        assert hasattr(previewer, "_preview_cache")
        assert len(previewer._preview_cache) == 2


class TestTrackerUnrecordedMetric:
    """tracker.py 126->125：登记了但一条值都没记的指标不进 final_metrics"""

    def test_empty_metric_list_skipped_in_final_metrics(self, tmp_path):
        tracker = ExperimentTracker(str(tmp_path / "exp"))
        exp = tracker.start_experiment("e1", "v1", "model-a")
        exp.metrics["recorded"] = [0.5]
        exp.metrics["never_recorded"] = []   # 真实形态：指标被登记、实验里从没采到点
        tracker.end_experiment()
        assert exp.final_metrics == {"recorded": 0.5}
        assert "never_recorded" not in exp.final_metrics


class TestLoggingDetachSkipsExternallyRemovedHandler:
    """logging_setup.py 133->135：handler 已被外部从 root 摘走 ⇒ 跳过 remove、仍 close"""

    def test_detach_skips_handler_absent_from_target_root(self):
        class _RecordingHandler(logging.StreamHandler):
            def __init__(self):
                super().__init__()
                self.close_called = False

            def close(self):
                self.close_called = True
                super().close()

        probe_root = logging.getLogger("l123_detach_probe")
        handler = _RecordingHandler()
        _INSTALLED.append(handler)
        try:
            # handler 不在 probe_root.handlers 里 ⇒ `if handler in root.handlers:` 假支
            _detach_installed(probe_root)
            assert probe_root.handlers == []
            assert handler.close_called is True
        finally:
            _INSTALLED.clear()


class TestModelSessionDclInnerFalseEdge:
    """models/base.py 223->258：双检锁内层假支——并发写者先置好 `_session`

    确定性配方：主线程握着 `_lock` 起 worker（worker 过外层 :221 后卡在锁上），
    主线程随后把 `_session` 置为预建会话再放锁 ⇒ worker 进内层 :223 时
    `self._session is None` 为假 ⇒ 跳过创建、直接返回预建会话。
    """

    def test_worker_sees_session_set_by_concurrent_writer(self):
        config = ModelConfig(type="openai", api_key="k", secret_key="s",
                             base_url="http://localhost:11434", model="m")
        backend = create_model_backend(config)
        prebuilt = backend._get_session()      # 预热一次（外层真支、内层真支）
        backend._session = None               # 复位，制造「还没人建过」的状态

        box: dict = {}

        def worker():
            box["session"] = backend._get_session()

        with backend._lock:
            thread = threading.Thread(target=worker)
            thread.start()
            # 让 worker 走到 :222 的锁等待（外层判据已过，_session 仍为 None）
            import time
            time.sleep(0.05)
            backend._session = prebuilt       # 并发写者抢跑：先建好了会话
        thread.join()

        assert box["session"] is prebuilt     # worker 用的是预建会话，没走创建支
        prebuilt.close()
        backend.close()


class TestApiMainImportsWithKeySet:
    """api/main.py 193->207：设了 `AUGMENTOR_API_KEY` 重新执行模块顶层 ⇒ 无「未设置」告警

    子进程隔离：不动测试进程里的 `api.main` 模块对象（TestClient 全家拿的是它）。
    """

    @staticmethod
    def _import_api_main(env_overrides: dict) -> subprocess.CompletedProcess:
        """在子进程里 `import api.main`，拿回**解码确定**的 stderr

        两侧编码必须同时钉住，否则这条用例的结论随本机 locale 漂移：

        1. 子进程侧 `PYTHONIOENCODING=utf-8`。不设时子进程按 locale 编码
           （这台中文 Windows 是 GBK、Linux CI 是 UTF-8），同一条中文告警有
           两种字节形态。
        2. 父进程侧 `encoding="utf-8"`。`text=True` 不给编码时按
           `locale.getpreferredencoding()` 解，而这台机器是 cp936。

        改前（`text=True` 且两侧都不钉）在项目文档约定的测试命令
        （`OPTIMIZATION_LOOP.md` 基线：`PYTHONIOENCODING=utf-8`）下，子进程写
        UTF-8、父进程按 GBK 解 ⇒ subprocess 读线程 `UnicodeDecodeError` 把
        `proc.stderr` 变成 `None`，`test_warning_when_key_absent` 直接
        TypeError 崩，而它是本文件唯一被这条偏支喂到的用例。同族守门见
        `tests/unit/test_subprocess_text_decoding.py`。
        """
        env = dict(os.environ)
        env.pop("AUGMENTOR_API_KEY", None)
        env.update(env_overrides)
        env["PYTHONIOENCODING"] = "utf-8"
        return subprocess.run(
            [sys.executable, "-c", "import api.main  # noqa: F401"],
            cwd=str(Path(__file__).resolve().parents[2]),
            env=env,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            timeout=120,
        )

    def test_no_warning_when_key_configured(self):
        proc = self._import_api_main({"AUGMENTOR_API_KEY": "test-key"})
        assert proc.returncode == 0, proc.stderr
        assert "未设置" not in proc.stderr

    def test_warning_when_key_absent(self):
        proc = self._import_api_main({})
        assert proc.returncode == 0, proc.stderr
        assert "未设置" in proc.stderr

    def test_decoded_warning_is_the_real_sentence_not_mojibake(self):
        """解码必须真的把中文还原出来，而不是靠 `errors="replace"` 蒙混过关

        `text=True` 不带 `encoding=` 时，即便父进程加了 `errors="replace"`，
        两侧编码不一致也只会把 `未设置` 换成一串 `\\ufffd`——「未设置 not in
        stderr」于是**假绿**。这条钉住告警原文，替换字符一出现就红。
        """
        proc = self._import_api_main({})
        assert proc.returncode == 0, proc.stderr
        assert "未设置 AUGMENTOR_API_KEY" in proc.stderr
        assert "\ufffd" not in proc.stderr
