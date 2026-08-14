
## MPIC 中断控制器


支持的设备类型：

  - KVM_DEV_TYPE_FSL_MPIC_20     Freescale MPIC v2.0
  - KVM_DEV_TYPE_FSL_MPIC_42     Freescale MPIC v4.2

任意类型都只能实例化一个 MPIC 实例。所创建的 MPIC 将充当系统中断控制器，连接到每个 vcpu 的中断输入。

组：
  KVM_DEV_MPIC_GRP_MISC
   属性：

    KVM_DEV_MPIC_BASE_ADDR (rw, 64 位)
      256 KiB MPIC 寄存器空间的首地址。必须按自然边界对齐。值为零时禁用映射。
      复位值为零。

  KVM_DEV_MPIC_GRP_REGISTER (rw, 32 位)
    访问一个 MPIC 寄存器，就像该访问是从客户机发起的一样。"attr" 是 MPIC 寄存器空间中的字节偏移。访问必须按 4 字节对齐。

    可以使用该属性组向相关的 MSIIR 写入来发出 MSI 信号。

  KVM_DEV_MPIC_GRP_IRQ_ACTIVE (rw, 32 位)
    每个标准 openpic 源的 IRQ 输入线。0 表示非激活，1 表示激活，与中断的触发方式无关。

    对于边沿触发的中断：写入 1 被视为一次激活边沿，写入 0 将被忽略。读取时，如果此前发出的边沿尚未被确认则返回 1，否则返回 0。

    "attr" 是 IRQ 编号。标准源的 IRQ 编号是相关 IVPR 相对于 EIVPR0 的字节偏移，再除以 32。

IRQ 路由：

  MPIC 模拟支持 IRQ 路由。只能实例化一个 MPIC 设备。该设备一旦被创建，就作为 irqchip id 0 可用。

  该 irqchip 0 具有 256 个中断引脚，暴露出主中断源数组（即"SRC"中断）中的中断。

  其编号方式与 MPIC 设备树绑定一致——基于从源数组起始处开始的寄存器偏移，而不考虑芯片文档中诸如"内部"或"外部"中断之类的任何细分。

  对非 SRC 中断的访问未通过 IRQ 路由机制实现。
