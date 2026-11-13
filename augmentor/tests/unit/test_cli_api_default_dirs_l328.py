# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L328（A301 + A308；用户 2026-11-13 拍板①）：CLI 的两个默认目录与端点那一份**同源**

**这一族是什么**：A301「默认备份目录在仓内有三份」、A308「默认注册表目录在仓内有三份」——
两份说的是同一个病：同一句「默认目录」，CLI / SDK / HTTP 三个面各说各话。

**改前实测**（2026-11-13 探针，工作目录 = `augmentor/`）：

| 面 | 备份目录 | 注册表目录 |
| --- | --- | --- |
| 端点（省略参数） | `<白名单首根>/.backups` = `data/.backups` | `<白名单首根>/.dependency_registry` |
| CLI 默认 | `.backups`（**CWD 相对**） | `.dependency_registry`（CWD 相对） |
| SDK 默认 | `.backups`（CWD 相对） | `.dependency_registry`（CWD 相对） |

症状：SDK 在 CWD 下建出的备份，端点默认目录里**有没有它 = False**；SDK 登记的 1 个数据集，
端点 `GET /api/system/dependency/datasets` 回 **0 条** ⇒ **CLI 建的页面看不见，反之亦然**。

**决策**（用户 2026-11-13 拍板**①**）：**CLI 的默认改成 `data/.backups` 与
`data/.dependency_registry`**（仍相对 CWD）⇒ 与端点那一份**同源**；
**SDK 的库级默认不动**（那一层没有白名单概念 ⇒ 保持 CWD 相对是库的惯例）。

**留下的残留（如实记，不藏）**：CLI 这一份按出厂数据根 `data` **写死** ⇒ 部署把
`web.data_roots` 第一项改名时，CLI 与端点**又漂开** ⇒ 本文件把那一格**钉成机器可查**：
出厂数据根一改，`test_the_cli_defaults_follow_the_shipped_data_root` 当场红。

**这一份守卫的分工**：A301 / A308 两支旧守卫只钉「三份默认各是什么」；本文件钉的是
**决策本身**（改后 CLI 那份与端点那份同源、SDK 那份不动、显式传参仍然优先）。
"""

import inspect
import json
import pathlib

import pytest

from api.deps import default_backup_dir, default_registry_dir
from augmentor.backup import create_backup, list_backups
from augmentor.cli.parser import build_parser
from augmentor.dependency import DependencyManager

ROOT = pathlib.Path(__file__).resolve().parents[2]

#: 决策①钦定的两个 CLI 默认值（逐字）
CLI_BACKUP_DEFAULT = "data/.backups"
CLI_REGISTRY_DEFAULT = "data/.dependency_registry"

#: 决策①钦定**不动**的两个 SDK 库级默认（逐字）
SDK_BACKUP_DEFAULT = ".backups"
SDK_REGISTRY_DEFAULT = ".dependency_registry"


def _cli_defaults() -> tuple:
    """从**真正的解析器**里取两个 CLI 默认（不是从源码文本里抠字符串）

    用 `build_parser()` 而不是正则读源码：源码文本的形状一变（换引号、换写法），
    正则就哑了；而解析器的取值是**真行为**——CLI 跑起来拿到的就是这个值。
    """
    parser = build_parser()
    backup_args = parser.parse_args(["backup", "--action", "list"])
    registry_args = parser.parse_args(["dependency", "--action", "list"])
    return backup_args.backup_dir, registry_args.registry_path


def _shipped_first_data_root() -> str:
    """**出厂** `config.yaml` 里 `web.data_roots` 的第一项（出厂即 `data`）

    读的是**出厂配置**，不是夹具那份临时数据根 —— 夹具把数据根换到 tmp 目录下，
    那是测试的安排、不是出厂形状；而「CLI 默认与端点默认同源」这句话说的是**出厂形状**。
    """
    import yaml

    config = yaml.safe_load((ROOT / "config.yaml").read_text(encoding="utf-8"))
    roots = config["web"]["data_roots"]
    assert isinstance(roots, list) and roots, roots
    return roots[0]


@pytest.fixture
def project_root_as_cwd(tmp_path, monkeypatch):
    """把工作目录设到 tmp 目录，白名单数据根设成相对路径 `data`

    这样**两边的解释基准一致**：CLI 默认是相对 CWD 的，端点默认是相对白名单首根的
    ⇒ 两边都落到 `<tmp>/data/...` ⇒ 「是不是同一个目录」这一格可以**用行为**回答，
    而不是靠比对两串字面量。
    """
    monkeypatch.chdir(tmp_path)
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", "data")
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    return tmp_path


class TestTheCliDefaultsAreNowTheSameSourceAsTheEndpoint:
    """A301 + A308：**L328 收口**（用户 2026-11-13 拍板①）"""

    def test_the_cli_defaults_are_the_data_root_scoped_ones(self):
        """**改后**：CLI 两个默认都落到出厂数据根下（与端点那一份同源）"""
        backup_default, registry_default = _cli_defaults()
        assert backup_default == CLI_BACKUP_DEFAULT, backup_default
        assert registry_default == CLI_REGISTRY_DEFAULT, registry_default

    def test_the_cli_defaults_are_still_relative_paths(self):
        """**仍然是相对路径**：CLI 不该被钉成绝对路径（那会让它与部署目录绑死）"""
        backup_default, registry_default = _cli_defaults()
        assert not pathlib.Path(backup_default).is_absolute(), backup_default
        assert not pathlib.Path(registry_default).is_absolute(), registry_default

    def test_the_sdk_defaults_are_untouched(self):
        """决策的另一半：**SDK 的库级默认不动**（那一层没有白名单概念）"""
        for name in ("create_backup", "restore_backup", "list_backups",
                     "delete_backup", "clean_old_backups", "verify_backup_integrity"):
            from augmentor import backup as backup_module

            signature = inspect.signature(getattr(backup_module, name))
            assert signature.parameters["backup_dir"].default == SDK_BACKUP_DEFAULT, name

        signature = inspect.signature(DependencyManager.__init__)
        assert signature.parameters["registry_path"].default == SDK_REGISTRY_DEFAULT

    def test_the_cli_defaults_follow_the_shipped_data_root(self):
        """**同源那一格**：CLI 默认 == **出厂配置**第一个数据根下的那一份

        比的是**出厂 `config.yaml`** 的数据根（不是夹具那份临时目录 —— 夹具把数据根
        换到 tmp 目录下，那是测试的安排、不是出厂形状）⇒ 断言把「CLI 默认的父目录名
        == 出厂数据根名」**钉成机器可查的一格**：哪天有人把 `web.data_roots` 的第一项
        改了名而没同步 CLI 默认，**这一支当场红** ⇒ 那条「残留」从此不靠人记着。
        """
        backup_default, registry_default = _cli_defaults()
        shipped_root = pathlib.Path(_shipped_first_data_root())
        assert pathlib.Path(backup_default) == shipped_root / ".backups", backup_default
        assert pathlib.Path(registry_default) == shipped_root / ".dependency_registry", registry_default

    def test_the_cli_and_the_endpoint_resolve_to_one_directory(self, project_root_as_cwd):
        """**行为那一格**（不是比字面量）：同一个 CWD 下，两边解到**同一个目录**"""
        backup_default, registry_default = _cli_defaults()
        cwd = pathlib.Path.cwd()
        assert (cwd / backup_default).resolve() == default_backup_dir()
        assert (cwd / registry_default).resolve() == default_registry_dir()

    def test_a_backup_made_with_the_cli_default_is_visible_to_the_endpoint(
        self, project_root_as_cwd
    ):
        """**症状那一格**：改前「CLI 建的备份在端点默认目录里有没有它 = False」

        这一支量的是**改后**的那一格：CLI 默认落点建出来的备份，端点默认目录**看得见**。
        """
        source = pathlib.Path.cwd() / "ds.json"
        source.write_text(
            json.dumps([{"instruction": "q", "output": "a"}], ensure_ascii=False),
            encoding="utf-8",
        )
        backup_default, _ = _cli_defaults()
        info = create_backup(str(source), backup_dir=backup_default)

        listed = list_backups(str(default_backup_dir()))
        assert any(item.get("backup_id") == info.backup_id for item in listed), (
            "CLI 默认落点建出来的备份，端点默认目录里看不见 ⇒ 同源没兑现")

    def test_the_endpoint_sees_the_backup_through_http(self, project_root_as_cwd):
        """**跨面那一格**：端点那只读端点回执里**真的有** CLI 建的那一份"""
        from fastapi.testclient import TestClient

        from api.main import app

        source = pathlib.Path.cwd() / "ds.json"
        source.write_text(
            json.dumps([{"instruction": "q", "output": "a"}], ensure_ascii=False),
            encoding="utf-8",
        )
        backup_default, _ = _cli_defaults()
        info = create_backup(str(source), backup_dir=backup_default)

        client = TestClient(app)
        response = client.get("/api/system/backups")
        assert response.status_code == 200, response.text
        backups = response.json().get("backups", [])
        assert any(item.get("backup_id") == info.backup_id for item in backups), (
            "端点回执里没有 CLI 建的那一份 ⇒ CLI 与端点还不是同一份")

    def test_an_explicit_argument_still_wins_over_the_default(self):
        """**显式传参优先**：决策改的是**默认值**，不是把显式路径架空"""
        parser = build_parser()
        backup_args = parser.parse_args(
            ["backup", "--action", "list", "--backup-dir", "custom/bk"]
        )
        registry_args = parser.parse_args(
            ["dependency", "--action", "list", "--registry-path", "custom/reg"]
        )
        assert backup_args.backup_dir == "custom/bk"
        assert registry_args.registry_path == "custom/reg"
