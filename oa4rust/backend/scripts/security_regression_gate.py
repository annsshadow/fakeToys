#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""O26 安全回归门禁 runner（Rust 与 Go 各一条独立入口）。

用法（在 oa4rust/backend 下，或任意目录用绝对路径）：

    # Rust 侧（184 条；独立入口，不碰 Go 的库）
    python scripts/security_regression_gate.py            # 跑 Rust 门禁（棘轮比对）
    python scripts/security_regression_gate.py baseline   # 重写 Rust 基线

    # Go 侧（21 条；独立入口，只建 Go 的 scratch 库）
    python scripts/security_regression_gate.py go           # 跑 Go 门禁
    python scripts/security_regression_gate.py go-baseline  # 重写 Go 基线

    # 破坏实验
    python scripts/security_regression_gate.py sabotage ownership   # Rust①：摘掉已修归属条件
    python scripts/security_regression_gate.py sabotage correct     # Rust②：弄坏正确端点
    python scripts/security_regression_gate.py sabotage organization    # Rust②b：摘掉 org 归属谓词
    python scripts/security_regression_gate.py sabotage program_center  # Rust②c：摘掉 collect/module 归属谓词
    python scripts/security_regression_gate.py sabotage hotpic_mind      # Rust②d：摘掉 hotpic/mind 归属谓词（4 条）
    python scripts/security_regression_gate.py sabotage write-idor-b5     # Rust②e：摘掉 folder_delete + cms 三条的归属（4 条）
    python scripts/security_regression_gate.py sabotage write-idor-b6     # Rust②f：摘掉 appeal + processplatform 两把的归属（5 条）
    python scripts/security_regression_gate.py sabotage write-idor-b7     # Rust②g：摘掉第七批读归属谓词（5 条）
    python scripts/security_regression_gate.py sabotage write-idor-w4     # Rust②j：摘掉 program_center 8 个读端点归属谓词
    python scripts/security_regression_gate.py sabotage write-idor-w5     # Rust②k：摘掉 query designer 5 个读端点归属校验
    python scripts/security_regression_gate.py sabotage write-idor-w6     # Rust②l：摘掉 query view/surface 8 个读端点归属校验
    python scripts/security_regression_gate.py sabotage read-a-b1         # Rust②h：摘掉分片 A 第一批 cms 读门（6 条）
    python scripts/security_regression_gate.py sabotage write-idor-w3     # Rust②i：摘掉 office/bbs/mind 写侧冒名 403（7 条）
    python scripts/security_regression_gate.py sabotage go-limit0   # Go①：扣款乘 2
    python scripts/security_regression_gate.py sabotage go-idor-rev # Go②：摘掉预设删除归属
    python scripts/security_regression_gate.py sabotage go-empty-list    # Go③：伪装成已修（列表恒空）
    python scripts/security_regression_gate.py sabotage go-swallow-write # Go④：吞掉写失败（2xx 但未落库）
    python scripts/security_regression_gate.py sabotage go-dead-write     # Go④b：写路径对所有人失效
    python scripts/security_regression_gate.py sabotage go-page2         # Go⑤：归属过滤只作用于第一页
    python scripts/security_regression_gate.py sabotage go-drift-to-leak # Go⑥：基线内 UNVERIFIED 恶化成 LEAK
    python scripts/security_regression_gate.py sabotage rust-drift-to-leak  # Rust③：基线内 UNVERIFIED 恶化成 LEAK
    python scripts/security_regression_gate.py sabotage rust-swallow-write  # Rust④：本人写 2xx 但未落库

刻意**不**把 Rust 与 Go 塞进同一个 `run`：两侧各自建库、各自迁移，
合并入口会让修复线每次多等另一侧的建库时间。

环境变量：
    DATABASE_URL        Rust 侧 Postgres 连接串
                        （默认 postgres://o2server:password@localhost:5432/oa4rust_test）
    TEST_DATABASE_URL   Go 侧 Postgres 连接串；未设置时默认
                        postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable
                        两侧都会自建 scratch 库、迁移、灌数，跑完 DROP。

退出码：0 = 绿 / 破坏实验确认抓到；1 = 门禁红 / 破坏实验未抓到。
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path

BACKEND = Path(__file__).resolve().parent.parent
REPO = BACKEND.parent.parent
TEST = "security_regression"

# --- Go 侧（与 Rust 完全独立） ---
GO_SERVER = REPO / "laiyipao/server"
GO_PKG = "./internal/httpapi"
GO_RUN = "TestSecurityGate"
GO_DSN_DEFAULT = "postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
GO_ECONOMY = GO_SERVER / "internal/service/economy.go"
GO_PRESETS = GO_SERVER / "internal/service/presets.go"
GO_BASELINE = GO_SERVER / "internal/httpapi/security_gate_baseline.txt"

ATTENDANCE = BACKEND / "crates/attendance_core_entity/src/lib.rs"
CMS = BACKEND / "crates/cms_assemble_control/src/lib.rs"
GENERAL = BACKEND / "crates/general_core_entity/src/lib.rs"
ORGANIZATION = BACKEND / "crates/organization_core_entity/src/lib.rs"
PROGRAM_CENTER = BACKEND / "crates/program_center/src/lib.rs"
HOTPIC_CORE = BACKEND / "crates/hotpic_core_entity/src/lib.rs"
HOTPIC_ASSEMBLE = BACKEND / "crates/hotpic_assemble_control/src/lib.rs"
MIND = BACKEND / "crates/mind_core_entity/src/lib.rs"
FILE_CORE = BACKEND / "crates/file_core_entity/src/lib.rs"
PROCESS_PLATFORM = BACKEND / "crates/processplatform_service_processing/src/lib.rs"
ORG_ASSEMBLE_CONTROL_HELPERS = BACKEND / "crates/organization_assemble_control/src/u2_helpers.rs"
ORG_ASSEMBLE_AUTH = BACKEND / "crates/organization_assemble_authentication/src/lib.rs"
MESSAGE_COMMUNICATE = BACKEND / "crates/message_assemble_communicate/src/lib.rs"
# 写 IDOR 第 W-b5 批：query designer 读端点归属校验（creator OR admin）
QUERY_DESIGNER = BACKEND / "crates/query_assemble_designer/src/lib.rs"

# 第七批写 IDOR：读面归属谓词所在的 crate
COMPONENT = BACKEND / "crates/component_assemble_control/src/lib.rs"
CORRELATION = BACKEND / "crates/correlation_service_processing/src/lib.rs"
JPUSH = BACKEND / "crates/jpush/src/lib.rs"
JPUSH_ASSEMBLE = BACKEND / "crates/jpush_assemble_control/src/lib.rs"
JPUSH_CORE = BACKEND / "crates/jpush_core_entity/src/lib.rs"

# 第十一轮：keyset 深分页与本人写正控制所在的 crate
ATTENDANCE_ASSEMBLE = BACKEND / "crates/attendance_assemble_control/src/lib.rs"
RUST_BASELINE = BACKEND / "tests/security_regression_baseline.txt"

ATTENDANCE_EXPECT = "attendance_core_entity|record_update|POST"
CMS_EXPECT = "gate-cms-readable"
GENERAL_EXPECT = "general_core_entity|file_delete|POST"
ORGANIZATION_EXPECT = "organization_core_entity|definition_update|PUT"
READ_B_EXPECTS = [
    "organization_assemble_control|identity_id",
    "organization_assemble_control|identity_flag",
    "organization_assemble_control|unit_flag",
    "organization_assemble_authentication|identity_id",
    "message_assemble_communicate|mass_id",
]
PROGRAM_CENTER_EXPECT = "program_center|collect_delete|DELETE"
HOTPIC_MIND_EXPECTS = [
    "hotpic_core_entity|delete_by_id|DELETE",
    "hotpic_assemble_control|user_hotpic_delete_by_ids|DELETE",
    "mind_core_entity|update_mind|POST",
    "mind_core_entity|update_folder|POST",
]

# 分页用例里，next handler 的归属谓词（只匹配第一处，不动 prev）
PAGE_SCOPE_RE = re.compile(r"AND \(\$3::text IS NULL OR person_id = \$3::text\)")
PAGE_SCOPE_PATCH = "AND ($3::text IS NULL OR TRUE)"
PAGE_DRIFT_EXPECT = "page-attendance-detail-next"

# 本人写正控制：把真实 INSERT 换成消费同样 5 个参数的 no-op，接口仍 2xx 但不落库。
# 不能提前 return（-D warnings 会把后续代码判成 unreachable / unused）。
SELF_WRITE_RE = re.compile(
    r'"INSERT INTO x_attendance_selfholiday .*?VALUES \(\$1, \$2, \$3, \$4, \$5, NOW\(\), NOW\(\)\)"',
    re.S,
)
SELF_WRITE_PATCH = (
    '"SELECT $1::text, $2::text, $3::text, $4::text, $5::text WHERE FALSE"'
)
SELF_WRITE_EXPECT = "self-write-roundtrip"

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

# program_center 的 collect/module/dict/log 归属谓词（本轮写 IDOR 第三批）。
# 三个常量相邻，一次匹配整段替换成恒真（保留 $2 绑定），摘掉归属判定。
PROGRAM_CENTER_OWNERSHIP_RE = re.compile(
    r'const CREATOR_OWNER_PREDICATE: &str = .*?false\)";\s*'
    r'/// .*?\n'
    r'const CREATOR_PERSON_OWNER_PREDICATE: &str = .*?false\)";\s*'
    r'/// .*?\n'
    r'const PERSON_OWNER_PREDICATE: &str = .*?false\)";',
    re.S,
)
PROGRAM_CENTER_OWNERSHIP_PATCH = (
    'const CREATOR_OWNER_PREDICATE: &str = "($2::text IS NOT NULL)";\n'
    'const CREATOR_PERSON_OWNER_PREDICATE: &str = "($2::text IS NOT NULL)";\n'
    'const PERSON_OWNER_PREDICATE: &str = "($2::text IS NOT NULL)";'
)

# 写 IDOR 第 W-b4 批：program_center 8 个读端点的归属谓词。
# 同一次整段替换，恒真式保留 $2 绑定，避免 deadpool 端点 500 造成假阳性。
W4_EXPECTS = [
    "program_center|application_get_by_id|GET",
    "program_center|collect_code_mobile_mobile|GET",
    "program_center|deploy_id|GET",
    "program_center|dict_id|GET",
    "program_center|module_id_compare|GET",
    "program_center|module_output_flag_file|GET",
    "program_center|prompterrorlog_id|GET",
    "program_center|unexpectederrorlog_id|GET",
]

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


def _git(*args: str, timeout: int = 30) -> subprocess.CompletedProcess:
    return subprocess.run(
        ["git", *args],
        cwd=str(REPO),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        timeout=timeout,
    )


def git_preflight() -> int:
    """门禁纪律：读数必须基于与 origin/main 同步的工作树。

    behind > 0 时直接拒绝：旧代码上的读数会让基线棘轮判错
    （修了却 STALE、没修却以为修了）。

    CI 里 checkout 的是 PR 合并引用，与 origin/main 算 ahead/behind 没有意义，
    直接跳过；CI 的正确性由「必须先合并 main + required check 全绿」保证。
    """
    if os.environ.get("GITHUB_ACTIONS") == "true":
        print("[preflight] skipped (CI)")
        return 0
    try:
        fetch = _git("fetch", "--quiet", "origin", "main", timeout=10)
        if fetch.returncode != 0:
            print("[preflight] 警告：无法 fetch origin/main，使用本地引用（可能已过期）")
    except subprocess.TimeoutExpired:
        print("[preflight] 警告：fetch origin/main 超时，使用本地引用（可能已过期）")
    head = _git("rev-parse", "HEAD").stdout.strip() or "unknown"
    branch = _git("rev-parse", "--abbrev-ref", "HEAD").stdout.strip() or "unknown"
    dirty = [l for l in _git("status", "--porcelain").stdout.splitlines() if l]
    upstream = _git("rev-parse", "--verify", "origin/main").stdout.strip()
    behind = ahead = 0
    relation = "origin/main 不存在"
    if upstream:
        counts = _git("rev-list", "--left-right", "--count", "origin/main...HEAD").stdout.split()
        if len(counts) == 2:
            behind, ahead = int(counts[0]), int(counts[1])
        relation = f"behind={behind} ahead={ahead}"
    print(
        f"[preflight] branch={branch} HEAD={head[:12]} "
        f"origin/main={upstream[:12] or '?'} {relation}"
    )
    if dirty:
        print(f"[preflight] 工作树不干净：{len(dirty)} 项（读数只代表当前工作树）")
        for line in dirty[:10]:
            print(f"[preflight]   {line}")
        if len(dirty) > 10:
            print(f"[preflight]   ... 其余 {len(dirty) - 10} 项省略")
    else:
        print("[preflight] 工作树干净")
    if behind > 0:
        print(f"\n门禁拒绝运行：HEAD 落后 origin/main {behind} 个提交。")
        print("当前读数会测到旧代码，基线棘轮会把「已修」读成「未修」。")
        print("请先同步：git fetch origin main && git merge --ff-only origin/main（或 rebase）后重跑。")
        return 1
    return 0


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
    # 全量输出：逐条 PASS/LEAK/UNVERIFIED，FAIL 行可直接定位端点与身份
    print(out)
    return code


def go_test(update_baseline: bool = False) -> tuple[int, str]:
    env = os.environ.copy()
    # Go 侧不读 Rust 的 DATABASE_URL：显式给 TEST_DATABASE_URL 默认值，
    # 否则修复线环境里指向 oa4rust_test 的 DATABASE_URL 会把 Go 门禁带偏。
    if not env.get("TEST_DATABASE_URL"):
        env["TEST_DATABASE_URL"] = GO_DSN_DEFAULT
    if update_baseline:
        env["SECURITY_GATE_UPDATE_BASELINE"] = "1"
    else:
        env.pop("SECURITY_GATE_UPDATE_BASELINE", None)
    p = subprocess.run(
        ["go", "test", GO_PKG, "-run", f"^{GO_RUN}$", "-v", "-count=1", "-timeout", "10m"],
        cwd=str(GO_SERVER),
        env=env,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    out = (p.stdout or "") + (p.stderr or "")
    return p.returncode, out


def run_go(update_baseline: bool = False) -> int:
    code, out = go_test(update_baseline)
    print(out)
    # 数据库不可达时 scratchdb 会 t.Skip —— 跳过不是绿，门禁必须红。
    if "--- SKIP: TestSecurityGate" in out or "no test files" in out:
        print("[FAIL] GO-GATE-SKIPPED：数据库不可用或测试未编译，不能当绿")
        return 1
    return code


def mutate(path: Path, pattern: re.Pattern[str], patch: str, label: str) -> bytes:
    # 用原始字节还原，避免 read_text 归一化 CRLF 后把整份业务文件"改脏"
    original = path.read_bytes()
    text = original.decode("utf-8")
    new, n = pattern.subn(patch, text, count=1)
    if n != 1:
        raise SystemExit(f"[{label}] 未命中改造点（模式匹配 {n} 次），业务代码可能已变动，请先更新脚本")
    path.write_bytes(new.encode("utf-8"))
    return original


def mutate_text(path: Path, old: str, new: str, label: str) -> bytes:
    """按字面量替换一次，保留原始字节用于还原（Go 侧用）。"""
    original = path.read_bytes()
    text = original.decode("utf-8")
    n = text.count(old)
    if n != 1:
        raise SystemExit(f"[{label}] 未命中改造点（字面量出现 {n} 次），业务代码可能已变动，请先更新脚本")
    path.write_bytes(text.replace(old, new, 1).encode("utf-8"))
    return original


def sabotage_go(kind: str) -> int:
    """Go 侧破坏实验。

    每条实验声明**期望状态**，而不是只查 "[LEAK]"：
      伪装成已修 / 吞错误 → 期望 UNVERIFIED + NEW-FAIL（不是 PASS，也不是 LEAK）；
      摘归属 / 只筛第一页   → 期望 LEAK；
      基线内恶化           → 期望 DRIFT-TO-LEAK（新增的致命漂移判定）。
    """
    expect_state = "LEAK"
    expect_extra = ""
    baseline_backup: bytes | None = None
    muts: list[tuple[str, str]] | None = None

    if kind == "go-limit0":
        path, old, new, expect = (
            GO_ECONOMY,
            "deduct[k] = int64(-v)",
            "deduct[k] = int64(-2 * v)",
            "go|shop|limit0-double-deduct",
        )
        title = "把商城扣款改成 2 倍（limit_per_day=0 连买两次应暴露多扣）"
    elif kind == "go-idor-rev":
        path, old, new, expect = (
            GO_PRESETS,
            "`DELETE FROM user_presets WHERE id = $1 AND user_id = $2`",
            "`DELETE FROM user_presets WHERE id = $1 AND $2::bigint IS NOT NULL`",
            "go|idor|presets-delete",
        )
        title = "摘掉删除预设的 user_id 归属条件（fwd/rev/adm 都应 LEAK）"
    elif kind == "go-empty-list":
        path, old, new, expect = (
            GO_ECONOMY,
            "FROM orders\n\t\t WHERE user_id = $1\n",
            "FROM orders\n\t\t WHERE user_id = $1 AND FALSE\n",
            "go|idor|orders-list",
        )
        expect_state = "UNVERIFIED"
        title = "让订单列表恒返回空（伪装成已修；canary 应判看不到自己的行）"
    elif kind == "go-swallow-write":
        path = GO_PRESETS
        old = (
            '\tif tag.RowsAffected() == 0 {\n'
            '\t\treturn fmt.Errorf("%w: 预设 %d 不存在", ErrNotFound, presetID)\n'
            '\t}'
        )
        new = '\tif tag.RowsAffected() == 0 {\n\t\treturn nil\n\t}'
        expect = "go|idor|presets-delete"
        expect_state = "UNVERIFIED"
        title = "把删除预设的未命中错误吞掉（2xx 但对方行仍在；本人正控制应仍 PASS）"
    elif kind == "go-dead-write":
        path = GO_PRESETS
        muts = [
            (
                "`DELETE FROM user_presets WHERE id = $1 AND user_id = $2`",
                "`DELETE FROM user_presets WHERE id = $1 AND user_id = $2 AND FALSE`",
            ),
            (
                '\tif tag.RowsAffected() == 0 {\n'
                '\t\treturn fmt.Errorf("%w: 预设 %d 不存在", ErrNotFound, presetID)\n'
                '\t}',
                '\tif tag.RowsAffected() == 0 {\n\t\treturn nil\n\t}',
            ),
        ]
        expect = "go|idor|presets-delete"
        expect_state = "UNVERIFIED"
        expect_extra = "go|write|self-roundtrip"
        title = "删除语句恒不命中 + 吞掉错误（攻击者与本人两条写都 2xx 未落库）"
    elif kind == "go-page2":
        path, old, new, expect = (
            GO_ECONOMY,
            "where := `user_id = $1`",
            "where := `user_id = $1`\n\tif offset > 0 {\n\t\twhere = `$1::bigint IS NOT NULL`\n\t}",
            "go|idor|battles-page2",
        )
        title = "让归属过滤只作用于第一页（offset>0 时放开）"
    elif kind == "go-drift-to-leak":
        path, old, new, expect = (
            GO_PRESETS,
            "`DELETE FROM user_presets WHERE id = $1 AND user_id = $2`",
            "`DELETE FROM user_presets WHERE id = $1 AND $2::bigint IS NOT NULL`",
            "go|idor|presets-delete",
        )
        expect_state = "DRIFT-TO-LEAK"
        title = "把一条基线内 UNVERIFIED 的用例恶化成 LEAK（应触发致命漂移）"
    else:
        raise SystemExit(
            f"未知 Go 破坏实验: {kind}"
            "（可选 go-limit0 / go-idor-rev / go-empty-list / go-swallow-write / "
            "go-dead-write / go-page2 / go-drift-to-leak）"
        )

    print(f"== Go 破坏实验：{title}")
    if muts is None:
        muts = [(old, new)]
    original = path.read_bytes()
    for o, n in muts:
        mutate_text(path, o, n, kind)
    if expect_state == "DRIFT-TO-LEAK":
        # 先把目标用例写成基线里的 UNVERIFIED，再把代码改坏成 LEAK。
        # 只有「漂移到 LEAK 是致命的」这条新判定才能让门禁红。
        baseline_backup = GO_BASELINE.read_bytes()
        GO_BASELINE.write_bytes(
            baseline_backup + f"{expect}\tUNVERIFIED\n".encode("utf-8")
        )
    try:
        code, out = go_test()
        if expect_state == "LEAK":
            caught = code != 0 and f"[LEAK] {expect}" in out
        elif expect_state == "UNVERIFIED":
            caught = (
                code != 0
                and f"[UNVERIFIED] {expect}" in out
                and "NEW-FAIL" in out
                and (not expect_extra or f"[UNVERIFIED] {expect_extra}" in out)
            )
        else:
            caught = code != 0 and "DRIFT-TO-LEAK" in out and expect in out
        for line in out.splitlines():
            if line.startswith(
                ("[LEAK]", "[PASS]", "[UNVERIFIED]", "[SUMMARY]", "[NEW-FAIL]",
                 "[STALE-BASELINE]", "[STATE-DRIFT]", "[DRIFT-TO-LEAK]")
            ) or "--- FAIL" in line or "test result:" in line:
                print(line)
        print()
        if caught:
            print(f"实验成立：Go 门禁变红（exit={code}），并点名 {expect}"
                  f"（期望状态 {expect_state}）")
            return 0
        print(f"实验失败：门禁未红或未点名 {expect}（期望 {expect_state}，exit={code}）")
        return 1
    finally:
        path.write_bytes(original)
        print(f"[restore] {path.name} 已还原")
        if baseline_backup is not None:
            GO_BASELINE.write_bytes(baseline_backup)
            print(f"[restore] {GO_BASELINE.name} 已还原")


def sabotage_rust_drift() -> int:
    """基线内 UNVERIFIED 恶化为 LEAK：必须由 [DRIFT-TO-LEAK] 判红。
    做法与 Go 侧一致：临时把目标写成基线 UNVERIFIED，再把真实代码改坏成 LEAK。
    """
    print("== Rust 破坏实验：基线内 UNVERIFIED 恶化成 LEAK（应触发致命漂移）")
    original = ATTENDANCE_ASSEMBLE.read_bytes()
    baseline_backup = RUST_BASELINE.read_bytes()
    try:
        text = original.decode("utf-8")
        new, n = PAGE_SCOPE_RE.subn(PAGE_SCOPE_PATCH, text, count=1)
        if n != 1:
            raise SystemExit("[rust-drift-to-leak] 未命中 next handler 归属谓词，请先更新脚本")
        ATTENDANCE_ASSEMBLE.write_bytes(new.encode("utf-8"))
        RUST_BASELINE.write_bytes(
            baseline_backup
            + f"{PAGE_DRIFT_EXPECT}\tUNVERIFIED\n".encode("utf-8")
        )
        code, out = cargo_test()
        caught = (
            code != 0
            and "[DRIFT-TO-LEAK]" in out
            and PAGE_DRIFT_EXPECT in out
        )
        for line in out.splitlines():
            if line.startswith(
                ("[LEAK]", "[UNVERIFIED]", "[INTEGRITY]", "[STATE-DRIFT]",
                 "[DRIFT-TO-LEAK]", "NEW-FAIL", "[ADMIN-POLICY-UNKNOWN]")
            ) or "非 PASS" in line or "test result:" in line:
                print(line)
        print()
        if caught:
            print(f"实验成立：Rust 门禁变红（exit={code}），"
                  f"[DRIFT-TO-LEAK] 点名 {PAGE_DRIFT_EXPECT}（UNVERIFIED -> LEAK）")
            return 0
        print("\n".join(out.splitlines()[-30:]))
        print(f"实验失败：门禁未红或未点名 {PAGE_DRIFT_EXPECT}"
              f"（exit={code}；期望 [DRIFT-TO-LEAK]）")
        return 1
    finally:
        ATTENDANCE_ASSEMBLE.write_bytes(original)
        RUST_BASELINE.write_bytes(baseline_backup)
        print(f"[restore] {ATTENDANCE_ASSEMBLE.name} / {RUST_BASELINE.name} 已还原")


def sabotage_rust_swallow() -> int:
    """本人写 2xx 但不落库：正控制必须判 INTEGRITY，而不是当 PASS 放行。"""
    print("== Rust 破坏实验：写接口 2xx 但未落库（本人正控制应判 INTEGRITY）")
    original = ATTENDANCE_ASSEMBLE.read_bytes()
    try:
        text = original.decode("utf-8")
        new, n = SELF_WRITE_RE.subn(SELF_WRITE_PATCH, text, count=1)
        if n != 1:
            raise SystemExit("[rust-swallow-write] 未命中 attendanceselfholiday_create 开头，请先更新脚本")
        ATTENDANCE_ASSEMBLE.write_bytes(new.encode("utf-8"))
        code, out = cargo_test()
        caught = (
            code != 0
            and f"[INTEGRITY] {SELF_WRITE_EXPECT}" in out
            and "NEW-FAIL" in out
        )
        for line in out.splitlines():
            if line.startswith(
                ("[LEAK]", "[UNVERIFIED]", "[INTEGRITY]", "[STATE-DRIFT]",
                 "[DRIFT-TO-LEAK]", "NEW-FAIL", "[ADMIN-POLICY-UNKNOWN]")
            ) or "非 PASS" in line or "test result:" in line:
                print(line)
        print()
        if caught:
            print(f"实验成立：Rust 门禁变红（exit={code}），"
                  f"{SELF_WRITE_EXPECT} 被判 INTEGRITY（2xx 但未落库）")
            return 0
        print("\n".join(out.splitlines()[-30:]))
        print(f"实验失败：门禁未红或未点名 {SELF_WRITE_EXPECT} 为 INTEGRITY（exit={code}）")
        return 1
    finally:
        ATTENDANCE_ASSEMBLE.write_bytes(original)
        print(f"[restore] {ATTENDANCE_ASSEMBLE.name} 已还原")


# hotpic 的归属谓词（本轮写 IDOR 第四批）：全局行仅管理员、专属行本人或管理员。
# 谓词字符串以单个 `)` 收尾（批次 3 曾把终结符写成两个括号导致匹配不到）。
HOTPIC_OWNERSHIP_RE = re.compile(
    r'const HOTPIC_OWNER_PREDICATE: &str = .*?false\)";', re.S
)
HOTPIC_OWNERSHIP_PATCH = 'const HOTPIC_OWNER_PREDICATE: &str = "TRUE";'

# mind 的归属谓词（同批）：creator 本人或管理员。形状与批次 1/2 相同。
READ_B_OWNERSHIP_RE = re.compile(r'const CREATOR_OWNER_PREDICATE: &str = .*?;', re.DOTALL)
READ_B_OWNERSHIP_PATCH = 'const CREATOR_OWNER_PREDICATE: &str = "TRUE";'

MIND_OWNERSHIP_RE = re.compile(
    r'const CREATOR_OWNER_PREDICATE: &str = .*?false\)";', re.S
)
MIND_OWNERSHIP_PATCH = 'const CREATOR_OWNER_PREDICATE: &str = "TRUE";'

# 第五批写 IDOR：file folder 的归属谓词（server 结尾是 `false))";`）
FOLDER_OWNERSHIP_RE = re.compile(
    r'const FOLDER_OWNER_PREDICATE: &str = .*?false\)\)";', re.S
)
FOLDER_OWNERSHIP_PATCH = (
    'const FOLDER_OWNER_PREDICATE: &str = "id = $1 AND $2 IS NOT NULL";'
)

# 第五批写 IDOR：cms 三个「POST 动作别名但实际是读」端点的归属门。
# 只摘掉本批新加的门；同文件其它批次的门保持不动（count=1 且模式锁定表名）。
APPINFO_GATE_RE = re.compile(
    r'match u2_check_owner\(\s*&pool,\s*"x_cms_appinfo",\s*"creator",\s*&app_id,\s*'
    r'&session\.person_unique,\s*\)\s*\.await\?\s*\{.*?'
    r'U2Gate::NotFound => return Ok\(Json\(ActionResult::error\("appinfo not found"\)\)\),\s*\}',
    re.S,
)
APPINFO_GATE_PATCH = "let _ = &session;\n    let _gate = U2Gate::Allowed;"
CATEGORY_GATE_RE = re.compile(
    r'match u2_check_owner\(\s*&pool,\s*"x_cms_categoryinfo",\s*"creator",\s*&id,\s*'
    r'&session\.person_unique,\s*\)\s*\.await\?\s*\{.*?'
    r'U2Gate::NotFound => return Ok\(Json\(ActionResult::error\("category not found"\)\)\),\s*\}',
    re.S,
)
CATEGORY_GATE_PATCH = "let _ = &session;\n    let _gate = U2Gate::Allowed;"
DOC_GATE_RE = re.compile(
    r'match data_document_readable\(&pool, &id, actor\.as_deref\(\)\)\.await\? \{.*?\n    \}',
    re.S,
)
DOC_GATE_PATCH = (
    "let _readable = data_document_readable(&pool, &id, actor.as_deref()).await?;"
)
B5_EXPECTS = [
    "file_core_entity|folder_delete|DELETE",
    "cms_assemble_control|appinfo_appId_icon_size_size|POST",
    "cms_assemble_control|categoryinfo_id_execute_projection|POST",
    "cms_assemble_control|data_document_id_array_data|POST",
]

# 第六批写 IDOR：appeal 处理的归属谓词（申请人/管理员）摘成恒真。
APPEAL_GATE_RE = re.compile(
    r'\n\s*AND \(NULLIF\(BTRIM\(person_id\), \'\'\) = \$3\s*\\?\s*'
    r'OR NULLIF\(BTRIM\(creator\), \'\'\) = \$3\s*\\?\s*'
    r'OR EXISTS\(.*?r\.disable = false\)\)',
    re.S,
)
APPEAL_GATE_PATCH = "\n                AND ($3 = $3)"

# 第六批写 IDOR：processplatform 两把读的 `WHERE` 归属谓词。
TASK_GATE_RE = re.compile(
    r'\n\s*AND \(NULLIF\(BTRIM\(t\.person\), \'\'\) = \$2\s*\\?\s*'
    r'OR NULLIF\(BTRIM\(w\.creator\), \'\'\) = \$2\s*\\?\s*'
    r'OR \{U4_ADMIN_EXISTS\}\)',
    re.S,
)
TASK_GATE_PATCH = "\n            AND ($2 = $2)"
ATTACH_GATE_RE = re.compile(
    r'\n\s*AND \(NULLIF\(BTRIM\(creator\), \'\'\) = \$2 OR \{U4_ADMIN_EXISTS\}\)',
    re.S,
)
ATTACH_GATE_PATCH = "\n            AND ($2 = $2)"
B6_EXPECTS = [
    "attendance_assemble_control|attendanceappealInfo_appeal_id|POST",
    "t1|attendanceappealInfo_appeal_id|POST",
    "t1|attendanceappealInfo_appeal_id|PUT",
    "processplatform_service_processing|attachment_edit_id_text|POST",
    "processplatform_service_processing|task_id_will|POST",
]

# 第七批写 IDOR：读面归属谓词（`*_READ_OWNER_PREDICATE`）。
# 替换必须保留 `$2` 引用，写成 `(TRUE AND $2::text IS NOT NULL)`；
# 裸 `TRUE` 会让按参数个数绑定的 deadpool 端点 bind 数不匹配 → 500，
# 500 被记成 UNVERIFIED 后又会变成假阳性 NEW-FAIL。
# 注意：calendar_core_entity 的同名常量归分片 B（#207），#209 撤段后不再存在，这里不含它。
READ_OWNER_PREDICATE_RE = re.compile(
    r'(const \w+_READ_OWNER_PREDICATE: &str = ").*?false\)\)";', re.S
)
READ_OWNER_PREDICATE_PATCH = r'\1(TRUE AND $2::text IS NOT NULL)";'
B7_EXPECTS = [
    "component_assemble_control|get_component|GET",
    "correlation_service_processing|get_correlation|GET",
    "jpush|device_get|GET",
    "jpush_assemble_control|get_jpush|GET",
    "jpush_core_entity|device_get|GET",
]

# 写 IDOR 第 W-b5 批：query designer 五个读端点的归属校验入口。
# 把 `require_owner_or_admin` 的判定条件改恒真（保留三元绑定，避免未使用告警），
# 非属主读受害者行 → 逐条 LEAK。
W5_OWNER_GATE_RE = re.compile(
    r"if creator\.is_some_and\(\|c\| c == actor\)\s*"
    r"\|\| shared::middleware::is_admin\(pool, actor\)\.await",
    re.S,
)
W5_OWNER_GATE_PATCH = (
    "if {\n        let _ = (creator, pool, actor);\n        true\n    }"
)
W5_EXPECTS = [
    "query_assemble_designer|get_designer|GET",
    "query_assemble_designer|query_id_permission|GET",
    "query_assemble_designer|stat_id|GET",
    "query_assemble_designer|stat_id_permission|GET",
    "query_assemble_designer|stat_id_simulate|GET",
]

# 写 IDOR 第 W-b6 批：query designer 三个 view 读 + surface 五个读面归属。
# 两 crate 各有一份同款 `require_owner_or_admin` 判定式，逐文件改恒真
# （保留三元绑定，避免未使用告警），非属主读受害者行 → 逐条 LEAK。
QUERY_SURFACE = BACKEND / "crates/query_assemble_surface/src/lib.rs"
W6_EXPECTS = [
    "query_assemble_designer|view_id_bundle|GET",
    "query_assemble_designer|view_id_permission|GET",
    "query_assemble_designer|view_id_simulate|GET",
    "query_assemble_surface|get_surface|GET",
    "query_assemble_surface|view_id_bundle_v2|GET",
    "query_assemble_surface|view_id_bundle|GET",
    "query_assemble_surface|view_id_excel|GET",
    "query_assemble_surface|view_id_execute|GET",
]

# 写 IDOR 第 W-b2 批：general_core 的 file/invoice 读面归属谓词 + 创建面冒名拒绝。
# 读面把 `creator = $2` 改成 `TRUE OR creator = $2`（保留 $2 绑定，避免 500 假阳性）；
# 写面摘掉非管理员 403 分支，让冒名创建落库 → 逐条 LEAK。
GENERAL_CORE = BACKEND / "crates/general_core_entity/src/lib.rs"
GENERAL_ASSEMBLE = BACKEND / "crates/general_assemble_control/src/lib.rs"
W2_READ_RE = re.compile(r"creator = \$2 OR EXISTS")
W2_READ_PATCH = "TRUE OR creator = $2 OR EXISTS"
W2_CORE_WRITE_RE = re.compile(
    r"if is_admin_db\(db, actor\)\.await \{\s*Ok\(owner\)\s*\} else \{\s*Err\(AppError::Forbidden\)\s*\}"
)
W2_CORE_WRITE_PATCH = "Ok({ let _ = is_admin_db(db, actor).await; owner })"
W2_ASSEMBLE_WRITE_RE = re.compile(r"require_admin\(pool, session\)\.await\?;")
W2_ASSEMBLE_WRITE_PATCH = "let _ = (&pool, &session);"
W2_EXPECTS = [
    "general_core_entity|file_download|GET",
    "general_core_entity|file_get|GET",
    "general_core_entity|invoice_get|GET",
    "t2|general_assemble|invoice_create",
    "t2|general_core|file_create",
    "t2|general_core|invoice_create",
]

# 写 IDOR 第 W-b1 批：general_assemble_control 五个读端点的 owner_gate。
# 直接摘掉 `owner_gate(...).await?;` 调用点（而非改谓词 SQL）：
# 每处调用点带表名，可逐条精确替换；摘掉后端点会回显受害者行 → 逐条 LEAK。
W1_GATE_RE = [
    (
        table,
        re.compile(
            r'\n\s*owner_gate\(\s*&pool,\s*&client,\s*&sess,\s*"' + table
            + r'",\s*"creator",\s*&id,\s*\)\s*\.await\?;',
            re.S,
        ),
    )
    for table in (
        "x_general_assemble_area",
        "x_general_attend_scope",
        "x_general_assemble_invoice",
        "x_general_assemble_qrcode",
        "x_general_assemble_security_clearance",
    )
]
W1_GATE_PATCH = "\n    let _ = (&sess, &id);"
W1_EXPECTS = [
    "general_assemble_control|area_get|GET",
    "general_assemble_control|attendscope_get|GET",
    "general_assemble_control|invoice_get_id|GET",
    "general_assemble_control|qrcode_get|GET",
    "general_assemble_control|securityclearance_get|GET",
]


# 分片 A / A-b1：cms 六条读端点的读门（文档 ACL / 单主体列 / 评论跟随文档 ACL）。
# 恒真式必须保留 `$2` 绑定：裸 TRUE 会让 deadpool 端点因 bind 数不匹配返回 500，
# 500 被记成 UNVERIFIED 后又会变成假阳性。
CMS_DOC_READ_GATE_RE = re.compile(
    r'(const CMS_DOCUMENT_READ_ALLOWED_SQL: &str = ").*?";', re.S
)
CMS_DOC_READ_GATE_PATCH = (
    r'\1SELECT 1 AS hit FROM x_cms_data_document WHERE id = $1 '
    r'AND deleted_at IS NULL '
    r'AND (($2::text IS NOT NULL OR $2::text IS NULL) OR $3::boolean)";'
)
CMS_OWNED_READ_GATE_RE = re.compile(
    r'AND \(\{owner_col\} = \$2::text OR \$3::boolean\)'
)
CMS_OWNED_READ_GATE_PATCH = (
    "AND ({owner_col} IS NOT NULL OR {owner_col} IS NULL "
    "OR $2::text IS NOT NULL OR $2::text IS NULL OR $3::boolean)"
)
CMS_COMMENT_READ_GATE_RE = re.compile(
    r'(const CMS_COMMENT_READ_ALLOWED_SQL: &str = ").*?";', re.S
)
CMS_COMMENT_READ_GATE_PATCH = (
    r'\1SELECT 1 AS hit FROM x_cms_comment WHERE id = $1 '
    r'AND deleted_at IS NULL '
    r'AND (($2::text IS NOT NULL OR $2::text IS NULL) OR $3::boolean)";'
)
A1_EXPECTS = [
    "cms_assemble_control|anonymous_document_id_view|GET",
    "cms_assemble_control|document_persons_u3|GET",
    "cms_assemble_control|file_id|GET",
    "cms_assemble_control|file_id_content|GET",
    "cms_assemble_control|file_id_download|GET",
    "cms_assemble_control|comment_id|GET",
]

# 写 IDOR 第 W-b3 批：office/bbs/mind 写侧冒名 403 臂。
# 把三个 crate resolve_* 的管理员臂改成恒放行（保留 is_admin_db 调用引用，避免未使用告警），
# 非管理员冒名落库 → 逐条 LEAK。
GENERAL_ASSEMBLE = BACKEND / "crates/general_assemble_control/src/lib.rs"
BBS_CORE = BACKEND / "crates/bbs_core_entity/src/lib.rs"
MIND_CORE = BACKEND / "crates/mind_core_entity/src/lib.rs"
W3_GENERAL_WRITE_RE = re.compile(
    r"Some\(owner\) => \{\s*require_admin\(pool, session\)\.await\?;\s*Ok\(owner\)\s*\}"
)
W3_GENERAL_WRITE_PATCH = "Some(owner) => { let _ = (&pool, &session); Ok(owner) }"
W3_SEA_WRITE_RE = re.compile(
    r"Some\(owner\) => \{\s*if is_admin_db\(db, &sess\.person_unique\)\.await \{\s*"
    r"Ok\(owner\)\s*\} else \{\s*Err\(AppError::Forbidden\)\s*\}\s*\}"
)
W3_SEA_WRITE_PATCH = (
    "Some(owner) => { Ok({ let _ = is_admin_db(db, &sess.person_unique).await; owner }) }"
)
W3_EXPECTS = [
    "t2|general|office",
    "t2|general|office_html_to_word",
    "t1|bbs|create_reply",
    "t1|bbs|create_subject",
    "t1|mind_core|create_mind",
    "t1|mind_core|create_folder",
    "t2|mind_core|create_version",
]


def sabotage_many(
    title: str,
    mutations: list[tuple[Path, re.Pattern[str], str, str]],
    expects: list[str],
    require_leak: bool = False,
) -> int:
    """多文件破坏实验：逐个替换，跑一次门禁，最后统一还原。

    require_leak=True 时逐条要求 `[LEAK] <id>`：这些用例的基线行已经在修复
    PR 里删掉，用 NEW-FAIL 判定会被「500 → UNVERIFIED」的假阳性蒙混过关。
    """
    print(f"== 破坏实验：{title}")
    originals: list[tuple[Path, bytes]] = []
    try:
        for path, pattern, patch, label in mutations:
            originals.append((path, mutate(path, pattern, patch, label)))
        code, out = cargo_test()
        if require_leak:
            caught = code != 0 and all(f"[LEAK] {e}" in out for e in expects)
        else:
            caught = code != 0 and "NEW-FAIL" in out and all(e in out for e in expects)
        for line in out.splitlines():
            if line.startswith(("[LEAK]", "[PASS]", "[UNVERIFIED]", "NEW-FAIL", "STALE-BASELINE")) \
                    or line.startswith("=== ") or "非 PASS" in line or "test result:" in line:
                print(line)
        print()
        if caught:
            print(f"实验成立：门禁变红（exit={code}），并点名 {len(expects)} 条用例")
            return 0
        print(f"实验失败：门禁未红或未点名全部用例（exit={code}）")
        return 1
    finally:
        # 同一文件可能有多次替换（如 processplatform 两把都在 lib.rs）：必须逆序还原，
        # 否则最后一次写入的是「已被改坏」的快照，而不是最初的原文件。
        for path, original in reversed(originals):
            path.write_bytes(original)
            print(f"[restore] {path.name} 已还原")


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
    elif kind == "write-idor-w4":
        return sabotage_many(
            "摘掉 W-b4 program_center 8 个读端点的归属谓词（8 条应逐条 LEAK）",
            [
                (
                    PROGRAM_CENTER,
                    PROGRAM_CENTER_OWNERSHIP_RE,
                    PROGRAM_CENTER_OWNERSHIP_PATCH,
                    "program_center_owner",
                ),
            ],
            W4_EXPECTS,
            require_leak=True,
        )
    elif kind == "read_b":
        return sabotage_many(
            "摘掉读越权分片B的 creator/管理员归属谓词（5 条应变 NEW-FAIL）",
            [
                (ORG_ASSEMBLE_CONTROL_HELPERS, READ_B_OWNERSHIP_RE, READ_B_OWNERSHIP_PATCH, "org-control"),
                (ORG_ASSEMBLE_AUTH, READ_B_OWNERSHIP_RE, READ_B_OWNERSHIP_PATCH, "org-auth"),
                (MESSAGE_COMMUNICATE, READ_B_OWNERSHIP_RE, READ_B_OWNERSHIP_PATCH, "message"),
            ],
            READ_B_EXPECTS,
        )
    elif kind == "hotpic_mind":
        return sabotage_many(
            "摘掉 hotpic/mind 的归属谓词（4 条应变 NEW-FAIL）",
            [
                (HOTPIC_CORE, HOTPIC_OWNERSHIP_RE, HOTPIC_OWNERSHIP_PATCH, "hotpic_core"),
                (HOTPIC_ASSEMBLE, HOTPIC_OWNERSHIP_RE, HOTPIC_OWNERSHIP_PATCH, "hotpic_assemble"),
                (MIND, MIND_OWNERSHIP_RE, MIND_OWNERSHIP_PATCH, "mind"),
            ],
            HOTPIC_MIND_EXPECTS,
        )
    elif kind == "write-idor-b5":
        return sabotage_many(
            "摘掉第五批写 IDOR 的归属（folder_delete + cms 三条，应变 NEW-FAIL）",
            [
                (FILE_CORE, FOLDER_OWNERSHIP_RE, FOLDER_OWNERSHIP_PATCH, "file_core"),
                (CMS, APPINFO_GATE_RE, APPINFO_GATE_PATCH, "cms_appinfo"),
                (CMS, CATEGORY_GATE_RE, CATEGORY_GATE_PATCH, "cms_categoryinfo"),
                (CMS, DOC_GATE_RE, DOC_GATE_PATCH, "cms_document"),
            ],
            B5_EXPECTS,
        )
    elif kind == "write-idor-b6":
        return sabotage_many(
            "摘掉第六批写 IDOR 的归属（appeal + processplatform 两把，应变 NEW-FAIL）",
            [
                (ATTENDANCE_ASSEMBLE, APPEAL_GATE_RE, APPEAL_GATE_PATCH, "appeal"),
                (PROCESS_PLATFORM, TASK_GATE_RE, TASK_GATE_PATCH, "task_will"),
                (PROCESS_PLATFORM, ATTACH_GATE_RE, ATTACH_GATE_PATCH, "attachment"),
            ],
            B6_EXPECTS,
        )
    elif kind == "write-idor-b7":
        return sabotage_many(
            "摘掉第七批写 IDOR 的读归属谓词（5 条应逐条 LEAK）",
            [
                (COMPONENT, READ_OWNER_PREDICATE_RE, READ_OWNER_PREDICATE_PATCH, "component"),
                (CORRELATION, READ_OWNER_PREDICATE_RE, READ_OWNER_PREDICATE_PATCH, "correlation"),
                (JPUSH, READ_OWNER_PREDICATE_RE, READ_OWNER_PREDICATE_PATCH, "jpush"),
                (
                    JPUSH_ASSEMBLE,
                    READ_OWNER_PREDICATE_RE,
                    READ_OWNER_PREDICATE_PATCH,
                    "jpush_assemble",
                ),
                (JPUSH_CORE, READ_OWNER_PREDICATE_RE, READ_OWNER_PREDICATE_PATCH, "jpush_core"),
            ],
            B7_EXPECTS,
            require_leak=True,
        )
    elif kind == "write-idor-w5":
        return sabotage_many(
            "摘掉 W-b5 query designer 读端点的归属校验（5 条应逐条 LEAK）",
            [
                (
                    QUERY_DESIGNER,
                    W5_OWNER_GATE_RE,
                    W5_OWNER_GATE_PATCH,
                    "query_designer_owner",
                ),
            ],
            W5_EXPECTS,
            require_leak=True,
        )
    elif kind == "write-idor-w6":
        return sabotage_many(
            "摘掉 W-b6 query view/surface 读端点的归属校验（8 条应逐条 LEAK）",
            [
                (
                    QUERY_DESIGNER,
                    W5_OWNER_GATE_RE,
                    W5_OWNER_GATE_PATCH,
                    "query_designer_view",
                ),
                (
                    QUERY_SURFACE,
                    W5_OWNER_GATE_RE,
                    W5_OWNER_GATE_PATCH,
                    "query_surface_view",
                ),
            ],
            W6_EXPECTS,
            require_leak=True,
        )
    elif kind == "write-idor-w2":
        return sabotage_many(
            "放开 W-b2 读归属谓词 + 摘掉冒名创建 403（6 条应逐条 LEAK）",
            [
                (GENERAL_CORE, W2_READ_RE, W2_READ_PATCH, "general_core_read"),
                (GENERAL_CORE, W2_CORE_WRITE_RE, W2_CORE_WRITE_PATCH, "general_core_write"),
                (
                    GENERAL_ASSEMBLE,
                    W2_ASSEMBLE_WRITE_RE,
                    W2_ASSEMBLE_WRITE_PATCH,
                    "general_assemble_write",
                ),
            ],
            W2_EXPECTS,
            require_leak=True,
        )
    elif kind == "read-a-b1":
        return sabotage_many(
            "摘掉分片 A 第一批 cms 读归属谓词（6 条应逐条 LEAK）",
            [
                (CMS, CMS_DOC_READ_GATE_RE, CMS_DOC_READ_GATE_PATCH, "cms_doc_read"),
                (CMS, CMS_OWNED_READ_GATE_RE, CMS_OWNED_READ_GATE_PATCH, "cms_owned_read"),
                (CMS, CMS_COMMENT_READ_GATE_RE, CMS_COMMENT_READ_GATE_PATCH, "cms_comment_read"),
            ],
            A1_EXPECTS,
            require_leak=True,
        )
    elif kind == "write-idor-w1":
        return sabotage_many(
            "摘掉 W-b1 五个读端点的 owner_gate（5 条应逐条 LEAK）",
            [
                (GENERAL_ASSEMBLE, pattern, W1_GATE_PATCH, table)
                for table, pattern in W1_GATE_RE
            ],
            W1_EXPECTS,
            require_leak=True,
        )
    elif kind == "write-idor-w3":
        return sabotage_many(
            "摘掉 W-b3 office/bbs/mind 写侧冒名 403（7 条应逐条 LEAK）",
            [
                (
                    GENERAL_ASSEMBLE,
                    W3_GENERAL_WRITE_RE,
                    W3_GENERAL_WRITE_PATCH,
                    "general_office_write",
                ),
                (BBS_CORE, W3_SEA_WRITE_RE, W3_SEA_WRITE_PATCH, "bbs_core_write"),
                (MIND_CORE, W3_SEA_WRITE_RE, W3_SEA_WRITE_PATCH, "mind_core_write"),
            ],
            W3_EXPECTS,
            require_leak=True,
        )
    else:
        raise SystemExit(
            f"未知破坏实验: {kind}（可选 ownership / correct / general / organization / "
            "program_center / hotpic_mind / read_b / read-a-b1 / "
            "write-idor-b5 / write-idor-b6 / write-idor-b7 / "
            "write-idor-w1 / write-idor-w2 / write-idor-w3 / write-idor-w4 / write-idor-w5 / "
            "write-idor-w6）"
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
    if cmd in ("run", "baseline", "go", "go-baseline", "sabotage"):
        code = git_preflight()
        if code:
            return code
    if cmd == "run":
        return run()
    if cmd == "baseline":
        return run(update_baseline=True)
    if cmd == "go":
        return run_go()
    if cmd == "go-baseline":
        return run_go(update_baseline=True)
    if cmd == "sabotage":
        if len(sys.argv) < 3:
            print("用法: security_regression_gate.py sabotage "
                  "<ownership|correct|general|organization|program_center|hotpic_mind|read_b|"
                  "read-a-b1|write-idor-b5|write-idor-b6|write-idor-b7|"
                  "write-idor-w1|write-idor-w2|write-idor-w3|write-idor-w4|write-idor-w5|"
                  "rust-drift-to-leak|rust-swallow-write|"
                  "go-limit0|go-idor-rev|go-empty-list|go-swallow-write|"
                  "go-dead-write|go-page2|go-drift-to-leak>")
            return 2
        kind = sys.argv[2]
        if kind.startswith("go-"):
            return sabotage_go(kind)
        if kind in ("rust-drift-to-leak", "drift-to-leak"):
            return sabotage_rust_drift()
        if kind in ("rust-swallow-write", "swallow-write"):
            return sabotage_rust_swallow()
        return sabotage(kind)
    print(__doc__)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
