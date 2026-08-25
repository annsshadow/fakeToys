## Linux 鐩叉枃鎺у埗鍙。

要在盲文设备上获取早期启动消息（在用户空间屏幕阅读器启动之前），你首先需编译对常规串行控制台的支持（Documentation/admin-guide/serial-console.rst
<serial_console>），以及对盲文设备的支持（在 `Device Drivers --> Accessibility
support --> Console on braille device` 中）
然后你需要指定一`console=brl` 选项在内核命令行上，

```
	console=brl,serial_options...
```
其中 `serial_options...` Documentation/admin-guide/serial-console.rst
<serial_console> 中描述的相同
例如，如果盲文设备连接到第一个串口，你可以使`console=brl,ttyS0`；使`console=brl,ttyS0,115200` 可将波特率覆盖为 115200，等等
默认情况下，盲文设备仅显示最后一条内核消息（控制台模式）。要查看先前的消息，
Insert 键切换到 VT 审查模式。在审查模式下，方向键允许浏VT 内容`PAGE-UP`/`PAGE-DOWN` 键跳到屏幕顶底部，`HOME` 键回到光标处，从而提非常基本的屏幕审查功能
可以通过添加 `braille_console.sound=1` 内核参数来获得声音反馈
为简单起见，只能启用一个盲文控制台，其`console=brl,...` 的使用将被丢弃还要注意，它不会干扰 Documentation/admin-guide/serial-console.rst
<serial_console> 中描述的控制台选择机制
目前仅支VisioBraille 设备
Samuel Thibault <samuel.thibault@ens-lyon.org>
