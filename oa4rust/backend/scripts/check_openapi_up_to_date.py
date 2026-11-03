#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
# -*- coding: utf-8 -*-
"""
check_openapi_up_to_date.py — OpenAPI 生成物漂移守卫。

判据（真实判定，不是存在性检查）：
  1. 快照已提交的生成物（crates/openapi/src/lib.rs、request_types.rs）；
  2. 跑 scripts/gen_openapi_paths.py 就地重生成；
  3. 逐字比对（仅归一化行尾 CRLF/LF，其余字节严格相等）；
  4. 无论结果如何都把快照写回原文件——本脚本不改仓库工作树。

退出码：
  0  生成物与路由源码一致；
  1  生成物落后于路由源码（regen diff 非空）；
  2  生成器本身执行失败（看 stderr，不是漂移问题）。

用法：
  python scripts/check_openapi_up_to_date.py
"""

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GEN = ROOT / "scripts" / "gen_openapi_paths.py"
TARGETS = [
    ROOT / "crates" / "openapi" / "src" / "lib.rs",
    ROOT / "crates" / "openapi" / "src" / "request_types.rs",
]


def normalized(data: bytes) -> bytes:
    return data.replace(b"\r\n", b"\n")


def main() -> int:
    snapshots = {}
    missing = []
    for path in TARGETS:
        if not path.exists():
            missing.append(path)
        else:
            snapshots[path] = path.read_bytes()
    if missing:
        for path in missing:
            print(f"生成物缺失：{path.relative_to(ROOT).as_posix()}")
        print("修复：cd oa4rust/backend && python scripts/gen_openapi_paths.py")
        return 1

    try:
        proc = subprocess.run(
            [sys.executable, str(GEN)],
            capture_output=True,
            text=True,
            cwd=str(GEN.parent),
        )
        if proc.returncode != 0:
            print("gen_openapi_paths.py 执行失败（退出码 %d）" % proc.returncode)
            if proc.stdout:
                print(proc.stdout, end="")
            if proc.stderr:
                print(proc.stderr, end="")
            return 2

        stale = []
        for path, before in snapshots.items():
            if normalized(path.read_bytes()) != normalized(before):
                stale.append(path.relative_to(ROOT).as_posix())

        if stale:
            print("生成物落后于路由源码：" + ", ".join(stale))
            print("修复：cd oa4rust/backend && python scripts/gen_openapi_paths.py（并提交生成物）")
            return 1

        print("openapi-paths: 生成物与路由源码一致（%d 个文件）" % len(TARGETS))
        return 0
    finally:
        # 检查本身绝不留痕：把重生成前的字节写回（含生成器中途失败的情况）。
        for path, before in snapshots.items():
            if path.read_bytes() != before:
                path.write_bytes(before)


if __name__ == "__main__":
    sys.exit(main())
