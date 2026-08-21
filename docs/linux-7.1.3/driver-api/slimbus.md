## Linux 内核 SLIMbus 支持


## 概述


### 什么是 SLIMbus

SLIMbus（Serial Low Power Interchip Media Bus，串行低功耗片间媒体总线）是MIPI（Mobile Industry Processor Interface，移动产业处理器接口）联盟制定的规范该总线采用从（master/slave）配置，是一2 线多分支（multi-drop）实（时钟线与数据线）
目前，SLIMbus 用于SoC（System-on-Chip，片上系统）应用处理器与外设组件（通常是编解码codec）之间进行接口连接。SLIMbus 使用
时分复用（Time-Division-Multiplexing）来容纳多个数据通道以及一个控制通道
控制通道用于各种控制功能，例如总线
管理、配置以及状态更新。这些消息可以是单播（例读取/写入设备特定值），也可以是组播（例如数据通道
重配置序列是向所有设备广播的消息）
数据通道用于2 SLIMbus 设备之间传输数据。数通道使用设备上的专用端口
### 硬件描述

SLIMbus 规范根据其能力对设备进行了不同的分类
管理器（manager）设备负责枚举、配置以及动通道分配。每条总线1 个活动的管理器
通用（generic）设备是提供应用功能的设备（例如编解码器 codec）
成帧器（Framer）设备负责为总线提供时钟，并在总线上传输帧同步
与成帧信息
每个 SLIMbus 组件都有一个接口（interface）设备用于监控物理层
通常每个 SoC 包含一SLIMbus 组件，其具有 1 个管理器 个成帧器设备1 个通用设备（用于数据通道支持）以1 个接口设备
外部外设 SLIMbus 组件通常具有 1 个通用设备（用功能/数据通道支持）以及一个关联的接口设备
通用设备的寄存器被映射为“值元素”（value elements），以便可以
使用 SLIMbus 控制通道交换控制/状态类型的信息
进行写入/读取
如果同一条总线上存在多个成帧器设备，则由管理器设备负责
选择用于为总线提供时钟的活动成帧器（active-framer）
按照规范，SLIMbus 使用“时钟齿轮”（clock gears）来根据
当前的频率和带宽需求进行电源管理。共10 个时钟齿轮，
每个齿轮SLIMbus 频率改变为其前一个齿轮的两倍
每个设备都有一6 字节的枚举地址（enumeration-address），当设在总线上报告存在后，管理器会为每个设备分配一1 字节的逻辑地址（logical address）
### 软件描述

SLIMbus 椹卞姩鏈?2 绉嶇被鍨嬶細

slim_controller 表示 SLIMbus 的一个“控制器”（controller）。该驱动实现 SoC 所需的职责（管理器设备、用于监控各层并报告错误的关接口设备、默认成帧器设备）
slim_device 表示 SLIMbus 的“通用设备/组件”（generic device/component），
slim_driver 应实现针对该 slim_device 的驱动
### 向驱动的设备通知

由于 SLIMbus 设备具有报告其存在状态的机制框架允许驱动在相应设备在总线上报告其
存在时进行绑定
然而，也可能存在驱动需要先被探测（probe以便它能够启用相应的 SLIMbus 设备（例如为其上电和/使其退出复位状态）的情况。为了支持这种行为，框架也允许驱先进行探测（例如使用标准DeviceTree 兼容性字段）。这便产生了
驱动需要知道设备何时进入可用状态（即已报告存在）的需求出于这个原因，当设备报告存在并被控制器分配了逻辑地址时，
会使device_up 回调
类似地，SLIMbus 设备下线时会“报absent”。当设备
报告 absent 且其逻辑地址分配被控制器置为无效时，
会使device_down 回调通知驱动
另一个通知“boot_device”用于在控制器重置总线时通知
slim_driver。该通知允许驱动采取必要步骤
启动设备，使其在总线重置后能够正常工作
### 驱动与控制器 API

   :internal:

   :internal:

   :export:

### 时钟暂停（Clock-pause）：


SLIMbus 规定，在总线进入低功耗模式之前，必须向总线上所活动设备广播一个重配置序列（称clock-pause）。控制器在决进入低功耗模式时使用该序列，以便可以关闭相应的时钟和/电源轨以节省功耗。Clock-pause 的退出通过唤醒成帧器设（如果由控制器驱动发起退出低功耗模式），或切换数据（如果由从设备想要发起）来实现
#### Clock-pause API锛。

   :export:

### 消息传递（Messaging）：


框架支持 regmap 以及API，以便与 SLIMbus 设备
交换控制信息。API 可以是同步的或异步的头文<linux/slimbus.h> 中有关于消息传API 的更多文档
#### 消息传API

   :export:

#### 流式（Streaming）API

   :export:
