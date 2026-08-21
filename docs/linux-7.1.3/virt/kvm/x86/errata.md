
## CPU 虚拟化的已知限制


每当对某CPU 特性进行完美模拟不可能或过于困难时，KVM 就必须在“完全不实现该特性”与
“在虚拟机与裸机系统之间引入行为差异”之间做出选择
本文档记录了 KVM 在虚拟化 CPU 特性方面的一些已知限制
## x86


### ``KVM_GET_SUPPORTED_CPUID`` 问题


#### x87 特

与大多数其他 CPUID 特性位不同，CPUID[EAX=7,ECX=0]:EBX[^6^]
（FDP_EXCPTN_ONLY）与 CPUID[EAX=7,ECX=0]:EBX]13]（ZERO_FCS_FDS）在特性存在时
被清除，在特性不存在时反而被置位
CPUID 中清除这些位guest 的运行没有影响；如果这些位在硬件上被置位，那么在该硬件上
运行的任何虚拟机都不会具备这些特性
**变通方案：** 建议guest CPUID 中始终置位这些位。不过请注意，任何期望这些特性存在的
软件（例`WIN87EM.DLL`）很可能早于这些 CPUID 特性位出现，因此无论如何都不知道要去检查它们
### ``KVM_SET_VCPU_EVENTS`` 问题


关于错误码的无效 KVM_SET_VCPU_EVENTS 输入**可能**导致 Intel CPU VM-Entry 失败。CET 之前Intel CPU 要求通过 VMCS 注入异常时正确设置“error code valid”标志，例如：注#GP 时要置位该标志，注入 #UD 时清除，注入软异常时清除等。将 IA32_VMX_BASIC[^56^] 枚举'1' Intel CPU 放宽VMX 的一致性检查，AMD CPU 则完全没有此类限制。KVM_SET_VCPU_EVENTS 不会
对向量与“has_error_code”进行合理性检查，KVM ABI 遵循 AMD 的行为
### 嵌套虚拟化特

AMD CPU 上，GIF 被清除时，由于断点寄存器匹配而产生的 #DB 异常或陷阱会CPU 忽略并丢弃CPU 依赖 VMM 来完全虚拟化这一行为，即使为 guest 启用vGIF（即 vGIF=0 并不会导CPU guest 运行时丢#DB）。鉴于该使用场景十分罕见，其复杂性并不合理，KVM 并未虚拟化这一行为一种处理方式是KVM 拦截 #DB，临时禁用断点，单步执行过该指令，然后重新启用断点
### x2APIC


当启KVM_X2APIC_API_USE_32BIT_IDS 时，KVM 会激活一hack/quirk，允许使用目vCPU x2APIC ID 向单vCPU 发送事件，即使目标 vCPU 启用了传统的 xAPIC，例如在具有 > 255 vCPU
的虚拟机上通过 INIT-SIPI 启动热插拔的 vCPU。该 quirk 的一个副作用是，如果多个 vCPU 拥有相同
的物APIC ID，KVM 只会将针对该 APIC ID 的事件投递给 vCPU ID 最小的那个 vCPU。如果未启用
KVM_X2APIC_API_USE_32BIT_IDS，KVM 在处理中断时遵循 x86 架构（所有匹配目APIC ID vCPU
都会收到中断）
### MTRR


KVM 不虚拟化 guest MTRR 内存类型。KVM 模拟MTRR MSR 的访问，guest 中的 {RD,WR}MSR
会如预期般工作，但在确定有效内存类型时，KVM 不会考虑 guest MTRR，而是将全guest 内存
视为具有 Writeback（WB）类型的 MTRR
### CR0.CD


KVM 不在 Intel CPU 上虚拟化 CR0.CD。与 MTRR MSR 类似，KVM 模拟CR0.CD 的访问，使对 CR0 加载与存储表现得如预期，但将 CR0.CD=1 并不会影guest 内存的可缓存性
注意，该 erratum 不影AMD CPU，后者在硬件中完全虚拟化 CR0.CD，即CR0.CD=1 时（即使guest 中运行）CPU 缓存置于“no fill”模式