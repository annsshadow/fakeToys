
## 鎻愪氦 Devicetree锛圖T锛夌粦瀹氳ˉ涓?

## I. 闈㈠悜琛ヤ竵鎻愪氦鑰?

  0) 鏉ヨ嚜 `Documentation/process/submitting-patches.rst` 鐨勫父瑙勮ˉ涓佹彁浜よ鍒欏悓鏍烽€傜敤銆?
  1) 琛ヤ竵涓?`Documentation/` 涓?`include/dt-bindings/` 閮ㄥ垎搴?
```

       "dt-bindings: <binding dir>: ..."

     灏戞暟瀛愮郴缁燂紝濡?ASoC銆乵edia銆乺egulators銆丼CSI銆丼PI 鍜?UFS锛屽熀浜庡瓙绯荤粺鍚嶇О鏈熸湜鍓嶇紑椤哄簭鐩稿弽::

       "<binding dir>: dt-bindings: ..."

     涓婚鐨?80 涓瓧绗﹀崄鍒嗗疂璐点€傚缓璁笉瑕佷娇鐢?"Documentation"銆?doc" 鎴?"YAML"锛屽洜涓鸿繖浜涢兘鏄殣鍚殑銆傛墍鏈夌粦瀹氶兘鏄枃妗ｏ紝涓旀墍鏈夋柊缁戝畾閮藉簲閲囩敤 Devicetree schema 鏍煎紡銆備篃搴旈伩鍏嶉噸澶?"binding"锛屽洜姝ゅ浜庝竴涓柊璁惧锛岄€氬父绫讳技涓嬮潰杩欐牱鍗冲彲::

       "dt-bindings: iio: adc: Add ROHM BD79100G"

     灏嗗叾浠栨牸寮忚浆鎹负 DT schema::

       "dt-bindings: iio: adc: adi,ad7476: Convert to DT schema"

  2) DT 缁戝畾鏂囦欢閲囩敤 DT schema 鏍煎紡涔﹀啓锛屼娇鐢?json-schema 璇嶆眹涓?YAML 鏂囦欢鏍煎紡銆侱T 缁戝畾鏂囦欢蹇呴』閫氳繃杩愯浠ヤ笅鍛戒护鐨勬牎楠?:

       make dt_binding_check

     鍏充簬 schema 涓庡伐鍏烽厤缃殑鏇村缁嗚妭锛岃鍙傝 `Documentation/devicetree/bindings/writing-schema.rst`銆?
  3) DT 缁戝畾鏂囦欢搴旈噰鐢ㄥ弻閲嶈鍙€傞閫夎鍙爣绛句负 (GPL-2.0-only OR BSD-2-Clause)銆?
  4) 灏嗘暣涓ˉ涓佺郴鍒楁彁浜ゅ埌 devicetree 閭欢鍒楄〃

       devicetree@vger.kernel.org

     骞舵妱閫侊紙Cc锛塂T 缁存姢鑰呫€備娇鐢?`scripts/get_maintainer.pl` 璇嗗埆鎵€鏈?DT 缁存姢鑰呫€?
  5) 琛ヤ竵鐨?`Documentation/` 閮ㄥ垎搴斾綅浜庡疄鐜拌缁戝畾鐨勪唬鐮佷箣鍓嶏紝闅忚ˉ涓佺郴鍒椾竴骞舵彁浜ゃ€?
  6) 鍦ㄨ姱鐗囨垨鏉跨骇 DTS 鏂囦欢涓娇鐢ㄧ殑浠讳綍 compatible 瀛楃涓诧紝蹇呴』鍏堝墠宸插湪瀵瑰簲鐨?DT 缁戝畾鏂囦欢 `Documentation/devicetree/bindings` 涓褰曘€傚嵆浣?Linux 璁惧椹卞姩灏氭湭鍖归厤璇?compatible 瀛楃涓诧紝姝よ鍒欏悓鏍烽€傜敤銆俒 鑻ユ湭閬靛惊姝ゆ楠わ紝checkpatch 灏嗕細鍙戝嚭璀﹀憡锛岃嚜鎻愪氦 bff5da4335256513497cc8c79f9a9d1665e09864锛?checkpatch: add DT compatible string documentation checks"锛夎捣鐢熸晥銆?]

  7) DTS 鎬讳綋涓婅瑙嗕负涓庨┍鍔ㄦ棤鍏崇殑纭欢鎻忚堪锛屽洜姝や换浣?DTS 琛ヤ竵锛屾棤璁轰娇鐢ㄥ凡鏈夎繕鏄柊鐨勭粦瀹氾紝閮藉簲缃簬琛ヤ竵闆嗘湯灏撅紝浠ヨ〃鏄庨┍鍔ㄥ DTS 娌℃湁渚濊禆銆侱TS 鏃犺濡備綍閮戒細閫氳繃鐙珛鐨勬爲鎴栧垎鏀悎鍏ワ紝鍥犳涓嶅悓鐨勯『搴忎細琛ㄦ槑璇ョ郴鍒椾笉鍙簩鍒嗭紙non-bisectable锛夈€?
     濡傛灉鏌愪釜椹卞姩瀛愮郴缁熺淮鎶よ€呭€惧悜浜庡悎鍏ユ暣涓泦鍚堣€岄潪鍏朵腑鐩稿叧閮ㄥ垎锛岃灏?DTS 琛ヤ竵鎷嗗垎涓虹嫭绔嬬殑琛ヤ竵闆嗭紝骞跺湪鍙樻洿鏃ュ織鎴栧皝闈俊涓紩鐢ㄩ偖浠跺垪琛ㄤ笂鐨勭粦瀹氭彁浜ゃ€?
  8) 濡傛灉鏌愪釜宸茶褰曠殑 compatible 瀛楃涓插皻鏈椹卞姩鍖归厤锛屾枃妗ｈ繕搴斿寘鍚椹卞姩鎵€鍖归厤鐨?compatible 瀛楃涓层€?
  9) 缁戝畾姝ｈ Linux 鍐呮牳涔嬪鐨勫涓」鐩Н鏋佷娇鐢紝鍦ㄤ慨鏀瑰凡鏈夌粦瀹氭椂鍙兘闇€瑕侀澶栫殑璋ㄦ厧涓庤€冮噺銆?
```
## II. 闈㈠悜鍐呮牳缁存姢鑰?

  1) 濡傛灉浣犲瀹℃煡鏌愪釜缁戝畾鎰熷埌涓嶇‘瀹氾紝璇峰洖澶嶈缁戝畾骞惰姹?devicetree 缁存姢鑰呯粰浜堟寚瀵笺€傝繖灏嗘湁鍔╀簬浠栦滑纭畾浼樺厛瀹℃煡鍝簺銆佸摢浜涘彲浠ユ斁琛屻€?
  2) 瀵逛簬椹卞姩锛堥潪瀛愮郴缁燂級缁戝畾锛氬鏋滀綘瀵硅缁戝畾鎰熷埌婊℃剰锛屼笖鍑犲懆鍚庝粛鏈敹鍒?devicetree 缁存姢鑰呯殑 Acked-by锛岃鐩存帴灏嗗叾鍚堝叆銆?
     瀵逛簬瀛愮郴缁熺粦瀹氾紙褰卞搷澶氫釜璁惧鐨勪换浣曞唴瀹癸級锛屽繀椤昏涓€浣?devicetree 缁存姢鑰呭鍏惰繘琛屽鏌ャ€?
  3) 瀵逛簬缁忚繃澶氭５鏍戠殑琛ヤ竵绯诲垪锛岀粦瀹氳ˉ涓佸簲涓庝娇鐢ㄨ缁戝畾鐨勯┍鍔ㄦ斁鍦ㄤ竴璧枫€?
  4) DTS 鏂囦欢缁濅笉搴旈€氳繃椹卞姩瀛愮郴缁熸爲鍚堝叆锛岃€屽簲濮嬬粓閫氳繃骞冲彴 SoC 鏍戝湪涓撶敤鍒嗘敮涓婂悎鍏ワ紙鍙﹁ `Documentation/process/maintainer-soc.rst`锛夈€?
## III. 娉ㄦ剰浜嬮」


  0) 鍏充簬 devicetree ABI 鐨勭粏鑺傦紝璇峰弬瑙?`Documentation/devicetree/bindings/ABI.rst`銆?
  1) 鏈枃妗ｆ棬鍦ㄤ綔涓哄 2013 骞村唴鏍稿嘲浼氭墍纭畾娴佺▼鐨勬€讳綋鐔熸倝鎸囧紩銆傚鏈夌枒闂紝devicetree 缁存姢鑰呭綋鍓嶇殑鎰忚浼樺厛浜庢湰鏂囨。銆傚湪杩欑鎯呭喌涓嬶紝娆㈣繋鎻愪氦鏇存柊鏈枃妗ｇ殑琛ヤ竵銆?