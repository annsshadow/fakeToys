
## 鍥撅紙Graphs锛?

## _DSD


_DSD锛圖evice Specific Data锛岃澶囩壒瀹氭暟鎹級[dsd-guide] 鏄竴涓瀹氫箟鐨?ACPI 璁惧
閰嶇疆瀵硅薄锛屽彲鐢ㄤ簬浼犺揪 ACPI 瑙勮寖 [acpi] 鏈笓闂ㄦ兜鐩栫殑纭欢鐗规€т俊鎭€備笌鍥剧浉鍏崇殑
_DSD 鎵╁睍鏈変袱绉嶏細property锛堝睘鎬э級[dsd-guide] 鎵╁睍鍜?hierarchical data锛堝垎灞傛暟鎹級
鎵╁睍銆俻roperty 鎵╁睍鎻愪緵閫氱敤鐨勯敭鍊煎锛岃€?hierarchical data 鎵╁睍鏀寔甯︽湁鎸囧悜鍏朵粬
鑺傜偣寮曠敤鐨勮妭鐐癸紝浠庤€屽舰鎴愪竴妫垫爲銆傛爲涓殑鑺傜偣鍙互鍖呭惈鐢?property 鎵╁睍瀹氫箟鐨勫睘鎬с€?杩欎袱绉嶆墿灞曚竴璧锋彁渚涗簡涓€涓被鏍戠粨鏋勶紝鏍戠殑姣忎釜鑺傜偣甯︽湁闆朵釜鎴栧涓睘鎬э紙閿€煎锛夈€?
璇ユ暟鎹粨鏋勫彲浠ュ湪杩愯鏃堕€氳繃 include/linux/fwnode.h 涓畾涔夌殑 device_* 鍜?fwnode_*
鍑芥暟鏉ヨ闂€?
fwnode 琛ㄧず涓€涓€氱敤鐨勫浐浠惰妭鐐瑰璞°€傚畠鐙珛浜庡浐浠剁被鍨嬨€傚湪 ACPI 涓紝fwnode 鏄?_DSD 鍒嗗眰鏁版嵁鎵╁睍瀵硅薄銆備竴涓澶囩殑 _DSD 瀵硅薄鐢变竴涓?fwnode 琛ㄧず銆?
璇ユ暟鎹粨鏋勫彲浠ュ湪 ACPI 琛ㄧ殑鍏跺畠浣嶇疆琚紩鐢紝鏂规硶鏄娇鐢ㄥ璁惧鏈韩鐨勭‖寮曠敤锛屼互鍙?鍦ㄦ瘡涓€娣卞害涓婂鍒嗗眰鏁版嵁鎵╁睍鏁扮粍鐨勭储寮曘€?

## 绔彛锛圥orts锛変笌绔偣锛坋ndpoints锛?

绔彛鍜岀鐐圭殑姒傚康涓?Devicetree [devicetree, graph-bindings] 涓殑闈炲父鐩镐技銆傜鍙?琛ㄧず涓€涓澶囦腑鐨勬帴鍙ｏ紝绔偣琛ㄧず瀵硅鎺ュ彛鐨勮繛鎺ャ€傚彟璇峰弬瑙?[data-node-ref] 浜嗚В
閫氱敤鐨勬暟鎹妭鐐瑰紩鐢ㄣ€?
鎵€鏈夌鍙ｈ妭鐐归兘浣嶄簬璁惧 _DSD 鑺傜偣涓嬬殑鍒嗗眰鏁版嵁鎵╁睍鏍戜腑銆備笌姣忎釜绔彛鑺傜偣鐩稿叧鐨勬暟鎹?鎵╁睍蹇呴』浠?"port" 寮€澶达紝鍏跺悗蹇呴』璺熼殢 "@" 瀛楃浠ュ強绔彛缂栧彿浣滀负閿€傚畠寮曠敤鐨勭洰鏍?瀵硅薄搴斿懡鍚嶄负 "PRTX"锛屽叾涓?```

    Package() { "port@4", "PRT4" }

```
姝ゅ锛岀鐐逛綅浜庣鍙ｈ妭鐐逛箣涓嬨€傜鐐硅妭鐐圭殑鍒嗗眰鏁版嵁鎵╁睍閿繀椤讳互 "endpoint" 寮€澶达紝
鍏跺悗蹇呴』璺熼殢 "@" 瀛楃浠ュ強绔偣缂栧彿銆傚畠寮曠敤鐨勫璞″簲鍛藉悕涓?"EPXY"锛屽叾涓?"X" 涓虹鍙?缂栧彿锛?Y" 涓虹鐐圭紪鍙枫€傛绫荤ず渚嬩负
```

    Package() { "endpoint@0", "EP40" }

```
姣忎釜绔彛鑺傜偣鍚湁涓€涓?property 鎵╁睍閿?"port"锛屽叾鍊间负绔彛缂栧彿銆傛瘡涓鐐逛篃绫讳技鍦扮敤
property 鎵╁睍閿?"reg" 缂栧彿锛屽叾鍊间负绔偣缂栧彿銆傜鍙ｇ紪鍙峰湪涓€涓澶囧唴蹇呴』鍞竴锛岀鐐?缂栧彿鍦ㄤ竴涓鍙ｅ唴蹇呴』鍞竴銆傚鏋滀竴涓澶囧璞″彲鑳藉彧鏈変竴涓鍙ｏ紝鍒欒绔彛鐨勭紪鍙峰簲涓?闆躲€傜被浼煎湴锛屽鏋滀竴涓鍙ｅ彲鑳藉彧鏈変竴涓鐐癸紝鍒欒绔偣鐨勭紪鍙峰簲涓洪浂銆?
绔偣寮曠敤浣跨敤甯?"remote-endpoint" 灞炴€х殑 property 鎵╁睍
```

    "device.datanode"

```
鍦ㄤ笂渚嬩腑锛?X" 涓虹鍙ｇ紪鍙凤紝"Y" 涓虹鐐圭紪鍙枫€?
瀵圭鐐圭殑寮曠敤蹇呴』鎬绘槸鍙屽悜杩涜锛屾棦瑕佹寚鍚戣繙绔鐐癸紝涔熻浠庤寮曠敤鐨勮繙绔鐐硅妭鐐规寚鍥?鏉ャ€?
```

    Scope (\_SB.PCI0.I2C2)
    {
	Device (CAM0)
	{
	    Name (_DSD, Package () {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "compatible", Package () { "nokia,smia" } },
		},
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "port@0", "PRT0" },
		}
	    })
	    Name (PRT0, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 0 },
		},
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "endpoint@0", "EP00" },
		}
	    })
	    Name (EP00, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 0 },
		    Package () { "remote-endpoint", "\\_SB.PCI0.ISP.EP40" },
		}
	    })
	}
    }

    Scope (\_SB.PCI0)
    {
	Device (ISP)
	{
	    Name (_DSD, Package () {
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "port@4", "PRT4" },
		}
	    })

	    Name (PRT4, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 4 }, /* CSI-2 port number */
		},
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "endpoint@0", "EP40" },
		}
	    })

	    Name (EP40, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 0 },
		    Package () { "remote-endpoint", "\\_SB.PCI0.I2C2.CAM0.EP00" },
		}
	    })
	}
    }

```
杩欓噷锛?CAM0" 璁惧鐨勭鍙?0 杩炴帴鍒颁簡 "ISP" 璁惧鐨勭鍙?4锛屽弽涔嬩害鐒躲€?

## 鍙傝€冿紙References锛?

[acpi] Advanced Configuration and Power Interface 瑙勮寖銆?    https://uefi.org/specifications/ACPI/6.4/锛屽紩鐢ㄦ棩鏈?2021-11-30銆?
[data-node-ref] Documentation/firmware-guide/acpi/dsd/data-node-references.rst

[devicetree] Devicetree銆俬ttps://www.devicetree.org锛屽紩鐢ㄦ棩鏈?2016-10-03銆?
[dsd-guide] DSD Guide銆?    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.adoc锛屽紩鐢ㄦ棩鏈?    2021-11-30銆?
[dsd-rules] _DSD Device Properties Usage Rules銆?    Documentation/firmware-guide/acpi/DSD-properties-rules.rst

[graph-bindings] Common bindings for device graphs锛圖evicetree 璁惧鍥剧殑閫氱敤缁戝畾锛夈€?    https://github.com/devicetree-org/dt-schema/blob/main/schemas/graph.yaml锛?    寮曠敤鏃ユ湡 2021-11-30銆?