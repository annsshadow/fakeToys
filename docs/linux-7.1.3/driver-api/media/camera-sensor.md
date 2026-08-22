

## 编写摄像头传感器驱动

本文档仅涵盖内核API。关于摄像头传感器驱动中用户空间 API 实现的最佳实践，参见 media_using_camera_sensor_drivers
### CSI-2、并行和 BT.656 总线

请参transmitter-receiver
### 处理时钟

摄像头传感器内部有一个时钟树，包括一PLL 和若干个分频器。时钟树通常由驱动根一些特定于硬件的输入参数进行配置：外部时钟频率和链路频率。这两个参数一般从系统
固件中获取*在任何情况下都不得使用其他频率*

时钟频率如此重要的原因是，时钟信号来SoC，并且在许多情况下，系统中设计使用的就是
某个特定频率。使用其他频率可能会在其他地方造成有害影响。因此，用户只能配置预先
确定的频率
外部时钟频率应通过 `devm_v4l2_sensor_clk_get()` 辅助函数获取外部时钟，然后用
`clk_get_rate()` 取得其频率来获得。无论传感器集成在基DT 还是基于 ACPI 的系中，使用该辅助函数都能保证正确的行为
#### ACPI

基于 ACPI 的系统通常不会向内核注册传感器外部时钟，而是`clock-frequency` _DSD
属性中指定外部时钟频率。`devm_v4l2_sensor_clk_get()` 辅助函数会创建并返回一个设为该频率的固定时钟
#### Devicetree

基于 Devicetree 的系统在设备树中声明传感器外部时钟，并从传感器节点引用它。选择
外部时钟频率的首选方式，是在传感器节点中使用 `assigned-clocks`、`assigned-clock-parents`
`assigned-clock-rates` 属性来设置时钟频率。更多信息请参见 `clock device tree bindings
<https://github.com/devicetree-org/dt-schema/blob/main/dtschema/schemas/clock/clock.yaml>`_`devm_v4l2_sensor_clk_get()` 辅助函数会获取并返回该时钟
这种方式的缺点是，无法保证该频率没有被另一个驱动直接或间接修改过，或者一开就得到板级时钟树的支持。要确保可靠性，需要更Common Clock Framework API
### 电源管理

摄像头传感器与其他设备配合使用，构成一条摄像头流水线。它们必须遵守此处列出的规则以确保整条流水线上一致的电源管理
摄像头传感器驱动还负责控制它们所控制的设备的电源状态。它们应当使用运行时 PM 管理电源状态。运行时 PM 应在 probe 时启用，remove 时禁用。驱动应当启用运行时 PM
自动挂起。另请参async sub-device registration <media-registering-async-subdevs>
运行PM 处理函数应当处理为传感器上电和下电所需的时钟、调节器、GPIO 以及其他
系统资源。对于那些不使用任何这些资源的驱动（例如仅支ACPI 系统的驱动），运行时
PM 处理函数可以留空不实现
一般来说，设备至少应在访问其寄存器时以及在进行流式传输时处于上电状态。驱动在
开始流式传输时应使`pm_runtime_resume_and_get()`，在停止流式传输时应使用
`pm_runtime_put()` `pm_runtime_put_autosuspend()`。它们可以在 probe 时给设备上电
（例如读取识别寄存器），但不应在 probe 之后无条件地保持供电
在系统挂起时，整条摄像头流水线必须停止流式传输，并在系统恢复时重新启动。这需摄像头传感器与摄像头流水线其余部分之间的协调。桥接驱动负责这种协调，并通过调用
适当的子设备操作（`.enable_streams()` `.disable_streams()`）来指示摄像头传感器
停止和重新启动流式传输。因此，摄像头传感器驱动**不应**PM 挂起处理函数中跟流式传输状态以停止流式传输、在恢复处理函数中重新启动它。驱动一般不应实现系PM
处理函数
摄像头传感器驱动**不应**实现子设备的 `.s_power()` 操作，因为它已被废弃。虽然该
操作在一些已有驱动中仍有实现（因为它们早于废弃时间），但新驱动应当改用运行时 PM如果你觉得自己需要从 ISP 或桥接驱动开始调`.s_power()`，则应改为向你正在使用的
传感器驱动添加运行时 PM 支持，并去掉它的 `.s_power()` 处理函数
另请参见 examples <media-camera-sensor-examples>
#### 控制框架

`v4l2_ctrl_handler_setup()` 函数不能在设备的运行PM `runtime_resume` 回调中使用，
因为它无法判断设备的电源状态。这是因为设备的电源状态只有在电源状态转换发生之才会改变。`s_ctrl` 回调可用于在电源状态转换之后获取设备的电源状态：

该函数在成功获取电源计数或运行时 PM 被禁用时返回一个非零值，在这两种情况下驱都可以继续访问设备
### 旋转、方向和翻转

使用 `v4l2_fwnode_device_parse()` 从系统固件获取旋转和方向信息，并使用
`v4l2_ctrl_new_fwnode_properties()` 注册相应的控制项
### 示例驱动

传感器驱动实现的功能各不相同，并且根据所支持功能集合以及其他特性，特定的传感器
驱动更适合作为示例。以下驱动被认为是良好的示例
    :header-rows: 0
    :widths:      1 1 1 2

    - - Driver name
      - File(s)
      - Driver type
      - Example topic
    - - CCS
      - `drivers/media/i2c/ccs/`
      - 可自由配      - 电源管理（ACPI DT）、UAPI
    - - imx219
      - `drivers/media/i2c/imx219.c`
      - 基于寄存器列      - 电源管理（DT）、UAPI、模式选择
    - - imx319
      - `drivers/media/i2c/imx319.c`
      - 基于寄存器列      - 电源管理（ACPI DT