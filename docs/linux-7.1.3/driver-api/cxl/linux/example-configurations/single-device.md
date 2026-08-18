
## 鍗曚竴璁惧


杩欎釜 cxl-cli 閰嶇疆杞偍鏄剧ず浜嗕互涓嬩富鏈洪厤缃細

- 涓€涓崟鎻掓Ы绯荤粺锛屽甫鏈変竴涓?CXL 鏍?- CXL 鏍规湁鍥涗釜锛?锛塁XL 涓绘満妗?- 鍏朵腑涓€涓?CXL 涓绘満妗ユ寕鎺ヤ簡涓€涓崟鐙殑 CXL 鍐呭瓨鎵╁睍鍣?- 涓嶅瓨鍦ㄤ氦閿欙紙interleave锛夈€?
璇ヨ緭鍑虹敱 `cxl list -v` 鐢熸垚锛屾弿杩颁簡鍦?`/sys/bus/cxl/devices/` 涓毚闇茬殑瀵硅薄
涔嬮棿鐨勫叧绯汇€?
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
杩欎竴娈垫樉绀?CXL 鈥渂us鈥濓紙root0锛夋湁 4 涓寕鎺ュ埌 CXL 涓绘満妗ョ殑涓嬫父绔彛銆俙Root` 鍙涓?鎸傛帴鍒板钩鍙板唴瀛樻帶鍒跺櫒鐨勫敮涓€涓婃父绔彛鈥斺€斿畠璐熻矗灏嗗唴瀛樿姹傝矾鐢卞埌璇ユ帶鍒跺櫒銆?
`ports:root0` 灏忚妭鍒楀嚭浜嗚繖浜涗笅娓哥鍙ｅ悇鑷殑閰嶇疆鏂瑰紡銆傚鏋滄煇涓鍙ｆ病鏈夐厤缃?锛坕d 涓?0銆? 鍜?4锛夛紝鍒欏皢鍏剁渷鐣ャ€?
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
杩欎竴娈垫樉绀轰簡涓?CXL 涓绘満妗?`port1` 鍏宠仈鐨勫彲鐢ㄤ笅娓哥鍙ｃ€傚湪鏈緥涓紝`port1` 鏈?3
涓彲鐢ㄧ殑涓嬫父绔彛锛歚dport1`銆乣dport2` 鍜?`dport113`銆傘€?
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
杩欎竴娈垫樉绀轰簡鎸傛帴鍒颁富鏈烘ˉ `port1` 鐨勭鐐广€?
`endpoint5` 鍖呭惈涓€涓凡閰嶇疆鐨勫崟涓В鐮佸櫒 `decoder5.0`锛屽叾浜ら敊閰嶇疆涓?`region0`
锛堢◢鍚庢樉绀猴級鐩稿悓銆?
鎺ヤ笅鏉ユ槸褰掑睘浜庤涓绘満妗ョ殑瑙ｇ爜鍣細

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
涓绘満妗?`port1` 鏈変竴涓崟鐙殑瑙ｇ爜鍣紙`decoder1.0`锛夛紝鍏跺敮涓€鐩爣鏄?`dport1`鈥斺€斿畠
鎸傛帴鍒颁簡 `endpoint5`銆?
涓嬩竴娈垫樉绀轰簡涓変釜娌℃湁鎸傛帴绔偣鐨?CXL 涓绘満妗ャ€?
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
鎺ヤ笅鏉ユ槸褰掑睘浜?`root0` 鐨?`Root Decoders`銆傝繖涓牴瑙ｇ爜鍣ㄦ槸涓€涓洿閫氳В鐮佸櫒锛屽洜涓?`interleave_ways` 琚涓?`1`銆?
杩欎簺淇℃伅鐢?CXL 椹卞姩璇诲彇 ACPI CEDT CMFWS 鐢熸垚銆?
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
鏈€鍚庢槸涓?`Root Decoder` `decoder0.0` 鍏宠仈鐨?`Memory Region`銆傝鍖哄煙鎻忚堪浜嗕笌
杩欎釜鍞竴璁惧鍏宠仈鐨勭鏁ｅ尯鍩熴€?
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
