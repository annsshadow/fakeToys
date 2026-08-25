## Asus 笔记本扩展功

Version 0.1

August 6, 2009

Corentin Chary <corentincj@iksaif.net>
http://acpi4asus.sf.net/

 该驱动为兼容 ACPI 的华硕（ASUS）笔记本提供额外功能的支持。它也可能支持部MEDION、JVC  VICTOR 笔记本（例如 MEDION 9675 VICTOR XP7210）。它使所有额外的按键都能生成输入事件
 （类似于键盘）
 在部分型号上，它还支持调整显示亮度与输出、开LCD 背光灯，而最重要的是，允许你让那些用 指示邮件和无线状态的炫酷 LED 闪烁
该驱动取代了旧的 asus_acpi 驱动
### 要求


  为你的计算机配置好的内核 2.6.X 源代码，并启ACPI 支持。你还需要配CONFIG_INPUT   CONFIG_ACPI
### 状

  当前支持的功能如下（详细说明见下文）
 - Fn 组合 - 蓝牙的开启与关闭
 - WLAN 的开启与关闭
 - GPS 的开启与关闭
 - 视频输出切换
 - 环境光传感器开 - LED 控制
 - LED 显示屏控 - LCD 亮度控制
 - LCD 开
  按型号和功能划分的兼容性对照表维护在网http://acpi4asus.sf.net/ 上
### 使用方法


  尝试执行 "modprobe asus-laptop"。查看你dmesg（直接输dmesg 即可）。你应该会看  类似如下的几行：

      Asus Laptop Extras version 0.42
        - L2D model detected.

  如果这并非你笔记本上的输出，请把它（以及笔记本的 DSDT）发给我
  就这样，现在你笔记本上所有由热键生成的事件都会通过 netlink 事件上报。你可以  "acpi_genl monitor"（acpica 项目的一部分）来检查
  热键也会作为按键（类似于键盘）上报，你可以在 X11 下使"xev" 来查看支持哪些按键
  你可以通过读取 /sys/devices/platform/asus-laptop/infos 条目来获取你DSDT 表版本信息  如果你有问题或要提交 bug 报告，请附上该条目的输出内容
### LED


```
    echo 1 >  /sys/class/leds/asus::mail/brightness

  will switch the mail LED on.

  You can also know if they are on/off by reading their content and use
  kernel triggers like disk-activity or heartbeat.

```
### 鑳屽厜鐏。

  你可以通过 /sys/class/backlight/asus-laptop/ 控制 LCD 背光灯的电源与亮度。亮度取值介0 15 之间
### 无线设备


  你可以通过 bluetooth 条目开关闭内置蓝牙适配器（仅限带蓝牙的型号）。这通常也会控制对应  LED。WLAN 适配器同理
### 显示切换


  注意：显示切换代码目前被视为实验性（EXPERIMENTAL）特性
  以下型号支持切换
    - L3800C
    - A2500H
    - L5800C
    - M5200N
    - W1000N（虽然会有一些瑕疵）
    - M6700R
    - A6JC
    - F3J

  以下型号不支持切换：

    - M3700N
    - L2X00D（在某些条件下会锁定笔记本）

  要切换显示，请向 /sys/devices/platform/asus-laptop/display 写入 0 15 之间的值。这些值的
  含义如下
  +-------+-----+-----+-----+-----+-----+
  | Bin   | Val | DVI | TV  | CRT | LCD |
  +-------+-----+-----+-----+-----+-----+
  | 0000  |   0 |     |     |     |     |
  +-------+-----+-----+-----+-----+-----+
  | 0001  |   1 |     |     |     |  X  |
  +-------+-----+-----+-----+-----+-----+
  | 0010  |   2 |     |     |  X  |     |
  +-------+-----+-----+-----+-----+-----+
  | 0011  |   3 |     |     |  X  |  X  |
  +-------+-----+-----+-----+-----+-----+
  | 0100  |   4 |     |  X  |     |     |
  +-------+-----+-----+-----+-----+-----+
  | 0101  |   5 |     |  X  |     | X   |
  +-------+-----+-----+-----+-----+-----+
  | 0110  |   6 |     |  X  |  X  |     |
  +-------+-----+-----+-----+-----+-----+
  | 0111  |   7 |     |  X  |  X  |  X  |
  +-------+-----+-----+-----+-----+-----+
  | 1000  |   8 |  X  |     |     |     |
  +-------+-----+-----+-----+-----+-----+
  | 1001  |   9 |  X  |     |     |  X  |
  +-------+-----+-----+-----+-----+-----+
  | 1010  |  10 |  X  |     |  X  |     |
  +-------+-----+-----+-----+-----+-----+
  | 1011  |  11 |  X  |     |  X  |  X  |
  +-------+-----+-----+-----+-----+-----+
  | 1100  |  12 |  X  |  X  |     |     |
  +-------+-----+-----+-----+-----+-----+
  | 1101  |  13 |  X  |  X  |     |  X  |
  +-------+-----+-----+-----+-----+-----+
  | 1110  |  14 |  X  |  X  |  X  |     |
  +-------+-----+-----+-----+-----+-----+
  | 1111  |  15 |  X  |  X  |  X  |  X  |
  +-------+-----+-----+-----+-----+-----+

  在大多数情况下，上述组合生效前必须先接好相应的显示器。TV-Out 可能需要在启动时初始化
  调试
  1) 检Fn+F8 键是否：

     a) 不会锁定笔记本（若会锁定，尝试以 noapic / nolapic 启动     b) 会生成事件（0x6n，其n 是对应于上述配置的值）
     c) 实际生效

     记录每种配置下的 disp 值  2) /sys/devices/platform/asus-laptop/display 写入 0 15 的值。记录其值，注意任何变化     若无变化，尝试更宽的范围，最大到 65535  3) 将任何输出（正面和负面报告都需要，除非你的机器已在上述列表中）发送至 acpi4asus-user
     邮件列表
  注意：在某些机器（如 L3C）上，模块加载后只会生成 0x6n 事件，并不会真正发生切换。这种情况下  一```
    echo $((10#$arg-60)) > /sys/devices/platform/asus-laptop/display

```
  通常就能解决问题arg 是传递给 acpid 的类0000006n 的事件）
  注意：目前在 xxN（Centrino）型号上还没有可靠的方法读取显示状态
### LED 显示

  部分型号（如 W1N）带有可用于显示若干信息LED 显示屏
  LED 显示屏在以下型号上可用：

    - W1000N
    - W1J

```
    echo 0x0T000DDD > /sys/devices/platform/asus-laptop/

```
  其中 T 控制 3 个字母的显示，DDD 控制 3 个数字的显示，如下表所示：

         DDD（数字）
         000 999 = 显示数字
         AAA        = ---
         BBB FFF = 关闭

         T（类型）
         0 = 关闭
         1 = dvd
         2 = vcd
         3 = mp3
         4 = cd
         5 = tv
         6 = cpu
         7 = vol

  例如 "echo 0x01000001 >/sys/devices/platform/asus-laptop/ledd" 会显"DVD001"
### 驱动选项


  可以使用标准模块参数语法asus-laptop 驱动传递选项（将选项传给模块时使<param>=<value>  asus-laptop 静态链接进内核时，在内核启动命令行上使asus-laptop.<param>=<value>）
	     wapf：WAPF 定义 Fn+Fx 无线网键的行		  这些值的含义尚未完全明确，但大多数情况下
     - 0x0 应不执行任何操作
     - 0x1 应允许用 Fn+Fx 键控制设     - 0x4 应在按下 Fn+Fx 键时发送一ACPI 事件x88     - 0x5 类似0x1 0x4

  默认值为 0x1
### 不支持的型号


  这些型号永远不会被本模块支持，因为它们使用了完全不同的机制来处理 LED 和其他附加功  （也就是说我们不清楚其工作原理）
 - ASUS A1300 (A1B), A1370D
 - ASUS L7300G
 - ASUS L8400

### 补丁、错误、问

  我欢迎任何成功或失败的反馈，尤其是能补充或修正兼容性对照表的反馈。请在报告中包含以下信息
 - ASUS 型号名称
 - 使用 "acpidump" 工具获取ACPI 表副 - /sys/devices/platform/asus-laptop/infos 的副 - 哪些驱动功能可用、哪些不可用
 - 不可用功能的实际表现

  其他任何意见或补丁同样非常欢迎
 acpi4asus-user@lists.sourceforge.net

 http://sourceforge.net/projects/acpi4asus
