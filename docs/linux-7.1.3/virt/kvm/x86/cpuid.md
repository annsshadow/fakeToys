
## KVM CPUID bits


:Author: Glauber Costa <glommer@gmail.com>

运行在 KVM 宿主机上的客户机，可以通过 cpuid 检查其部分特性。这并不总是保证生效，因为用户空间可以在启动客户机之前，将部分甚至全部 KVM 相关的 cpuid 特性屏蔽掉。

KVM cpuid 函数为：

function: KVM_CPUID_SIGNATURE (0x40000000)

```

   eax = 0x40000001
   ebx = 0x4b4d564b
   ecx = 0x564b4d56
   edx = 0x4d

```
注意，ebx、ecx 和 edx 中的这个值对应于字符串 "KVMKVMKVM"。
eax 中的值对应于本 leaf 中存在的最大 cpuid 函数，若未来增加了更多函数，该值会随之更新。
另请注意，旧版宿主机将 eax 值设为 0x0。应将其解释为值 0x40000001。
本函数用于查询 KVM cpuid leaf 是否存在。

function: define KVM_CPUID_FEATURES (0x40000001)

```

          ebx, ecx
          eax = an OR'ed group of (1 << flag)

```
其中 `flag` 定义如下：

================================== =========== ================================
flag                               value       meaning
================================== =========== ================================
KVM_FEATURE_CLOCKSOURCE            0           kvmclock 在 msrs 0x11 和 0x12
                                               处可用

KVM_FEATURE_NOP_IO_DELAY           1           无需在 PIO 操作上执行延迟

KVM_FEATURE_MMU_OP                 2           已废弃

KVM_FEATURE_CLOCKSOURCE2           3           kvmclock 在 msrs 0x4b564d00 和
                                               0x4b564d01 处可用

KVM_FEATURE_ASYNC_PF               4           可通过写入 msr 0x4b564d02 启用
                                               async pf

KVM_FEATURE_STEAL_TIME             5           可通过写入 msr 0x4b564d03 启用
                                               steal time

KVM_FEATURE_PV_EOI                 6           可通过写入 msr 0x4b564d04 启用
                                               半虚拟化 end of interrupt 处理程序

KVM_FEATURE_PV_UNHALT              7           客户机在启用半虚拟化自旋锁支持前
                                               检查该特性位

KVM_FEATURE_PV_TLB_FLUSH           9           客户机在启用半虚拟化 tlb flush 前
                                               检查该特性位

KVM_FEATURE_ASYNC_PF_VMEXIT        10          可通过在写入 msr 0x4b564d02 时
                                               设置位 2 来启用半虚拟化 async PF
                                               VM EXIT

KVM_FEATURE_PV_SEND_IPI            11          客户机在启用半虚拟化发送 IPI 前
                                               检查该特性位

KVM_FEATURE_POLL_CONTROL           12          可通过写入 msr 0x4b564d05 禁用
                                               宿主机侧对 HLT 的轮询

KVM_FEATURE_PV_SCHED_YIELD         13          客户机在使用半虚拟化 sched yield
                                               前检查该特性位

KVM_FEATURE_ASYNC_PF_INT           14          客户机在使用第二个 async pf 控制
                                               msr 0x4b564d06 以及 async pf 确认
                                               msr 0x4b564d07 前检查该特性位

KVM_FEATURE_MSI_EXT_DEST_ID        15          客户机在 MSI 地址位 11-5 中使用
                                               扩展目标 ID 位前检查该特性位

KVM_FEATURE_HC_MAP_GPA_RANGE       16          客户机在使用 map gpa range hypercall
                                               通知页状态变更前检查该特性位

KVM_FEATURE_MIGRATION_CONTROL      17          客户机在使用 MSR_KVM_MIGRATION_CONTROL
                                               前检查该特性位

KVM_FEATURE_CLOCKSOURCE_STABLE_BIT 24          若客户机侧 kvmclock 中预期不会出现
                                               per-cpu 偏差，宿主机将发出警告
================================== =========== ================================

```

      edx = an OR'ed group of (1 << flag)

```
这里的 `flag` 定义如下：

================== ============ =================================
flag               value        meaning
================== ============ =================================
KVM_HINTS_REALTIME 0            客户机检查该特性位以确定 vCPU 不会
                                在无限长的时间内被抢占，从而允许进行
                                优化
================== ============ =================================
