## UCAN 协议


UCAN 是基于微控制器的 USB-CAN 适配器所使用的协议，该适配器集成在 Theobroma Systems System-on-Module 上，也可作为独立USB 棒获得
UCAN 协议被设计为与硬件无关。它紧密地模仿了 Linux 内部表示 CAN 设备的方式。所有多字节整数都编码为 Little Endian
本文档中提到的所有结构都定义`drivers/net/can/usb/ucan.c`
## USB 端点


UCAN 设备使用三个 USB 端点
CONTROL 端点
  驱动在此端点上发送设备管理命
IN 端点
  设备发CAN 数据帧和 CAN 错误
OUT 端点
  驱动OUT 端点上发CAN 数据
## 控制消息（CONTROL Messages

UCAN 设备通过使用控制管道（control pipe）上的厂商请求（vendor request）进行配置
为了支持单个 USB 设备中的多个 CAN 接口，所有配置命令都指向 USB 描述符中相应的接口
驱动使用 `ucan_ctrl_command_in/out` `ucan_device_request_in` 向设备传递命令
### 建立包（Setup Packet

=================  =====================================================
`bmRequestType`  方向 | 厂商（Vendor| （接口或设备`bRequest`       命令编号
`wValue`         子命令编号（16 位），若未使用则0
`wIndex`         USB 接口索引（设备命令为 0`wLength`        * 主机到设- 要发送的字节                   - 设备到主- 要接收的最大字节数
                     如果设备发送较少，则使用常见的 ZLP 语义=================  =====================================================

### 错误处理


设备通过阻塞（stall）该管道来指示失败的控制命令
### 设备命令


#### UCAN_DEVICE_GET_FW_STRING


**Dev2Host；可*

请求设备固件字符串
### 接口命令


#### UCAN_COMMAND_START


**Host2Dev；必需**

启动 CAN 接口
Payload 格式
  `ucan_ctl_payload_t.cmd_start`

====  ============================
mode  `UCAN_MODE_*` 的或掩码
====  ============================

#### UCAN_COMMAND_STOP


**Host2Dev；必需**

停止 CAN 接口

Payload 格式
  **绌?*

#### UCAN_COMMAND_RESET


**Host2Dev；必需**

复位 CAN 控制器（包括错误计数器）

Payload 格式
  **绌?*

#### UCAN_COMMAND_GET


**Host2Dev；必需**

从设备获取信
##### 子命

UCAN_COMMAND_GET_INFO
  请求设备信息结构 `ucan_ctl_payload_t.device_info`
  细节参见 `device_info` 字段，以  `uapi/linux/can/netlink.h` 中对 `can_bittiming 字段` 的说明
  Payload 格式
    `ucan_ctl_payload_t.device_info`

UCAN_COMMAND_GET_PROTOCOL_VERSION

  请求设备协议版本
  `ucan_ctl_payload_t.protocol_version`。当前协议版本为 3
  Payload 格式
    `ucan_ctl_payload_t.protocol_version`

          protocol version 1

#### UCAN_COMMAND_SET_BITTIMING


**Host2Dev；必需**

通过发送结`ucan_ctl_payload_t.cmd_set_bittiming`（细节见 `struct bittiming`）来设置位时序（bittiming
Payload 格式
  `ucan_ctl_payload_t.cmd_set_bittiming`銆。
#### UCAN_SLEEP/WAKE


**Host2Dev；可*

配置睡眠和唤醒模式。驱动尚不支持
#### UCAN_FILTER


**Host2Dev；可*

设置硬件 CAN 过滤器。驱动尚不支持
### 允许的接口命

==================  ===================  ==================
合法设备状        命令                 新设备状==================  ===================  ==================
stopped             SET_BITTIMING        stopped
stopped             START                started
started             STOP or RESET        stopped
stopped             STOP or RESET        stopped
started             RESTART              started
any                 GET                  **无变*
==================  ===================  ==================

## IN 消息格式


USB IN 端点上的数据包包含一个或多个 `ucan_message_in` 值。如果多个消息被批处理在一USB 数据包中，`len` 字段可用于跳到下一`ucan_message_in` 值（注意`len` 值做健全性检查，以对照实际数据大小）
### ``len`` 字段


每个 `ucan_message_in` 必须对齐4 字节边界（相对于数据缓冲区起始的位置）。这意味着在多`ucan_message_in` 值之间可能有填充字节
    +----------------------------+ < 0
    |                            |
    |   struct ucan_message_in   |
    |                            |
    +----------------------------+ < len
              [padding]
    +----------------------------+ < round_up(len, 4)
    |                            |
    |   struct ucan_message_in   |
    |                            |
    +----------------------------+
                [...]

### ``type`` 字段


`type` 字段指定消息的类型
#### UCAN_IN_RX


`subtype`
  zero

CAN 总线接收到的数据（ID + 载荷）
#### UCAN_IN_TX_COMPLETE


`subtype`
  zero

CAN 设备已向 CAN 总线发送了一条消息。它用一个元组列<echo-ids, flags> 作为应答
echo-id 标识了来自（回显了先UCAN_OUT_TX 消息id）的帧。flag 指示传输的结果。其中，置位Bit 0 表示成功。所有其他位保留并设为零
### 流控


接收 CAN 消息时，USB 缓冲区上没有流控。驱动必须足够快地处理入站消息以避免丢包。如果设备缓冲区溢出，该状况会通过发送相应的错误帧来报告（参can_ucan_error_handling）
## OUT 消息格式


USB OUT 端点上的数据包包含一个或多个 ``struct ucan_message_out`` 值。如果多个消息被批处理到一个数据包中，设备使用 `len` 字段跳到下一ucan_message_out 值。每ucan_message_out 必须对齐4 字节（相对于数据缓冲区起始的位置）。该机制can_ucan_in_message_len 中描述的一样
    +----------------------------+ < 0
    |                            |
    |   struct ucan_message_out  |
    |                            |
    +----------------------------+ < len
              [padding]
    +----------------------------+ < round_up(len, 4)
    |                            |
    |   struct ucan_message_out  |
    |                            |
    +----------------------------+
                [...]

### ``type`` 字段


在协议版3 中只定义`UCAN_OUT_TX`，其他的仅由旧设备（协议版本 1）使用
#### UCAN_OUT_TX

`subtype`
  要在 CAN_IN_TX_COMPLETE 消息中应答的 echo id

发送一CAN 帧。（参数：`id`、`data`
### 流控


当设备出站缓冲区满时，它开始在 **OUT** 管道上发**NAK**，直到有更多缓冲区可用。当未完成出站包达到一定阈值时，驱动停止队列
## CAN 错误处理


如果开启了错误报告，设备会把错误编码为 CAN 错误帧（参见 `uapi/linux/can/error.h`）并通过 IN 端点发送。驱动更新其错误统计并转发它
尽管 UCAN 设备可以完全抑制错误帧，但在 Linux 中驱动总是感兴趣的。因此，设备总是以设置了 `UCAN_MODE_BERR_REPORT` 的方式启动。为 user space 过滤这些消息由驱动完成
### 总线关闭（Bus OFF

- 设备不会自动从总线关闭中恢复- 总线关闭由错误帧指示（参`uapi/linux/can/error.h`- 总线关闭恢复`UCAN_COMMAND_RESTART` 启动
- 一旦总线关闭恢复完成，设备发送一个错误帧，指示其处于 ERROR-ACTIVE 状态- 在总线关闭期间，设备不发送任何帧- 在总线关闭期间，来自主机的传输请求会立即完成，且成功位保持未置位
## 示例会话


#) 设备连接USB
#) 主机发送命`UCAN_COMMAND_RESET`，subcmd 0
#) 主机发送命`UCAN_COMMAND_GET`，subcmd `UCAN_COMMAND_GET_INFO`
#) 设备发`UCAN_IN_DEVICE_INFO`
#) 主机发送命`UCAN_OUT_SET_BITTIMING`
#) 主机发送命`UCAN_COMMAND_START`，subcmd 0，mode `UCAN_MODE_BERR_REPORT`
