
## x86 特性标

## 简

/proc/cpuinfo 中的特性标志列表并不完整，它代表了很久以前将特性标志放到一个便于用户空间查找的位置的一次失败的尝试
然而，特性标志的数量随着每一CPU 而增长，导致 /proc/cpuinfo 变得无法解析且难以处理
更重要的是，这些特性标志甚至不需要出现在该文件中，因为用户空间并不关心它们——glibc 等已经使CPUID 来查明目标机器支持什么、不支持什么
而且，即使它没有显示某个特定的特性标志——尽管该 CPU 仍然支持相应的硬件功能，并且CPU 支持 CPUID faulting——用户空间也可以直接探测该特性并判断它是否被支持，而不管它是否在某个地方被通告
此外，这些标志字符串一旦出现便成为一ABI，而在没有任何东西使用它们的情况下永远维护它们是一种极大的浪费
因此，当/proc/cpuinfo 的用途是显示内核*启用***支持**的特性。也就是说：CPUID 特性标志存在，内核在启动期间进行了额外的设置，并且该功能已就绪可用。一个典型的例子是“user_shstk”，其中内核中存在额外的代码启用，以支持用户程序的影子栈（shadow stack）
所以，如果用户想知道某个特性在给定系统上是否可用，他们会尝试在 /proc/cpuinfo 中查找该标志。如果某个标志存在，则意味着
- 内核对该特性有足够的了解，拥有对应X86_FEATURE - 内核支持它，并且当前正将其提供给用户空间或内核的其他部分
- 如果该标志代表一个硬件特性，则硬件支持它

/proc/cpuinfo 中某个标志的缺失，对最终用户而言本身几乎不说明什么
一方面，像“vaes”这样的特性可能在尚未定义 X86_FEATURE_VAES 的内核上完全可供用户应用程序使用，因/proc/cpuinfo 中没有“vaes”
另一方面，在不支VAES 的硬件上运行的新内核同样不会有“vaes”出现在 /proc/cpuinfo 中。应用程序或用户无法区分这两种情况
最终结果是proc/cpuinfo 中的 flags 字段对内核调试略有用途，但对其他任何事情而言并非如此。应用程序应改用类似 glibc 的设施来查询 CPU 支持情况。用户应依赖 tools/arch/x86/kcpuid cpuid(1) 这样的工具
关于实现，出现在 /proc/cpuinfo 中的标志arch/x86/include/asm/cpufeatures.h 中有一X86_FEATURE 定义。这些标志既代表硬件特性，也代表软件特性
如果内核关心某个特性，或KVM 想要将该特性暴露给 KVM 客户机，那么只有当客户机需要解/proc/cpuinfo 时，它才应将其暴露给客户机。而如上所述，这种情况极不可能发生。KVM 可以合成 CPUID 位，KVM 客户机可以直接查CPUID 来判hypervisor 支持什么。正如已经说明的proc/cpuinfo 不是无用的特性标志的垃圾场

## 特性标志是如何创建的？


### 特性标志可以派生自 CPUID 叶的内容


这些特性定义的组织方式映射CPUID 叶的布局，并分组cpufeatures.h enum cpuid_leafs 所映射的、带有偏移的字（word）中（详arch/x86/include/asm/cpufeatures.h）。如果一个特性在 cpufeatures.h 中定义了 X86_FEATURE_<name>，并且在运行时被检测到，则相应标志会显示在 /proc/cpuinfo 中。例如，标志“avx2”来cpufeatures.h 中的 X86_FEATURE_AVX2
### 标志可以来自分散的基CPUID 的特

在稀疏填充的 CPUID 叶中枚举的硬件特性会获得软件定义的值。尽管如此，仍需要查CPUID 以确定给定特性是否存在。这init_scattered_cpuid_features() 完成。例如，X86_FEATURE_CQM_LLC 定义11*32 + 0，其存在性在运行时于相应CPUID [EAX=f, ECX=0] 的位 EDX[^1^] 中检查
分散 CPUID 叶的意图是不必要地膨胀 struct cpuinfo_x86.x86_capability[]。例如，CPUID [EAX=7, ECX=0] 30 个特性且很密集，CPUID [EAX=7, EAX=1] 只有一个特性，会在 x86_capability[] 数组中浪31 位空间。由于每个可能的 CPU 都有一struct cpuinfo_x86，浪费的内存并不小
### 在特定条件下可为硬件特性合成地创建标志


条件示例包括：某些特性是否存在于 MSR_IA32_CORE_CAPS 中，或是否识别了特定CPU 型号。如果所需条件满足，这些特性会通过 set_cpu_cap setup_force_cpu_cap 宏启用。例如，如果 MSR_IA32_CORE_CAPS 中设置了5 位，则会启用特X86_FEATURE_SPLIT_LOCK_DETECT，并显示“split_lock_detect”。标志“ring3mwait”仅在运行于 INTEL_XEON_PHI_[KNL|KNM] 处理器时显示
### 标志可以代表纯软件特

这些标志不代表硬件特性，而是代表内核中实现的软件特性。例如，内核页表隔离（Kernel Page Table Isolation）是纯软件特性，其特性标X86_FEATURE_PTI 也定义在 cpufeatures.h 中
## 标志的命

脚本 arch/x86/kernel/cpu/mkcapflags.sh 处理来自 cpufeatures.h #define X86_FEATURE_<name>，并kernel/cpu/capflags.c 中生x86_cap/bug_flags[] 数组。生成的 x86_cap/bug_flags[] 中的名称用于填充 /proc/cpuinfo。x86_cap/bug_flags[] 中标志的命名如下
### 标志默认不会出现/proc/cpuinfo 

特性标志默认从 /proc/cpuinfo 中省略，因为在大多数情况下将该特性暴露给用户空间没有意义。例如，X86_FEATURE_ALWAYS 定义cpufeatures.h 中，但该标志alternative 运行时补丁功能使用的一个内部内核特性。所以该标志不会出现/proc/cpuinfo 中
### 绝对需要时才指定标志名

如果 #define X86_FEATURE_* 所在行的注释以双引号字符（""）开头，双引号内的字符串将成为标志的名称。例如，标志“sse4_1”来自跟X86_FEATURE_XMM4_1 定义的注释“sse4_1”
在某些情况下需要覆盖标志的显示名称。例如，/proc/cpuinfo 是一个用户空间接口，必须保持不变。如果由于某种原X86_FEATURE_<name> 的命名发生了变化，应使用 /proc/cpuinfo 中已使用的名称来覆盖新的命名
## 当发生以下一种或多种情况时，标志会缺

### 硬件没有枚举出对它的支持


例如，当新内核运行在旧硬件上，或者该特性未被引导固件启用时。即使硬件是新的，在运行时启用该特性也可能出现问题，此时标志不会显示
### 内核不知道该标志


例如，当旧内核运行在新硬件上时
### 内核在编译时禁用了对它的支持


例如，如果构建时未启用线性地址掩码（LAM）（即未选择 CONFIG_ADDRESS_MASKING），则标志“lam”不会显示。即使该特性仍会通过 CPUID 被检测到，内核也会通过 setup_clear_cpu_cap(X86_FEATURE_LAM) 清除它来禁用它
### 该特性在引导时被禁用


特性可以通过命令行参数禁用，或者因为它未能被启用而禁用。命令行参数 clearcpuid= 可用于使/arch/x86/include/asm/cpufeatures.h 中定义的特性编号来禁用特性。例如，用户模式指令保护（User Mode Instruction Protection）可以使clearcpuid=514 禁用。数514 #define X86_FEATURE_UMIP (16*32 + 2) 计算得出
不要在生产环境中使用此命令行选项——它仅用作快速而粗糙的调试辅助，用来排除“启用特性的代码”是罪魁祸首。如果使用它，会给内核打上污点（taint）
此外，还存在各种自定义命令行参数用于禁用特定特性。参数列表包括但不限nofsgsbase、nosgx、noxsave 等 级分页也可以使用“no5lvl”禁用
### 已知该特性不可用


由于运行时缺少某个依赖项，已知该特性不可用。例如，如果禁用XSAVE 特性，AVX 标志不会出现，因为它们依赖于 XSAVE 特性。另一个例子是有缺陷的 CPU 以及缺失的微码补丁。因此，内核决定不启用该特性