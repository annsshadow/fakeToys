
## 鍦?ACPI 涓弿杩颁笌寮曠敤 LED


鍗曚釜 LED 鐢辫澶囪妭鐐癸紙鍗?LED 椹卞姩鑺墖锛変笅鐨勫垎灞傛暟鎹墿灞?[^5^] 鑺傜偣鎻忚堪銆侺ED 鐗瑰畾鑺傜偣涓殑 "reg" 灞炴€у憡鐭ユ瘡涓繛鎺ュ埌 LED 鐨?LED 杈撳嚭鐨勬暟瀛?ID銆俒leds] 鍒嗗眰鏁版嵁鑺傜偣鍛藉悕涓?"led@X"锛屽叾涓?X 鏄?LED 杈撳嚭鐨勭紪鍙枫€?

鍦?Device Tree 涓紩鐢?LED 鐨勬枃妗ｄ綅浜?[video-interfaces] 鐨?"flash-leds" 灞炴€ф枃妗ｄ腑銆傜畝鑰岃█涔嬶紝LED 鏄€氳繃浣跨敤 phandle 鐩存帴寮曠敤鐨勩€?

ACPI锛堜笌 DT 涓€鏍凤級鍏佽鍦ㄥ紩鐢ㄥ悗浣跨敤鏁存暟鍙傛暟銆侺ED 椹卞姩璁惧寮曠敤涓庝竴涓暣鏁板弬鏁帮紙寮曠敤鐩稿叧 LED 鐨?"reg" 灞炴€э級鐨勭粍鍚堬紝鐢ㄤ簬鏍囪瘑鍗曚釜 LED銆?reg" 灞炴€х殑鍊兼槸鍥轰欢涓庤蒋浠朵箣闂寸殑绾﹀畾锛屽畠鍞竴鍦版爣璇?LED 椹卞姩杈撳嚭銆?

鍦?LED 椹卞姩璁惧涓嬶紝绗竴涓垎灞傛暟鎹墿灞曞寘鍒楄〃鏉＄洰搴斿寘鍚瓧绗︿覆 "led@" 鍚庤窡 LED 鐨勭紪鍙凤紝鍐嶅悗璺熻寮曠敤瀵硅薄鐨勫悕绉般€傝瀵硅薄搴斿懡鍚嶄负 "LED" 鍚庤窡 LED 鐨勭紪鍙枫€?

## 绀轰緥


涓嬮潰灞曠ず浜嗕竴涓浉鏈轰紶鎰熷櫒璁惧涓庝竴涓甫涓や釜 LED 鐨?LED 椹卞姩璁惧鐨?ASL 绀轰緥銆備笌 LED 鎴栧鍏剁殑寮曠敤鏃犲叧鐨勫璞″凡琚?
```

	Device (LED)
	{
		Name (_DSD, Package () {
			ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
			Package () {
				Package () { "led@0", LED0 },
				Package () { "led@1", LED1 },
			}
		})
		Name (LED0, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () { "reg", 0 },
				Package () { "flash-max-microamp", 1000000 },
				Package () { "flash-timeout-us", 200000 },
				Package () { "led-max-microamp", 100000 },
				Package () { "label", "white:flash" },
			}
		})
		Name (LED1, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () { "reg", 1 },
				Package () { "led-max-microamp", 10000 },
				Package () { "label", "red:indicator" },
			}
		})
	}

	Device (SEN)
	{
		Name (_DSD, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () {
					"flash-leds",
					Package () { "^LED.LED0", "^LED.LED1" },
				}
			}
		})
	}

```

鍏朵腑
```

	LED	LED 椹卞姩璁惧
	LED0	绗竴涓?LED
	LED1	绗簩涓?LED
	SEN	鐩告満浼犳劅鍣ㄨ澶囷紙鎴?LED 鐩稿叧鐨勫彟涓€涓澶囷級

```

## 鍙傝€?


[acpi] Advanced Configuration and Power Interface Specification銆?
    https://uefi.org/specifications/ACPI/6.4/锛屽紩鐢ㄦ棩鏈?2021-11-30銆?

[data-node-ref] Documentation/firmware-guide/acpi/dsd/data-node-references.rst

[devicetree] Devicetree銆俬ttps://www.devicetree.org锛屽紩鐢ㄦ棩鏈?2019-02-21銆?

[dsd-guide] DSD Guide銆?
    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.adoc锛屽紩鐢?
    鏃ユ湡 2021-11-30銆?

[leds] Documentation/devicetree/bindings/leds/common.yaml

[video-interfaces] Documentation/devicetree/bindings/media/video-interfaces.yaml
