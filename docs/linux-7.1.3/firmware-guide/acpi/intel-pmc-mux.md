## Intel North Mux-Agent


## 绠€浠?

North Mux-Agent 鏄?Intel PMC 鍥轰欢鐨勪竴椤瑰姛鑳斤紝鍦ㄥぇ澶氭暟甯︽湁 PMC 寰帶鍒跺櫒鐨?Intel 骞冲彴涓婃槸鍙楁敮鎸佺殑銆傚畠鐢ㄤ簬閰嶇疆绯荤粺涓婄殑鍚勭 USB 澶氳矾澶嶇敤鍣?瑙ｅ鐢ㄥ櫒
锛圡ultiplexer/DeMultiplexer锛夈€傚厑璁镐粠鎿嶄綔绯荤粺閰嶇疆 mux-agent 鐨勫钩鍙版湁涓€涓?ACPI 璁惧瀵硅薄锛堣妭鐐癸級锛屽叾 HID 涓?"INTC105C"锛屼唬琛ㄥ畠銆?
North Mux-Agent锛堝張绉?Intel PMC Mux Control锛屾垨绠€绉?mux-agent锛夐┍鍔ㄩ€氳繃
浣跨敤 PMC IPC 鏂规硶锛坉rivers/platform/x86/intel_scu_ipc.c锛変笌 PMC 寰帶鍒跺櫒
閫氫俊銆傝椹卞姩鍚?USB Type-C Mux Class 娉ㄥ唽锛屼粠鑰屽厑璁?USB Type-C 鎺у埗鍣ㄥ拰
鎺ュ彛椹卞姩閰嶇疆绾跨紗鎻掑ご鏂瑰悜鍜屾ā寮忥紙鍙婁氦鏇挎ā寮忥紝Alternate Modes锛夈€傝椹卞姩涔?鍚?USB Role Class 娉ㄥ唽锛屼互鏀寔 USB Host 鍜?Device 涓ょ妯″紡銆傝椹卞姩浣嶄簬锛?drivers/usb/typec/mux/intel_pmc_mux.c銆?
## 绔彛鑺傜偣


### 姒傝堪


瀵逛簬绯荤粺涓婂彈 mux-agent 鎺у埗鐨勬瘡涓?USB Type-C 杩炴帴鍣紝鍦?PMC mux-agent
璁惧鑺傜偣涓嬮兘鏈変竴涓嫭绔嬬殑瀛愯妭鐐广€傝繖浜涜妭鐐逛笉浠ｈ〃瀹為檯鐨勮繛鎺ュ櫒锛岃€屾槸 mux-agent
涓殑鈥滈€氶亾锛坈hannel锛夆€?```

	Scope (_SB.PCI0.PMC.MUX)
	{
	    Device (CH0)
	    {
		Name (_ADR, 0)
	    }

	    Device (CH1)
	    {
		Name (_ADR, 1)
	    }
	}

```
### _PLD锛堣澶囩殑鐗╃悊浣嶇疆锛孭hysical Location of Device锛?
鍙€夌殑 _PLD 瀵硅薄鍙互涓庣鍙ｏ紙閫氶亾锛夎妭鐐逛竴璧蜂娇鐢ㄣ€傚鏋?_PLD
```

	Scope (_SB.PCI0.PMC.MUX)
	{
	    Device (CH0)
	    {
		Name (_ADR, 0)
	        Method (_PLD, 0, NotSerialized)
                {
		    /* 灏嗘瑙嗕负浼唬鐮併€?*/
		    Return (\_SB.USBC.CON0._PLD())
		}
	    }
	}

```
### mux-agent 涓撶敤鐨?_DSD 璁惧灞炴€?

#### 绔彛鍙?
涓轰簡閰嶇疆 USB Type-C 杩炴帴鍣ㄨ儗鍚庣殑 mux锛孭MC 鍥轰欢闇€瑕佺煡閬撲笌璇ヨ繛鎺ュ櫒鍏宠仈鐨?USB2 绔彛鍜?USB3 绔彛銆傞┍鍔ㄩ€氳繃璇诲彇鍚嶄负 "usb2-port-number" 鍜?"usb3-port-number" 鐨勭壒瀹?_DSD 璁惧灞炴€ф潵鎻愬彇姝ｇ‘鐨勭鍙ｅ彿銆傝繖浜涘睘鎬у叿鏈?琛ㄧず绔彛绱㈠紩鐨勬暣鏁板€笺€傜鍙ｇ储寮曠紪鍙锋槸鍩轰簬 1 鐨勶紝鍊?0 鏄潪娉曠殑銆傞┍鍔ㄥ湪鍚?mux-agent 鍙戦€佺壒瀹氭秷鎭椂锛屽師鏍蜂娇鐢ㄤ粠杩欎簺璁惧灞炴€т腑鎻愬彇鐨勬暟瀛?```

	Name (_DSD, Package () {
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
	    Package() {
	        Package () {"usb2-port-number", 6},
	        Package () {"usb3-port-number", 3},
	    },
	})

```
#### 鏂瑰悜

鏍规嵁骞冲彴鐨勪笉鍚岋紝鏉ヨ嚜杩炴帴鍣ㄧ殑鏁版嵁绾垮拰 SBU 绾夸粠 mux-agent 鐨勮搴︾湅鍙兘鏄?鈥滃浐瀹氱殑锛坒ixed锛夆€濓紝杩欐剰鍛崇潃 mux-agent 椹卞姩涓嶅簲鏍规嵁绾跨紗鎻掑ご鏂瑰悜閰嶇疆瀹冧滑銆?渚嬪锛屽綋骞冲彴涓婄殑閲嶅畾鏃跺櫒锛坮etimer锛夊鐞嗙嚎缂嗘彃澶存柟鍚戞椂锛屽氨浼氬彂鐢熻繖绉嶆儏鍐点€?椹卞姩浣跨敤鐗瑰畾鐨勮澶囧睘鎬?"sbu-orientation"锛圫BU锛夊拰 "hsl-orientation"锛堟暟鎹級
鏉ヤ簡瑙ｈ繖浜涚嚎鏄惁鈥滃浐瀹氣€濓紝浠ュ強鍥哄畾鍒板摢涓柟鍚戙€傝繖浜涘睘鎬у叿鏈夌殑鍊兼槸瀛楃涓插€硷紝
瀹冨彲浠ユ槸涓?USB Type-C 杩炴帴鍣ㄦ柟鍚戝畾涔夌殑鍊间箣涓€锛?normal"
```

	Name (_DSD, Package () {
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
	    Package() {
	        Package () {"sbu-orientation", "normal"},
	        Package () {"hsl-orientation", "normal"},
	    },
	})

```
## 绀轰緥 ASL


浠ヤ笅 ASL 鏄竴涓ず渚嬶紝灞曠ず浜?mux-agent 鑺傜偣浠ュ強涓や釜
```

	Scope (_SB.PCI0.PMC)
	{
	    Device (MUX)
	    {
	        Name (_HID, "INTC105C")

	        Device (CH0)
	        {
	            Name (_ADR, 0)

	            Name (_DSD, Package () {
	                ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
	                Package() {
	                    Package () {"usb2-port-number", 6},
	                    Package () {"usb3-port-number", 3},
	                    Package () {"sbu-orientation", "normal"},
	                    Package () {"hsl-orientation", "normal"},
	                },
	            })
	        }

	        Device (CH1)
	        {
	            Name (_ADR, 1)

	            Name (_DSD, Package () {
	                ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
	                Package() {
	                    Package () {"usb2-port-number", 5},
	                    Package () {"usb3-port-number", 2},
	                    Package () {"sbu-orientation", "normal"},
	                    Package () {"hsl-orientation", "normal"},
	                },
	            })
	        }
	    }
	}

```
