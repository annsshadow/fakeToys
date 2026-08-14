
## Multi-Level Interleave

本 cxl-cli 配置转储展示了以下主机配置：

- 单路系统，含一个 CXL root
- CXL root 有四个（4）个 CXL 主机桥（Host Bridge）
- 其中两个 CXL 主机桥各挂接了两个 CXL 内存扩展器（Memory Expander）。
- 该 CXL root 被配置为在这两个主机桥之间进行交错（interleave）。
- 每个带有扩展器的主机桥在两个端点（endpoint）之间进行交错。

本输出由 `cxl list -v` 生成，描述了 `/sys/bus/cxl/devices/` 中暴露的对象之间的关系。

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

此代码块显示 CXL "bus"（root0）有 4 个下游端口（downstream port），挂接在 CXL 主机桥上。该 `Root` 可视为连接平台内存控制器的单一上游端口——它负责将内存请求路由到自身。

`ports:root0` 部分说明了这些下游端口各自是如何配置的。如果某个端口未被配置（id 为 0 和 1），则会被省略。

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

此代码块显示与 CXL 主机桥 `port1` 关联的可用下游端口。本例中，`port1` 有 3 个可用下游端口：`dport0`、`dport2` 和 `dport113`。

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
                                "size":549755813888,
                                "interleave_ways":4,
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
                        "parent_dport":"0000:d2:01.3",
                        "depth":2,
                        "memdev":{
                            "memdev":"mem1",
                            "ram_size":137438953472,
                            "serial":0,
                            "numa_node":0,
                            "host":"0000:d3:00.0"
                        },
                        "decoders:endpoint6":[
                            {
                                "decoder":"decoder6.0",
                                "resource":825975898112,
                                "size":549755813888,
                                "interleave_ways":4,
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

此代码块显示挂接在主机桥 `port1` 上的端点。

`endpoint5` 含有一个已配置的 decoder `decoder5.0`，其交错配置与 `region0` 相同（稍后展示）。

`endpoint6` 含有一个已配置的 decoder `decoder5.0`，其交错配置与 `region0` 相同（稍后展示）。

接下来是归属该主机桥的 decoder：

```

                "decoders:port1":[
                    {
                        "decoder":"decoder1.0",
                        "resource":825975898112,
                        "size":549755813888,
                        "interleave_ways":2,
                        "interleave_granularity":512,
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
                                "position":2,
                                "id":0
                            }
                        ]
                    }
                ]
            },

```

主机桥 `port1` 含有一个 decoder（`decoder1.0`），其 target 为 `dport0` 和 `dport2`——它们分别挂接在 `endpoint5` 和 `endpoint6` 上。

以下代码块展示了主机桥 `port3` 的类似配置，这是第二个挂接了内存设备的主机桥。

```

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
                    },
                    {
                        "dport":"0000:a8:01.3",
                        "alias":"device:c5",
                        "id":0
                    }
                ],
                "endpoints:port3":[
                    {
                        "endpoint":"endpoint7",
                        "host":"mem2",
                        "parent_dport":"0000:a8:01.1",
                        "depth":2,
                        "memdev":{
                            "memdev":"mem2",
                            "ram_size":137438953472,
                            "serial":0,
                            "numa_node":0,
                            "host":"0000:a9:00.0"
                        },
                        "decoders:endpoint7":[
                            {
                                "decoder":"decoder7.0",
                                "resource":825975898112,
                                "size":549755813888,
                                "interleave_ways":4,
                                "interleave_granularity":256,
                                "region":"region0",
                                "dpa_resource":0,
                                "dpa_size":137438953472,
                                "mode":"ram"
                            }
                        ]
                    },
                    {
                        "endpoint":"endpoint8",
                        "host":"mem3",
                        "parent_dport":"0000:a8:01.3",
                        "depth":2,
                        "memdev":{
                            "memdev":"mem3",
                            "ram_size":137438953472,
                            "serial":0,
                            "numa_node":0,
                            "host":"0000:a9:00.0"
                        },
                        "decoders:endpoint8":[
                            {
                                "decoder":"decoder8.0",
                                "resource":825975898112,
                                "size":549755813888,
                                "interleave_ways":4,
                                "interleave_granularity":256,
                                "region":"region0",
                                "dpa_resource":0,
                                "dpa_size":137438953472,
                                "mode":"ram"
                            }
                        ]
                    }
                ],
                "decoders:port3":[
                    {
                        "decoder":"decoder3.0",
                        "resource":825975898112,
                        "size":549755813888,
                        "interleave_ways":2,
                        "interleave_granularity":512,
                        "region":"region0",
                        "nr_targets":1,
                        "targets":[
                            {
                                "target":"0000:a8:01.1",
                                "alias":"device:c3",
                                "position":1,
                                "id":0
                            },
                            {
                                "target":"0000:a8:01.3",
                                "alias":"device:c5",
                                "position":3,
                                "id":0
                            }
                        ]
                    }
                ]
            },



```

下一个代码块展示了两个没有挂接端点的 CXL 主机桥。

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

接下来是归属 `root0` 的 `Root Decoders`。该 root decoder 在下游端口 `port1` 和 `port3` 上应用交错——粒度为 256 字节。

该信息由 CXL 驱动读取 ACPI CEDT CMFWS 生成。

```

        "decoders:root0":[
            {
                "decoder":"decoder0.0",
                "resource":825975898112,
                "size":549755813888,
                "interleave_ways":2,
                "interleave_granularity":256,
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
                    {
                        "target":"pci0000:d2",
                        "alias":"ACPI0016:00",
                        "position":0,
                        "id":5
                    }
                ],

```

最后是归属 `Root Decoder` `decoder0.0` 的 `Memory Region`。该 region 描述了交错集合（interleave set）的整体交错配置。因此我们看到在 4 个端点 decoder 上总共有 `4` 个交错 target。

```

                "regions:decoder0.0":[
                    {
                        "region":"region0",
                        "resource":825975898112,
                        "size":549755813888,
                        "type":"ram",
                        "interleave_ways":4,
                        "interleave_granularity":256,
                        "decode_state":"commit",
                        "mappings":[
                            {
                                "position":3,
                                "memdev":"mem3",
                                "decoder":"decoder8.0"
                            },
                            {
                                "position":2,
                                "memdev":"mem1",
                                "decoder":"decoder6.0"
                            }
                            {
                                "position":1,
                                "memdev":"mem2",
                                "decoder":"decoder7.0"
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
