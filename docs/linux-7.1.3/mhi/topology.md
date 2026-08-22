
## MHI 拓扑


本文档介绍内核中 MHI 拓扑的建模与表示方式
### MHI 鎺у埗鍣。

MHI 控制器驱动管理着MHI 客户端设备（如外部调制解调器WiFi 芯片组）的交互。它同时也是 MHI 总线主设备，负责管理与主机和设备之间的物理链路。不过它不参与实际的数据传输，因为数据传输由 PCIe 等物理总线负责。每个控制器驱动会根据客户端设备类型暴露相应的通道和事件
MHI 控制器驱动的职责如下
- 开启物理总线并建立与设备的链- 配置 IRQ、IOMMU IOMEM
- 分配 struct mhi_controller 并通过 mhi_register_controller MHI 总线框架注册，同时提供通道和事件配- 发起上电和掉电序- 发起设备的挂起和恢复电源管理操作

### MHI 设备


MHI 设备是逻辑设备，最多绑定两MHI 通道以实现双向通信。一MHI 进入上电状态，MHI 核心会根据控制器暴露的通道配置创建 MHI 设备。每个通道或每对通道可以对应一MHI 设备
```

        /sys/bus/mhi/devices/

```
### MHI 驱动


MHI 驱动是绑定到一个或多个 MHI 设备的客户端驱动。MHI 驱动通过 MHI 发送和接收上层协议报文，如 IP 包、调制解调器控制消息和诊断消息。MHI 核心会将 MHI 设备绑定MHI 驱动
```

        /sys/bus/mhi/drivers/

```
MHI 驱动的职责如下：

- 通过 mhi_driver_register MHI 总线框架注册驱动
- 调用 mhi_prepare_for_transfer 为传输准备设- 调用 mhi_queue_transfer 发起数据传输
- 数据传输完成后，调用 mhi_unprepare_from_transfer 结束数据传输
