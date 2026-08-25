
## POWER9 eXternal Interrupt Virtualization Engine (XIVE Gen1)


支持的设备类型：
  - KVM_DEV_TYPE_XIVE     POWER9 XIVE 中断控制1 
该设备充当虚拟机的终端控制器。它提供 KVM 接口，用于在底层 POWER9 XIVE 中断控制器中配置虚拟机的各种中断源
只能实例化一XIVE 实例。客户机 XIVE 设备需POWER9 主机，并且客户机操作系统应当支持 XIVE 原生利用（native exploitation）中断模式。否则，它应当运行在称为 XICS（POWER7/8）的遗留中断模式下
- 设备映射

  KVM 设备暴露XIVE 硬件上用于中断管理的不同 MMIO 区间。这些区间通过自定义的 VM 缺页（fault）处理程序，VMA 的形式暴露给客户机
  1. 线程中断管理区（TIMA
  每个线程都有一个关联的线程中断管理上下文，由一组寄存器组成。这些寄存器让线程能够处理优先级管理和中断确认。其中最重要的有
      - Interrupt Pending Buffer     (IPB)
      - Current Processor Priority   (CPPR)
      - Notification Source Register (NSR)

  它们以四个不同的页暴露给软件，每页提供一种具有不同特权级的视图。第一页用于物理线程上下文，第二页用于管理程序（hypervisor）。只有第三页（操作系统）和第四页（用户级）被暴露给客户机
  2. 事件状态缓冲区（ESB
  每个源都关联一个事件状态缓冲区（ESB），它对应一对偶奇数页，提供用于管理该源的命令：例如触发（trigger）、EOI、关闭该源等
  3. 设备直
  当设备被直通到客户机时，源中断来自不同的硬件控制器（PHB4），暴露给客户机ESB 页应当适应这种变化
  当设备的硬件中断被映射进或解除映射出客户IRQ 号空间时，会调用 passthru_irq 辅助函数 kvmppc_xive_set_mapped() kvmppc_xive_clr_mapped()。KVM 设备扩展了这些辅助函数，以清除正被映射的客户IRQ 号对应的 ESB 页，然后VM 缺页处理程序重新填充。该处理程序会插入与被直通设备的硬件中断相对应的 ESB 页；若设备已被移除，则插入初始的 IPI ESB 页
  ESB 重映射对客户机和操作系统的设备驱动是完全透明的。所有处理都VFIO 以及 KVM-PPC 中的上述辅助函数内完成
- 组：

1. KVM_DEV_XIVE_GRP_CTRL
     提供对设备的全局控制

  属性：
    1.1 KVM_DEV_XIVE_RESET（只写）
    复位中断控制器中关于源和事件队列的配置。供 kexec kdump 使用
    错误：无

    1.2 KVM_DEV_XIVE_EQ_SYNC（只写）
    同步所有源和队列，并将 EQ 页标记为脏。这是为了在迁移虚拟机时确保捕获到一致的内存状态
    错误：无

    1.3 KVM_DEV_XIVE_NR_SERVERS（只写）
    kvm_device_attr.addr 指向一__u32 值，该值为中断服务器编号的数量（即可能的最vcpu id 加一）
    错误
      =======  ==========================================
      -EINVAL  Value greater than KVM_MAX_VCPU_IDS.
      -EFAULT  Invalid user pointer for attr->addr.
      -EBUSY   A vCPU is already connected to the device.
      =======  ==========================================

2. KVM_DEV_XIVE_GRP_SOURCE（只写）
     XIVE 设备中初始化一个新的源并将其屏蔽（mask）
  属性：
    中断源编 (64-bit)

```
    bits:     | 63   ....  2 |   1   |   0
    values:   |    unused    | level | type

  - type:  0:MSI 1:LSI
  - level: assertion level in case of an LSI.

  Errors:

    =======  ==========================================
    -E2BIG   Interrupt source number is out of range
    -ENOMEM  Could not create a new source block
    -EFAULT  Invalid user pointer for attr->addr.
    -ENXIO   Could not allocate underlying HW interrupt
    =======  ==========================================

```
3. KVM_DEV_XIVE_GRP_SOURCE_CONFIG（只写）
     配置源的定向（targeting
  属性：
    中断源编 (64-bit)

```
    bits:     | 63   ....  33 |  32  | 31 .. 3 |  2 .. 0
    values:   |    eisn       | mask |  server | priority

  - priority: 0-7 interrupt priority level
  - server: CPU number chosen to handle the interrupt
  - mask: mask flag (unused)
  - eisn: Effective Interrupt Source Number

  Errors:

    =======  =======================================================
    -ENOENT  Unknown source number
    -EINVAL  Not initialized source number
    -EINVAL  Invalid priority
    -EINVAL  Invalid CPU number.
    -EFAULT  Invalid user pointer for attr->addr.
    -ENXIO   CPU event queues not configured or configuration of the
	     underlying HW interrupt failed
    -EBUSY   No CPU available to serve interrupt
    =======  =======================================================

```
4. KVM_DEV_XIVE_GRP_EQ_CONFIG（读写）
     配置某个 CPU 的事件队
  属性：
    EQ 描述符标识符 (64-bit)

```
    bits:     | 63   ....  32 | 31 .. 3 |  2 .. 0
    values:   |    unused     |  server | priority

  The kvm_device_attr.addr points to::

    struct kvm_ppc_xive_eq {
	__u32 flags;
	__u32 qshift;
	__u64 qaddr;
	__u32 qtoggle;
	__u32 qindex;
	__u8  pad[40];
    };

  - flags: queue flags
      KVM_XIVE_EQ_ALWAYS_NOTIFY (required)
	forces notification without using the coalescing mechanism
	provided by the XIVE END ESBs.
  - qshift: queue size (power of 2)
  - qaddr: real address of queue
  - qtoggle: current queue toggle bit
  - qindex: current queue index
  - pad: reserved for future use

  Errors:

    =======  =========================================
    -ENOENT  Invalid CPU number
    -EINVAL  Invalid priority
    -EINVAL  Invalid flags
    -EINVAL  Invalid queue size
    -EINVAL  Invalid queue address
    -EFAULT  Invalid user pointer for attr->addr.
    -EIO     Configuration of the underlying HW failed
    =======  =========================================

```
5. KVM_DEV_XIVE_GRP_SOURCE_SYNC（只写）
     同步该源以刷新事件通知

  属性：
    中断源编 (64-bit)

  错误
    =======  =============================
    -ENOENT  Unknown source number
    -EINVAL  Not initialized source number
    =======  =============================

- VCPU 状
  XIVE 中断控制器（IC）在称为 NVT 的内部结构中维护 VP 的中断状态。当某个 VP 未被调度到硬件处理器线程上时，若VP 是某个事件通知的目标，硬件就可以更新这一结构
  对于迁移而言，捕NVT 中缓存的 IPB 很重要，因为它合成了待处理中断的优先级。我们还会多捕获一些内容以报告调试信息
```
    bits:     |  63  ....  32  |  31  ....  0  |
    values:   |   TIMA word0   |   TIMA word1  |
    bits:     | 127       ..........       64  |
    values:   |            unused              |

```
- 迁移
  使用 XIVE 原生利用模式保存虚拟机状态时，应当遵循一个特定的顺序。当虚拟机停止时
  1. 屏蔽（mask）所有源（PQ=01）以停止事件流
  2. KVM 控制 KVM_DEV_XIVE_EQ_SYNC 同步 XIVE 设备，以刷新所有在途的事件通知并稳EQ。在此阶段，EQ 页被标记为脏，以确保它们在迁移序列中被传输
  3. 捕获源的定向状态、EQ 配置以及线程中断上下文寄存器的状态
  恢复过程类似
  1. 恢复 EQ 配置，因为定向（targeting）依赖于它  2. 恢复定向
  3. 恢复线程中断上下  4. 恢复源状  5. vCPU 运行
