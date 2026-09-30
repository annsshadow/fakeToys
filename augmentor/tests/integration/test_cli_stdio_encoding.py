# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI stdio 编码兜底守卫（L56，把 A98 的根因按住）

L55 换掉了两个会崩的装饰符，但根因一字未动：stdout 仍按 locale 编码，zh-CN Windows 的
管道就是 GBK ⇒ 用户数据里一个 emoji 能让整条命令崩成 rc=1，而 1 在 `docs/ARCHITECTURE.md`
§6 的口径里正是「判决未通过」那一档 —— **崩溃与判负同号，脚本读不出区别**。
L56 实测破坏面（Temp `l56/gbkblast_pre.txt`，GBK 管道 + 含 emoji 语料，16 条命令各跑一次）：
`check-leakage` / `stats` / `preview` / `search` 四条崩成 rc=1，前三条 stdout 0 字节。

修法只换错误处理器（`strict` → `replace`），不换编码 ⇒ 今天正常的输出逐字节不变
（Temp `l56/gbkbytes.txt`：10 条 A/B sha 相同、2 条自身连跑即变因而不可判定）。
本文件因此两头都钉：崩不许再吃掉退出码，且能被编码的字符一个都不许变。
"""

import io
import json
import os
import subprocess
import sys
from pathlib import Path

import pytest

from augmentor.cli.commands import COMMANDS
from augmentor.cli.io import harden_stdio

AI_DIR = Path(__file__).resolve().parent.parent.parent

# 落叶符号：在 GBK 里编不出来，也不在 CLI 任何 help / 装饰符里 ⇒ 只有用户数据会带它
EMOJI = "\U0001f342"

# L56 实测改前崩在这四条上（见模块 docstring 的取证文件）；argv 由 `emoji_corpus` 补齐
CRASHED_BEFORE_FIX = ("check-leakage", "preview", "search", "stats")


def _run_gbk(argv):
    """强制子进程 stdio 走 GBK 跑一次 CLI，返回 (退出码, 按 GBK 解码的 stdout, stderr)

    Args:
        argv: 子命令及其参数（不含程序名）

    Returns:
        (退出码, stdout 文本, stderr 文本)
    """
    env = dict(os.environ)
    env["PYTHONIOENCODING"] = "gbk"
    proc = subprocess.run([sys.executable, "cli.py"] + argv, cwd=str(AI_DIR),
                          capture_output=True, env=env)
    return (proc.returncode, proc.stdout.decode("gbk", "replace"),
            proc.stderr.decode("gbk", "replace"))


@pytest.fixture()
def emoji_corpus(tmp_path):
    """含 emoji 的三份语料 + 每条命令的 argv 形状"""
    items = [{"instruction": f"请写一首关于秋天的短诗 {EMOJI}，并注明作者。",
              "output": f"这是一首关于秋天的短诗 {EMOJI}，作者：某某。"},
             {"instruction": "Explain the difference between a list and a tuple.",
              "output": f"A list is mutable while a tuple is immutable. {EMOJI}"}]
    overlap = {"instruction": f"完全相同的一条训练样本用于制造泄漏 {EMOJI}",
               "output": f"同样的输出文本 {EMOJI * 3}"}
    paths = {}
    for name, payload in (("clean", items), ("train", [overlap] * 2), ("test", [overlap])):
        path = tmp_path / f"{name}.json"
        path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
        paths[name] = str(path)
    argvs = {
        "check-leakage": ["check-leakage", "--train", paths["train"], "--test", paths["test"]],
        "preview": ["preview", "--input", paths["clean"]],
        "search": ["search", "--input", paths["clean"], "--query", "秋天"],
        "stats": ["stats", "--input", paths["clean"]],
    }
    return argvs


class TestGbkPipeCannotEatTheExitCode:
    """数据侧的编码崩：命令必须返回它自己的码，而不是编码异常的 1"""

    def test_guarded_commands_are_real(self):
        unknown = [c for c in CRASHED_BEFORE_FIX if c not in COMMANDS]
        assert CRASHED_BEFORE_FIX, "守卫名单为空 ⇒ 本文件在空转"
        assert not unknown, f"名单里有已不存在的子命令: {unknown}"

    def test_gbk_pipe_would_still_crash_without_the_guard(self):
        # 反空转：证明「GBK 管道编不出 emoji」这件事在子进程里至今为真 —— 否则本文件
        # 全部通过只是因为语料或编码设置失效了
        proc = subprocess.run(
            [sys.executable, "-c",
             "import json;print(json.dumps({'t': %r}, ensure_ascii=False))" % EMOJI],
            cwd=str(AI_DIR), capture_output=True,
            env={**os.environ, "PYTHONIOENCODING": "gbk"})
        assert proc.returncode == 1, "GBK 管道不再因 emoji 崩 ⇒ 这条前提变了，守卫该重写了"
        assert "codec can't encode" in proc.stderr.decode("gbk", "replace")

    @pytest.mark.parametrize("cmd", CRASHED_BEFORE_FIX)
    def test_command_survives_emoji_data(self, cmd, emoji_corpus):
        rc, out, err = _run_gbk(emoji_corpus[cmd])
        assert rc == 0, f"{cmd} 在 GBK 管道 + emoji 语料下退出码为 {rc}（改前实测是 1）"
        assert "codec can't encode" not in err, f"{cmd} 仍崩在编码上: {err[:160]}"
        assert "错误:" not in err, f"{cmd} 的编码崩被 main() 翻译成了判决码那一档: {err[:160]}"
        assert out.strip(), f"{cmd} 什么都没印出来（改前三条是 0 字节）"

    def test_json_stdout_stays_machine_parseable(self, emoji_corpus):
        # 选 `replace` 而不是 `backslashreplace` 的理由就在这条：兜底字符必须是合法的
        # JSON 字符串内容，否则下游 `json.loads` 会读出一篇带着 `\U` 非法转义的废文档
        rc, out, _ = _run_gbk(emoji_corpus["preview"])
        assert rc == 0
        parsed = json.loads(out)
        assert "converted_data" in parsed
        assert "?" in out, "emoji 该被替换成一个 '?'，替换没发生说明语料或编码设置失效"


class TestHardenStdioIsTargeted:
    """兜底只许打在「今天本来编不出」的字符上，其它一律不许动"""

    class _NoReconfigure:
        """既非 TextIOWrapper 又自称 errors='strict' 的流（pytest 捕获与 redirect_stdout）"""

        errors = "strict"

        def write(self, text):
            return len(text)

    def _wrap(self, errors="strict"):
        buf = io.BytesIO()
        return buf, io.TextIOWrapper(buf, encoding="gbk", errors=errors, newline="")

    def test_strict_becomes_replace_on_both_streams(self, monkeypatch):
        buf, stream = self._wrap()
        monkeypatch.setattr(sys, "stdout", stream)
        monkeypatch.setattr(sys, "stderr", stream)
        harden_stdio()
        assert stream.errors == "replace"

    def test_encodable_text_keeps_its_exact_bytes(self, monkeypatch):
        buf, stream = self._wrap()
        monkeypatch.setattr(sys, "stdout", stream)
        monkeypatch.setattr(sys, "stderr", stream)
        harden_stdio()
        stream.write("总体状态: 未通过\n")
        stream.flush()
        assert buf.getvalue() == "总体状态: 未通过\n".encode("gbk")

    def test_unencodable_char_becomes_a_single_question_mark(self, monkeypatch):
        buf, stream = self._wrap()
        monkeypatch.setattr(sys, "stdout", stream)
        monkeypatch.setattr(sys, "stderr", stream)
        harden_stdio()
        stream.write(EMOJI)
        stream.flush()
        assert buf.getvalue() == b"?"

    def test_a_user_chosen_error_handler_is_left_alone(self, monkeypatch):
        buf, stream = self._wrap(errors="backslashreplace")
        monkeypatch.setattr(sys, "stdout", stream)
        monkeypatch.setattr(sys, "stderr", stream)
        harden_stdio()
        assert stream.errors == "backslashreplace", "已经有人定过策略，守卫不该覆写"

    def test_streams_without_reconfigure_do_not_raise(self, monkeypatch):
        fake = self._NoReconfigure()
        monkeypatch.setattr(sys, "stdout", fake)
        monkeypatch.setattr(sys, "stderr", fake)
        harden_stdio()
        assert fake.errors == "strict"
