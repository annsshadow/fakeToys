
## Clang 实现说明


本文档提供了特定eBPF 指令集的 Clang/LLVM 实现的更多详细信息

## 版本


Clang 定义了“CPU”版本，其中 CPU 版本 3 对应于当前的 eBPF ISA

Clang 可以使用0000”选择 eBPF ISA 版本，例如选择版本 3

## 算术指令


对于 3 之前CPU 版本，Clang v7.0 及更高版本可以启`BPF_ALU` 支持
`-Xclang -target-feature -Xclang +alu32` CPU 版本 3 中，自动包含支持

## 跳转指令


如果使用`-O0`，Clang将生成`BPF_CALL | BPF_X | BPF_JMP`x8d
指令，Linux 内核验证器不支持该指令

## 原子操作


当`-mcpu=v3`为时，Clang可以默认生成原子指令
已启用。如果设置了较低版本的`-mcpu`，则唯一的原子指
Clang 可以生成的是 `BPF_ADD` **没有** `BPF_FETCH`。如果您需要启
原子功能，同时保持较低的 `-mcpu` 版本，您可以使用
`-Xclang -target-feature -Xclang +alu32`銆。
