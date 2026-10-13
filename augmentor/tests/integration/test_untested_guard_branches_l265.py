# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L265：五处「护栏写了、但从未被执行」的覆盖率残差接线

**这一档证据从哪来**：定标门禁产出的 ``coverage.xml`` 逐格现量（不是记忆，也不是
``# pragma`` 抹平）。全仓只剩 5 处、8 条语句从未被执行（逐格点数与门禁里「缺数 8」那一格
闭合：1 + 2 + 1 + 2 + 2）：

- ``api/routes/vector.py:150`` —— ``except HTTPException: raise`` 那一支的 ``raise``
- ``api/routes/multimodal.py:102`` 与 ``:113`` —— 「两级都空 ⇒ 交给处理器内置默认」
- ``augmentor/semantic.py:94`` —— 空文本清单的早退
- ``augmentor/versioning.py:275-276`` —— 当前版本指针**是软链接**那一形
- ``augmentor/data/image.py:135-136`` —— Pillow 在场时的解析路径

**为什么这不是凑覆盖率**：``except HTTPException: raise`` 是本轮守卫
``test_http_exception_passthrough_l265.py`` 的承重面 —— 一条从未被执行过的保码支，
和一条写错的保码支，在「客户端拿到什么状态码」这一层可能是同一回事，也可能不是：
它今天恰好通，是因为 try 体里没人抛 4xx，而不是因为有人验过它通。残差接线就是把
「恰好」换成「量过」。每一格都配一个**反向臂**（同一处没有支/走了另一档时会怎样），
否则断言只是在复述代码。

**诚实记账**：
- ``semantic.py:93-94`` 不是防崩护栏。实测 ``normalize_vectors(np.zeros((0, d)))``
  不抛、``list(...)`` 也得到 ``[]`` ⇒ 早退与不早退**同值**，它是等价类快路径。
  本文件按快路径接线，并把它唯一有牙齿的那一格（早退不越过维度闸）钉住。
- ``versioning.py:275-276`` 的真软链接形状本机不可测：无特权创建符号链接会抛
  ``WinError 1314``。这里替的是**指针对象**本身（只替 ``is_symlink()`` 与 ``resolve()``），
  ``with_suffix('.txt')`` 仍走真实路径 ⇒ ``.txt`` 那一档的读法没有被替身掩盖，
  配套的反向臂证明第一臂的返回值确实来自软链接分支。
- ``image.py:135-136`` 的真 Pillow 形状本机也不可测（未装 Pillow）。这里用
  ``sys.modules`` 桩，且桩报的尺寸与标准库文件头解析器报的尺寸**故意不同**，
  于是断言量到的是「Pillow 优先」这条声明，不是「能跑通」。
"""

import struct
import sys
import types
from pathlib import Path
from types import SimpleNamespace

import numpy as np
import pytest

from api.deps import INTERNAL_ERROR_DETAIL

# ============ 一、vector.py:150 —— 真端点上的保码支 ============


@pytest.fixture
def client(tmp_path, monkeypatch):
    """把 app 打到临时目录：本文件不写配置、不写数据根，只借 TestClient 走一次 GET"""
    from fastapi.testclient import TestClient

    from api.main import app

    monkeypatch.chdir(tmp_path)
    return TestClient(app)


class TestVectorStatusKeepsTheGateAnswer:
    """``GET /api/dataset/vector-status`` 的 ``except HTTPException: raise``（vector.py:150）

    该端点的支注释写着「白名单越界（403）等已由 deps 出声，不能被 to_http_error 降级成 500」，
    但在接线之前，这条支的 ``raise`` 一次都没被执行过 —— 注释里的承诺没有任何读数。
    """

    @staticmethod
    def _gate_refuses():
        from fastapi import HTTPException

        raise HTTPException(status_code=403, detail="数据目录不在白名单内（注入）")

    def test_a_4xx_from_the_body_reaches_the_client_untouched(self, client, monkeypatch):
        import api.routes.vector as vector_module

        monkeypatch.setattr(vector_module, "get_pipeline", self._gate_refuses)
        response = client.get("/api/dataset/vector-status")
        assert response.status_code == 403, response.text
        assert response.json()["detail"] == "数据目录不在白名单内（注入）"
        # 反向臂的对照：这一格若被降级，看到的会是固定文案而不是原话
        assert response.json()["detail"] != INTERNAL_ERROR_DETAIL

    def test_the_same_body_without_a_4xx_still_collapses_to_a_fixed_500(self, client, monkeypatch):
        """对照臂：同一只 try 抛非 HTTP 异常 ⇒ 500 + 固定文案

        两臂合起来才叫「那条支有牙齿」：只有第一臂时，无法区分「支在保码」与
        「整条路径本来就回 403」。
        """
        import api.routes.vector as vector_module

        def _boom():
            raise RuntimeError("pipeline down")

        monkeypatch.setattr(vector_module, "get_pipeline", _boom)
        response = client.get("/api/dataset/vector-status")
        assert response.status_code == 500, response.text
        assert response.json()["detail"] == INTERNAL_ERROR_DETAIL


# ============ 二、multimodal.py:102 与 :113 —— 两级都空 = 让处理器自己定 ============


class TestBothLevelsEmptyHandTheDecisionBack:
    """``_resolve_extensions`` 的第三档（102）与 ``_normalize_extensions`` 的 None 透传（113）

    权威序是「请求参数 > ``config.multimodal.*`` > 处理器内置默认」。前两档各有测试，
    第三档 —— 两级**都**空 ⇒ 交回 ``None`` —— 一次没走过。``None`` 在这里不是「没有扩展名」，
    而是「本层不做决定」，所以它必须**原样**穿过归一函数；把 ``None`` 归一成 ``[]``
    就等于替处理器做了决定（内置默认被清空）。
    """

    @staticmethod
    def _cfg(image=(), audio=()):
        mod = SimpleNamespace(image_extensions=list(image), audio_extensions=list(audio))
        return SimpleNamespace(config=SimpleNamespace(multimodal=mod))

    def _resolve(self, monkeypatch, image_req, audio_req, cfg_image, cfg_audio):
        import api.routes.multimodal as mm

        monkeypatch.setattr(mm, "get_pipeline", lambda: self._cfg(cfg_image, cfg_audio))
        return mm._resolve_extensions(image_req, audio_req)

    def test_both_levels_empty_yields_two_nones(self, monkeypatch):
        assert self._resolve(monkeypatch, [], [], [], []) == (None, None)

    def test_request_level_still_wins_over_empty_config(self, monkeypatch):
        assert self._resolve(monkeypatch, ["jpg"], [], [], []) == ([".jpg"], None)

    def test_config_level_fills_a_empty_request(self, monkeypatch):
        got = self._resolve(monkeypatch, [], None, [".png"], [".wav"])
        assert got == ([".png"], [".wav"])

    def test_none_is_not_the_same_as_an_empty_list(self):
        """``None`` 透传 vs 空清单归一：两档必须可区分，否则第三档就塌成了「什么都别匹配」"""
        import api.routes.multimodal as mm

        assert mm._normalize_extensions(None) is None
        assert mm._normalize_extensions([]) == []


# ============ 三、semantic.py:94 —— 空文本清单（等价类快路径，不是护栏） ============


class TestEmptyTextInputShortCircuits:
    """``embed_texts([], d)`` 的早退（semantic.py:94）

    实测记在案：``list(normalize_vectors(np.zeros((0, d))))`` 也返回 ``[]`` ——
    所以这一格**不是**防 500 的护栏，而是省一次归一化的快路径。本类的价值在两处：
    把「空输入 ⇒ 空清单」这条声明从「碰巧成立」变成「有断言」，并钉住早退的**位置**
    （维度闸在前 ⇒ 空清单配坏维度照样出声）。
    """

    def test_empty_input_returns_an_empty_plain_list(self):
        from augmentor.semantic import embed_texts

        got = embed_texts([], 8)
        assert isinstance(got, list) and got == []

    def test_the_short_circuit_is_after_the_dimension_gate(self):
        """顺序承重：把早退挪到维度判之前，这里就红 —— 空输入不该豁免坏配置"""
        from augmentor.exceptions import VectorError
        from augmentor.semantic import embed_texts

        with pytest.raises(VectorError):
            embed_texts([], 0)
        with pytest.raises(VectorError):
            embed_texts([], True)

    def test_the_fallthrough_would_have_given_the_same_answer(self):
        """自证「快路径而非护栏」：不走早退的那条表达式，逐值等于早退的返回值"""
        from augmentor.semantic import normalize_vectors

        rows = np.zeros((0, 8), dtype=np.float32)
        assert list(normalize_vectors(rows)) == []


# ============ 四、versioning.py:275-276 —— 当前版本指针是软链接那一形 ============


class _PointerLike:
    """「指针是软链接」的替身：只替 ``is_symlink()`` 与 ``resolve()``

    ``with_suffix`` 透传给真实 Path ⇒ ``.txt`` 那一档仍按磁盘上的真文件读，
    替身没有把另一条分支一起遮掉。真软链接本机不可测（无特权 ⇒ ``WinError 1314``）。
    """

    def __init__(self, real: Path, link_target: Path, *, as_symlink: bool):
        self._real = real
        self._target = link_target
        self._as_symlink = as_symlink

    def is_symlink(self) -> bool:
        return self._as_symlink

    def resolve(self) -> Path:
        return self._target

    def __getattr__(self, name):
        return getattr(self._real, name)


class TestCurrentVersionPointerBranches:
    def _manager(self, tmp_path):
        from augmentor.versioning import VersionManager

        return VersionManager(storage_dir=str(tmp_path / "versions"))

    def test_symlink_pointer_reports_its_target_dir_name(self, tmp_path):
        vm = self._manager(tmp_path)
        (tmp_path / "versions" / "v3").mkdir(parents=True)
        pointer = vm._current_symlink
        (tmp_path / "versions").mkdir(parents=True, exist_ok=True)
        pointer.with_suffix(".txt").write_text("v9", encoding="utf-8")
        vm._current_symlink = _PointerLike(pointer, tmp_path / "versions" / "v3", as_symlink=True)
        assert vm.get_current_version() == "v3"

    def test_without_a_symlink_the_txt_pointer_wins(self, tmp_path):
        """反向臂：``is_symlink()`` 为假时同一份磁盘状态读出的是 ``v9``

        没有这一臂，前一臂的 "v3" 就可能是从 ``.txt`` 读来的 —— 两档合起来才证明
        275-276 那两行确实参与了决定。
        """
        vm = self._manager(tmp_path)
        (tmp_path / "versions" / "v3").mkdir(parents=True)
        pointer = vm._current_symlink
        pointer.with_suffix(".txt").write_text("v9", encoding="utf-8")
        vm._current_symlink = _PointerLike(pointer, tmp_path / "versions" / "v3", as_symlink=False)
        assert vm.get_current_version() == "v9"

    def test_neither_pointer_present_reads_as_none(self, tmp_path):
        vm = self._manager(tmp_path)
        (tmp_path / "versions").mkdir(parents=True, exist_ok=True)
        assert vm.get_current_version() is None


# ============ 五、image.py:135-136 —— Pillow 在场时的那条路 ============


def _png_like(tmp_path, name, width, height):
    """写一只文件头形状合法的 PNG：标准库解析器会读出 (width, height)"""
    path = tmp_path / name
    body = (b"\x89PNG\r\n\x1a\n" + struct.pack(">I", 13) + b"IHDR"
            + struct.pack(">II", width, height) + b"\x08\x06\x00\x00\x00" + b"\x00" * 8)
    path.write_bytes(body)
    return path


class _FakeImageFile:
    def __init__(self, payload, log):
        self._p = payload
        self._log = log

    def __enter__(self):
        self._log.append("enter")
        return self

    def __exit__(self, *exc):
        self._log.append("exit")
        return False

    def __getattr__(self, name):
        return self._p[name]


@pytest.fixture
def pillow_stub(monkeypatch):
    """装上假 Pillow，返回可填的答复表与句柄动作日志

    本机未装 Pillow，``_parse_with_pillow`` 的真路径在这里不可测；桩报的尺寸与文件头
    解析器**故意不同**，于是断言量到的是「Pillow 优先」这条声明。
    """
    log = []
    answers = {"format": "WEBP", "width": 9, "height": 8, "mode": "RGB", "info": {"dpi": 1}}
    image_mod = types.ModuleType("PIL.Image")
    image_mod.open = lambda path: _FakeImageFile(answers, log)
    pil_mod = types.ModuleType("PIL")
    pil_mod.Image = image_mod
    monkeypatch.setitem(sys.modules, "PIL", pil_mod)
    monkeypatch.setitem(sys.modules, "PIL.Image", image_mod)
    return SimpleNamespace(answers=answers, log=log)


class TestPillowPathWhenPillowExists:
    def test_pillow_answers_beat_the_header_parser(self, tmp_path, pillow_stub):
        from augmentor.data.image import ImageProcessor

        path = _png_like(tmp_path, "l265.png", 111, 222)
        info = ImageProcessor().process_image(str(path))
        assert (info.width, info.height) == (9, 8), "Pillow 在场时不该用文件头的 111x222"
        assert info.valid is True and info.error == ""

    def test_format_falls_back_to_the_suffix_only_when_pillow_says_nothing(self, tmp_path, pillow_stub):
        """``img.format or path.suffix...``：桩报 None ⇒ 才走后缀兜底那一侧"""
        from augmentor.data.image import ImageProcessor

        pillow_stub.answers["format"] = None
        path = _png_like(tmp_path, "l265.png", 111, 222)
        assert ImageProcessor().process_image(str(path)).format == "PNG"
        assert pillow_stub.answers["format"] is None

    def test_pillow_reported_format_wins_over_the_suffix(self, tmp_path, pillow_stub):
        from augmentor.data.image import ImageProcessor

        path = _png_like(tmp_path, "l265.png", 111, 222)
        assert ImageProcessor().process_image(str(path)).format == "WEBP"

    def test_the_handle_is_closed_via_the_context_manager(self, tmp_path, pillow_stub):
        """135 那一行的内容就是 ``with``：句柄必须成对进出，否则解析过的文件一直开着"""
        from augmentor.data.image import ImageProcessor

        path = _png_like(tmp_path, "l265.png", 111, 222)
        ImageProcessor().process_image(str(path))
        assert pillow_stub.log == ["enter", "exit"]

    def test_metadata_carries_pillow_info_keys_not_the_stdlib_tag(self, tmp_path, pillow_stub):
        """``metadata.parser == "stdlib"`` 是兜底那档的签名，Pillow 档不该带它"""
        from augmentor.data.image import ImageProcessor

        path = _png_like(tmp_path, "l265.png", 111, 222)
        info = ImageProcessor().process_image(str(path))
        assert info.metadata.get("parser") != "stdlib"
        assert info.mode == "RGB"

    def test_without_the_stub_the_same_file_goes_through_the_header_parser(self, tmp_path):
        """反向臂：不装桩 ⇒ 本机没 Pillow ⇒ 走文件头，读出 111x222、``parser == "stdlib"``

        与前一档对照才说明桩改变的是「谁来答」，而不是把断言写成了桩自己的回声。
        """
        from augmentor.data.image import ImageProcessor

        path = _png_like(tmp_path, "l265.png", 111, 222)
        info = ImageProcessor().process_image(str(path))
        assert (info.width, info.height) == (111, 222)
        assert info.metadata.get("parser") == "stdlib"


# ============ 六、桩不得漏进别处：本文件的替身只在测试内活着 ============


class TestTheStubDoesNotLeak:
    def test_pillow_is_still_absent_after_the_stub_tests(self, tmp_path):
        """``sys.modules`` 桩由 monkeypatch 复原：跑完本文件之后 PIL 依旧不可导入

        这一支是给「接线本身可能污染后续测试」准备的：桩漏出去会让后续 Pillow 相关
        断言从「走文件头」变成「走桩」，而那种红通常落在很远的一支上。
        """
        with pytest.raises(ImportError):
            from PIL import Image  # noqa: F401
