
## 控制流强制技术（CET）影子栈


## CET 背景


控制流强制技术（CET）涵盖若干相关的 x86 处理器特性，用于抵御控制流劫持攻击。CET 可以同时保护应用程序和内核
CET 引入了影子栈（shadow stack）和间接分支跟踪（IBT）。影子栈是从内存中分配的一个辅助栈，应用程序无法直接修改它。执CALL 指令时，处理器将返回地址同时压入普通栈和影子栈。在函数返回时，处理器弹出影子栈副本并将其与普通栈副本比较。如果两者不同，处理器会引发控制保护错误（control-protection fault）。IBT 验证间接 CALL/JMP 目标确实是编译器'ENDBR' 操作码标记的意图目标。并非所CPU 都同时具有影子栈和间接分支跟踪。目前在 64 位内核中，仅支持用户空间影子栈和内核 IBT
## 使用影子栈的要求


要使用用户空间影子栈，你需要支持它的硬件、配置了该功能的内核，以及用该功能编译的用户空间库
内核 Kconfig 选项X86_USER_SHADOW_STACK。编译进内核后，可以通过内核参数 nousershstk 在运行时禁用影子栈
要构建启用了用户影子栈的内核，需Binutils v2.29 LLVM v6 或更高版本
在运行时，如果处理器支持 CETproc/cpuinfo 会显CET 特性user_shstk" 表示当前内核和硬件支持用户空间影子栈
## 应用程序启用


应用程序CET 能力标记在其 ELF note 中，可以通过以下方式验证
```

    readelf -n <application> | grep -a SHSTK
        properties: x86 feature: SHSTK

```
内核不直接处理这些应用程序标记。应用程序或加载器必须使用第 4 节描述的接口来启CET 特性。这通常会在动态加载器或静态运行时对象中完成，GLIBC 就是如此
## 启用 arch_prctl()


Elf 特性应由加载器使用以下 arch_prctl 启用。它们仅64 位用户应用程序中受支持。这些操作以每个线程为基础作用于特性。启用状态在 clone 时继承，因此如果特性在第一个线程上启用，它将传播到应用程序中的所有线程
arch_prctl(ARCH_SHSTK_ENABLE, unsigned long feature)
    启用 'feature' 中指定的单个特性。一次只能操作一个特性
arch_prctl(ARCH_SHSTK_DISABLE, unsigned long feature)
    禁用 'feature' 中指定的单个特性。一次只能操作一个特性
arch_prctl(ARCH_SHSTK_LOCK, unsigned long features)
    将特性锁定为当前的启用或禁用状态features' 是要锁定的所有特性的掩码。所有置位的位都会被处理，未置位的位被忽略。该掩码与现有值进OR 运算。因此此处置位的任何特性位之后都无法被启用或禁用
arch_prctl(ARCH_SHSTK_UNLOCK, unsigned long features)
    解锁特性features' 是要解锁的所有特性的掩码。所有置位的位都会被处理，未置位的位被忽略。仅通过 ptrace 生效
arch_prctl(ARCH_SHSTK_STATUS, unsigned long addr)
    将当前已启用的特性复制到 addr 传入的地址。特性由 'features' 中传入其他调用的位来描述
返回值如下。成功时返回 0。出错时，errno 可能```

        -EPERM 如果传入的任何特性被锁定        -ENOTSUPP 如果硬件或内核不支持该特性        -EINVAL 参数错误（不存在的特性等        -EFAULT 如果无法将信息复制回用户空间

```
```

    ARCH_SHSTK_SHSTK - 褰卞瓙鏍?    ARCH_SHSTK_WRSS  - WRSS

```
目前通过此接口支持影子栈WRSS。WRSS 只能与影子栈一起启用，并且在影子栈被禁用时会自动禁用
## Proc 状

要检查一个应用程序是否确实在影子栈下运行，用户可以读/proc/$PID/status。它会报"wrss" "shstk"
```

    x86_Thread_features: shstk wrss
    x86_Thread_features_locked: shstk wrss

```
## 影子栈的实现


### 褰卞瓙鏍堝ぇ灏?

任务的影子栈从内存中分配，固定大小为 MIN(RLIMIT_STACK, 4 GB)。换句话说，影子栈按普通栈的最大大小分配，但上限为 4 GB。对clone3 系统调用，会传入一个栈大小，影子栈使用此值而非 rlimit
### 信号


主程序及其信号处理程序使用同一个影子栈。由于影子栈只存储返回地址，一个大的影子栈可以覆盖程序栈和信号备用栈同时耗尽的情况
当发生信号时，信号的旧前置状态被压入栈。启用影子栈时，影子栈特定的状态被压入影子栈。目前这只是旧的 SSP（影子栈指针），以一种设置了63 位的特殊格式压入。在 sigreturn 时，这个旧的 SSP 令牌由内核验证并恢复。内核还会将普通的 restorer 地址压入影子栈，以帮助用户空间避免在经过 restorer sigreturn 路径上发生影子栈违规```

    |1...old SSP| - sigframe 令牌格式表示的指向旧信号ssp 的指                    （第 63 位置 1    |        ...| - 未来可能会添加其他状

```
影子栈进程不支持 32 ABI 信号。Linux 通过32 位地址空间之外分配影子栈，在影子栈启用时阻32 位执行。当执行进入 32 位模式（无论是通过远调用还是返回到用户空间）时，硬件会产生一#GP，它将被作为段错误（segfault）传递给进程。转换到用户空间时，寄存器状态就像是被返回的用户空间 ip 引发了段错误一样
### Fork


影子栈的 vma 设置VM_SHADOW_STACK 标志；其 PTE 必须为只读且脏（dirty）。当影子PTE 不是 RO 且脏时，影子访问会触发一个缺页异常，其缺页错误码中设置了影子栈访问位
当任fork 一个子进程时，其影子栈 PTE 被复制，并且父进程和子进程的影子PTE 都清除脏位。在下一次影子栈访问时，由此产生的影子栈缺页异常通过页面复制/复用处理
当创建一pthread 子线程时，内核为新线程分配一个新的影子栈。新影子栈的创建ASLR 行为上类似于 mmap()。类似地，在线程退出时，该线程的影子栈被禁用
### Exec


exec 时，影子栈特性被内核禁用。此时，用户空间可以选择重新启用或锁定它们