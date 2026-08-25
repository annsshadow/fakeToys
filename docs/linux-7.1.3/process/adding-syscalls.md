

## 添加新的系统调用


本文档描述了Linux 内核添加一个新的系统调用所涉及的各项工作，超出Documentation/process/submitting-patches.rst <submittingpatches> 中常规的提交建议范围

### 系统调用的替代方

添加新系统调用时要考虑的第一件事，是某个替代方案是否可能更合适。尽管系统调用是用户空间与内核之间最传统、最明显的交互点，但还有其他可能——请选择最适合你的接口的方案
 - 如果所涉及的操作用起来可以像一个类文件系统的对象，那么创建一个新文件系统或设备可能更有意义。这也更容易把新功能封装进一个内核模块，而不必要求将其编入主内核
     - 如果新功能涉及内核通知用户空间某件事已经发生的操作，那么为相关对象返回一个新的文件描述符，可让用户空间使`poll`/`select`/`epoll` 来接收该通知     - 不过，无法映射到 `read(2)`/`write(2)` 这类操作的操作用 `ioctl(2)` 请求来实现，这可能导致一个不太透明API
 - 如果你只是要暴露运行时系统信息，sysfs 中新建一个节点（参见 `Documentation/filesystems/sysfs.rst`）或 `/proc` 文件系统可能更合适。不过，使用这些机制要求相关文件系统已被挂载，而这并不总是成立（例如在命名空间沙箱chroot 的环境中）。避免向 debugfs 添加任何 API，因为它不被视为面向用户空间的“生产”接口 - 如果操作特定于某个文件或文件描述符，那么增加一个新`fcntl(2)` 命令选项可能更合适。不过，`fcntl(2)` 是一个隐藏了大量复杂性的多路复用系统调用，因此该选项最适合新功能与现有 `fcntl(2)` 功能非常相似，或者新功能非常简单的情况（例如获设置一个与文件描述符相关的简单标志） - 如果操作特定于某个任务或进程，那么增加一个新`prctl(2)` 命令选项可能更合适。与 `fcntl(2)` 一样，这个系统调用是一个复杂的多路复用器，因此最好留给与现有 `prctl()` 命令近乎等价的情况，或者获设置与进程相关的简单标志

### 设计 API：为扩展做规

新的系统调用构成内核 API 的一部分，并且必须无限期地得到支持。因此，在内核邮件列表上明确讨论该接口是个非常好的做法，而为接口的未来扩展做规划也很重要
（系统调用表中到处都是没有这样做的历史例子，以及相应的后续系统调用——`eventfd`/`eventfd2`、`dup2`/`dup3`、`inotify_init`/`inotify_init1`、`pipe`/`pipe2`、`renameat`/`renameat2`——所以要吸取内核的历史教训，从一开始就为扩展做规划。）

对于只接受少数几个参数的较简单系统调用，允许未来扩展性的首选方式是给系统调用增加一flags 参数。为了确保用户空间程序能够在不同内核版本之间安全地使flags，需要检flags 值中是否含有任何未知
```

    if (flags & ~(THING_FLAG1 | THING_FLAG2 | THING_FLAG3))
        return -EINVAL;

```
（如果尚未使用任flags 值，则检flags 参数是否为零。）

对于涉及较多参数的更复杂的系统调用，首选方式是将大部分参数封装进一个通过指针传入的结构体中。这样的结构体可以应对未来的扩展

```

    struct xyzzy_params {
        u32 size; /* userspace sets p->size = sizeof(struct xyzzy_params) */
        u32 param_1;
        u64 param_2;
        u64 param_3;
    };

```
只要后续添加的任何字段（例如 `param_4`）在设计上使得零值给出先前的表现，就能应对两种方向的版本不匹配：

 - 为了应对较新的用户空间程序调用较旧内核的情况，内核代码应检查超出其所期望的结构体大小之外的任何内存是否为零（实质上就是检`param_4 == 0`） - 为了应对较旧的用户空间程序调用较新内核的情况，内核代码可以对较小的结构体实例做零扩展（实质上就是设置 `param_4 = 0`）
关于这种做法的例子，参见 `perf_event_open(2)` `perf_copy_attr()` 函数（位`kernel/events/core.c`）

### 设计 API：其他注意事

如果你的新系统调用允许用户空间引用一个内核对象，它应该使用文件描述符作为该对象的句柄——既然内核已经有了使用文件描述符的机制和明确的语义，就不要发明一种新类型的用户空间对象句柄
如果你的xyzzy(2) 系统调用确实返回一个新文件描述符，那么 flags 参数应当包含一个等价于在新 FD 上设`O_CLOEXEC` 的值。这使用户空间能够关`xyzzy()` 与调`fcntl(fd, F_SETFD, FD_CLOEXEC)` 之间的时间窗口，否则另一个线程中意外`fork()` `execve()` 可能将描述符泄漏给被 exec 的程序。（不过，不要贪图复`O_CLOEXEC` 常量的实际数值，因为它是架构相关的，并且属于一个相当拥挤的 `O_*` 标志编号空间的一部分。）

如果你的系统调用返回一个新文件描述符，你还应当考虑在该文件描述符上使用 `poll(2)` 系列系统调用意味着什么。让文件描述符处于可读或可写就绪状态，是内核向用户空间指示相应内核对象上已发生事件的正常方式
```

    int sys_xyzzy(const char __user *path, ..., unsigned int flags);

```
```

    int sys_xyzzyat(int dfd, const char __user *path, ..., unsigned int flags);

```
这为用户空间指定相关文件提供了更大的灵活性；特别是它允许用户空间使用 `AT_EMPTY_PATH` 标志为已经打开的文件描述符请求该功能，实质
```

 - xyzzyat(AT_FDCWD, path, ..., 0) is equivalent to xyzzy(path,...)
 - xyzzyat(fd, "", ..., AT_EMPTY_PATH) is equivalent to fxyzzy(fd, ...)

```
（关\*at() 调用理由的更多细节，参见 `openat(2)` 手册页；关于 AT_EMPTY_PATH 的例子，参见 `fstatat(2)` 手册页。）

如果你的xyzzy(2) 系统调用涉及一个描述文件内偏移量的参数，请将其类型设为 `loff_t`，以便即使在 32 位架构上也能支持 64 位偏移量
如果你的xyzzy(2) 系统调用涉及特权功能，它必须受相应的 Linux capability 位（通过调用 `capable()` 检查）管辖，如 `capabilities(7)` 手册页所述。选择一个管辖相关功能的已有 capability 位，但要尽量避免把许多仅模糊相关的功能归到同一个位下，因为这违背了 capability 拆分 root 权限的初衷。尤其要避免新增对已过度通用`CAP_SYS_ADMIN` capability 的使用
如果你的xyzzy(2) 系统调用操作调用进程之外的另一个进程，应当加以限制（通过调用 `ptrace_may_access()`），使得只有与目标进程具有相同权限、或者具有必capability 的调用进程才能操作目标进程
最后要注意，如果显式为 64 位的系统调用参数落在奇数号参数上（即参数 1），某些x86 架构会更容易处理，以便使用连续的 32 位寄存器对。（如果参数是通过指针传入的结构体的一部分，则不存在此问题。）


### 提交 API 提案


为了让新系统调用易于审查，最好把补丁集拆分成独立的块。这些块至少应包含以下作为独立提交的项（每一项在下面进一步说明）
 - 系统调用的核心实现，连同原型、通用编号、Kconfig 变更以及兜底桩实现 - 为某个特定架构（通常x86，包x86_64、x86_32 x32）接入新系统调用 - 通过 `tools/testing/selftests/` 中的一个自测试来演示新系统调用在用户空间中的用法 - 新系统调用的手册页草稿，可以作为纯文本放在封面信中，也可以作为补丁提交到（独立的）man-pages 仓库
与内API 的任何变更一样，新的系统调用提案应当始终抄送（cc）到 linux-api@vger.kernel.org

### 通用系统调用实现


你的xyzzy(2) 系统调用的主入口点将被称`sys_xyzzy()`，但你应该用相应`SYSCALL_DEFINEn()` 宏来添加这个入口点，而不是显式地添加。其中的 'n' 表示系统调用的参数个数，该宏接受系统调用名，后面跟上作为参数的（类型，名称）对。使用这个宏可以让有关新系统调用的元数据对其他工具可用
新的入口点还需要一个对应的函数原型，位`include/linux/syscalls.h` 中，标记asmlinkage 以匹配系
```

    asmlinkage long sys_xyzzy(...);

```
某些架构（例x86）有它们自己架构特定的系统调用表，但其他一些架构共享一个通用系统调用表。通过在以下列表添加一个条目，将你的新系统调用加入通用列表

```

    #define __NR_xyzzy 292
    __SYSCALL(__NR_xyzzy, sys_xyzzy)

```
还要更新 __NR_syscalls 计数以反映新增的系统调用，并且注意，如果在同一个合并窗口中添加了多个新系统调用，你的新系统调用号可能会被调整以解决冲突
文件 `kernel/sys_ni.c` 为每个系统调用提供一个兜底桩实现
```

    COND_SYSCALL(xyzzy);

```
你的新内核功能以及控制它的系统调用通常应当是可选的，因此为它添加一`CONFIG` 选项（通常`init/Kconfig` 中）。与新增 `CONFIG` 选项的惯例一样：

 - 包含对该选项所控制的新功能和系统调用的说明 - 如果它应该对普通用户隐藏，则让该选项依赖EXPERT - 让任何实现该功能的源文件Makefile 中依赖于CONFIG 选项（例`obj-$(CONFIG_XYZZY_SYSCALL) += xyzzy.o`） - 再次检查在内核关闭该新 CONFIG 选项时仍能正常构建
总结一下，你需要一个包含以下内容的提交
 - `CONFIG` option for the new function, normally in `init/Kconfig`
 - `SYSCALL_DEFINEn(xyzzy, ...)` for the entry point
 - corresponding prototype in `include/linux/syscalls.h`
 - generic table entry in `include/uapi/asm-generic/unistd.h`
 - fallback stub in `kernel/sys_ni.c`


#### Since 6.11


从内核版6.11 开始，针对以下架构的通用系统调用实现不再需要修`include/uapi/asm-generic/unistd.h`
 - arc
 - arm64
 - csky
 - hexagon
 - loongarch
 - nios2
 - openrisc
 - riscv

取而代之，你需要更`scripts/syscall.tbl`，并在适用时调`arch/*/kernel/Makefile.syscalls`
由于 `scripts/syscall.tbl` 充当跨越多个架构的通用系统调用表，

```

    468   common   xyzzy     sys_xyzzy

```
注意，向 `scripts/syscall.tbl` 添加带有 "common" ABI 的条目也会影响共享此表的所有架构。对于更受限或特定于架构的更改，可以考虑使用特定于架构的 ABI 或定义一个新ABI
如果引入一个新ABI，例`xyz`，相应的更新应为

```

    syscall_abis_{32,64} += xyz (...)

```
总结一下，你需要一个包含以下内容的提交
 - `CONFIG` option for the new function, normally in `init/Kconfig`
 - `SYSCALL_DEFINEn(xyzzy, ...)` for the entry point
 - corresponding prototype in `include/linux/syscalls.h`
 - new entry in `scripts/syscall.tbl`
 - (if needed) Makefile updates in `arch/*/kernel/Makefile.syscalls`
 - fallback stub in `kernel/sys_ni.c`


### x86 系统调用实现


要为 x86 平台接入你的新系统调用，你需要更新主系统调用表。假设你的新系统调用在某种程度上不特殊（见下文），这涉及
```

    333   common   xyzzy     sys_xyzzy

```
```

    380   i386     xyzzy     sys_xyzzy

```
同样，这些编号如果在相关合并窗口中出现冲突，很可能会被更改

### 兼容性系统调用（通用

对于大多数系统调用，即使是用 32 位编译的用户空间程序，也可以调用相同64 位实现；即使系统调用的参数包含一个显式指针，这也会被透明地处理
不过，有几种情况需要兼容性层来应32 位与 64 位之间的大小差异
第一种情况是，如64 位内核同时也支持 32 位用户空间程序，因此需要解析可能保32 位或 64 位值的（`__user`）内存区域。特别地，只要系统调用的参数是以下之一，就需要这样做
 - 指向指针的指 - 指向包含指针的结构体的指针（例如 `struct iovec __user *` - 指向大小可变的整型的指针（`time_t`、`off_t`、`long`……）
 - 指向包含大小可变的整型的结构体的指针

需要兼容性层的第二种情况是，如果系统调用的某个参数具有即使在 32 位架构上也显式为 64 位的类型，例`loff_t` `__u64`。在这种情况下，32 位应用程序到64 位内核的值将被拆分成两个 32 位值，随后需要在兼容性层中重新组装
（注意，指向显式 64 位类型的指针作为系统调用参数**需要兼容性层；例如，`splice(2)` 中类型为 `loff_t __user *` 的参数并不会触发`compat_` 系统调用的需求。）

系统调用的兼容性版本称`compat_sys_xyzzy()`，使`COMPAT_SYSCALL_DEFINEn()` 宏添加，类似SYSCALL_DEFINEn。这个实现版本作64 位内核的一部分运行，但期望接收 32 位参数值，并做所需的一切来处理它们。（通常，`compat_sys_` 版本会把值转换为 64 位版本，然后要么调用 `sys_` 版本，要么由它们两者都调用一个公共的内部实现函数。）

兼容性入口点还需要一个对应的函数原型，位`include/linux/compat.h` 中，标记asmlinkage 以匹配系
```

    asmlinkage long compat_sys_xyzzy(...);

```
如果系统调用涉及一个在 32 位和 64 位系统上布局不同的结构体，例`struct xyzzy_args`，那include/linux/compat.h 头文件还应包含一个该结构体的 compat 版本（``struct compat_xyzzy_args``），其中每个可变大小的字段都具有`struct xyzzy_args` 中类型对应的适当`compat_` 类型。`compat_sys_xyzzy()` 例程随后便可以使用这`compat_` 结构体来解析来自 32 位调用的参数
```

    struct xyzzy_args {
        const char __user *ptr;
        __kernel_long_t varying_val;
        u64 fixed_val;
        /* ... */
    };

```
```

    struct compat_xyzzy_args {
        compat_uptr_t ptr;
        compat_long_t varying_val;
        u64 fixed_val;
        /* ... */
    };

```
通用系统调用列表也需要调整以容纳 compat 版本；`include/uapi/asm-generic/unistd.h` 中的条目应当使用

```

    #define __NR_xyzzy 292
    __SC_COMP(__NR_xyzzy, sys_xyzzy, compat_sys_xyzzy)

```
总结一下，你需要：

 - a `COMPAT_SYSCALL_DEFINEn(xyzzy, ...)` for the compat entry point
 - corresponding prototype in `include/linux/compat.h`
 - (if needed) 32-bit mapping struct in `include/linux/compat.h`
 - instance of `__SC_COMP` not `__SYSCALL` in
   `include/uapi/asm-generic/unistd.h`


#### Since 6.11


这适用于“通用系统调用实现”中列出的、除 arm64 之外的所有架Since 6.11<syscall_generic_6_11>。更多信息参Compatibility System Calls (arm64)<compat_arm64>
你需要为 `scripts/syscall.tbl` 中的条目增加一个额外的列，以指示运行在 64 位内核上32 位用户空间程序应
```

    468   common     xyzzy     sys_xyzzy    compat_sys_xyzzy

```
总结一下，你需要：

 - `COMPAT_SYSCALL_DEFINEn(xyzzy, ...)` for the compat entry point
 - corresponding prototype in `include/linux/compat.h`
 - modification of the entry in `scripts/syscall.tbl` to include an extra
   "compat" column
 - (if needed) 32-bit mapping struct in `include/linux/compat.h`



##### 兼容性系统调用（arm64

arm64 上，有一个专用于面向 32 位（AArch32）用户空间的兼容性系统调用的系统调用表：`arch/arm64/tools/syscall_32.tbl`。你需要向此表添加一行，指定 compat

```

    468   common     xyzzy     sys_xyzzy    compat_sys_xyzzy


```
### 兼容性系统调用（x86

要为带有兼容性版本的系统调用接入 x86 架构，需要调整系统调用表中的条目
首先，`arch/x86/entry/syscalls/syscall_32.tbl` 中的条目会获得一个额外的列，以指示运行在 64 位内核上32 位用户空间程
```

    380   i386     xyzzy     sys_xyzzy    __ia32_compat_sys_xyzzy

```
其次，你需要弄清楚新系统调用的 x32 ABI 版本应当如何表现。这里有个选择：参数的布局应当要么匹配 64 位版本，要么匹配 32 位版本
如果涉及指向指针的指针，决定就很简单：x32 ILP32，因此布局应当匹配 32 位版本，并且 `arch/x86/entry/syscalls/syscall_64.tbl` 中的条目会被拆分，使x32 程序命中

```

    333   64       xyzzy     sys_xyzzy
    ...
    555   x32      xyzzy     __x32_compat_sys_xyzzy

```
如果不涉及任何指针，那么最好为 x32 ABI 复用 64 位系统调用（因此 arch/x86/entry/syscalls/syscall_64.tbl 中的条目保持不变）
无论哪种情况，你都应当检查参数布局中所涉及的类型确实能精确地从 x32mx32）映射到 32 位（-m32）或 64 位（-m64）的等价类型

### 在其他位置返回的系统调用


对于大多数系统调用，一旦系统调用完成，用户程序会恰好从它离开的地方继续——即下一条指令处，栈与系统调用前相同，大多数寄存器也相同，并且具有相同的虚拟内存空间
不过，少数系统调用的行为不同。它们可能返回到不同的位置（`rt_sigreturn`），或者改变程序的内存空间（`fork`/`vfork`/`clone`），甚至改变架构（`execve`/`execveat`）
为了支持这一点，系统调用的内核实现可能需要向内核栈保存并恢复额外的寄存器，从而完全控制系统调用之后执行的位置和方式
这是架构相关的，但通常涉及定义汇编入口点，这些入口点保恢复额外的寄存器并调用真正的系统调用入口点
对于 x86_64，这被实现为 `arch/x86/entry/entry_64.S` 中名`stub_xyzzy` 的入口点，而系统调用表中的条目

```

    333   common   xyzzy     stub_xyzzy

```
64 位内核上运行32 位程序的等价物通常称为 `stub32_xyzzy`，在 `arch/x86/entry/entry_64_compat.S` 中实现，相应的系统调用表调整
```

    380   i386     xyzzy     sys_xyzzy    stub32_xyzzy

```
如果系统调用需要一个兼容性层（如上一节所述），那`stub32_` 版本需要调用系统调用的 `compat_sys_` 版本，而不是原生的 64 位版本。此外，如果 x32 ABI 的实现与 x86_64 版本不相同，那么它的系统调用表也需要调用一个会转向 `compat_sys_` 版本的桩
为了完整性，最好也建立一个映射，使用户Linux（User-Mode Linux）仍能工作——它的系统调用表会引stub_xyzzy，但 UML 的构建不包含 `arch/x86/entry/entry_64.S` 的实现（因为 UML 模拟了寄存器等）。修复方法很简单，只需
```

    #define stub_xyzzy sys_xyzzy


```
### 其他细节


内核的大部分以通用方式处理系统调用，但偶尔也有例外，可能需要为你的特定系统调用做更新
审计（audit）子系统就是这样一个特殊情况；它包含（架构相关的）函数，用于对某些特殊类型的系统调用进行分类——具体是文件打开（`open`/`openat`）、程序执行（`execve`/`exeveat`）或套接字多路复用器（`socketcall`）操作。如果你的新系统调用类似于其中之一，那么应当更新审计系统
更一般地说，如果存在与你的新系统调用类似的已有系统调用，值得在内核范围内对该已有系统调用做一grep，以检查是否没有其他特殊情况

### 测试


新的系统调用显然应当被测试；为审查者提供用户空间程序将如何使用该系统调用的演示也很有用。结合这两个目标的一个好办法是，`tools/testing/selftests/` 下的一个新目录中包含一个简单的自测试程序
对于新的系统调用，显然不会有 libc 包装函数，因此测试需要使`syscall()` 来调用它；此外，如果系统调用涉及一个新的用户空间可见的结构体，则需要安装相应的头文件才能编译测试
确保该自测试在所有支持的架构上都能成功运行。例如，检查当它被编译x86_64m64）、x86_32m32）和 x32mx32ABI 程序时都能工作
要对新功能做更广泛和彻底的测试，你还应当考虑把测试添加到 Linux Test Project，或者针对文件系统相关的更改添加xfstests 项目
 - https://linux-test-project.github.io/
 - git://git.kernel.org/pub/scm/fs/xfs/xfstests-dev.git


### 鎵嬪唽椤。

所有新的系统调用都应附带一份完整的手册页，理想情况使用 groff 标记，但纯文本也可以。如果使groff，最好在补丁集的封面邮件中包含一份预渲染ASCII 版手册页，以方便审查者
手册页应当抄送（cc）到 linux-man@vger.kernel.org
更多细节，参https://www.kernel.org/doc/man-pages/patches.html


### 不要在内核中调用系统调用


如上所述，系统调用是用户空间与内核之间的交互点。因此，`sys_xyzzy()` `compat_sys_xyzzy()` 这样的系统调用函数只应从用户空间通过系统调用表调用，而不应从内核的其他地方调用。如果系统调用的功能在内核内部有用、需要在新旧两个系统调用之间共享，或者需要在系统调用与其兼容性变体之间共享，那么它应当通过“辅助（helper）”函数（例如 `ksys_xyzzy()`）来实现。这个内核函数随后可以在系统调用桩（`sys_xyzzy()`）、兼容性系统调用桩（`compat_sys_xyzzy()`）和/或其他内核代码中调用
至少64 x86 上，v4.17 开始，不在内核中调用系统调用函数将是一个硬性要求。它使用不同的系统调用调用约定，其中 `struct pt_regs` 在系统调用包装器中即时解码，然后将处理交给实际的系统调用函数。这意味着系统调用入口处只传递特定系统调用实际需要的那些参数，而不是始终用随机的用户空间内容填满六CPU 寄存器（这可能会在调用链下游造成严重麻烦）
此外，关于数据如何被访问的规则在内核数据与用户数据之间可能不同。这是不应调`sys_xyzzy()` 的另一个原因
这条规则的例外只允许出现在架构相关的覆盖、架构相关的兼容性包装器，或 arch/ 中的其他代码里

### 参考资料与来源


 - Michael Kerrisk 关于系统调用flags 参数用法LWN 文章   https://lwn.net/Articles/585415/
 - Michael Kerrisk 关于系统调用中如何处理未flags LWN 文章   https://lwn.net/Articles/588444/
 - Jake Edge 描述 64 位系统调用参数约束的 LWN 文章   https://lwn.net/Articles/311630/
 - David Drysdale 详细描述 v3.14 系统调用实现路径的一LWN 文章
    - https://lwn.net/Articles/604287/
    - https://lwn.net/Articles/604515/

 - 系统调用的架构特定要求在 `syscall(2)` 手册页中讨论   http://man7.org/linux/man-pages/man2/syscall.2.html#NOTES
 - Linus Torvalds 讨论 `ioctl()` 问题的往来邮件合集：
   https://yarchive.net/comp/linux/ioctl.html
 - "如何不发明内核接，Arnd Bergmann   https://www.ukuug.org/events/linux2007/2007/papers/Bergmann.pdf
 - Michael Kerrisk 关于避免新使CAP_SYS_ADMIN LWN 文章   https://lwn.net/Articles/486306/
 - Andrew Morton 建议新系统调用的所有相关信息应当出现在同一个邮件线程中   https://lore.kernel.org/r/20140724144747.3041b208832bbdf9fbce5d96@linux-foundation.org
 - Michael Kerrisk 建议新系统调用应当附带手册页   https://lore.kernel.org/r/CAKgNAkgMA39AfoSoA5Pe1r9N+ZzfYQNvNPvcRN7tOvRb8+v06Q@mail.gmail.com
 - Thomas Gleixner 建议 x86 接入应放在单独的提交中：
   https://lore.kernel.org/r/alpine.DEB.2.11.1411191249560.3909@nanos
 - Greg Kroah-Hartman 建议新系统调用最好附带手册页和自测试   https://lore.kernel.org/r/20140320025530.GA25469@kroah.com
 - Michael Kerrisk 关于新系统调用与 `prctl(2)` 扩展的讨论：
   https://lore.kernel.org/r/CAHO5Pa3F2MjfTtfNxa8LbnkeeU8=YJ+9tDqxZpw7Gz59E-4AUg@mail.gmail.com
 - Ingo Molnar 建议涉及多个参数的系统调用应将那些参数封装进一个结构体，并包含一个用于未来扩展性的 size 字段   https://lore.kernel.org/r/20150730083831.GA22182@gmail.com
 - 由（重新）使O_* 编号空间标志引起的编号怪象
    - commit 75069f2b5bfb ("vfs: renumber FMODE_NONOTIFY and add to uniqueness
      check")
    - commit 12ed2e36c98a ("fanotify: FMODE_NONOTIFY and __O_SYNC in sparc
      conflict")
    - commit bb458c644a59 ("Safer ABI for O_TMPFILE")

 - Matthew Wilcox 关于 64 位参数限制的讨论   https://lore.kernel.org/r/20081212152929.GM26095@parisc-linux.org
 - Greg Kroah-Hartman 建议应当对未flags 进行管控   https://lore.kernel.org/r/20140717193330.GB4703@kroah.com
 - Linus Torvalds 建议 x32 系统调用应当优先64 位版本而非 32 位版本兼容：
   https://lore.kernel.org/r/CA+55aFxfmwfB7jbbrXxa=K7VBYPfAvmu3XOkGrLbB1UFjX1+Ew@mail.gmail.com
 - 修改系统调用表基础设施以在多个架构上使scripts/syscall.tbl 的补丁系列：
   https://lore.kernel.org/lkml/20240704143611.2979589-1-arnd@kernel.org
