#!/bin/bash
# Temporary link-probe: build UML vmlinux WITHOUT gcov to capture the exact
# set of undefined-reference symbols (link symbols are identical with/without
# gcov instrumentation). Used to know precisely which !CONFIG stubs to restore.
set -x
cd /work
rm -rf build-probe
make ARCH=um O=build-probe defconfig >/dev/null 2>&1
scripts/config --file build-probe/.config \
    -e BLK_INLINE_ENCRYPTION -e BLK_DEV_INTEGRITY -e DAX -e SMP \
    -e KUNIT -e DEBUG_FS
make ARCH=um O=build-probe olddefconfig >/dev/null 2>&1
# Promote every tristate =m to =y so all modules are built-in (mirrors the
# coverage build's Step 3c), giving network family real symbols.
sed -i -E 's/^(CONFIG_[A-Za-z0-9_]+)=m$/\1=y/' build-probe/.config
make ARCH=um O=build-probe olddefconfig >/dev/null 2>&1
make ARCH=um O=build-probe vmlinux -j"$(nproc)" 2>build-probe/linkerr.txt
echo "MAKE_EXIT=$?"
grep -E 'undefined reference' build-probe/linkerr.txt | sort -u > build-probe/undefined.txt
echo "=== UNDEFINED SYMBOLS ($(wc -l < build-probe/undefined.txt) unique) ==="
cat build-probe/undefined.txt
