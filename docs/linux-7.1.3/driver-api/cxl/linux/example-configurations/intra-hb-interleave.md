
## Intra-Host-Bridge Interleave

此 cxl-cli 配置转储显示了以下主机配置：

- 一个单插槽系统，带一个 CXL root
- CXL Root 有四个（4）个 CXL 主机桥
- 其中一个（1）CXL 主机桥挂载了两个 CXL 内存扩展器
- 主机桥的 decoder 被编程为在这些扩展器之间进行交错

该输出由 `cxl list -v` 生成，描述了 `/sys/bus/cxl/devices/` 中暴露的对象之间的关系。

```

  [
    {
        "bus":"root0",
        "provider":"ACPI.CXL",
        "nr_dports":4,
        "dports":[
            {
                "dport":"pci0000:00",
                "alias":"ACPI0016:01",
                "id":0
            },
            {
                "dport":"pci0000:a8",
                "alias":"ACPI0016:02",
                "id":4
            },
            {
                "dport":"pci0000:2a",
                "alias":"ACPI0016:03",
                "id":1
            },
            {
                "dport":"pci0000:d2",
                "alias":"ACPI0016:00",
                "id":5
            }
        ],

```
此代码块显示 CXL“bus”（root0）有 4 个下游端口挂载到 CXL 主机桥。可将 `Root` 视为挂载到平台
内存控制器（并由此将内存请求路由给它）的单一上游端口。

`ports:root0` 一节说明了这些下游端口各自是如何配置的。如果某个端口未配置（id 为 0 和 1），则
省略它们。

```

        "ports:root0":[
            {
                "port":"port1",
                "host":"pci0000:d2",
                "depth":1,
                "nr_dports":3,
                "dports":[
                    {
                        "dport":"0000:d2:01.1",
                        "alias":"device:02",
                        "id":0
                    },
                    {
                        "dport":"0000:d2:01.3",
                        "alias":"device:05",
                        "id":2
                    },
                    {
                        "dport":"0000:d2:07.1",
                        "alias":"device:0d",
                        "id":113
                    }
                ],

```
此代码块显示了与 CXL 主机桥 `port1` 关联的可用下游端口。在本例中，`port1` 有 3 个可用的下游
端口：`dport1`、`dport2` 和 `dport113`。

```

                "endpoints:port1":[
                    {
                        "endpoint":"endpoint5",
                        "host":"mem0",
                        "parent_dport":"0000:d2:01.1",
                        "depth":2,
                        "memdev":{
                            "memdev":"mem0",
                            "ram_size":137438953472,
                            "serial":0,
                            "numa_node":0,
                            "host":"0000:d3:00.0"
                        },
                        "decoders:endpoint5":[
                            {
                                "decoder":"decoder5.0",
                                "resource":825975898112,
                                "size":274877906944,
                                "interleave_ways":2,
                                "interleave_granularity":256,
                                "region":"region0",
                                "dpa_resource":0,
                                "dpa_size":137438953472,
                                "mode":"ram"
                            }
                        ]
                    },
                    {
                        "endpoint":"endpoint6",
                        "host":"mem1",
                        "parent_dport":"0000:d2:01.3,
                        "depth":2,
                        "memdev":{
                            "memdev":"mem1",
                            "ram_size":137438953472,
                            "serial":0,
                            "numa_node":0,
                            "host":"0000:a9:00.0"
                        },
                        "decoders:endpoint6":[
                            {
                                "decoder":"decoder6.0",
                                "resource":825975898112,
                                "size":274877906944,
                                "interleave_ways":2,
                                "interleave_granularity":256,
                                "region":"region0",
                                "dpa_resource":0,
                                "dpa_size":137438953472,
                                "mode":"ram"
                            }
                        ]
                    }
                ],

```
此代码块显示了挂载到主机桥 `port1` 的端点。

`endpoint5` 包含一个已配置的 decoder `decoder5.0`，它具有与其所属的内存区域相同的交错配置
（稍后显示）。

接下来我们看看属于主机桥的 decoder：

```

                "decoders:port1":[
                    {
                        "decoder":"decoder1.0",
                        "resource":825975898112,
                        "size":274877906944,
                        "interleave_ways":2,
                        "interleave_granularity":256,
                        "region":"region0",
                        "nr_targets":2,
                        "targets":[
                            {
                                "target":"0000:d2:01.1",
                                "alias":"device:02",
                                "position":0,
                                "id":0
                            },
                            {
                                "target":"0000:d2:01.3",
                                "alias":"device:05",
                                "position":1,
                                "id":0
                            }
                        ]
                    }
                ]
            },

```
主机桥 `port1` 有一个 decoder（`decoder1.0`），带两个目标：`dport1` 和 `dport3`——它们分别挂载
到 `endpoint5` 和 `endpoint6`。

主机桥的 decoder 以 256 字节的粒度在这些设备之间进行交错。

下一个代码块显示了三个没有挂载端点的 CXL 主机桥。

```

            {
                "port":"port2",
                "host":"pci0000:00",
                "depth":1,
                "nr_dports":2,
                "dports":[
                    {
                        "dport":"0000:00:01.3",
                        "alias":"device:55",
                        "id":2
                    },
                    {
                        "dport":"0000:00:07.1",
                        "alias":"device:5d",
                        "id":113
                    }
                ]
            },
            {
                "port":"port3",
                "host":"pci0000:a8",
                "depth":1,
                "nr_dports":1,
                "dports":[
                    {
                        "dport":"0000:a8:01.1",
                        "alias":"device:c3",
                        "id":0
                    }
                ],
            },
            {
                "port":"port4",
                "host":"pci0000:2a",
                "depth":1,
                "nr_dports":1,
                "dports":[
                    {
                        "dport":"0000:2a:01.1",
                        "alias":"device:d0",
                        "id":0
                    }
                ]
            }
        ],

```
接下来我们看看属于 `root0` 的 `Root Decoders`。该 root decoder 在下游端口 `port1` 和 `port3` 之间
应用交错——粒度为 256 字节。

该信息由 CXL 驱动读取 ACPI CEDT CMFWS 生成。

```

        "decoders:root0":[
            {
                "decoder":"decoder0.0",
                "resource":825975898112,
                "size":274877906944,
                "interleave_ways":1,
                "max_available_extent":0,
                "volatile_capable":true,
                "nr_targets":2,
                "targets":[
                    {
                        "target":"pci0000:a8",
                        "alias":"ACPI0016:02",
                        "position":1,
                        "id":4
                    },
                ],

```
最后我们看看与 `Root Decoder` `decoder0.0` 关联的 `Memory Region`。该区域描述了交错集的整体
交错配置。

```

                "regions:decoder0.0":[
                    {
                        "region":"region0",
                        "resource":825975898112,
                        "size":274877906944,
                        "type":"ram",
                        "interleave_ways":2,
                        "interleave_granularity":256,
                        "decode_state":"commit",
                        "mappings":[
                            {
                                "position":1,
                                "memdev":"mem1",
                                "decoder":"decoder6.0"
                            },
                            {
                                "position":0,
                                "memdev":"mem0",
                                "decoder":"decoder5.0"
                            }
                        ]
                    }
                ]
            }
        ]
    }
  ]

```