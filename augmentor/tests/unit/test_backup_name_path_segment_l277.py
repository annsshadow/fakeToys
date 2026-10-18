# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""A366 / L277 守卫：备份名**只能占一个路径段**——「建得出来却寻不了址」的失踪态被关掉

**缺陷形状（`TestThePhantomState` 现场复现，不是叙述）**：`augmentor/backup.py` 里
`backup_id = name or 时间戳` 之后，这个名字同时是**落盘文件名**和 HTTP 路径段。L231 补的那道闸
只问「会不会写出备份目录之外」，于是 `sub/deep` 完全放行。摘掉本轮新闸重跑同一请求，实测：

- `POST /api/system/backups` (`name=sub/deep`) ⇒ **200**，回执 `backup_id="sub/deep"`；
- `GET /api/system/backups` ⇒ 清单里**有这一行**，磁盘上 `sub\deep.json` 也在；
- `DELETE /api/system/backups/sub/deep` ⇒ **404**，`POST …/sub/deep/restore` ⇒ **404**（这一档
  回执码随部署形状分叉：`/` 上挂着静态兜底时两形都是 **405**，判据按 `app.routes` 现算，见 A361 与 `_phantom_code`）；
- 换成 percent 编码 `sub%2Fdeep` 两只入口**照样 404**（路由按原始路径切段，`{backup_id}` 只吃
  `[^/]+`，编码救不回来）。

⇒ 这份备份对**任何客户端**都不可操作：数据不是被拒，是失踪，而回执与清单都报告它存在。
所以判据设在名字的出生点（创建那一刻），拒在磁盘动作之前——实测拒后连备份目录都不建。

**八档拒绝**（`REJECTED` 逐档点名，各自文案不同 ⇒ 调用方知道是哪一形不成立）：
含 `/` 或 `\`（两种都拒、不分平台：`\` 是 Windows 的 `os.sep`，而同一份 `index.json` 两边都要读，
跨平台的 id 面按严的一边收）、含 `:`（L279 续格：`pathlib` 把 `C:foo` 当盘符前缀吞掉 ⇒ 落盘叫
`foo`、回执叫 `C:foo`，两份名字还能落到同一只文件上，见 `TestTheColonIsSwallowedByPathlib`）、
`.` 与 `..`、`#` 与 `?`（URL 的 fragment/query 分隔符，最朴素的
客户端会把 id 截成前半截 ⇒ 同一份失踪态的另一条来路，见 `TestTheHashIsStrandedOnTheWire`）、
控制字符（NUL 让 `open()` 抛 `ValueError` ⇒ 旧归宿 500）、
尾随点或空白（Windows 静默裁剪 ⇒ 落点名 ≠ 回执名）、超过 255（单段长度上限，越界 `ENAMETOOLONG`）、
非字符串与纯空白。

**没有跟着改的两格（如实点名）**：① `name=""` 仍走 `name or 时间戳` 生成 id（L231 契约第 32 行，
`TestEmptyStringStillMeansUnset` 钉住）——那是 L262「空容器吞并」族里**字符串档**的一格，
本轮不顺手改，免得把「拒空白名」与「空串按没传」两种语义混在一次改动里；② `path_lands_within`
那道旧闸**保留**。L277 当轮写的「新闸只认单段，形状上已无名字能走到它」本轮被探针**证伪**：
把新闸在内存里摘成恒等，`\\?\C:\evil`、`D:evil`、`\absolute`、`/posix/abs` 四形照样落到它手上
并被拒、且零落盘（`TestTheLandingGateIsStillLive` 现场跑）⇒ 它守的是**落点**、新闸守的是**名字**，
口径不同，两道都不许摘。

跑法：`python -m pytest tests/unit/test_backup_name_path_segment_l277.py -q --no-cov`
"""
import json
import os
import pathlib
import re
import urllib.parse

import pytest

ROOT = pathlib.Path(__file__).resolve().parents[2]
BACKUP_PY = ROOT / "augmentor" / "backup.py"
VALIDATION_PY = ROOT / "augmentor" / "validation.py"
CREATED = "/api/system/backups"

ITEMS = [{"instruction": "q", "input": "", "output": "a"}]

#: 每档一个代表名，值位写内联字面量（`l97` / `l98` 的口径），第三格是判决文案里的**独有子串**
REJECTED = [
    ("sub/deep", "路径分隔符"),
    ("back\\slash", "路径分隔符"),
    ("C:foo", "冒号"),
    ("C:", "冒号"),
    ("a:b", "冒号"),
    ("D:evil", "冒号"),
    ("..", "相对父目录"),
    (".", "相对父目录"),
    ("frag#ment", "URL 结构字符"),
    ("que?ry", "URL 结构字符"),
    ("nul\x00name", "控制字符"),
    ("tab\there", "控制字符"),
    ("trail.", "以点或空白结尾"),
    ("  padded  ", "以点或空白结尾"),
    ("x" * 256, "超过单个路径段上限"),
    ("   ", "非空字符串"),
]


@pytest.fixture
def http(tmp_path, monkeypatch):
    """白名单根 `ds/p.json` + 一只空备份目录路径（**不预先建**，好让副作用可见）"""
    root = tmp_path / "ds"
    root.mkdir()
    (root / "p.json").write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root


def _phantom_code(client) -> int:
    """「按回执名寻不到址」那一档的回执码随部署形状分叉 ⇒ 按 `app.routes` 现算（A361 口径）

    `/` 上挂着静态兜底（这一树构建过前端、`web/dist` 在场）时，未匹配的 API 路径会被那只
    Mount 吃掉、非 GET 给出 **405**；没挂时才是路由表自己的 **404**。把任何一侧抄成常数，
    等于把「作者机构建过 UI」写进判据——干净克隆与构建过的机器各红一边（L293 撞到的就是
    这一形：合并进主工作树后同一份代码、同一个用例，只因那边有 `web/dist` 就从 404 变 405）。
    现算不等于放宽：两档都只认一个码，判决由装配事实决定。
    """
    mounted = any(type(route).__name__ == "Mount" for route in client.app.routes)
    return 405 if mounted else 404


class TestThePhantomState:
    """缺陷本体钉在这里：把新闸在**内存里**摘成恒等 ⇒ 失踪态当场重现，装回去 ⇒ 消失"""

    def test_without_the_gate_the_name_is_created_but_unaddressable(
        self, http, tmp_path, monkeypatch
    ):
        client, root = http
        monkeypatch.setattr("augmentor.backup.require_path_segment",
                            lambda label, value: value)
        bdir = root / ".ghost"
        r = client.post(CREATED, json={"input_file": "p.json", "name": "sub/deep"},
                        params={"backup_dir": str(bdir)})
        assert r.status_code == 200, r.text
        assert r.json()["backup_id"] == "sub/deep"
        assert (bdir / "sub" / "deep.json").is_file(), "摘了闸还不落盘 ⇒ 这一臂没在测落点"
        listed = client.get(CREATED, params={"backup_dir": str(bdir)})
        assert [b["backup_id"] for b in listed.json()["backups"]] == ["sub/deep"]
        expected = _phantom_code(client)
        for wire in ("sub/deep", urllib.parse.quote("sub/deep", safe="")):
            assert client.delete(f"{CREATED}/{wire}",
                                 params={"backup_dir": str(bdir)}).status_code == expected
            assert client.post(f"{CREATED}/{wire}/restore",
                               json={"output_file": "out.json"},
                               params={"backup_dir": str(bdir)}).status_code == expected
        assert (bdir / "sub" / "deep.json").is_file(), \
            f"回执 {expected} 说寻不到址，那一份就必须还在盘上 ⇒ 少了这条，「码对而数据已毁」也会放行"
        assert not (root / "out.json").exists(), "寻不到址却把内容恢复了 ⇒ 上面那几读数不可信"

    def test_with_the_gate_the_same_name_is_400_and_writes_nothing(self, http, tmp_path):
        client, root = http
        bdir = root / ".guarded"
        r = client.post(CREATED, json={"input_file": "p.json", "name": "sub/deep"},
                        params={"backup_dir": str(bdir)})
        assert r.status_code == 400, r.text
        assert "路径分隔符" in r.json()["detail"]
        assert not bdir.exists(), "判在磁盘动作之前才叫拒生；建了目录再退货是假判决"


class TestTheHashIsStrandedOnTheWire:
    """第二维失踪态（`#` / `?`）：段形状合法，坏在**上线之后**——最朴素的客户端够不着

    实测（Temp `probe_addressing.py`，raw 臂 / quoted 臂各用全新目录、互不遮蔽）：含 `#` 的名字
    创建 **200**、清单里躺着它，照回执原样拼 URL ⇒ restore **405**、delete **404**
    （`#` 之后整段被客户端当 fragment 丢掉，服务端只收到前半截），而 `frag%23ment` 两只入口都
    **200**。⇒ 这份备份只对「记得编码」的客户端存在，对 `web/src` 这种直接插值的客户端是失踪的。
    前端那一半由 `encodeURIComponent` 补（存量 id 仍要能寻址），出生这一半由本闸拒。
    """

    def test_without_the_gate_a_hash_name_is_only_half_addressable(self, http, monkeypatch):
        client, root = http
        monkeypatch.setattr("augmentor.backup.require_path_segment",
                            lambda label, value: value)
        bdir = root / ".hashy"
        r = client.post(CREATED, json={"input_file": "p.json", "name": "frag#ment"},
                        params={"backup_dir": str(bdir)})
        assert r.status_code == 200, r.text
        assert client.delete(f"{CREATED}/frag#ment",
                             params={"backup_dir": str(bdir)}).status_code == 404
        listed = client.get(CREATED, params={"backup_dir": str(bdir)})
        assert [b["backup_id"] for b in listed.json()["backups"]] == ["frag#ment"], "裸路径删不掉却也不在清单里 ⇒ 测的不是失踪"
        assert client.delete(f"{CREATED}/frag%23ment",
                             params={"backup_dir": str(bdir)}).status_code == 200

    def test_with_the_gate_it_is_400_at_birth(self, http):
        client, root = http
        bdir = root / ".hashy2"
        for name in ("frag#ment", "que?ry"):
            r = client.post(CREATED, json={"input_file": "p.json", "name": name},
                            params={"backup_dir": str(bdir)})
            assert r.status_code == 400, f"{name!r} 实得 {r.status_code}: {r.text[:160]}"
            assert "URL 结构字符" in r.json()["detail"]
        assert not bdir.exists(), "拒生不许留下目录"


class TestTheColonIsSwallowedByPathlib:
    """第三维失踪态（L279 续格）：冒号让 **落盘名** 与回执 id 分叉，两个名字还能共用一只文件

    实测（Temp `_l279_alias_probe.py`，把出生闸在内存里摘成恒等）：`name="C:foo"` 创建 **200**、
    回执 `backup_id="C:foo"`，而盘上那份叫 `foo.json`；再创建 `name="foo"` 同样 **200**，两份名字
    落到 **同一只文件**，清单里两行并存、都自称一条备份。此时 `DELETE /C:foo` **200** ⇒ 那只共用
    文件被 unlink，清单里剩下的 `foo` 一行指向一只已经不存在的文件（`GET /verify` 如实报
    `exists:false`，`POST /foo/restore` 回 **404**「备份不存在」、零落盘）。⇒ 一次按 `C:foo` 的删除
    毁掉了另一条记录的全部载荷，这是跨 id 的数据损毁，而不是「只是名字难看」。
    `name="C:"` 更落到 `.json`，正是档 1 要避免的那个形状。POSIX 上 `C:foo.json` 是合法文件名
    ⇒ 同一个 id 在两个平台上是两个落点，所以按「更严的一边」出生就拒。
    """

    def test_the_pathlib_shapes_disagree_between_platforms(self):
        """纯形状断言（不碰磁盘、跨平台恒绿）：同一份请求名在两边拼出不同的落点"""
        win = pathlib.PureWindowsPath(r"C:\ds\backups")
        posix = pathlib.PurePosixPath("/ds/backups")
        assert str(win / "C:foo.json") == r"C:\ds\backups\foo.json", "盘符前缀被吞掉、名字被改写"
        assert str(posix / "C:foo.json") == "/ds/backups/C:foo.json"
        assert str(win / "a:b.json") == "a:b.json", "基座整个被丢掉 ⇒ 落点不在备份目录里"
        assert str(posix / "a:b.json") == "/ds/backups/a:b.json"

    def test_without_the_gate_two_names_share_one_file_and_one_delete_kills_both(
        self, http, monkeypatch
    ):
        if os.name != "nt":
            pytest.skip("盘符吞并是 Windows 的 pathlib 形状，POSIX 上两平台分叉由纯形状臂负责")
        client, root = http
        monkeypatch.setattr("augmentor.backup.require_path_segment", lambda label, value: value)
        bdir = root / ".alias"

        def rows():
            return [(b["backup_id"], pathlib.Path(b["backup_path"]).name)
                    for b in client.get(CREATED, params={"backup_dir": str(bdir)}).json()["backups"]]

        first = client.post(CREATED, json={"input_file": "p.json", "name": "C:foo"},
                            params={"backup_dir": str(bdir)})
        assert first.status_code == 200, first.text
        assert first.json()["backup_id"] == "C:foo", "回执必须原样报告请求里那个名字"
        assert pathlib.Path(first.json()["backup_path"]).name == "foo.json", \
            "摘了闸却没改名 ⇒ 这一臂没在测盘符吞并"
        assert rows() == [("C:foo", "foo.json")]

        second = client.post(CREATED, json={"input_file": "p.json", "name": "foo"},
                              params={"backup_dir": str(bdir)})
        assert second.status_code == 200, second.text
        assert rows() == [("C:foo", "foo.json"), ("foo", "foo.json")], \
            "两条记录自称两个备份，却共用一只文件"
        assert sorted(p.name for p in bdir.glob("*.json")) == ["foo.json", "index.json"]

        dropped = client.delete(f"{CREATED}/C%3Afoo", params={"backup_dir": str(bdir)})
        assert dropped.status_code == 200, dropped.text
        assert rows() == [("foo", "foo.json")], "清单里仍写着 foo 指向那只文件"
        assert not (bdir / "foo.json").exists(), \
            "按另一个 id 删除却把这只文件也毁了 ⇒ 上面那读数不是在测跨 id 损毁"
        ghost = client.post(f"{CREATED}/foo/restore", json={"output_file": "out.json"},
                            params={"backup_dir": str(bdir)})
        assert ghost.status_code == 404, f"条目还在而文件已丢，恢复必须响亮失败: {ghost.text[:120]}"
        assert not (root / "out.json").exists()

    def test_bare_colon_name_lands_as_dot_json_on_windows(self, http, monkeypatch):
        if os.name != "nt":
            pytest.skip("同上一臂：POSIX 上 `C:` 是合法文件名字符，不会落到 `.json`")
        client, root = http
        monkeypatch.setattr("augmentor.backup.require_path_segment", lambda label, value: value)
        bdir = root / ".bare"
        r = client.post(CREATED, json={"input_file": "p.json", "name": "C:"},
                        params={"backup_dir": str(bdir)})
        assert r.status_code == 200, r.text
        assert sorted(p.name for p in bdir.iterdir()) == [".json", "index.json"], \
            "空名落盘成 `.json` 那一档由冒号绕进来了"

    def test_with_the_gate_the_landing_name_still_equals_the_receipt(self, http):
        """放行档不许留下「回执与盘名不同形」的余地：收下什么名字，那份文件就叫什么"""
        client, root = http
        bdir = root / ".legit"
        for name in ("nightly-01", "a.b.c", "备份-周三"):
            r = client.post(CREATED, json={"input_file": "p.json", "name": name},
                            params={"backup_dir": str(bdir)})
            assert r.status_code == 200, r.text
            rows = client.get(CREATED, params={"backup_dir": str(bdir)}).json()["backups"]
            assert [pathlib.Path(x["backup_path"]).name for x in rows] == [f"{name}.json"]
            assert client.delete(f"{CREATED}/{urllib.parse.quote(name, safe='')}",
                                 params={"backup_dir": str(bdir)}).status_code == 200


class TestTheLandingGateIsStillLive:
    """L277 当轮那句「形状上已无名字能走到它」本轮被证伪 ⇒ 旧闸的可达性得由探针钉，不靠叙述

    摘掉新闸后实测（Temp `_l279_landing_probe.py`）：`\\\\?\\C:\\evil`、`D:evil`、
    `\\\\nas\\share\\x`、`\\absolute` 四形全部由 `backup.py` 的落点判据拒掉（同一句
    「不能指向备份目录之外」），且备份目录之外零落盘；`../victim` 那一形两个平台都走到这里。
    两闸口径不同（名字 vs 落点）⇒ 都不许摘。
    """

    def test_shapes_that_reach_the_old_gate_are_rejected_and_leave_nothing(self, tmp_path,
                                                                            monkeypatch):
        from augmentor.backup import DatasetBackup
        from augmentor.exceptions import DataValidationError

        monkeypatch.setattr("augmentor.backup.require_path_segment", lambda label, value: value)
        source = tmp_path / "p.json"
        source.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
        bdir = tmp_path / "backups"
        manager = DatasetBackup(str(bdir))
        # `../victim` 两平台都能走到落点闸；其余四形只有 Windows 的形状才逃得出基座
        names = ["../victim"]
        if os.name == "nt":
            names += ["\\\\?\\C:\\evil", "D:evil", "\\\\nas\\share\\x", "\\absolute"]
        for name in names:
            with pytest.raises(DataValidationError) as exc:
                manager.backup(str(source), name=name)
            assert "指向备份目录之外" in str(exc.value), f"{name!r}: {exc.value}"
        assert not bdir.exists() or list(bdir.iterdir()) == [], "拒生却不许建目录"
        assert sorted(str(p.relative_to(tmp_path)) for p in tmp_path.rglob("*.json")) == ["p.json"], \
            "备份目录之外落了盘 ⇒ 上面那几读数不可信"


class TestEachRefusalIsNamed:
    """十六个代表名逐个打 HTTP（覆盖八档判决），逐档点自己的文案；顺带证明整批拒完磁盘上什么都没留下"""

    @pytest.mark.parametrize(
        "name,reason",
        [
            ("sub/deep", "路径分隔符"),
            ("back\\slash", "路径分隔符"),
            ("C:foo", "冒号"),
            ("C:", "冒号"),
            ("a:b", "冒号"),
            ("D:evil", "冒号"),
            ("..", "相对父目录"),
            (".", "相对父目录"),
            ("frag#ment", "URL 结构字符"),
            ("que?ry", "URL 结构字符"),
            ("nul\x00name", "控制字符"),
            ("tab\there", "控制字符"),
            ("trail.", "以点或空白结尾"),
            ("  padded  ", "以点或空白结尾"),
            ("x" * 256, "超过单个路径段上限"),
            ("   ", "非空字符串"),
        ],
        ids=[
            "slash_forward", "slash_back", "colon_drive_relative", "colon_bare",
            "colon_mid", "colon_other_drive", "dotdot", "dot", "hash", "question",
            "nul", "tab", "trailing_dot", "padded", "over_255", "whitespace_only",
        ],
    )
    def test_http_create_refuses_it_with_its_own_verdict(self, http, name, reason):
        client, root = http
        bdir = root / f".b{abs(hash(name))}"
        r = client.post(CREATED, json={"input_file": "p.json", "name": name},
                        params={"backup_dir": str(bdir)})
        assert r.status_code == 400, f"{name!r} 实得 {r.status_code}: {r.text[:200]}"
        assert reason in r.json()["detail"], f"{name!r} 的判决没点名这一档: {r.json()['detail']}"

    def test_the_sdk_gate_is_the_same_one(self, tmp_path):
        """不只 HTTP：SDK 直接打同一档 ⇒ 同一句判决（闸住在库里，路由只负责落 400）"""
        from augmentor.backup import DatasetBackup
        from augmentor.exceptions import DataValidationError

        root = tmp_path / "ds"
        root.mkdir()
        source = root / "p.json"
        source.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
        manager = DatasetBackup(str(root / ".backups"))
        for name, reason in REJECTED:
            with pytest.raises(DataValidationError) as exc:
                manager.backup(str(source), name=name)
            assert reason in str(exc.value), f"{name!r}: {exc.value}"
        assert sorted(p.name for p in (root / ".backups").glob("*")) == [], "SDK 面拒了却建了目录"

    def test_every_rejected_shape_says_which_kind_of_addressability_it_breaks(self):
        """尺子的口径不是「十二个例子恰好红」：每一档都要能说出它坏在哪一维"""
        seps = [n for n, r in REJECTED if r == "路径分隔符"]
        urlish = [n for n, r in REJECTED if r == "URL 结构字符"]
        colonish = [n for n, r in REJECTED if r == "冒号"]
        assert seps == ["sub/deep", "back\\slash"]
        assert urlish == ["frag#ment", "que?ry"]
        assert colonish == ["C:foo", "C:", "a:b", "D:evil"]
        # 分隔符族连「一个路径段」这个形状都不成立；URL 与冒号两族**段形状合法**，坏在上线之后
        # （`TestTheHashIsStrandedOnTheWire` 现场证）或坏在落盘改名（`TestTheColonIsSwallowedByPathlib`）
        # ⇒ 三档不是同一把尺，不许合并成一句文案
        assert all(not re.fullmatch(r"[^/\\]+", n) for n in seps), seps
        assert all(re.fullmatch(r"[^/\\]+", n) for n in urlish + colonish), urlish + colonish
        assert len({r for _n, r in REJECTED}) == 8, "八档判决文案必须各自独立，不许两档共用一句"


class TestAcceptedNamesStayAddressable:
    """放行档的反向证据：新闸不许把好名字一并拒掉——每一个收下的名字都得用得完

    最长那支取 **120** 而不是 255：本闸的 255 只是**必要**条件，路径总长还叠加备份目录那一段，
    Windows 上更早的一刀由操作系统落下（实测 200 字符可成、240 字符 `errno 22`），那一档由
    `TestWhatTheOsCutsFirst` 单独钉，不许混进「合法名可达」这条断言里。
    """

    @pytest.mark.parametrize(
        "name",
        ["nightly", "nightly-01", "a.b.c", "a b", "备份-周三", "w" * 120],
        ids=["plain", "hyphen", "interior_dots", "interior_space", "cjk", "char_120"],
    )
    def test_create_restore_delete_all_reach_it(self, http, name):
        client, root = http
        bdir = root / ".ok"
        r = client.post(CREATED, json={"input_file": "p.json", "name": name},
                        params={"backup_dir": str(bdir)})
        assert r.status_code == 200, f"{name!r} 实得 {r.status_code}: {r.text[:160]}"
        assert r.json()["backup_id"] == name, "回执名 ≠ 请求名 ⇒ 出现了第二个 id 面"
        assert re.fullmatch(r"[^/\\]+", name), f"{name!r} 根本不是一个路径段"
        wire = urllib.parse.quote(name, safe="")
        rr = client.post(f"{CREATED}/{wire}/restore", json={"output_file": f"out-{wire}.json"},
                         params={"backup_dir": str(bdir)})
        assert rr.status_code == 200, f"{name!r} 恢复不到: {rr.status_code} {rr.text[:120]}"
        assert (root / f"out-{wire}.json").is_file()
        dr = client.delete(f"{CREATED}/{wire}", params={"backup_dir": str(bdir)})
        assert dr.status_code == 200, f"{name!r} 删不掉: {dr.status_code} {dr.text[:120]}"
        assert [b["backup_id"] for b in client.get(
            CREATED, params={"backup_dir": str(bdir)}).json()["backups"]] == []


class TestEmptyStringStillMeansUnset:
    """L231 契约第 32 行原样保留：`name=""` 走 `name or 时间戳`，不是本轮判据的靶子"""

    def test_blank_body_name_gets_a_generated_id_and_survives_the_gate(self, http):
        client, root = http
        bdir = root / ".gen"
        r = client.post(CREATED, json={"input_file": "p.json", "name": ""},
                        params={"backup_dir": str(bdir)})
        assert r.status_code == 200, r.text
        bid = r.json()["backup_id"]
        assert re.fullmatch(r"backup_\d{8}_\d{6}", bid), bid
        assert client.delete(f"{CREATED}/{bid}",
                             params={"backup_dir": str(bdir)}).status_code == 200


class TestWhatTheOsCutsFirst:
    """255 这一档是**必要不充分**：Windows 更早落下的一刀不许变成新的失踪态

    实测（Temp `probe_255.py`，备份目录名长固定）：100 / 160 / 200 字符 ⇒ **200**；
    240 / 255 字符 ⇒ **400**「文件路径包含当前系统不能的字符或超过域名长度」（`errno 22`，
    由 L271 的 `is_rejected_path_name` 映射，边界随备份目录长度移动）。这一臂钉的是
    「那一刀落下来之后没有留下寻不了址的东西」：没有 `index.json`、没有备份文件、清单回空。
    """

    def test_an_os_refused_long_name_leaves_no_phantom(self, http):
        client, root = http
        bdir = root / ".long"
        r = client.post(CREATED, json={"input_file": "p.json", "name": "z" * 255},
                        params={"backup_dir": str(bdir)})
        assert r.status_code == 400, r.text
        assert "路径分隔符" not in r.json()["detail"], "这一档由操作系统判，不该顶着新闸的文案"
        listed = client.get(CREATED, params={"backup_dir": str(bdir)})
        assert [b["backup_id"] for b in listed.json()["backups"]] == [], "清单里躺着一条动不了的备份"
        assert not list(bdir.rglob("*.json")), f"落盘残骸：{sorted(bdir.rglob('*.json'))}"


class TestTheRulerAndTheWiring:
    """判据住在哪、接在哪、有没有搬动被点名的行号——三面各钉一条，防止修法退化成口号"""

    def test_the_gate_sits_at_the_birth_of_the_id_on_one_line(self):
        src = BACKUP_PY.read_text(encoding="utf-8")
        assert src.count("require_path_segment(") == 1, "名字只有一个出生点，接两处就是两套口径"
        lines = src.split("\n")
        birth = [i for i, ln in enumerate(lines, 1) if "require_path_segment(" in ln]
        assert len(birth) == 1
        # 同行包裹（不插行）：`backup.py:117` 那一句拼接被 l259 的名册按行号点名，
        # 插一行会把它平移出被指认的位置 ⇒ 修法必须写成 `= require_path_segment(… or …)`
        assert "backup_id = require_path_segment(" in lines[birth[0] - 1]
        assert "backup_path = self._backup_dir / f\"{backup_id}.json\"" in lines[116]
        assert birth[0] == 116, f"出生点漂了（现量第 {birth[0]} 行）⇒ l259 名册行号要跟着重量"

    def test_the_old_landing_gate_is_kept_as_second_line(self):
        src = BACKUP_PY.read_text(encoding="utf-8")
        assert "if not path_lands_within(" in src, "两道闸口径不同（名字 vs 落点），不许顺手摘旧闸"

    def test_the_validator_returns_the_value_untouched(self):
        """不裁剪 ⇒ 不造第二个 id 面：收下什么就落什么，只在成立与否上说话"""
        from augmentor.validation import require_path_segment

        for value in ("a b", "a.b.c", "  legit", "x" * 255):
            assert require_path_segment("备份名称", value) == value
        assert require_path_segment("备份名称", "ok") == "ok"

    def test_the_gate_is_a_data_validation_error_so_http_reads_it_400(self):
        from augmentor.exceptions import DataValidationError
        from augmentor.validation import require_path_segment

        with pytest.raises(DataValidationError) as exc:
            require_path_segment("备份名称", "sub/deep")
        assert issubclass(DataValidationError, ValueError), "to_http_error 认 ValueError ⇒ 400 不是 500"
        assert "寻不了址" in str(exc.value), "判决要点名后果，不然调用方不知道为什么要改名字"

    def test_route_description_and_openapi_both_announce_the_new_refusal(self):
        """对外面不许停在旧口径：路由 400 说明要跟着改口，且真进 OpenAPI"""
        route = (ROOT / "api" / "routes" / "system_ops.py").read_text(encoding="utf-8")
        assert "路径段" in route and "指向备份目录之外" not in route, \
            "路由说明里仍留着旧那句 ⇒ 契约面上还是在承诺「根内嵌套名可用」"
        from api.main import app

        spec = app.openapi()["paths"][CREATED]["post"]
        desc = json.dumps(spec["responses"]["400"], ensure_ascii=False)
        assert "寻不了址" in desc, desc[:200]

    def test_the_front_end_and_API_doc_rows_announce_it_too(self):
        """前端注释与 `docs/augmentor/API.md` 的详表行同批改口（A357 那一族失效面的正向那一面）"""
        ts = (ROOT / "web" / "src" / "services" / "api.ts").read_text(encoding="utf-8")
        assert "寻不了址" in ts, "服务层里仍写着「带 / 放行」⇒ 下一个改这里的人会被旧说明误导"
        # 前端那一半：两只按 id 寻址的入口都必须编码（存量含 `#` 的备份要还能删得掉）
        assert ts.count("encodeURIComponent(backupId)") == 2, \
            "restore 与 delete 两只入口都得走 encodeURIComponent，实得 " \
            f"{ts.count('encodeURIComponent(backupId)')} 处"
        assert "${backupId}" not in ts, "还有一处裸插值 ⇒ 那个入口对含 `#` 的存量 id 仍是 404"
        doc = (ROOT.parent / "docs" / "augmentor" / "API.md").read_text(encoding="utf-8")
        # 详表里同名端点出现两次（速览一行 + 详述一行），只取详述那一行：按行长度切，不按同名配对
        rows = [ln for ln in doc.split("\n")
                if ln.startswith("| `POST` | `/api/system/backups` |") and len(ln) > 200]
        assert len(rows) == 1, f"详述行应恰好一行，实得 {len(rows)}"
        assert "寻不了址" in rows[0] and "A366" in rows[0]

