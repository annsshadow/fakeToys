
## Coresight 虚拟跟踪模块


    :Author:   Hao Zhang <quic_hazha@quicinc.com>
    :Date:     June 2023

### 简介


Coresight 虚拟跟踪模块适用于内核无权访问或配置的特定设备，例如
Qualcomm 平台上的 CoreSight TPDM。对于这些设备，需要一个虚拟驱动程序
将它们注册为 Coresight 设备。该模块也可用于定义可能没有任何编程接口的
组件，从而可以在驱动程序中创建路径。它为虚拟设备上的操作提供 Coresight
API，例如启用和禁用它们。它还提供用于调试的 Coresight 虚拟 sink/source
路径。

### 配置详情


有两种类型的节点：虚拟 sink 和虚拟 source。这些节点位于
`/sys/bus/coresight/devices`。

```

    $ ls -l /sys/bus/coresight/devices | grep dummy
    dummy_sink0 -> ../../../devices/platform/soc@0/soc@0:sink/dummy_sink0
    dummy_source0 -> ../../../devices/platform/soc@0/soc@0:source/dummy_source0

```
