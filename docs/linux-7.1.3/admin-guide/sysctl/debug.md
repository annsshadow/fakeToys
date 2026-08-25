## /proc/sys/debug/


这些文件出现`/proc/sys/debug/`，具体取决于内核配置


## exception-trace


该标志控制内核是否将有关未处理信号（如段错误）的信息打印到内核日志（`dmesg`）

- `0`：不跟踪未处理的信号
- `1`：打印有关未处理信号的信息

默认值在大多数架构（x86、MIPS、RISC-V）上`1`，但**arm64** 上为 `0`

实际打印的信息和提供的上下文CPU 架构而异，差异显著。例如：

- **x86** 上，通常打印指令指针（IP）、错误码以及导致页错误的地址
- **PowerPC** 上，可能打印下一指令指针（NIP）、链接寄存器（LR）以及其它相关寄存器

启用后，该特性通常会被限速，以防止在内核崩溃循环中内核日志被淹没

## kprobes-optimization


该标志启用或禁用某些架构（如 x86）上 Kprobes 的优化

- `0`：关Kprobes 优化
- `1`：开Kprobes 优化（默认）

有关 Kprobes 及其优化的更多细节，请参Documentation/trace/kprobes.rst

Copyright (c) 2026, Shubham Chakraborty <chakrabortyshubham66@gmail.com>

有关一般信息和法律声明，请参阅 Documentation/admin-guide/sysctl/index.rst
