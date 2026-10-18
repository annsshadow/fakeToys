# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L234（A314）：`current` 那一格 —— 断链不许炸掉 create_version，链接目标不许双重前缀

**这条守卫是本轮全量门禁那一支红倒推出来的**，不是设计出来的。原文：

    OSError: [WinError 1920] 系统无法访问此文件。: 'data\\versions\\current'
    augmentor/versioning.py:185: in create_version → if self._current_symlink.exists():

现场取证（本机 `data/versions/`，gitignored 的运行期产物）量到三件事，每件都推翻了
「删掉那个断链就完事」的处置：

1. `current` 在 Git Bash 的 `ls -la` 里显示成 `lrwxrwxrwx current -> data/versions/v_…`，
   但 Python 侧 `is_symlink()` 是 **False**、`os.readlink()` 报 *not a symbolic link*、
   只有 `os.stat` 抛 1920 ⇒ 它不是 NT 符号链接，是 **WSL 在 drvfs 上留下的 LX 链接**。
   所以「先判 is_symlink 再判 exists」这一类重排**治不了它**：lstat 也认不出来。
2. 链接的目标串是 `data/versions/v_20261011_130434_980900` —— 而链接自己就住在
   `data/versions/` 里 ⇒ 按自己所在目录解析成 `data/versions/data/versions/v_…`，
   **恒断链**。成因是 `symlink_to(version_dir)` 把整段相对路径当目标存了进去；
   `storage_dir` 的出厂默认正是相对路径 `data/versions`。
3. 这个目录里有 **67** 个条目（`v_*` 快照 + `current.txt` + `history.jsonl`），
   全是测试跑出来的：`AugmentorPipeline` 按 `config.versioning.storage_dir` 建
   `VersionManager`，而用例的 cwd 就是仓库根 ⇒ 每跑一次套件就往仓库里堆一批快照。
   这一格是本文件的**范围之外**（见 A315），但它解释了 LX 链接是怎么进来的：
   在 WSL 里跑过一次套件，`current` 就变成了 Windows 读不了的东西。

**两处修法（`augmentor/versioning.py`，`create_version` 与 `rollback` 各一处，净零行 ——
账本里有三处活引用钉在这只文件的 `:205` / `:289` / `:380` 上（L273 因插行重钉，原为 `:197` / `:281` / `:372`），插行会平移它们）**：

- 判存在与清除搬进新的 `_clear_current_link()`：先 lstat 再 stat，**stat 抛出来就当
  「那里有东西」照清**；清除按「先当文件删、删不掉再当目录删」两步，两步都失败就放过
  （这一格是便利索引不是数据，随后的 `symlink_to` 会拿到 FileExistsError 并落回 `.txt`）。
- `symlink_to(version_dir)` → `symlink_to(version_dir.name)`：目标只写版本目录名。

**修完的现场证据**：那支红的用例转绿，`data/versions/current` 这个 LX 链接被 `unlink`
清掉了（`ls` 报 No such file），`current.txt` 重写成最新版本 id ⇒ 自愈，不用人工删。

四面判据，各对应一种真实失效形状：

1. **相对 storage_dir 下连创两版不炸**，且 `current` 指的是最新那一版（`TestRelativeStorageDir`）。
   这一面在两个平台上都真跑磁盘。
2. **链接目标是裸名**（`TestTheLinkTargetIsABareName`）。本机 `os.symlink` 直接吃
   WinError 1314（无管理员特权）⇒ 真链接建不出来，「目标写的是裸名还是整段相对路径」
   在真盘上无从观测；所以这一面用替身把 `symlink_to` 的实参录下来，两个平台同一份判据。
   第 1 面里另有一条**只在链接真建得出来时才生效**的读盘对账（`os.readlink` 不含分隔符），
   Linux CI 上它是活的，本机上是空档 —— 这一点写在断言消息里，不装作两边都量到了。
3. **stat 抛 1920 也不许炸**（`TestUnstatableCurrentDoesNotBlowUp`）：`create_version` 与
   `rollback` 各一条。这正是门禁那支红的形状，也是修法①的棘轮。
4. **`.txt` 不会停在旧版本上**（`TestTheTxtCannotGoStale`）：`_clear_current_link` 把
   配套的 `.txt` 一起清掉，所以「建得出链接」与「建不出链接」两次交替跑，读侧也不会
   拿到一个更早的版本。改前 `rollback` 那一支只在 `except OSError` 里覆写 `.txt`，
   链接建得出来时那份旧 `.txt` 会一直躺着。

跑法：`python -m pytest tests/unit/test_version_current_link_l234.py -q --no-cov`
"""

import os
import pathlib

import pytest

from augmentor.versioning import VersionManager

ITEMS = [{"instruction": "甲", "input": "", "output": "乙"}]
OTHER = [{"instruction": "丙", "input": "", "output": "丁"}]


@pytest.fixture
def manager(tmp_path, monkeypatch):
    """cwd 换到临时目录，storage_dir 用**相对路径**——出厂默认那一档

    相对路径是本轮缺陷的成因之一（链接按自己所在目录解析），所以判据必须在这一档上量。
    顺带把仓库的 `data/versions/` 挡在外面：改前每跑一次用例就往仓库里堆一个 `v_*` 快照。
    """
    monkeypatch.chdir(tmp_path)
    return VersionManager("data/versions")


def current_paths(root: pathlib.Path):
    return root / "data" / "versions" / "current", root / "data" / "versions" / "current.txt"


class FakeCurrent:
    """`current` 那一格的替身：录下 `symlink_to` 的实参，按需让 `exists()` 抛 WinError 1920

    `is_symlink()` 在链接建出来之前恒为 False、`unlink()` 只记账不动盘 —— 这两条正是 LX
    链接的真实形状（lstat 认不出来、但 DeleteFile 删得掉）。`link_ok=True` 时模拟「这台机器
    建得出链接」（Linux、开了开发者模式的 Windows），`symlink_to` 不抛。`with_suffix` 递回
    真路径，好让 `_clear_current_link` 循环里的第二项（`.txt`）真的落盘、真的被清。
    """

    def __init__(self, real: pathlib.Path, stat_raises: bool = False, link_ok: bool = False):
        self.real = real
        self.stat_raises = stat_raises
        self.link_ok = link_ok
        self.linked_to = []
        self.unlink_calls = 0
        self.linked = False

    def is_symlink(self):
        return self.linked

    def exists(self):
        if self.stat_raises:
            raise OSError("模拟 WinError 1920：系统无法访问此文件。")
        return self.real.exists()

    def with_suffix(self, suffix):
        return self.real.with_suffix(suffix)

    def unlink(self, missing_ok=False):
        self.unlink_calls += 1
        self.linked = False

    def symlink_to(self, target, **kwargs):
        self.linked_to.append(target)
        if not self.link_ok:
            raise OSError("模拟 WinError 1314：客户端没有所需的特权。")
        self.linked = True


class TestRelativeStorageDir:
    """第 1 面：真跑磁盘 —— 相对 storage_dir 下连创两版不炸，current 指最新那版"""

    def test_two_versions_in_a_row_do_not_raise(self, manager, tmp_path):
        v1 = manager.create_version(ITEMS, "第一版")
        v2 = manager.create_version(OTHER, "第二版")
        assert v2.version_id != v1.version_id
        assert manager.get_current_version() == v2.version_id, (
            "连创两版之后 current 仍指着旧版本 ⇒ 清除或重写那一格没生效")

    def test_exactly_one_face_of_current_names_the_newest(self, manager, tmp_path):
        """链接与 .txt 是同一件事的两张脸：哪张在场就该写着最新版本 id

        建得出链接的平台（Linux、开了开发者模式的 Windows）走 `is_symlink` 那一支；
        建不出的（本机，WinError 1314）走 `.txt` 那一支。两支都不许给出旧版本。
        """
        v1 = manager.create_version(ITEMS, "第一版")
        v2 = manager.create_version(OTHER, "第二版")
        link, txt = current_paths(tmp_path)

        if link.is_symlink():
            target = os.readlink(link)
            assert os.sep not in target and "/" not in target, (
                f"链接目标是 {target!r} 而不是裸版本目录名 —— 它按自己所在目录解析，"
                f"带前缀就恒断链（A314 的第二半）")
            assert target == v2.version_id
            assert link.exists(), f"current 是断链：{link} -> {target}"
        else:
            assert txt.is_file(), f"链接与 .txt 两张脸都不在场：{list(link.parent.iterdir())}"
            assert txt.read_text(encoding="utf-8").strip() == v2.version_id
        if txt.is_file():
            assert v1.version_id not in txt.read_text(encoding="utf-8"), (
                f".txt 里同时留着 {v1.version_id} 与 {v2.version_id} ⇒ 那一格是追加而不是覆写")

    def test_load_version_still_reads_the_snapshot_back(self, manager):
        v1 = manager.create_version(ITEMS, "第一版")
        manager.create_version(OTHER, "第二版")
        assert manager.load_version(v1.version_id) == ITEMS, (
            "清 current 那一格时把版本目录一起删了 ⇒ _clear_current_link 的范围越界")


class TestTheLinkTargetIsABareName:
    """第 2 面：替身录下 `symlink_to` 的实参 —— 本机建不出真链接，这一面才量得到"""

    def test_create_version_links_to_the_bare_directory_name(self, manager, tmp_path):
        fake = FakeCurrent(tmp_path / "data" / "versions" / "current")
        manager._current_symlink = fake
        info = manager.create_version(ITEMS, "第一版")
        assert fake.linked_to == [info.version_id], (
            f"symlink_to 收到的是 {fake.linked_to}，而链接住在 storage_dir 里、"
            f"按自己所在目录解析 ⇒ 只可以是裸版本目录名")

    def test_rollback_links_to_the_bare_directory_name(self, manager, tmp_path):
        v1 = manager.create_version(ITEMS, "第一版")
        fake = FakeCurrent(tmp_path / "data" / "versions" / "current")
        manager._current_symlink = fake
        assert manager.rollback(v1.version_id) is True
        assert fake.linked_to == [v1.version_id], fake.linked_to


class TestUnstatableCurrentDoesNotBlowUp:
    """第 3 面：门禁那支红的形状 —— `exists()` 抛 WinError 1920，两个入口都不许炸"""

    def test_create_version_survives_an_unstatable_current(self, manager, tmp_path):
        link = tmp_path / "data" / "versions" / "current"
        link.parent.mkdir(parents=True, exist_ok=True)
        manager._current_symlink = FakeCurrent(link, stat_raises=True)
        info = manager.create_version(ITEMS, "第一版")
        assert info.item_count == 1
        assert manager._current_symlink.unlink_calls >= 1, (
            "stat 抛出来的那一格没被当作「那里有东西」照清 ⇒ 修法①的棘轮松了")

    def test_rollback_survives_an_unstatable_current(self, manager, tmp_path):
        v1 = manager.create_version(ITEMS, "第一版")
        link = tmp_path / "data" / "versions" / "current"
        manager._current_symlink = FakeCurrent(link, stat_raises=True)
        assert manager.rollback(v1.version_id) is True

    def test_a_real_directory_in_that_slot_is_cleared_too(self, manager, tmp_path):
        """老注释承诺过「使用目录副本」，既有测试也确实会 `mkdir` 出真目录 ⇒ 这一档要在"""
        link = tmp_path / "data" / "versions" / "current"
        v1 = manager.create_version(ITEMS, "第一版")
        if link.is_symlink():
            link.unlink()
        link.mkdir(parents=True, exist_ok=True)
        (link / "leftover.json").write_text("{}", encoding="utf-8")
        v2 = manager.create_version(OTHER, "第二版")
        assert v2.version_id != v1.version_id
        assert manager.get_current_version() == v2.version_id


class TestTheTxtCannotGoStale:
    """第 4 面：链接那张脸赢的时候，旧 `.txt` 必须一起被清掉

    改前两个入口都只在 `except OSError` 里**覆写** `.txt`：链接建得出来的那次根本不碰它，
    于是一份写着更早版本的 `.txt` 会一直躺在那儿。读侧今天先判 `is_symlink()`，所以它暂时
    读不到 —— 但那张链接哪天不在了（换机器、换文件系统、被人 `rm`），读侧就退回旧版本。
    """

    def test_a_stale_txt_is_removed_when_the_link_face_wins(self, manager, tmp_path):
        link, txt = current_paths(tmp_path)
        # ⚠️ 第一版必须**确定**落在 .txt 上，所以开头就注入「建不出链接」的替身。
        #
        # 曾经的写法是直接 `manager.create_version(...)`，注释写着「本机建不出链接」——
        # 那是 Windows 无管理员特权时的形状（WinError 1314）。Linux CI 上 `os.symlink`
        # 照常成功，`current` 格是个真链接、`.txt` 从头到尾不存在，下一行的
        # `assert txt.is_file()` 当场红（夹具前提不成立）。这正是本项目反复栽的
        # 「开发机绿、CI 红」：前提依赖运行平台的字节。
        # 用替身把「建不出链接」钉成夹具的一部分，两个平台同一份判据。
        manager._current_symlink = FakeCurrent(link, link_ok=False)
        manager.create_version(ITEMS, "第一版")   # 链接建不出 ⇒ 这一版落在 .txt 上
        assert txt.is_file(), "夹具前提不成立：这一格连 .txt 都没有，后面量不到东西"
        txt.write_text("v_19700101_000000_000000", encoding="utf-8")

        fake = FakeCurrent(link, link_ok=True)    # 换一台建得出链接的机器
        manager._current_symlink = fake
        v2 = manager.create_version(OTHER, "第二版")

        assert fake.linked_to == [v2.version_id]
        assert not txt.exists(), (
            f"链接建得出来时旧 .txt 没被一起清掉，里面还写着 "
            f"{txt.read_text(encoding='utf-8')!r} ⇒ 链接那张脸一丢，读侧就退回 1970 那一版")

    def test_rollback_clears_the_txt_too(self, manager, tmp_path):
        link, txt = current_paths(tmp_path)
        v1 = manager.create_version(ITEMS, "第一版")
        txt.write_text("v_19700101_000000_000000", encoding="utf-8")

        fake = FakeCurrent(link, link_ok=True)
        manager._current_symlink = fake
        assert manager.rollback(v1.version_id) is True
        assert not txt.exists(), "rollback 那一支只覆写不清除 ⇒ 同一份旧 .txt 换个入口还在"
