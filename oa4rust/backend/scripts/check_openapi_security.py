#!/usr/bin/env python3
"""
check_openapi_security.py — OpenAPI 安全方案守卫（plan002 U9 / R8）。

`scripts/gen_openapi_paths.py` 在重新生成 `crates/openapi/src/lib.rs` 时，
会通过 `SecurityAddon` 把 Bearer 鉴权方案（`bearer_token`）注入到 OpenAPI
的 securitySchemes 中。若该注入在重新生成时静默丢失，API 文档将缺少鉴权
信息，且难以察觉。

本脚本校验已纳入版本控制的生成产物（openapi/src/lib.rs）确实含有安全方案
注入，防止回归。CI 在“重新生成 OpenAPI”步骤之后应再调用一次本脚本
（当前 CI 未自动重新生成，故直接校验已提交的产物）。

退出码：
  0  产物含完整安全方案注入
  1  产物缺失安全方案注入（应修复 gen_openapi_paths.py 后重新生成）
"""
from pathlib import Path
import sys

OA4RUST = Path(__file__).resolve().parent.parent
LIB_RS = OA4RUST / "crates" / "openapi" / "src" / "lib.rs"

# 必须同时存在的标记，缺一不可认为注入完整。
REQUIRED_MARKERS = [
    ("SecurityAddon 结构体", "pub struct SecurityAddon"),
    ("SecurityAddon 实现 utoipa::Modify", "impl utoipa::Modify for SecurityAddon"),
    ("add_security_scheme 调用", "add_security_scheme"),
    ("安全方案名称 bearer_token", '"bearer_token"'),
    ("Bearer 鉴权方案", "HttpAuthScheme::Bearer"),
    ("OpenApi derive 应用 SecurityAddon", "modifiers(&SecurityAddon)"),
]


def main() -> int:
    if not LIB_RS.exists():
        print(f"::error::找不到生成产物 {LIB_RS}", file=sys.stderr)
        return 1

    content = LIB_RS.read_text(encoding="utf-8")
    missing = [name for name, marker in REQUIRED_MARKERS if marker not in content]

    if missing:
        print("::error::OpenAPI 产物缺少安全方案注入（securitySchemes 可能静默丢失）：")
        for m in missing:
            print(f"  - 缺失: {m}")
        print("请修复 scripts/gen_openapi_paths.py 并重新生成 "
              "crates/openapi/src/lib.rs，再提交产物。")
        return 1

    print("OK: OpenAPI 产物含完整 Bearer 安全方案注入（securitySchemes 完好）。")
    return 0


if __name__ == "__main__":
    sys.exit(main())
