# 行覆盖率测量指南（plan002 U9b）

## 前置条件

```powershell
rustup component add llvm-tools-preview
cargo install grcov
```

## 运行

```powershell
cd oa4rust
.\scripts\run-coverage.ps1 -Crates shared,search,cms_assemble_control
# 生成 target\coverage\lcov.info 与 target\coverage\html\index.html
```

## 已知限制

- Windows MSVC 工具链使用 `-C instrument-coverage`，首次会全量重编译（30+ 分钟），之后增量
- `LLVM_PROFILE_FILE` 模式中 `%p`/`%m` 由 llvm-tools 展开，多测试二进制不会互相覆盖
- tarpaulin 官方不支持 Windows，本方案选型 grcov
- 插桩构建与常规开发构建共享 target 目录会导致反复重编译；如需隔离可设 `CARGO_TARGET_DIR=target-cov`

## 基线（2026-08-21）

- 首次基线待 CI 或本地完整跑出后回填此处
- handler 级"路由可达"覆盖已由 tests_generated 体系承担（90 文件）；本测量补充的是**行级**视角
