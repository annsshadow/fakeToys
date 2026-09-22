# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""变异抽检运行器（cosmic-ray 包装）

用法（在 `augmentor/` 目录下，用项目 venv 的 python 执行）：

    python scripts/mutation/run.py quality dedup export
    python scripts/mutation/run.py --report quality     # 只出报告，不执行
    python scripts/mutation/run.py --reinit quality     # 丢弃旧会话重新抽样
    python scripts/mutation/run.py --per-family 4 quality

## 为什么需要这层包装

1. **解释器必须显式指定。** `test-command` 里写裸 `python` 会解析到托管版解释器
   （`~/.workbuddy-ai/binaries/python/versions/...`，没装 pytest）。于是每个变异体都
   因 `ModuleNotFoundError` 退出码非 0 而被 cosmic-ray 记为 `killed`，产出**假的
   100% 杀死率**——比失败更糟，因为它看起来像好消息。
   把 venv 的 `Scripts` 目录前置到 PATH 也**无效**（实测：即使排在第一位，`python`
   仍解析到托管版），因此 TOML 里写占位符 `{python}`，由本脚本替换为 `sys.executable`。

2. **路径必须用正斜杠。** cosmic-ray 用 `shlex.split(command)`（posix 模式）解析命令，
   反斜杠会被当成转义符吃掉，`C:\\Users\\...` 会变成 `C:Users...`。

3. **报告口径。** 变异得分 = `killed / (killed + survived)`；`incompetent`
   （超时、无法执行）不计入分母，但必须单独显示——它占比高说明测试命令本身有问题，
   此时得分不可信。

会话文件写到 `%TEMP%/mut_<name>.sqlite`，解析后的配置写到
`%TEMP%/mut_<name>.resolved.toml`，都不进仓库。
"""

import argparse
import os
import sqlite3
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]          # augmentor/
MUT_DIR = Path(__file__).resolve().parent
TEMP = Path(os.environ.get("TEMP") or os.environ.get("TMP") or ".")

sys.path.insert(0, str(MUT_DIR))
from sample import stratified_sample  # noqa: E402  pylint: disable=wrong-import-position

# 规划 T1.4 的验收门槛
TARGET_KILL_RATE = 0.60


def session_path(name: str) -> Path:
    "会话数据库路径"
    return TEMP / f"mut_{name}.sqlite"


def resolved_config(name: str) -> Path:
    """把 TOML 模板里的 `{python}` 替换成本进程的解释器

    Args:
        name: 模块名（与 `scripts/mutation/<name>.toml` 同名）

    Returns:
        解析后的配置文件路径

    Raises:
        SystemExit: 模板缺失或没有 `{python}` 占位符
    """
    template = MUT_DIR / f"{name}.toml"
    if not template.is_file():
        raise SystemExit(f"找不到配置模板：{template}")

    text = template.read_text(encoding="utf-8")
    if "{python}" not in text:
        raise SystemExit(
            f"{template} 缺少 {{python}} 占位符。\n"
            "直接写裸 `python` 会让所有变异体因解释器缺 pytest 被误判为 killed，"
            "产出假的 100% 杀死率。"
        )

    # 只替换 test-command 那一行：注释里也出现了 `{python}` 这个字样（在讲为什么
    # 需要占位符），整体 replace 会把注释里的说明也替换掉，读起来莫名其妙。
    interpreter = sys.executable.replace("\\", "/")
    lines = []
    for line in text.splitlines():
        if line.lstrip().startswith("test-command"):
            line = line.replace("{python}", interpreter)
        lines.append(line)

    out = TEMP / f"mut_{name}.resolved.toml"
    out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return out


def _cosmic_ray(*args: str) -> subprocess.CompletedProcess:
    "调用 cosmic-ray CLI（cwd 固定为项目根，module-path 才解析得到）"
    return subprocess.run(
        [sys.executable, "-m", "cosmic_ray.cli", *args],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )


def init_session(name: str, config: Path, per_family: int) -> Path:
    "建立会话并按算子族分层抽样"
    sess = session_path(name)
    if sess.exists():
        sess.unlink()

    result = _cosmic_ray("init", str(config), str(sess))
    if result.returncode != 0:
        raise SystemExit(f"[{name}] init 失败：{result.stderr.strip()[:600]}")

    con = sqlite3.connect(sess)
    specs = con.execute(
        "SELECT job_id, operator_name, occurrence FROM mutation_specs"
    ).fetchall()
    total = con.execute("SELECT COUNT(*) FROM work_items").fetchone()[0]

    keep = {str(job_id) for job_id in stratified_sample(specs, per_family)}
    rows = con.execute("SELECT job_id, CAST(job_id AS TEXT) FROM work_items").fetchall()
    drop = [(rowid,) for rowid, text in rows if text not in keep]
    con.executemany("DELETE FROM work_items WHERE job_id = ?", drop)
    con.commit()
    after = con.execute("SELECT COUNT(*) FROM work_items").fetchone()[0]
    con.close()

    print(f"  建立会话：{total} 个变异体 → 抽样保留 {after} 个（每族上限 {per_family}）")
    return sess


def execute(name: str, sess: Path, config: Path) -> None:
    "执行待处理变异体"
    pending = _pending(sess)
    if pending == 0:
        print("  没有待执行项，跳过执行")
        return

    print(f"  开始执行 {pending} 个变异体……（串行，每个跑一遍该模块的测试文件）")
    result = _cosmic_ray("exec", str(config), str(sess))
    if result.returncode != 0:
        raise SystemExit(f"[{name}] exec 失败：{result.stderr.strip()[:600]}")


def _pending(sess: Path) -> int:
    con = sqlite3.connect(sess)
    total = con.execute("SELECT COUNT(*) FROM work_items").fetchone()[0]
    done = con.execute("SELECT COUNT(*) FROM work_results").fetchone()[0]
    con.close()
    return total - done


def collect(name: str) -> dict:
    """统计单个会话的结果

    Returns:
        {total, killed, survived, incompetent, other, kill_rate, examples}
    """
    con = sqlite3.connect(session_path(name))
    total = con.execute("SELECT COUNT(*) FROM work_items").fetchone()[0]
    rows = con.execute(
        "SELECT worker_outcome, test_outcome, COUNT(*) FROM work_results GROUP BY 1, 2"
    ).fetchall()
    survived_rows = con.execute(
        """SELECT r.diff FROM work_results r
           WHERE UPPER(r.test_outcome) = 'SURVIVED' LIMIT 5"""
    ).fetchall()
    con.close()

    killed = survived = incompetent = 0
    for _worker_outcome, test_outcome, count in rows:
        # 库里存的是枚举**名**（`KILLED`/`SURVIVED`），不是 StrEnum 的值（`killed`）。
        # 早先按小写比较导致全部落空，报告显示「已执行 0」——但结果其实都写进去了。
        outcome = (test_outcome or "").lower()
        if outcome == "killed":
            killed += count
        elif outcome == "survived":
            survived += count
        elif outcome == "incompetent":
            incompetent += count

    other = total - killed - survived - incompetent
    denominator = killed + survived
    kill_rate = killed / denominator if denominator else float("nan")

    return {
        "total": total,
        "killed": killed,
        "survived": survived,
        "incompetent": incompetent,
        "other": other,
        "kill_rate": kill_rate,
        "survivor_diffs": [row[0] for row in survived_rows],
    }


def print_report(name: str, stats: dict, show_diffs: bool) -> None:
    "打印单模块报告"
    total, killed = stats["total"], stats["killed"]
    survived, incompetent = stats["survived"], stats["incompetent"]
    rate = stats["kill_rate"]
    verdict = "达标" if rate >= TARGET_KILL_RATE else "**未达标**"

    print(f"  已执行 {killed + survived + incompetent}/{total}")
    print(f"    killed      {killed}")
    print(f"    survived    {survived}")
    print(f"    incompetent {incompetent}")
    if stats["other"]:
        print(f"    未执行      {stats['other']}")
    if rate == rate:  # not NaN
        print(f"  变异得分 = {killed}/{killed + survived} = {rate:.1%}  "
              f"（门槛 {TARGET_KILL_RATE:.0%}，{verdict}）")
    else:
        print("  变异得分：无法计算（killed + survived = 0）")

    if show_diffs and stats["survivor_diffs"]:
        print("  存活变异体样例（测试没能发现的行为改变）：")
        for diff in stats["survivor_diffs"]:
            lines = [ln for ln in (diff or "").splitlines() if ln[:1] in "+-" and ln[:3] not in ("+++", "---")]
            print(f"    {' | '.join(lines[:2])}")


def main() -> int:
    parser = argparse.ArgumentParser(description="cosmic-ray 变异抽检运行器")
    parser.add_argument("modules", nargs="+", help="模块名（对应 scripts/mutation/<name>.toml）")
    parser.add_argument("--per-family", type=int, default=8,
                        help="每个算子族保留的变异体数（默认 8）")
    parser.add_argument("--reinit", action="store_true", help="丢弃旧会话，重新生成并抽样")
    parser.add_argument("--report", action="store_true", help="只出报告，不执行")
    parser.add_argument("--diffs", action="store_true", help="打印存活变异体的代码 diff")
    args = parser.parse_args()

    print(f"解释器：{sys.executable}")
    print(f"会话目录：{TEMP}")

    summary = []
    for name in args.modules:
        print(f"\n{'#' * 10} {name} {'#' * 10}")
        config = resolved_config(name)
        sess = session_path(name)

        if args.report:
            if not sess.exists():
                print("  会话不存在，跳过")
                continue
        elif args.reinit or not sess.exists():
            sess = init_session(name, config, args.per_family)
        else:
            print(f"  复用已有会话：{sess.name}（待执行 {_pending(sess)} 个）")

        if not args.report:
            execute(name, sess, config)

        stats = collect(name)
        print_report(name, stats, args.diffs)
        summary.append((name, stats))

    if len(summary) > 1:
        print(f"\n{'=' * 46}")
        print("汇总")
        print(f"{'=' * 46}")
        total_killed = total_survived = total_incompetent = 0
        for name, stats in summary:
            rate = stats["kill_rate"]
            shown = f"{rate:.1%}" if rate == rate else "n/a"
            print(f"  {name:10s} killed={stats['killed']:3d} "
                  f"survived={stats['survived']:3d} "
                  f"incompetent={stats['incompetent']:3d}  得分={shown}")
            total_killed += stats["killed"]
            total_survived += stats["survived"]
            total_incompetent += stats["incompetent"]

        denominator = total_killed + total_survived
        overall = total_killed / denominator if denominator else float("nan")
        verdict = "达标" if overall >= TARGET_KILL_RATE else "**未达标**"
        print(f"  {'合计':10s} killed={total_killed:3d} survived={total_survived:3d} "
              f"incompetent={total_incompetent:3d}")
        print(f"  整体变异得分 = {overall:.1%}（门槛 {TARGET_KILL_RATE:.0%}，{verdict}）")
        return 0 if overall >= TARGET_KILL_RATE else 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
