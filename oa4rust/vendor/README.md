# GNU 工具链 vendor（Windows 本机 Rust 编译支持）

本目录存放**仅本机**使用的 MinGW/MSYS2 工具链二进制，支撑
`stable-x86_64-pc-windows-gnu` 目标在 Windows 上完成 `cargo check/test`
（该目标的 proc-macro 原生链接需要 `dlltool`/`objdump`/`as`/`ld`/`ar`；
cc-rs 编译 C 依赖需要 `gcc`）。

**二进制不入 git**（`mingw-bin/`、`mingw-gcc/` 已 gitignore，合计 ~450MB）；
本 README 记录完整复现方法，任何 Windows 机器可按此重建。

## 复现方法（已实测 2026-10-11）

1. `rustup toolchain install stable --profile minimal`（默认目标即 windows-gnu）。
2. 从 MSYS2 官方镜像下载 pkg.tar.zst（清华源 `mirrors.tuna.tsinghua.edu.cn/msys2/mingw/mingw64/`），
   解包 `mingw64/bin/*` 与 `mingw64/lib/*`：
   - `mingw-w64-x86_64-binutils` → `mingw-bin/`：`dlltool.exe objdump.exe nm.exe
     as.exe ld.bfd.exe(另拷一份为 ld.exe) ar.exe ranlib.exe windres.exe objcopy.exe`
     —— dlltool `-D` 会 CreateProcess 调 objdump，且用 as/ld 链接导入库，缺一不可。
   - `mingw-w64-x86_64-gcc` + 其运行时依赖包（`libgcc`/`libstdc++` 同版本、
     `headers` `crt` `winpthreads` `gmp` `isl` `mpfr` `mpc` `zlib` `zstd`
     `windows-default-manifest`）→ 合并进 `mingw-gcc/`（保留 `mingw64/` 目录结构，
     gcc 驱动按相对路径找 cc1）。
   - gcc 包只提供 `cc.exe`，需 `copy cc.exe gcc.exe`（cc-rs 按名找 `gcc.exe`）。
   - **运行时 DLL 必须版本匹配**：`libgcc_s_seh-1.dll`/`libwinpthread-1.dll` 用
     Git for Windows 自带（`D:\software\Git\mingw64\bin\`，MSYS2 代际）的版本放入
     `mingw-gcc/mingw64/bin/`；混用 rustup 旧 GNU 工具链的 DLL 会触发
     0xC0000139（STATUS_ENTRYPOINT_NOT_FOUND）静默失败。
3. 调用方式（Git Bash 会篡改 MSYS 风格 PATH，必须用原生 cmd 批处理）：

   ```bat
   @echo off
   set PATH=C:\Users\<user>\.cargo\bin;D:\...\oa4rust\vendor\mingw-gcc\mingw64\bin;D:\...\oa4rust\vendor\mingw-bin;D:\...\oa4rust\vendor\mingw-bin\mingw64\bin;%PATH%
   set RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
   cd /d D:\...\oa4rust\backend
   cargo check -p <crate>
   ```

## 实测结果

- `cargo check`（file_assemble_control / program_center / organization_assemble_authentication /
  attendance_assemble_control / parity / mcp_server / openapi）CARGO_EXIT=0
- `cargo test` 无 DB 4563 通过；本地 PG（o2server/oa4rust，`--migrate-only` 379 表）
  后 `cargo test -p parity --lib` 4143/4143。
