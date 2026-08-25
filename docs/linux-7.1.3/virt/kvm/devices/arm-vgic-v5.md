
## ARM 虚拟通用中断控制v5（VGICv5

支持的设备类型：
  - KVM_DEV_TYPE_ARM_VGIC_V5     ARM 通用中断控制v5.0

通过API 只能实例化一VGIC 实例。所创建VGIC 将充VM 的中断控制器，要求被模拟的用户空间设备向 VGIC 注入中断，而不是直接向 CPU 注入
创建一个客户机 GICv5 设备需要一个宿GICv5 主机。当前的 VGICv5 设备仅支PPI 中断。这些中断既可以从被模拟的内核内设备（如 Arch Timer PMU）注入，也可以通过 KVM_IRQ_LINE ioctl 注入
组：
  KVM_DEV_ARM_VGIC_GRP_CTRL
   属性：

    KVM_DEV_ARM_VGIC_CTRL_INIT
     请求初始VGIC，kvm_device_attr.addr 中没有额外参数。必须在所VCPU 创建之后调用
   KVM_DEV_ARM_VGIC_USERPSPACE_PPIs
     请求用户空间可驱动的 PPI 掩码。在 GICv5 中，只有一部分 PPI 可以直接由用户空间驱动，返回的掩码告知用户空间哪PPI 允许通过 KVM_IRQ_LINE 驱动
     用户空间必须分配并指kvm_device_attr.addr 处的 __u64[^2^] 数据。当此调用返回时，提供的内存将被填充为用户空PPI 掩码。较低的 __u64 包含较低 64 PPI 的掩码，其余 64 个位于第二个 __u64 中
     这是一个只读属性，不能被设置。尝试设置它会被拒绝
  错误
    =======  ========================================================
    -ENXIO   VGIC 在调用此属性之前未按要求正确配    -ENODEV  没有在线VCPU
    -ENOMEM  分配 vgic 内部数据时内存不    -EFAULT  无效的客户机 ram 访问
    -EBUSY   一个或多个 VCPU 正在运行
    =======  ========================================================
