
## ARM 虚拟中断转换服务（ITS）


支持的设备类型：
  KVM_DEV_TYPE_ARM_VGIC_ITS    ARM 中断转换服务控制器

ITS 允许将 MSI(-X) 中断注入到客户机中。该扩展是可选的。创建一个虚拟 ITS 控制器
还需要一个主机 GICv3（参见 arm-vgic-v3.txt），但不依赖于是否有物理 ITS 控制器。

每个客户机可以有多个 ITS 控制器，每个都必须有独立、不重叠的 MMIO 区域。


## 组


### KVM_DEV_ARM_VGIC_GRP_ADDR


  属性：
    KVM_VGIC_ITS_ADDR_TYPE (rw, 64-bit)
      GICv3 ITS 控制寄存器帧在客户机物理地址空间中的基地址。
      该地址需要 64K 对齐，并且该区域覆盖 128K。

  错误：

    =======  =================================================
    -E2BIG   地址超出可寻址的 IPA 范围
    -EINVAL  地址对齐不正确
    -EEXIST  地址已配置
    -EFAULT  attr->addr 的用户指针无效。
    -ENODEV  属性不正确或 ITS 不受支持。
    =======  =================================================


### KVM_DEV_ARM_VGIC_GRP_CTRL


  属性：
    KVM_DEV_ARM_VGIC_CTRL_INIT
      请求初始化 ITS，kvm_device_attr.addr 中没有额外参数。

    KVM_DEV_ARM_ITS_CTRL_RESET
      复位 ITS，kvm_device_attr.addr 中没有额外参数。
      参见“ITS 复位状态”一节。

    KVM_DEV_ARM_ITS_SAVE_TABLES
      将 ITS 表数据保存到客户机 RAM 中，位置由客户机在相应寄存器/表项中所提供。
      如果用户空间需要某种形式的脏页跟踪来识别哪些页被保存过程修改，它应使用一个
      位图，即使使用其它机制来跟踪由 vCPU 弄脏的内存。

      客户机内存中表的布局定义了一个 ABI。表项以小端格式排列，如最后一段所述。

    KVM_DEV_ARM_ITS_RESTORE_TABLES
      将 ITS 表从客户机 RAM 恢复到 ITS 内部结构。

      GICV3 必须在 ITS 之前恢复，并且除 GITS_CTLR 之外的所有 ITS 寄存器都必须在
      恢复 ITS 表之前恢复。

      GITS_IIDR 只读寄存器也必须在调用 KVM_DEV_ARM_ITS_RESTORE_TABLES 之前恢复，
      因为 IIDR 修订字段编码了 ABI 修订号。

      恢复 GICv3/ITS 时的预期顺序在“ITS 恢复序列”一节中描述。

  错误：

    =======  ==========================================================
     -ENXIO  ITS 在设置此属性之前未按要求正确配置
    -ENOMEM  分配 ITS 内部数据时内存不足
    -EINVAL  恢复的數据不一致
    -EFAULT  无效的客户机 ram 访问
    -EBUSY   一个或多个 VCPU 正在运行
    -EACCES  虚拟 ITS 由物理 GICv4 ITS 支撑，并且在没有 GICv4.1 的情况下状态不可用
    =======  ==========================================================

### KVM_DEV_ARM_VGIC_GRP_ITS_REGS


  属性：
      kvm_device_attr 的 attr 字段编码了 ITS 寄存器相对于 ITS 控制帧基地址
      （ITS_base）的偏移量。

      kvm_device_attr.addr 指向一个 __u64 值，无论被寻址寄存器的宽度（32/64 位）
      如何。64 位寄存器只能以完整长度访问。

      对只读寄存器的写入会被内核忽略，但以下除外：

      - GITS_CREADR。必须恢复它，否则队列中的命令会在恢复 CWRITER 后重新执行。
        GITS_CREADR 必须在恢复 GITS_CTLR（后者可能会启用 ITS）之前恢复。同时它必须
        在 GITS_CBASER 之后恢复，因为对 GITS_CBASER 的写入会重置 GITS_CREADR。
      - GITS_IIDR。Revision 字段编码了表布局 ABI 修订号。将来我们可能实现虚拟 LPI
        的直接注入。这将需要升级表布局以及 ABI 的演进。GITS_IIDR 必须在调用
        KVM_DEV_ARM_ITS_RESTORE_TABLES 之前恢复。

      对于其它寄存器，获取或设置一个寄存器与在真实硬件上读取/写入该寄存器具有相同的
      效果。

  错误：

    =======  ====================================================
    -ENXIO   偏移量不对应于任何受支持的寄存器
    -EFAULT  attr->addr 的用户指针无效
    -EINVAL  偏移量未 64 位对齐
    -EBUSY   一个或多个 VCPU 正在运行
    =======  ====================================================

### ITS 恢复序列：


在恢复 GIC、ITS 和 KVM_IRQFD 赋值时必须遵循以下顺序：

a) 恢复所有客户机内存并创建 vcpu
b) 恢复所有重分发器（redistributor）
c) 提供 ITS 基地址
   (KVM_DEV_ARM_VGIC_GRP_ADDR)
d) 按以下顺序恢复 ITS：

     1. 恢复 GITS_CBASER
     2. 恢复所有其它 `GITS_` 寄存器，但 GITS_CTLR 除外！
     3. 加载 ITS 表数据（KVM_DEV_ARM_ITS_RESTORE_TABLES）
     4. 恢复 GITS_CTLR

e) 恢复 MSI 的 KVM_IRQFD 赋值

然后 vcpu 可以启动。

### ITS 表 ABI REV0：


ABI 的修订 0 仅支持虚拟 GICv3 的特性，不支持带有嵌套虚拟机监控程序虚拟中断
直接注入支持的虚拟 GICv4。

设备表和 ITT 分别由 DeviceID 和 EventID 索引。集合表不由 CollectionID 索引，集合
中的表项以任意顺序列出。所有表项均为 8 字节。

```

   bits:     | 63| 62 ... 49 | 48 ... 5 | 4 ... 0 |
   values:   | V |   next    | ITT_addr |  Size   |

 where:

 - V 指示该表项是否有效。如果无效，其它字段没有意义。
 - next：如果此表项是最后一个，则等于 0；否则它对应于到下一个 DTE 的 DeviceID
   偏移量，上限为 2^14 -1。
 - ITT_addr 匹配 ITT 地址的 [51:8] 位（256 字节对齐）。
 - Size 指定 EventID 支持的位数减一

 Collection Table Entry (CTE)::

   bits:     | 63| 62 ..  52  | 51 ... 16 | 15  ...   0 |
   values:   | V |    RES0    |  RDBase   |    ICID     |

 where:

 - V 指示该表项是否有效。如果无效，其它字段没有意义。
 - RES0：保留字段，具有 Should-Be-Zero-or-Preserved 行为。
 - RDBase 是 PE 编号（GICR_TYPER.Processor_Number 语义），
 - ICID 是集合 ID

 Interrupt Translation Entry (ITE)::

   bits:     | 63 ... 48 | 47 ... 16 | 15 ... 0 |
   values:   |    next   |   pINTID  |  ICID    |

 where:

 - next：如果此表项是最后一个，则等于 0；否则它对应于到下一个 ITE 的 EventID
   偏移量，上限为 2^16 -1。
 - pINTID 是物理 LPI ID；如果为零，意味着该表项无效，其它字段没有意义。
 - ICID 是集合 ID

```
### ITS 复位状态：


RESET 将 ITS 返回到它首次被创建和初始化时的相同状态。当 RESET 命令返回时，保证
以下事项：

- ITS 未启用且静止
  GITS_CTLR.Enabled = 0 .Quiescent=1
- 没有内部缓存的状态
- 没有使用集合表或设备表
  GITS_BASER<n>.Valid = 0
- GITS_CBASER = 0, GITS_CREADR = 0, GITS_CWRITER = 0
- ABI 版本不变，并保持为 ITS 设备首次创建时所设置的版本。
