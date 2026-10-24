#!/usr/bin/env bash
# 修复本机 Rust MSVC 构建环境。
# 根因：Git Bash 的 /usr/bin/link (GNU coreutils) 抢先于 MSVC link.exe，
#       导致 cargo 报 "link: extra operand"；即使找到 MSVC link.exe，
#       还缺 LIB 环境变量指向 MSVC/SDK 的 .lib 目录（否则 LNK1181 kernel32.lib）。
#
# 用法： source scripts/rust-env.sh   （必须 source，才能导出到当前 shell）

MSVC_DIR="/c/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.44.35207"
SDK_LIB="/c/Program Files (x86)/Windows Kits/10/Lib/10.0.26100.0"

# fail-loud：版本号是硬编码的，VS/SDK 升级后目录会变。曾经的脚本对不存在的
# 路径静默照常 export，症状是下游 link.exe 找不到、LNK1181——排障时很难想到
# 根因在「这个脚本指向了一个已被卸载的版本目录」（2026-10-24 加）。
for d in "$MSVC_DIR" "$SDK_LIB"; do
  if [ ! -d "$d" ]; then
    echo "[rust-env] 目录不存在：$d" >&2
    echo "[rust-env] VS/SDK 版本可能已升级。请把上面 MSVC_DIR / SDK_LIB 改成实际版本，" >&2
    echo "[rust-env] 可用 'ls \"/c/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/\"' 查看。" >&2
    return 1
  fi
done

export PATH="$MSVC_DIR/bin/Hostx64/x64:$PATH"
export LIB="$(cygpath -w "$MSVC_DIR/lib/x64");$(cygpath -w "$SDK_LIB/ucrt/x64");$(cygpath -w "$SDK_LIB/um/x64")"

echo "[rust-env] link -> $(which link)"
echo "[rust-env] LIB  = $LIB"

# 注意：cargo test 时请加 -j 1。
# 并行编译 ring 等含 C 代码的 crate 时，多个 cl.exe 会并发写同一个
# target/*/build/*/out/ 目录，Windows 上会报
#   fatal error C1083: Cannot open compiler generated file: ... Permission denied
# cargo check 不受影响（不编译 C、不链接）。
#
# 建议同时设 CARGO_INCREMENTAL=0。
# 沙箱环境下增量编译目录（target/debug/incremental/）会因文件权限报
#   error: failed to remove ...rcgu.o: 拒绝访问。 (os error 5)
# 或让 rustc 直接 panic（exit code 101，"query stack during panic"）。
# 禁用增量可稳定规避；代价是重编译稍慢。
export CARGO_INCREMENTAL=0
