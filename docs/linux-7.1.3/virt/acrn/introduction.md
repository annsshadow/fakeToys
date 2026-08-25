
## ACRN Hypervisor 简

ACRN Hypervisor 是一Type 1 型（裸金属）hypervisor，直接运行在裸金属硬件上。它有一个特权管VM，称Service VM（服务虚拟机），用于管理 User VM（用户虚拟机）并进行 I/O 仿真
ACRN 用户空间（userspace）是一个运行在 Service VM 中的应用，它基于命令行配置为用户 VM 仿真设备。ACRN Hypervisor Service Module（HSM）是 Service VM 中的一个内核模块，ACRN 用户空间提供 hypervisor 服务
下图展示了其架构
```

                Service VM                    User VM
      +----------------------------+  |  +------------------+
      |        +--------------+    |  |  |                  |
      |        |ACRN userspace|    |  |  |                  |
      |        +--------------+    |  |  |                  |
      |-----------------ioctl------|  |  |                  |   ...
      |kernel space   +----------+ |  |  |                  |
      |               |   HSM    | |  |  | Drivers          |
      |               +----------+ |  |  |                  |
      +--------------------|-------+  |  +------------------+
  +---------------------hypercall----------------------------------------+
  |                         ACRN Hypervisor                              |
  +----------------------------------------------------------------------+
  |                          Hardware                                    |
  +----------------------------------------------------------------------+

```
ACRN 用户空间为用VM 分配内存、配置并初始化用VM 使用的设备、加载虚拟引导加载器、初始化虚拟 CPU 状态，并处理来自用VM I/O 请求访问。它使用 ioctl HSM 通信。HSM 通过ACRN Hypervisor 的交互（通过 hypercall）实hypervisor 服务。HSM 向用户空间导出一个字符设备接口（/dev/acrn_hsm）
ACRN hypervisor 向任何人开放贡献。源码仓库位https://github.com/projectacrn/acrn-hypervisor