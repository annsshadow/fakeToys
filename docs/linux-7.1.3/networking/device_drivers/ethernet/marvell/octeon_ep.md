
## Linux 内核面向 Marvell Octeon PCI Endpoint NIC 的网络驱

Marvell Octeon PCI EndPoint NIC 的网络驱动。版(c) 2020 Marvell International Ltd.

## 目录


- `Overview`_
- `Supported Devices`_
- `Interface Control`_

## 概述

该驱动实Marvell Octeon PCI EndPoint NIC 的网络功能
## 支持的设
目前，该驱动支持以下设备 - 网络控制器：Cavium, Inc. Device b100
 - 网络控制器：Cavium, Inc. Device b200
 - 网络控制器：Cavium, Inc. Device b400
 - 网络控制器：Cavium, Inc. Device b900
 - 网络控制器：Cavium, Inc. Device ba00
 - 网络控制器：Cavium, Inc. Device bc00
 - 网络控制器：Cavium, Inc. Device bd00

## 接口控制

网络接口控制（如更改 mtu、链路速率、链down/up）通过将命令写入邮箱命令队完成，该邮箱接口通过 BAR4 中的保留区域实现。该驱动将命令写入邮箱，Octeon 设备
上的固件处理它们。固件还通过作为邮箱接口一部分实现的通知队列，向驱动发送链变化等事件的非请求通知