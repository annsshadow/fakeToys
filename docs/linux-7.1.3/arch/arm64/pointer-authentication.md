## AArch64 Linux 中的指针认证


Author: Mark Rutland <mark.rutland@arm.com>

Date: 2017-07-19

本文档简要描AArch64 Linux 中指针认证（pointer authentication）功能的提供
### 架构概述


ARMv8.3 指针认证扩展添加了可用于缓解某些类别攻击的原语，这类攻击中攻击者可以破坏某些内存（例如栈）的内容
该扩展使用指针认证码（PAC，Pointer Authentication Code）来判断指针是否被意外修改。PAC 由指针、另一个值（如栈指针）以及保存在系统寄存器中的密钥推导而来
该扩展添加了将有PAC 插入指针、以及验移除指针PAC 的指令。PAC 占据指针的若干高位，其数量取决于配置的虚拟地址大小以及是否使用了指针标记（pointer tagging）
这些指令的一个子集从 HINT 编码空间中分配。在缺少该扩展（或被禁用）的情况下，这些指令表现NOP。无论是否存在该扩展，使用这些指令的应用程序和库都能正确运行
该扩展提供了五个独立的密钥来生成 PAC——两个用于指令地址（APIAKey、APIBKey），两个用于数据地址（APDAKey、APDBKey），以及一个用于通用认证（APGAKey）
### 基本支持


当选中 CONFIG_ARM64_PTR_AUTH，且存在相关硬件支持时，内核将在 exec*() 时为每个进程分配随机的密钥值。这些密钥由进程内的所有线程共享，并在 fork() 时保留
地址认证功能的存在通过 HWCAP_PACA 通告，通用认证功能通过 HWCAP_PACG 通告
PAC 在指针中占据的位数等55 减去内核配置的虚拟地址大小。例如，在虚拟地址大小48 时，PAC 宽度7 位
当选中 ARM64_PTR_AUTH_KERNEL 时，内核将使HINT 空间的指针认证指令编译，以保护函数返回。使用该选项构建的内核可以在支持或不支持指针认证的硬件上工作
除了 exec() 之外，也可以使用 PR_PAC_RESET_KEYS prctl 将密钥重新初始化为随机值。由 PR_PAC_APIAKEY、PR_PAC_APIBKEY、PR_PAC_APDAKEY、PR_PAC_APDBKEY PR_PAC_APGAKEY 组成的位掩码指定要重新初始化哪些密钥；指0 表示“所有密钥”
### 调试


当选中 CONFIG_ARM64_PTR_AUTH，且存在地址认证的硬件支持时，内核将NT_ARM_PAC_MASK regset（struct user_pac_mask）中暴露 TTBR0 PAC 位的位置，用户空间可以通过 PTRACE_GETREGSET 获取
regset 仅在设置HWCAP_PACA 时暴露。为数据指针和指令指针分别暴露了独立的掩码，因为两者的 PAC 位集合可能不同。注意，这些掩码适用TTBR0 地址，并且不能用于应用于 TTBR1 地址（例如内核指针）
此外，当同时选中CONFIG_CHECKPOINT_RESTORE 时，内核将暴NT_ARM_PACA_KEYS NT_ARM_PACG_KEYS regset（struct user_pac_address_keys struct user_pac_generic_keys）。它们可用于获取和设置某个线程的密钥
### 虚拟

当每个虚CPU 通过传递标KVM_ARM_VCPU_PTRAUTH_[ADDRESS/GENERIC] 并请求启用这两个独立CPU 特性来初始化时，KVM 客户机中启用指针认证。当前的 KVM 客户机实现通过将两个特性一起启用来工作，因此在启用指针认证之前会检查这两个用户空间标志。独立的用户空间标志将允许在未来添加支持以允许这两个特性相互独立地启用时，不会产生用户空间 ABI 变更
由于 Arm 架构规定指针认证特性与 VHE 特性一起实现，因此 KVM arm64 ptrauth 代码依赖于存VHE 模式
此外，当未设置这vcpu 特性标志时，KVM 将从 KVM_GET/SET_REG_* ioctl 中过滤掉指针认证系统密钥寄存器，并从 cpufeature ID 寄存器中屏蔽这些特性。任何使用指针认证指令的尝试都将导致向客户机注入一UNDEFINED 异常
### 启用和禁用密

prctl PR_PAC_SET_ENABLED_KEYS 允许用户程序控制某个特定任务中启用哪PAC 密钥。它接受两个参数，第一个是 PR_PAC_APIAKEY、PR_PAC_APIBKEY、PR_PAC_APDAKEY PR_PAC_APDBKEY 的位掩码，指定哪些密钥受prctl 影响，第二个是相同位的位掩码，指定该密钥
```

  prctl(PR_PAC_SET_ENABLED_KEYS,
        PR_PAC_APIAKEY | PR_PAC_APIBKEY | PR_PAC_APDAKEY | PR_PAC_APDBKEY,
        PR_PAC_APIBKEY, 0, 0);

```
禁用IB 密钥以外的所有密钥
这样做有用的主要原因是，能够启用一个使PAC 指令对函数指针和函数外部暴露的其他指针进行签名和认证的用户空ABI，同时仍然允许符合该 ABI 的二进制文件与不对指针签名或认证的旧二进制文件互操作
其思路是，动态加载器或早期启动代码会在确定进程可能加载旧二进制文件之后、但在执行任PAC 指令之前，非常早地发出此 prctl
为了与先前的内核版本兼容，进程启动时 IA、IB、DA DB 是启用的，并exec() 时重置为该状态。通过 fork() clone() 创建的进程从调用进程继承密钥启用状态
建议避免禁用 IA 密钥，因为这比其他任何密钥的禁用都有更高的性能开销