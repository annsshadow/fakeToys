
## XICS 中断控制


支持的设备类型：KVM_DEV_TYPE_XICS

组：
  1. KVM_DEV_XICS_GRP_SOURCES
       属性：

         每个中断源一个，以源编号索引
  2. KVM_DEV_XICS_GRP_CTRL
       属性：

         2.1 KVM_DEV_XICS_NR_SERVERS（只写）

  kvm_device_attr.addr 指向一__u32 值，即中断服务器编号的数量（即最高的可能 vcpu id 加一）

  错误

    =======  ==========================================
    -EINVAL  值大KVM_MAX_VCPU_IDS
    -EFAULT  attr->addr 的用户指针无效
    -EBUSY   已有 vcpu 连接到该设备
    =======  ==========================================

该设备模拟了 PAPR 中定义的 XICS（eXternal Interrupt Controller Specification，外部中断控制器规范）。XICS 具有一组中断源，每个由 20 位源编号标识，以及一组中断控制呈现（ICP）实体，也称为“服务器”，每个关联一个虚CPU

ICP 实体通过为每vcpu 启用 KVM_CAP_IRQ_ARCH 能力来创建，kvm_enable_cap 结构体的 args[^0^] 中指KVM_CAP_IRQ_XICS，在 args[^1^] 中指定中断服务器编号（即XICS 视角vcpu 编号）。每ICP 64 位状态，可以使用 vcpu 上的 KVM_GET_ONE_REG KVM_SET_ONE_REG ioctl 进行读写。这 64 位状态字具有以下位域，从字的最低有效位一端开始：

- 未使用，16 

- 待处理中断优先级 
  零为最高优先级55 表示无待处理中断

- 待处IPI（处理器间中断）优先级，8 
  零为最高优先级55 表示无待处理 IPI

- 待处理中断源编号4 
  零表示无待处理中断，2 表示有待处理 IPI

- 当前处理器优先级 
  零为最高优先级，意味着无法投递任何中断，255 为最低优先级

每个源有 64 位状态，可以使用 KVM_GET_DEVICE_ATTR KVM_SET_DEVICE_ATTR ioctl 读写，指KVM_DEV_XICS_GRP_SOURCES 属性组，属性编号为中断源编号。这 64 位状态字具有以下位域，从字的最低有效位一端开始：

- 目标（服务器编号），32 

  这指定中断应发送到的位置，即为目标 vcpu 指定的中断服务器编号

- 优先级，8 

  这是为该中断源指定的优先级，其中 0 为最高优先级55 为最低。优先级255 的中断永远不会被投递

- 电平敏感标志 

  对于电平敏感的中断源，该位为 1；对于边沿敏感（MSI），则为 0

- 掩码标志 

  如果该位1，则表示中断被屏蔽（无论其优先级如何都无法投递），例如通过 ibm,int-off RTAS 调用；否则为 0

- 待处理标志，1 

  如果该位1，则表示该源有待处理中断，否则为 0

每个 VM 只能创建一XICS 实例
