
## 子系统追踪点：PCI


## 概述

PCI 追踪系统提供追踪点，用于监控可能影响系统性能与可靠性的关键硬件事件。这些事件通常出现在以下位置：

	/sys/kernel/tracing/events/pci

相关事件定义include/trace/events/pci.h
## 可用的追踪点


### pci_hp_event


监控 PCI 热插拔事件，包括卡的插入/移除以及链路状态变化```

    pci_hp_event  "%s slot:%s, event:%s\n"

```
**事件类型**
- `LINK_UP` - PCIe 链路已建- `LINK_DOWN` - PCIe 链路丢失
- `CARD_PRESENT` - 插槽中检测到- `CARD_NOT_PRESENT` - 卡已从插槽移
```

    # 启用追踪    echo 1 > /sys/kernel/debug/tracing/events/pci/pci_hp_event/enable

    # 监控事件（以下输出在设备热插拔时产生    cat /sys/kernel/debug/tracing/trace_pipe
       irq/51-pciehp-88      [001] .....  1311.177459: pci_hp_event: 0000:00:02.0 slot:10, event:CARD_PRESENT

       irq/51-pciehp-88      [001] .....  1311.177566: pci_hp_event: 0000:00:02.0 slot:10, event:LINK_UP

```
### pcie_link_event


监控 PCIe 链路速率变化，并提供详细的链路状态信息```

    pcie_link_event  "%s type:%d, reason:%d, cur_bus_speed:%d, max_bus_speed:%d, width:%u, flit_mode:%u, status:%s\n"

```
**参数**
- `type` - PCIe 设备类型=Root Port，等- `reason` - 链路变化的原因：

  - `0` - 链路重训  - `1` - 总线枚举
  - `2` - 带宽通知使能
  - `3` - 带宽通知 IRQ
  - `4` - 热插拔事

```

    # 启用追踪    echo 1 > /sys/kernel/debug/tracing/events/pci/pcie_link_event/enable

    # 监控事件（以下输出在设备热插拔时产生    cat /sys/kernel/debug/tracing/trace_pipe
       irq/51-pciehp-88      [001] .....   381.545386: pcie_link_event: 0000:00:02.0 type:4, reason:4, cur_bus_speed:20, max_bus_speed:23, width:1, flit_mode:0, status:DLLLA

```
