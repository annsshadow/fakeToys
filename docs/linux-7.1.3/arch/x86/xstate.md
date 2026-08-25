## 在用户空间应用程序中使用 XSTATE 特

x86 架构支持通过 CPUID 枚举的浮点扩展。应用程序通过查询 CPUID 并使XGETBV 来评估内XCR0 已启用哪些特性
直到 AVX-512 PKRU 状态，如果可用，这些特性会由内核自动启用。像 AMX
TILE_DATA（XSTATE 组件 18）这样的特性同样由 XCR0 启用，但相关指令的首使用会被内核捕获，因为默认情况下并不会自动分配所需的大XSTATE 缓冲区
### 引入动态特性的目的


传统的用户空间库通常为备用信号栈（alternate signal stack）硬编码了静大小，常常使MINSIGSTKSZ（通常2KB）。该栈必须至少能够存放内核在跳入
信号处理函数之前建立的信号帧。该信号帧必须包含由 CPU 定义XSAVE 缓冲区
然而，这意味着信号栈的大小是动态的而非静态的，因为不同的 CPU 拥有不同
大小XSAVE 缓冲区。对现有应用程序而言，编译期确定2KB 大小对于 AMX
这类CPU 特性来说太小了。与其普遍要求更大的栈，借助动态启用机制，内核
可以强制用户空间应用程序使用尺寸恰当altstack
### 在用户空间应用程序中使用动态启用的 XSTATE 特

内核提供了一种基arch_prctl(2) 的机制，供应用程序请求使用此类特性与此类操作相关的 arch_prctl(2) 选项如下
-ARCH_GET_XCOMP_SUPP

 arch_prctl(ARCH_GET_XCOMP_SUPP, &features);

 ARCH_GET_XCOMP_SUPP 将受支持的特性存储到 uint64_t 类型的用户空间存储中 第二个参数是指向该存储的指针
-ARCH_GET_XCOMP_PERM

 arch_prctl(ARCH_GET_XCOMP_PERM, &features);

 ARCH_GET_XCOMP_PERM 将用户空间进程已获得许可的特性存储到 uint64_t 类型 用户空间存储中。第二个参数是指向该存储的指针
-ARCH_REQ_XCOMP_PERM

 arch_prctl(ARCH_REQ_XCOMP_PERM, feature_nr);

 ARCH_REQ_XCOMP_PERM 用于请求某个动态启用的特性或特性集合的许可。一 特性集合可以映射到一个设施（facility），例如 AMX，并且可能需要启用一 或多XSTATE 组件
 feature 参数是某个设施正常工作所需的最XSTATE 组件编号
请求某个特性的许可时，内核会检查其可用性。内核会确保进程各任务的 sigaltstack
足够大，以容纳由此产生的大型信号帧。无论是ARCH_REQ_XCOMP_SUPP 期间，还在后续任sigaltstack(2) 调用期间，内核都会强制这一约束。如果已安装sigaltstack 小于由此产生sigframe 大小，ARCH_REQ_XCOMP_SUPP 会返-ENOSUPP。同样，如果请求altstack 对于已许可的特性而言过小，sigaltstack(2)
会返-ENOMEM
许可一经授予即对进程有效。许可在 fork(2) 时继承，exec(3) 时清除
与动态启用特性相关的指令首次被使用时会被内核捕获。陷阱处理程序会检查该
进程是否具有使用该特性的权限。如果进程没有权限，内核会向应用程序发SIGILL。如果进程拥有权限，则处理程序会为该任务分配更大xstate 缓冲区，
以便对大型状态进行上下文切换。在分配失败的罕见情况下，内核会发SIGSEGV
##### AMX TILE_DATA 启用示例


下面是用户空间应用程序如何动态启TILE_DATA 的示例：

  1. 应用程序首先需要向内核查询 AMX
```

        #include <asm/prctl.h>
        #include <sys/syscall.h>
        #include <stdio.h>
        #include <unistd.h>

        #ifndef ARCH_GET_XCOMP_SUPP
        #define ARCH_GET_XCOMP_SUPP  0x1021
        #endif

        #ifndef ARCH_XCOMP_TILECFG
        #define ARCH_XCOMP_TILECFG   17
        #endif

        #ifndef ARCH_XCOMP_TILEDATA
        #define ARCH_XCOMP_TILEDATA  18
        #endif

        #define MASK_XCOMP_TILE      ((1 << ARCH_XCOMP_TILECFG) | \
                                      (1 << ARCH_XCOMP_TILEDATA))

        unsigned long features;
        long rc;

        ...

        rc = syscall(SYS_arch_prctl, ARCH_GET_XCOMP_SUPP, &features);

        if (!rc && (features & MASK_XCOMP_TILE) == MASK_XCOMP_TILE)
            printf("AMX is available.\n");

  2. After that, determining support for AMX, an application must
     explicitly ask permission to use it::

        #ifndef ARCH_REQ_XCOMP_PERM
        #define ARCH_REQ_XCOMP_PERM  0x1023
        #endif

        ...

        rc = syscall(SYS_arch_prctl, ARCH_REQ_XCOMP_PERM, ARCH_XCOMP_TILEDATA);

        if (!rc)
            printf("AMX is ready for use.\n");

```
Note this example does not include the sigaltstack preparation.

### 信号帧中的动态特

动态启用的特性如果在初始配置下，则在信号进入时不会被写入信号帧。这非动态特性不同，后者无论其配置如何总会被写入。信号处理程序可以检XSAVE 缓冲区的 XSTATE_BV 字段来判断某个特性是否已被写入
### 虚拟机的动态特

访客（guest）状态组件的许可需要与宿主机（host）分开管理，因为它们彼互斥。系统扩展了若干选项用于控制访客许可
-ARCH_GET_XCOMP_GUEST_PERM

 arch_prctl(ARCH_GET_XCOMP_GUEST_PERM, &features);

 ARCH_GET_XCOMP_GUEST_PERM ARCH_GET_XCOMP_PERM 的一个变体。因此它提供
 相同的语义和功能，但面向的是访客组件
-ARCH_REQ_XCOMP_GUEST_PERM

 arch_prctl(ARCH_REQ_XCOMP_GUEST_PERM, feature_nr);

 ARCH_REQ_XCOMP_GUEST_PERM ARCH_REQ_XCOMP_PERM 的一个变体。它对访客许 具有相同的语义。在提供类似功能的同时，它也带来一个约束：在创建第一 VCPU 时许可会被冻结。此后任何更改许可的尝试都将被拒绝。因此，必须在创 第一VCPU 之前请求许可
请注意，某些 VMM 可能已经建立了一组受支持的状态组件。这些选项并不假定
支持任何特定VMM