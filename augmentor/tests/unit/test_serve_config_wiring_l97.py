# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L97：`api/main.py` 的启动面真的读 `web` 节吗（B5 / A178 的第一格收口）

改前的形状（`Temp/l97q/serve_faces.py` 实测）：配置文件里写 `web.host: 127.0.0.1`
与 `web.port: 8137`，`load_config` 逐字读到，而传给 uvicorn 的是两个字面量
`0.0.0.0` / `8000`；`web.static_dir` 改成别的目录，挂载点仍然是硬编码的
`<仓根>/web/dist`。三键在**两侧都有判据**（`WebConfig.__post_init__` 与
`config_validator`），判据管的是「值合不合法」，没有任何一条管「值有没有人用」，
所以它可以一直绿着说谎 —— 其中 `host` 那一格还是安全相关的：想只绑回环的收紧
意图无声失效。

口径：相对 `static_dir` 锚在**仓库根**而不是工作目录，与改动前那条硬编码同一锚点，
所以出厂默认逐字指向同一个目录。接线类改动最容易顺手换掉默认值，因此三条反向用例
钉在原处：`test_factory_defaults_are_not_replaced`（不写 `web` 节时仍绑 `0.0.0.0:8000`）、
`test_relative_dir_anchors_at_repo_root`（默认值仍解析到 `<仓根>/web/dist`，未构建时跳过）、
`test_out_of_range_port_refuses_to_start`（判据没被绕过：越界端口让启动直接抛）。
"""

import logging
import runpy
import sys
import types
from pathlib import Path

import pytest

from augmentor.config import WebConfig
from augmentor.exceptions import DataValidationError

AI_ROOT = Path(__file__).resolve().parents[2]
MAIN = AI_ROOT / "api" / "main.py"


def _run_main(tmp_path, monkeypatch, yaml_text):
    """以 `__main__` 身份跑 api/main.py，返回 (uvicorn 收到的关键字, 模块全局, 挂载点)

    假 uvicorn 的做法与 `tests/unit/test_entrypoint_guards.py` 同源：不真起服务，
    只把 `uvicorn.run` 的关键字抄下来。配置经 `AUGMENTOR_CONFIG_PATH` 指到临时文件，
    所以本用例既不碰仓里那份 `config.yaml`，也不依赖跑测试时的工作目录。

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest monkeypatch
        yaml_text: 写进临时配置的内容

    Returns:
        `(kwargs, globals, [(挂载路径, 静态目录或 None)])`
    """
    cfg = tmp_path / "serve.yaml"
    cfg.write_text(yaml_text, encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_CONFIG_PATH", str(cfg))

    captured = {}

    class FakeUvicorn:
        def run(self, app, *args, **kwargs):
            captured.update(kwargs)

    fake = types.ModuleType("uvicorn")
    fake.run = FakeUvicorn().run
    monkeypatch.setitem(sys.modules, "uvicorn", fake)

    globs = runpy.run_path(str(MAIN), run_name="__main__")
    mounts = [(r.path, getattr(getattr(r, "app", None), "directory", None))
              for r in globs["app"].routes if type(r).__name__ == "Mount"]
    return captured, globs, mounts


def _static_dir_of(mounts):
    """取唯一那个静态挂载点的目录；没挂载时 None"""
    dirs = [d for _, d in mounts if d]
    assert len(dirs) <= 1, f"挂载点不止一个：{mounts}"
    return dirs[0] if dirs else None


class TestBindAddressComesFromConfig:
    """监听面：`web.host` / `web.port` 必须是 uvicorn 的参数来源"""

    def test_host_and_port_passed_through(self, tmp_path, monkeypatch):
        kwargs, _, _ = _run_main(
            tmp_path, monkeypatch, "web:\n  host: 127.0.0.1\n  port: 8137\n")
        assert kwargs.get("host") == "127.0.0.1"
        assert kwargs.get("port") == 8137

    def test_ipv6_literal_survives_untouched(self, tmp_path, monkeypatch):
        # require_string 不裁剪也不改写，所以 ::1 这种带冒号的地址必须逐字到达
        kwargs, _, _ = _run_main(tmp_path, monkeypatch, "web:\n  host: ::1\n")
        assert kwargs.get("host") == "::1"

    def test_factory_defaults_are_not_replaced(self, tmp_path, monkeypatch):
        """没有配置文件时，绑到出厂默认 —— 接线不许顺手换默认值"""
        kwargs, _, _ = _run_main(tmp_path, monkeypatch, "quality:\n  threshold: 0.5\n")
        defaults = WebConfig()
        assert (kwargs.get("host"), kwargs.get("port")) == (defaults.host, defaults.port)
        assert defaults.host == "0.0.0.0" and defaults.port == 8000

    def test_out_of_range_port_refuses_to_start(self, tmp_path, monkeypatch):
        """越界端口仍然让启动失败，而不是被绕过判据直接绑上"""
        with pytest.raises(DataValidationError, match="web.port"):
            _run_main(tmp_path, monkeypatch, "web:\n  port: 99999\n")


class TestStaticDirComesFromConfig:
    """静态面：`web.static_dir` 决定挂载点，目录不存在时出声"""

    def test_absolute_dir_is_mounted(self, tmp_path, monkeypatch):
        dist = tmp_path / "built" / "dist"
        dist.mkdir(parents=True)
        _, _, mounts = _run_main(
            tmp_path, monkeypatch, f"web:\n  static_dir: {dist.as_posix()}\n")
        assert _static_dir_of(mounts) == str(dist)

    def test_relative_dir_anchors_at_repo_root(self, tmp_path, monkeypatch):
        """相对值锚在仓库根：出厂默认 `web/dist` 逐字指向改动前同一个目录"""
        _, _, mounts = _run_main(tmp_path, monkeypatch, "web:\n  static_dir: web/dist\n")
        if not (AI_ROOT / "web" / "dist").is_dir():
            pytest.skip("本仓未构建前端产物，默认挂载点不存在")
        assert _static_dir_of(mounts) == str(AI_ROOT / "web" / "dist")

    def test_missing_dir_mounts_nothing_and_names_the_path(self, tmp_path, monkeypatch, caplog):
        gone = tmp_path / "never-built"
        with caplog.at_level(logging.WARNING):
            _, _, mounts = _run_main(
                tmp_path, monkeypatch, f"web:\n  static_dir: {gone.as_posix()}\n")
        assert _static_dir_of(mounts) is None
        assert "web.static_dir" in caplog.text
        assert str(gone) in caplog.text
