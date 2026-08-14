
## Compute Express Link 驱动操作原理

一个 Compute Express Link 内存设备（CXL Memory Device）是实现 CXL.mem 协议的 CXL 组件。它
包含一定数量的易失性内存、持久化内存，或两者兼有。它被枚举为一个 PCI 设备，用于配置以及通过
MMIO 邮箱传递消息。它对系统物理地址空间（System Physical Address）的贡献通过 HDM（Host
Managed Device Memory，主机管理设备内存）解码器来处理，这些解码器可选地定义设备在主机桥之下
多个设备之间交错（interleaved）地址范围的贡献，或跨主机桥交错。

## CXL 总线

类似于 RAID 驱动将磁盘对象组装成新的逻辑设备，CXL 子系统负责将 PCIe 和 ACPI 对象组装成
CXL.mem 解码拓扑。CXL.mem 拓扑需要运行时配置，这一点也类似于 RAID：具有相同硬件配置的不同
环境可能会决定以截然不同的方式组装拓扑。一个环境可能为了性能（RAID0）而将内存跨多个主机桥和
端点做条带化（striping），而另一个环境可能为了容错而禁用 CXL.mem 拓扑中的任何条带化。

平台固件在“CXL 根端口”（Linux 对 CXL 解码拓扑顶层的术语）处枚举出一份交错选项菜单。从那里开始，
PCIe 拓扑决定了哪些端点可以参与哪些主机桥解码机制。根与端点之间路径上的每个 PCIe 交换机都引入
了一个可以将交错拆分的点。例如，平台固件可能说某个给定范围只解码到某一个主机桥，但该主机桥
反过来又可能跨多个根端口交错周期。端口与端点之间的中间交换机可能跨多个下游交换机端口交错周期，
等等。

下面是一个由 'cxl_test' 定义的 CXL 拓扑示例。'cxl_test' 模块生成一个模拟的 CXL 拓扑，包含 2 个
主机桥，每个主机桥各有 2 个根端口。这些根端口各自连接到带有端点的双路交换机：

```
    # cxl list -BEMPu -b cxl_test
    {
      "bus":"root3",
      "provider":"cxl_test",
      "ports:root3":[
        {
          "port":"port5",
          "host":"cxl_host_bridge.1",
          "ports:port5":[
            {
              "port":"port8",
              "host":"cxl_switch_uport.1",
              "endpoints:port8":[
                {
                  "endpoint":"endpoint9",
                  "host":"mem2",
                  "memdev":{
                    "memdev":"mem2",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x1",
                    "numa_node":1,
                    "host":"cxl_mem.1"
                  }
                },
                {
                  "endpoint":"endpoint15",
                  "host":"mem6",
                  "memdev":{
                    "memdev":"mem6",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x5",
                    "numa_node":1,
                    "host":"cxl_mem.5"
                  }
                }
              ]
            },
            {
              "port":"port12",
              "host":"cxl_switch_uport.3",
              "endpoints:port12":[
                {
                  "endpoint":"endpoint17",
                  "host":"mem8",
                  "memdev":{
                    "memdev":"mem8",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x7",
                    "numa_node":1,
                    "host":"cxl_mem.7"
                  }
                },
                {
                  "endpoint":"endpoint13",
                  "host":"mem4",
                  "memdev":{
                    "memdev":"mem4",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x3",
                    "numa_node":1,
                    "host":"cxl_mem.3"
                  }
                }
              ]
            }
          ]
        },
        {
          "port":"port4",
          "host":"cxl_host_bridge.0",
          "ports:port4":[
            {
              "port":"port6",
              "host":"cxl_switch_uport.0",
              "endpoints:port6":[
                {
                  "endpoint":"endpoint7",
                  "host":"mem1",
                  "memdev":{
                    "memdev":"mem1",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0",
                    "numa_node":0,
                    "host":"cxl_mem.0"
                  }
                },
                {
                  "endpoint":"endpoint14",
                  "host":"mem5",
                  "memdev":{
                    "memdev":"mem5",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x4",
                    "numa_node":0,
                    "host":"cxl_mem.4"
                  }
                }
              ]
            },
            {
              "port":"port10",
              "host":"cxl_switch_uport.2",
              "endpoints:port10":[
                {
                  "endpoint":"endpoint16",
                  "host":"mem7",
                  "memdev":{
                    "memdev":"mem7",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x6",
                    "numa_node":0,
                    "host":"cxl_mem.6"
                  }
                },
                {
                  "endpoint":"endpoint11",
                  "host":"mem3",
                  "memdev":{
                    "memdev":"mem3",
                    "pmem_size":"256.00 MiB (268.44 MB)",
                    "ram_size":"256.00 MiB (268.44 MB)",
                    "serial":"0x2",
                    "numa_node":0,
                    "host":"cxl_mem.2"
                  }
                }
              ]
            }
          ]
        }
      ]
    }

```
在该列表中，每个 "root"、"port" 和 "endpoint" 对象都对应一个内核 'struct cxl_port' 对象。
'cxl_port' 是一个能够将其后代解码为 CXL.mem 的设备。因此 "root" 声明非 PCIe 可枚举的平台解码
范围，并将它们解码到 "ports"；"ports" 解码到 "endpoints"；而 "endpoints" 表示从 SPA（系统物理
地址）到 DPA（设备物理地址）的解码。

延续 RAID 的类比，磁盘既有决定 RAID 集合组装的拓扑元数据，也有设备上的元数据。CXL 端口拓扑和
CXL 端口链路状态就是 CXL.mem 集合组装的元数据。CXL 端口拓扑由 CXL.mem 设备的到达而枚举。即，
除非且直到 PCIe 核心将 cxl_pci 驱动绑定到一个 CXL 内存扩展器（Memory Expander），否则 CXL 端口
对象没有用武之地。反之，对于热拔插/移除场景，Linux PCI 核心不需要拆除交换机级别的 CXL 资源，
因为 endpoint 的 ->remove() 事件会清理为支持该内存扩展器而建立的端口数据。

给定内存设备可能拥有的端口元数据和潜在解码方案：

```
    # cxl list -BDMu -d root -m mem3
    {
      "bus":"root3",
      "provider":"cxl_test",
      "decoders:root3":[
        {
          "decoder":"decoder3.1",
          "resource":"0x8030000000",
          "size":"512.00 MiB (536.87 MB)",
          "volatile_capable":true,
          "nr_targets":2
        },
        {
          "decoder":"decoder3.3",
          "resource":"0x8060000000",
          "size":"512.00 MiB (536.87 MB)",
          "pmem_capable":true,
          "nr_targets":2
        },
        {
          "decoder":"decoder3.0",
          "resource":"0x8020000000",
          "size":"256.00 MiB (268.44 MB)",
          "volatile_capable":true,
          "nr_targets":1
        },
        {
          "decoder":"decoder3.2",
          "resource":"0x8050000000",
          "size":"256.00 MiB (268.44 MB)",
          "pmem_capable":true,
          "nr_targets":1
        }
      ],
      "memdevs:root3":[
        {
          "memdev":"mem3",
          "pmem_size":"256.00 MiB (268.44 MB)",
          "ram_size":"256.00 MiB (268.44 MB)",
          "serial":"0x2",
          "numa_node":0,
          "host":"cxl_mem.2"
        }
      ]
    }

```
……该命令查询 CXL 拓扑以询问“给定一个内核设备名为 'mem3' 的 CXL 内存扩展器，该设备可以参与哪些
平台级解码范围”。根据给定的扩展器拥有多少个解码器资源，它可以同时参与多个 CXL.mem 交错集合。
在此示例中，mem3 可以参与以下一个或多个：跨两个主机桥的 PMEM 交错、面向单个主机桥的 PMEM 交错、
跨 2 个主机桥的易失性内存交错，以及仅面向单个主机桥的易失性内存交错。

反之，可以参与给定平台级解码范围的内存设备：

```
    # cxl list -MDu -d 3.2
    [
      {
        "memdevs":[
          {
            "memdev":"mem1",
            "pmem_size":"256.00 MiB (268.44 MB)",
            "ram_size":"256.00 MiB (268.44 MB)",
            "serial":"0",
            "numa_node":0,
            "host":"cxl_mem.0"
          },
          {
            "memdev":"mem5",
            "pmem_size":"256.00 MiB (268.44 MB)",
            "ram_size":"256.00 MiB (268.44 MB)",
            "serial":"0x4",
            "numa_node":0,
            "host":"cxl_mem.4"
          },
          {
            "memdev":"mem7",
            "pmem_size":"256.00 MiB (268.44 MB)",
            "ram_size":"256.00 MiB (268.44 MB)",
            "serial":"0x6",
            "numa_node":0,
            "host":"cxl_mem.6"
          },
          {
            "memdev":"mem3",
            "pmem_size":"256.00 MiB (268.44 MB)",
            "ram_size":"256.00 MiB (268.44 MB)",
            "serial":"0x2",
            "numa_node":0,
            "host":"cxl_mem.2"
          }
        ]
      },
      {
        "root decoders":[
          {
            "decoder":"decoder3.2",
            "resource":"0x8050000000",
            "size":"256.00 MiB (268.44 MB)",
            "pmem_capable":true,
            "nr_targets":1
          }
        ]
      }
    ]

```
……其中解码器的命名方案为 "decoder<port_id>.<instance_id>"。

## 驱动基础设施

本节介绍 CXL 内存设备的驱动基础设施。

### CXL 内存设备

   :doc: cxl pci

   :internal:

   :doc: cxl mem

   :internal:

   :identifiers:

### CXL 端口

   :doc: cxl port

### CXL 核心

   :doc: cxl objects

   :internal:

   :identifiers: add_cxl_resources

   :doc: cxl core hdm

   :identifiers:

   :identifiers:

   :doc: cxl core

   :identifiers:

   :doc: cxl core pci

   :identifiers:

   :doc: cxl pmem

   :identifiers:

   :doc: cxl registers

   :identifiers:

   :doc: cxl mbox

   :identifiers:

   :doc: cxl features

API 详情参见 `devm_cxl_setup_features`。

### CXL 区域

   :doc: cxl core region

   :identifiers:

## 外部接口

### CXL IOCTL 接口

   :doc: UAPI

   :internal:
