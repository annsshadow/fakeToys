## Linux Gadget 串行驱动 v2.0


11/20/2004

（2008-05-08 更新至 v2.3）


### 许可证与免责声明

本程序是自由软件；你可以在自由软件基金会发布的 GNU 通用公共许可证条款下重新分发和/或修改它；可以是许可证的第 2 版，或者（由你选择）任何更晚的版本。

本程序分发的目的是希望它有用，但没有任何担保；甚至没有对适销性或特定用途适用性的默示担保。详见 GNU 通用公共许可证。

你应该已经随本程序收到了一份 GNU 通用公共许可证的副本；如果没有，请写信给自由软件基金会，地址：59 Temple Place, Suite 330, Boston, MA 02111-1307 USA。

本文档以及 gadget 串行驱动本身版权归 (C) 2004 Al Borchers (alborchers@steinerpoint.com) 所有。

如果你对本驱动有疑问、问题或建议，请联系 Al Borchers：alborchers@steinerpoint.com。


### 先决条件

gadget 串行驱动有适用于 2.4 Linux 内核的版本，但本文档假定你正在 2.6 Linux 内核中使用 2.3 或更高版本的 gadget 串行驱动。

本文档假定你熟悉 Linux 和 Windows，并且知道如何配置和构建 Linux 内核、运行标准工具、使用 minicom 和 HyperTerminal，以及使用 USB 和串行设备。它还假定你将 Linux gadget 和 usb 驱动配置为模块。

在驱动的 2.3 版本中，主设备号和次设备号不再静态定义。你的 Linux 系统应当把 sysfs 挂载在 /sys，并使用 “mdev”（Busybox 中）或 “udev” 来创建与 sysfs /sys/class/tty 文件匹配的 /dev 节点。



### 概述

gadget 串行驱动是一个 Linux USB gadget 驱动，即一个 USB 设备侧驱动。它运行在具备 USB 设备侧硬件的 Linux 系统上；例如 PDA、嵌入式 Linux 系统，或带有 USB 开发板的 PC。

gadget 串行驱动通过 USB 与一个 CDC ACM 驱动通信
```

   Host
   --------------------------------------
  | Host-Side   CDC ACM       USB Host   |
  | Operating |   or        | Controller |   USB
  | System    | Generic USB | Driver     |--------
  | (Linux or | Serial      | and        |        |
  | Windows)    Driver        USB Stack  |        |
   --------------------------------------         |
                                                  |
                                                  |
                                                  |
   Gadget                                         |
   --------------------------------------         |
  | Gadget                   USB Periph. |        |
  | Device-Side |  Gadget  | Controller  |        |
  | Linux       |  Serial  | Driver      |--------
  | Operating   |  Driver  | and         |
  | System                   USB Stack   |
   --------------------------------------

```
在设备侧 Linux 系统上，gadget 串行驱动看起来像一个串行设备。

在主机侧系统上，gadget 串行设备看起来像是一个符合 CDC ACM 标准的类设备，或者是一个带有 bulk in 和 bulk out 端点的简单厂商特定设备，并且它的处理方式与其他串行设备类似。

主机侧驱动可以是任意符合 ACM 标准的驱动，或者任何能够与带有简单 bulk in/out 接口的设备通信的驱动。Gadget 串行已经用 Linux ACM 驱动、Windows usbser.sys ACM 驱动以及 Linux USB 通用串行驱动测试过。

在 gadget 串行驱动和主机侧 ACM 或通用串行驱动运行的情况下，你应该能够在主机和 gadget 侧系统之间通信，就像它们通过串行电缆连接一样。

gadget 串行驱动只提供简单的不可靠数据通信。它还未能处理流控或普通串行设备的许多其他特性。


### 安装 Gadget 串行驱动

要使用 gadget 串行驱动，你必须将 Linux gadget 侧内核配置为 “Support for USB Gadgets”、一个 “USB Peripheral Controller”（例如 net2280），以及 “Serial Gadget” 驱动。配置内核时这些都列在 “USB Gadget Support” 下。然后重新构建并安装内核或模块。

然后你必须加载 gadget 串行驱动。要将其作为
```
  modprobe g_serial

```
```
  modprobe g_serial use_acm=0

```
加载，这也会自动加载底层的 gadget 外设控制器驱动。每次重启 gadget 侧 Linux 系统时都必须这样做。如果需要，你可以把它加入启动脚本。

你的系统应当使用 mdev（来自 busybox）或 udev 来创建设备节点。在这个 gadget 驱动设置好之后，你应该看到
```

  # ls -l /dev/ttyGS0 | cat
  crw-rw----    1 root     root     253,   0 May  8 14:10 /dev/ttyGS0
  #

```
注意，主设备号（上面的 253）是系统特定的。如果你需要手动创建 /dev 节点，要使用的正确号码会在 /sys/class/tty/ttyGS0/dev 文件中。

如果你较早链接这个 gadget 驱动（甚至可能静态链接），你可能想设置一个 /etc/inittab 条目来在上面运行 “getty”。/dev/ttyGS0 这一行应当像大多数其他串行端口一样工作。


如果 gadget 串行作为 ACM 设备加载，你会在主机侧使用 Windows 或 Linux ACM 驱动。如果 gadget 串行作为 bulk in/out 设备加载，你会在主机侧使用 Linux 通用串行驱动。请按照下面相应的说明来安装主机侧驱动。


### 安装 Windows 主机 ACM 驱动

要使用 Windows ACM 驱动，你必须拥有 “linux-cdc-acm.inf” 文件（随本文档一起提供），它支持所有近期版本的 Windows。

当 gadget 串行驱动已加载、并且 USB 设备通过 USB 电缆连接到 Windows 主机时，Windows 应当识别 gadget 串行设备并请求驱动。告诉 Windows 在包含 “linux-cdc-acm.inf” 文件的文件夹中查找驱动。

例如，在 Windows XP 上，当 gadget 串行设备首次插入时，“Found New Hardware Wizard” 会启动。选择 “Install from a list or specific location (Advanced)”，然后在下一个屏幕上选择 “Include this location in the search” 并输入路径，或浏览到包含 “linux-cdc-acm.inf” 文件的文件夹。Windows 会抱怨 Gadget Serial 驱动没有通过 Windows Logo 测试，但选择 “Continue anyway” 并完成驱动安装。

在 Windows XP 上，在 “Device Manager”（位于 “Control Panel”、“System”、“Hardware” 下）中展开 “Ports (COM & LPT)” 条目，你应该会看到 “Gadget Serial” 被列为其中一个 COM 端口的驱动。

要卸载 Windows XP 上的 “Gadget Serial” 驱动，请在 “Device Manager” 中右键单击 “Gadget Serial” 条目并选择 “Uninstall”。


### 安装 Linux 主机 ACM 驱动

要使用 Linux ACM 驱动，你必须将 Linux 主机侧内核配置为 “Support for Host-side USB” 和 “USB Modem (CDC ACM) support”。

一旦 gadget 串行驱动已加载、并且 USB 设备通过 USB 电缆连接到 Linux 主机，主机系统应当识别
```

  cat /sys/kernel/debug/usb/devices

```
```

  T:  Bus=01 Lev=01 Prnt=01 Port=01 Cnt=02 Dev#=  5 Spd=480 MxCh= 0
  D:  Ver= 2.00 Cls=02(comm.) Sub=00 Prot=00 MxPS=64 #Cfgs=  1
  P:  Vendor=0525 ProdID=a4a7 Rev= 2.01
  S:  Manufacturer=Linux 2.6.8.1 with net2280
  S:  Product=Gadget Serial
  S:  SerialNumber=0
  C:* #Ifs= 2 Cfg#= 2 Atr=c0 MxPwr=  2mA
  I:  If#= 0 Alt= 0 #EPs= 1 Cls=02(comm.) Sub=02 Prot=01 Driver=acm
  E:  Ad=83(I) Atr=03(Int.) MxPS=   8 Ivl=32ms
  I:  If#= 1 Alt= 0 #EPs= 2 Cls=0a(data ) Sub=00 Prot=00 Driver=acm
  E:  Ad=81(I) Atr=02(Bulk) MxPS= 512 Ivl=0ms
  E:  Ad=02(O) Atr=02(Bulk) MxPS= 512 Ivl=0ms

```
如果主机侧 Linux 系统配置正确，ACM 驱动应当自动加载。命令 “lsmod” 应当显示 “acm” 模块已加载。


### 安装 Linux 主机通用 USB 串行驱动

要使用 Linux 通用 USB 串行驱动，你必须将 Linux 主机侧内核配置为 “Support for Host-side USB”、“USB Serial Converter support” 以及 “USB Generic Serial Driver”。

一旦 gadget 串行驱动已加载、并且 USB 设备通过 USB 电缆连接到 Linux 主机，主机系统应当识别
```

  cat /sys/kernel/debug/usb/devices

```
```

  T:  Bus=01 Lev=01 Prnt=01 Port=01 Cnt=02 Dev#=  6 Spd=480 MxCh= 0
  D:  Ver= 2.00 Cls=ff(vend.) Sub=00 Prot=00 MxPS=64 #Cfgs=  1
  P:  Vendor=0525 ProdID=a4a6 Rev= 2.01
  S:  Manufacturer=Linux 2.6.8.1 with net2280
  S:  Product=Gadget Serial
  S:  SerialNumber=0
  C:* #Ifs= 1 Cfg#= 1 Atr=c0 MxPwr=  2mA
  I:  If#= 0 Alt= 0 #EPs= 2 Cls=0a(data ) Sub=00 Prot=00 Driver=serial
  E:  Ad=81(I) Atr=02(Bulk) MxPS= 512 Ivl=0ms
  E:  Ad=02(O) Atr=02(Bulk) MxPS= 512 Ivl=0ms

```
你必须加载 usbserial 驱动并显式设置其参数
```

  echo 0x0525 0xA4A6 >/sys/bus/usb-serial/drivers/generic/new_id

```
```

  modprobe usbserial vendor=0x0525 product=0xA4A6

```
如果一切正常，usbserial 会在系统日志中打印一条类似 “Gadget Serial converter now attached to ttyUSB0” 的消息。


### 使用 Minicom 或 HyperTerminal 测试

一旦 gadget 串行驱动和主机驱动都安装好，并且 USB 电缆将 gadget 设备连接到主机，你就应该能够在 gadget 和主机系统之间通过 USB 通信。你可以使用 minicom 或 HyperTerminal 来尝试。

在 gadget 侧运行 “minicom -s” 来配置一个新的 minicom 会话。在 “Serial port setup” 下将 “/dev/ttygserial” 设为 “Serial Device”。将波特率、数据位、校验位和停止位设为 9600、8、none 和 1——这些设置基本无关紧要。在 “Modem and dialing” 下清除所有调制解调器和拨号字符串。

在运行 ACM 驱动的 Linux 主机上，类似地配置 minicom，但使用 “/dev/ttyACM0” 作为 “Serial Device”。（如果你连接了其他 ACM 设备，请相应地更改设备名。）

在运行 USB 通用串行驱动的 Linux 主机上，类似地配置 minicom，但使用 “/dev/ttyUSB0” 作为 “Serial Device”。（如果你连接了其他 USB 串行设备，请相应地更改设备名。）

在 Windows 主机上，配置一个新的 HyperTerminal 会话以使用分配给 Gadget Serial 的 COM 端口。“Port Settings” 会在 HyperTerminal 连接到 gadget 串行设备时自动设置，因此你可以将其保留为默认值——这些设置基本无关紧要。

在 gadget 侧配置并运行 minicom，并且在主机侧配置并运行 minicom 或 HyperTerminal 之后，你应该能够在 gadget 侧和主机侧系统之间来回发送数据。你在 gadget 侧终端窗口中键入的任何内容都应该出现在主机侧的终端窗口中，反之亦然。
