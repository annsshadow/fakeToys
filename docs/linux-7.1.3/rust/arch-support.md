
## 架构支持


目前，Rust 编译器（`rustc`）使用 LLVM 进行代码生成，这限制了可以
定位的受支持架构。此外，使用 LLVM/Clang 构建内核的支持程度各不相同
（请参阅 Documentation/kbuild/llvm.rst）。`bindgen` 需要使用 LLVM/Clang，
因此也需要该支持。

以下是当前可用架构的总体概述。支持级别对应于 `MAINTAINERS` 文件中的
`S` 值。

=============  ================  ==============================================
架构            支持级别          约束
=============  ================  ==============================================
`arm`        Maintained        仅 ARMv7 小端。
`arm64`      Maintained        仅小端。
`loongarch`  Maintained        \-
`riscv`      Maintained        `riscv64` 且仅 LLVM/Clang。
`um`         Maintained        \-
`x86`        Maintained        仅 `x86_64`。
=============  ================  ==============================================
