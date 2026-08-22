
## ARM 虚拟通用中断控制v3 及更高版本（VGICv3



支持的设备类型：
  - KVM_DEV_TYPE_ARM_VGIC_V3     ARM 通用中断控制v3.0

通过API 只能实例化一VGIC 实例。创建的 VGIC 将充VM 的中断控制器，要求被模拟
用户空间设备将中断注入到 VGIC，而不是直接注入到 CPU。不可能在同一VM 上同时创GICv3
鍜?GICv2銆。

创建客机 GICv3 设备需要一个主GICv3 主机，或一个支FEAT_GCIE_LEGACY GICv5 主机


组：
  KVM_DEV_ARM_VGIC_GRP_ADDR
   属性：

    KVM_VGIC_V3_ADDR_TYPE_DIST (rw, 64-bit)
      在客机物理地址空间GICv3 分配器（distributor）寄存器映射的基地址
      仅对 KVM_DEV_TYPE_ARM_VGIC_V3 有效。该地址需64K 对齐，且区域覆盖 64 KByte

    KVM_VGIC_V3_ADDR_TYPE_REDIST (rw, 64-bit)
      在客机物理地址空间GICv3 重分发器（redistributor）寄存器映射的基地址
      每个 VCPU 有两64K 页，所有重分发器页是连续的。仅KVM_DEV_TYPE_ARM_VGIC_V3
      有效。该地址需64K 对齐

    KVM_VGIC_V3_ADDR_TYPE_REDIST_REGION (rw, 64-bit)
```

        bits:     | 63   ....  52  |  51   ....   16 | 15 - 12  |11 - 0
        values:   |     count      |       base      |  flags   | index

      - index 编码唯一的重分发器区域索
      - flags：为未来使用保留，当前为 0
      - base 字段编码该区域中第一个重分发器客机物理基地址[51:16] 位
      - count 编码该区域中重分发器的数量。必须大0

```

      该区域中每个重分发器有两64K 页，重分发器在区域内连续布局。区域按索引顺序填充
      重分发器。所有区count 字段之和必须大于或等VCPU 的数量。重分发器区域必须按递增
      索引顺序注册，从索引 0 开始

      特定重分发器区域的特性可以通过attr 数据中预index 字段来读取。仅
      KVM_DEV_TYPE_ARM_VGIC_V3 有效

  混合使用 KVM_VGIC_V3_ADDR_TYPE_REDIST KVM_VGIC_V3_ADDR_TYPE_REDIST_REGION 属性是无效的

  注意：为了获得可复现的结果（在保恢复操作中同一VCPU 关联到同一个重分发器），VCPU
  的创建顺序、重分发器区域的创建顺序以及二者各自的交错方式都必须保持不变。任一顺序的改
  都可能导致不同的 vcpu_id/重分发器关联，从而导VM 在恢复时无法运行

  错误

    =======  =============================================================
    -E2BIG   Address outside of addressable IPA range
    -EINVAL  地址对齐不正确、重分发器区count/index 错误
             混用重分发器区域属
    -EEXIST  地址已配
    -ENOENT  尝试读取不存在的
             重分发器区域的特
    -ENXIO   该设备的组或属性未不受支持
             或缺少硬件支持
    -EFAULT  attr->addr 的用户指针无效
    -EBUSY   尝试写入初始化后为只读的
             瀵勫瓨鍣。
    =======  =============================================================


  KVM_DEV_ARM_VGIC_GRP_DIST_REGS, KVM_DEV_ARM_VGIC_GRP_REDIST_REGS
   属性：

    kvm_device_attr attr 字段编码两个:

      bits:     | 63   ....  32  |  31   ....    0 |
      values:   |      mpidr     |      offset     |

    所有分配器寄存器都(rw, 32-bit)，且 kvm_device_attr.addr 指向一__u32 值
    64 位寄存器必须通过分别访问低字和高字来进行访问

    对只读寄存器的写入会被内核忽略

    KVM_DEV_ARM_VGIC_GRP_DIST_REGS 访问主分配器寄存器
    KVM_DEV_ARM_VGIC_GRP_REDIST_REGS 访问mpidr 指定CPU 的重分发器

    offset 相对GICv3/4 规范中定义的“[Re]Distributor 基地址”。获取或设置这样一个寄存器
    效果与在真实硬件上读取或写入该寄存器相同，但以下寄存器除外：GICD_STATUSR、GICR_STATUSR
    GICD_ISPENDR、GICR_ISPENDR0、GICD_ICPENDR GICR_ICPENDR0。与它们架构定义的行为相比，
    通过这些接口访问这些寄存器时的行为有所不同，以便软件能够完整查VGIC 的内部状态

    mpidr 字段用于指定访问的是哪个重分发器。对于分配器，mpidr 被忽略

    mpidr 编码基于架构定义MPIDR 中的亲和性（affinity）信息，字段编码如下::

      | 63 .... 56 | 55 .... 48 | 47 .... 40 | 39 .... 32 |
      |    Aff3    |    Aff2    |    Aff1    |    Aff0    |

    注意，分配器字段不是CPU 分组的（banked），无论使用哪个 mpidr 访问寄存器都返回相同的值

    VGIC 初始化之前，用户空间允许写入以下寄存器字段：

      * GICD_IIDR.Revision
      * GICD_TYPER2.nASSGIcap

    GICD_IIDR.Revision KVM 实现以客机或用户空间可直接观察的方式发生改变时被更新。用户空
    应当KVM 读取 GICD_IIDR 并将读回的值写回，以确认其预期行为KVM 实现一致。用户空间应
    设置任何其他寄存器之前设GICD_IIDR，以确保预期的行为

    GICD_TYPER2.nASSGIcap 允许用户空间控制对无活跃状态（active state）的 SGI 的支持。在 VGIC
    创建时，该字段重置为系统的最大能力。用户空间应读取该字段以确定支持的值，然后再写入该字段

    GICD_STATUSR GICR_STATUSR 寄存器在架构上定义为：写入清零位无效，而写入置位位则清除该值
    为了让用户空间能够自由设置这两个寄存器的值，使用这两个寄存器的寄存器偏移来设置属性时，只
    将非保留位设置为所写入的值

    GICD_ISPENDR 寄存器区域和 GICR_ISPENDR0 寄存器的访问（读和写）获设置中断的锁存（latched
    挂起状态的值

    这与客机ISPENDR 对边沿触发中断的读取所返回的值相同，但对于电平触发中断可能不同。对于边
    触发中断，一旦一个中断变为挂起（无论是由于输入线上检测到边沿，还是由于客机写ISPENDR），
    该状态就被“锁存”，并且仅在该中断被激活或客机写入 ICPENDR 时才被清除。电平触发中断可能因设备
    将电平输入保持为高而处于挂起状态，也可能因客机写入 ISPENDR 寄存器而处于挂起状态。只ISPENDR
    写入会被锁存；如果设备将线电平拉低，则中断不再挂起，除非客机也写入了 ISPENDR；反之，写入 ICPENDR
    或中断的激活并不会清除挂起状态，如果线电平仍被保持为高。（这些规则记录GICv3 规范ICPENDR
    ISPENDR 寄存器的描述中。）对于电平触发中断，此处访问的值是锁存器的值，ISPENDR 置位
    ICPENDR 或中断激活清除；而客机从 ISPENDR 读取返回的值是锁存器值与输入线电平的逻辑或

    向用户空间提供对锁存状态的原始访问，以便其能够保存和恢复整GIC 内部状态（该状态由当前输入
    线电平与锁存状态的组合定义，且无法仅从线电平和 ISPENDR 寄存器的值推导出来）

    GICD_ICPENDR 寄存器区域和 GICR_ICPENDR0 寄存器的访问具有 RAZ/WI 语义，即读取始终返回 0
    写入始终被忽略

  错误

    ======  =====================================================
    -ENXIO  获取或设置该寄存器尚不支
    -EBUSY  一个或多个 VCPU 正在运行
    ======  =====================================================


  KVM_DEV_ARM_VGIC_GRP_CPU_SYSREGS
   属性：

    kvm_device_attr attr 字段编码两个:

      bits:     | 63      ....       32 | 31  ....  16 | 15  ....  0 |
      values:   |         mpidr         |      RES     |    instr    |

    mpidr 字段基于架构定义MPIDR 中的亲和性信息编CPU ID，字段编码如:

      | 63 .... 56 | 55 .... 48 | 47 .... 40 | 39 .... 32 |
      |    Aff3    |    Aff2    |    Aff1    |    Aff0    |

    instr 字段基于 A64 指令集对系统寄存器访问的编码（RES 表示该位为未来保留，应为零）编码要访问的
    绯荤粺瀵勫瓨鍣?:

      | 15 ... 14 | 13 ... 11 | 10 ... 7 | 6 ... 3 | 2 ... 0 |
      |   Op 0    |    Op1    |    CRn   |   CRm   |   Op2   |

    通过API 访问的所有系统寄存器都是 (rw, 64-bit)，且 kvm_device_attr.addr 指向一__u64 值

    KVM_DEV_ARM_VGIC_GRP_CPU_SYSREGS 访问mpidr 字段指定CPU CPU 接口寄存器

    可用的寄存器有：

    ===============  ====================================================
    ICC_PMR_EL1
    ICC_BPR0_EL1
    ICC_AP0R0_EL1   当主机实现至6 位优先级
    ICC_AP0R1_EL1   当主机实7 位优先级
    ICC_AP0R2_EL1   当主机实7 位优先级
    ICC_AP1R0_EL1
    ICC_AP1R1_EL1   当主机实现至6 位优先级
    ICC_AP1R2_EL1   当主机实7 位优先级
    ICC_AP1R3_EL1   当主机实7 位优先级
    ICC_BPR1_EL1
    ICC_CTLR_EL1
    ICC_SRE_EL1
    ICC_IGRPEN0_EL1
    ICC_IGRPEN1_EL1
    ===============  ====================================================

    当客机可EL2 时，以下寄存器也可用

    =============  ====================================================
    ICH_AP0R0_EL2
    ICH_AP0R1_EL2 当主机实现至6 位优先级
    ICH_AP0R2_EL2 当主机实7 位优先级
    ICH_AP0R3_EL2 当主机实7 位优先级
    ICH_AP1R0_EL2
    ICH_AP1R1_EL2 当主机实现至6 位优先级
    ICH_AP1R2_EL2 当主机实7 位优先级
    ICH_AP1R3_EL2 当主机实7 位优先级
    ICH_HCR_EL2
    ICC_SRE_EL2
    ICH_VTR_EL2
    ICH_VMCR_EL2
    ICH_LR0_EL2
    ICH_LR1_EL2
    ICH_LR2_EL2
    ICH_LR3_EL2
    ICH_LR4_EL2
    ICH_LR5_EL2
    ICH_LR6_EL2
    ICH_LR7_EL2
    ICH_LR8_EL2
    ICH_LR9_EL2
    ICH_LR10_EL2
    ICH_LR11_EL2
    ICH_LR12_EL2
    ICH_LR13_EL2
    ICH_LR14_EL2
    ICH_LR15_EL2
    =============  ====================================================

    CPU 接口寄存器仅使用 AArch64 编码描述

  错误

    =======  =================================================
    -ENXIO   获取或设置该寄存器不受支
    -EBUSY   VCPU 正在运行
    -EINVAL  提供mpidr 或寄存器值无
    =======  =================================================


  KVM_DEV_ARM_VGIC_GRP_NR_IRQS
   属性：

    一个描述此 GIC 实例中断（SGI、PPI SPI）数量的值，范围64 1024，以 32 为增量

    kvm_device_attr.addr 指向一__u32 值

  错误

    =======  ======================================
    -EINVAL  设置的值超出预期范
    -EBUSY   值已经设置
    =======  ======================================


  KVM_DEV_ARM_VGIC_GRP_CTRL
   属性：

    KVM_DEV_ARM_VGIC_CTRL_INIT
      请求初始VGIC，kvm_device_attr.addr 中无额外参数。必须在所VCPU 创建之后调用
    KVM_DEV_ARM_VGIC_SAVE_PENDING_TABLES
      将所LPI 挂起位保存进客机 RAM 的挂起表中

      该操作不会改变挂起表的前 1 kB

  错误

    =======  ========================================================
    -ENXIO   VGIC 在调用此属性前未按要求正确配置
             该属
    -ENODEV  鏃犲湪绾?VCPU
    -ENOMEM  分配 vgic 内部数据时内存不
    -EFAULT  无效的客RAM 访问
    -EBUSY   一个或多个 VCPU 正在运行
    =======  ========================================================


  KVM_DEV_ARM_VGIC_GRP_LEVEL_INFO
   属性：

    kvm_device_attr attr 字段编码以下:

      bits:     | 63      ....       32 | 31   ....    10 | 9  ....  0 |
      values:   |         mpidr         |      info       |   vINTID   |

    vINTID 指定报告的是哪一IRQ

    info 字段指定用户空间想要通过此接口获取或设置的信息。当前我们支持以info 值：

      VGIC_LEVEL_INFO_LINE_LEVEL:
	获取/设置一组连续编号的 32 个中断的 IRQ 线输入电平

	vINTID 必须32 的倍数

	kvm_device_attr.addr 指向一__u32 值，其中包含一个位图，置位的位表示该中断电平被断言

	Bit[n] 表示中断 vINTID + n 的状态

    SGI 以及任何 ID 高于所支持中断数量IRQ 将是 RAZ/WI。LPI 始终是边沿触发的，因此不被此
    接口支持

    PPI mpidr 字段指定的那样VCPU 报告，SPI 不论指定mpidr 如何都报告相同的值

    mpidr 字段基于架构定义MPIDR 中的亲和性信息编CPU ID，字段编码如:

      | 63 .... 56 | 55 .... 48 | 47 .... 40 | 39 .... 32 |
      |    Aff3    |    Aff2    |    Aff1    |    Aff0    |

  错误
    =======  =============================================
    -EINVAL  vINTID 不是 32 的倍数，或 info 字段
	     不是 VGIC_LEVEL_INFO_LINE_LEVEL
    =======  =============================================

  KVM_DEV_ARM_VGIC_GRP_MAINT_IRQ
   属性：

    kvm_device_attr attr 字段编码以下值：

```
      bits:     | 31   ....    5 | 4  ....  0 |
      values:   |      RES0      |   vINTID   |

    vINTID 指定vGIC 必须生成维护中断时生成的哪个中断。这必须是一PPI

```
