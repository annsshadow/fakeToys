
## 通用 vcpu 接口


虚拟 CPU “设备” 也接受 ioctl KVM_SET_DEVICE_ATTR、KVM_GET_DEVICE_ATTR 和 KVM_HAS_DEVICE_ATTR。该接口使用与其他设备相同的 struct
kvm_device_attr，但目标是 VCPU 级别的设置和控制。

每个虚拟 CPU 的组和属性（如果有的话）是架构相关的。

## 1. 组：KVM_ARM_VCPU_PMU_V3_CTRL


:Architectures: ARM64

### 1.1. 属性：KVM_ARM_VCPU_PMU_V3_IRQ


:Parameters: in kvm_device_attr.addr the address for PMU overflow interrupt is a
	     pointer to an int

返回：

	 =======  ========================================================
	 -EBUSY   PMU 溢出中断已经设置
	 -EFAULT  读取中断号时出错
	 -ENXIO   PMUv3 不支持，或者尝试获取时溢出中断未设置
	 -ENODEV  VCPU 缺少 KVM_ARM_VCPU_PMU_V3 特性
	 -EINVAL  提供了无效的 PMU 溢出中断号，或者
		  在未使用内核内 irqchip 的情况下尝试设置 IRQ 号。
	 =======  ========================================================

描述此 vcpu 的 PMUv3（Performance Monitor Unit v3，性能监视单元 v3）溢出中断号的一个值。该中断可以是 PPI 或 SPI，但每个 vcpu 的中断类型必须相同。作为 PPI 时，所有 vcpu 的中断号相同；而作为 SPI 时，每个 vcpu 必须是单独的中断号。对于基于 GICv5 的客户机，必须使用架构规定的 PPI（23）。

### 1.2 属性：KVM_ARM_VCPU_PMU_V3_INIT


:Parameters: no additional parameter in kvm_device_attr.addr

返回：

	 =======  ======================================================
	 -EEXIST  中断号已被使用
	 -ENODEV  PMUv3 不支持或 GIC 未初始化
	 -ENXIO   PMUv3 不支持、缺少 VCPU 特性或中断号未设置
		  （仅非 GICv5 客户机）
	 -EBUSY   PMUv3 已经初始化
	 =======  ======================================================

请求初始化 PMUv3。如果配合内核内虚拟 GIC 实现使用 PMUv3，这必须在初始化内核内 irqchip 之后进行。

### 1.3 属性：KVM_ARM_VCPU_PMU_V3_FILTER


:Parameters: in kvm_device_attr.addr the address for a PMU event filter is a
             pointer to a struct kvm_pmu_event_filter

:Returns:

	 =======  ======================================================
	 -ENODEV  PMUv3 不支持或 GIC 未初始化
	 -ENXIO   PMUv3 未正确配置，或者调用此属性前未按要求
	 	  配置内核内 irqchip
	 -EBUSY   PMUv3 已经初始化，或者某个 VCPU 已经运行过
	 -EINVAL  无效的过滤器范围
	 =======  ======================================================

```

    struct kvm_pmu_event_filter {
	    __u16	base_event;
	    __u16	nevents;

    #define KVM_PMU_EVENT_ALLOW	0
    #define KVM_PMU_EVENT_DENY	1

	    __u8	action;
	    __u8	pad[3];
    };

```
一个过滤器范围定义为范围 [@base_event, @base_event + @nevents)，连同 @action（KVM_PMU_EVENT_ALLOW 或 KVM_PMU_EVENT_DENY）。第一个注册的范围定义了全局策略（如果第一个 @action 是 DENY，则为全局 ALLOW；如果第一个 @action 是 ALLOW，则为全局 DENY）。可以编程多个范围，并且必须适配 PMU 架构所定义的事件空间（ARMv8.0 上为 10 位，从 ARMv8.1 起为 16 位）。

注意：通过为同一范围注册相反的动作来 “取消” 一个过滤器并不会改变默认动作。例如，先将事件范围 [0:10) 的 ALLOW 过滤器作为第一个过滤器安装，然后对该范围应用 DENY 动作，将使整个范围保持禁用状态。

限制：事件 0（SW_INCR）永远不会被过滤，因为它不统计硬件事件。过滤事件 0x1E（CHAIN）也没有效果，因为它严格来说不是一个事件。可以使用事件 0x11（CPU_CYCLES）来过滤周期计数器。

### 1.4 属性：KVM_ARM_VCPU_PMU_V3_SET_PMU


:Parameters: in kvm_device_attr.addr the address to an int representing the PMU
             identifier.

:Returns:

	 =======  ====================================================
	 -EBUSY   PMUv3 已经初始化、某个 VCPU 已经运行过，或者
                  已经设置了一个事件过滤器
	 -EFAULT  访问 PMU 标识符时出错
	 -ENXIO   未找到 PMU
	 -ENODEV  PMUv3 不支持或 GIC 未初始化
	 -ENOMEM  无法分配内存
	 =======  ====================================================

请求 VCPU 在创建客户机事件用于 PMU 仿真时使用指定的硬件 PMU。PMU 标识符可以从 /sys/devices 下所需 PMU 实例的 “type” 文件（或等价的 /sys/bus/even_source）读取。此属性在至少有两个 CPU PMU 的异构系统上特别有用。为一个 VCPU 设置的 PMU 将被所有其他 VCPU 使用。如果已经存在 PMU 事件过滤器，则无法设置 PMU。

注意，KVM 不会尝试将此属性指定的、与 PMU 相关联的物理 CPU 上运行 VCPU。这完全留给用户空间处理。然而，尝试在与 PMU 不支持的物理 CPU 上运行 VCPU 将会失败，KVM_RUN 将以
exit_reason = KVM_EXIT_FAIL_ENTRY 返回，并通过将 hardare_entry_failure_reason 字段设为 KVM_EXIT_FAIL_ENTRY_CPU_UNSUPPORTED、将 cpu 字段设为处理器 id 来填充 fail_entry 结构。

### 1.5 属性：KVM_ARM_VCPU_PMU_V3_SET_NR_COUNTERS


:Parameters: in kvm_device_attr.addr the address to an unsigned int
	     representing the maximum value taken by PMCR_EL0.N

:Returns:

	 =======  ====================================================
	 -EBUSY   PMUv3 已经初始化、某个 VCPU 已经运行过，或者
                  已经设置了事件过滤器
	 -EFAULT  访问 addr 所指向的值时出错
	 -ENODEV  PMUv3 不支持或 GIC 未初始化
	 -EINVAL  未显式选择 PMUv3，或者 N 的值超出范围
	 =======  ====================================================

设置虚拟 PMU 中实现的事件计数器数量。这要求已通过 KVM_ARM_VCPU_PMU_V3_SET_PMU 显式选择了一个 PMU，并且当未显式选择 PMU、或者计数器数量超出所选 PMU 的范围时会失败。选择新的 PMU 会取消设置此属性的效果。

## 2. 组：KVM_ARM_VCPU_TIMER_CTRL


:Architectures: ARM64

### 2.1. 属性：KVM_ARM_VCPU_TIMER_IRQ_{VTIMER,PTIMER,HVTIMER,HPTIMER}


:Parameters: in kvm_device_attr.addr the address for the timer interrupt is a
	     pointer to an int

返回：

	 =======  =================================
	 -EINVAL  无效的定时器中断号
	 -EBUSY   一个或多个 VCPU 已经运行
	 =======  =================================

描述连接到内核内虚拟 GIC 时的架构定时器中断号。它们必须是 PPI（16 <= intid < 32）。设置该属性会覆盖默认值（见下文）。

==============================  ==========================================
KVM_ARM_VCPU_TIMER_IRQ_VTIMER   EL1 虚拟定时器 intid（默认：27）
KVM_ARM_VCPU_TIMER_IRQ_PTIMER   EL1 物理定时器 intid（默认：30）
KVM_ARM_VCPU_TIMER_IRQ_HVTIMER  EL2 虚拟定时器 intid（默认：28）
KVM_ARM_VCPU_TIMER_IRQ_HPTIMER  EL2 物理定时器 intid（默认：26）
==============================  ==========================================

为不同的定时器设置相同的 PPI 会阻止 VCPU 运行。在某个 VCPU 上设置中断号会将当时创建的所有 VCPU 配置为对给定定时器使用该号码，覆盖其他 VCPU 上之前配置的任何值。用户空间应在创建所有 VCPU 之后、运行任何 VCPU 之前，在至少一个 VCPU 上配置中断号。


## 3. 组：KVM_ARM_VCPU_PVTIME_CTRL


:Architectures: ARM64

### 3.1 属性：KVM_ARM_VCPU_PVTIME_IPA


:Parameters: 64-bit base address

返回：

	 =======  ======================================
	 -ENXIO   未实现窃取时间
	 -EEXIST  此 VCPU 的基地址已经设置
	 -EINVAL  基地址未按 64 字节对齐
	 =======  ======================================

指定此 VCPU 的窃取时间结构的基地址。基地址必须按 64 字节对齐，并且位于有效的客户机内存区域内。更多信息（包括窃取时间结构的布局）请参见 Documentation/virt/kvm/arm/pvtime.rst。

## 4. 组：KVM_VCPU_TSC_CTRL


:Architectures: x86

4.1 属性：KVM_VCPU_TSC_OFFSET

:Parameters: 64-bit unsigned TSC offset

返回：

	 ======= ======================================
	 -EFAULT 读取/写入所提供的参数地址时出错。
	 -ENXIO  属性不受支持
	 ======= ======================================

指定客户机相对于主机的 TSC 偏移。客户机的 TSC 然后通过以下等式推导：

  guest_tsc = host_tsc + KVM_VCPU_TSC_OFFSET

此属性可用于在实时迁移时调整客户机的 TSC，使 TSC 计入 VM 被暂停期间的时间。下面描述了用于此目的的一种可能算法。

来自源 VMM 进程：

1. 调用 KVM_GET_CLOCK ioctl 记录主机 TSC（tsc_src）、kvmclock 纳秒（guest_src）和主机 CLOCK_REALTIME 纳秒（host_src）。

2. 读取每个 vCPU 的 KVM_VCPU_TSC_OFFSET 属性以记录客户机 TSC 偏移（ofs_src[i]）。

3. 调用 KVM_GET_TSC_KHZ ioctl 记录客户机 TSC 的频率（freq）。

来自目标 VMM 进程：

4. 调用 KVM_SET_CLOCK ioctl，在各自字段中提供来自 kvmclock 的源纳秒（guest_src）和 CLOCK_REALTIME（host_src）。确保在所提供的结构中设置了 KVM_CLOCK_REALTIME 标志。

   KVM 将推进 VM 的 kvmclock，以计入记录时钟值以来经过的时间。注意，除非源和目标之间的 CLOCK_REALTIME 是同步的，并且源暂停 VM 与目标执行步骤 4-7 之间经过的时间足够短，否则这会在客户机中引发问题（例如超时）。

5. 调用 KVM_GET_CLOCK ioctl 记录主机 TSC（tsc_dest）和 kvmclock 纳秒（guest_dest）。

6. 调整每个 vCPU 的客户机 TSC 偏移，以计入（1）记录状态以来经过的时间，以及（2）源机器和目标机器之间 TSC 的差异：

   ofs_dst[i] = ofs_src[i] -
     (guest_src - guest_dest) * freq +
     (tsc_src - tsc_dest)

   （“ofs[i] + tsc - guest * freq” 是对应于 kvmclock 中时间 0 的客户机 TSC 值。上述公式确保它与源上相同，在目标上也相同）。

7. 用前一步推导出的各自值写出每个 vCPU 的 KVM_VCPU_TSC_OFFSET 属性。
