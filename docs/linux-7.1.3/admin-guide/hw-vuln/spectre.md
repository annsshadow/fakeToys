
## Spectre 侧信道


Spectre 是一类侧信道攻击，它利用现代 CPU 上的分支预测和推测执行（speculative
execution）来读取内存，可能绕过访问控制。推测执行侧信道攻击不修改内存，而是
试图推断内存中的特权数据。

本文档涵盖 Spectre 变体 1 和 Spectre 变体 2。

### 受影响的处理器


推测执行侧信道方法影响了广泛范围的现代高性能处理器，因为大多数现代高速
处理器都使用分支预测和推测执行。

以下 CPU 易受攻击：

    - Intel Core、Atom、Pentium 和 Xeon 处理器

    - AMD Phenom、EPYC 和 Zen 处理器

    - IBM POWER 和 zSeries 处理器

    - 高端 ARM 处理器

    - Apple CPU

    - 高端 MIPS CPU

    - 可能还有大多数其他高性能 CPU。详情请联系您的 CPU 供应商。

某个处理器是否受影响，可以从 sysfs 中的 Spectre 漏洞文件中读出。请参阅
spectre_sys_info。

### 相关的 CVE


以下 CVE 条目描述了 Spectre 变体：

   =============   =======================  ==========================
   CVE-2017-5753   Bounds check bypass      Spectre variant 1
   CVE-2017-5715   Branch target injection  Spectre variant 2
   CVE-2019-1125   Spectre v1 swapgs        Spectre variant 1 (swapgs)
   =============   =======================  ==========================

### 问题


CPU 使用推测操作来提升性能。这可能会在处理器的缓存、缓冲区和分支预测器中
留下内存访问或计算的痕迹。恶意软件可能影响推测执行路径，然后利用推测执行
在 CPU 缓存和缓冲区中留下的副作用，来推断在推测执行期间被触及的特权数据。

Spectre 变体 1 攻击利用条件分支的推测执行，而 Spectre 变体 2 攻击使用间接
分支的推测执行来泄露特权内存。参见 [^1^] <spec_ref1> [^5^] <spec_ref5> [^6^] <spec_ref6>
[^7^] <spec_ref7> [^10^] <spec_ref10> [^11^] <spec_ref11>。

### Spectre 变体 1（边界检查绕过，Bounds Check Bypass）


边界检查绕过攻击 [^2^] <spec_ref2> 利用推测执行绕过用于内存访问边界检查
（例如，检查数组的索引是否导致内存在有效范围内的访问）的条件分支指令。这会
导致对无效内存（带有越界索引）的内存访问，这些访问在验证检查解决之前被
推测地执行。这样的推测内存访问可能留下副作用，从而创建将信息泄露给攻击者的
侧信道。

Spectre 变体 1 攻击有一些扩展，用于通过网络读取数据，参见 [^12^] <spec_ref12>。
然而此类攻击困难、带宽低、脆弱，被视为低风险。

请注意，尽管名为“Bounds Check Bypass（边界检查绕过）”，Spectre 变体 1 并非
仅关乎用户控制的数组边界检查。它可以影响任何条件检查。内核入口代码的
中断、异常和 NMI 处理程序都有条件的 swapgs 检查。在内核代码可以在推测执行中
以一个用户 GS 运行的情况下，那些检查在 Spectre v1 的语境下可能成问题。

### Spectre 变体 2（分支目标注入，Branch Target Injection）


分支目标注入攻击利用间接分支的推测执行 [^3^] <spec_ref3>。处理器内部用于
猜测间接分支目标的间接分支预测器可能受到攻击者影响，导致 gadget 代码被推测
执行，从而暴露受害者触及的敏感数据。推测执行期间在 CPU 缓存中留下的副作用
可以被测量以推断数据值。


在 Spectre 变体 2 攻击中，攻击者可以通过毒化（poisoning）用于预测间接分支
地址的 CPU 的分支目标缓冲区（BTB），来将受害者的推测间接分支导向 gadget 代码。
这种毒化可以通过间接分支进入已有代码来完成，间接分支的地址偏移由攻击者控制。
由于在受影响硬件上的分支预测不能完全消歧义分支地址、而是使用偏移进行预测，
这可能导致特权代码的间接分支跳转到具有相同偏移的 gadget 代码。

最有用的 gadget 接受一个攻击者控制的输入参数（例如一个寄存器值），以便受控
地读取内存。没有输入参数的 gadget 也有可能，但攻击者对其能读取什么内存几乎
无法控制，从而降低了攻击泄露有用数据的风险。

变体 2 的另一种攻击途径是攻击者毒化返回栈缓冲区（RSB）[^13^] <spec_ref13>，
以导致推测的子例程返回指令执行跳转到 gadget。攻击者不平衡的子例程调用指令
可能“毒化”返回栈缓冲区中的条目，这些条目随后被受害者的子例程返回指令消费。
这种攻击可以通过在上下文切换或虚拟机（VM）退出时刷新返回栈缓冲区来缓解。

在具有同步多线程（SMT）的系统上，攻击可能来自兄弟线程（sibling thread），
因为一级缓存和分支目标缓冲区（BTB）可能在 CPU 核内的硬件线程之间共享。运行
在兄弟线程上的恶意程序可能影响其对等方的 BTB，将其间接分支推测导向 gadget
代码，并测量留在一级缓存中的推测执行副作用，以推断受害者的数据。

变体 2 的又一种攻击途径是攻击者毒化分支历史缓冲区（BHB），以推测地将一个
间接分支导向一个特定的分支目标缓冲区（BTB）条目，即使该条目与该间接分支的
源地址并不关联。具体来说，即使在 Enhanced IBRS 存在的情况下，BHB 也可能跨
特权级别共享。

此前唯一已知的真实世界 BHB 攻击途径是通过非特权 eBPF。进一步的研究发现了
不需要非特权 eBPF 的攻击。为了对 BHB 攻击进行完整缓解，建议使用 BHI_DIS_S
或采用 BHB 清除序列。

### 攻击场景


以下是已被预见的攻击场景列表，但可能未涵盖所有可能的攻击途径。

##### 1. 用户进程攻击内核


#### Spectre 变体 1


   攻击者通过寄存器或在系统调用（syscall）期间经由内存中已知地址，向内核
   传递一个参数。这样的参数之后可能被内核用作数组的索引，或用于推导一个
   指向 Spectre 变体 1 攻击的指针。该索引或指针是无效的，但在被采取用于
   推测执行的代码分支中，边界检查被绕过。这可能导致特权内存被访问并泄露。

   对于已识别出数据指针可能受 Spectre 攻击影响的内核代码，使用新的“nospec”
   访问器宏来防止数据的推测加载。

#### Spectre 变体 1（swapgs）


   攻击者可以训练分支预测器，以推测地跳过中断或异常对应的 swapgs 路径。
   如果他们把 GS 寄存器初始化为一个用户空间值，而 swapgs 被推测跳过，那么在
   推测窗口中后续的 GS 相关 percpu 访问将使用攻击者控制的 GS 值进行。这可能
   导致特权内存被访问并泄露。

   例如：

```

     if (coming from user space)
         swapgs
     mov %gs:<percpu_offset>, %reg
     mov (%reg), %reg1

   当来自用户空间时，CPU 可以推测地跳过 swapgs，然后利用用户 GS 值进行推测
   的 percpu 加载。因此用户可以推测地强制读取任何内核值。如果存在一个 gadget，
   使用 percpu 值作为另一次加载/存储中的地址，那么内核值的内容可能通过 L1
   侧信道攻击变得可见。

   当来自内核空间时存在类似的攻击。CPU 可以推测地执行 swapgs，导致用户 GS
   被用于推测窗口的其余部分。

```

#### Spectre 变体 2


   一个 Spectre 变体 2 攻击者可以在发起攻击之前毒化 <poison_btb> 分支目标
   缓冲区（BTB）。进入内核后，内核可能在间接跳转上使用被毒化的分支目标缓冲
   区，并跳转到推测执行中的 gadget 代码。

   如果攻击者试图控制推测执行期间泄露的内存地址，他还需要通过寄存器或内存中
   已知地址向 gadget 传递一个参数。在 gadget 执行之后，他可以测量副作用。

   内核可以通过对所有间接分支使用返回蹦床（return trampolines，也称为
   “retpoline”）[^3^] <spec_ref3> [^9^] <spec_ref9> 来防止消费被毒化的分支
   目标缓冲区条目。返回蹦床捕获推测执行路径，以防止在推测执行期间跳转到 gadget
   代码。具有硬件可用的 Enhanced Indirect Branch Restricted Speculation
   （Enhanced IBRS，增强型间接分支限制推测）的 x86 CPU 应使用该特性来缓解
   Spectre 变体 2，而不是 retpoline。Enhanced IBRS 比 retpoline 更高效。

   固件中可能含有 gadget 代码，可能被恶意用户进程利用 Spectre 变体 2 攻击
   加以利用。为了在 x86 上缓解此类攻击，在调用任何固件代码之前开启 Indirect
   Branch Restricted Speculation（IBRS，间接分支限制推测）特性。

##### 2. 用户进程攻击另一个用户进程


   恶意用户进程可以尝试攻击另一个用户进程，要么经由同一硬件线程上的上下文
   切换，要么来自在同步多线程（SMT）系统上共享一个物理处理器核的兄弟超线程
   （sibling hyperthread）。

   Spectre 变体 1 攻击通常需要在进程之间传递参数，这需要一种数据传递关系，
   例如远程过程调用（RPC）。这些参数在 gadget 代码中被用来推导访问被攻击
   进程中特权内存的无效数据指针。

   Spectre 变体 2 攻击可以由一个恶意进程通过毒化 <poison_btb> 分支目标缓冲
   区发起。这可以影响受害者进程的间接分支目标，该受害者进程要么稍后在同一
   硬件线程上运行，要么在共享同一物理核的兄弟硬件线程上并发运行。

   用户进程可以通过使用 prctl() 系统调用来为自己禁用间接分支推测，从而保护
   自身免受 Spectre 变体 2 攻击。管理员也可以通过禁用该进程的间接分支推测，
   将其隔离（cordon off），防止其污染分支目标缓冲区。这会带来性能代价，因为
   不再使用间接分支推测并需要清除分支目标缓冲区。在 x86 上启用 SMT 时，对于
   间接分支推测被禁用的进程，会开启单线程间接分支预测器（STIBP）[^4^] <spec_ref4>
   以防止兄弟线程控制分支目标缓冲区。此外，在切换到此类进程以及从此类进程
   切换出去时，会发出间接分支预测屏障（IBPB）以清除分支目标缓冲区。

   在 x86 上，返回栈缓冲区在上下文切换时被填充（stuffed）。这防止了返回栈
   缓冲区在切换到更深的调用栈时发生下溢时被用于分支预测。前一个进程留在返回
   栈缓冲区中的任何被毒化条目也将被清除。

   用户程序应使用地址空间随机化来使攻击更困难（设置
   /proc/sys/kernel/randomize_va_space = 1 或 2）。

##### 3. 虚拟化客户机攻击宿主机


   攻击机制类似于用户进程攻击内核的方式。内核经由超级调用（hyper-call）或
   其他虚拟化退出路径进入。

   对于 Spectre 变体 1 攻击，恶意客户机可以经由超级调用传递参数（例如在
   寄存器中），以在进入内核后推导指向特权内存的无效指针进行推测。对于已识别
   出此类内核代码的地方，使用 nospec 访问器宏来阻止推测内存访问。

   对于 Spectre 变体 2 攻击，恶意客户机可以 :ref:`poison <poison_btb>` 分支
   目标缓冲区或返回栈缓冲区，导致内核跳转到推测执行路径中的 gadget 代码。

   为了缓解变体 2，宿主机内核可以对间接分支使用返回蹦床，以绕过被毒化的分支
   目标缓冲区，并在 VM 退出时刷新返回栈缓冲区。这可以防止恶意客户机影响宿主机
   内核中的间接分支。

   为了保护宿主机进程免受恶意客户机影响，宿主机进程可以通过 prctl() 禁用其
   间接分支推测。在切换到此类进程之前，会清除分支目标缓冲区。

##### 4. 虚拟化客户机攻击其他客户机


   恶意客户机可以攻击另一个客户机，以获取该客户机可访问的数据。

   如果参数可以在客户机之间传递，Spectre 变体 1 攻击是可能的。这可以通过
   共享内存或消息传递等机制完成。此类参数可用于推导指向客户机中特权数据的
   数据指针。该特权数据可能被受害者推测路径中的 gadget 代码访问。

   Spectre 变体 2 攻击可以由恶意客户机通过毒化 <poison_btb> 分支目标缓冲区
   或返回栈缓冲区发起。这些被毒化的条目可用于影响受害者客户机中的推测执行路径。

   Linux 内核通过在 VM 退出时刷新返回栈缓冲区，以及在切换到新客户机之前清除
   分支目标缓冲区，来缓解对同一 CPU 硬件线程上运行的其他客户机的攻击。

   如果使用 SMT，来自兄弟超线程中不受信任客户机的 Spectre 变体 2 攻击可以由
   管理员通过将不安全客户机的间接分支推测经由 prctl() 关闭来缓解。客户机也
   可以通过在自身内部开启基于微码的缓解（例如 x86 上的 IBPB 或 STIBP）来保护
   自己。


### Spectre 系统信息


Linux 内核提供一个 sysfs 接口，用于枚举系统针对 Spectre 的当前缓解状态：
系统是否易受攻击，以及哪些缓解措施处于活动状态。

显示 Spectre 变体 1 缓解状态的 sysfs 文件是：

   /sys/devices/system/cpu/vulnerabilities/spectre_v1

该文件中可能的值为：

```

     * - 'Not affected'
       - 处理器不易受攻击。
     * - 'Vulnerable: __user pointer sanitization and usercopy barriers only; no swapgs barriers'
       - swapgs 保护被禁用；否则它在内核中基于具体情况，通过显式的指针净化和
         usercopy LFENCE 屏障提供保护。
     * - 'Mitigation: usercopy/swapgs barriers and __user pointer sanitization'
       - 内核中基于具体情况，通过显式指针净化、usercopy LFENCE 屏障和 swapgs
         LFENCE 屏障提供保护。

```
然而，这些保护是按具体情况实施的，并不能保证覆盖 Spectre 变体 1 的所有可能
攻击途径。

spectre_v2 内核文件报告内核是否使用 retpoline 缓解编译，或者 CPU 是否具有
硬件缓解，以及 CPU 是否支持额外的、进程特定的缓解。

该文件还报告由微码启用的、用于缓解用户进程之间攻击的 CPU 特性：

1. Indirect Branch Prediction Barrier（IBPB，间接分支预测屏障）以增加不同
   用户进程之间的隔离。
2. Single Thread Indirect Branch Predictors（STIBP，单线程间接分支预测器）以
   增加运行在同一核上的 CPU 线程之间的隔离。

这些 CPU 特性在使用时可能影响性能，可以按进程基于具体情况启用。

显示 Spectre 变体 2 缓解状态的 sysfs 文件是：

   /sys/devices/system/cpu/vulnerabilities/spectre_v2

该文件中可能的值为：

  - 内核状态：

  ========================================  =================================
  'Not affected'                            The processor is not vulnerable
  'Mitigation: None'                        Vulnerable, no mitigation
  'Mitigation: Retpolines'                  Use Retpoline thunks
  'Mitigation: LFENCE'                      Use LFENCE instructions
  'Mitigation: Enhanced IBRS'               Hardware-focused mitigation
  'Mitigation: Enhanced IBRS + Retpolines'  Hardware-focused + Retpolines
  'Mitigation: Enhanced IBRS + LFENCE'      Hardware-focused + LFENCE
  ========================================  =================================

  - 固件状态：显示调用固件时（仅 x86），是否使用 Indirect Branch Restricted
    Speculation（IBRS）来防范 Spectre 变体 2 攻击。

  ========== =============================================================
  'IBRS_FW'  Protection against user program attacks when calling firmware
  ========== =============================================================

  - 间接分支预测屏障（IBPB）状态，用于不同用户进程之间的保护。该特性可以按
    进程通过 prctl() 控制，或通过内核命令行选项控制。这是一个仅 x86 的特性。
    更多细节见下文。

  ===================   ========================================================
  'IBPB: disabled'      IBPB unused
  'IBPB: always-on'     Use IBPB on all tasks
  'IBPB: conditional'   Use IBPB on SECCOMP or indirect branch restricted tasks
  ===================   ========================================================

  - 单线程间接分支预测（STIBP）状态，用于不同超线程之间的保护。该特性可以按
    进程通过 prctl 控制，或通过内核命令行选项控制。这是一个仅 x86 的特性。
    更多细节见下文。

  ====================  ========================================================
  'STIBP: disabled'     STIBP unused
  'STIBP: forced'       Use STIBP on all tasks
  'STIBP: conditional'  Use STIBP on SECCOMP or indirect branch restricted tasks
  ====================  ========================================================

  - 返回栈缓冲区（RSB）保护状态：

  =============   ===========================================
  'RSB filling'   Protection of RSB on context switch enabled
  =============   ===========================================

  - EIBRS 屏障后返回栈缓冲区（PBRSB）保护状态：

  ===========================  =======================================================
  'PBRSB-eIBRS: SW sequence'   CPU is affected and protection of RSB on VMEXIT enabled
  'PBRSB-eIBRS: Vulnerable'    CPU is vulnerable
  'PBRSB-eIBRS: Not affected'  CPU is not affected by PBRSB
  ===========================  =======================================================

  - 分支历史注入（BHI）保护状态：


 - - BHI: Not affected
   - System is not affected
 - - BHI: Retpoline
   - System is protected by retpoline
 - - BHI: BHI_DIS_S
   - System is protected by BHI_DIS_S
 - - BHI: SW loop, KVM SW loop
   - System is protected by software clearing sequence
 - - BHI: Vulnerable
   - System is vulnerable to BHI
 - - BHI: Vulnerable, KVM: SW loop
   - System is vulnerable; KVM is protected by software clearing sequence

完整的缓解可能需要来自 CPU 供应商的微码更新。当必要的微码不可用时，内核将
报告漏洞。

### 开启针对 Spectre 变体 1 和 Spectre 变体 2 的缓解


##### 1. 内核缓解


#### Spectre 变体 1


   对于 Spectre 变体 1，易受攻击的内核代码（由代码审查或扫描工具确定）基于
   具体情况进行标注，以使用 nospec 访问器宏进行边界裁剪 :ref:`[^2^]
   <spec_ref2>`，以避免任何可用的泄露 gadget。然而，它可能无法覆盖 Spectre
   变体 1 的所有攻击途径。

   从用户复制（copy-from-user）代码有一个 LFENCE 屏障，以防止 access_ok()
   检查被错误推测。该屏障由 barrier_nospec() 宏完成。

   对于 Spectre 变体 1 的 swapgs 变体，在需要时，会在中断、异常和 NMI 入口
   添加 LFENCE 屏障。这些屏障由 FENCE_SWAPGS_KERNEL_ENTRY 和
   FENCE_SWAPGS_USER_ENTRY 宏完成。

#### Spectre 变体 2


   对于 Spectre 变体 2 缓解，编译器将内核中的间接调用或跳转转换为等价的
   返回蹦床（retpolines）[^3^] <spec_ref3> [^9^] <spec_ref9> 以跳转到目标
   地址。返回蹦床下的推测执行路径被捕获在一个无限循环中，以防止任何推测执行
   跳转到 gadget。

   要在易受攻击的 CPU 上开启 retpoline 缓解，内核需要使用支持
   -mindirect-branch=thunk-extern -mindirect-branch-register 选项的 gcc 编译器
   编译。如果内核使用 Clang 编译器编译，编译器需要支持 -mretpoline-external-thunk
   选项。需要开启内核配置 CONFIG_MITIGATION_RETPOLINE，并且 CPU 需要运行最新的
   更新微码。

   在 Intel Skylake 时代的系统上，缓解覆盖了大多数但并非全部情况。更多细节
   参见 [^3^] <spec_ref3>。

   在具有针对 Spectre 变体 2 的硬件缓解（例如 x86 上的 IBRS 或 enhanced IBRS）
   的 CPU 上，retpoline 在运行时被自动禁用。

   支持 enhanced IBRS（eIBRS）的系统在启动时通过置位 IBRS 位一次性开启 IBRS
   保护，它们自动受到针对某些 Spectre v2 变体攻击的保护。BHB 仍然可以影响间接
   分支预测器条目的选择，并且尽管在启用 eIBRS 时分支预测器条目在模式之间被
   隔离，BHB 本身在模式之间并未被隔离。支持 BHI_DIS_S 的系统会设置它以防备
   BHI 攻击。

   在 Intel 的 enhanced IBRS 系统上，这包括 SMT 系统（STIBP）上的跨线程分支
   目标注入。换句话说，Intel eIBRS 也启用了 STIBP。

   AMD Automatic IBRS 不保护用户空间，而 Legacy IBRS 系统在返回用户空间时清除
   IBRS 位，因此两者都显式地启用 STIBP。

   retpoline 缓解在易受攻击的 CPU 上默认开启。管理员可以通过内核命令行和
   sysfs 控制文件强制开启或关闭它。请参阅 spectre_mitigation_control_command_line。

   在 x86 上，在调用任何固件代码之前，默认开启间接分支限制推测，以防止利用
   固件的 Spectre 变体 2 攻击。

   使用内核地址空间随机化（内核配置中的 CONFIG_RANDOMIZE_BASE=y 和
   CONFIG_SLAB_FREELIST_RANDOM=y）使针对内核的攻击通常更困难。

##### 2. 用户程序缓解


   用户程序可以使用 LFENCE 或“边界裁剪（bounds clipping）”来缓解 Spectre
   变体 1。更多细节参见 [^2^] <spec_ref2>。

   对于 Spectre 变体 2 缓解，单个用户程序可以用针对间接分支的返回蹦床编译。
   这保护它们免受恶意软件留在分支目标缓冲区中的被毒化条目的消费。

   在 legacy IBRS 系统上，在返回用户空间时，隐式 STIBP 被禁用，因为内核清除了
   IBRS 位。在这种情况下，用户空间程序可以通过 prctl() 禁用其间接分支推测
   （参见 Documentation/userspace-api/spec_ctrl.rst <set_spec_ctrl>）。在 x86
   上，这将在用户程序运行时开启 STIBP 以防备来自兄弟线程的攻击，并在切换到/
   从该程序的切换中使用 IBPB 刷新分支目标缓冲区。

   限制用户程序的间接分支推测也将防止该程序在 x86 上发起变体 2 攻击。管理员
   可以通过内核命令行和 sysfs 控制文件改变这种行为。请参阅
   spectre_mitigation_control_command_line。

   禁用其间接分支推测的程序会有更多开销并运行得更慢。

   用户程序应使用地址空间随机化（/proc/sys/kernel/randomize_va_space = 1 或
   2）来使攻击更困难。

##### 3. 虚拟机缓解


   在内核内部，来自恶意客户机的 Spectre 变体 1 攻击在 VM 退出路径上基于具体
   情况被缓解。易受攻击的代码使用 nospec 访问器宏进行“边界裁剪”，以避免任何
   可用的泄露 gadget。然而，这可能无法覆盖所有变体 1 攻击途径。

   针对来自恶意客户机到内核的 Spectre 变体 2 攻击，Linux 内核使用 retpoline
   或 Enhanced IBRS 来防止消费恶意客户机留在分支目标缓冲区中的被毒化条目。它
   还会在每次 VM 退出时刷新返回栈缓冲区，以防止返回栈缓冲区的下溢，从而使被
   毒化的分支目标缓冲区可被使用，或防止攻击客户机在返回栈缓冲区中留下被毒化
   的条目。

   为了缓解同一 CPU 硬件线程上的客户机到客户机攻击，在切换到 CPU 上的新客户机
   之前，通过刷新来净化分支目标缓冲区。

   上述缓解在易受攻击的 CPU 上默认开启。

   为了缓解 SMT 使用时来自兄弟线程的客户机到客户机攻击，在兄弟线程中运行的
   不受信任客户机可以由管理员经由 prctl() 禁用其间接分支推测。

   内核还允许客户机使用它们选择的任何基于微码的缓解（例如 x86 上的 IBPB 或
   STIBP）来保护自己。


### 内核命令行上的缓解控制


一般而言，内核会为当前 CPU 选择合理的默认缓解措施。

Spectre 默认缓解可以通过以下选项在内核命令行上禁用或更改：

 - nospectre_v1
 - nospectre_v2
 - spectre_v2={option}
 - spectre_v2_user={option}
 - spectre_bhi={option}

有关可用选项的更多细节，请参阅 Documentation/admin-guide/kernel-parameters.txt

### 缓解选择指南


##### 1. 可信用户空间


   如果所有用户空间应用都来自可信来源，并且不执行外部提供的不受信任代码，
   那么可以禁用缓解措施。

##### 2. 保护敏感程序


   对于带有秘密（例如加密密钥）的安全敏感程序，可以在程序运行时通过禁用其
   间接分支推测来实施针对 Spectre 变体 2 的保护（参见
   Documentation/userspace-api/spec_ctrl.rst <set_spec_ctrl>）。

##### 3. 沙箱化不受信任程序


   可能成为攻击来源的不受信任程序可以通过在运行时禁用其间接分支推测来隔离
   （参见 Documentation/userspace-api/spec_ctrl.rst <set_spec_ctrl>）。这防止
   不受信任程序污染分支目标缓冲区。这种行为可以通过内核命令行和 sysfs 控制
   文件改变。请参阅 spectre_mitigation_control_command_line。

##### 3. 高安全模式


   所有 Spectre 变体 2 缓解都可以在启动时对所有程序强制开启（参见
   spectre_mitigation_control_command_line 中的“on”选项）。这将增加开销，因为
   所有程序的间接分支推测都将受到限制。

   在 x86 上，在切换到新程序时，分支目标缓冲区将通过 IBPB 刷新。STIBP 始终
   保持开启，以保护程序免受来自兄弟线程上运行的程序的变体 2 攻击。

   或者，STIBP 可以仅用于运行那些间接分支推测被显式禁用的程序，而 IBPB 仍
   在切换到新程序时始终使用，以清除分支目标缓冲区（参见
   spectre_mitigation_control_command_line 中的“ibpb”选项）。这个“ibpb”选项
   比“on”选项性能代价更小，后者会让 STIBP 始终开启。

### Spectre 相关参考资料


Intel 白皮书：


[^1^] `Intel analysis of speculative execution side channels <https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/analysis-of-speculative-execution-side-channels-white-paper.pdf>`_.


[^2^] `Bounds check bypass <https://software.intel.com/security-software-guidance/software-guidance/bounds-check-bypass>`_.


[^3^] `Deep dive: Retpoline: A branch target injection mitigation <https://software.intel.com/security-software-guidance/insights/deep-dive-retpoline-branch-target-injection-mitigation>`_.


[^4^] `Deep Dive: Single Thread Indirect Branch Predictors <https://software.intel.com/security-software-guidance/insights/deep-dive-single-thread-indirect-branch-predictors>`_.

AMD 白皮书：


[^5^] `AMD64 technology indirect branch control extension <https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/white-papers/111006-architecture-guidelines-update-amd64-technology-indirect-branch-control-extension.pdf>`_.


[^6^] `Software techniques for managing speculation on AMD processors <https://developer.amd.com/wp-content/resources/Managing-Speculation-on-AMD-Processors.pdf>`_.

ARM 白皮书：


[^7^] `Cache speculation side-channels <https://developer.arm.com/support/arm-security-updates/speculative-processor-vulnerability/download-the-whitepaper>`_.


[^8^] `Cache speculation issues update <https://developer.arm.com/support/arm-security-updates/speculative-processor-vulnerability/latest-updates/cache-speculation-issues-update>`_.

Google 白皮书：


[^9^] `Retpoline: a software construct for preventing branch-target-injection <https://support.google.com/faqs/answer/7625886>`_.

MIPS 白皮书：


[^10^] `MIPS: response on speculative execution and side channel vulnerabilities <https://web.archive.org/web/20220512003005if_/https://www.mips.com/blog/mips-response-on-speculative-execution-and-side-channel-vulnerabilities/>`_.

学术论文：


[^11^] `Spectre Attacks: Exploiting Speculative Execution <https://spectreattack.com/spectre.pdf>`_.


[^12^] `NetSpectre: Read Arbitrary Memory over Network <https://arxiv.org/abs/1807.10535>`_.


[^13^] `Spectre Returns! Speculation Attacks using the Return Stack Buffer <https://www.usenix.org/system/files/conference/woot18/woot18-paper-koruyeh.pdf>`_.
