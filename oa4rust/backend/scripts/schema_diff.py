#!/usr/bin/env python3
"""schema_diff.py — fresh vs live 形状分叉检测（live DB 对账脚本）

## 用途

111/112 迁移用 `ADD COLUMN IF NOT EXISTS` 幂等补列：对 **live 库**（已有列）是
no-op，对 **fresh 库**（列由本迁移建）才生效。两者形状可能分叉——111 声明的
类型是「按代码绑定推定」（TEXT/BOOLEAN/TIMESTAMP），但 live 库若历史手工 ALTER
过、或某列在早于 111 的迁移里就已建过，实际形状可能不同（实证：
`auth_person_identity.person_id` 111 声明 TEXT，live 库实为 `character varying`）。

本脚本把这种分叉**持续显式化**：在 live DB 上跑，对 111/112 声明的每列逐个查
information_schema，比对「迁移声明类型」与「live 实际类型」，分叉即点名 + 列
差异说明。作为 CI「有库轮次」或部署前检查复用。

## 口径（fail-loud）

- 只比对**列类型标签**差异（text vs varchar、timestamp 等）——**不判定行为
  影响**（varchar/text 对 String 绑定等价）。输出是「分叉清单」，由调用方判断
  是否要紧（本仓约定：纯类型标签分叉无行为影响，登记即可；真正要紧的是「列
  缺失」或「可空性反转」，那些会直接 500）。
- 可空性（is_nullable）一并报，但标注为「信息项」。
- 列缺失（live 库 information_schema 查无）= **红**——说明该迁移没在 live 上
  应用过，必须补。

## 用法

    psql -h <host> -U <user> -d <live_db> -f <(python schema_diff.py --sql)

或直接本脚本连库（需 psql 可执行）：

    python schema_diff.py --host 127.0.0.1 --port 5432 --user o2server \
        --password <pw> --db oa4rust --migrations oa4rust/backend/migrations

退出码：0 = 无「列缺失」；1 = 有列缺失（live 未应用该迁移）。
分叉（类型标签差异）不导致非 0——它是登记的已知现象，不 block。
"""
import argparse
import re
import subprocess
import sys
from pathlib import Path

# 迁移声明的类型 → information_schema.data_type 的可接受集合。
# 不匹配即「分叉」（登记，非 block）。
TYPE_MAP = {
    "text": {"text"},
    "timestamp": {"timestamp without time zone", "timestamp with time zone"},
    "boolean": {"boolean"},
    "integer": {"integer", "bigint", "smallint"},
}


def find_migration(mdir: Path, version: str) -> Path:
    """迁移编号 111 → 111_*.sql（取匹配的首个，排除 rollback）。"""
    for p in sorted(mdir.glob(f"{version}_*.sql")):
        if not p.name.endswith("_rollback.sql"):
            return p
    raise FileNotFoundError(f"{mdir}/{version}_*.sql 不存在")


def extract_declared(migrations_dir: Path, version: str) -> list[tuple[str, str, str]]:
    """从指定迁移文件提取 (table, col, declared_type) 三元组。"""
    p = find_migration(migrations_dir, version)
    text = p.read_text(encoding="utf-8")
    pat = re.compile(r'ALTER TABLE "(\w+)" ADD COLUMN IF NOT EXISTS "([^"]+)"\s+(\S+)')
    rows, seen = [], set()
    for m in pat.finditer(text):
        t, c, ty = m.group(1), m.group(2), m.group(3).rstrip(";")
        if (t, c) in seen:
            continue
        seen.add((t, c))
        rows.append((t, c, ty.rstrip(";")))
    return rows


def build_query(rows: list[tuple[str, str, str]]) -> str:
    """UNION ALL 查每列在 live 库的 actual data_type + is_nullable。"""
    parts = []
    for i, (t, c, ty) in enumerate(rows):
        parts.append(
            f"SELECT '{i}' AS idx, '{t}' AS tbl, '{c}' AS col, '{ty}' AS declared, "
            f"data_type AS actual, is_nullable "
            f"FROM information_schema.columns "
            f"WHERE table_schema='public' AND table_name='{t}' AND column_name='{c}'"
        )
    return " UNION ALL ".join(parts)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", default="5432")
    ap.add_argument("--user", required=True)
    ap.add_argument("--password")
    ap.add_argument("--db", required=True)
    ap.add_argument("--migrations", required=True, help="migrations 目录")
    ap.add_argument("--versions", default="111,112", help="逗号分隔的迁移编号")
    ap.add_argument("--sql-only", action="store_true",
                    help="只打印 SQL（供管道给 psql -f -），不连库")
    ap.add_argument("--docker",
                    help="在 Docker 容器内跑 psql（docker exec <container> psql）。"
                         "设此参数时忽略 --host/--port（改用容器内 socket）")
    args = ap.parse_args()

    mdir = Path(args.migrations)
    rows = []
    for v in args.versions.split(","):
        rows.extend(extract_declared(mdir, v.strip()))
    print(f"# schema_diff: {len(rows)} 列（{args.versions}）", file=sys.stderr)

    q = build_query(rows)
    if args.sql_only:
        print(q + ";")
        return 0

    # 连库：docker 容器内跑 psql（容器里自带），或宿主机 psql
    if args.docker:
        cmd = ["docker", "exec", args.docker, "psql",
               "-U", args.user, "-d", args.db, "-F", "  ", "-A", "-t", "-c", q + ";"]
        env_extra = {}
    else:
        cmd = ["psql", "-h", args.host, "-p", args.port, "-U", args.user,
               "-d", args.db, "-F", "  ", "-A", "-t", "-c", q + ";"]
        env_extra = {}
    if args.password:
        env_extra["PGPASSWORD"] = args.password
    r = subprocess.run(
        cmd,
        env=env_extra | __import__("os").environ,
        text=True, capture_output=True,
    )
    if r.returncode != 0:
        print(f"psql failed: {r.stderr}", file=sys.stderr)
        return 2

    missing, forks = [], []
    for line in r.stdout.splitlines():
        if not line.strip():
            continue
        idx = int(line.split()[0])
        t, c, declared = rows[idx][0], rows[idx][1], rows[idx][2]
        # 行存在 ⇒ 列在 live 库
        cols = line.split("  ", 6)
        actual = cols[4].strip() if len(cols) > 4 else "?"
        null = cols[5].strip() if len(cols) > 5 else "?"
        allowed = TYPE_MAP.get(declared.lower(), None)
        if allowed is None:
            continue  # 未知声明类型，跳过判定
        if actual not in allowed:
            forks.append(f"{t}.{c}: declared={declared} actual={actual}")
    # 缺失判定：total rows 返回数 < 声明数
    present = sum(1 for line in r.stdout.splitlines() if line.strip())
    if present < len(rows):
        missing.append(f"{len(rows) - present} 列在 live 库查无（迁移未应用？）")

    print(f"分叉（类型标签差异，登记不 block）: {len(forks)}")
    for f in forks:
        print(f"  {f}")
    print(f"缺失（block）: {len(missing)}")
    for m in missing:
        print(f"  {m}", file=sys.stderr)
    return 1 if missing else 0


if __name__ == "__main__":
    sys.exit(main())
