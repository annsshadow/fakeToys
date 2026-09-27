# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L94 / A168：仓库根产物残留的全局守卫

这轮要收的是 L93 踩过的那格：`tests/conftest.py` 的 `allow_temp_data_roots` 把
`os.getcwd()` 整个放进路径白名单，于是用例有权往**仓库根**写文件；写掉的东西不会随用例结束
消失，而它会改写后面用例的落点判据（L93 实测：三条上传用例先把 `l87_ok.json`、
`l87_edge_ok.json`、`l87dir/nested.json` 真写进了 `augmentor/`，产品码还原后那三条**仍然红**，
因为「候选里第一个存在的」命中的是上一轮留下的残留 —— 磁盘历史单独就能翻转判决）。
当时清扫只住在变异探针里，救不了「跑完整套忘了看一眼」那种形状，所以本轮把它做成逐用例判据。

入库前先量，两端都有数（同形状探针，L94 实测）：

| 跑法 | 结果 |
| --- | --- |
| 全套 + 残留探针（7 063 passed / 3 skipped） | `RESIDUE_COUNT 0` ⇒ 守卫**不需要白名单**，也不撞既有 7 千条用例 |
| 一条故意留产物的注入用例 | `RESIDUE_COUNT 1 CASES 1`，名单里就是它的 node-id ⇒ 尺子能红 |

守卫的**范围**也在这里钉住（见 `tests/residue_watch.py` 的表格）：只看 `WATCHED` 三处的
顶层子项，只认「开始前不存在、结束后还在」。所以「建了又删」「改已有文件的内容」
「删掉一件原有产物」「往更深一层写」都不归它管 —— 那不是漏，是桶的边界，
越界的那几格各有自己的尺子。
"""

import os
import sys

from tests.residue_watch import WATCHED, display, find_residue, leaked, list_top, snapshot


#: 仓库根（本文件在 `augmentor/tests/unit/` 下，往上两层）
REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


class TestRulerScope:
    """判据的五档边界，每档双向注入一次（该红的红、不该红的不红）"""

    def test_new_top_level_entry_under_a_watched_dir_is_residue(self, tmp_path):
        root = str(tmp_path)
        os.mkdir(os.path.join(root, "data"))
        before = snapshot(root)
        with open(os.path.join(root, "data", "out.json"), "w", encoding="utf-8") as handle:
            handle.write("{}")

        assert leaked(root, before) == ["data/out.json"]

    def test_entry_at_the_repo_root_is_shown_bare_not_dot_slashed(self, tmp_path):
        root = str(tmp_path)
        before = snapshot(root)
        with open(os.path.join(root, "leak.json"), "w", encoding="utf-8") as handle:
            handle.write("{}")

        got = leaked(root, before)
        assert got == ["leak.json"], f"仓库根的产物要相对根给裸名，实测 {got}"

    def test_a_watched_dir_created_mid_test_reports_its_own_entries(self, tmp_path):
        """`data/` 原本不存在也不算崩溃；用例把它建出来并写入，两笔都算残留

        实测形状是 `["data", "data/x.json"]` 而不是只报里面那件：目录本身是仓库根顶层
        多出来的子项，那件文件是 `data/` 顶层多出来的子项，两笔都是真的。宁多不漏。
        """
        root = str(tmp_path)
        assert snapshot(root)["data"] == set()
        before = snapshot(root)
        os.makedirs(os.path.join(root, "data"))
        with open(os.path.join(root, "data", "x.json"), "w", encoding="utf-8") as handle:
            handle.write("{}")

        assert leaked(root, before) == ["data", "data/x.json"]

    def test_created_then_deleted_is_not_residue(self, tmp_path):
        root = str(tmp_path)
        before = snapshot(root)
        path = os.path.join(root, "data")
        os.mkdir(path)
        with open(os.path.join(path, "temp.json"), "w", encoding="utf-8") as handle:
            handle.write("{}")
        os.remove(os.path.join(path, "temp.json"))
        os.rmdir(path)

        assert leaked(root, before) == []

    def test_changing_a_pre_existing_file_contents_is_not_residue(self, tmp_path):
        root = str(tmp_path)
        with open(os.path.join(root, "keep.json"), "w", encoding="utf-8") as handle:
            handle.write("before")
        before = snapshot(root)
        with open(os.path.join(root, "keep.json"), "w", encoding="utf-8") as handle:
            handle.write("after")

        assert leaked(root, before) == []

    def test_removing_a_pre_existing_entry_is_out_of_this_bucket(self, tmp_path):
        """删掉原有产物不是「残留」，本守卫如实不管 —— 那是另一条判据的形状"""
        root = str(tmp_path)
        with open(os.path.join(root, "old.json"), "w", encoding="utf-8") as handle:
            handle.write("{}")
        before = snapshot(root)
        os.remove(os.path.join(root, "old.json"))

        assert leaked(root, before) == []

    def test_deeper_paths_are_not_watched(self, tmp_path):
        """只盯顶层：往 `data/` 更深一层写不算残留（那是产品自己的目录结构）"""
        root = str(tmp_path)
        os.makedirs(os.path.join(root, "data", "nested"))
        before = snapshot(root)
        os.makedirs(os.path.join(root, "data", "nested", "deeper"))
        with open(os.path.join(root, "data", "nested", "deeper", "x.json"),
                  "w", encoding="utf-8") as handle:
            handle.write("{}")

        assert leaked(root, before) == []

    def test_watched_set_is_the_three_documented_dirs(self):
        assert WATCHED == ("", "data", ".backups")

    def test_snapshot_covers_every_watched_dir(self, tmp_path):
        got = snapshot(str(tmp_path))

        assert sorted(got) == sorted(WATCHED)
        assert all(isinstance(value, set) for value in got.values())

    def test_list_top_on_a_missing_dir_is_empty_not_an_error(self, tmp_path):
        assert list_top(str(tmp_path / "nope")) == set()


class TestRulerOnTheRealTree:
    """尺子架在真仓库上的一次读数：证明 `display` 与快照口径在这里也成立"""

    def test_the_repo_root_snapshot_matches_the_live_listing(self):
        assert snapshot(REPO_ROOT)[""] == set(os.listdir(REPO_ROOT))

    def test_a_real_entry_doctored_out_of_the_baseline_is_reported_bare(self):
        """把一件**真实存在**的根产物从基线里抹掉，尺子要正好把它点名出来

        这不是自证：`before` 被人造错了，报出来的必须是那一条、而且是相对根的裸名形式。
        """
        live = snapshot(REPO_ROOT)
        chosen = sorted(live[""])[0]
        doctored = {key: set(value) for key, value in live.items()}
        doctored[""].discard(chosen)

        got = leaked(REPO_ROOT, doctored)

        assert got == [chosen]
        assert "\\" not in got[0] and "./" not in got[0]

    def test_display_never_leaks_a_backslash(self):
        assert display(REPO_ROOT, "data", "x.json") == "data/x.json"
        assert display(REPO_ROOT, "", "x.json") == "x.json"


class TestGuardIsWired:
    """守卫必须真的挂在每一个用例上，而不只是住在 conftest 里等着被删装饰器"""

    def test_the_fixture_applies_without_being_requested(self, request):
        """autouse 的活体证据：本用例签名里没写它，会话仍然把它挂给了我"""
        assert "fail_on_repo_residue" in request.node.fixturenames

    def test_find_residue_names_the_case_the_count_and_the_paths(self, tmp_path):
        root = str(tmp_path)
        before = snapshot(root)
        with open(os.path.join(root, "leak.json"), "w", encoding="utf-8") as handle:
            handle.write("{}")

        message = find_residue(root, before, "tests/unit/test_x.py::test_y")

        assert message is not None
        assert "tests/unit/test_x.py::test_y" in message
        assert "1 件新产物" in message and "leak.json" in message

    def test_find_residue_is_silent_when_the_case_leaves_nothing(self, tmp_path):
        root = str(tmp_path)
        before = snapshot(root)
        path = os.path.join(root, "ok.json")
        with open(path, "w", encoding="utf-8") as handle:
            handle.write("{}")
        os.remove(path)

        assert find_residue(root, before, "tests/unit/test_x.py::test_y") is None

    def test_the_conftest_fixture_fails_instead_of_printing(self):
        """接线读数：conftest 里那份守卫调的是本模块的判定，并且用 `pytest.fail` 出声

        pytest 9 起不许直接调用 fixture，所以「留残留会让用例变红」这件事不能在本进程里
        驱动，改由下面那条子进程对照来证；这条钉的是接线形状 —— 把 `pytest.fail` 换成
        `print`（红变成提示）或把判定换成别的东西，这里就先红。
        """
        import inspect

        import tests.conftest as live_conftest

        source = inspect.getsource(live_conftest.fail_on_repo_residue)

        assert "@pytest.fixture(autouse=True)" in source
        assert "find_residue(" in source and "snapshot(" in source
        assert "pytest.fail(" in source

    def _mini_run(self, tmp_path, body: str):
        """在一个独立的最小 rootdir 里跑一次真 pytest，用同一份判定逻辑当守卫

        Args:
            tmp_path: 本次的临时仓库根
            body: 写进 `test_case.py` 的用例正文

        Returns:
            `(退出码, 输出)`
        """
        import subprocess

        wrapper = (
            "import os\nimport pytest\nimport sys\n\n"
            f"sys.path.insert(0, {REPO_ROOT!r})\n\n"
            "from tests.residue_watch import find_residue, snapshot\n\n\n"
            '@pytest.fixture(autouse=True)\ndef guard(request):\n'
            "    before = snapshot(os.getcwd())\n"
            "    yield\n"
            "    message = find_residue(os.getcwd(), before, request.node.nodeid)\n"
            "    if message:\n"
            "        pytest.fail(message)\n"
        )
        (tmp_path / "conftest.py").write_text(wrapper, encoding="utf-8")
        (tmp_path / "test_case.py").write_text(body, encoding="utf-8")
        env = dict(os.environ, PYTHONIOENCODING="utf-8")
        run = subprocess.run(
            [sys.executable, "-m", "pytest", "-q", "-p", "no:cacheprovider", "--no-header"],
            cwd=str(tmp_path), env=env, capture_output=True, timeout=180,
        )
        return run.returncode, (run.stdout + run.stderr).decode("utf-8", "replace")

    def test_a_case_that_leaves_residue_really_goes_red(self, tmp_path):
        """端到端红：autouse 包一份本模块判定，留残留的那条用例必须让这轮跑不出 0

        实测形状是 `1 passed, 1 error`，报错正文里点着产物名 —— 也就是判红发生在
        teardown 阶段，被点名的仍是那条留下残留的用例本身，不会连带下一条（残留对下一条
        而言已经是「开始前就在」）。这条就是尺子能红的正面证据。
        """
        code, out = self._mini_run(
            tmp_path,
            "def test_leaves_a_file():\n"
            "    with open('leak.json', 'w', encoding='utf-8') as handle:\n"
            "        handle.write('{}')\n",
        )

        assert code != 0, f"留残留的那轮本该非零退出，实测 rc={code}\n{out}"
        assert "leak.json" in out
        assert "1 error" in out

    def test_a_case_that_cleans_up_after_itself_stays_green(self, tmp_path):
        """端到端绿：同一份守卫下，建了又删的用例不能被误判 —— 否则 7 千条一起遭殃"""
        code, out = self._mini_run(
            tmp_path,
            "def test_creates_and_removes():\n"
            "    with open('temp.json', 'w', encoding='utf-8') as handle:\n"
            "        handle.write('{}')\n"
            "    import os\n\n"
            "    os.remove('temp.json')\n",
        )

        assert code == 0, f"干净的用例不该被守卫判红：\n{out}"
        assert "1 passed" in out


def test_sys_modules_identity_of_conftest():
    """`tests.conftest` 与 pytest 加载的那份必须是同一个模块对象

    接线那条用例是靠 `import tests.conftest` 拿到守卫本体的；如果 pytest 用别的名字加载
    conftest，那个 import 就会拿到第二份副本，接线守卫就成了「看一份复制品」。这里把
    这个前提本身钉住，前提崩了要在这里红。
    """
    module = sys.modules.get("tests.conftest")

    assert module is not None, "pytest 未以 tests.conftest 之名加载 conftest"
    assert os.path.realpath(module.__file__) == os.path.realpath(
        os.path.join(REPO_ROOT, "tests", "conftest.py"))
