## Synopsys DesignWare Cores (DWC) PCIe 性能监控单元（PMU）


## DesignWare Cores (DWC) PCIe PMU


该 PMU 是由每个 PCIe Root Port 在名为 RAS D.E.S（Debug、Error injection、Statistics，调试、错误注入与统计）的厂商特定扩展能力（Vendor-Specific Extended Capability）中提供的 PCIe 配置空间寄存器块。

顾名思义，RAS DES 能力支持系统级调试、AER 错误注入以及统计信息的收集。为便于统计信息的收集，Synopsys DesignWare Cores PCIe 控制器提供以下两个特性：

- 一个用于基于时间分析（RX/TX 数据吞吐量与在各低功耗 LTSSM 状态上花费的时间）的 64 位计数器，以及
- 每个事件一个 32 位计数器，用于事件计数（指定通道的错误与非错误事件）

注意：计数器溢出没有中断。

### 基于时间的分析


使用该特性，你可以获得有关控制器 RX/TX 数据吞吐量与在各低功耗 LTSSM 状态上花费时间的信息。PMU 将数据的测量分为两类：

- 组#0：控制器停留在 LTSSM 状态的时间百分比。
- 组#1：处理的数据量（以 16 字节为单位）。

### 通道事件计数器


使用该特性，你可以获得控制器在特定通道上的错误与非错误信息。PMU 事件由以下全部选择：

- 组 i
- 组 i 中的事件 j
- 通道 k

某些事件仅存在于特定配置中。

## DesignWare Cores (DWC) PCIe PMU 驱动


该驱动为每个 PCIe Root Port 添加 PMU 设备，名称基于该 Root Port 的 SBDF。例如，

    0001:30:03.0 PCI bridge: Device 1ded:8000 (rev 01)

该 Root Port 的 PMU 设备名称为 dwc_rootport_13018。

DWC PCIe PMU 驱动注册一个 perf PMU 驱动，它在 sysfs 中提供可用事件与配置选项的描述，见 /sys/bus/event_source/devices/dwc_rootport_{sbdf}。

"format" 目录描述 perf_event_attr 结构体的 config 字段格式。"events" 目录为所有已文档化的事件提供配置模板。例如，"rx_pcie_tlp_data_payload" 等价于 "eventid=0x21,type=0x0"。

```

    $# perf list | grep dwc_rootport
    <...>
    dwc_rootport_13018/Rx_PCIe_TLP_Data_Payload/        [Kernel PMU event]
    <...>
    dwc_rootport_13018/rx_memory_read,lane=?/               [Kernel PMU event]

```

### 基于时间的分析事件用法


```

    $# perf stat -a -e dwc_rootport_13018/Rx_PCIe_TLP_Data_Payload/

```

平均 RX/TX 带宽可使用以下公式计算：

    PCIe RX Bandwidth = rx_pcie_tlp_data_payload / Measure_Time_Window
    PCIe TX Bandwidth = tx_pcie_tlp_data_payload / Measure_Time_Window

### 通道事件用法


每个通道具有相同的事件集合，为避免生成数百个条目的列表
```

    $# perf stat -a -e dwc_rootport_13018/rx_memory_read,lane=4/

```

该驱动不支持采样，因此 "perf record" 无法工作。不支持按任务（不带 "-a"）的 perf 会话。
