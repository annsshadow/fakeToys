
## ARM 虚拟中断转换服务（ITS

支持的设备类型：
  KVM_DEV_TYPE_ARM_VGIC_ITS    ARM 中断转换服务控制
ITS 允许MSI(-X) 中断注入到客户机中。该扩展是可选的。创建一个虚ITS 控制还需要一个主GICv3（参arm-vgic-v3.txt），但不依赖于是否有物理 ITS 控制器
每个客户机可以有多个 ITS 控制器，每个都必须有独立、不重叠MMIO 区域

## 缁。

### KVM_DEV_ARM_VGIC_GRP_ADDR


  属性：
    KVM_VGIC_ITS_ADDR_TYPE (rw, 64-bit)
      GICv3 ITS 控制寄存器帧在客户机物理地址空间中的基地址      该地址需64K 对齐，并且该区域覆盖 128K
  错误
    =======  =================================================
    -E2BIG   地址超出可寻址IPA 范围
    -EINVAL  地址对齐不正    -EEXIST  地址已配    -EFAULT  attr->addr 的用户指针无效    -ENODEV  属性不正确ITS 不受支持    =======  =================================================


### KVM_DEV_ARM_VGIC_GRP_CTRL


  属性：
    KVM_DEV_ARM_VGIC_CTRL_INIT
      请求初始ITS，kvm_device_attr.addr 中没有额外参数
    KVM_DEV_ARM_ITS_CTRL_RESET
      复位 ITS，kvm_device_attr.addr 中没有额外参数      参见“ITS 复位状态”一节
    KVM_DEV_ARM_ITS_SAVE_TABLES
      ITS 表数据保存到客户RAM 中，位置由客户机在相应寄存器/表项中所提供      如果用户空间需要某种形式的脏页跟踪来识别哪些页被保存过程修改，它应使用一      位图，即使使用其它机制来跟踪vCPU 弄脏的内存
      客户机内存中表的布局定义了一ABI。表项以小端格式排列，如最后一段所述
    KVM_DEV_ARM_ITS_RESTORE_TABLES
      ITS 表从客户RAM 恢复ITS 内部结构
      GICV3 必须ITS 之前恢复，并且除 GITS_CTLR 之外的所ITS 寄存器都必须      恢复 ITS 表之前恢复
      GITS_IIDR 只读寄存器也必须在调KVM_DEV_ARM_ITS_RESTORE_TABLES 之前恢复      因为 IIDR 修订字段编码ABI 修订号
      恢复 GICv3/ITS 时的预期顺序在“ITS 恢复序列”一节中描述
  错误
    =======  ==========================================================
     -ENXIO  ITS 在设置此属性之前未按要求正确配    -ENOMEM  分配 ITS 内部数据时内存不    -EINVAL  恢复的數据不一    -EFAULT  无效的客户机 ram 访问
    -EBUSY   一个或多个 VCPU 正在运行
    -EACCES  虚拟 ITS 由物GICv4 ITS 支撑，并且在没有 GICv4.1 的情况下状态不可用
    =======  ==========================================================

### KVM_DEV_ARM_VGIC_GRP_ITS_REGS


  属性：
      kvm_device_attr attr 字段编码ITS 寄存器相对于 ITS 控制帧基地址
      （ITS_base）的偏移量
      kvm_device_attr.addr 指向一__u64 值，无论被寻址寄存器的宽度2/64 位）
      如何4 位寄存器只能以完整长度访问
      对只读寄存器的写入会被内核忽略，但以下除外：

      - GITS_CREADR。必须恢复它，否则队列中的命令会在恢CWRITER 后重新执行        GITS_CREADR 必须在恢GITS_CTLR（后者可能会启用 ITS）之前恢复。同时它必须
        GITS_CBASER 之后恢复，因为对 GITS_CBASER 的写入会重置 GITS_CREADR      - GITS_IIDR。Revision 字段编码了表布局 ABI 修订号。将来我们可能实现虚LPI
        的直接注入。这将需要升级表布局以及 ABI 的演进。GITS_IIDR 必须在调        KVM_DEV_ARM_ITS_RESTORE_TABLES 之前恢复
      对于其它寄存器，获取或设置一个寄存器与在真实硬件上读写入该寄存器具有相同      效果
  错误
    =======  ====================================================
    -ENXIO   偏移量不对应于任何受支持的寄存器
    -EFAULT  attr->addr 的用户指针无    -EINVAL  偏移量未 64 位对    -EBUSY   一个或多个 VCPU 正在运行
    =======  ====================================================

### ITS 恢复序列

在恢GIC、ITS KVM_IRQFD 赋值时必须遵循以下顺序
a) 恢复所有客户机内存并创vcpu
b) 恢复所有重分发器（redistributorc) 提供 ITS 基地址
   (KVM_DEV_ARM_VGIC_GRP_ADDR)
d) 按以下顺序恢ITS
     1. 恢复 GITS_CBASER
     2. 恢复所有其`GITS_` 寄存器，GITS_CTLR 除外     3. 加载 ITS 表数据（KVM_DEV_ARM_ITS_RESTORE_TABLES     4. 恢复 GITS_CTLR

e) 恢复 MSI KVM_IRQFD 赋
然后 vcpu 可以启动
### ITS 琛?ABI REV0锛。

ABI 的修0 仅支持虚GICv3 的特性，不支持带有嵌套虚拟机监控程序虚拟中断
直接注入支持的虚GICv4
设备表和 ITT 分别DeviceID EventID 索引。集合表不由 CollectionID 索引，集中的表项以任意顺序列出。所有表项均8 字节
```

   bits:     | 63| 62 ... 49 | 48 ... 5 | 4 ... 0 |
   values:   | V |   next    | ITT_addr |  Size   |

 where:

 - V 指示该表项是否有效。如果无效，其它字段没有意义 - next：如果此表项是最后一个，则等0；否则它对应于到下一DTE DeviceID
   偏移量，上限2^14 -1 - ITT_addr 匹配 ITT 地址[51:8] 位（256 字节对齐） - Size 指定 EventID 支持的位数减一

 Collection Table Entry (CTE)::

   bits:     | 63| 62 ..  52  | 51 ... 16 | 15  ...   0 |
   values:   | V |    RES0    |  RDBase   |    ICID     |

 where:

 - V 指示该表项是否有效。如果无效，其它字段没有意义 - RES0：保留字段，具有 Should-Be-Zero-or-Preserved 行为 - RDBase PE 编号（GICR_TYPER.Processor_Number 语义），
 - ICID 是集ID

 Interrupt Translation Entry (ITE)::

   bits:     | 63 ... 48 | 47 ... 16 | 15 ... 0 |
   values:   |    next   |   pINTID  |  ICID    |

 where:

 - next：如果此表项是最后一个，则等0；否则它对应于到下一ITE EventID
   偏移量，上限2^16 -1 - pINTID 是物LPI ID；如果为零，意味着该表项无效，其它字段没有意义 - ICID 是集ID

```
### ITS 复位状态：


RESET ITS 返回到它首次被创建和初始化时的相同状态。当 RESET 命令返回时，保证
以下事项
- ITS 未启用且静止
  GITS_CTLR.Enabled = 0 .Quiescent=1
- 没有内部缓存的状- 没有使用集合表或设备  GITS_BASER<n>.Valid = 0
- GITS_CBASER = 0, GITS_CREADR = 0, GITS_CWRITER = 0
- ABI 版本不变，并保持ITS 设备首次创建时所设置的版本