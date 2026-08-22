## 閫氱敤绯荤粺浜掕繛瀛愮郴缁。


### 简


该框架旨在提供一个标准的内核接口，用于控SoC 上互连（interconnect）的
设置。这些设置可以是多个互连设备或功能块之间的吞吐量、延迟和优先级。可
动态控制这些设置，以节省功耗或提供最大性能

互连总线是一种具有可配置参数的硬件，可根据从各个驱动收到的请求在数据路径
进行设置。互连总线的一个例子是芯片组中各个组件或功能块之间的互连。一SoC
上可以存在多条互连，并且可以是多层的

下面是一张真SoC 互连总线拓扑的简化示意图
```

 +----------------+    +----------------+
 | HW Accelerator |--->|      M NoC     |<---------------+
 +----------------+    +----------------+                |
                         |      |                    +------------+
  +-----+  +-------------+      V       +------+     |            |
  | DDR |  |                +--------+  | PCIe |     |            |
  +-----+  |                | Slaves |  +------+     |            |
    ^ ^    |                +--------+     |         |   C NoC    |
    | |    V                               V         |            |
 +------------------+   +------------------------+   |            |   +-----+
 |                  |-->|                        |-->|            |-->| CPU |
 |                  |-->|                        |<--|            |   +-----+
 |     Mem NoC      |   |         S NoC          |   +------------+
 |                  |<--|                        |---------+    |
 |                  |<--|                        |<------+ |    |   +--------+
 +------------------+   +------------------------+       | |    +-->| Slaves |
   ^  ^    ^    ^          ^                             | |        +--------+
   |  |    |    |          |                             | V
 +------+  |  +-----+   +-----+  +---------+   +----------------+   +--------+
 | CPUs |  |  | GPU |   | DSP |  | Masters |-->|       P NoC    |-->| Slaves |
 +------+  |  +-----+   +-----+  +---------+   +----------------+   +--------+
           |
       +-------+
       | Modem |
       +-------+

```
### 术语


互连提供者（Interconnect provider）是互连硬件的软件定义。上图中的互连提供
M NoC、S NoC、C NoC、P NoC Mem NoC

互连节点（Interconnect node）是互连硬件端口的软件定义。每个互连提供者由多个
互连节点组成，这些节点连接到其他 SoC 组件，包括其他互连提供者。图CPU 连接
到内存的点称为互连节点，它属Mem NoC 互连提供者

互连端点（Interconnect endpoints）是路径的第一个或最后一个元素。每个端点都
一个节点，但并非每个节点都是端点

互连路径（Interconnect path）是两个端点之间的所有内容，包括从源节点到达目的
节点所必须遍历的所有节点。它可以包含跨越多个互连提供者的多对主从（master-slave
组合

互连使用者（Interconnect consumers）是利用提供者所暴露的数据路径的实体。使用
向提供者发送请求，要求各种不同的吞吐量、延迟和优先级。通常使用者是设备驱动
它们根据自身需求发送请求。使用者的一个例子是支持多种格式和图像尺寸的视频解码器

### 浜掕繛鎻愪緵鑰。


互连提供者是实现初始化和配置互连总线硬件方法的实体。互连提供者驱动应当向互连
提供者核心（interconnect provider core）注册


   :functions: icc_provider_init icc_provider_register icc_provider_deregister
               icc_node_create icc_node_create_dyn icc_node_destroy
               icc_node_add icc_node_del icc_nodes_remove icc_node_set_name
               icc_link_create icc_link_nodes

### 互连使用


互连使用者是使用互连 API 来获取端点之间路径、并为这些互连路径设置其
带宽/延迟/QoS 要求的客户端

   :functions: devm_of_icc_get of_icc_get_by_index of_icc_get icc_get
               icc_put icc_enable icc_disable icc_set_bw icc_set_tag
               icc_get_name


### 互连 debugfs 接口


与其他一些子系统类似，互连也会创建一些用于调试和内省的文件。debugfs 中的文件
不被视为 ABI，因此应用程序不应依赖各内核版本之间格式细节的变化

`/sys/kernel/debug/interconnect/interconnect_summary`锛。

显示系统中所有互连节点及其聚合的带宽请求。在每个节点下方，缩进显示各设备发出
带宽请求

`/sys/kernel/debug/interconnect/interconnect_graph`锛。

graphviz dot 格式显示互连图。它显示系统中所有的互连节点和链接，并将来自
同一提供者的节点归为子图。该格式具有人类可读性，也可以管道（pipe）输
```

        $ cat /sys/kernel/debug/interconnect/interconnect_graph | \
                dot -Tsvg > interconnect_graph.svg

```
`test-client` 目录提供了向任意路径发出带宽（BW）请求的接口。请注意，出于安
原因，该特性默认是禁用的，且没有用于启用它Kconfig。启用它需要修改代
```

        cd /sys/kernel/debug/interconnect/test-client/

        # Configure node endpoints for the path from CPU to DDR on
        # qcom/sm8550.
        echo chm_apps > src_node
        echo ebi > dst_node

        # Get path between src_node and dst_node. This is only
        # necessary after updating the node endpoints.
        echo 1 > get

        # Set desired BW to 1GBps avg and 2GBps peak.
        echo 1000000 > avg_bw
        echo 2000000 > peak_bw

        # Vote for avg_bw and peak_bw on the latest path from "get".
        # Voting for multiple paths is possible by repeating this
        # process for different nodes endpoints.
        echo 1 > commit

```