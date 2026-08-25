
## 概述


Surface/System Aggregator Module（SAM、SSAM）是微软 Surface 设备上的嵌入式控制器（EC）（可以说是**最核心*一个）。它最初引入于4 代设备（Surface Pro 4、Surface Book 1），但此后其职责和功能集在后续几代中得到了显著扩展

## 特性与集成


关于4 代设备（Surface Pro 4、Surface Book 1）上SAM，目前所知甚少，因为主机EC 之间使用了不同的通信接口（如下文详述）。在5 代（Surface Pro 2017、Surface Book 2、Surface Laptop 1）及更晚的设备上，SAM 负责向主机提供电池信息（当前状态和最大容量等静态值）、一系列温度传感器（如外壳温度）以及散热/性能模式设置。具体而言，在 Surface Book 2 上，它还提供用于正确处理剪贴板分离（即将显示部分与键盘部分分离）的接口；Surface Laptop 1 2 上，键盘 HID 输入需要它。该 HID 子系统在7 代设备上进行了重构，在这些设备（具体Surface Laptop 3 Surface Book 3）上负责所有主要的 HID 输入（即键盘和触摸板）
虽然自第 5 代以来特性在粗粒度层面变化不大，但内部接口经历了相当大的改变。在5 代和6 代设备上，电池和温度信息都通过一shim 驱动（称Surface ACPI Notify，或 SAN）暴露给 ACPI，将 ACPI 通用串行总线的写/读访问转换为 SAM 请求。在7 代设备上，这一额外层消失了，这些设备需要一个直接挂SAM 接口的驱动。同样，在更新的代际中，ACPI 中声明的设备更少，使它们更难被发现，并迫使我们硬编码某种设备注册表。因此，实现了一个带有客户端设备SSAM 总线和子系统（`struct ssam_device <ssam_device>`）

## 通信


主机EC 之间的通信接口类型取决Surface 设备的代际。在4 代设备上，主机和 EC 通过 HID 通信，具体使HID-over-I2C 设备；而在5 代及更晚的设备上，通信通过 USART 串行设备进行。参照其他操作系统上的驱动，我们将该串行设备及其驱动称为 Surface Serial Hub（SSH）。在需要时，我们通过 SAM-over-SSH SAM-over-HID 来区分这两类 SAM
目前，该子系统仅支持 SAM-over-SSH。SSH 通信接口在下文有更详细的描述。HID 接口尚未被逆向工程，目前尚不清楚下文详述的 SSH 接口的哪些概念可以移植到它上面
### Surface Serial Hub


如前所述，Surface Serial Hub（SSH）是5 代及所有更晚代Surface 设备SAM 的通信接口。在最高层，通信可分为两种主要类型：请求（从主机发送到 EC、可能触EC 直接响应（显式关联于该请求）的消息），以及事件（有时也称为通知，由 EC 发往主机，并非对先前请求的直接响应）。我们也可以将没有响应的请求称为命令。一般而言，事件需要先通过多个专用请求之一启用，然后才会由 EC 发送
有关更技术性的协议文档，请参见 Documentation/driver-api/surface_aggregator/ssh.rst；关于内部驱动架构的概述，请参见 Documentation/driver-api/surface_aggregator/internal.rst