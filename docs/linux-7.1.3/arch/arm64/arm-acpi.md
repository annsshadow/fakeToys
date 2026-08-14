## Arm 绯荤粺涓婄殑 ACPI


ACPI 鍙敤浜庨伒寰?BSA锛圓rm Base System Architecture锛孉rm 鍩虹绯荤粺鏋舵瀯锛塠^0^] 涓?BBR锛圓rm Base Boot Requirements锛孉rm 鍩虹鍚姩瑕佹眰锛塠^1^] 瑙勮寖璁捐鐨?Armv8 鍜?Armv9 绯荤粺銆侭SA 涓?BBR 鍧囦负鍏紑鍙幏鍙栫殑鏂囨。銆傞櫎绗﹀悎 BSA 澶栵紝Arm 鏈嶅姟鍣ㄨ繕闇€閬靛惊 SBSA锛圫erver Base System Architecture锛屾湇鍔″櫒鍩虹绯荤粺鏋舵瀯锛塠^2^] 涓畾涔夌殑涓€缁勮鍒欍€?

Arm 鍐呮牳瀹炵幇浜?ACPI 5.1 鎴栨洿楂樼増鏈殑绮剧畝纭欢妯″瀷锛坮educed hardware model锛夈€傝瑙勮寖鍙婂叾寮曠敤鐨勬墍鏈夊閮ㄦ枃妗ｇ殑閾炬帴鍧囩敱 UEFI Forum 绠＄悊銆傝鑼冨彲浠?http://www.uefi.org/specifications 鑾峰彇锛岃鑼冨紩鐢ㄧ殑鏂囨。鍙€氳繃 http://www.uefi.org/acpi 鎵惧埌銆?

濡傛灉鏌愪釜 Arm 绯荤粺涓嶆弧瓒?BSA 涓?BBR 鐨勮姹傦紝鎴栬€呮棤娉曠敤鎵€闇€ ACPI 瑙勮寖涓畾涔夌殑鏈哄埗鏉ユ弿杩帮紝閭ｄ箞 ACPI 鍙兘骞朵笉閫傚悎璇ョ‖浠躲€?

灏界涓婅堪鏂囨。瑙勫畾浜嗘瀯寤鸿涓氭爣鍑嗙殑 Arm 绯荤粺鐨勮姹傦紝瀹冧滑鍚屾牱閫傜敤浜庝笉姝竴涓搷浣滅郴缁熴€傛湰鏂囨。鐨勭洰鐨勪粎鍦ㄤ簬鎻忚堪鍦?Arm 绯荤粺涓?ACPI 涓?Linux 涔嬮棿鐨勪氦浜掆€斺€斾篃灏辨槸璇达紝Linux 瀵?ACPI 鏈変綍鏈熸湜锛屼互鍙?ACPI 瀵?Linux 鏈変綍鏈熸湜銆?


### 涓轰綍鍦?Arm 涓婁娇鐢?ACPI锛?

鍦ㄨ€冨療 ACPI 涓?Linux 涔嬮棿鎺ュ彛鐨勮澶氱粏鑺備箣鍓嶏紝鍏堢悊瑙ｄ负浣曡浣跨敤 ACPI 鏄湁鐩婄殑銆傛瘯绔燂紝Linux 涓棭宸插瓨鍦ㄥ绉嶇敤浜庢弿杩颁笉鍙灇涓撅紙non-enumerable锛夌‖浠剁殑鎶€鏈€傛湰鑺傛垜浠鎷簡 Grant Likely 鐨勪竴绡囧崥瀹㈡枃绔?[^3^]锛屽叾涓杩颁簡鍦?Arm 绯荤粺涓婁娇鐢?ACPI 鐨勭紭鐢便€傝€佸疄璇达紝鎴戜滑鍑犱箮鐩存帴鎽樺綍浜嗗叾涓殑澶ч儴鍒嗘€荤粨鏂囧瓧銆?

鍦?Arm 涓婁娇鐢?ACPI 鐨勭畝瑕佺悊鐢卞涓嬶細

- ACPI 鐨勫瓧鑺傜爜锛圓ML锛夊厑璁稿钩鍙板纭欢琛屼负杩涜缂栫爜锛岃€?DT 鏄庣‘涓嶆敮鎸佽繖涓€鐐广€傚纭欢鍘傚晢鑰岃█锛岃兘澶熷琛屼负缂栫爜鏄敮鎸佹柊纭欢涓婃搷浣滅郴缁熷彂甯冪殑鍏抽敭宸ュ叿銆?

- ACPI 鐨?OSPM 瀹氫箟浜嗕竴绉嶇數婧愮鐞嗘ā鍨嬶紝灏嗗钩鍙拌鍏佽鎵ц鐨勬搷浣滅害鏉熷埌鐗瑰畾鐨勬ā鍨嬩腑锛屽悓鏃朵粛涓虹‖浠惰璁′繚鐣欑伒娲绘€с€?

- 鍦ㄤ紒涓氭湇鍔″櫒鐜涓紝ACPI 宸茬粡寤虹珛浜嗕竴濂楃粦瀹氾紙渚嬪鐢ㄤ簬 RAS锛夛紝鐩墠宸插湪鐢熶骇绯荤粺涓娇鐢ㄣ€侱T 娌℃湁銆傝繖绫荤粦瀹氭垨璁稿皢鏉ュ彲浠ュ湪 DT 涓畾涔夛紝浣嗛偅鏍峰仛鎰忓懗鐫€ Arm 涓?x86 鏈€缁堝皢涓嶅緱涓嶅湪鍥轰欢鍜屽唴鏍镐腑閮戒娇鐢ㄥ畬鍏ㄤ笉鍚岀殑浠ｇ爜璺緞銆?

- 閫夋嫨鍗曚竴鎺ュ彛鏉ユ弿杩板钩鍙颁笌鎿嶄綔绯荤粺涔嬮棿鐨勬娊璞℃槸寰堥噸瑕佺殑銆傝嫢纭欢鍘傚晢甯屾湜鏀寔澶氫釜鎿嶄綔绯荤粺锛屼粬浠皢涓嶅繀鍚屾椂瀹炵幇 DT 鍜?ACPI銆傝€屼笖锛屽氨鍗曚竴鎺ュ彛杈炬垚涓€鑷淬€佽€屼笉鏄悇鑷垎瑁傛垚姣忎釜鎿嶄綔绯荤粺涓€濂楁帴鍙ｏ紝鏁翠綋涓婁細甯︽潵鏇村ソ鐨勪簰鎿嶄綔鎬с€?

- 鏂扮殑 ACPI 娌荤悊娴佺▼杩愪綔鑹ソ锛屽浠?Linux 涓庣‖浠跺巶鍟嗗強鍏朵粬鎿嶄綔绯荤粺鍘傚晢鍧愬湪鍚屼竴寮犺皥鍒ゆ涓娿€備簨瀹炰笂锛屽凡娌℃湁浠讳綍鐞嗙敱璁や负 ACPI 鍙睘浜?Windows锛屾垨璁や负 Linux 鍦ㄨ繖涓€棰嗗煙浠庝换浣曟剰涔変笂閮戒綆浜?Microsoft銆侫CPI 娌荤悊鏉冪Щ浜ょ粰 UEFI Forum 鏋佸ぇ鍦板紑鏀句簡瑙勮寖鐨勫紑鍙戞祦绋嬶紝鐩墠瀵?ACPI 鎵€鍋氱殑澶ч噺淇敼姝ｆ槸鐢?Linux 鎺ㄥ姩鐨勩€?

浣跨敤 ACPI 鐨勫叧閿湪浜庡叾鏀寔妯″瀷銆傚浜庢湇鍔″櫒鎬讳綋鑰岃█锛岀‖浠惰涓虹殑璐ｄ换涓嶈兘浠呯敱鍐呮牳鎵挎媴锛岃€屽繀椤诲湪骞冲彴涓庡唴鏍镐箣闂村垎鎷咃紝浠ヤ究鑳藉鏈夊簭鍦伴殢鏃堕棿婕旇繘銆侫CPI 浣挎搷浣滅郴缁熸棤闇€鐞嗚В纭欢鐨勬墍鏈夌粏寰粏鑺傦紝浠庤€屼笉蹇呴拡瀵规瘡涓澶囬€愪竴杩涜绉绘銆傚畠璁╃‖浠跺巶鍟嗚兘澶熸壙鎷呯數婧愮鐞嗚涓虹殑璐ｄ换锛岃€屾棤闇€渚濊禆鍏舵棤娉曟帶鍒剁殑鎿嶄綔绯荤粺鍙戝竷鍛ㄦ湡銆?

ACPI 涔嬫墍浠ラ噸瑕侊紝杩樺洜涓虹‖浠朵笌鎿嶄綔绯荤粺鍘傚晢宸茬粡鎽哥储鍑轰簡鏀拺閫氱敤璁＄畻鐢熸€佺殑鏈哄埗銆傜浉鍏冲熀纭€璁炬柦宸插氨浣嶏紝缁戝畾宸插氨浣嶏紝娴佺▼涔熷凡灏变綅銆傚湪澶勭悊鍨傜洿鏁村悎鐨勮澶囨椂锛孌T 鎭板ソ瀹屾垚浜?Linux 鎵€闇€鐨勫伐浣滐紝浣嗗苟娌℃湁鑹ソ鐨勬祦绋嬫潵鏀拺鏈嶅姟鍣ㄥ巶鍟嗙殑闇€姹傘€侺inux 鎴栬鏈€缁堣兘鐢?DT 鍋氬埌锛屼絾閭ｆ牱鍋氬疄闄呬笂鍙槸閲嶅涓€涓凡缁忓彲琛岀殑涓滆タ銆侫CPI 宸茬粡瀹炵幇浜嗙‖浠跺巶鍟嗘墍闇€鐨勫姛鑳斤紝Microsoft 涓嶄細鍦?DT 涓婂悎浣滐紝鑰岀‖浠跺巶鍟嗘渶缁堜粛灏嗕笉寰椾笉鎻愪緵涓ゅ瀹屽叏鐙珛鐨勫浐浠舵帴鍙ｂ€斺€斾竴濂楃粰 Linux锛屼竴濂楃粰 Windows銆?


### 鍐呮牳鍏煎鎬?

閲囩敤 ACPI 鐨勪富瑕佸姩鏈轰箣涓€鏄爣鍑嗗寲锛屽苟鍊熸涓?Linux 鍐呮牳鎻愪緵鍚戝悗鍏煎鎬с€傚湪鏈嶅姟鍣ㄥ競鍦猴紝杞‖浠跺父琚暱鏈熶娇鐢ㄣ€侫CPI 璁╁唴鏍镐笌鍥轰欢灏变竴涓竴鑷寸殑鎶借薄杈炬垚涓€鑷达紝璇ユ娊璞″嵆浣跨‖浠舵垨杞欢鍙戠敓鍙樺寲涔熻兘闀挎湡缁存姢銆傚彧瑕佽鎶借薄浠嶅彈鏀寔锛岀郴缁熷氨鍙互鍦ㄤ笉蹇呴』鏇存崲鍐呮牳鐨勬儏鍐典笅杩涜鏇存柊銆?

褰撲竴涓?Linux 椹卞姩鎴栧瓙绯荤粺棣栨鍩轰簬 ACPI 瀹炵幇鏃讹紝瀹冨繀鐒堕渶瑕佺壒瀹氱増鏈殑 ACPI 瑙勮寖鈥斺€斿嵆鍏跺熀绾跨増鏈€侫CPI 鍥轰欢蹇呴』缁х画宸ヤ綔锛屽嵆渚垮彲鑳戒笉鏄渶浼樼殑锛屼篃瑕佽兘閰嶅悎鏈€鏃╁紑濮嬫敮鎸佽鍩虹嚎 ACPI 鐗堟湰鐨勫唴鏍哥増鏈€傚彲鑳介渶瑕侀澶栫殑椹卞姩锛屼絾娣诲姞鏂板姛鑳斤紙渚嬪 CPU 鐢垫簮绠＄悊锛変笉搴旂牬鍧忔棫鍐呮牳鐗堟湰銆傛澶栵紝ACPI 鍥轰欢杩樺繀椤昏兘閰嶅悎鏈€鏂扮増鏈殑鍐呮牳宸ヤ綔銆?


### 涓?Device Tree 鐨勫叧绯?

鍦?Arm 鐨勯┍鍔ㄤ笌瀛愮郴缁熶腑锛孉CPI 鏀寔涓?DT 鏀寔鍦ㄧ紪璇戞椂缁濅笉搴斿綋浜掓枼銆?

鍦ㄥ惎鍔ㄦ椂锛屽唴鏍稿彧浼氭牴鎹紩瀵煎姞杞界▼搴忥紙鍖呮嫭鍐呮牳 bootargs锛変紶鍏ョ殑鍙傛暟浣跨敤涓€绉嶆弿杩版柟娉曘€?

鏃犺浣跨敤 DT 杩樻槸 ACPI锛屽唴鏍稿繀椤诲缁堣兘澶熺敤杩欎袱绉嶆柟妗堜腑鐨勪换鎰忎竴绉嶅惎鍔紙鍦ㄧ紪璇戞椂鍚屾椂鍚敤涓ょ鏂规鐨勫唴鏍镐腑锛夈€?


### 浣跨敤 ACPI 琛ㄥ惎鍔?

鍦?Arm 涓婂悜鍐呮牳浼犻€?ACPI 琛ㄧ殑鍞竴瀹氫箟鏂瑰紡鏄€氳繃 UEFI 绯荤粺閰嶇疆琛ㄣ€傛槑纭湴璇达紝杩欐剰鍛崇潃 ACPI 浠呭湪浣跨敤 UEFI 鍚姩鐨勫钩鍙颁笂鍙楁敮鎸併€?

褰?Arm 绯荤粺鍚姩鏃讹紝瀹冨彲鑳芥湁 DT 淇℃伅銆丄CPI 琛紝鎴栬€呭湪鏋佸皯鏁版儏鍐典笅涓よ€呴兘鏈夈€傚鏋滀笉浣跨敤浠讳綍鍛戒护琛屽弬鏁帮紝鍐呮牳灏嗗皾璇曚娇鐢?DT 杩涜璁惧鏋氫妇锛涘鏋滄病鏈?DT锛屽唴鏍稿皢灏濊瘯浣跨敤 ACPI 琛紝浣嗕粎褰撳畠浠瓨鍦ㄦ椂銆傚鏋滀袱鑰呴兘涓嶅彲鐢紝鍐呮牳灏嗘棤娉曞惎鍔ㄣ€傚鏋滃湪鍛戒护琛屼娇鐢?acpi=force锛屽唴鏍稿皢棣栧厛灏濊瘯浣跨敤 ACPI 琛紝浣嗗鏋滄病鏈?ACPI 琛ㄥ瓨鍦ㄥ垯鍥為€€鍒?DT銆傚叾鍩烘湰鐞嗗康鏄細闄ら潪纭疄鍒棤閫夋嫨锛屽唴鏍镐笉浼氬惎鍔ㄥけ璐ャ€?

閫氳繃鍦ㄥ唴鏍稿懡浠よ浼犲叆 acpi=off 鍙互绂佺敤 ACPI 琛ㄧ殑澶勭悊锛涜繖鏄粯璁よ涓恒€?

涓轰簡璁╁唴鏍稿姞杞藉苟浣跨敤 ACPI 琛紝UEFI 瀹炵幇蹇呴』璁剧疆 ACPI_20_TABLE_GUID 鎸囧悜 RSDP 琛紙甯︽湁 ACPI 绛惧悕 "RSD PTR " 鐨勮〃锛夈€傚鏋滆鎸囬拡涓嶆纭笖浣跨敤浜?acpi=force锛屽唴鏍稿皢绂佺敤 ACPI 骞跺皾璇曟敼鐢?DT 鍚姩锛涘疄闄呬笂锛屽唴鏍告鏃跺凡鍒ゅ畾 ACPI 琛ㄤ笉瀛樺湪銆?

濡傛灉鎸囧悜 RSDP 琛ㄧ殑鎸囬拡姝ｇ‘锛孉CPI 鏍稿績灏嗕娇鐢?UEFI 鎻愪緵鐨勫湴鍧€灏嗚琛ㄦ槧灏勫埌鍐呮牳涓€?

闅忓悗锛孉CPI 鏍稿績浼氬埄鐢?RSDP 琛ㄤ腑鐨勫湴鍧€鎵惧埌 XSDT锛坋Xtended System Description Table锛屾墿灞曠郴缁熸弿杩拌〃锛夛紝杩涜€屽畾浣嶅苟鏄犲皠鎵€鏈夊叾浠?ACPI 琛ㄣ€俋SDT 鍙嶈繃鏉ユ彁渚涗簡绯荤粺鍥轰欢鎻愪緵鐨勬墍鏈夊叾浠?ACPI 琛ㄧ殑鍦板潃锛汚CPI 鏍稿績闅忓悗浼氶亶鍘嗚琛ㄥ苟鏄犲皠鍏朵腑鍒楀嚭鐨勮〃銆?

ACPI 鏍稿績浼氬拷鐣ヤ换浣曟彁渚涚殑 RSDT锛圧oot System Description Table锛屾牴绯荤粺鎻忚堪琛級銆俁SDT 宸茶寮冪敤锛屽湪 arm64 涓婁細琚拷鐣ワ紝鍥犱负瀹冧滑鍙敮鎸?32 浣嶅湴鍧€銆?

姝ゅ锛孉CPI 鏍稿績鍙細浣跨敤 FADT锛團ixed ACPI Description Table锛屽浐瀹?ACPI 鎻忚堪琛級涓殑 64 浣嶅湴鍧€瀛楁銆侳ADT 涓殑浠讳綍 32 浣嶅湴鍧€瀛楁鍦?arm64 涓婇兘浼氳蹇界暐銆?

纭欢绮剧畝妯″紡锛坔ardware reduced mode锛屽弬瑙?ACPI 6.1 瑙勮寖绗?4.1 鑺傦級灏嗙敱 ACPI 鏍稿績鍦?arm64 涓婂己鍒跺惎鐢ㄣ€傝繖鏍峰仛鍙互璁?ACPI 鏍稿績杩愯鏇寸畝鍗曠殑浠ｇ爜锛屽洜涓哄畠涓嶅啀闇€瑕佷负鍏朵粬鏋舵瀯鐨勬棫纭欢鎻愪緵鏀寔銆備换浣曚笉鐢ㄤ簬纭欢绮剧畝妯″紡鐨勫瓧娈甸兘蹇呴』璁剧疆涓洪浂銆?

涓轰簡璁?ACPI 鏍稿績姝ｇ‘杩愯锛岃繘鑰屾彁渚涘唴鏍搁厤缃澶囨墍闇€鐨勪俊鎭紝瀹冮渶瑕佹壘鍒颁互涓嬭〃锛堟墍鏈夌珷鑺傚彿鍧囨寚 ACPI 6.5 瑙勮寖锛夛細

    - RSDP锛圧oot System Description Pointer锛屾牴绯荤粺鎻忚堪鎸囬拡锛夛紝绗?5.2.5 鑺?

    - XSDT锛坋Xtended System Description Table锛屾墿灞曠郴缁熸弿杩拌〃锛夛紝绗?5.2.8 鑺?

    - FADT锛團ixed ACPI Description Table锛屽浐瀹?ACPI 鎻忚堪琛級锛岀 5.2.9 鑺?

    - DSDT锛圖ifferentiated System Description Table锛屽樊寮傚寲绯荤粺鎻忚堪琛級锛岀
       5.2.11.1 鑺?

    - MADT锛圡ultiple APIC Description Table锛屽 APIC 鎻忚堪琛級锛岀 5.2.12 鑺?

    - GTDT锛圙eneric Timer Description Table锛岄€氱敤瀹氭椂鍣ㄦ弿杩拌〃锛夛紝绗?5.2.24 鑺?

    - PPTT锛圥rocessor Properties Topology Table锛屽鐞嗗櫒灞炴€ф嫇鎵戣〃锛夛紝绗?5.2.30 鑺?

    - DBG2锛圖eBuG port table 2锛岃皟璇曠鍙ｈ〃 2锛夛紝绗?5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?

    - APMT锛圓rm Performance Monitoring unit Table锛孉rm 鎬ц兘鐩戞帶鍗曞厓琛級锛岀 5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?

    - AGDI锛圓rm Generic diagnostic Dump and Reset Device Interface Table锛孉rm 閫氱敤璇婃柇杞偍涓庡浣嶈澶囨帴鍙ｈ〃锛夛紝绗?5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?

    - 濡傛灉鏀寔 PCI锛屽垯 MCFG锛圡emory mapped ConFiGuration Table锛屽唴瀛樻槧灏勯厤缃〃锛夛紝绗?5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?

    - 濡傛灉鏀寔鍦ㄤ笉甯?console=<device> 鍐呮牳鍙傛暟鐨勬儏鍐典笅鍚姩锛屽垯 SPCR锛圫erial Port Console Redirection table锛屼覆鍙ｆ帶鍒跺彴閲嶅畾鍚戣〃锛夛紝绗?5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?

    - 濡傛灉涓烘弿杩?I/O 鎷撴墤銆丼MMUs 鍜?GIC ITSs 鎵€蹇呴渶锛屽垯 IORT锛圛nput Output Remapping Table锛岃緭鍏ヨ緭鍑洪噸鏄犲皠琛級锛岀 5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?

    - 濡傛灉鏀寔 NUMA锛屽垯杩橀渶瑕佷互涓嬭〃锛?

       - SRAT锛圫ystem Resource Affinity Table锛岀郴缁熻祫婧愪翰鍜屾€ц〃锛夛紝绗?5.2.16 鑺?

       - SLIT锛圫ystem Locality distance Information Table锛岀郴缁熷眬閮ㄨ窛绂讳俊鎭〃锛夛紝绗?5.2.17 鑺?

    - 濡傛灉鏀寔 NUMA锛屼笖绯荤粺鍖呭惈寮傛瀯鍐呭瓨锛屽垯 HMAT锛圚eterogeneous Memory Attribute Table锛屽紓鏋勫唴瀛樺睘鎬ц〃锛夛紝绗?5.2.28 鑺傘€?

    - 濡傛灉闇€瑕?ACPI Platform Error Interfaces锛屽垯浠ヤ笅鏉′欢鎬у湴闇€瑕佷互涓嬭〃锛?

       - BERT锛圔oot Error Record Table锛屽惎鍔ㄩ敊璇褰曡〃锛夛紝绗?18.3.1 鑺?

       - EINJ锛圗rror INJection table锛岄敊璇敞鍏ヨ〃锛夛紝绗?18.6.1 鑺?

       - ERST锛圗rror Record Serialization Table锛岄敊璇褰曞簭鍒楀寲琛級锛岀 18.5 鑺?

       - HEST锛圚ardware Error Source Table锛岀‖浠堕敊璇簮琛級锛岀 18.3.2 鑺?

       - SDEI锛圫oftware Delegated Exception Interface table锛岃蒋浠跺鎵樺紓甯告帴鍙ｈ〃锛夛紝绗?5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6

       - AEST锛圓rm Error Source Table锛孉rm 閿欒婧愯〃锛夛紝绗?5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6

       - RAS2锛圓CPI RAS2 feature table锛孉CPI RAS2 鐗规€ц〃锛夛紝绗?5.2.21 鑺?

    - 濡傛灉绯荤粺鍖呭惈浣跨敤 PCC 閫氶亾鐨勬帶鍒跺櫒锛屽垯 PCCT锛圥latform Communications Channel Table锛屽钩鍙伴€氫俊閫氶亾琛級锛岀 14.1 鑺?

    - 濡傛灉绯荤粺鍖呭惈鐢ㄤ簬鎹曡幏鏉跨骇绯荤粺鐘舵€併€佸苟閫氳繃 PCC 涓庝富鏈洪€氫俊鐨勬帶鍒跺櫒锛屽垯 PDTT锛圥latform Debug Trigger Table锛屽钩鍙拌皟璇曡Е鍙戣〃锛夛紝绗?5.2.29 鑺傘€?

    - 濡傛灉鏀寔 NVDIMM锛屽垯 NFIT锛圢VDIMM Firmware Interface Table锛孨VDIMM 鍥轰欢鎺ュ彛琛級锛岀 5.2.26 鑺?

    - 濡傛灉瀛樺湪瑙嗛甯х紦鍐诧紝鍒?BGRT锛圔oot Graphics Resource Table锛屽惎鍔ㄥ浘褰㈣祫婧愯〃锛夛紝绗?5.2.23 鑺?

    - 濡傛灉瀹炵幇浜?IPMI锛屽垯 SPMI锛圫erver Platform Management Interface锛屾湇鍔″櫒骞冲彴绠＄悊鎺ュ彛锛夛紝绗?5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?

    - 濡傛灉绯荤粺鍖呭惈 CXL Host Bridge锛屽垯 CEDT锛圕XL Early Discovery Table锛孋XL 鏃╂湡鍙戠幇琛級锛岀 5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?

    - 濡傛灉绯荤粺鏀寔 MPAM锛屽垯 MPAM锛圡emory Partitioning And Monitoring table锛屽唴瀛樺垎鍖轰笌鐩戣琛級锛岀 5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?

    - 濡傛灉绯荤粺缂哄皯鎸佷箙鍖栧瓨鍌紝鍒?IBFT锛圛SCSI Boot Firmware Table锛宨SCSI 鍚姩鍥轰欢琛級锛岀 5.2.6 鑺傦紝鍏蜂綋涓鸿〃 5-6銆?


濡傛灉涓婅堪琛ㄦ湭鍏ㄩ儴瀛樺湪锛屽唴鏍稿彲鑳借兘澶熶篃鍙兘鏃犳硶姝ｅ父鍚姩锛屽洜涓哄畠鍙兘鏃犳硶閰嶇疆鎵€鏈夊彲鐢ㄨ澶囥€傛琛ㄦ竻鍗曞苟闈炴棬鍦ㄥ寘缃椾竾璞★紱鍦ㄦ煇浜涚幆澧冧腑锛屽彲鑳介渶瑕佸叾浠栬〃锛堜緥濡傜 18 鑺備腑鐨勪换浣?APEI 琛級鏉ユ敮鎸佺壒瀹氬姛鑳姐€?


### ACPI 妫€娴?

椹卞姩搴旈€氳繃妫€鏌?ACPI_HANDLE 鏄惁涓虹┖鍊硷紝鎴栨鏌?.of_node锛屾垨妫€鏌ヨ澶囩粨鏋勪綋涓殑鍏朵粬淇℃伅锛屾潵纭畾鍏?probe() 绫诲瀷銆傝繖涓€鐐瑰湪鈥滈┍鍔ㄥ缓璁€濅竴鑺備腑鏈夋洿璇︾粏鐨勮鏄庛€?

鍦ㄩ潪椹卞姩浠ｇ爜涓紝濡傛灉闇€瑕佸湪杩愯鏃舵娴?ACPI 鏄惁瀛樺湪锛屽垯妫€鏌?acpi_disabled 鐨勫€笺€傚鏋滄湭璁剧疆 CONFIG_ACPI锛宎cpi_disabled 灏嗗缁堜负 1銆?


### 璁惧鏋氫妇

ACPI 涓殑璁惧鎻忚堪搴斾娇鐢ㄦ爣鍑嗕笖琚鍙殑 ACPI 鎺ュ彛銆傝繖浜涙弿杩版墍鍖呭惈鐨勪俊鎭彲鑳藉皯浜庨€氬父閫氳繃 Device Tree 涓哄悓涓€璁惧鎻愪緵鐨勪俊鎭€傝繖涔熸槸 ACPI 涔嬫墍浠ユ湁鐢ㄧ殑鍘熷洜涔嬩竴鈥斺€旈┍鍔ㄤ細鑰冭檻鍒板畠鍙兘鎷ユ湁鍏充簬璇ヨ澶囩殑杈冨皯璇︾粏淇℃伅锛岃浆鑰屼娇鐢ㄥ悎鐞嗙殑榛樿鍊笺€傚鏋滃湪椹卞姩涓Ε鍠勫鐞嗭紝纭欢鍙互闅忕潃鏃堕棿鐨勬帹绉昏€屽彉鏇村拰鏀硅繘锛岃€岄┍鍔ㄥ畬鍏ㄦ棤闇€鏀瑰彉銆?

鏃堕挓灏辨槸涓€涓瀬濂界殑渚嬪瓙銆傚湪 DT 涓紝鏃堕挓闇€瑕佽鏄惧紡鎸囧畾锛岄┍鍔ㄤ篃闇€瑕佸皢鍏惰€冭檻鍦ㄥ唴銆傚湪 ACPI 涓紝鍋囪鏄?UEFI 浼氬皢璁惧鐣欏湪鍚堢悊鐨勯粯璁ょ姸鎬侊紝鍖呮嫭浠讳綍鏃堕挓璁剧疆銆傚鏋滅敱浜庢煇绉嶅師鍥犻┍鍔ㄩ渶瑕佹敼鍙樻煇涓椂閽熷€硷紝杩欏彲浠ラ€氳繃鏌愪釜 ACPI method 鏉ュ畬鎴愶紱椹卞姩鍙渶璋冪敤璇ユ柟娉曪紝鑰屾棤闇€鍏冲績璇ユ柟娉曚负浜嗘敼鍙樻椂閽熼渶瑕佸仛浠€涔堛€傚姝や竴鏉ワ紝纭欢鐨勫彉鏇村氨鍙互闅忕潃鏃堕棿鎺ㄧЩ閫氳繃鏀瑰彉 ACPI method 鐨勮涓烘潵瀹炵幇锛岃€屼笉鏄敼鍙橀┍鍔ㄣ€?

鍦?DT 涓紝椹卞姩涓鸿缃笂杩版椂閽熸墍闇€鐨勫弬鏁拌绉颁负鈥渂indings锛堢粦瀹氾級鈥濓紱鍦?ACPI 涓紝杩欎簺琚О涓衡€淒evice Properties锛堣澶囧睘鎬э級鈥濓紝骞堕€氳繃 _DSD 瀵硅薄鎻愪緵缁欓┍鍔ㄣ€?

ACPI 琛ㄧ敤涓€绉嶇О涓?ASL锛圓CPI Source Language锛孉CPI 婧愯瑷€锛岃鑼冪 19 鑺傦級鐨勫舰寮忓寲璇█鏉ユ弿杩般€傝繖鎰忓懗鐫€鎬绘湁澶氱鏂瑰紡鏉ユ弿杩板悓涓€浜嬬墿鈥斺€斿寘鎷澶囧睘鎬с€備緥濡傦紝璁惧灞炴€у彲浠ヤ娇鐢ㄥ涓嬪舰寮忕殑 ASL 鏋勯€狅細Name(KEY0, "value0")銆傞殢鍚庯紝ACPI 璁惧椹卞姩浼氶€氳繃姹傚€?KEY0 瀵硅薄鏉ヨ幏鍙栬灞炴€х殑鍊笺€傜劧鑰岋紝浠ヨ繖绉嶆柟寮忎娇鐢?Name() 瀛樺湪澶氫釜闂锛?1) 涓?DT 涓嶅悓锛孉CPI 灏嗗悕绉帮紙"KEY0"锛夐檺鍒朵负鍥涗釜瀛楃锛?2) 娌℃湁涓氱晫鑼冨洿鐨勬敞鍐岃〃鏉ョ淮鎶ゅ悕绉板垪琛紝闅句互澶嶇敤锛?3) 鍚屾牱涔熸病鏈夐拡瀵瑰睘鎬у€硷紙"value0"锛夊畾涔夌殑娉ㄥ唽琛紝鍚屾牱浣垮鐢ㄥ彉寰楀洶闅撅紱(4) 褰撴柊纭欢鍑虹幇鏃讹紝濡備綍淇濇寔鍚戝悗鍏煎鎬э紵_DSD 鏂规硶姝ｆ槸涓鸿В鍐虫绫婚棶棰樿€屽垱寤虹殑锛汱inux 椹卞姩搴斿缁堝璁惧灞炴€т娇鐢?_DSD 鏂规硶锛岃€屼笉浣跨敤鍏朵粬浠讳綍鏂规硶銆?

_DSM 瀵硅薄锛圓CPI 绗?9.14.1 鑺傦級涔熷彲鐢ㄤ簬鍚戦┍鍔ㄤ紶閫掕澶囧睘鎬с€侺inux 椹卞姩搴斾粎鍦?_DSD 鏃犳硶琛ㄧず鎵€闇€鏁版嵁銆佷笖鏃犳硶涓?_DSD 瀵硅薄鍒涘缓鏂?UUID 鏃舵墠鏈熸湜浣跨敤瀹冦€傛敞鎰忥紝瀵?_DSM 浣跨敤鐨勮鑼冪敋鑷冲皯浜庡 _DSD 鐨勮鑼冦€傛鍥犲姝わ紝渚濊禆 _DSM 瀵硅薄鍐呭鐨勯┍鍔ㄥ湪灏嗘潵浼氭洿闅剧淮鎶わ紱鍦ㄦ挵鍐欐湰鏂囨椂锛宊DSM 鐨勪娇鐢ㄦ鏄浉褰撳鍥轰欢闂鐨勬垚鍥狅紝鍥犳涓嶅缓璁娇鐢ㄣ€?

椹卞姩搴斾粎鍦?_DSD 瀵硅薄涓煡鎵捐澶囧睘鎬э紱_DSD 瀵硅薄鍦?ACPI 瑙勮寖绗?6.2.5 鑺備腑鎻忚堪锛屼絾璇ヨ妭浠呮弿杩颁簡濡備綍瀹氫箟閫氳繃 _DSD 杩斿洖鐨勫璞＄殑缁撴瀯锛屼互鍙婄壒瀹氭暟鎹粨鏋勫浣曠敱鐗瑰畾 UUID 瀹氫箟銆侺inux 搴斿彧浣跨敤 _DSD Device Properties UUID [^4^]锛?

   - UUID: daffd814-6eba-4d8c-8a91-bc9bbf4aa301

甯歌鐨勮澶囧睘鎬у彲浠ラ€氳繃鍚?[^4^] 鍒涘缓 pull request 鏉ユ敞鍐岋紝浠ヤ究瀹冧滑鑳藉湪鎵€鏈夋敮鎸?ACPI 鐨勬搷浣滅郴缁熶腑浣跨敤銆傛湭鍚?UEFI Forum 娉ㄥ唽鐨勮澶囧睘鎬т篃鍙互浣跨敤锛屼絾涓嶈兘浣滀负 "uefi-" 鍏叡灞炴€с€?

鍦ㄥ垱寤烘柊鐨勮澶囧睘鎬т箣鍓嶏紝璇峰厛纭瀹冧滑姝ゅ墠鏈瀹氫箟锛屼篃鏈娉ㄥ唽鍒?Linux 鍐呮牳鏂囨。涓綔涓?DT 缁戝畾锛屾垨娉ㄥ唽鍒?UEFI Forum 浣滀负璁惧灞炴€с€傝櫧鐒舵垜浠苟涓嶆兂绠€鍗曞湴鎶婃墍鏈夌殑 DT 缁戝畾閮芥惉杩?ACPI 璁惧灞炴€э紝浣嗘垜浠彲浠ヤ粠涔嬪墠瀹氫箟鐨勫唴瀹逛腑瀛︿範銆?

濡傛灉蹇呴』瀹氫箟鏂扮殑璁惧灞炴€э紝鎴栬€呭皢鏌愪釜缁戝畾鐨勫畾涔夌患鍚堟暣鐞嗕娇鍏跺湪浠绘剰鍥轰欢涓兘鑳戒娇鐢ㄦ槸鏈夋剰涔夌殑锛岄偅涔?DT 缁戝畾鍜?ACPI 璁惧灞炴€ч拡瀵硅澶囬┍鍔ㄩ兘鏈夊悇鑷殑瀹℃煡娴佺▼銆備袱鑰呴兘瑕佷娇鐢ㄣ€傚綋椹卞姩鏈韩鎻愪氦鍒?Linux 閭欢鍒楄〃瀹℃煡鏃讹紝鎵€闇€鐨勮澶囧睘鎬у畾涔夊繀椤诲悓鏃舵彁浜ゃ€備竴涓敮鎸?ACPI 骞朵娇鐢ㄨ澶囧睘鎬х殑椹卞姩锛岃嫢鏃犲叾瀹氫箟锛屽皢涓嶈瑙嗕负瀹屾暣銆備竴鏃﹁澶囧睘鎬ц Linux 绀惧尯鎺ュ彈锛屽氨蹇呴』鍚?UEFI Forum [^4^] 娉ㄥ唽锛屽悗鑰呬細鍐嶆瀹℃煡鍏跺湪娉ㄥ唽琛ㄤ腑鐨勪竴鑷存€с€傝繖鍙兘闇€瑕佸弽澶嶈凯浠ｃ€備笉杩囷紝UEFI Forum 濮嬬粓鏄澶囧睘鎬у畾涔夌殑鏉冨▉绔欑偣銆?

鍚?UEFI Forum 鍙戝嚭閫氱煡锛岃〃鏄庢湁鎰忔敞鍐屼竴涓鍓嶆湭浣跨敤鐨勮澶囧睘鎬у悕绉帮紝浠ユ浣滀负涓哄皢鏉ヤ娇鐢ㄤ繚鐣欒鍚嶇О鐨勬墜娈碉紝杩欑鍋氭硶鎴栬鏄湁鎰忎箟鐨勩€傚叾浠栨搷浣滅郴缁熷巶鍟嗕篃浼氭彁浜ゆ敞鍐岃姹傦紝杩欏彲鑳芥湁鍔╀簬璁╂祦绋嬫洿鍔犻『鐣呫€?

涓€鏃︽敞鍐屼笌瀹℃煡瀹屾垚锛屽唴鏍镐細鎻愪緵涓€涓帴鍙ｏ紝浠ヤ笌 DT 鎴?ACPI 鏄惁姝ｅ湪浣跨敤鏃犲叧鐨勬柟寮忔潵鏌ユ壘璁惧灞炴€с€傚簲褰撲娇鐢ㄦ API [^5^]锛涘畠鍙互娑堥櫎椹卞姩鎺㈡祴鍑芥暟涓竴浜涗唬鐮佽矾寰勭殑閲嶅锛屽苟鎶戝埗 DT 缁戝畾涓?ACPI 璁惧灞炴€т箣闂寸殑鍒嗗寲銆?


### 鍙紪绋嬬數婧愭帶鍒惰祫婧?

鍙紪绋嬬數婧愭帶鍒惰祫婧愬寘鎷濡傜數鍘?鐢垫祦鎻愪緵鏂癸紙regulators锛岀ǔ鍘嬪櫒锛夊拰鏃堕挓婧愮瓑璧勬簮銆?

鍦ㄤ娇鐢?ACPI 鏃讹紝鍐呮牳鐨?clock 涓?regulator 妗嗘灦棰勬湡瀹屽叏涓嶄細琚娇鐢ㄣ€?

鍐呮牳鍋囪杩欎簺璧勬簮鐨勭數婧愭帶鍒剁敱 Power Resource Objects锛堢數婧愯祫婧愬璞★紝ACPI 绗?7.1 鑺傦級琛ㄧず銆傞殢鍚?ACPI 鏍稿績浼氭纭鐞嗗湪闇€瑕佹椂瀵硅祫婧愮殑鍚敤涓庣鐢ㄣ€備负浜嗕娇涔嬪伐浣滐紝ACPI 鍋囪姣忎釜璁惧閮藉畾涔変簡 D-states锛屽苟涓斿彲浠ラ€氳繃鍙€夌殑 ACPI 鏂规硶 _PS0銆乢PS1銆乢PS2 鍜?_PS3 鏉ユ帶鍒讹紱鍦?ACPI 涓紝_PS0 鏄娇璁惧瀹屽叏寮€鍚墍璋冪敤鐨勬柟娉曪紝鑰?_PS3 鏄娇璁惧瀹屽叏鍏抽棴鎵€璋冪敤鐨勬柟娉曘€?

浣跨敤杩欎簺 Power Resources 鏈変袱绉嶉€夋嫨銆傚畠浠彲浠ワ細

   - 鍦?_PSx 鏂规硶涓鐞嗭紝璇ユ柟娉曞湪杩涘叆鐢垫簮鐘舵€?Dx 鏃惰璋冪敤銆?

   - 浣滀负鐙珛鐨勭數婧愯祫婧愬０鏄庯紝骞舵嫢鏈夊悇鑷殑 _ON 鍜?_OFF 鏂规硶銆傞殢鍚庡畠浠€氳繃 _PRx 鍏宠仈鍥炵壒瀹氳澶囩殑 D-states锛宊PRx 鎸囧畾璁惧鍦?Dx 鐘舵€佷笅闇€瑕佷繚鎸佸紑鍚殑鐢垫簮璧勬簮銆傚唴鏍搁殢鍚庤窡韪娇鐢ㄦ煇涓數婧愯祫婧愮殑璁惧鏁伴噺锛屽苟鎸夐渶璋冪敤 _ON/_OFF銆?

鍐呮牳 ACPI 浠ｇ爜杩樹細鍋囪 _PSx 鏂规硶閬靛惊姝ょ被鏂规硶鐨勬甯?ACPI 瑙勫垯锛?

   - 濡傛灉瀹炵幇浜?_PS0 鎴?_PS3 涓殑浠绘剰涓€涓紝閭ｄ箞鍙︿竴涓柟娉曚篃蹇呴』琚疄鐜般€?

   - 濡傛灉璁惧鍦ㄥ紑鍚椂闇€瑕佺敤鍒版垨璁剧疆鏌愪釜鐢垫簮璧勬簮锛孉SL 搴斿綋纭繚鍦?_PS0 鏂规硶涓垎閰?鍚敤瀹冦€?

   - 鍦?_PS0 鏂规硶涓垎閰嶆垨鍚敤鐨勮祫婧愶紝搴斿湪 _PS3 鏂规硶涓绂佺敤鎴栭噴鏀俱€?

   - 鍥轰欢鍦ㄥ皢鎺у埗鏉冧氦缁欏唴鏍镐箣鍓嶏紝浼氬皢璧勬簮淇濇寔鍦ㄥ悎鐞嗙殑鐘舵€併€?

褰撶劧锛宊PSx 鏂规硶涓殑杩欑被浠ｇ爜浼氶潪甯稿钩鍙扮浉鍏炽€備絾鏄紝杩欏彲浠ヨ椹卞姩灏嗘搷浣滆澶囩殑鎺ュ彛鎶借薄鍑烘潵锛岄伩鍏嶅繀椤讳粠 ACPI 琛ㄤ腑璇诲彇鐗规畩鐨勯潪鏍囧噯鍊笺€傛澶栵紝灏嗚繖浜涜祫婧愮殑浣跨敤鎶借薄鍖栵紝鍙互璁╃‖浠堕殢鏃堕棿鏀瑰彉鑰屾棤闇€鏇存柊椹卞姩銆?


### 鏃堕挓

ACPI 鍋囪鏃堕挓鍦ㄦ帶鍒舵潈绉讳氦缁欏唴鏍镐箣鍓嶏紝宸茬敱鍥轰欢锛堝湪姝ゅ嵆 UEFI锛夊垵濮嬪寲涓烘煇涓彲鐢ㄧ殑鍊笺€傝繖瀵逛簬璇稿 UART 鎴?SoC 椹卞姩鐨?LCD 鏄剧ず灞忕瓑璁惧鍏锋湁褰卞搷銆?

鍐呮牳鍚姩鏃讹紝鍋囪鏃堕挓宸茶璁剧疆涓哄悎鐞嗙殑鍙敤鍊笺€傚鏋滅敱浜庢煇绉嶅師鍥犻渶瑕佹敼鍙橀鐜団€斺€斾緥濡備负浜嗙數婧愮鐞嗚€岃妭娴佲€斺€旇澶囬┍鍔ㄥ簲棰勬湡璇ヨ繃绋嬭鎶借薄鍒版煇涓彲琚皟鐢ㄧ殑 ACPI method 涓紙鍏充簬鏈熸湜浣跨敤鐨勬爣鍑嗘柟娉曠殑杩涗竴姝ュ缓璁紝璇峰弬闃?ACPI 瑙勮寖锛夈€傚敮涓€鐨勪緥澶栨槸 CPU 鏃堕挓锛屽叾涓?CPPC 鎻愪緵浜嗘瘮 ACPI 鏂规硶涓板瘜寰楀鐨勬帴鍙ｃ€傚鏋滄椂閽熸湭琚缃紝Linux 娌℃湁鐩存帴鐨勬柟寮忔潵鎺у埗瀹冧滑銆?

濡傛灉鏌愪釜 SoC 鍘傚晢甯屾湜鎻愪緵瀵圭郴缁熸椂閽熺殑缁嗙矑搴︽帶鍒讹紝浠栦滑鍙互閫氳繃鎻愪緵鍙 Linux 椹卞姩璋冪敤鐨?ACPI method 鏉ュ疄鐜般€傜劧鑰岋紝杩欏苟涓嶈鎺ㄨ崘锛孡inux 椹卞姩涓嶅簲浣跨敤姝ょ被鏂规硶锛屽嵆浣垮畠浠鎻愪緵浜嗐€傛绫绘柟娉曠洰鍓嶅湪 ACPI 瑙勮寖涓皻鏈爣鍑嗗寲锛屼娇鐢ㄥ畠浠彲鑳芥妸鍐呮牳缁戝畾鍒版煇涓潪甯哥壒瀹氱殑 SoC锛屾垨鑰呮妸 SoC 缁戝畾鍒版煇涓潪甯哥壒瀹氱殑鍐呮牳鐗堟湰锛岃€岃繖浜岃€呴兘鏄垜浠瘯鍥鹃伩鍏嶇殑銆?


### 椹卞姩寤鸿

鍦ㄤ负椹卞姩娣诲姞 ACPI 鏀寔鏃讹紝涓嶈绉婚櫎浠讳綍 DT 澶勭悊浠ｇ爜銆傚悓涓€涓澶囧彲鑳界敤鍦ㄨ澶氫笉鍚岀殑绯荤粺涓娿€?

璇峰敖閲忓皢椹卞姩鏋勫缓涓烘暟鎹┍鍔紙data-driven锛夌殑褰㈠紡銆備篃灏辨槸璇达紝鍩轰簬榛樿鍊间互鍙婇┍鍔?probe 鍑芥暟蹇呴』鍙戠幇鐨勫叾浠栧唴瀹癸紝寤虹珛涓€涓寘鍚瘡涓澶囧唴閮ㄧ姸鎬佺殑 struct銆傜劧鍚庤椹卞姩鐨勫叾浣欓儴鍒嗘牴鎹 struct 鐨勫唴瀹硅繍浣溿€傝繖鏍峰仛搴旇兘浣?ACPI 涓?DT 鍔熻兘涔嬮棿鐨勫ぇ閮ㄥ垎宸紓淇濇寔鍦?probe 鍑芥暟灞€閮紝鑰屼笉鏄暎甯冧簬鏁翠釜椹卞姩銆傚浜?

```
  static int device_probe_dt(struct platform_device *pdev)
  {
         /* DT specific functionality */
         ...
  }

  static int device_probe_acpi(struct platform_device *pdev)
  {
         /* ACPI specific functionality */
         ...
  }

  static int device_probe(struct platform_device *pdev)
  {
         ...
         struct device_node node = pdev->dev.of_node;
         ...

         if (node)
                 ret = device_probe_dt(pdev);
         else if (ACPI_HANDLE(&pdev->dev))
                 ret = device_probe_acpi(pdev);
         else
                 /* other initialization */
                 ...
         /* Continue with any generic probe operations */
         ...
  }
```

璇峰皢 MODULE_DEVICE_TABLE 鏉＄洰淇濈暀鍦ㄩ┍鍔ㄤ竴璧凤紝浠ユ竻妤氬湴琛ㄦ槑椹卞姩閽堝 DT 鍜?

```
  static struct of_device_id virtio_mmio_match[] = {
          { .compatible = "virtio,mmio", },
          { }
  };
  MODULE_DEVICE_TABLE(of, virtio_mmio_match);

  static const struct acpi_device_id virtio_mmio_acpi_match[] = {
          { "LNRO0005", },
          { }
  };
  MODULE_DEVICE_TABLE(acpi, virtio_mmio_acpi_match);


```

### ASWG

ACPI 瑙勮寖浼氬畾鏈熷彉鏇淬€備緥濡傦紝鍦?2014 骞存湡闂达紝鍙戝竷浜?5.1 鐗堟湰锛屽苟鍩烘湰瀹屾垚浜?6.0 鐗堟湰锛屽叾涓ぇ閮ㄥ垎鍙樻洿鐢?Arm 鐗瑰畾鐨勯渶姹傛帹鍔ㄣ€傛彁妗堜腑鐨勫彉鏇村湪 ASWG锛圓CPI Specification Working Group锛孉CPI 瑙勮寖宸ヤ綔缁勶級涓睍绀哄拰璁ㄨ锛岃宸ヤ綔缁勬槸 UEFI Forum 鐨勪竴閮ㄥ垎銆傚綋鍓嶇増鏈殑 ACPI 瑙勮寖鏄?2022 骞?8 鏈堝彂甯冪殑 6.5銆?

璇ュ皬缁勫鎵€鏈?UEFI 鎴愬憳寮€鏀惧弬涓庛€傛湁鍏冲皬缁勬垚鍛樼殑璇︾粏淇℃伅锛岃鍙傞槄 http://www.uefi.org/workinggroup銆?

Arm ACPI 鍐呮牳浠ｇ爜鐨勬剰鍥炬槸灏藉彲鑳戒弗鏍奸伒寰?ACPI 瑙勮寖锛屽苟涓斿彧瀹炵幇绗﹀悎 UEFI ASWG 宸插彂甯冩爣鍑嗙殑鍔熻兘銆傚疄闄呬笂锛屾€讳細鏈夊巶鍟嗘彁渚涚碂绯曠殑 ACPI 琛ㄦ垨浠ユ煇绉嶆柟寮忚繚鍙嶆爣鍑嗐€傚鏋滆繖鏄洜閿欒鎵€鑷达紝鍙兘蹇呰鐨勫彉閫氫笌淇ˉ锛坬uirks and fix-ups锛夊皢浼氳閲囩敤锛屼絾濡傛湁鍙兘浼氬姞浠ラ伩鍏嶃€傚鏋?ACPI 缂哄皯鏌愪簺鐗规€т互鑷充簬鏃犳硶鍦ㄦ煇涓钩鍙颁笂浣跨敤锛屽簲鍚?ASWG 鎻愪氦 ECRs锛圗ngineering Change Requests锛屽伐绋嬪彉鏇磋姹傦級锛屽苟璧版甯哥殑瀹℃壒娴佺▼锛涘浜庨偅浜涗笉鏄?UEFI 鎴愬憳鐨勪汉锛孡inux 绀惧尯鐨勮澶氬叾浠栨垚鍛樻槸鎴愬憳锛屽苟涓斿緢鍙兘鎰挎剰鍗忓姪鎻愪氦 ECRs銆?


### Linux 浠ｇ爜

鍐呯疆浜?Linux 婧愮爜涓€佺壒瀹氫簬 Arm 涓?Linux 鐨勪釜鍒潯鐩垪浜庝笅鏂囦腑锛?

ACPI_OS_NAME
                       This macro defines the string to be returned when
                       an ACPI method invokes the _OS method.  On Arm
                       systems, this macro will be "Linux" by default.
                       The command line parameter acpi_os=<string>
                       can be used to set it to some other value.  The
                       default value for other architectures is "Microsoft
                       Windows NT", for example.


### ACPI 瀵硅薄

鏈夊叧 ACPI 琛ㄤ笌瀵硅薄鐨勮缁嗘湡鏈涳紝鍒椾簬鏂囦欢 Documentation/arch/arm64/acpi_object_usage.rst 涓€?


### 鍙傝€冭祫鏂?

[^0^] https://developer.arm.com/documentation/den0094/latest
    document Arm-DEN-0094: "Arm Base System Architecture", version 1.0C, dated 6 Oct 2022

[^1^] https://developer.arm.com/documentation/den0044/latest
    Document Arm-DEN-0044: "Arm Base Boot Requirements", version 2.0G, dated 15 Apr 2022

[^2^] https://developer.arm.com/documentation/den0029/latest
    Document Arm-DEN-0029: "Arm Server Base System Architecture", version 7.1, dated 06 Oct 2022

[^3^] http://www.secretlab.ca/archives/151,
    10 Jan 2015, Copyright (c) 2015,
    Linaro Ltd., written by Grant Likely.

[^4^] _DSD锛圖evice Specific Data锛岃澶囩壒瀹氭暟鎹級瀹炵幇鎸囧崡
    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.pdf

[^5^] 缁熶竴璁惧灞炴€ф帴鍙ｇ殑 Linux 鍐呮牳浠ｇ爜鍙湪 include/linux/property.h 鍜?drivers/base/property.c 涓壘鍒般€?


### 浣滆€?

- Al Stone <al.stone@linaro.org>
- Graeme Gregory <graeme.gregory@linaro.org>
- Hanjun Guo <hanjun.guo@linaro.org>

- Grant Likely <grant.likely@linaro.org>锛岃礋璐ｂ€滀负浣曞湪 Arm 涓婁娇鐢?ACPI锛熲€濅竴鑺?
