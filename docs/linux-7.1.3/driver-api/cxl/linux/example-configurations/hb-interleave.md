
## 涓绘満妗ラ棿浜ら敊锛圛nter-Host-Bridge Interleave锛?
杩欎唤 cxl-cli 閰嶇疆杞偍灞曠ず浜嗗涓嬬殑涓绘満閰嶇疆锛?
- 涓€涓崟璺紙single socket锛夌郴缁燂紝甯︽湁涓€涓?CXL root
- CXL Root 鏈夊洓涓紙4锛塁XL Host Bridge锛堜富鏈烘ˉ锛?- 鍏朵腑涓や釜 CXL Host Bridge 鍚勬寕杞戒簡涓€涓?CXL Memory Expander锛堝唴瀛樻墿灞曞櫒锛?- 璇?CXL root 琚厤缃负鍦ㄤ袱涓富鏈烘ˉ涔嬮棿杩涜浜ら敊锛坕nterleave锛?
璇ヨ緭鍑虹敱 `cxl list -v` 鐢熸垚锛屾弿杩颁簡鍦?`/sys/bus/cxl/devices/` 涓毚闇茬殑鍚勫璞′箣闂寸殑
鍏崇郴銆?
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
杩欐灞曠ず浜?CXL 鈥渂us鈥濓紙root0锛夋湁 4 涓繛鎺ュ埌 CXL Host Bridge 鐨勪笅娓哥鍙ｏ紙downstream port锛夈€?`Root` 鍙互鐪嬩綔杩炴帴鍒板钩鍙板唴瀛樻帶鍒跺櫒锛坢emory controller锛夌殑鍗曚竴涓婃父绔彛锛坲pstream port锛夆€斺€斿畠
灏嗗唴瀛樿姹傝矾鐢卞埌鑷韩銆?
`ports:root0` 閮ㄥ垎璇存槑浜嗚繖浜涗笅娓哥鍙ｅ悇鑷槸濡備綍閰嶇疆鐨勩€傚鏋滀竴涓鍙ｆ湭琚厤缃紙id 涓?0 鍜?1锛夛紝
鍒欎細琚渷鐣ャ€?
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
杩欐灞曠ず浜嗕笌 CXL Host Bridge `port1` 鍏宠仈鐨勫彲鐢ㄤ笅娓哥鍙ｃ€傚湪鏈緥涓紝`port1` 鏈?3 涓彲鐢ㄧ殑
涓嬫父绔彛锛歚dport1`銆乣dport2` 涓?`dport113`..

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
杩欐灞曠ず浜嗚繛鎺ュ埌涓绘満妗?`port1` 鐨勭鐐癸紙endpoint锛夈€?
`endpoint5` 鍖呭惈涓€涓凡閰嶇疆鐨?decoder `decoder5.0`锛屽叾浜ら敊閰嶇疆涓?`region0`锛堢◢鍚庡睍绀猴級鐩稿悓銆?
鎺ヤ笅鏉ユ槸褰掑睘浜庝富鏈烘ˉ鐨?decoder锛堣В鐮佸櫒锛夛細

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
涓绘満妗?`port1` 鏈変竴涓?decoder锛坄decoder1.0`锛夛紝鍏跺敮涓€鐩爣鏄?`dport1`鈥斺€斿畠杩炴帴鍒?`endpoint5`銆?
鎺ヤ笅鏉ョ殑涓€娈靛睍绀轰簡涓绘満妗?`port3` 鐨勭被浼奸厤缃紝鍗崇浜屼釜鎸傝浇浜嗗唴瀛樿澶囩殑涓绘満妗ャ€?
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
涓嬩竴娈靛睍绀轰簡涓や釜娌℃湁杩炴帴绔偣鐨?CXL 涓绘満妗ャ€?
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
鎺ヤ笅鏉ユ槸褰掑睘浜?`root0` 鐨?`Root Decoders`锛堟牴瑙ｇ爜鍣級銆傝繖涓牴瑙ｇ爜鍣ㄥ湪涓嬫父绔彛 `port1` 涓?`port3` 涔嬮棿搴旂敤浜ら敊鈥斺€旂矑搴︿负 256 瀛楄妭銆?
杩欎簺淇℃伅鐢?CXL 椹卞姩璇诲彇 ACPI CEDT CFMWS 鐢熸垚銆?
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
鏈€鍚庢槸鍏宠仈鍒?`Root Decoder` `decoder0.0` 鐨?`Memory Region`锛堝唴瀛樺尯鍩燂級銆傝鍖哄煙鎻忚堪浜嗘暣涓?浜ら敊闆嗗悎锛坕nterleave set锛夌殑鎬讳綋浜ら敊閰嶇疆銆?
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
