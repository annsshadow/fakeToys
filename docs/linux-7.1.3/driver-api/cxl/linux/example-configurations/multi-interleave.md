
## Multi-Level Interleave

鏈?cxl-cli 閰嶇疆杞偍灞曠ず浜嗕互涓嬩富鏈洪厤缃細

- 鍗曡矾绯荤粺锛屽惈涓€涓?CXL root
- CXL root 鏈夊洓涓紙4锛変釜 CXL 涓绘満妗ワ紙Host Bridge锛?- 鍏朵腑涓や釜 CXL 涓绘満妗ュ悇鎸傛帴浜嗕袱涓?CXL 鍐呭瓨鎵╁睍鍣紙Memory Expander锛夈€?- 璇?CXL root 琚厤缃负鍦ㄨ繖涓や釜涓绘満妗ヤ箣闂磋繘琛屼氦閿欙紙interleave锛夈€?- 姣忎釜甯︽湁鎵╁睍鍣ㄧ殑涓绘満妗ュ湪涓や釜绔偣锛坋ndpoint锛変箣闂磋繘琛屼氦閿欍€?
鏈緭鍑虹敱 `cxl list -v` 鐢熸垚锛屾弿杩颁簡 `/sys/bus/cxl/devices/` 涓毚闇茬殑瀵硅薄涔嬮棿鐨勫叧绯汇€?
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

姝や唬鐮佸潡鏄剧ず CXL "bus"锛坮oot0锛夋湁 4 涓笅娓哥鍙ｏ紙downstream port锛夛紝鎸傛帴鍦?CXL 涓绘満妗ヤ笂銆傝 `Root` 鍙涓鸿繛鎺ュ钩鍙板唴瀛樻帶鍒跺櫒鐨勫崟涓€涓婃父绔彛鈥斺€斿畠璐熻矗灏嗗唴瀛樿姹傝矾鐢卞埌鑷韩銆?
`ports:root0` 閮ㄥ垎璇存槑浜嗚繖浜涗笅娓哥鍙ｅ悇鑷槸濡備綍閰嶇疆鐨勩€傚鏋滄煇涓鍙ｆ湭琚厤缃紙id 涓?0 鍜?1锛夛紝鍒欎細琚渷鐣ャ€?
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

姝や唬鐮佸潡鏄剧ず涓?CXL 涓绘満妗?`port1` 鍏宠仈鐨勫彲鐢ㄤ笅娓哥鍙ｃ€傛湰渚嬩腑锛宍port1` 鏈?3 涓彲鐢ㄤ笅娓哥鍙ｏ細`dport0`銆乣dport2` 鍜?`dport113`銆?
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

姝や唬鐮佸潡鏄剧ず鎸傛帴鍦ㄤ富鏈烘ˉ `port1` 涓婄殑绔偣銆?
`endpoint5` 鍚湁涓€涓凡閰嶇疆鐨?decoder `decoder5.0`锛屽叾浜ら敊閰嶇疆涓?`region0` 鐩稿悓锛堢◢鍚庡睍绀猴級銆?
`endpoint6` 鍚湁涓€涓凡閰嶇疆鐨?decoder `decoder5.0`锛屽叾浜ら敊閰嶇疆涓?`region0` 鐩稿悓锛堢◢鍚庡睍绀猴級銆?
鎺ヤ笅鏉ユ槸褰掑睘璇ヤ富鏈烘ˉ鐨?decoder锛?
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

涓绘満妗?`port1` 鍚湁涓€涓?decoder锛坄decoder1.0`锛夛紝鍏?target 涓?`dport0` 鍜?`dport2`鈥斺€斿畠浠垎鍒寕鎺ュ湪 `endpoint5` 鍜?`endpoint6` 涓娿€?
浠ヤ笅浠ｇ爜鍧楀睍绀轰簡涓绘満妗?`port3` 鐨勭被浼奸厤缃紝杩欐槸绗簩涓寕鎺ヤ簡鍐呭瓨璁惧鐨勪富鏈烘ˉ銆?
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

涓嬩竴涓唬鐮佸潡灞曠ず浜嗕袱涓病鏈夋寕鎺ョ鐐圭殑 CXL 涓绘満妗ャ€?
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

鎺ヤ笅鏉ユ槸褰掑睘 `root0` 鐨?`Root Decoders`銆傝 root decoder 鍦ㄤ笅娓哥鍙?`port1` 鍜?`port3` 涓婂簲鐢ㄤ氦閿欌€斺€旂矑搴︿负 256 瀛楄妭銆?
璇ヤ俊鎭敱 CXL 椹卞姩璇诲彇 ACPI CEDT CMFWS 鐢熸垚銆?
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

鏈€鍚庢槸褰掑睘 `Root Decoder` `decoder0.0` 鐨?`Memory Region`銆傝 region 鎻忚堪浜嗕氦閿欓泦鍚堬紙interleave set锛夌殑鏁翠綋浜ら敊閰嶇疆銆傚洜姝ゆ垜浠湅鍒板湪 4 涓鐐?decoder 涓婃€诲叡鏈?`4` 涓氦閿?target銆?
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
