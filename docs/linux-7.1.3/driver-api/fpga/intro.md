## 简

FPGA 子系统支持在 Linux 下动态重编程 FPGA。FPGA 子系统的一些核心意图是
- FPGA 子系统与厂商无关（vendor agnostic）
- FPGA 子系统将上层（用户空间接口和枚举）与知道如何对特FPGA 进行编程的下层分离开来
- 代码不应在上层和下层之间共享。这一点本应不言自明。如果这看起来有必要，那很可能存在可以添加到框架中以惠及其他用户的功能。请写信linux-fpga 邮件列表和维护者，寻求一种能够扩展框架以供广泛复用的解决方案
- 一般来说，在添加代码时要着眼未来。为复用做计划
内核中的框架分为
### FPGA Manager


如果你要添加一个新FPGA 或一种新FPGA 编程方法，这个子系统正适合你。低FPGA Manager 驱动包含如何对特定设备进行编程的知识。该子系统包fpga-mgr.c 中的框架以及注册到它的低层驱动
### FPGA Bridge


FPGA 桥（Bridge）用于防止在编程期间有杂散信号从 FPGA FPGA 的某个区域发出。它们在编程开始前被禁用，并在之后重新启用。一FPGA 桥可以是实际硬件，用于门控通往 CPU 的总线，也可以是围FPGA 部分重配置区域的 FPGA  fabric 中的软（"freeze"）桥。该子系统包fpga-bridge.c 以及注册到它的低层驱动
### FPGA Region


如果你要添加一个新FPGA 框架接口，请FPGA 区域之上添加它
FPGA Region 框架（fpga-region.c）将 Manager Bridge 关联为可重配置区域。一个区域可以指完整FPGA（完整重配置），也可以指一个部分重配置区域
设备FPGA Region 支持（of-fpga-region.c）在应用设备树叠加层（overlay）时处理 FPGA 的重编程