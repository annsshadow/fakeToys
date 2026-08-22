
## 使用 Clang/LLVM 构建 Linux


本文档介绍如何使Clang LLVM 工具构建 Linux 内核
### 简

Linux 内核历来都是使用 GNU 工具链（GCC binutils）编译的。持续的努力已经使得 `Clang <https://clang.llvm.org/>`_ `LLVM <https://llvm.org/>`_ 工具可以作为可行的替代方案使用。诸`Android <https://www.android.com/>`_、`ChromeOS <https://www.chromium.org/chromium-os>`_、`OpenMandriva <https://www.openmandriva.org/>`_ `Chimera Linux <https://chimera-linux.org/>`_ 等发行版使用 Clang 构建的内核。Google Meta 的数据中心集群也运行Clang 构建的内核
`LLVM 是一组以 C++ 对象形式实现的工具链组件 <https://www.aosabook.org/en/llvm.html>`_。Clang LLVM 的前端，支持内核所需C 语言以及 GNU C 扩展，其发音"klang"，而不"see-lang"
### 使用 LLVM 构建


```
make LLVM=1
```
```
make LLVM=1 ARCH=arm64
```
LLVM 提供GNU binutils 工具的替代品。可以通过以下方式启用它们```
make CC=clang LD=ld.lld AR=llvm-ar NM=llvm-nm STRIP=llvm-strip \
  OBJCOPY=llvm-objcopy OBJDUMP=llvm-objdump READELF=llvm-readelf \
  HOSTCC=clang HOSTCXX=clang++ HOSTAR=llvm-ar HOSTLD=ld.lld
```
`LLVM=1` 即展开为上述命令
如果你的 LLVM 工具不在 PATH 中，可以提供它们的路径：
```
make LLVM=/path/to/llvm/
```
这将使用 `/path/to/llvm/clang`、`/path/to/llvm/ld.lld` 等。以下方式：
```
PATH=/path/to/llvm:$PATH make LLVM=1
```
如果你的 LLVM 工具带有版本后缀，并且想使用该显式版本，而不是像 `LLVM=1` 那样使用不带后缀的可执行文件，可以：
```
make LLVM=-14
```
这将使用 `clang-14`、`ld.lld-14` 等
为了支持树外路径与版本后缀的组合，可以```
PATH=/path/to/llvm/:$PATH make LLVM=-14
```
如果使用相互独立的命令进行配置和构建，那么每次调`make` 时都应设置与 `LLVM=` 相同的值。在运行最终会执行 `make` 的脚本时，`LLVM=` 也应作为环境变量进行设置
### 交叉编译


单一Clang 编译器二进制文件（以及相应的 LLVM 工具）通常包含所有受支持的后端，这有助于简化交叉编译，尤其是在使用 `LLVM=1` 时。如果只使用 LLVM 工具，可以：
```
make LLVM=1 ARCH=arm64
```
下面是一个混合使LLVM GNU 工具的例子，对于`ARCH=s390` 这样尚不支持 `ld.lld` `llvm-objcopy` 的目标，可以```
make LLVM=1 ARCH=s390 LD=s390x-linux-gnu-ld.bfd \
  OBJCOPY=s390x-linux-gnu-objcopy
```
该示例将调用 `s390x-linux-gnu-ld.bfd` 作为链接器以`s390x-linux-gnu-objcopy`，因此请确保它们可在你的 `$PATH` 中找到
`CROSS_COMPILE` 不会像未设置 `LLVM=1` 时为 GNU 工具所做的那样，作Clang 编译器二进制文件（或相应LLVM 工具）的前缀
### LLVM_IAS= 参数


Clang 能够汇编汇编代码。你可以传入 `LLVM_IAS=0` 来禁用该行为，让 Clang 调用对应的非集成汇编器：
```
make LLVM=1 LLVM_IAS=0
```
在交叉编译并使用 `LLVM_IAS=0` 时，必须使用 `CROSS_COMPILE` 来为编译器设`--prefix=`，以便找到对应的非集成汇编器（通常你并不想使用```
make LLVM=1 ARCH=arm LLVM_IAS=0 CROSS_COMPILE=arm-linux-gnueabi-

```

### Ccache


`ccache` 可以`clang` 配合使用以改善后续构建（不过在多次构建之间，KBUILD_BUILD_TIMESTAMP_ 应设置为确定值）```
KBUILD_BUILD_TIMESTAMP='' make LLVM=1 CC="ccache clang"
```

### 受支持的架构


LLVM 并未Linux 支持的所有架构为目标，仅仅因为某个目标在 LLVM 中受支持，也并不意味着内核能够毫无问题地构建或运行。下面是目前可以使用 `CC=clang` `LLVM=1` 正常工作的架构的总体概述。支持级别对应于 MAINTAINERS 文件中的 "S" 值。如果某个架构未列出，则意味着 LLVM 并未以其为目标，或者存在已知问题。使LLVM 的最新稳定版本甚至开发分支通常能获得最佳结果。某个架构的 `defconfig` 通常预期能良好工作，但某些配置可能仍存在尚未发现的问题。欢迎在下面issue 追踪器中提交 bug 报告
   :widths: 10 10 10
   :header-rows: 1

   - - 架构
     - 支持级别
     - `make` 命令
   - - arm
     - 鍙楁敮鎸?     - `LLVM=1`
   - - arm64
     - 鍙楁敮鎸?     - `LLVM=1`
   - - hexagon
     - 维护     - `LLVM=1`
   - - loongarch
     - 维护     - `LLVM=1`
   - - mips
     - 维护     - `LLVM=1`
   - - powerpc
     - 维护     - `LLVM=1`
   - - riscv
     - 鍙楁敮鎸?     - `LLVM=1`
   - - s390
     - 维护     - `LLVM=1` (LLVM >= 18.1.0), `CC=clang` (LLVM < 18.1.0)
   - - sparc (sparc64 only)
     - 维护     - `CC=clang LLVM_IAS=0` (LLVM >= 20)
   - - um (User Mode)
     - 维护     - `LLVM=1`
   - - x86
     - 鍙楁敮鎸?     - `LLVM=1`

### 获取帮助


- `网站 <https://clangbuiltlinux.github.io/>`_
- `邮件列表 <https://lore.kernel.org/llvm/>`_: <llvm@lists.linux.dev>
- `旧邮件列表归<https://groups.google.com/g/clang-built-linux>`_
- `Issue 杩借釜鍣?<https://github.com/ClangBuiltLinux/linux/issues>`_
- IRC: #clangbuiltlinux on irc.libera.chat
- `Telegram <https://t.me/ClangBuiltLinux>`_: @ClangBuiltLinux
- `维基 <https://github.com/ClangBuiltLinux/linux/wiki>`_
- `新手 Bug <https://github.com/ClangBuiltLinux/linux/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22>`_

### 获取 LLVM


我们`kernel.org <https://kernel.org/pub/tools/llvm/>`_ 上提供预构建的稳定版LLVM。这些版本已使用 profile 数据进行优化以构Linux 内核，相比其LLVM 发行版应能改善内核构建时间
下面列出的一些链接可能有助于从源码构LLVM，或通过发行版的包管理器获取 LLVM
- https://releases.llvm.org/download.html
- https://github.com/llvm/llvm-project
- https://llvm.org/docs/GettingStarted.html
- https://llvm.org/docs/CMake.html
- https://apt.llvm.org/
- https://www.archlinux.org/packages/extra/x86_64/llvm/
- https://github.com/ClangBuiltLinux/tc-build
- https://github.com/ClangBuiltLinux/linux/wiki/Building-Clang-from-source
- https://android.googlesource.com/platform/prebuilts/clang/host/linux-x86/
