
## Linux KVM 超级调用（Hypercall


X86锛。
 KVM 超级调用vmcall vmmcall 指令的三字节序列组成。虚拟机监控器（hypervisor）可以将其替换为保证受支持的指令

 最多可以有四个参数分别通过 rbx、rcx、rdx rsi 传递。超级调用号应放rax 中，返回值将放在 rax 中。除非特定的超级调用明确说明，否则不会破坏其它寄存器

S390锛。
  R2-R7 用于参数 1-6。此外，R1 用于超级调用号。返回值写R2

  S390 使用诊断指令（diagnose instruction）作为超级调用（0x500），超级调用号放R1 中

  有关 KVM 支持S390 诊断调用的更多信息，请参Documentation/virt/kvm/s390/s390-diag.rst

PowerPC锛。
  它使R3-R10，超级调用号R11 中。R4-R11 用作输出寄存器。返回值放R3 中

  KVM 超级调用使用 4 字节操作码，这些操作码会被打补丁替换为设备树 /hypervisor 节点内的 'hypercall-instructions' 属性
  更多信息请参Documentation/virt/kvm/ppc-pv.rst

MIPS锛。
  KVM 超级调用使用 HYPCALL 指令，代码为 0，超级调用号$2 (v0) 中。最多四个参数可以放$4-$7 (a0-a3) 中，返回值放$2 (v0) 中

## KVM 超级调用文档


每个超级调用的模板为
1. 超级调用名称
2. 架构（Architecture(s)
3. 状态（deprecated 已弃用、obsolete 已废弃、active 生效中）
4. 用途（Purpose

### 1. KVM_HC_VAPIC_POLL_IRQ


:Architecture: x86
:Status: active（生效中
:Purpose: 触发客户机退出，以便宿主机可以在重新进入时检查是否有挂起的中断

### 2. KVM_HC_MMU_OP


:Architecture: x86
:Status: deprecated（已弃用）
:Purpose: 支持 MMU 操作，例如写PTE、刷TLB、释PT

### 3. KVM_HC_FEATURES


:Architecture: PPC
:Status: active（生效中
:Purpose: 向客户机暴露超级调用的可用性。在 x86 平台上，使用 cpuid 来枚举哪些超级调用可用。在 PPC 上，既可以使用基于设备树的查找（这也EPAPR 所规定的），也可以使用 KVM 特定的枚举机制（即本超级调用）

### 4. KVM_HC_PPC_MAP_MAGIC_PAGE


:Architecture: PPC
:Status: active（生效中
:Purpose: 为了在虚拟机监控器与客户机之间建立通信，存在一个共享页，其中包含部分管理程序可见的寄存器状态。客户机可以通过此超级调用将该共享页映射，从而通过内存访问其管理程序寄存器

### 5. KVM_HC_KICK_CPU


:Architecture: x86
:Status: active（生效中
:Purpose: 用于将处HLT 状态的 vcpu 唤醒的超级调
:Usage example:
  一个半虚拟化客户机vcpu 在客户机内核模式下忙等待某个事件发生（例如某个自旋锁变为可用）时，一旦忙等待超过某个阈值时间间隔，就可以执HLT 指令。执HLT 指令会导致虚拟机监控器将vcpu 置为睡眠，直到出现合适的事件。同一客户机的另一vcpu 可以通过发出 KVM_HC_KICK_CPU 超级调用并指定要唤醒vcpu APIC ID (a1) 来唤醒该睡眠中的 vcpu。超级调用中还有一个额外参(a0) 留作将来使用

### 6. KVM_HC_CLOCK_PAIRING


:Architecture: x86
:Status: active（生效中
:Purpose: 用于同步宿主机和客户机时钟的超级调用

用法

a0：宿主机复制 "struct kvm_clock_offset" 结构的客户机物理地址

a1：clock_type，目前仅支持 KVM_CLOCK_PAIRING_WALLCLOCK (0)（对应于宿主机的 CLOCK_REALTIME 时钟）

```

		struct kvm_clock_pairing {
			__s64 sec;
			__s64 nsec;
			__u64 tsc;
			__u32 flags;
			__u32 pad[9];
		};

       Where:
               * sec: seconds from clock_type clock.
               * nsec: nanoseconds from clock_type clock.
               * tsc: guest TSC value used to calculate sec/nsec pair
               * flags: flags, unused (0) at the moment.

```
该超级调用让客户机能够在宿主机和客户机之间计算精确的时间戳。客户机可以使用返回TSC 值，在同一时刻计算其时钟的 CLOCK_REALTIME

如果宿主机未使用 TSC 时钟源，或者时钟类型不同于 KVM_CLOCK_PAIRING_WALLCLOCK，则返回 KVM_EOPNOTSUPP

### 7. KVM_HC_SEND_IPI


:Architecture: x86
:Status: active（生效中
:Purpose: 向多vCPU 发IPI

- a0：目APIC ID 位图的低位部
- a1：目APIC ID 位图的高位部
- a2：位图中的最APIC ID
- a3：APIC ICR

该超级调用让客户机发送多IPI，在 64 位模式下每次超级调用最128 个目标，32 位模式下每次最64 vCPU。目标由前两个参数（a0 a1）中包含的位图表示。a0 的位 0 对应第三个参数（a2）中APIC ID，位 1 对应 a2+1，依此类推

返回成功投IPI CPU 数量

### 8. KVM_HC_SCHED_YIELD


:Architecture: x86
:Status: active（生效中
:Purpose: 如果 IPI 目标 vCPU 被抢占，则用于让步（yield）的超级调用

a0：目APIC ID

:Usage example: 当向 vCPU 发call-function IPI-many 时，如果任一 IPI 目标 vCPU 被抢占，则让步

### 9. KVM_HC_MAP_GPA_RANGE


:Architecture: x86
:Status: active（生效中
:Purpose: 请求 KVM 以指定的属性映射一GPA 范围

a0：起始页的客户机物理地址
a1：（4kb）页的数量（GPA 空间中必须连续）
a2：属

    其中 'attributes' 
        - 3:0 - 首选页大小编码 0 = 4kb = 2mb = 1gb，等等…
        - 4 - plaintext（明文）= 0，encrypted（加密）= 1
        - 63:5 - 保留（必须为 0

**实现说明**：该超级调用在用户空间通过 KVM_CAP_EXIT_HYPERCALL 能力实现。用户空间必须在客户CPUID 中通告 KVM_FEATURE_HC_MAP_GPA_RANGE 之前启用该能力。此外，如果客户机支KVM_FEATURE_MIGRATION_CONTROL，用户空间还必须设置一MSR 过滤器来处理MSR_KVM_MIGRATION_CONTROL 的写入
