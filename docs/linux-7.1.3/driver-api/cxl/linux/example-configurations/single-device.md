
## 单一设备


这个 cxl-cli 配置转储显示了以下主机配置：

- 一个单插槽系统，带有一个 CXL 根
- CXL 根有四个（4）CXL 主机桥
- 其中一个 CXL 主机桥挂接了一个单独的 CXL 内存扩展器
- 不存在交错（interleave）。

该输出由 `cxl list -v` 生成，描述了在 `/sys/bus/cxl/devices/` 中暴露的对象
之间的关系。

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
这一段显示 CXL “bus”（root0）有 4 个挂接到 CXL 主机桥的下游端口。`Root` 可视为
挂接到平台内存控制器的唯一上游端口——它负责将内存请求路由到该控制器。

`ports:root0` 小节列出了这些下游端口各自的配置方式。如果某个端口没有配置
（id 为 0、1 和 4），则将其省略。

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
这一段显示了与 CXL 主机桥 `port1` 关联的可用下游端口。在本例中，`port1` 有 3
个可用的下游端口：`dport1`、`dport2` 和 `dport113`。。

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
                                "size":137438953472,
                                "interleave_ways":1,
                                "region":"region0",
                                "dpa_resource":0,
                                "dpa_size":137438953472,
                                "mode":"ram"
                            }
                        ]
                    }
                ],

```
这一段显示了挂接到主机桥 `port1` 的端点。

`endpoint5` 包含一个已配置的单个解码器 `decoder5.0`，其交错配置与 `region0`
（稍后显示）相同。

接下来是归属于该主机桥的解码器：

```

                "decoders:port1":[
                    {
                        "decoder":"decoder1.0",
                        "resource":825975898112,
                        "size":137438953472,
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
主机桥 `port1` 有一个单独的解码器（`decoder1.0`），其唯一目标是 `dport1`——它
挂接到了 `endpoint5`。

下一段显示了三个没有挂接端点的 CXL 主机桥。

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
接下来是归属于 `root0` 的 `Root Decoders`。这个根解码器是一个直通解码器，因为
`interleave_ways` 被设为 `1`。

这些信息由 CXL 驱动读取 ACPI CEDT CMFWS 生成。

```

        "decoders:root0":[
            {
                "decoder":"decoder0.0",
                "resource":825975898112,
                "size":137438953472,
                "interleave_ways":1,
                "max_available_extent":0,
                "volatile_capable":true,
                "nr_targets":1,
                "targets":[
                    {
                        "target":"pci0000:d2",
                        "alias":"ACPI0016:00",
                        "position":0,
                        "id":5
                    }
                ],

```
最后是与 `Root Decoder` `decoder0.0` 关联的 `Memory Region`。该区域描述了与
这个唯一设备关联的离散区域。

```

                "regions:decoder0.0":[
                    {
                        "region":"region0",
                        "resource":825975898112,
                        "size":137438953472,
                        "type":"ram",
                        "interleave_ways":1,
                        "decode_state":"commit",
                        "mappings":[
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
