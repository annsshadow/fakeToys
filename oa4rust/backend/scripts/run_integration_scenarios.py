#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""CI 集成场景运行器：真跑 `#[ignore]` 用例，只允许清单内一条显式豁免。

为什么不直接 `cargo test -- --ignored`：
`integration_runner` 的全部场景都标了 `#[ignore]`，此前 CI 的
`cargo test --test integration_runner` 一条也没跑。这里把「跑」固化下来，
同时把豁免锁成一件事：

  1. 豁免清单必须是可读文件（tests/integration_skip_list.txt），不是命令行参数；
  2. 条目数必须恰好 1（多塞一条必须改本脚本，留下 diff）；
  3. 每个条目必须真实存在于 `--list` 的测试集合里（点错名字＝没排除 → 报错）。

退出码：0 全绿；1 守卫失败；其余为 cargo test 的退出码。
"""

from __future__ import annotations

import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
SKIP_LIST = ROOT / "tests" / "integration_skip_list.txt"
EXPECTED_SKIP_COUNT = 1

TEST_NAME_RE = re.compile(r"^(?P<name>[A-Za-z0-9_:]+):\s+test$")


def read_skip_list() -> list[str]:
    entries: list[str] = []
    for raw in SKIP_LIST.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        entries.append(line)
    return entries


def list_test_names() -> set[str]:
    proc = subprocess.run(
        ["cargo", "test", "--test", "integration_runner", "--", "--ignored", "--list"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        print("无法列出 integration_runner 的测试集合：", file=sys.stderr)
        print(proc.stdout, file=sys.stderr)
        print(proc.stderr, file=sys.stderr)
        sys.exit(1)

    names: set[str] = set()
    for line in proc.stdout.splitlines():
        m = TEST_NAME_RE.match(line.strip())
        if m:
            names.add(m.group("name"))
    if not names:
        print("--list 输出里没有解析到任何测试名（格式可能变了）", file=sys.stderr)
        sys.exit(1)
    return names


def main() -> int:
    entries = read_skip_list()

    if len(entries) != EXPECTED_SKIP_COUNT:
        print(
            f"守卫失败：豁免清单必须恰好 {EXPECTED_SKIP_COUNT} 条，当前 {len(entries)} 条。"
            " 新增豁免请同时修改本脚本的 EXPECTED_SKIP_COUNT 并说明理由。",
            file=sys.stderr,
        )
        return 1

    known = list_test_names()
    missing = [e for e in entries if e not in known]
    if missing:
        print(
            "守卫失败：以下豁免条目在测试集合中不存在（点错名字＝没有真正排除）："
            + ", ".join(missing),
            file=sys.stderr,
        )
        return 1

    cmd = [
        "cargo",
        "test",
        "--test",
        "integration_runner",
        "--",
        "--ignored",
        "--test-threads=1",
    ]
    for entry in entries:
        cmd.extend(["--skip", entry])

    print("运行：", " ".join(cmd), flush=True)
    return subprocess.run(cmd, cwd=ROOT, check=False).returncode


if __name__ == "__main__":
    sys.exit(main())
