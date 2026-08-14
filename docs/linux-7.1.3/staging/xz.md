
## Linux 中的 XZ 数据压缩


## 简介


XZ 是一种具有高压缩比的通用的数据压缩格式。Linux 中的 XZ 解压器称为 XZ Embedded。
它支持 LZMA2 过滤器，以及可选的用于可执行代码的 Branch/Call/Jump（BCJ）过滤器。
支持 CRC32 用于完整性校验。

最新的版本请参见 `XZ Embedded`_ 主页，其中包含 Linux 内核不需要的一些可选额外特性，
以及在内核之外使用该代码的信息。

对于用户空间，`XZ Utils`_ 提供了一个类似 zlib 的压缩库和一个类似 gzip 的命令行工具。


## 内核中与 XZ 相关的组件


xz_dec 模块在 include/linux/xz.h 中提供带有单次调用（buffer 到 buffer）与
多次调用（有状态）API 的 XZ 解压器。

为解压内核镜像、initramfs 与 initrd，在 lib/decompress_unxz.c 中有一个包装函数。
其 API 与其他 decompress_*.c 文件相同，定义于 include/linux/decompress/generic.h。

对于内核 makefile，提供了三个可与 `$(call if_changed)` 配合使用的命令。它们需要
XZ Utils 中的 xz 工具。

- `$(call if_changed,xzkern)` 用于压缩内核镜像。它运行脚本 scripts/xz_wrap.sh，
  该脚本使用架构优化选项和一个大尺寸的 LZMA2 字典。

- `$(call if_changed,xzkern_with_size)` 与上方的 `xzkern` 类似，但还会追加一个
  包含文件未压缩大小的四字节尾部。某些架构的引导代码需要该尾部。

- 其他内容可以用 `$(call if_needed,xzmisc)` 压缩，它将不使用 BCJ 过滤器，并使用
  1 MiB 的 LZMA2 字典。

## 关于压缩选项的说明


由于 XZ Embedded 只支持带有 CRC32 或无完整性校验的数据流，请确保在编码那些应由
内核解码的文件时不要使用其他类型的完整性校验。使用 XZ Utils 的 liblzma 时，编码时需
使用 `LZMA_CHECK_CRC32` 或 `LZMA_CHECK_NONE`。使用 `xz` 命令行工具时，使用
`--check=crc32` 或 `--check=none` 来覆盖默认的 `--check=crc64`。

强烈建议使用 CRC32，除非另有其他层会验证未压缩数据的完整性。重复校验完整性可能会
浪费 CPU 周期。注意，头部总会带有一个 CRC32，并由解码器校验；你只能为实际的未压缩
数据更改（或禁用）完整性校验类型。

在用户空间中，LZMA2 通常与几兆字节大小的字典配合使用。解码器需要将该字典放在 RAM 中：

- 在多次调用模式下，字典作为解码器状态的一部分分配。在内核内使用的合理最大字典大小
  取决于目标硬件：对桌面系统是几兆字节，而对某些嵌入式系统 64 KiB 到 1 MiB 可能
  更合适。

- 在单次调用模式下，输出缓冲区被用作字典缓冲区。也就是说，字典的大小完全不影响
  解压器的内存占用。只分配基础数据结构，占用略少于 30 KiB 的内存。为获得最佳压缩效果，
  字典至少应与未压缩数据一样大。单次调用模式的一个显著例子是解压内核本身（PowerPC 除外）。

XZ Utils 中的压缩预设在为内核创建文件时可能并非最优，因此不要犹豫，使用自定义设置，
例如设置字典大小。此外，xz 在单线程模式下可能产生更小的文件，因此建议显式设置该选项。
```

    xz --threads=1 --check=crc32 --lzma2=dict=512KiB inputfile

```
## xz_dec API


可通过 `#include <linux/xz.h>` 使用。
