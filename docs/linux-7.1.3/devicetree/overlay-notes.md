
## Devicetree Overlay 绗旇


鏈枃妗ｆ弿杩颁綅浜?drivers/of/overlay.c 鐨勫唴鏍稿唴璁惧鏍?overlay 鍔熻兘鐨勫疄鐜帮紝鏄?
Documentation/devicetree/dynamic-resolution-notes.rst[^1^] 鐨勯厤濂楁枃妗ｃ€?

### overlay 濡備綍宸ヤ綔


Devicetree 鐨?overlay 鐩殑鏄慨鏀瑰唴鏍哥殑瀹炴椂鏍戯紙live tree锛夛紝骞朵娇璇ヤ慨鏀逛互鍙嶆槧鍙樻洿鐨勬柟寮?
褰卞搷鍐呮牳鐨勭姸鎬併€傜敱浜庡唴鏍镐富瑕佸鐞嗚澶囷紝浠讳綍瀵艰嚧涓€涓椿鍔ㄨ澶囩殑鏂拌澶囪妭鐐归兘搴斿綋鍦ㄥ垱寤烘椂
琚缓绔嬶紝鑰屽鏋滆澶囪妭鐐硅绂佺敤鎴栨暣浣撶Щ闄わ紝鍙楀奖鍝嶇殑璁惧搴斿綋琚敞閿€銆?

```

    ---- foo.dts ---------------------------------------------------------------
	/* FOO platform */
	/dts-v1/;
	/ {
		compatible = "corp,foo";

		/* shared resources */
		res: res {
		};

		/* On chip peripherals */
		ocp: ocp {
			/* peripherals that are always instantiated */
			peripheral1 { ... };
		};
	};
    ---- foo.dts ---------------------------------------------------------------

```
overlay bar.dtso锛?
```

    ---- bar.dtso - overlay target location by label ---------------------------
	/dts-v1/;
	/plugin/;
	&ocp {
		/* bar peripheral */
		bar {
			compatible = "corp,bar";
			... /* various properties and child nodes */
		};
	};
    ---- bar.dtso --------------------------------------------------------------

```
```

    ---- foo+bar.dts -----------------------------------------------------------
	/* FOO platform + bar peripheral */
	/ {
		compatible = "corp,foo";

		/* shared resources */
		res: res {
		};

		/* On chip peripherals */
		ocp: ocp {
			/* peripherals that are always instantiated */
			peripheral1 { ... };

			/* bar peripheral */
			bar {
				compatible = "corp,bar";
				... /* various properties and child nodes */
			};
		};
	};
    ---- foo+bar.dts -----------------------------------------------------------

```
浣滀负 overlay 鐨勭粨鏋滐紝涓€涓柊鐨勮澶囪妭鐐癸紙bar锛夎鍒涘缓锛屽洜姝や竴涓?bar platform 璁惧浼氳娉ㄥ唽锛?
濡傛灉鍔犺浇浜嗗尮閰嶇殑璁惧椹卞姩锛岃璁惧浼氬鏈熻鍒涘缓銆?

濡傛灉鍩虹 DT 鍦ㄧ紪璇戞椂娌℃湁浣跨敤 -@ 閫夐」锛岄偅涔?"&ocp" 鏍囩灏嗕笉鍙敤浜庢妸 overlay 鑺傜偣瑙ｆ瀽鍒?
鍩虹 DT 涓殑姝ｇ‘浣嶇疆銆傚湪杩欑鎯呭喌涓嬶紝鍙互鎻愪緵鐩爣璺緞銆傚熀浜庢爣绛捐娉曠殑鐩爣浣嶇疆鏄閫夌殑锛?
鍥犱负 overlay 鍙互搴旂敤鍒颁换浣曞寘鍚鏍囩鐨勫熀纭€ DT锛屾棤璁鸿鏍囩鍦?DT 涓嚭鐜板湪浣曞銆?

```

    ---- bar.dtso - overlay target location by explicit path -------------------
	/dts-v1/;
	/plugin/;
	&{/ocp} {
		/* bar peripheral */
		bar {
			compatible = "corp,bar";
			... /* various properties and child nodes */
		}
	};
    ---- bar.dtso --------------------------------------------------------------


```
### 鍐呮牳鍐?overlay API


璇?API 浣跨敤璧锋潵鐩稿綋瀹规槗銆?

1) 璋冪敤 of_overlay_fdt_apply() 浠ュ垱寤哄苟搴旂敤涓€涓?overlay changeset銆傝繑鍥炲€艰涔堟槸涓€涓敊璇紝
   瑕佷箞鏄竴涓爣璇嗚 overlay 鐨?cookie銆?

2) 璋冪敤 of_overlay_remove() 浠ョЩ闄ゅ苟娓呯悊涔嬪墠閫氳繃 of_overlay_fdt_apply() 璋冪敤鍒涘缓鐨?overlay
   changeset銆備笉鍏佽绉婚櫎琚彟涓€涓?overlay 鍫嗗彔鐨?overlay changeset銆?

鏈€鍚庯紝濡傛灉浣犻渶瑕佷竴娆℃€хЩ闄ゆ墍鏈?overlay锛屽彧闇€璋冪敤 of_overlay_remove_all()锛屽畠浼氫互姝ｇ‘鐨勯『搴?
绉婚櫎姣忎竴涓?overlay銆?

杩樺彲浠ユ敞鍐屽湪 overlay 鎿嶄綔鏃惰皟鐢ㄧ殑閫氱煡鍣紙notifier锛夈€傝瑙?of_overlay_notifier_register/unregister
鍜?enum of_overlay_notify_action銆?

閽堝 OF_OVERLAY_PRE_APPLY銆丱F_OVERLAY_POST_APPLY 鎴?OF_OVERLAY_PRE_REMOVE 鐨勯€氱煡鍣ㄥ洖璋?
鍙互鍦?overlay 鎴栧叾鍐呭涓繚瀛樻寚鍚戣澶囨爲鑺傜偣鐨勬寚閽堬紝浣嗚繖浜涙寚閽堝湪 OF_OVERLAY_POST_REMOVE
鐨勯€氱煡鍣ㄥ洖璋冭繑鍥炲悗涓嶅緱缁х画瀛樺湪銆傚寘鍚?overlay 鐨勫唴瀛樹細鍦?OF_OVERLAY_POST_REMOVE 閫氱煡鍣ㄨ
璋冪敤鍚庤 kfree()銆傛敞鎰忥紝鍗充娇 OF_OVERLAY_POST_REMOVE 鐨勯€氱煡鍣ㄨ繑鍥為敊璇紝璇ュ唴瀛樹粛浼氳 kfree()銆?

drivers/of/dynamic.c 涓殑 changeset 閫氱煡鍣ㄦ槸绗簩绫诲彲鑳界敱搴旂敤鎴栫Щ闄?overlay 瑙﹀彂鐨勯€氱煡鍣ㄣ€?
杩欎簺閫氱煡鍣ㄤ笉鍏佽淇濆瓨鎸囧悜 overlay 涓澶囨爲鑺傜偣鎴栧叾鍐呭鐨勬寚閽堛€俹verlay 浠ｇ爜骞朵笉闃叉姝ょ被鎸囬拡
鍦ㄥ寘鍚?overlay 鐨勫唴瀛樺洜绉婚櫎 overlay 鑰岃閲婃斁鏃朵粛鐒朵繚鎸佹椿鍔ㄣ€?

浠讳綍鍏跺畠淇濈暀鎸囧悜 overlay 鑺傜偣鎴栨暟鎹寚閽堢殑浠ｇ爜閮借瑙嗕负缂洪櫡锛坆ug锛夛紝鍥犱负鍦ㄧЩ闄?overlay 鍚?
璇ユ寚閽堝皢鎸囧悜宸查噴鏀剧殑鍐呭瓨銆?

overlay 鐨勪娇鐢ㄨ€呭繀椤荤壒鍒暀鎰忕郴缁熶笂鍙戠敓鐨勬暣浣撴搷浣滐紝浠ョ‘淇濆叾瀹冨唴鏍镐唬鐮佷笉浼氫繚鐣欎换浣曟寚鍚?
overlay 鑺傜偣鎴栨暟鎹殑鎸囬拡銆備竴涓棤鎰忎腑浣跨敤姝ょ被鎸囬拡鐨勪緥瀛愭槸锛氬湪 overlay 琚簲鐢ㄤ箣鍚庢墠鍔犺浇
椹卞姩鎴栧瓙绯荤粺妯″潡锛岃€岃椹卞姩鎴栧瓙绯荤粺鎵弿鏁翠釜璁惧鏍戞垨鍏跺ぇ閮ㄥ垎锛屽寘鎷?overlay 鑺傜偣銆?
