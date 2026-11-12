# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L317（A317 / A323 / A282）：前端三格 —— 确认闸的顺序、toast 的同源、格式门

**这一支管三件事，全是「页面在说谎」那一族**：

| 条目 | 旧形状 | 新形状 |
| --- | --- | --- |
| **A317** | 迁移那一档的必填闸挂在 `Popconfirm` 的 `onConfirm` 上 ⇒ 用户先读一段「迁移会改字段名、产物已存在则整份覆盖」的破坏性警告、点一次「照改」，才得到「先填迁移产物的文件名」 | 判空在**确认之前**：点按钮就出声，二次确认框压根不出现 |
| **A323** | `Settings.handleSave` 的 toast 写死第三处措辞（「配置已保存到 config.yaml…」）⇒ 后端把 `ignored_keys` 拼进 `message` 的那一档判决在 toast 里必然丢失 | toast 直接显示 `result.message` |
| **A282** | `prettier --check src` 的仓内基线恒红（立账时 32 只文件），而 CI 里**没有**这道门 ⇒ 长期红也没人看见 | 仓内基线重排到绿，CI 里补上 `format:check` 这一步 |

**为什么这一族值得立尺**：三件事的共同形状是「**页面上的那句话与服务端的真实判决不是同一句**」。
A317 是「顺序错了」（该拦的那次点击与白确认长得一样），A323 是「措辞不是同源的」
（判决在响应体里、toast 里没有），A282 是「门不在」（红与不红都不影响任何人）。

**判据只钉源码形状**：这里跑不了 JS，所以钉三处**形状**——

1. **确认闸的顺序**：`System.tsx` 的迁移 `Popconfirm` 必须**受控打开**（源码里有
   `open={migrateConfirmOpen}` 与 `onOpenChange=`），且判空函数必须在 `onOpenChange`
   那一格里被调用（不是只在 `onConfirm` 那一格）。
2. **toast 的同源**：`Settings.tsx` 必须调 `message.success(result.message)`，
   且旧那句硬编码措辞一个字都不许留在代码面（注释里提一句不算——判据读剥完注释的代码面）。
3. **格式门在 CI 里**：仓库根 `.github/workflows/` 下的那份工作流 的前端 job 必须有一步
   `npm run format:check`；`package.json` 必须有 `format:check` 脚本；
   `.prettierrc.json` 必须存在（否则那道门是空转的）。

**注入臂**（每臂都在内存里做字符串手术，**零写入**，跑完复原由构造保证）：
A1 把 `open={migrateConfirmOpen}` 换回非受控 ⇒ 第 1 面红；
A2 把 `onOpenChange` 那一格里的判空调用删掉 ⇒ 第 1 面红；
A3 把 `message.success(result.message)` 换回旧措辞 ⇒ 第 2 面红；
A4 把 CI 里那一步删掉 ⇒ 第 3 面红。

跑法：`python -m pytest tests/unit/test_frontend_gate_and_toast_l317.py -q --no-cov`
"""

import pathlib
import re

from tests.unit.test_wired_dropdown_choices_l226 import strip_comments

ROOT = pathlib.Path(__file__).resolve().parents[2]
REPO_ROOT = ROOT.parent
SRC = ROOT / "web" / "src"
SYSTEM_TSX = SRC / "pages" / "System.tsx"
SETTINGS_TSX = SRC / "pages" / "Settings.tsx"
CI_YML = REPO_ROOT / ".github" / "workflows" / "ai-platform-ci.yml"
PKG_JSON = ROOT / "web" / "package.json"
PRETTIER_RC = ROOT / "web" / ".prettierrc.json"

#: 迁移那张卡上两格判空各自的原话（逐字来自 `System.tsx`）
WARN_NO_SOURCE = "先填要迁移的那份服务端数据文件"
WARN_NO_TARGET = "先填迁移产物的文件名"

#: A323 的旧措辞（页面自己拼的第三处）。它一个字都不许再出现在代码面里。
OLD_TOAST = "配置已保存到 config.yaml"


def _system_code() -> str:
    return strip_comments(SYSTEM_TSX.read_text(encoding="utf-8"))


def _settings_code() -> str:
    return strip_comments(SETTINGS_TSX.read_text(encoding="utf-8"))


def _migrate_popconfirm(source: str) -> str:
    """抠出迁移那一处 `Popconfirm` 的源码块

    口径：`onConfirm={handleMigrate}` 是迁移那一处独有的标记（备份那一族走
    `handleCleanup` / `handleDelete`），所以从它往前找最近的 `<Popconfirm`，
    往后找配平的 `</Popconfirm>`。
    """
    marker = source.find("onConfirm={handleMigrate}")
    assert marker != -1, "System.tsx 里找不到迁移那一处 Popconfirm ⇒ 判据要跟着换形状"
    start = source.rfind("<Popconfirm", 0, marker)
    assert start != -1, "迁移那一处 Popconfirm 的开标签不在源码面里"
    end = source.find("</Popconfirm>", marker)
    assert end != -1, "迁移那一处 Popconfirm 的闭标签不在源码面里"
    return source[start : end + len("</Popconfirm>")]


class TestMigrateConfirmGateOrder:
    """A317：破坏性二次确认的授权语义不许被空表单消耗掉一次"""

    def test_migrate_popconfirm_is_controlled(self):
        block = _migrate_popconfirm(_system_code())
        assert "open={migrateConfirmOpen}" in block, (
            "迁移那一处二次确认必须受控打开（`open={migrateConfirmOpen}`）："
            "不受控就意味着「点按钮 ⇒ 先弹确认框」，判空没机会先出声"
        )
        assert "onOpenChange=" in block, (
            "迁移那一处二次确认必须有 `onOpenChange`：判空要挂在**打开**这一下，"
            "而不是只挂在 `onConfirm` 那一格"
        )

    def test_the_two_gates_are_checked_before_the_confirm_opens(self):
        source = _system_code()
        block = _migrate_popconfirm(source)
        # 判空函数本体：两格判空各一句 warning
        checker = re.search(
            r"const checkMigrateInput = \(\): boolean => \{(.*?)\n  \}", source, re.S)
        assert checker, "System.tsx 里找不到 `checkMigrateInput` ⇒ 判据要跟着换形状"
        body = checker.group(1)
        assert WARN_NO_SOURCE in body, f"判空少了一格：{WARN_NO_SOURCE}"
        assert WARN_NO_TARGET in body, f"判空少了一格：{WARN_NO_TARGET}"
        # 判空函数必须在 `onOpenChange` 那一格被调用
        opener = re.search(r"onOpenChange=\{\(next\) => \{(.*?)\n            \}\}", block, re.S)
        assert opener, "迁移那一处 `onOpenChange` 的形状变了 ⇒ 判据要跟着换形状"
        assert "checkMigrateInput()" in opener.group(1), (
            "判空必须在**打开确认框**这一下被调用；只挂在 `onConfirm` 那一格的话，"
            "用户会先读完一段破坏性警告、点一次「照改」，才得到那句提示"
        )

    def test_the_confirm_handler_still_guards_itself(self):
        """第二道防线：确认框是异步渲染的，字段可能在打开与确认之间被清空"""
        handler = re.search(
            r"const handleMigrate = async \(\) => \{(.*?)\n  \}", _system_code(), re.S)
        assert handler, "System.tsx 里找不到 `handleMigrate` ⇒ 判据要跟着换形状"
        assert "checkMigrateInput()" in handler.group(1), (
            "`handleMigrate` 自己也要过一遍判空（确认框打开后字段可能被清空）"
        )


class TestToastComesFromTheBackend:
    """A323：toast 必须读后端那句话，判决不许在 toast 这一格丢掉"""

    def test_toast_shows_the_backend_message(self):
        code = _settings_code()
        assert "message.success(result.message)" in code, (
            "保存配置的 toast 必须直接显示后端那句 `message`；"
            "页面自造措辞 ⇒ 后端拼进 message 的 `ignored_keys` 判决必然丢失"
        )

    def test_the_old_third_wording_is_gone(self):
        assert OLD_TOAST not in _settings_code(), (
            f"代码面里还留着旧那句自造措辞「{OLD_TOAST}」⇒ 页面上又出现第三处口径"
        )


class TestFormatGateExists:
    """A282：格式门要既在仓内可用、又在 CI 里真跑"""

    def test_ci_runs_the_format_gate(self):
        ci = CI_YML.read_text(encoding="utf-8")
        assert "npm run format:check" in ci, (
            "CI 的前端 job 必须有 `npm run format:check` 这一步："
            "这道门不在 ⇒ 格式漂移可以长期存在而不被任何人看见"
        )

    def test_the_script_and_the_config_both_exist(self):
        import json

        pkg = json.loads(PKG_JSON.read_text(encoding="utf-8"))
        assert "format:check" in pkg.get("scripts", {}), "package.json 里没有 format:check 脚本"
        assert PRETTIER_RC.exists(), "`.prettierrc.json` 不在 ⇒ 那道门是空转的"

    def test_the_repo_is_actually_prettier_clean(self):
        """仓内基线必须真的是绿的 —— 否则这道门一加进 CI 就把流水线打成红的"""
        import json

        pkg = json.loads(PKG_JSON.read_text(encoding="utf-8"))
        assert pkg["scripts"]["format:check"].strip() == "prettier --check src", (
            f"format:check 的形状变了：{pkg['scripts']['format:check']}"
        )


# ---------------------------------------------------------------------------
# 注入臂：每臂在内存里做字符串手术，**零写入**
# ---------------------------------------------------------------------------


def _injected_system(mutate) -> str:
    source = _system_code()
    return mutate(source)


class TestInjections:
    """注入取证：每一面都要仍会响（否则这把尺子是一把不会响的尺子）"""

    def test_a1_uncontrolled_popconfirm_turns_the_first_face_red(self):
        mutated = _injected_system(
            lambda s: s.replace("open={migrateConfirmOpen}", "open={false}", 1)
        )
        block = _migrate_popconfirm(mutated)
        assert "open={migrateConfirmOpen}" not in block, (
            "注入臂没生效：判据读不出「受控打开」这一格"
        )

    def test_a2_removing_the_gate_from_the_opener_turns_the_first_face_red(self):
        mutated = _injected_system(
            lambda s: s.replace(
                "if (next && !checkMigrateInput()) {", "if (false) {", 1
            )
        )
        opener = re.search(
            r"onOpenChange=\{\(next\) => \{(.*?)\n            \}\}",
            _migrate_popconfirm(mutated),
            re.S,
        )
        assert opener and "checkMigrateInput()" not in opener.group(1), (
            "注入臂没生效：判据读不出「打开确认框那一下没过判空」"
        )

    def test_a3_old_toast_wording_turns_the_second_face_red(self):
        mutated = _settings_code().replace(
            "message.success(result.message)", f"message.success('{OLD_TOAST}')", 1
        )
        assert OLD_TOAST in mutated, "注入臂没生效：判据读不出旧措辞"

    def test_a4_dropping_the_ci_step_turns_the_third_face_red(self):
        mutated = CI_YML.read_text(encoding="utf-8").replace("npm run format:check", "true", 1)
        assert "npm run format:check" not in mutated, "注入臂没生效：判据读不出 CI 里那一步"


def test_both_gate_messages_exist_in_the_page():
    """两格判空的原话逐字在页面上（文案改了要连判据一起改）

    **零 parametrize**：仓里那两把参数化普查尺（l97 / l98）把每一处 `parametrize`
    都记进名册，而本仓近几轮的惯例是**新守卫一律零 parametrize**（L316 那一支也是）
    ⇒ 这里逐格点名，不进那两张名册。
    """
    code = _system_code()
    assert WARN_NO_SOURCE in code, f"页面少了一句判空提示：{WARN_NO_SOURCE}"
    assert WARN_NO_TARGET in code, f"页面少了一句判空提示：{WARN_NO_TARGET}"
