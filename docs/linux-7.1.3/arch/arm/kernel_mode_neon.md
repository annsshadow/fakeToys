## 内核模式 NEON

### 速览总结（TL;DR

- 仅使NEON 指令，或不依赖支持代码的 VFP 指令
- 把你NEON 代码隔离在一个单独的编译单元中，并用 '-march=armv7-a -mfpu=neon -mfloat-abi=softfp' 编译
- 在调用你NEON 代码处前后加kernel_neon_begin() kernel_neon_end() 调用
- 不要在你NEON 代码中睡眠，并要注意它会在禁用抢占的情况下执

### 引言

在运行于内核模式的代码中可以使用 NEON 指令（以及在某些情况下使VFP 指令）。但是出于性能原因，NEON/VFP 寄存器文件并不像普通寄存器文件那样在每次上下文切换或发生异常时都被保存和恢复，因此需要一些人工干预。此外，对于可能会睡眠的代码（即可能会调schedule()），需要特别小心，因为出于下文所述的原因，NEON VFP 指令会在一个不可抢占的区段中执行

### 惰性保存与恢复

NEON/VFP 寄存器文件是通过惰性保存（UP 系统上）和惰性恢复（SMP UP 系统上）来管理的。这意味着该寄存器文件保持“活跃（live）”，并且仅当多个任务争用 NEON/VFP 单元（或者在 SMP 情况下，当一个任务迁移到另一个核心）时才会被保存和恢复。惰性恢复是这样实现的：在每次上下文切换后禁NEON/VFP 单元，于是当随后发出一NEON/VFP 指令时会产生一个陷阱，让内核介入并在必要时执行恢复

任何在内核模式中NEON/VFP 单元的使用都不应干扰这一点，因此需要“主动（eager）”保NEON/VFP 寄存器文件，并显式地启用 NEON/VFP 单元，这样在后续首次使用时就不会产生异常。这是由函数 kernel_neon_begin() 处理的，它应在发出任何内核模式的 NEON VFP 指令之前被调用。同样，使用之后应当再次禁用 NEON/VFP 单元，以确保用户模式在下次使用时会命中惰性恢复陷阱。这由函kernel_neon_end() 处理

### 内核模式中的中断

出于性能和简洁的考虑，决定对内核模式NEON/VFP 寄存器内容不设置保存/恢复机制。这意味着只有当能保证不触NEON/VFP 寄存器时，才允许中断内核模式NEON 区段。因此，内核中适用以下规则和限制：
- 中断上下文中不允NEON/VFP 代码
- NEON/VFP 代码不允许睡眠；
- NEON/VFP 代码在禁用抢占的情况下执行

如果在意延迟，可以在代码中没有任NEON 寄存器处于活跃状态的那些位置，把kernel_neon_end() kernel_neon_begin() 的调用紧挨着放置。（如果没有在此期间发生上下文切换，额外kernel_neon_begin() 调用应当相当廉价。）

### VFP 与支持代

较早版本VFP（版3 之前）依赖软件支持来实现诸如 IEEE-754 兼容的下溢处理等功能。当 VFP 单元需要这种软件协助时，它会通过引发一个未定义指令异常来向内核发出信号。内核会检VFP 控制寄存器以及当前指令和参数，并在软件中模拟该指令

目前在内核模式下执行VFP 指令尚未实现这种软件协助。如果遇到这种情况，内核会失败并生成一OOPS

### NEON 代码与普通代码分

编译器并不知kernel_neon_begin() kernel_neon_end() 的特殊意义，即只允许在分别对这些函数的两次调用之间发NEON/VFP 指令。此外，如果选择-mfpu=neon，GCC -O3 级别可能会自己生NEON 指令，而且即便内核目前-O2 编译，如果不在意的话，未来的变更也可能导NEON/VFP 指令出现在意想不到的地方

因此，在内核中使NEON/VFP 的推荐且唯一受支持的方式是遵循以下规则：

- NEON 代码隔离在一个单独的编译单元中，并用 '-march=armv7-a -mfpu=neon -mfloat-abi=softfp' 编译它；
- 从一*没有**设置 GCC 标志 '-mfpu=neon' 的编译单元中，发出对 kernel_neon_begin()、kernel_neon_end() 的调用，以及对包NEON 代码的那个单元的调用

由于内核是用 '-msoft-float' 编译的，上述做法将保证在任何优化级别下，NEON VFP 指令都只会出现在指定的编译单元中

### NEON 汇编

只要遵循上述规则，NEON 汇编是受支持的，没有额外的注意事项

### GCC 生成NEON 代码

GCC 选项 -ftree-vectorize（由 -O3 隐含）试图利用隐式并行性，并从普C 源代码生NEON 代码。只要遵循上述规则，这是完全受支持的

### NEON 内建函数（intrinsics

NEON 内建函数也是受支持的。不过，由于使用 NEON 内建函数的代码依赖于 GCC 头文<arm_neon.h>（它 #include <stdint.h>），除了上述规则外你还应遵守以下事项

- '-ffreestanding' 编译包含 NEON 内建函数的单元，这样 GCC 会使用其内建<stdint.h> 版本（这是一个内核不提供C99 头文件）
- 最后再包含 <arm_neon.h>，或至少放在 <linux/types.h> 之后
