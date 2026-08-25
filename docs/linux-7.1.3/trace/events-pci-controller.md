
## 子系统跟踪点：PCI 控制


## 概述

PCI控制器跟踪系统提供跟踪点来监控控制器
用于调试目的的级别信息。事件通常显示在这里：

/sys/内核/跟踪/事件/pci_controller

比照include/trace/events/pci_controller.h 用于事件定义

## 可用的跟踪点


### PCIe_ltssm_state_transition


监控 PCIe LTSSM 状态转换，包括状态和速率信息
```

    pcie_ltssm_state_transition  "dev: %s state: %s rate: %s\n"

```
**参数**

- `dev` - PCIe 鎺у埗鍣ㄥ疄渚。
- `state` - PCIe LTSSM 状
- `rate` - PCIe 数据速率

**用法示例**


# 启用跟踪
echo 1 > /sys/kernel/debug/tracing/events/pci_controller/pcie_ltssm_state_transition/enable

# 监控事件（设备链接时会生成以下输出）
系统/内核/调试/跟踪/trace_pipe
kworker/0:0-9 [^000^] ..... 5.600221: pcie_ltssm_state_transition: dev: a40000000.pcie 状 RCVRY_EQ2 速率: 8.0 GT/s
