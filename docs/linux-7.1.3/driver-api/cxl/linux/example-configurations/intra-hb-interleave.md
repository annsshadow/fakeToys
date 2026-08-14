
## Intra-Host-Bridge Interleave

姝?cxl-cli 閰嶇疆杞偍鏄剧ず浜嗕互涓嬩富鏈洪厤缃細

- 涓€涓崟鎻掓Ы绯荤粺锛屽甫涓€涓?CXL root
- CXL Root 鏈夊洓涓紙4锛変釜 CXL 涓绘満妗?
- 鍏朵腑涓€涓紙1锛塁XL 涓绘満妗ユ寕杞戒簡涓や釜 CXL 鍐呭瓨鎵╁睍鍣?
- 涓绘満妗ョ殑 decoder 琚紪绋嬩负鍦ㄨ繖浜涙墿灞曞櫒涔嬮棿杩涜浜ら敊

璇ヨ緭鍑虹敱 `cxl list -v` 鐢熸垚锛屾弿杩颁簡 `/sys/bus/cxl/devices/` 涓毚闇茬殑瀵硅薄涔嬮棿鐨勫叧绯汇€?

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
姝や唬鐮佸潡鏄剧ず CXL鈥渂us鈥濓紙root0锛夋湁 4 涓笅娓哥鍙ｆ寕杞藉埌 CXL 涓绘満妗ャ€傚彲灏?`Root` 瑙嗕负鎸傝浇鍒板钩鍙?
鍐呭瓨鎺у埗鍣紙骞剁敱姝ゅ皢鍐呭瓨璇锋眰璺敱缁欏畠锛夌殑鍗曚竴涓婃父绔彛銆?

`ports:root0` 涓€鑺傝鏄庝簡杩欎簺涓嬫父绔彛鍚勮嚜鏄浣曢厤缃殑銆傚鏋滄煇涓鍙ｆ湭閰嶇疆锛坕d 涓?0 鍜?1锛夛紝鍒?
鐪佺暐瀹冧滑銆?

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
姝や唬鐮佸潡鏄剧ず浜嗕笌 CXL 涓绘満妗?`port1` 鍏宠仈鐨勫彲鐢ㄤ笅娓哥鍙ｃ€傚湪鏈緥涓紝`port1` 鏈?3 涓彲鐢ㄧ殑涓嬫父
绔彛锛歚dport1`銆乣dport2` 鍜?`dport113`銆?

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
姝や唬鐮佸潡鏄剧ず浜嗘寕杞藉埌涓绘満妗?`port1` 鐨勭鐐广€?

`endpoint5` 鍖呭惈涓€涓凡閰嶇疆鐨?decoder `decoder5.0`锛屽畠鍏锋湁涓庡叾鎵€灞炵殑鍐呭瓨鍖哄煙鐩稿悓鐨勪氦閿欓厤缃?
锛堢◢鍚庢樉绀猴級銆?

鎺ヤ笅鏉ユ垜浠湅鐪嬪睘浜庝富鏈烘ˉ鐨?decoder锛?

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
涓绘満妗?`port1` 鏈変竴涓?decoder锛坄decoder1.0`锛夛紝甯︿袱涓洰鏍囷細`dport1` 鍜?`dport3`鈥斺€斿畠浠垎鍒寕杞?
鍒?`endpoint5` 鍜?`endpoint6`銆?

涓绘満妗ョ殑 decoder 浠?256 瀛楄妭鐨勭矑搴﹀湪杩欎簺璁惧涔嬮棿杩涜浜ら敊銆?

涓嬩竴涓唬鐮佸潡鏄剧ず浜嗕笁涓病鏈夋寕杞界鐐圭殑 CXL 涓绘満妗ャ€?

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
鎺ヤ笅鏉ユ垜浠湅鐪嬪睘浜?`root0` 鐨?`Root Decoders`銆傝 root decoder 鍦ㄤ笅娓哥鍙?`port1` 鍜?`port3` 涔嬮棿
搴旂敤浜ら敊鈥斺€旂矑搴︿负 256 瀛楄妭銆?

璇ヤ俊鎭敱 CXL 椹卞姩璇诲彇 ACPI CEDT CMFWS 鐢熸垚銆?

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
鏈€鍚庢垜浠湅鐪嬩笌 `Root Decoder` `decoder0.0` 鍏宠仈鐨?`Memory Region`銆傝鍖哄煙鎻忚堪浜嗕氦閿欓泦鐨勬暣浣?
浜ら敊閰嶇疆銆?

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