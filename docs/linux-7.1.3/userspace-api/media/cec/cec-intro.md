## 简介


HDMI 连接器为消费电子控制（CEC）协议提供单个引脚。该协议允许通过 HDMI
线缆连接的不同设备进行通信。CEC 1.4 版本的协议定义于 HDMI 1.4a（hdmi）
规范的补充 1（CEC）和补充 2（HEAC 或 HDMI 以太网与音频回传通道）中，而
添加到 CEC 2.0 版本的扩展定义于 HDMI 2.0（hdmi2）规范的第 11 章。

比特率非常慢（实际上每秒不超过 36 字节），并且基于老式 SCART 连接器中
使用的古老 AV.link 协议。该协议非常类似于一个疯狂的 Rube Goldberg 装置，
是低层与高层消息的怪异混合。某些消息，尤其是那些位于 CEC 之上的 HEAC
协议部分的消息，需要由内核处理，其他消息则可由内核或用户空间处理。

此外，CEC 可在 HDMI 接收器、发送器以及具有 HDMI 输入与 HDMI 输出且仅控制
CEC 引脚的 USB 设备中实现。

支持 CEC 的驱动将创建一个 CEC 设备节点（/dev/cecX）以使用户空间能够访问
CEC 适配器。CEC_ADAP_G_CAPS ioctl 将告知用户空间其允许执行的操作。

为检查并测试支持情况，建议下载 `v4l-utils <https://git.linuxtv.org/v4l-utils.git/>`_ 软件包。它提供三个用于处理 CEC 的工具：

- cec-ctl：CEC 的瑞士军刀。允许配置、发送和监控 CEC 消息。

- cec-compliance：对远程 CEC 设备执行 CEC 合规性测试，以确定 CEC 实现的
  合规程度。

- cec-follower：模拟一个 CEC follower。
