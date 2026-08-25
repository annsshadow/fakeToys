## AArch64 Linux 中的内存标记扩展（MTE

作者：Vincenzo Frascino <vincenzo.frascino@arm.com>
         Catalin Marinas <catalin.marinas@arm.com>

日期020-02-25

本文档描述了AArch64 Linux 中提供内存标记扩展（Memory Tagging Extension功能的相关内容
## 简

基于 ARMv8.5 的处理器引入了内存标记扩展（MTE）特性。MTE 构建ARMv8.0 虚拟地址标记 TBI（Top Byte Ignore，忽略最高字节）特性之上，并允许软件访物理地址空间中每16 字节粒度（granule）的一4 位分配标记（allocation tag）这样的内存范围必须以 Normal-Tagged 内存属性映射。逻辑标记（logical tag）取用于内存访问的虚拟地址的第 59-56 位。启用了 MTE CPU 会将逻辑标记与分配标进行比较，并可能在二者不匹配时（取决于系统寄存器的配置）引发异常
## 用户空间支持


当选择`CONFIG_ARM64_MTE` 且硬件支持内存标记扩展时，内核通过 `HWCAP2_MTE`
向用户空间通告该特性
### PROT_MTE


为了访问分配标记，用户进程必须使`mmap()` `mprotect()` 的一个新`prot`
标志，在一段地址范围上启用标记（Tagged）内存属性：

`PROT_MTE` - 页允许访MTE 分配标记
这些页首次映射到用户地址空间时，分配标记被设0，并在写时复制（copy-on-write时保留。`MAP_SHARED` 受支持，分配标记可以在进程之间共享
**注意**：`PROT_MTE` 仅受 `MAP_ANONYMOUS` 和基RAM 的文件映射（`tmpfs`、`memfd`支持。将其传给其他类型的映射会导致这些系统调用返`-EINVAL`
**注意**：`PROT_MTE` 标志（及相应的内存类型）不能`mprotect()` 清除
**注意**：使`MADV_DONTNEED` `MADV_FREE` `madvise()` 内存范围，在该系调用之后的任何时候都可能被清除分配标记（设为 0）
### 标记检查错误（Tag Check Faults

当某地址范围启用`PROT_MTE`，且访问时逻辑标记与分配标记不匹配时，有三种可配置行为
- **Ignore（忽略）** - 这是默认模式。CPU（和内核）忽略标记检查错误
- **Synchronous（同步）** - 内核同步地引发一`SIGSEGV`，其  `.si_code = SEGV_MTESERR` `.si_addr = <fault-address>`。内存访问不会被执行  如果 `SIGSEGV` 被出错线程忽略或阻塞，所属进程将被终止并生成 `coredump`
- **Asynchronous（异步）** - 内核在出错线程中，在一个或多个标记检查错误之后异步地
  引发一`SIGSEGV`，其`.si_code = SEGV_MTEAERR` `.si_addr = 0`（出错地址未知）
- **Asymmetric（非对称* - 读操作按同步模式处理，而写操作按异步模式处理
用户可以按线程，使用 `prctl(PR_SET_TAGGED_ADDR_CTRL, flags, 0, 0, 0)` 系统调用
选择上述模式，其`flags` `PR_MTE_TCF_MASK` 位域中包含以下任意值：

- `PR_MTE_TCF_NONE`  - **忽略**标记检查错                         （若与其他选项组合则被忽略- `PR_MTE_TCF_SYNC`  - **同步**标记检查错误模- `PR_MTE_TCF_ASYNC` - **异步**标记检查错误模
如果未指定任何模式，标记检查错误将被忽略。如果只指定了单一模式，程序将在该模式运行。如果指定了多个模式，则按下文“每 CPU 偏好的标记检查模式”一节所述选择模式
当前的标记检查错误配置可以使`prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0)` 系统
调用读取。如果请求了多个模式，则全部都会被报告
标记检查也可以通过设置 `PSTATE.TCO` 位（使用 `MSR TCO, #1`）对某个用户线程禁用
**注意**：信号处理程序始终以 `PSTATE.TCO = 0` 被调用，与被中断的上下文无关`PSTATE.TCO` 会在 `sigreturn()` 时恢复
**注意**：用户应用程序没有可用的**匹配全部（match-all*逻辑标记
**注意**：内核对用户地址空间（例`read()` 系统调用）的访问，在用户线程的标记检模式`PR_MTE_TCF_NONE` `PR_MTE_TCF_ASYNC` 时不被检查。如果标记检查模式为
`PR_MTE_TCF_SYNC`，内核会尽最大努力检查其对用户地址的访问，但无法始终保证。不用户配置如何，内核对用户地址的访问始终以有效`PSTATE.TCO` 0 执行
### ``IRG``、``ADDG`` ``SUBG`` 指令中排除标

体系结构允许通过 `GCR_EL1.Exclude` 寄存器位域排除某些被随机生成的标记。默认情况下Linux 排除0 以外的所有标记。用户线程可以使``prctl(PR_SET_TAGGED_ADDR_CTRL,
flags, 0, 0, 0)`` 系统调用在随机生成的集合里启用特定标记，其中 `flags`` `PR_MTE_TAG_MASK` 位域中包含标记位图
**注意**：硬件使用的是排除掩码，`prctl()` 接口提供的是包含掩码。包含掩码为 `0`
（排除掩`0xffff`）会导致 CPU 始终生成标记 `0`
### CPU 偏好的标记检查模

在某CPU 上，MTE 在更严格标记检查模式下的性能与较宽松标记检查模式下的性能相近当请求了较宽松的检查模式时，在这些 CPU 上启用更严格的检查是值得的，以便在不带来
性能下降的前提下获得更严格检查的错误检测优势。为支持这种场景，特权用户可以将更严的标记检查模式配置为CPU 偏好的标记检查模式
每个 CPU 偏好的标记检查模式由 `/sys/devices/system/cpu/cpu<N>/mte_tcf_preferred`
控制，特权用户可以向其写入`async`、`sync` `asymm`。每CPU 默认的偏好模式为
`async`銆。
为了允许程序可能CPU 偏好的标记检查模式下运行，用户程序可以在 ``prctl(PR_SET_TAGGED_ADDR_CTRL,
flags, 0, 0, 0)`` 系统调用`flags` 参数中设置多个标记检查错误模式位。如果同时请求了
同步和异步模式，那么内核也可能选择非对称模式。如CPU 偏好的标记检查模式处于任所提供的标记检查模式集合中，则选择该模式。否则，内核将从任务的模式集中按下述偏好
顺序选择一种模式：

 1. 异步（Asynchronous 2. 非对称（Asymmetric 3. 同步（Synchronous
注意，用户空间无法在请求多种模式的同时禁用非对称模式
### 初始进程状

`execve()` 时，新进程具有以下配置：

- `PR_TAGGED_ADDR_ENABLE` 设为 0（禁用）
- 未选择任何标记检查模式（标记检查错误被忽略- `PR_MTE_TAG_MASK` 设为 0（所有标记都被排除）
- `PSTATE.TCO` 设为 0
- 初始内存映射均未设置 `PROT_MTE`

`fork()` 时，新进程继承父进程的配置和内存映射属性，但使`MADV_WIPEONFORK` `madvise()` 范围除外——这些范围的数据和标记会被清除（设为 0）
### ``ptrace()`` 接口


`PTRACE_PEEKMTETAGS` `PTRACE_POKEMTETAGS` 允许追踪者（tracer）从被追踪者（tracee的地址空间读取标记或向其设置标记。`ptrace()` 系统调用``ptrace(request, pid, addr,
data)`` 形式调用，其中：

- `request` - `PTRACE_PEEKMTETAGS` `PTRACE_POKEMTETAGS` 之一- `pid` - 被追踪者的 PID- `addr` - 被追踪者地址空间中的地址- `data` - 指向一`struct iovec` 的指针，其中 `iov_base` 指向追踪者地址空间  长度`iov_len` 的缓冲区
追踪者的 `iov_base` 缓冲区中的标记表示为每字节一4 位标记，对应于被追踪者地址空间
中的一16 字节 MTE 标记粒度
**注意**：如`addr` 未对齐到 16 字节粒度，内核将使用相应的对齐地址
`ptrace()` 返回值：

- 0 - 标记已被复制，追踪者的 `iov_len` 被更新为传输的标记数量。如果被追踪者或追踪  的地址空间中的请求地址范围无法访问或不具有有效标记，该值可能小于请求的 `iov_len`- `-EPERM` - 无法追踪指定的进程- `-EIO` - 无法访问被追踪者的地址范围（例如无效地址），未复制任何标记。`iov_len`
  未更新- `-EFAULT` - 访问追踪者内存（`struct iovec` `iov_base` 缓冲区）时出错，未复  任何标记。`iov_len` 未更新- `-EOPNOTSUPP` - 被追踪者的地址没有有效标记（从未以 `PROT_MTE` 标志映射）。`iov_len`
  未更新
**注意**：上述请求没有瞬时错误，因此用户程序在系统调用返回非零值时不应对其重试
`PTRACE_GETREGSET` `PTRACE_SETREGSET`，配``addr ==
`NT_ARM_TAGGED_ADDR_CTRL`，允`ptrace()` 按照
Documentation/arch/arm64/tagged-address-abi.rst 及上文的 `prctl()` 选项所述，访问
进程的标记地址 ABI 控制MTE 配置。相应的 `regset` 1 8 字节的元（`sizeof(long))`）
### Core dump 支持


`PROT_MTE` 映射的用户内存的分配标记，会作为额外`PT_AARCH64_MEMTAG_MTE` 转储core 文件中。此类段的程序头定义如下
:`p_type`: `PT_AARCH64_MEMTAG_MTE`
:`p_flags`: 0
:`p_offset`: 段在文件中的偏移:`p_vaddr`: 段的虚拟地址，与相应`PT_LOAD` 段相:`p_paddr`: 0
:`p_filesz`: 段在文件中的大小，计算为 `p_mem_sz / 32`
  （两4 位标记覆32 字节内存:`p_memsz`: 段在内存中的大小，与相应`PT_LOAD` 段相:`p_align`: 0

标记以两4 位标记存于一个字节的方式，存放在 core 文件`p_offset` 处。标记粒度为
16 字节，一4K 页在 core 文件中需128 字节
## 正确用法示例


**MTE 示例代码**


    /*
     - 需-march=armv8.5-a+memtag 编译
     */
    #include <errno.h>
    #include <stdint.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <unistd.h>
    #include <sys/auxv.h>
    #include <sys/mman.h>
    #include <sys/prctl.h>

    /*
     - From arch/arm64/include/uapi/asm/hwcap.h
     */
    #define HWCAP2_MTE              (1 << 18)

    /*
     - From arch/arm64/include/uapi/asm/mman.h
     */
    #define PROT_MTE                 0x20

    /*
     - From include/uapi/linux/prctl.h
     */
    #define PR_SET_TAGGED_ADDR_CTRL 55
    #define PR_GET_TAGGED_ADDR_CTRL 56
    # define PR_TAGGED_ADDR_ENABLE  (1UL << 0)
    # define PR_MTE_TCF_SHIFT       1
    # define PR_MTE_TCF_NONE        (0UL << PR_MTE_TCF_SHIFT)
    # define PR_MTE_TCF_SYNC        (1UL << PR_MTE_TCF_SHIFT)
    # define PR_MTE_TCF_ASYNC       (2UL << PR_MTE_TCF_SHIFT)
    # define PR_MTE_TCF_MASK        (3UL << PR_MTE_TCF_SHIFT)
    # define PR_MTE_TAG_SHIFT       3
    # define PR_MTE_TAG_MASK        (0xffffUL << PR_MTE_TAG_SHIFT)

    /*
     - 向给定指针插入一个随机的逻辑标记     */
    #define insert_random_tag(ptr) ({                       \
            uint64_t __val;                                 \
            asm("irg %0, %1" : "=r" (__val) : "r" (ptr));   \
            __val;                                          \
    })

    /*
     - 在目标地址上设置分配标记     */
    #define set_tag(tagged_addr) do {                                      \
            asm volatile("stg %0, [%0]" : : "r" (tagged_addr) : "memory"); \
    } while (0)

    int main()
    {
            unsigned char *a;
            unsigned long page_sz = sysconf(_SC_PAGESIZE);
            unsigned long hwcap2 = getauxval(AT_HWCAP2);

            /** 检查是否存MTE **/
            if (!(hwcap2 & HWCAP2_MTE))
                    return EXIT_FAILURE;

            /*
             - 启用标记地址 ABI、同步或异步（基于每 CPU 偏好）的 MTE
             - 标记检查错误，并允许随机生成集合中0 外的所             - 标记             */
            if (prctl(PR_SET_TAGGED_ADDR_CTRL,
                      PR_TAGGED_ADDR_ENABLE | PR_MTE_TCF_SYNC | PR_MTE_TCF_ASYNC |
                      (0xfffe << PR_MTE_TAG_SHIFT),
                      0, 0, 0)) {
                    perror("prctl() failed");
                    return EXIT_FAILURE;
            }

            a = mmap(0, page_sz, PROT_READ | PROT_WRITE,
                     MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
            if (a == MAP_FAILED) {
                    perror("mmap() failed");
                    return EXIT_FAILURE;
            }

            /*
             - 在上面的匿名 mmap 上启MTE。该标志也可以直接传             - mmap() 从而跳过这一步             */
            if (mprotect(a, page_sz, PROT_READ | PROT_WRITE | PROT_MTE)) {
                    perror("mprotect() failed");
                    return EXIT_FAILURE;
            }

            /** 以默认标(0) 访问 **/
            a[^0^] = 1;
            a[^1^] = 2;

            printf("a[^0^] = %hhu a[^1^] = %hhu\n", a[^0^], a[^1^]);

            /** 设置逻辑与分配标**/
            a = (unsigned char *)insert_random_tag(a);
            set_tag(a);

            printf("%p\n", a);

            /** 以非零标记访**/
            a[^0^] = 3;
            printf("a[^0^] = %hhu a[^1^] = %hhu\n", a[^0^], a[^1^]);

            /*
             - 如果 MTE 被正确启用，下一条指令将产生一             - 异常             */
            printf("Expecting SIGSEGV...\n");
            a[^16^] = 0xdd;

            /** PR_MTE_TCF_SYNC 模式下不应打印这**/
            printf("...haven't got one\n");

            return EXIT_FAILURE;
    }
