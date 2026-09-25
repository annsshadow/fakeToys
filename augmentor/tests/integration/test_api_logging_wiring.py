# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""API 面上 `logging` 节真的生效（L58，A102 的行为面）

L57 把三根旋钮接上了 `load_config`，但它的 17 例集成全走 CLI 真子进程，单元面又只能
在**同一个**进程里摆 root handler。于是有一条分支今天没有任何一次「真起 API」的验证：

`api/main.py:42` 自己 `basicConfig(level=INFO, format="…%(name)s…")` —— root 一上来就
有且只有一条 handler；随后 `api/main.py:116` 的 `load_config` 命中 `logging` 节时，
`apply_logging_config` 走的是「root 已有 handler」那一支：**不加第二条 handler、级别照写、
formatter 照换**（`logging_setup.py` 的 `if keys and not root.handlers` 与
`for handler in root.handlers`）。这条支的正确性在进程内测不到（pytest 自己的
`LogCaptureHandler` 也在 root 上，形状与 API 那个不是一回事，A97 单测注释里就记过）。

本文件全部**真起子进程 import `api.main`**，把 API 进程的日志形状当观察窗，判据来自
`Temp/l58/probe.py`（NONCE-L58-PROBE）本轮实测的五臂：

1. 没写 `logging` 节 ⇒ root 一条 handler、级别 INFO、格式是 `api/main.py` 自己那副
   （`… - WARNING - name - msg`）—— 接线的默认档不许动 API 自己的形状。
2. `level: ERROR` ⇒ handler 数**仍是 1**（不为「只写级别」凭空装 handler），API 自己的
   WARNING 被静音。
3. `format: "…"` ⇒ 那**一条**既有 handler 被重排，哨兵行只出**一次**（这条用例的存在理由
   就是抓「装第二条 stderr handler ⇒ 每条日志打两遍」这个回退）。
4. `file: x.log` ⇒ handler 数变 2（API 那条 + 文件那条），且**控制台不少字**、文件也落。
5. 只写 `level: INFO` ⇒ `format` 没写就不动，仍是 API 那副带 `%(name)s` 的格式（「没写的
   键一律不动」这条口径在 API 面上的投影）。

`AUGMENTOR_API_KEY` 被设成哑值：不设时 `api/main.py` 会在 import 期打一条「未设置
API_KEY」的 WARNING，那是**产品自发的噪声**，会污染「静音成功 = stderr 为空」这类判据；
设了它就只剩哨兵，观察窗才干净。字节比较顺带钉住行尾。
"""

import os
import subprocess
import sys
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent

#: 底座：只有 `models` 一节，保证各臂的内容差只在 `logging` 节
BASE = "models:\n  default: ernie\n  ernie:\n    type: baidu\n    model: x\n"

#: 哨兵词：import 完成后由子进程自己发的一条 WARNING，唯一稳定的观察对象
SENTINEL = "SENTINEL_L58"

#: 子进程脚本：先 import api.main（触发 basicConfig + load_config），再发哨兵、报 handler 数
CHILD = (
    "import sys, logging\n"
    "import api.main\n"
    "root = logging.getLogger()\n"
    "print('HANDLERS=' + str(len(root.handlers)) + ' LEVEL=' + str(root.level),"
    " file=sys.stderr)\n"
    "logging.getLogger('api.probe').warning('" + SENTINEL + "')\n"
)


def _config(tmp_path, body):
    path = tmp_path / "c.yaml"
    path.write_text(body, encoding="utf-8", newline="\n")
    return path


def _run(tmp_path, body):
    """真起子进程 import `api.main`，返回 `(rc, stderr_bytes, root handler 数)`"""
    cfg = _config(tmp_path, body)
    env = dict(
        os.environ,
        PYTHONIOENCODING="utf-8",
        AUGMENTOR_CONFIG_PATH=str(cfg),
        AUGMENTOR_API_KEY="dummy-key-for-quiet-import",
        PYTHONPATH=str(AI_DIR),
    )
    proc = subprocess.run(
        [sys.executable, "-c", CHILD],
        cwd=str(tmp_path), capture_output=True, env=env, timeout=180)
    handlers = None
    for line in proc.stderr.splitlines():
        if line.startswith(b"HANDLERS="):
            handlers = int(line.split(b"HANDLERS=")[1].split()[0])
    return proc.returncode, proc.stderr, handlers


class TestApiDefaultShapeUntouched:
    """接线不许动 API 进程今天的日志形状"""

    def test_no_section_keeps_api_info_and_its_own_format(self, tmp_path):
        rc, err, handlers = _run(tmp_path, BASE)
        assert rc == 0, f"import api.main 失败：{err!r}"
        assert handlers == 1, f"没写 logging 节却动了 handler 数：{handlers}"
        # 默认档：哨兵按 API 自己那副 `… - WARNING - name - msg` 出现
        line = next((l for l in err.splitlines() if SENTINEL.encode() in l), None)
        assert line is not None, f"默认档下 API 自己的 WARNING 不见了：{err!r}"
        assert b" - WARNING - api.probe - " in line, (
            f"默认档被接线的默认值覆盖了（格式不再是 api/main.py 那副）：{line!r}")


class TestWrittenKeysTakeEffectOnApi:
    """`root 已有 handler` 那一支：写出的键生效，且不为写出的键多装 handler"""

    def test_level_error_mutes_api_warning_without_adding_handler(self, tmp_path):
        rc, err, handlers = _run(tmp_path, BASE + "logging:\n  level: ERROR\n")
        assert rc == 0
        assert handlers == 1, (
            f"「只写 level」那一支不该为装 handler 而 new 第二条，实得 {handlers}")
        assert SENTINEL.encode() not in err, (
            f"`level: ERROR` 没静音 API 自己的 WARNING：{err!r}")

    def test_format_rewrites_the_existing_handler_exactly_once(self, tmp_path):
        """这条是 A102 的存在理由：重排既有 handler，而不是再装一条导致每条打两遍"""
        rc, err, handlers = _run(
            tmp_path,
            BASE + 'logging:\n  level: WARNING\n  format: "APIMARK|%(levelname)s|%(message)s"\n')
        assert rc == 0
        assert handlers == 1, (
            f"写出 format 后多装了 handler（正是本用例要抓的回退）：{handlers}")
        hits = [l for l in err.splitlines() if SENTINEL.encode() in l]
        assert len(hits) == 1, f"哨兵出现 {len(hits)} 次，handler 装重了：{err!r}"
        assert hits[0].startswith(b"APIMARK|WARNING|"), (
            f"写出的 format 没作用到 API 那条既有 handler：{hits[0]!r}")

    def test_unwritten_format_key_is_left_alone(self, tmp_path):
        """只写 level: INFO ⇒ format 不许被 CLI 默认档 `%(message)s` 顺手覆盖"""
        rc, err, handlers = _run(tmp_path, BASE + "logging:\n  level: INFO\n")
        assert rc == 0
        line = next((l for l in err.splitlines() if SENTINEL.encode() in l), None)
        assert line is not None
        assert b" - WARNING - api.probe - " in line, (
            f"只写 level 却重排了没写的 format：{line!r}")


class TestFileKeyOnApi:
    """`logging.file` 在 API 面 = 加一条文件 handler，控制台那条一个字不少"""

    def test_file_adds_handler_and_keeps_the_console(self, tmp_path):
        rc, err, handlers = _run(
            tmp_path, BASE + "logging:\n  level: WARNING\n  file: run.log\n")
        assert rc == 0
        assert handlers == 2, f"写了 file 应是 API 那条 + 文件那条：{handlers}"
        assert SENTINEL.encode() in err, f"控制台那条被 file 键吃掉了：{err!r}"
        log = tmp_path / "run.log"
        assert log.exists(), "写了 logging.file 却没落盘 —— 旋钮在 API 面仍然没人接"
        assert SENTINEL in log.read_text(encoding="utf-8")


class TestBadValueRefusedAtApiLoad:
    """坏值判在加载期：API 进程 import 时就要 rc!=0，而不是拖到第一条日志炸 traceback"""

    def test_bad_level_fails_the_import(self, tmp_path):
        rc, err, _ = _run(tmp_path, BASE + "logging:\n  level: warning\n")
        assert rc != 0, f"小写级别在 API 面没被拒收：{err!r}"
        assert "大写".encode() in err or b"logging.level" in err, (
            f"报错没指向可行动的那件事：{err!r}")
        assert b"Logging error" not in err, "坏值拖到了发日志期"
