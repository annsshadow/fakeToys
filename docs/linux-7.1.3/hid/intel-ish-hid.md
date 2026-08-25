## Intel 集成传感器中枢（ISH


传感器中枢（sensor hub）能够将传感器轮询和算法处理的工作卸载到一个专用的
低功耗协处理器上。这使得核心处理器能够更频繁地进入低功耗模式，从而延
电池续航时间

有许多供应商提供符合 HID Sensor 使用表的外部传感器中枢。它们可见于平板
电脑、二合一可转换笔记本-in-1 convertible laptops）和嵌入式产品中
Linux Linux 3.9 起就支持这一特性

Intel® Cherry Trail 开始，作为 SoC 的一部分引入了集成传感器中枢，目
已在多代 CPU 封装上得到支持。已经有许多商业设备搭载了集成传感器中枢（ISH
出货。这ISH 同样符合 HID 传感器规范，但区别在于用于通信的传输协议
当前的外部传感器中枢主要使用 HID over I2C USB。但 ISH 两者都不使用，
既不I2C 也不USB

## 概述


用一个与 usbhid 实现相类比的方式来说，ISH 遵循类似的模
```

	-----------------		----------------------
	|    USB HID	|	-->	|    ISH HID	     |
	-----------------		----------------------
	-----------------		----------------------
	|  USB protocol	|	-->	|    ISH Transport   |
	-----------------		----------------------
	-----------------		----------------------
	|  EHCI/XHCI	|	-->	|    ISH IPC	     |
	-----------------		----------------------
	      PCI				 PCI
	-----------------		----------------------
	|Host controller|	-->	|    ISH processor   |
	-----------------		----------------------
	     USB Link
	-----------------		----------------------
	| USB End points|	-->	|    ISH Clients     |
	-----------------		----------------------

```
就像 USB 协议提供了一种用于设备枚举、链路管理和用户数据封装的方法一样，
ISH 也提供类似的服务。但它非常轻量，专为管理和与固件中实现的 ISH 客户
应用通信而量身定制

ISH 允许固件中执行多个传感器管理应用。如USB 端点，消息可以发往或来
一个客户端。作为枚举过程的一部分，这些客户端会被识别出来。这些客户端可以
是简单的 HID 传感器应用、传感器校准应用或传感器固件更新应用

实现模型是类似的，如USB 总线，ISH 传输也被实现为一个总线。在 ISH
处理器中执行的每个客户端应用都在该总线上注册为一个设备。将每个设备绑定
起来的驱动（ISH HID 驱动）会识别设备类型，并HID 核心注册

## ISH 实现：框


```

	 ---------------------------
	|  User Space Applications  |
	 ---------------------------

  ----------------IIO ABI----------------
	 --------------------------
	|  IIO Sensor Drivers	  |
	 --------------------------
	 --------------------------
	|	 IIO core	  |
	 --------------------------
	 --------------------------
	|   HID Sensor Hub MFD	  |
	 --------------------------
	 --------------------------
	|       HID Core	  |
	 --------------------------
	 --------------------------
	|   HID over ISH Client   |
	 --------------------------
	 --------------------------
	|   ISH Transport (ISHTP) |
	 --------------------------
	 --------------------------
	|      IPC Drivers	  |
	 --------------------------
  OS
  ---------------- PCI -----------------
  Hardware + Firmware
	 ----------------------------
	| ISH Hardware/Firmware(FW) |
	 ----------------------------

```
## 上述各模块中的高层处


### 硬件接口


ISH 对主机暴露为“Non-VGA 未分PCI 设备”。PCI 的产品和厂商 ID 在不
代处理器之间会变化。因此用于枚举驱动的源代码需要一代一代地更新

### 处理器间通信（IPC）驱


位置：drivers/hid/intel-ish-hid/ipc

IPC 消息使用内存映射 I/O。寄存器定义hw-ish-regs.h 中

##### IPC/FW 消息类型


有两类消息，一类用于链路管理，另一类用于传输层之间的消息

传输消息的发送与接收（TX and RX
.......................................

一组内存映射寄存器提供对多字节消息发送与接收（例IPC_REG_ISH2HOST_MSG
IPC_REG_HOST2ISH_MSG）的支持。IPC 层维护内部队列以对消息排序并按顺
发送给固件。调用方还可以选择性地注册处理程序以获取完成通知。在消息传递中
使用门铃（doorbell）机制来触发主机和客户端固件侧的处理。当 ISH 中断处理
程序被调用时，主机驱动使ISH2HOST 门铃寄存器来确定该中断是发给 ISH 的

每一侧有 32 32 位消息寄存器1 32 位门铃。门
```

  Bits 0..6: fragment length (7 bits are used)
  Bits 10..13: encapsulated protocol
  Bits 16..19: management command (for IPC management protocol)
  Bit 31: doorbell trigger (signal H/W interrupt to the other side)
  Other bits are reserved, should be 0.

```

##### 浼犺緭灞傛帴鍙。


为了抽象硬件级的 IPC 通信，注册了一组回调（callback）。传输层使用它们
发送和接收消息。有关回调请参struct ishtp_hw_ops

### ISH 浼犺緭灞。


位置：drivers/hid/intel-ish-hid/ishtp/

##### 閫氱敤浼犺緭灞。


传输层是一个双向协议，它定义了
- 一组用于启动、停止、连接、断开和流控的命令
（详ishtp/hbm.h
- 一种用于避免缓冲区溢出的流控机

该协议类似于以下文档中描述的总线消息
http://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/dcmi-hi-1-0-spec.pdf
鈥淐hapter 7: Bus Message Layer鈥濄€?

##### 连接与流控机


每个 FW 客户端和每个协议都由 UUID 标识。为了与某个 FW 客户端通信，必
使用 connect 请求和响应总线消息来建立连接。如果成功，一
（host_client_id fw_client_id）将标识该连接

一旦连接建立，对等方彼此独立地发送流控总线消息。每个对等方只有在之
收到过流控信用（flow-control credit）时才可以发送消息。一旦它发送了一
消息，在收到下一个流控信用之前，它都不能再发送另一条消息

任一方都可以发disconnect 请求总线消息来结束通信。此外，如果发生重大
FW 重置，链路也会被丢弃

##### 对等数据传输


对等（Peer to Peer）数据传输可以在使用或不使用 DMA 的情况下发生。根
传感器带宽需求，DMA 可以通过 intel_ishtp 下的模块参数 ishtp_use_dma 
启用

每一侧（主机FW）独立管理其 DMA 传输内存。当来自主机FW 侧的某个
ISHTP 客户端想要发送某些内容时，它会独立地决定是通过 IPC 还是 DMA 发送；
每次传输的决定都是独立的。发送方在消息位于相应的主机缓冲区中时发
DMA_XFER 消息（发送时为主机客户端 TX，接收时FW 客户RX）。DMA 消息
接收方以 DMA_XFER_ACK 响应，向发送方表明该消息的内存区域可以被重用

DMA 初始化由主机发DMA_ALLOC_NOTIFY 总线消息（包RX 缓冲区）开始，FW
DMA_ALLOC_NOTIFY_ACK 响应。除DMA 地址通信之外，该序列还检查能力：
如果主机不支DMA，那么它不会发DMA 分配，因FW 无法发DMA；如
FW 不支DMA，那么它不会DMA_ALLOC_NOTIFY_ACK 响应，在这种情况下主机将
不使DMA 传输

这里 ISH 充当总线主控（busmaster）DMA 控制器。因此，当主机发DMA_XFER
时，它是请求执行 host->ISH DMA 传输；当 FW 发DMA_XFER 时，意味着
已经完成DMA，消息驻留在主机处。因此，DMA_XFER DMA_XFER_ACK 充当
所有权指示器

在初始状态下，所有传出内存都属于发送方（TX 属于主机，RX 属于 FW），
DMA_XFER 将包ISHTP 消息的区域的所有权转移给接收方，DMA_XFER_ACK 
所有权返还给发送方。发送方无需等待先前DMA_XFER 被确认（ack），只要
拥有的剩余连续内存足够，就可以发送另一条消息。原则上，可以一次性发送多
DMA_XFER DMA_XFER_ACK 消息（最多到 IPC MTU），从而允许进行中断节
（interrupt throttling）。目前，如果 ISHTP 消息超过 3 IPC 分片（fragment），
ISH FW 决定通过 DMA 发送，否则通过 IPC 发送

##### 环形缓冲


当客户端发起连接时，会分配一RX TX 缓冲区。环的大小可以由客户端指定
HID 客户端分别将 TX RX 缓冲区设置为 16 32。在客户端的发送请求上，要
发送的数据被复制到其中一个发送环形缓冲区中，并安排使用总线消息协议发送
需要这些缓冲区，因FW 可能尚未处理上一条消息，并且可能没有足够的流
信用来发送。接收侧同样如此，因此需要流控

##### 主机枚举


主机枚举总线命令允许发现 FW 中存在的客户端。可以存在多个传感器客户端以
用于校准功能的客户端

为了简化实现并允许独立的驱动来处理每个客户端，该传输层利用Linux 总线
驱动模型。每个客户端都在传输总线（ishtp 总线）上注册为一个设备

枚举消息序列

- 主机发HOST_START_REQ_CMD，表明主ISHTP 层已就绪
- FW HOST_START_RES_CMD 响应
- 主机发HOST_ENUM_REQ_CMD（枚FW 客户端）
- FW HOST_ENUM_RES_CMD 响应，其中包含可FW 客户ID 的位图
- 对于该位图中找到的每FW ID，主机发
  HOST_CLIENT_PROPERTIES_REQ_CMD銆。
- FW HOST_CLIENT_PROPERTIES_RES_CMD 响应。属性包UUID
  ISHTP 消息最大大小等
- 一旦主机收到最后一个被发现的客户端的属性，它就认为 ISHTP 设备已完
  功能正常（并分配 DMA 缓冲区）

### HID over ISH 瀹㈡埛绔。


位置：drivers/hid/intel-ish-hid

ISHTP 客户端驱动负责：

- 枚举 FW ISH 客户端下HID 设备
- 获取报告描述符（Report descriptor
- 作为 LL 驱动HID 核心注册
- 处理 Get/Set 特性请
- 获取输入报告

### HID 传感器中MFD IIO 传感器驱


这些驱动中的功能与外部传感器中枢相同。请参
Documentation/hid/hid-sensor.rst 以了HID 传感器，
Documentation/ABI/testing/sysfs-bus-iio 以了IIO 向用户空间的 ABI

### 端到HID 传输时序


```

  HID-ISH-CLN                    ISHTP                    IPC                             HW
          |                        |                       |                               |
          |                        |                       |-----WAKE UP------------------>|
          |                        |                       |                               |
          |                        |                       |-----HOST READY--------------->|
          |                        |                       |                               |
          |                        |<----MNG_RESET_NOTIFY_ACK----- |
          |                        |                       |                               |
          |                        |<----ISHTP_START------ |                               |
          |                        |                       |                               |
          |                        |<-----------------HOST_START_RES_CMD-------------------|
          |                        |                       |                               |
          |                        |------------------QUERY_SUBSCRIBER-------------------->|
          |                        |                       |                               |
          |                        |------------------HOST_ENUM_REQ_CMD------------------->|
          |                        |                       |                               |
          |                        |<-----------------HOST_ENUM_RES_CMD--------------------|
          |                        |                       |                               |
          |                        |------------------HOST_CLIENT_PROPERTIES_REQ_CMD------>|
          |                        |                       |                               |
          |                        |<-----------------HOST_CLIENT_PROPERTIES_RES_CMD-------|
          |       Create new device on in ishtp bus        |                               |
          |                        |                       |                               |
          |                        |------------------HOST_CLIENT_PROPERTIES_REQ_CMD------>|
          |                        |                       |                               |
          |                        |<-----------------HOST_CLIENT_PROPERTIES_RES_CMD-------|
          |       Create new device on in ishtp bus        |                               |
          |                        |                       |                               |
          |                        |--Repeat HOST_CLIENT_PROPERTIES_REQ_CMD-till last one--|
          |                        |                       |                               |
       probed()
          |----ishtp_cl_connect--->|----------------- CLIENT_CONNECT_REQ_CMD-------------->|
          |                        |                       |                               |
          |                        |<----------------CLIENT_CONNECT_RES_CMD----------------|
          |                        |                       |                               |
          |register event callback |                       |                               |
          |                        |                       |                               |
          |ishtp_cl_send(
          HOSTIF_DM_ENUM_DEVICES)  |----------fill ishtp_msg_hdr struct write to HW-----  >|
          |                        |                       |                               |
          |                        |<-----IRQ(IPC_PROTOCOL_ISHTP---|
          |                        |                       |                               |
          |<--ENUM_DEVICE RSP------|                       |                               |
          |                        |                       |                               |
  for each enumerated device
          |ishtp_cl_send(
          HOSTIF_GET_HID_DESCRIPTOR|----------fill ishtp_msg_hdr struct write to HW-----  >|
          |                        |                       |                               |
          ...Response
          |                        |                       |                               |
  for each enumerated device
          |ishtp_cl_send(
       HOSTIF_GET_REPORT_DESCRIPTOR|--------------fill ishtp_msg_hdr struct write to HW-- >|
          |                        |                       |                               |
          |                        |                       |                               |
   hid_allocate_device
          |                        |                       |                               |
   hid_add_device                  |                       |                               |


```

### 从主机加ISH 固件流程


Lunar Lake 这一代开始，ISH 固件被划分为两个组件，以获得更好的空间优
和更高的灵活性。这些组件包括一个集成在 BIOS 中的引导加载程序（bootloader），
以及一个存储在操作系统文件系统内的主固件（main firmware）

该过程工作方式如下：

- 最初，ISHTP 驱动ISH 引导加载程序发送一个命HOST_START_REQ_CMD
  作为响应，引导加载程序发回一HOST_START_RES_CMD。该响应包含
  ISHTP_SUPPORT_CAP_LOADER 位。随后，ISHTP 驱动检查该位是否被设置。如果是
  则从主机进行的固件加载过程开始

- 在此过程中，ISHTP 驱动首先调用 request_firmware() 函数，然后发送一
  LOADER_CMD_XFER_QUERY 命令。在收到来自引导加载程序的响应后，ISHTP 驱动
  发送一LOADER_CMD_XFER_FRAGMENT 命令。在收到另一个响应后，ISHTP 驱动
  发送一LOADER_CMD_START 命令。引导加载程序做出响应，然后跳转到主固件

- 该过程结束后，ISHTP 驱动调用 release_firmware() 函数

有关更详细的信息，请参阅下面提供的流程描述：

```

  +---------------+                                                    +-----------------+
  | ISHTP Driver  |                                                    | ISH Bootloader  |
  +---------------+                                                    +-----------------+
          |                                                                     |
          |~~~Send HOST_START_REQ_CMD~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>|
          |                                                                     |
          |<--Send HOST_START_RES_CMD(Includes ISHTP_SUPPORT_CAP_LOADER bit)----|
          |                                                                     |
  ****************************************************************************************
  * if ISHTP_SUPPORT_CAP_LOADER bit is set                                               *
  ****************************************************************************************
          |                                                                     |
          |~~~start loading firmware from host process~~~+                      |
          |                                              |                      |
          |<---------------------------------------------+                      |
          |                                                                     |
  ---------------------------                                                   |
  | Call request_firmware() |                                                   |
  ---------------------------                                                   |
          |                                                                     |
          |~~~Send LOADER_CMD_XFER_QUERY~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>|
          |                                                                     |
          |<--Send response-----------------------------------------------------|
          |                                                                     |
          |~~~Send LOADER_CMD_XFER_FRAGMENT~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>|
          |                                                                     |
          |<--Send response-----------------------------------------------------|
          |                                                                     |
          |~~~Send LOADER_CMD_START~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>|
          |                                                                     |
          |<--Send response-----------------------------------------------------|
          |                                                                     |~~~Jump to Main Firmware~~~+
          |                                                                     |                           |
          |                                                                     |<--------------------------+
          |                                                                     |
  ---------------------------                                                   |
  | Call release_firmware() |                                                   |
  ---------------------------                                                   |
          |                                                                     |
  ****************************************************************************************
  * end if                                                                               *
  ****************************************************************************************
          |                                                                     |
  +---------------+                                                    +-----------------+
  | ISHTP Driver  |                                                    | ISH Bootloader  |
  +---------------+                                                    +-----------------+

```

##### 供应商自定义固件加载


运行ISH 内部的固件可以由 Intel 提供，也可以由供应商使用 Intel 提供
固件开发套件（FDK，Firmware Development Kit）开发。Intel 会将 Intel 构建
固件上游`linux-firmware.git` 仓库，路径位`intel/ish/` 下。对
Lunar Lake 平台，Intel 构建ISH 固件将命名为 `ish_lnlm.bin`

希望将其自定义固件上游的供应商应遵循以下命名其固件文件的准则

- 固件文件名应使用以下模式之一

  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_NAME_CRC32}_${PRODUCT_SKU_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_SKU_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_NAME_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_NAME_CRC32}_${PRODUCT_SKU_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_SKU_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_NAME_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}.bin`

- `${intel_plat_gen}` 表示 Intel 平台代次（例`lnlm` 代表 Lunar Lake），
  且长度不得超8 个字符
- `${SYS_VENDOR_CRC32}` 是来DMI 字段 `DMI_SYS_VENDOR` `sys_vendor`
  值的 CRC32 校验和
- `${PRODUCT_FAMILY_CRC32}` 是来DMI 字段 `DMI_PRODUCT_FAMILY` 
  `product_family` 值的 CRC32 校验和
- `${PRODUCT_NAME_CRC32}` 是来DMI 字段 `DMI_PRODUCT_NAME` `product_name`
  值的 CRC32 校验和
- `${PRODUCT_SKU_CRC32}` 是来DMI 字段 `DMI_PRODUCT_SKU` `product_sku`
  值的 CRC32 校验和

在系统启动期间，ISH Linux 驱动将尝试按以下顺序加载固件，优先使用匹配模
更精确的自定义固件：

1. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_NAME_CRC32}_${PRODUCT_SKU_CRC32}.bin`
2. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_SKU_CRC32}.bin`
3. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_NAME_CRC32}.bin`
4. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}.bin`
5. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_NAME_CRC32}_${PRODUCT_SKU_CRC32}.bin`
6. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_SKU_CRC32}.bin`
7. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_NAME_CRC32}.bin`
8. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}.bin`
9. `intel/ish/ish_${intel_plat_gen}.bin`

驱动将加载第一个匹配的固件并跳过其余的。如果未找到匹配的固件，它将按照
指定的顺序继续尝试下一种模式。如果所有搜索都失败，将加载上面顺序中列出的
最后的默认 Intel 固件

### ISH 调试


```

  echo 1 > /sys/kernel/tracing/events/intel_ish/enable
  cat /sys/kernel/tracing/trace

```

### ISH IIO sysfs Lenovo ThinkPad Yoga 260 上的示例


```

  root@otcpl-ThinkPad-Yoga-260:~# tree -l /sys/bus/iio/devices/
  /sys/bus/iio/devices/
  鈹溾攢鈹€ iio:device0 -> ../../../devices/0044:8086:22D8.0001/HID-SENSOR-200073.9.auto/iio:device0
  鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  ...
  鈹偮犅?鈹溾攢鈹€ in_accel_hysteresis
  鈹偮犅?鈹溾攢鈹€ in_accel_offset
  鈹偮犅?鈹溾攢鈹€ in_accel_sampling_frequency
  鈹偮犅?鈹溾攢鈹€ in_accel_scale
  鈹偮犅?鈹溾攢鈹€ in_accel_x_raw
  鈹偮犅?鈹溾攢鈹€ in_accel_y_raw
  鈹偮犅?鈹溾攢鈹€ in_accel_z_raw
  鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_x_en
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_x_index
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_x_type
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_y_en
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_y_index
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_y_type
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_z_en
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_z_index
  鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_accel_z_type
  ...
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ devices
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_both_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_both_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_both_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_intensity_both_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ trigger
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ current_trigger
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_magn_z_type
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_from_north_magnetic_tilt_comp_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_magn_z_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ trigger
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ current_trigger
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_anglvel_z_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ trigger
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ current_trigger
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_anglvel_z_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ trigger
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ current_trigger
  ...
```
