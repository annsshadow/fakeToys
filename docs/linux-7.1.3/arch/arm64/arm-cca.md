
## Arm 鏈哄瘑璁＄畻鏋舵瀯锛圕onfidential Compute Architecture锛?

鏀寔 Realm Management Extension锛圧ME锛夌殑 Arm 绯荤粺鍖呭惈纭欢锛屽厑璁镐互鏌愮鏂瑰紡杩愯 VM 瀹㈡埛鏈猴紝浠庤€屼繚鎶?瀹㈡埛鏈虹殑浠ｇ爜涓庢暟鎹厤鍙?hypervisor 鐨勪镜瀹炽€傚畠灏嗘棫鐨勨€滀袱涓笘鐣屸€濇ā鍨嬶紙Normal 涓?Secure World锛夋墿灞曚负
鍥涗釜涓栫晫锛歂ormal銆丼ecure銆丷oot 涓?Realm銆侺inux 涔熷彲浠ヤ綔涓哄鎴锋満杩愯鍦?Realm 涓栫晫涓繍琛岀殑鐩戣鍣紙monitor锛?涔嬩笅銆?
杩愯鍦?Realm 涓栫晫涓殑鐩戣鍣ㄨ绉颁负 Realm Management Monitor锛圧MM锛夛紝瀹炵幇浜?Realm Management Monitor
瑙勮寖[^1^]銆傝鐩戣鍣ㄦ湁鐐瑰儚 hypervisor锛堜緥濡傚畠杩愯鍦?EL2锛屽苟绠＄悊杩愯鍦?Realm 涓栫晫涓殑瀹㈡埛鏈虹殑 stage 2
椤佃〃绛夛級锛屼絾澶ч儴鍒嗘帶鍒舵潈鐢辫繍琛屽湪 Normal World 涓殑 hypervisor 鎺屾彙銆侼ormal World 鐨?hypervisor 浣跨敤
RMM 瑙勮寖瀹氫箟鐨?Realm Management Interface锛圧MI锛夋潵璇锋眰 RMM 鎵ц鎿嶄綔锛堜緥濡傛槧灏勫唴瀛樻垨鎵ц涓€涓?vCPU锛夈€?
RMM 涓哄鎴锋満瀹氫箟浜嗕竴涓幆澧冿紝鍏朵腑鍦板潃绌洪棿锛圛PA锛夎涓€鍒嗕负浜屻€備笅鍗婇儴鍒嗘槸鍙椾繚鎶ょ殑鈥斺€旀槧灏勫埌姝ゅ崐閮ㄥ垎鐨勪换浣曞唴瀛?閮芥棤娉曡 Normal World 鐪嬪埌锛屼笖 RMM 闄愬埗 Normal World 瀵规鍐呭瓨鍙墽琛岀殑鎿嶄綔锛堜緥濡傦紝鏈粡瀹㈡埛鏈洪厤鍚堬紝Normal World
鏃犳硶鏇挎崲姝ゅ尯鍩熷唴鐨勯〉锛夈€備笂鍗婇儴鍒嗘槸鍏变韩鐨勶紝Normal World 鍙互鑷敱鍦版洿鏀规鍖哄煙鍐呯殑椤碉紝骞惰兘澶熷湪姝ゅ尯鍩熶腑妯℃嫙
MMIO 璁惧銆?
杩愯鍦?Realm 涓殑瀹㈡埛鏈轰篃鍙互閫氳繃 Realm Services Interface锛圧SI锛変笌 RMM 閫氫俊锛屼互璇锋眰鏇存敼鍏剁幆澧冩垨瀵瑰叾
鐜杩涜璇佹槑锛坅ttestation锛夈€傜壒鍒湴锛屽畠鍙互璇锋眰灏嗗彈淇濇姢鍦板潃绌洪棿鐨勬煇浜涘尯鍩熷湪鈥淩AM鈥濅笌鈥淓MPTY鈥濅箣闂磋浆鎹紙浠讳竴鏂瑰悜锛夈€?杩欏厑璁?Realm 瀹㈡埛鏈轰氦鍑哄唴瀛樹互褰掕繕缁?Normal World锛屾垨鍚?Normal World 璇锋眰鏂板唴瀛樸€傚鏋滄病鏈?Realm 瀹㈡埛鏈虹殑鏄惧紡璇锋眰锛?RMM 浼氶樆姝?Normal World 杩涜杩欎簺鏇存敼銆?
### Linux 浣滀负 Realm 瀹㈡埛鏈?

瑕佸湪 Realm 涓皢 Linux 浣滀负瀹㈡埛鏈鸿繍琛岋紝浠ヤ笅鍐呭蹇呴』鐢?VMM 鎴栧湪 Linux 涔嬪墠杩愯浜?Realm 涓殑 `boot loader` 鎻愪緵锛?
 - 鎻忚堪缁?Linux 鐨勬墍鏈夊彈淇濇姢 RAM锛堥€氳繃 DT 鎴?ACPI锛夊湪绉讳氦缁?Linux 涔嬪墠蹇呴』鏍囪涓?RIPAS RAM銆?
 - MMIO 璁惧蹇呴』鏈淇濇姢锛堜緥濡傜敱 Normal World 妯℃嫙锛夋垨鏍囪涓?RIPAS DEV銆?
 - 鐢?Normal World 妯℃嫙骞跺湪鍚姩鏃╂湡锛堢壒鍒槸 earlycon锛変娇鐢ㄧ殑 MMIO 璁惧蹇呴』鎸囧畾鍦?IPA 鐨勪笂鍗婇儴鍒嗐€傚浜?earlycon锛?   杩欏彲浠ラ€氳繃鍦ㄥ懡浠よ涓婃寚瀹氬湴鍧€鏉ュ畬鎴愶紝渚嬪 IPA 澶у皬涓?33 浣嶃€佽妯℃嫙 UART 鐨勫熀鍦板潃涓?0x1000000锛?   `earlycon=uart,mmio,0x101000000`

 - Linux 灏嗕娇鐢ㄥ弽寮圭紦鍐插尯涓庢湭鍙椾繚鎶ょ殑璁惧閫氫俊銆傚畠浼氬皢涓€浜涘彈淇濇姢鍐呭瓨杞崲涓?RIPAS EMPTY锛屽苟鏈熸湜鑳藉鍦ㄧ浉鍚岀殑 IPA 鍦板潃銆?   浣嗘渶楂樻湁鏁?IPA 浣嶇疆浣嶇殑鎯呭喌涓嬭闂湭鍙椾繚鎶ょ殑椤点€傞鏈熸槸 VMM 浼氫粠鍙椾繚鎶ゆ槧灏勪腑绉婚櫎鐗╃悊椤碉紝骞跺皢杩欎簺椤典綔涓烘湭鍙椾繚鎶ょ殑椤垫彁渚涖€?
### 鍙傝€?

[^1^] https://developer.arm.com/documentation/den0137/
