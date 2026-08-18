
## 寮曠敤灞傜骇鏁版嵁鑺傜偣


:Copyright: |copy| 2018, 2021 Intel Corporation
:Author: Sakari Ailus <sakari.ailus@linux.intel.com>

ACPI 閫氬父鍙厑璁稿紩鐢ㄦ爲涓殑璁惧瀵硅薄銆傚眰绾ф暟鎹墿灞曡妭鐐规棤娉曠洿鎺ヨ寮曠敤锛屽洜姝ゆ湰鏂囨。瀹氫箟浜嗕竴绉嶅疄鐜版绫诲紩鐢ㄧ殑鏂规銆?
瀵?_DSD 灞傜骇鏁版嵁鑺傜偣鐨勫紩鐢ㄦ槸涓€涓瓧绗︿覆锛岀敱涓€涓澶囧璞″紩鐢ㄣ€佷竴涓偣锛堚€?鈥濓級浠ュ強鍒版暟鎹妭鐐瑰璞＄殑鐩稿璺緞缁勬垚銆備笉瑕佷娇鐢ㄩ潪瀛楃涓插紩鐢紝鍥犱负閭ｄ細浜х敓灞傜骇鏁版嵁鑺傜偣鐨勫壇鏈紝鑰屼笉鏄紩鐢紒

琚紩鐢ㄧ殑灞傜骇鏁版嵁鎵╁睍鑺傜偣搴旂洿鎺ヤ綅浜庡叾鐖跺璞′箣涓嬶紝鍗宠涔堜綅浜庤澶囧璞′箣涓嬶紝瑕佷箞浣嶄簬鍙︿竴涓眰绾ф暟鎹墿灞曡妭鐐逛箣涓?[dsd-guide]銆?
灞傜骇鏁版嵁鑺傜偣涓殑閿簲鐢辫妭鐐瑰悕绉般€佲€淍鈥濆瓧绗︿互鍙婅妭鐐圭殑缂栧彿锛堝崄鍏繘鍒惰〃绀猴紝涓嶅甫鍓嶅悗缂€锛夌粍鎴愩€傚悓涓€涓?ACPI 瀵硅薄搴斿寘鍚甫鏈?鈥渞eg鈥?灞炴€х殑 _DSD 灞炴€ф墿灞曪紝璇ュ睘鎬х殑鏁板€煎簲涓庤妭鐐圭紪鍙风浉鍚屻€?
濡傛灉鏌愪釜灞傜骇鏁版嵁鎵╁睍鑺傜偣娌℃湁鏁板€硷紝鍒欏簲浠?ACPI 瀵硅薄鐨?_DSD 灞炴€т腑鐪佺暐 鈥渞eg鈥?灞炴€э紝骞朵粠灞傜骇鏁版嵁鎵╁睍閿腑鐪佺暐 鈥淍鈥?瀛楃涓庣紪鍙枫€?

## 绀轰緥


鍦ㄤ笅闈㈢殑 ASL 鐗囨涓紝鈥渞eference鈥?_DSD 灞炴€у寘鍚灞傜骇鏁版嵁鎵╁睍鑺傜偣 ANOD 鐨勫瓧绗︿覆寮曠敤锛岃鑺傜偣浣嶄簬 DEV1 鐖跺璞′箣涓嬬殑 DEV0 涔嬩笅銆侫NOD 鍚屾椂涔熸槸璇ュ紩鐢ㄧ殑鏈€缁堢洰鏍囪妭鐐广€?```

	Device (DEV0)
	{
	    Name (_DSD, Package () {
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "node@0", "NOD0" },
		    Package () { "node@1", "NOD1" },
		}
	    })
	    Name (NOD0, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 0 },
		    Package () { "random-property", 3 },
		}
	    })
	    Name (NOD1, Package() {
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "reg", 1 },
		    Package () { "anothernode", "ANOD" },
		}
	    })
	    Name (ANOD, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "random-property", 0 },
		}
	    })
	}

	Device (DEV1)
	{
	    Name (_DSD, Package () {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reference", "^DEV0.ANOD" }
		    },
		}
	    })
	}

```
鍙﹁鍙傞槄鍥捐〃绀轰緥锛?Documentation/firmware-guide/acpi/dsd/graph.rst銆?
## 鍙傝€?

[dsd-guide] DSD Guide.
    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.adoc锛屽紩鐢ㄦ棩鏈?    2021-11-30銆?