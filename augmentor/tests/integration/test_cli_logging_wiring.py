# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 的 `logging` 节在真子进程里生效（L57，A97 的行为面）

单元面（`tests/unit/test_logging_wiring_a97.py`）能钉「装配函数本身的口径」，但钉不住
今天这个形状最要命的一条：**默认档一字不变**。理由是 `logging.lastResort` 只在 root
一条 handler 都没有时才兜底，而 pytest 自己就在 root 上挂着一条 —— 进程内测到的永远是
API 那一支。本文件全部**真起子进程**，判据按 `Temp/l57/ab.txt`（NONCE-L57-AB）实测的
口径写：

1. 「没写这节」与「写了节名没给键」与「给了空映射」三臂的 rc / stdout / stderr
   **逐字节相同**（ab.txt 判据 1：14 条命令不等名单为空）。
2. 写了 `level: ERROR` ⇒ 那条「键没人读取」的 WARNING 从 107 B / 1 行变 0 B / 0 行，
   而 stdout 与退出码一字不变（判据 4）。
3. 写了 `file` ⇒ 文件落盘**且控制台不少字**（判据 3：16 条命令「控制台被改动的命令:
   无」，同时 app.log 真长出来）。
4. 坏值在**加载期**出声拒收，不是等到第一条日志打出 `--- Logging error ---` + 31 行
   traceback（`Temp/l57/probe2.txt` P2/P3 是接线前的实测形状，接线后那就是用户的
   配置文件造成的）。

字节比较而不用文本：`PYTHONIOENCODING=utf-8` 只保证编码口径一致，逐字节等式还顺带钉住
行尾。`monitor` / `benchmark` 两条不进任何等式 —— 实测它们**同一入口连跑两次自身即变**
（快照 ID / timestamp，ab.txt 判据 1 的「不可判定名单」），按 L52 的纪律不做主张。
"""

import io
import os
import subprocess
import sys
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent

#: 三臂共用的底座：只有 `models` 一节，保证三份配置的内容差**只在 logging 节**，
#: 否则比的是配置内容而不是代码（Temp `l57/ab.py` 第一版就撞在这里）。
BASE = "models:\n  default: ernie\n  ernie:\n    type: baidu\n    model: x\n"

#: 一个拼错的键：这是本轮唯一稳定的、由产品面自己发出来的 WARNING（L54 的 A84 通道）
TYPO = "augmentation:\n  variants_per_seeds: 5\n"

#: A84 那条 WARNING 的判据词（`ConfigValidator.UNREAD_MARKER`）
UNREAD = "键没人读取"

def _argv(corpus):
    """`validate` 的 argv —— 挑它是因为它**不碰模型、不联网、不写盘**，而 stderr 上
    只有产品面自己发出的那一条 WARNING，是最干净的观察窗。"""
    return ["validate", "--input", corpus]


@pytest.fixture()
def corpus(tmp_path):
    """一份离线可跑的小语料（`validate` 不需要模型，也不需要网络）"""
    path = tmp_path / "data.json"
    path.write_text(
        '[{"instruction": "写一首关于秋天的短诗。", "output": "一叶落知天下秋。"},'
        ' {"instruction": "Explain list vs tuple.", "output": "Mutable vs immutable."}]',
        encoding="utf-8", newline="\n")
    return str(path)


def _config(tmp_path, name, body):
    path = tmp_path / name
    path.write_text(body, encoding="utf-8", newline="\n")
    return str(path)


def _run(tmp_path, config_path, argv):
    """在 `tmp_path` 里跑一次真 CLI，返回 `(rc, stdout 字节, stderr 字节)`

    `--config` 是**全局旗标**，必须排在子命令之前（实测排后面是 `unrecognized
    arguments`）。cwd 设成 tmp_path 是让 `logging.file` 的相对路径有一个**已知**的
    落点，而不是仓库目录。
    """
    env = dict(os.environ, PYTHONIOENCODING="utf-8")
    proc = subprocess.run([sys.executable, str(AI_DIR / "cli.py"),
                           "--config", config_path] + list(argv),
                          cwd=str(tmp_path), capture_output=True, env=env, timeout=180)
    return proc.returncode, proc.stdout, proc.stderr


class TestDefaultShapeIsUnchanged:
    """「没要求」的三种写法必须逐字节等于今天"""

    @pytest.mark.parametrize("body", [
        "",                                        # 整节都不写
        "logging:\n",                              # 写了节名，值是 YAML 的 null
        "logging: {}\n",                           # 写了节名，值是空映射
    ])
    def test_three_ways_of_not_asking_are_identical(self, tmp_path, corpus, body):
        ref = _run(tmp_path, _config(tmp_path, "a.yaml", BASE + TYPO), _argv(corpus))
        got = _run(tmp_path, _config(tmp_path, "b.yaml", BASE + TYPO + body),
                   _argv(corpus))
        assert got[0] == ref[0], "退出码被一个「什么都没写」的节改了"
        assert got[1] == ref[1], "stdout 被一个「什么都没写」的节改了"
        assert got[2] == ref[2], (
            f"stderr 被一个「什么都没写」的节改了：{ref[2]!r} → {got[2]!r}")

    def test_the_baseline_actually_says_the_warning(self, tmp_path, corpus):
        """上一条是三臂互比，这条钉住参照物的**内容** —— 否则三臂全空也是「相同」"""
        rc, out, err = _run(tmp_path, _config(tmp_path, "a.yaml", BASE + TYPO),
                            _argv(corpus))
        assert rc == 0
        lines = err.splitlines()
        assert len(lines) == 1, f"默认档应当只由 lastResort 打一行裸消息，实得 {err!r}"
        assert UNREAD in lines[0].decode("utf-8")
        # lastResort 的格式是 `%(message)s`：级别名与时间戳都不该出现
        assert b"WARNING" not in err and b" - " not in err, (
            f"默认档的消息被加了前缀，说明装配在无要求时动了 format：{err!r}")


class TestWrittenKeysTakeEffect:
    """写了就必须生效，而且只生效写出的那一键"""

    def test_level_error_mutes_the_warning_and_nothing_else(self, tmp_path, corpus):
        base_cfg = _config(tmp_path, "a.yaml", BASE + TYPO)
        err_cfg = _config(tmp_path, "b.yaml", BASE + TYPO + "logging:\n  level: ERROR\n")
        ref = _run(tmp_path, base_cfg, _argv(corpus))
        got = _run(tmp_path, err_cfg, _argv(corpus))
        assert got[0] == ref[0] and got[1] == ref[1], (
            "静音不该改判决：rc/stdout 变了就是越过了 `level` 这一键的范围")
        assert got[2] == b"", f"`level: ERROR` 没静音成功：{got[2]!r}"

    def test_format_key_reaches_the_console_line(self, tmp_path, corpus):
        cfg = _config(tmp_path, "f.yaml", BASE + TYPO
                      + 'logging:\n  level: WARNING\n  format: "MARK|%(levelname)s|%(message)s"\n')
        rc, _, err = _run(tmp_path, cfg, _argv(corpus))
        assert rc == 0
        assert err.startswith(b"MARK|WARNING|"), (
            f"写出的 `format` 没到控制台：{err!r}")
        assert UNREAD in err.decode("utf-8"), "换格式把消息本身换掉了"

    def test_file_key_lands_in_the_file_and_keeps_the_console(self, tmp_path, corpus):
        """两条出口都得有字：root 一有了 handler，`lastResort` 就不再兜底，不自己补
        一条 stderr handler 的话 CLI 的输出会当场变空（实测这就是「只写 file 一键」
        最容易踩的坑）。"""
        cfg = _config(tmp_path, "fl.yaml", BASE + TYPO
                      + "logging:\n  level: WARNING\n  file: run.log\n")
        rc, _, err = _run(tmp_path, cfg, _argv(corpus))
        assert rc == 0
        assert UNREAD in err.decode("utf-8"), f"控制台那条被 file 键吃掉了：{err!r}"
        log = tmp_path / "run.log"
        assert log.exists(), "写了 `logging.file` 却没有落盘文件 —— 旋钮仍然没人接"
        assert UNREAD in log.read_text(encoding="utf-8")

    def test_only_the_written_key_moves(self, tmp_path, corpus):
        """只写 `file` ⇒ 格式仍是今天那个裸消息：没写的键不许被默认值顺手覆盖"""
        cfg = _config(tmp_path, "one.yaml", BASE + TYPO + "logging:\n  file: only.log\n")
        rc, _, err = _run(tmp_path, cfg, _argv(corpus))
        assert rc == 0
        assert err.startswith(UNREAD.encode("utf-8")), (
            f"只写 file 一键却动了 format/rc/级别：{err!r}")


class TestBadValuesAreRefusedAtLoad:
    """坏值判在加载期，不判在第一条日志上"""

    @pytest.mark.parametrize("body, needle", [
        ("  level: warning\n", "大写"),
        ("  level: 5\n", "logging.level"),
        ('  format: "%(nope)s"\n', "logging.format"),
        ("  format: \"\"\n", "logging.format"),
        ("  file: nope_dir/x.log\n", "当前工作目录"),
    ])
    def test_the_message_names_the_key_and_the_shape(self, tmp_path, corpus, body, needle):
        cfg = _config(tmp_path, "bad.yaml", BASE + TYPO + "logging:\n" + body)
        rc, _, err = _run(tmp_path, cfg, _argv(corpus))
        text = err.decode("utf-8", "replace")
        assert rc == 1, f"坏值没被拒收（rc={rc}）：{text!r}"
        assert needle in text, f"报错没指向可行动的那件事（要找 {needle!r}）：{text!r}"

    def test_a_bad_format_does_not_become_a_logging_error_traceback(self, tmp_path, corpus):
        """接线前 `Formatter(\"%(nope)s\")` 是能装上的，代价在**每条日志**上：31 行
        `--- Logging error ---`。这条钉的是「判在加载」这个修法本身。"""
        cfg = _config(tmp_path, "bad2.yaml", BASE + TYPO
                      + 'logging:\n  level: WARNING\n  format: "%(nope)s"\n')
        rc, _, err = _run(tmp_path, cfg, _argv(corpus))
        assert b"Logging error" not in err, f"格式串错误仍然推迟到发日志时才炸：{err!r}"
        assert b"Traceback" not in err, "给用户的是原文 traceback 而不是一行可行动的错"
        assert rc == 1

    @pytest.mark.parametrize("body", [
        "logging:\n  level: INFO\n",
        'logging:\n  level: WARNING\n  file: ""\n  format: "%(message)s"\n',
        "logging:\n  level: CRITICAL\n",
    ])
    def test_the_validator_says_the_same_verdict_as_the_runtime(self, tmp_path, corpus, body):
        """合法值两侧都绿（承 A77 的口径：校验器不许比运行时严）

        两侧都是 `exit 0`：拼错的 `variants_per_seeds` 只出 WARNING、不改判决，所以这
        里比的是「合法 logging 节会不会让某一侧红」。单边红就是 A77 那两种症状之一
        ——「validate-config 报红的配置其实跑得起来」或「绿灯跑进流水线才炸」。
        """
        cfg = _config(tmp_path, "ok.yaml", BASE + TYPO + body)
        run_rc, _, run_err = _run(tmp_path, cfg, _argv(corpus))
        val_rc, val_out, val_err = _run(tmp_path, cfg, ["validate-config"])
        assert run_rc == 0, f"运行时拒收了合法 logging 节：{run_err!r}"
        assert val_rc == 0, f"校验器独有地拒收了合法值（A77 镜像症状）：{val_out!r}"
        assert b"logging" not in run_err.lower(), (
            f"合法值在产品面出了错：{run_err!r}")
        assert b"logging" not in val_out.lower(), (
            f"合法值在诊断面出了错：{val_out!r}")
