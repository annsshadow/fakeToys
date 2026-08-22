## 面向 Intel M.2 PCIe 调制解调器的 IOSM 驱动


IOSM（IPC over Shared Memory，基于共享内存的 IPC）驱动是一WWAN PCIe 主机
驱动，为 Linux Chrome 平台开发，用于在主机平台与 Intel M.2 调制解调器之通过 PCIe 接口交换数据。该驱动提供符合 MBIM 协议 [^1^] 的接口。任何前端应程序（如 Modem Manager）都可以轻松管理MBIM 接口，以启用通往 WWAN 的数通信
## 基本用法


未受管理时，MBIM 功能处于非活动状态。IOSM 驱动仅提供一个用户态接MBIM
“WWAN PORT”，代表 MBIM 控制通道，并不参与功能的管理。检测端口枚举并启用
MBIM 功能是用户态应用程序的职责
此类用户态应用程序的例子有：
- mbimcli（随 libmbim [^2^] 库一同提供），以- Modem Manager [^3^]

管理应用程序需要执行以下必要操作以建立 MBIM IP 会话- 打开 MBIM 控制通道
- 配置网络连接设置
- 连接到网- 配置 IP 网络接口

## 管理应用程序开

驱动与用户态接口描述如下。MBIM 协议[^1^] Mobile Broadband Interface
Model v1.0 Errata-1 中描述
### MBIM 控制通道用户ABI


#### /dev/wwan0mbim0 字符设备


该驱动通过实现 MBIM WWAN Port MBIM 功能暴露一MBIM 接口。控制通道管道用户态一端是 /dev/wwan0mbim0 字符设备。应用程序应使用此接口进MBIM 协议
通信
#### 分片（Fragmentation

用户态应用程序负责按MBIM 规范进行所有控制消息的分片与重组
#### /dev/wwan0mbim0 write()


来自管理应用程序MBIM 控制消息不得超过协商的控制消息大小
#### /dev/wwan0mbim0 read()


管理应用程序必须接受等于协商控制消息大小的控制消息
### MBIM 数据通道用户ABI


#### wwan0-X 网络设备


IOSM 驱动IP 流量暴露一个类型为 “wwanIP 链路接口 “wwan0-X”。Iproute
网络工具用于创建 “wwan0-X网络接口并将其关联到 MBIM IP 会话。该驱动支持
最8 IP 会话以进行并IP 通信
用户态管理应用程序负责在建立 SessionId 大于 0 MBIM IP 会话之前创建新的
IP 链路
例如，为 SessionId 1 MBIM IP 会话创建新的 IP 链路
  ip link add dev wwan0-1 parentdev-name wwan0 type wwan linkid 1

驱动将自动把 “wwan0-1网络设备映射MBIM IP 会话 1
## 参

[^1^] "MBIM (Mobile Broadband Interface Model) Errata-1"
      - https://www.usb.org/document-library/

[^2^] libmbim - "a glib-based library for talking to WWAN modems and
      devices which speak the Mobile Interface Broadband Model (MBIM)
      protocol"
      - http://www.freedesktop.org/wiki/Software/libmbim/

[^3^] Modem Manager - "a DBus-activated daemon which controls mobile
      broadband (2G/3G/4G) devices and connections"
      - http://www.freedesktop.org/wiki/Software/ModemManager/
