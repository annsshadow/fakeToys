# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L271：操作系统对「路径名字本身不合法」的判词必须落 400，不得降成 500

**这一格此前的形状**（实测读数，非推断）：同一族「客户端把名字写坏了」的输入在
`POST /api/dataset/convert` 上拿两种码——含 NUL 的名字由操作系统抛 `ValueError`
（`embedded null character`），经 `to_http_error` 第 3 支拿到 400；而超长名与含
控制符的名字由操作系统抛 `OSError(errno=22)`，落到「其它 → 500 固定文案」那一支。
`OSError` 的原文带着部署根绝对路径，所以 500 支不转发原文是对的（A155①），但把
「这个名字在任何账户下都不可能成立」报成**服务端故障**是错的。

**判据**：只认操作系统自己填的 `errno`（`EINVAL` / `ENAMETOOLONG`），不认字符串、
不做平台分支、不猜长度上限。注入型 `OSError("磁盘读失败")` 的 `errno` 是 `None`，
照旧走 500——本轮没有把任何「服务端状态」那一档收松（既有 500 档七支套件 337 支
全绿是这件事的外部证据）。

**跨平台确定性**：Windows 把「名字里有控制符」与「名字 5000 字符」都折成 errno 22，
Linux 的对应物是 22 与 36，两支并收；端到端那一支刻意只用超长名，因为
`"a\\nb.json"` 在 POSIX 上是**合法文件名**，拿它做断言就是在钉一个平台特定的读数。

**留在册的那一半**（A351 的 ②③，本轮不拍）：读侧同样的坏名字拿的是 404「文件不存在」，
而 NUL 那一格的 400 文案是操作系统英文原文、与本轮新增档的中文固定文案不同形。
⇒ 详见循环日志 L271
"""

import errno

import pytest
from fastapi import HTTPException

from api import deps

MARKER = "D:/deploy/root/a\nb.json"
OVERLONG = "a" * 5000 + ".json"


def oserror(code, text="Invalid argument"):
    """造一个操作系统风格的 OSError（带路径原文，用于验「路径不许进响应」）"""
    return OSError(code, text, MARKER)


class TestDispatchTable:
    """`to_http_error` 的分派表：新增档只吃 errno 22 / 36，其余档位一格未动"""

    def test_the_two_rejected_name_errnos_land_in_400(self):
        for code in (errno.EINVAL, errno.ENAMETOOLONG):
            exc = deps.to_http_error(oserror(code))
            assert exc.status_code == 400, code
            assert exc.detail == deps.INVALID_PATH_NAME_DETAIL, code

    def test_the_detail_never_carries_the_deploy_path(self):
        """A155① 的那道墙在新档上同样成立：原文里有绝对路径，一个字符都不许进响应"""
        exc = deps.to_http_error(oserror(errno.EINVAL))
        assert MARKER not in exc.detail and "deploy" not in exc.detail, exc.detail

    def test_server_side_oserrors_stay_500(self):
        for code in (errno.EACCES, errno.ENOSPC, errno.EROFS):
            exc = deps.to_http_error(oserror(code, "server state"))
            assert exc.status_code == 500, code
            assert exc.detail == deps.INTERNAL_ERROR_DETAIL, code

    def test_injected_oserror_without_errno_stays_500(self):
        """既有用例注入的是 `OSError("磁盘读失败")`，`errno` 为 None ⇒ 判据不认字符串"""
        raw = OSError("磁盘读失败")
        assert deps.is_rejected_path_name(raw) is False
        exc = deps.to_http_error(raw)
        assert exc.status_code == 500
        assert exc.detail == deps.INTERNAL_ERROR_DETAIL

    def test_file_not_found_is_still_404_although_it_is_an_oserror(self):
        """`FileNotFoundError` 是 `OSError` 子类，新档排在它之后 ⇒ 404 未被翻掉"""
        assert deps.to_http_error(FileNotFoundError(errno.ENOENT, "no", MARKER)).status_code == 404

    def test_the_existing_branches_are_untouched(self):
        import json

        assert deps.to_http_error(ValueError("参数不合法")).status_code == 400
        assert deps.to_http_error(ValueError("参数不合法")).detail == "参数不合法"
        assert deps.to_http_error(
            json.JSONDecodeError("Expecting value", "{", 1)
        ).status_code == 400
        assert deps.to_http_error(RuntimeError("x")).status_code == 500


class TestPredicate:
    """判据本体的边界：只认 errno，不认类型名、不认文案"""

    def test_only_the_two_errnos_are_rejected_names(self):
        assert deps.is_rejected_path_name(oserror(errno.EINVAL))
        assert deps.is_rejected_path_name(oserror(errno.ENAMETOOLONG))
        assert not deps.is_rejected_path_name(oserror(errno.EACCES))
        assert not deps.is_rejected_path_name(OSError("磁盘读失败"))
        assert not deps.is_rejected_path_name(ValueError("x"))
        assert not deps.is_rejected_path_name(FileNotFoundError(errno.ENOENT, "no"))


class TestRaiseInternalError:
    """34 处收尾分支共用同一判据（A77 一条泄漏判据住一处）"""

    def test_rejected_name_raises_400_not_500(self):
        with pytest.raises(HTTPException) as got:
            deps.raise_internal_error(oserror(errno.EINVAL))
        assert got.value.status_code == 400
        assert got.value.detail == deps.INVALID_PATH_NAME_DETAIL

    def test_server_state_still_raises_500(self):
        with pytest.raises(HTTPException) as got:
            deps.raise_internal_error(OSError(errno.EACCES, "denied", MARKER))
        assert got.value.status_code == 500
        assert MARKER not in got.value.detail


@pytest.fixture
def api_env(tmp_path, monkeypatch):
    """把数据根指到 tmp_path 并撤掉密钥，端点调用不落仓库（残留由 conftest 逐用例盯）"""
    from fastapi.testclient import TestClient

    from api.main import app

    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    (tmp_path / "train_data.json").write_text("[]", encoding="utf-8")
    return TestClient(app)


class TestEndToEnd:
    """写侧端到端：超长名不再是服务端故障，且响应体不含部署根路径"""

    def test_overlong_output_file_is_400(self, api_env, tmp_path):
        r = api_env.post(
            "/api/dataset/convert",
            json={"input_file": str(tmp_path / "train_data.json"),
                  "output_file": str(tmp_path / OVERLONG),
                  "target_format": "json"},
        )
        assert r.status_code == 400, r.status_code
        assert r.json()["detail"] == deps.INVALID_PATH_NAME_DETAIL
        assert str(tmp_path) not in r.text

    def test_plain_missing_input_is_still_404_not_400(self, api_env, tmp_path):
        """新档不许把「文件不存在」那一档一起收进 400"""
        r = api_env.post(
            "/api/dataset/stats",
            json={"input_file": str(tmp_path / "nope.json")},
        )
        assert r.status_code == 404, r.status_code


class TestRulerBites:
    """三臂自证：全部在内存里改判据，仓库文件与磁盘一格未动"""

    def test_arm_empty_set_sends_the_name_back_to_500(self, monkeypatch):
        monkeypatch.setattr(deps, "_OS_REJECTED_NAME_ERRNOS", frozenset())
        assert deps.to_http_error(oserror(errno.EINVAL)).status_code == 500

    def test_arm_widening_the_set_swallows_server_state(self, monkeypatch):
        monkeypatch.setattr(
            deps, "_OS_REJECTED_NAME_ERRNOS", frozenset({errno.EINVAL, errno.EACCES})
        )
        assert deps.to_http_error(oserror(errno.EACCES)).status_code == 400

    def test_arm_echoing_the_original_text_leaks_the_deploy_root(self, monkeypatch):
        """把固定文案换成 `str(exc)` 那一形（ValueError 支的写法）⇒ 路径必须出现在断言里

        这一臂证明 `test_the_detail_never_carries_the_deploy_path` 不是恒真：
        新档的文案换成原文照回后，尺子当场看到部署根。
        """
        monkeypatch.setattr(deps, "INVALID_PATH_NAME_DETAIL", MARKER)
        exc = deps.to_http_error(oserror(errno.EINVAL))
        assert MARKER in exc.detail
