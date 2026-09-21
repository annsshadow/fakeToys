"""把 cosmic-ray 会话裁剪为分层抽样，使变异测试在可接受时间内完成

## 为什么需要抽样

cosmic-ray 生成的变异体分布极度不均。以 `augmentor/quality.py` 为例，676 个变异体中：

- `core/NumberReplacer` 144 个（把字面量数字换成别的数字）；
- `core/ReplaceBinaryOperator_*` 共 253 个——每个二元运算符被替换成**其余所有**运算符
  （`Add→Sub`、`Add→Mul`、`Add→Div`、`Add→BitAnd` …），其中绝大多数是等价变异体
  （改了代码但行为不变），对"测试能否杀死变异体"几乎没有信息量。

三个目标模块合计 1320 个变异体，逐个跑一次测试约 4.2 秒，总计约 92 分钟。
按算子**族**分层抽样后压到约 1/4，保留全部算子类型的信息量。

## 抽样规则

1. 族名 = 算子名去掉尾部的 `_X_Y` 后缀
   （`ReplaceBinaryOperator_Add_Sub` → `ReplaceBinaryOperator`；
   `ReplaceTrueWithFalse` 无后缀，自成一族）；
2. 每族最多保留 `--per-family` 个（默认 8）；
3. 族内**按位置均匀取样**，不是取前 N 个——取前 N 个会集中在该文件的开头，
   等于只测了文件前几行。

抽样是确定性的（按 occurrence 排序后等距取），因此结果可复现。

用法：
    python scripts/mutation/sample.py <session.sqlite> [--per-family 8]
"""

import argparse
import re
import sqlite3
import sys

# 尾部形如 `_Add_Sub` 的后缀
_SUFFIX = re.compile(r"_[A-Za-z]+_[A-Za-z]+$")


def family_of(operator_name: str) -> str:
    """取算子族名（去掉尾部 `_X_Y` 后缀）

    Args:
        operator_name: cosmic-ray 算子名，如 `core/ReplaceBinaryOperator_Add_Sub`

    Returns:
        族名，如 `core/ReplaceBinaryOperator`
    """
    return _SUFFIX.sub("", operator_name)


def stratified_sample(specs: list, per_family: int) -> list:
    """按算子族分层抽样

    Args:
        specs: (job_id, operator_name, occurrence) 列表
        per_family: 每族最多保留几个

    Returns:
        被选中的 job_id 列表
    """
    groups = {}
    for job_id, operator_name, occurrence in specs:
        groups.setdefault(family_of(operator_name), []).append(
            (occurrence, job_id)
        )

    selected = []
    for family in sorted(groups):
        members = sorted(groups[family])          # 按 occurrence 排序
        if len(members) <= per_family:
            selected.extend(job_id for _, job_id in members)
            continue
        # 等距取样：把 [0, n-1] 均分成 per_family 段，每段取一个
        n = len(members)
        for i in range(per_family):
            idx = round(i * (n - 1) / (per_family - 1)) if per_family > 1 else 0
            selected.append(members[idx][1])
    return selected


def main() -> int:
    parser = argparse.ArgumentParser(description="cosmic-ray 会话分层抽样")
    parser.add_argument("session", help="cosmic-ray session sqlite 文件")
    parser.add_argument("--per-family", type=int, default=8,
                        help="每个算子族保留的变异体数（默认 8）")
    args = parser.parse_args()

    con = sqlite3.connect(args.session)

    specs = con.execute(
        "SELECT job_id, operator_name, occurrence FROM mutation_specs"
    ).fetchall()
    total = con.execute("SELECT COUNT(*) FROM work_items").fetchone()[0]

    keep = set(str(job_id) for job_id in stratified_sample(specs, args.per_family))

    # job_id 在 work_items 中为 TEXT，mutation_specs 中为 INTEGER，统一成字符串比较
    rows = con.execute("SELECT job_id, CAST(job_id AS TEXT) FROM work_items").fetchall()
    drop = [rowid for rowid, text in rows if text not in keep]
    con.executemany("DELETE FROM work_items WHERE job_id = ?",
                    [(rowid,) for rowid in drop])
    con.commit()

    after = con.execute("SELECT COUNT(*) FROM work_items").fetchone()[0]

    families = {}
    for job_id, operator_name, _ in specs:
        if str(job_id) in keep:
            fam = family_of(operator_name)
            families[fam] = families.get(fam, 0) + 1

    print(f"抽样：{total} → {after} 个变异体（每族上限 {args.per_family}）")
    print(f"覆盖 {len(families)} 个算子族：")
    for fam in sorted(families, key=lambda f: -families[f]):
        print(f"  {families[fam]:4d}  {fam}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
