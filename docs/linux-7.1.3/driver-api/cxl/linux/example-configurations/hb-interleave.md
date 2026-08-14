
## 主机桥间交错（Inter-Host-Bridge Interleave）

这份 cxl-cli 配置转储展示了如下的主机配置：

- 一个单路（single socket）系统，带有一个 CXL root
- CXL Root 有四个（4）CXL Host Bridge（主机桥）
- 其中两个 CXL Host Bridge 各挂载了一个 CXL Memory Expander（内存扩展器）
- 该 CXL root 被配置为在两个主机桥之间进行交错（interleave）

该输出由 `cxl list -v` 生成，描述了在 `/sys/bus/cxl/devices/` 中暴露的各对象之间的
关系。

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
这段展示了 CXL “bus”（root0）有 4 个连接到 CXL Host Bridge 的下游端口（downstream port）。
`Root` 可以看作连接到平台内存控制器（memory controller）的单一上游端口（upstream port）——它
将内存请求路由到自身。

`ports:root0` 部分说明了这些下游端口各自是如何配置的。如果一个端口未被配置（id 为 0 和 1），
则会被省略。

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
这段展示了与 CXL Host Bridge `port1` 关联的可用下游端口。在本例中，`port1` 有 3 个可用的
下游端口：`dport1`、`dport2` 与 `dport113`..

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
                    }
                ],

```
这段展示了连接到主机桥 `port1` 的端点（endpoint）。

`endpoint5` 包含一个已配置的 decoder `decoder5.0`，其交错配置与 `region0`（稍后展示）相同。

接下来是归属于主机桥的 decoder（解码器）：

```

                "decoders:port1":[
                    {
                        "decoder":"decoder1.0",
                        "resource":825975898112,
                        "size":274877906944,
                        "interleave_ways":1,
                        "region":"region0",
                        "nr_targets":1,
                        "targets":[
                            {
                                "target":"0000:d2:01.1",
                                "alias":"device:02",
                                "position":0,
                                "id":0
                            }
                        ]
                    }
                ]
            },

```
主机桥 `port1` 有一个 decoder（`decoder1.0`），其唯一目标是 `dport1`——它连接到 `endpoint5`。

接下来的一段展示了主机桥 `port3` 的类似配置，即第二个挂载了内存设备的主机桥。

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
                    }
                ],
                "endpoints:port3":[
                    {
                        "endpoint":"endpoint6",
                        "host":"mem1",
                        "parent_dport":"0000:a8:01.1",
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
                "decoders:port3":[
                    {
                        "decoder":"decoder3.0",
                        "resource":825975898112,
                        "size":274877906944,
                        "interleave_ways":1,
                        "region":"region0",
                        "nr_targets":1,
                        "targets":[
                            {
                                "target":"0000:a8:01.1",
                                "alias":"device:c3",
                                "position":0,
                                "id":0
                            }
                        ]
                    }
                ]
            },


```
下一段展示了两个没有连接端点的 CXL 主机桥。

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
接下来是归属于 `root0` 的 `Root Decoders`（根解码器）。这个根解码器在下游端口 `port1` 与
`port3` 之间应用交错——粒度为 256 字节。

这些信息由 CXL 驱动读取 ACPI CEDT CFMWS 生成。

```

        "decoders:root0":[
            {
                "decoder":"decoder0.0",
                "resource":825975898112,
                "size":274877906944,
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
最后是关联到 `Root Decoder` `decoder0.0` 的 `Memory Region`（内存区域）。该区域描述了整个
交错集合（interleave set）的总体交错配置。

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
