#!/usr/bin/env bash
# 修复本机 Rust MSVC 构建环境。
# 根因：Git Bash 的 /usr/bin/link (GNU coreutils) 抢先于 MSVC link.exe，
#       导致 cargo 报 "link: extra operand"；即使找到 MSVC link.exe，
#       还缺 LIB 环境变量指向 MSVC/SDK 的 .lib 目录（否则 LNK1181 kernel32.lib）。
#
# 用法： source scripts/rust-env.sh   （必须 source，才能导出到当前 shell）

MSVC_DIR="/c/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.44.35207"
SDK_LIB="/c/Program Files (x86)/Windows Kits/10/Lib/10.0.26100.0"

export PATH="$MSVC_DIR/bin/Hostx64/x64:$PATH"
export LIB="$(cygpath -w "$MSVC_DIR/lib/x64");$(cygpath -w "$SDK_LIB/ucrt/x64");$(cygpath -w "$SDK_LIB/um/x64")"

echo "[rust-env] link -> $(which link)"
echo "[rust-env] LIB  = $LIB"

# 注意：cargo test 时请加 -j 1。
# 并行编译 ring 等含 C 代码的 crate 时，多个 cl.exe 会并发写同一个
# target/*/build/*/out/ 目录，Windows 上会报
#   fatal error C1083: Cannot open compiler generated file: ... Permission denied
# cargo check 不受影响（不编译 C、不链接）。
