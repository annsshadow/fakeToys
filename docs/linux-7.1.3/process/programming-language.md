
## 编程语言


Linux 内核使用 C 编程语言 [c-language]_ 编写。更准确地说，它通常使用 `gcc` [gcc]_ `-std=gnu11` [gcc-c-dialect-options]_（即 ISO C11 GNU 方言）下编译。`clang` [clang]_ 也受支持；请参阅《使Clang/LLVM 构建 Linuxkbuild_llvm>
该方言包含对语言的许多扩[gnu-extensions]_，其中许多扩展在内核中作为惯例被使用
### 属

内核中广泛使用的扩展之一是属[gcc-attribute-syntax]_。属性允许在不对语言做重大语法变更（例如新增关键字）的情况下，为语言实体（如变量、函数或类型）引入实现自定义的语[n2049]_
在某些情况下，属性是可选的（即不支持它们的编译器仍应生成正确的代码，即使代码更慢或执行的编译期检诊断更少）
内核定义了伪关键字（例如 `__pure`），而不是直接使GNU 属性语法（例如 `__attribute__((__pure__))`），以便进行特性探测以判断哪些可用，并/或缩短代码
更多信息请参`include/linux/compiler_attributes.h`
### Rust


内核`CONFIG_RUST` 下支Rust 编程语言 [rust-language]_。它使用 `rustc` [rustc]_ `--edition=2021` [rust-editions]_ 下编译。Edition（版本）是一种引入与以往不向后兼容的小幅语言变更的方式
在此基础上，内核还使用了一些不稳定特[rust-unstable-features]_。不稳定特性未来可能发生变化，因此达到仅使用稳定特性的状态是一个重要目标
更多信息请参Documentation/rust/index.rst