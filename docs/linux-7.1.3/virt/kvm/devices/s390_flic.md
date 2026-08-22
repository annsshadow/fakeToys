
## FLIC（floating interrupt controller，浮动中断控制器


FLIC 处理浮动（非CPU）中断，I/O、服务以及某些机器检查（machine check）中断
所有中断都存储在每 VM 的挂起中断列表中。FLIC 对该列表执行操作

只能实例化一FLIC 实例

FLIC 提供以下支持
- 添加中断（KVM_DEV_FLIC_ENQUEUE
- 检查当前挂起的中断（KVM_FLIC_GET_ALL_IRQS
- 清除所有挂起的浮动中断（KVM_DEV_FLIC_CLEAR_IRQS
- 清除一个挂起的浮动 I/O 中断（KVM_DEV_FLIC_CLEAR_IO_IRQ
- 为客机启禁用透明的异步页错误（async page faults
- 注册和修改适配器中断源（KVM_DEV_FLIC_ADAPTER_*
- 修改 AIS（adapter-interruption-suppression，适配器中断抑制）模式状态（KVM_DEV_FLIC_AISM
- 在指定适配器上注入适配器中断（KVM_DEV_FLIC_AIRQ_INJECT
- 获取/设置所AIS 模式状态（KVM_DEV_FLIC_AISM_ALL

组：
  KVM_DEV_FLIC_ENQUEUE
    将一个缓冲区和长度传入内核，随后它们被注入到挂起中断列表中
    attr->addr 包含指向缓冲区的指针，attr->attr 包含缓冲区的长度
    从用户空间复制的数据结构 kvm_s390_irq 的格式定义于 usr/include/linux/kvm.h

  KVM_DEV_FLIC_GET_ALL_IRQS
    将所有浮动中断复制到一个由用户空间提供的缓冲区中
    当缓冲区太小时返-ENOMEM，这是指示用户空间用一个更大的缓冲区重试

    -ENOBUFS 在分配内核空间缓冲区失败时返回

    -EFAULT 在将数据复制到用户空间失败时返回。所有中断保持挂起，即不会被从当
    挂起中断列表中删除。attr->addr 包含用户空间缓冲区的地址，所有中断数据将被复
    到该缓冲区。attr->attr 包含缓冲区的大小（字节）

  KVM_DEV_FLIC_CLEAR_IRQS
    简单地从当前挂起的浮动中断列表中删除所有元素。没有中断被注入到客机

  KVM_DEV_FLIC_CLEAR_IO_IRQ
    删除一个（如果存在）I/O 中断，该中断针对attr->addr（地址）和 attr->attr（长度）
    所指定缓冲区传入的子系统标识字（subsystem identification word）所标识的子通道
    （subchannel）

  KVM_DEV_FLIC_APF_ENABLE
    为客机启用异步页错误。因此在大页错误（major page fault）情况下，宿主机被允许异
    处理它并继续运行客机

    -EINVAL 在针ucontrol VM FLIC 调用时返回

  KVM_DEV_FLIC_APF_DISABLE_WAIT
    为客机禁用异步页错误，并等待直到已经挂起的异步页错误完成。这对于在迁移中断列
    之前为每init 中断触发一个完成中断是必要的

    -EINVAL 在针ucontrol VM FLIC 调用时返回

  KVM_DEV_FLIC_ADAPTER_REGISTER
    注册一I/O 适配器中断源。接受一kvm_s390_io_adapter
```

	struct kvm_s390_io_adapter {
		__u32 id;
		__u8 isc;
		__u8 maskable;
		__u8 swap;
		__u8 flags;
	};

   id contains the unique id for the adapter, isc the I/O interruption subclass
   to use, maskable whether this adapter may be masked (interrupts turned off),
   swap whether the indicators need to be byte swapped, and flags contains
   further characteristics of the adapter.

   Currently defined values for 'flags' are:

   - KVM_S390_ADAPTER_SUPPRESSIBLE: adapter is subject to AIS
     (adapter-interrupt-suppression) facility. This flag only has an effect if
     the AIS capability is enabled.

   Unknown flag values are ignored.


  KVM_DEV_FLIC_ADAPTER_MODIFY
    Modifies attributes of an existing I/O adapter interrupt source. Takes
    a kvm_s390_io_adapter_req specifying the adapter and the operation::

	struct kvm_s390_io_adapter_req {
		__u32 id;
		__u8 type;
		__u8 mask;
		__u16 pad0;
		__u64 addr;
	};

    id specifies the adapter and type the operation. The supported operations
    are:

    KVM_S390_IO_ADAPTER_MASK
      mask or unmask the adapter, as specified in mask

    KVM_S390_IO_ADAPTER_MAP
      This is now a no-op. The mapping is purely done by the irq route.
    KVM_S390_IO_ADAPTER_UNMAP
      This is now a no-op. The mapping is purely done by the irq route.

  KVM_DEV_FLIC_AISM
    modify the adapter-interruption-suppression mode for a given isc if the
    AIS capability is enabled. Takes a kvm_s390_ais_req describing::

	struct kvm_s390_ais_req {
		__u8 isc;
		__u16 mode;
	};

    isc contains the target I/O interruption subclass, mode the target
    adapter-interruption-suppression mode. The following modes are
    currently supported:

    - KVM_S390_AIS_MODE_ALL: ALL-Interruptions Mode, i.e. airq injection
      is always allowed;
    - KVM_S390_AIS_MODE_SINGLE: SINGLE-Interruption Mode, i.e. airq
      injection is only allowed once and the following adapter interrupts
      will be suppressed until the mode is set again to ALL-Interruptions
      or SINGLE-Interruption mode.

  KVM_DEV_FLIC_AIRQ_INJECT
    Inject adapter interrupts on a specified adapter.
    attr->attr contains the unique id for the adapter, which allows for
    adapter-specific checks and actions.
    For adapters subject to AIS, handle the airq injection suppression for
    an isc according to the adapter-interruption-suppression mode on condition
    that the AIS capability is enabled.

  KVM_DEV_FLIC_AISM_ALL
    Gets or sets the adapter-interruption-suppression mode for all ISCs. Takes
    a kvm_s390_ais_all describing::

	struct kvm_s390_ais_all {
	       __u8 simm; /* Single-Interruption-Mode mask */
	       __u8 nimm; /* No-Interruption-Mode mask *
	};

    simm contains Single-Interruption-Mode mask for all ISCs, nimm contains
    No-Interruption-Mode mask for all ISCs. Each bit in simm and nimm corresponds
    to an ISC (MSB0 bit 0 to ISC 0 and so on). The combination of simm bit and
    nimm bit presents AIS mode for a ISC.

    KVM_DEV_FLIC_AISM_ALL is indicated by KVM_CAP_S390_AIS_MIGRATION.

```
注意：在 FLIC 上执行的带有未知组或属性的 KVM_SET_DEVICE_ATTR/KVM_GET_DEVICE_ATTR 设备 ioctl 会给出错误码 EINVAL（而不API 文档中规定的 ENXIO）。无法基于使用尝试所产生的错误码来推断某FLIC 操作不可用

  指定了零 schid
