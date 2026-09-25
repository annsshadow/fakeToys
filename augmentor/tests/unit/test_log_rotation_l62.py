# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L62 / A104：`logging.file` 的落盘从「无界」改为「有界轮转」

改前 `logging_setup._open_file_handler` 用裸 `logging.FileHandler`（默认 `mode='a'`、
无 `maxBytes`、无 `backupCount`）。一个开了 `logging.file` 的长驻 `augmenter serve`
/ API 进程把 INFO/DEBUG 一路追加，文件会一直写到把磁盘写满 —— 没有任何界兜底。

L62 把它换成 `logging.handlers.RotatingFileHandler`：追加语义一字未动（`mode` 仍是
`a`，「重启不清零」不变），只是加了 `maxBytes` / `backupCount` 的上界。本文件锁死三件
事：装的是带界的轮转 handler、真的会滚出备份、以及打不开时的 `DataValidationError`
契约（A97 时代立的）没被这次换型丢掉。
"""

import logging
import logging.handlers

from augmentor import logging_setup
from augmentor.config import LoggingConfig
from augmentor.exceptions import DataValidationError


def _emit(handler, n, msg_len=40):
    """通过 handler 直接写 n 条定长 record，绕开 root logger 的级别过滤"""
    for i in range(n):
        rec = logging.LogRecord(
            "x", logging.INFO, __file__, 1, ("L%06d_" % i) + "z" * msg_len, None, None
        )
        handler.emit(rec)


def test_open_file_handler_returns_bounded_rotating_handler(tmp_path):
    """旋钮换型：装的是 RotatingFileHandler，且带着模块级上界"""
    path = str(tmp_path / "app.log")
    h = logging_setup._open_file_handler(path)
    try:
        assert type(h) is logging.handlers.RotatingFileHandler
        assert h.maxBytes == logging_setup._LOG_FILE_MAX_BYTES
        assert h.backupCount == logging_setup._LOG_FILE_BACKUP_COUNT
        assert h.maxBytes > 0, "上界被设成 0 = 等于没封顶"
    finally:
        h.close()


def test_log_file_stays_bounded_and_rolls_backup(tmp_path, monkeypatch):
    """真滚：超过 maxBytes 后主文件不无界增长，且滚出 .1 备份"""
    monkeypatch.setattr(logging_setup, "_LOG_FILE_MAX_BYTES", 400)
    log = tmp_path / "app.log"
    h = logging_setup._open_file_handler(str(log))
    try:
        _emit(h, 60, msg_len=60)  # 远超 400 字节 ⇒ 必然多次轮转
    finally:
        h.close()
    assert log.with_name(log.name + ".1").exists(), "超过上界却没滚出 .1 = 封顶没生效"
    assert log.stat().st_size <= 400 + 200, "主文件突破了 maxBytes 上界"


def test_unopenable_path_still_raises_data_validation_error(tmp_path):
    """换型没丢 A97 契约：路径打不开仍是可行动的 DataValidationError"""
    missing_dir = tmp_path / "nope" / "deep" / "app.log"
    try:
        logging_setup._open_file_handler(str(missing_dir))
    except DataValidationError as exc:
        assert "logging.file 打不开" in str(exc)
    else:
        raise AssertionError("父目录不存在时应抛 DataValidationError，却静默成功")


def test_apply_logging_config_installs_bounded_file_handler(tmp_path):
    """装配面：写了 logging.file 时 root 上那条确实是有界的轮转 handler"""
    log = tmp_path / "app.log"
    settings = LoggingConfig(level="INFO", file=str(log), format="%(message)s")
    summary = logging_setup.apply_logging_config(settings, written={"file"})
    try:
        assert summary["file"] == str(log.resolve())
        installed = [
            h for h in logging.getLogger().handlers
            if type(h) is logging.handlers.RotatingFileHandler
        ]
        assert len(installed) == 1, "写一个 logging.file 只应装一条文件 handler"
        assert installed[0].maxBytes == logging_setup._LOG_FILE_MAX_BYTES
    finally:
        logging_setup._detach_installed(logging.getLogger())
