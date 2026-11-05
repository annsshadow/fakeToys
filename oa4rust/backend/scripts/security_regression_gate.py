#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""O26 安全回归门禁 runner。

用法（在 oa4rust/backend 下，或任意目录用绝对路径）：

    python scripts/security_regression_gate.py            # 跑门禁（棘轮比对）
    python scripts/security_regression_gate.py baseline   # 重写基线（修复/确认后）
    python scripts/security_regression_gate.py sabotage ownership  # 破坏实验①：摘掉已修的归属条件
    python scripts/security_regression_gate.py sabotage correct    # 破坏实验②：弄坏一条本来正确的端点
    python scripts/security_regression_gate.py sabotage organization # 破坏实验③：摘掉 org 的归属谓词
    python scripts/security_regression_gate.py sabotage program_center # 破坏实验④：摘掉 collect/module 的归属谓词

环境变量：
    DATABASE_URL   Postgres 连接串（默认 postgres://o2server:password@localhost:5432/oa4rust_test）
                   门禁会自己建 `oa_par_gate` scratch 库、迁移、灌数，跑完 DROP；
                   不需要预先存在任何测试库。

退出码：0 = 绿 / 破坏实验确认抓到；1 = 门禁红 / 破坏实验未抓到。
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path

BACKEND = Path(__file__).resolve().parent.parent
TEST = "security_regression"

ATTENDANCE = BACKEND / "crates/attendance_core_entity/src/lib.rs"
CMS = BACKEND / "crates/cms_assemble_control/src/lib.rs"
GENERAL = BACKEND / "crates/general_core_entity/src/lib.rs"
ORGANIZATION = BACKEND / "crates/organization_core_entity/src/lib.rs"
PROGRAM_CENTER = BACKEND / "crates/program_center/src/lib.rs"

ATTENDANCE_EXPECT = "attendance_core_entity|record_update|POST"
CMS_EXPECT = "gate-cms-readable"
GENERAL_EXPECT = "general_core_entity|file_delete|POST"
ORGANIZATION_EXPECT = "organization_core_entity|definition_update|PUT"
PROGRAM_CENTER_EXPECT = "program_center|collect_delete|DELETE"

OWNERSHIP_RE = re.compile(
    r'const RECORD_OWNER_PREDICATE: &str = .*?false\)\)";', re.S
)
OWNERSHIP_PATCH = 'const RECORD_OWNER_PREDICATE: &str = "id = $1 AND $2 IS NOT NULL";'

# file/invoice 的归属谓词（本轮写 IDOR 第一批）：摘掉 creator/admin 判定
GENERAL_OWNERSHIP_RE = re.compile(
    r'const CREATOR_OWNER_PREDICATE: &str = .*?false\)\)";', re.S
)
GENERAL_OWNERSHIP_PATCH = (
    'const CREATOR_OWNER_PREDICATE: &str = "id = $1 AND $2 IS NOT NULL";'
)

# org definition/identity 的归属谓词（本轮写 IDOR 第二批）：摘掉 creator/admin 判定。
# 复用上面的正则形状，但作用在 organization crate 的 lib.rs 上。
ORGANIZATION_OWNERSHIP_RE = GENERAL_OWNERSHIP_RE
ORGANIZATION_OWNERSHIP_PATCH = (
    'const CREATOR_OWNER_PREDICATE: &str = "id = $1 AND $2 IS NOT NULL";'
)

# program_center 的 collect/module 归属谓词（本轮写 IDOR 第三批）。两个常量相邻，
# 一次匹配整段替换成恒真，摘掉归属判定。
PROGRAM_CENTER_OWNERSHIP_RE = re.compile(
    r'const CREATOR_OWNER_PREDICATE: &str = .*?false\)";\s*'
    r'/// .*?\n'
    r'const PERSON_OWNER_PREDICATE: &str = .*?false\)";',
    re.S,
)
PROGRAM_CENTER_OWNERSHIP_PATCH = (
    'const CREATOR_OWNER_PREDICATE: &str = "TRUE";\n'
    'const PERSON_OWNER_PREDICATE: &str = "TRUE";'
)

# document_permission_read_u3 的 readable 计算（不可读即 403）
CMS_READABLE_RE = re.compile(
    r"    let readable = admin\n"
    r".*?"
    r'== "published";\n',
    re.S,
)
CMS_READABLE_PATCH = (
    '    let readable = admin || row.get::<_, Option<String>>("creator").is_some();\n'
)


def cargo_test(update_baseline: bool = False) -> tuple[int, str]:
    env = os.environ.copy()
    if update_baseline:
        env["SECURITY_GATE_UPDATE_BASELINE"] = "1"
    else:
        env.pop("SECURITY_GATE_UPDATE_BASELINE", None)
    p = subprocess.run(
        ["cargo", "test", "--test", TEST, "--", "--nocapture"],
        cwd=str(BACKEND),
        env=env,
        capture_output=True,
    )
    out = (p.stdout + p.stderr).decode("utf-8", "replace")
    return p.returncode, out


def run(update_baseline: bool = False) -> int:
    code, out = cargo_test(update_baseline)
    # 全量输出：148 条逐条 PASS/LEAK/UNVERIFIED，FAIL 行可直接定位端点与身份
    print(out)
    return code


def mutate(path: Path, pattern: re.Pattern[str], patch: str, label: str) -> bytes:
    # 用原始字节还原，避免 read_text 归一化 CRLF 后把整份业务文件“改脏”
    original = path.read_bytes()
    text = original.decode("utf-8")
    new, n = pattern.subn(patch, text, count=1)
    if n != 1:
        raise SystemExit(f"[{label}] 未命中改造点（模式匹配 {n} 次），业务代码可能已变动，请先更新脚本")
    path.write_bytes(new.encode("utf-8"))
    return original


def sabotage(kind: str) -> int:
    if kind == "ownership":
        path, pattern, patch, expect = ATTENDANCE, OWNERSHIP_RE, OWNERSHIP_PATCH, ATTENDANCE_EXPECT
        title = "摘掉考勤记录的归属谓词（record_update 原本已修，应变 NEW-FAIL）"
    elif kind == "correct":
        path, pattern, patch, expect = CMS, CMS_READABLE_RE, CMS_READABLE_PATCH, CMS_EXPECT
        title = "把不可读即 403 的文档读权限端点改成恒 true（应变 NEW-FAIL）"
    elif kind == "general":
        path, pattern, patch, expect = (
            GENERAL,
            GENERAL_OWNERSHIP_RE,
            GENERAL_OWNERSHIP_PATCH,
            GENERAL_EXPECT,
        )
        title = "摘掉 file/invoice 的 creator/管理员归属谓词（应变 NEW-FAIL）"
    elif kind == "organization":
        path, pattern, patch, expect = (
            ORGANIZATION,
            ORGANIZATION_OWNERSHIP_RE,
            ORGANIZATION_OWNERSHIP_PATCH,
            ORGANIZATION_EXPECT,
        )
        title = "摘掉 org definition/identity 的 creator/管理员归属谓词"
    elif kind == "program_center":
        path, pattern, patch, expect = (
            PROGRAM_CENTER,
            PROGRAM_CENTER_OWNERSHIP_RE,
            PROGRAM_CENTER_OWNERSHIP_PATCH,
            PROGRAM_CENTER_EXPECT,
        )
        title = "摘掉 program_center collect/module 的归属谓词（4 条应变 NEW-FAIL）"
    else:
        raise SystemExit(
            f"未知破坏实验: {kind}（可选 ownership / correct / general / organization / program_center）"
        )

    print(f"== 破坏实验：{title}")
    original = mutate(path, pattern, patch, kind)
    try:
        code, out = cargo_test()
        caught = (
            code != 0
            and "NEW-FAIL" in out
            and expect in out
        )
        # 只摘出关键证据行，避免把 148 条明细全部倒进终端
        for line in out.splitlines():
            if line.startswith(("[LEAK]", "[PASS]", "[UNVERIFIED]", "NEW-FAIL", "STALE-BASELINE")) \
                    or line.startswith("=== ") or "非 PASS" in line or "test result:" in line:
                print(line)
        print()
        if caught:
            print(f"实验成立：门禁变红（exit={code}），并点名 {expect}")
            return 0
        print(f"实验失败：门禁未红或未点名 {expect}（exit={code}）")
        return 1
    finally:
        path.write_bytes(original)
        print(f"[restore] {path.name} 已还原")


def main() -> int:
    cmd = sys.argv[1] if len(sys.argv) > 1 else "run"
    if cmd == "run":
        return run()
    if cmd == "baseline":
        return run(update_baseline=True)
    if cmd == "sabotage":
        if len(sys.argv) < 3:
            print("用法: security_regression_gate.py sabotage <ownership|correct>")
            return 2
        return sabotage(sys.argv[2])
    print(__doc__)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
