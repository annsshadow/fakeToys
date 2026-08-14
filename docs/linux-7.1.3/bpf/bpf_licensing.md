## BPF 许可


## 背景


- 经典 BPF（Classic BPF）采用 BSD 许可

“BPF”最初作为 BSD Packet Filter 在
http://www.tcpdump.org/papers/bpf-usenix93.pdf 中被引入。相应的指令集及其
实现随 BSD 以 BSD 许可证一同而来。该原始指令集现在被称为“经典 BPF”。

然而，指令集是对机器语言交互的一种规范，类似于一种编程语言。它并不是代码。
因此，BSD 许可的应用在某些语境下可能产生误导，因为该指令集可能不受版权保护。

- eBPF（扩展 BPF）指令集继续采用 BSD 许可

2014 年，经典 BPF 指令集被大幅扩展。我们通常称此指令集为 eBPF，以区别于 cBPF。
eBPF 指令集仍然采用 BSD 许可。

## eBPF 的实现


使用 eBPF 指令集需要在内核空间和用户空间都实现代码。

### 在 Linux 内核中


eBPF 解释器以及各种即时（JIT）编译器的参考实现是 Linux 的一部分，并采用
GPLv2 许可。eBPF 辅助函数（helper）的实现也采用 GPLv2 许可。解释器、JIT、
辅助函数和验证器（verifier）被称为 eBPF 运行时（runtime）。

### 在用户空间中


也存在采用以下许可证的 eBPF 运行时（解释器、JIT、辅助函数）实现：
Apache2 (https://github.com/iovisor/ubpf)、
MIT (https://github.com/qmonnet/rbpf)，以及
BSD (https://github.com/DPDK/dpdk/blob/main/lib/librte_bpf)。

### 在硬件中


硬件可以选择原生执行 eBPF 指令，并通过硬件或在固件中以专有许可实现 eBPF 运行时。

### 在其他操作系统中


其他内核或用户空间对 eBPF 指令集与运行时的实现可以采用专有许可。

## 在 Linux 内核中使用 BPF 程序


Linux 内核（虽然是 GPLv2）允许在这些规则下链接专有内核模块：
Documentation/process/license-rules.rst

当加载一个内核模块时，Linux 内核会检查它打算使用哪些函数。如果函数被标记为
“仅 GPL”，相应的模块或程序就必须具有与 GPL 兼容的许可证。

把 BPF 程序加载进 Linux 内核类似于加载一个内核模块。BPF 在运行时加载，而
不是静态链接到 Linux 内核。BPF 程序加载遵循与内核模块相同的许可证检查规则。
如果 BPF 程序不使用“仅 GPL”的 BPF 辅助函数，它们可以是专有的。

此外，某些 BPF 程序类型——Linux 安全模块（LSM）和 TCP 拥塞控制（struct_ops），
截至 2021 年 8 月——即使不直接使用“仅 GPL”的辅助函数，也要求与 GPL 兼容。
Linux 内核中 LSM 和 TCP 拥塞控制模块注册步骤是通过 EXPORT_SYMBOL_GPL 内核
函数完成的。从这个意义上说，LSM 和 struct_ops 的 BPF 程序是在隐式地调用
“仅 GPL”的函数。同样限制也适用于通过不稳定接口（也称为“kfunc”）直接调用
内核函数的 BPF 程序。

## 把 BPF 程序与用户空间应用打包在一起


一般来说，专有许可的应用和为 Linux 内核编写的 GPL 许可 BPF 程序放在同一个包中
可以共存，因为它们是独立的、可执行的进程。这既适用于 cBPF 也适用于 eBPF 程序。
