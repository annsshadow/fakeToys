### 基于 EDT ft5x06 Polytouch 设备


edt-ft5x06 驱动适用EDT “Polytouch系列电容式触摸屏。注意它***适用基于 focaltech ft5x06 的其他设备，因为它们包含厂商特定的固件。具体而言，该
驱动不适用Nook 平板
它已在以下设备上测试  - EP0350M06
  - EP0430M06
  - EP0570M06
  - EP0700M06

该驱动允许通过一sysfs 文件配置触摸屏：

/sys/class/input/eventX/device/device/threshold    允许设置 “click”（点击）阈值，范围0 80
/sys/class/input/eventX/device/device/gain    允许设置灵敏度，范围0 31。注意值越小表示灵敏度越高
/sys/class/input/eventX/device/device/offset    允许设置边缘补偿，范围为 0 31
/sys/class/input/eventX/device/device/report_rate    允许设置报告速率，范围为 3 14

出于调试目的，该驱动在调试文件系统（若内核中可用）中提供少量文件。它们位于：

    /sys/kernel/debug/i2c/<i2c-bus>/<i2c-device>/

如果你不知道总线和设备号，可以用以下命令查找
    $ ls -l /sys/bus/i2c/drivers/edt_ft5x06

symlink 的解引用将包含所需的信息。你需要其路径的最后两个元素：

    0-0038 -> ../../../../devices/platform/soc/fcfee800.i2c/i2c-0/0-0038

因此在此例中，调试文件的位置为：

    /sys/kernel/debug/i2c/i2c-0/0-0038/

在那里，你将找到以下文件
num_x, num_y    （只读）包含 X Y 方向的传感器字段数量
mode    通过向其写入 “factory mode”（工厂模式）与 “operation
    mode”（工作模式）之间切换传感器。在工厂模式）下可以获取传感器的原始
    数据。注意在工厂模式下不会传递常规事件，且上述选项不可用
raw_data    包含 num_x * num_y 个大16 位值，描述每个传感器字段的原始值。注意对
    本文件的每次 read() 调用都会触发一次新的读取。建议提供一个足够大的缓冲区
    以容num_x ** num_y ** 2 字节
注意当设备不处于工厂模式时，读取 raw_data 会给I/O 错误。当设备不处于常工作模式时，对参数文件进行读/写也会发生同样的情况