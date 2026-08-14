


## 基于 MTK PCIe 的 T700 5G 调制解调器的 t7xx 驱动


t7xx 驱动是一个为 linux 或 Chrome OS 平台开发的 WWAN PCIe 主机驱动，用于在主機平台
与 MediaTek 的 T700 5G 调制解调器之间通过 PCIe 接口进行数据交换。
该驱动暴露了一个符合 MBIM 协议 [^1^] 的接口。任何前端应用程序（例如 Modem Manager）
都可以轻松管理 MBIM 接口以启用通往 WWAN 的数据通信。该驱动还提供了一个通过 AT 命令
与 MediaTek 调制解调器交互的接口。

## 基本用法


当不受管理时，MBIM 和 AT 功能处于非活动状态。t7xx 驱动提供代表 MBIM 和 AT 控制通道
的 WWAN 端口用户空间接口，但在管理其功能方面不起任何作用。检测端口枚举并启用 MBIM
和 AT 功能是用户空间应用程序的工作。

几个这样的用户空间应用程序示例：

- mbimcli（包含在 libmbim [^2^] 库中），以及
- Modem Manager [^3^]

管理应用程序执行以下建立 MBIM IP 会话所需的操作：

- 打开 MBIM 控制通道
- 配置网络连接设置
- 连接到网络
- 配置 IP 网络接口

管理应用程序执行以下发送 AT 命令并接收响应所需的操作：

- 使用 UART 工具或专用用户工具打开 AT 控制通道

## Sysfs


该驱动向用户空间提供 sysfs 接口。

### t7xx_mode


该 sysfs 接口向用户空间提供对设备模式的访问，此接口支持读和写操作。

设备模式：

- `unknown` 表示设备处于未知状态
- `ready` 表示设备处于就绪状态
- `reset` 表示设备处于复位状态
- `fastboot_switching` 表示设备处于 fastboot 切换状态
- `fastboot_download` 表示设备处于 fastboot 下载状态
- `fastboot_dump` 表示设备处于 fastboot 转储状态

从用户空间读取以获取当前设备模式。

```
  $ cat /sys/bus/pci/devices/${bdf}/t7xx_mode

```
从用户空间写入以设置设备模式。

```
  $ echo fastboot_switching > /sys/bus/pci/devices/${bdf}/t7xx_mode

```
### t7xx_debug_ports


该 sysfs 接口向用户空间提供启用/禁用调试端口的访问，此接口支持读和写操作。

调试端口状态：

- `1` 表示启用调试端口
- `0` 表示禁用调试端口

当前支持的调试端口（ADB/MIPC）。

从用户空间读取以获取当前调试端口状态。

```
  $ cat /sys/bus/pci/devices/${bdf}/t7xx_debug_ports

```
从用户空间写入以设置调试端口状态。

```
  $ echo 1 > /sys/bus/pci/devices/${bdf}/t7xx_debug_ports

```
## 管理应用程序开发


驱动和用户空间接口描述如下。MBIM 协议在 [^1^] Mobile Broadband Interface Model
v1.0 Errata-1 中描述。

### MBIM 控制通道用户空间 ABI


#### /dev/wwan0mbim0 字符设备


该驱动通过实现 MBIM WWAN 端口向 MBIM 功能暴露一个 MBIM 接口。控制通道管道的用户空间
一端是一个 /dev/wwan0mbim0 字符设备。应用程序应使用此接口进行 MBIM 协议通信。

#### 分片


用户空间应用程序负责按照 MBIM 规范进行所有控制消息的分片和重组。

#### /dev/wwan0mbim0 write()


来自管理应用程序的 MBIM 控制消息不得超过协商的控制消息大小。

#### /dev/wwan0mbim0 read()


管理应用程序必须接受协商控制消息大小的控制消息。

### MBIM 数据通道用户空间 ABI


#### wwan0-X 网络设备


t7xx 驱动暴露类型为 "wwan" 的 IP 链路接口 "wwan0-X"，用于 IP 流量。Iproute 网络
实用程序用于创建 "wwan0-X" 网络接口，并将其与 MBIM IP 会话关联。

用户空间管理应用程序负责在建立 SessionId 大于 0 的 MBIM IP 会话之前创建新的 IP 链路。

例如，为 SessionId 为 1 的 MBIM IP 会话创建新的 IP 链路：

  ip link add dev wwan0-1 parentdev wwan0 type wwan linkid 1

该驱动将自动把 "wwan0-1" 网络设备映射到 MBIM IP 会话 1。

### AT 端口用户空间 ABI


#### /dev/wwan0at0 字符设备


该驱动通过实现 AT WWAN 端口暴露一个 AT 端口。控制端口的用户空间一端是一个
/dev/wwan0at0 字符设备。应用程序应使用此接口发出 AT 命令。

### fastboot 端口用户空间 ABI


#### /dev/wwan0fastboot0 字符设备


该驱动通过实现 fastboot WWAN 端口暴露一个 fastboot 协议接口。fastboot 通道管道的
用户空间一端是一个 /dev/wwan0fastboot0 字符设备。应用程序应使用此接口进行 fastboot
协议通信。

请注意，驱动需要重新加载以导出 /dev/wwan0fastboot0 端口，因为设备在进入 `fastboot_switching`
模式后需要冷复位。

### ADB 端口用户空间 ABI


#### /dev/wwan0adb0 字符设备


该驱动通过实现 ADB WWAN 端口暴露一个 ADB 协议接口。ADB 通道管道的用户空间一端是一个
/dev/wwan0adb0 字符设备。应用程序应使用此接口进行 ADB 协议通信。

### MIPC 端口用户空间 ABI


#### /dev/wwan0mipc0 字符设备


该驱动通过实现 MIPC（Modem Information Process Center）WWAN 端口暴露一个诊断接口。
MIPC 通道管道的用户空间一端是一个 /dev/wwan0mipc0 字符设备。
应用程序应使用此接口进行 MTK 调制解调器诊断通信。

MediaTek 的 T700 调制解调器支持 3GPP TS 27.007 [^4^] 规范。

## 参考


[^1^] **MBIM (Mobile Broadband Interface Model) Errata-1**

- https://www.usb.org/document-library/

[^2^] *libmbim "a glib-based library for talking to WWAN modems and devices which
speak the Mobile Interface Broadband Model (MBIM) protocol"*

- http://www.freedesktop.org/wiki/Software/libmbim/

[^3^] *Modem Manager "a DBus-activated daemon which controls mobile broadband
(2G/3G/4G/5G) devices and connections"*

- http://www.freedesktop.org/wiki/Software/ModemManager/

[^4^] **Specification # 27.007 - 3GPP**

- https://www.3gpp.org/DynaReport/27007.htm

[^5^] **fastboot "a mechanism for communicating with bootloaders"**

- https://android.googlesource.com/platform/system/core/+/refs/heads/main/fastboot/README.md

[^6^] *ADB (Android Debug Bridge) "a mechanism to keep track of Android devices
and emulators instances connected to or running on a given host developer
machine with ADB protocol"*

- https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/main/README.md
