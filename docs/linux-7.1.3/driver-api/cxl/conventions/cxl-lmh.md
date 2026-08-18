
## Resolve conflict between CFMWS, Platform Memory Holes, and Endpoint Decoders


### Document


CXL Revision 3.2, Version 1.0

### License


SPDX-License Identifier: CC-BY-4.0

### Creator/Contributors


- Fabio M. De Francesco, Intel
- Dan J. Williams, Intel
- Mahesh Natu, Intel

### Summary of the Change


鏍规嵁褰撳墠鐨?Compute Express Link锛圕XL锛夎鑼冿紙Revision 3.2, Version 1.0锛夛紝CXL 鍥哄畾鍐呭瓨绐楀彛缁撴瀯锛圕FMWS锛夋弿杩颁簡涓庢瘡涓?CXL 涓绘満妗ョ浉鍏宠仈鐨勯浂涓垨澶氫釜涓绘満鐗╃悊鍦板潃锛圚PA锛夌獥鍙ｃ€傛瘡涓獥鍙ｄ唬琛ㄤ竴涓彲鑳借法涓€涓垨澶氫釜鐩爣锛堝寘鎷?CXL 涓绘満妗ワ級浜ょ粐鐨勮繛缁?HPA 鑼冨洿銆傛瘡涓獥鍙ｉ兘鏈変竴缁勭害鏉熷叾浣跨敤鐨勯檺鍒躲€傜敱鎿嶄綔绯荤粺涓诲鐨勯厤缃笌鐢垫簮绠＄悊锛圤SPM锛夎礋璐ｅ皢姣忎釜绐楀彛鐢ㄤ簬鎸囧畾鐨勭敤閫斻€?
褰撳墠 CXL 瑙勮寖鐨勮〃 9-22 鎸囧嚭锛學indow Size 瀛楁鍖呭惈璇ョ獥鍙ｆ弿杩扮殑 HPA 杩炵画瀛楄妭鎬绘暟銆傝鍊煎繀椤绘槸浜ょ粐璺暟锛圢IW锛? 256 MB 鐨勬暣鏁板€嶃€?
骞冲彴鍥轰欢锛圔IOS锛夊彲鑳藉湪 4 GB 浠ヤ笅淇濈暀鐗╃悊鍦板潃锛岄偅閲屽彲鑳藉瓨鍦ㄥ唴瀛樼┖娲烇紙渚嬪鐢ㄤ簬 PCIe MMIO 鐨勪綆鍐呭瓨绌烘礊锛夈€傚湪杩欑鎯呭喌涓嬶紝CFMWS 鑼冨洿澶у皬鍙兘涓嶉伒寰?NIW * 256 MB 瑙勫垯銆?
HPA 浠ｈ〃 CXL 璁惧鑳藉瑙ｇ爜骞跺搷搴旂殑瀹為檯鐗╃悊鍐呭瓨鍦板潃绌洪棿锛岃€岀郴缁熺墿鐞嗗湴鍧€锛圫PA锛夋槸涓€涓浉鍏充絾涓嶅悓鐨勬蹇碉紝瀹冧唬琛ㄧ敤鎴峰彲浠ョ洿鎺ュ彂璧蜂簨鍔″鍧€鐨勭郴缁熷彲瑙佸湴鍧€绌洪棿锛屽洜姝ゆ帓闄や簡淇濈暀鍖哄煙銆?
BIOS 鍙戝竷 CFMWS 鏉ヤ紶杈炬椿璺冪殑 SPA 鑼冨洿锛屽湪鏈?LMH 鐨勫钩鍙颁笂锛岃繖浜涜寖鍥存槧灏勫埌 HPA 鐨勪竴涓弗鏍煎瓙闆嗐€係PA 鑼冨洿瑁佹帀浜嗙┖娲烇紝瀵艰嚧 Endpoints 涓湁涓€閮ㄥ垎 HPA 鑼冨洿涓庣┖娲炵浉浜ゅ嵈鏃犲搴?SPA 鍙槧灏勶紝浠庤€屼涪澶卞閲忋€?
渚嬪锛屼竴涓甫涓や釜 CFMWS 涓?LMH 浠?2 GB 寮€濮嬬殑 x86 骞冲彴锛?
 +--------+------------+-------------------+------------------+-------------------+------+
 | Window | CFMWS Base |    CFMWS Size     | HDM Decoder Base |  HDM Decoder Size | Ways |
 +========+============+===================+==================+===================+======+
 |  鈥?    |   0 GB     |       2 GB        |      0 GB        |       3 GB        |  12  |
 +--------+------------+-------------------+------------------+-------------------+------+
 |  鈥?    |   4 GB     | NIW**256MB Aligned |      4 GB        | NIW**256MB Aligned |  12  |
 +--------+------------+-------------------+------------------+-------------------+------+

HDM decoder base 鍜?HDM decoder size 浠ｈ〃涓€涓?12 璺尯鍩熺殑鍏ㄩ儴 12 涓?Endpoint Decoder 浠ュ強鎵€鏈変腑闂?Switch Decoder銆傚畠浠敱 BIOS 鏍规嵁 NIW * 256MB 瑙勫垯閰嶇疆锛屼骇鐢?3GB 鐨?HPA 鑼冨洿澶у皬銆傝€?CFMWS Base 鍜?CFMWS Size 鐢ㄤ簬閰嶇疆 Root Decoder 鐨?HPA 鑼冨洿锛岀粨鏋滐紙2GB锛夋瘮灞傛缁撴瀯涓?Switch 鍜?Endpoint Decoder 鐨勮寖鍥达紙3GB锛夋洿灏忋€?
杩欎細閫犳垚涓や釜闂锛屽鑷存棤娉曟瀯寤哄尯鍩燂紙region锛夛細

1) Root 涓庝换浣?HDM decoder 涔嬮棿鐨勫尯鍩熷ぇ灏忎笉鍖归厤銆傜敱浜庤鍑忥紝Root decoder 鎬绘槸鏇村皬銆?
2) 瑁佸噺瀵艰嚧 root decoder 杩濆弽锛圢IW * 256MB锛夎鍒欍€?
璇ユ敼鍔ㄥ厑璁稿熀鍧€涓?0GB 鐨勫尯鍩熺粫杩囪繖浜涙鏌ワ紝浠ヤ究鐢ㄨ瑁佸噺鐨?root decoder 鍦板潃鑼冨洿鏋勫缓鍖哄煙銆?
璇ユ敼鍔ㄤ笉鍏佽浠讳綍鍏朵粬浠绘剰鍖哄煙杩濆弽杩欎簺妫€鏌モ€斺€斿畠涓撻棬鐢ㄤ簬浣垮皢 CXL 鍐呭瓨鏄犲皠鍒?4GB 浠ヤ笅鐨?x86 骞冲彴鑳藉鏋勫缓鍖哄煙銆?
灏界 HDM decoder 瑕嗙洊浜?PCIE 绌烘礊鐨?HPA 鍖哄煙锛屼絾棰勮骞冲彴姘歌繙涓嶄細鎶婂湴鍧€璁块棶璺敱鍒?CXL 澶嶅悎浣擄紝鍥犱负 root decoder 鍙鐩栬瑁佸噺鐨勫尯鍩燂紙鍗虫帓闄や簡璇ョ┖娲烇級銆傝繖瓒呭嚭浜?Linux 鑳藉寮哄埗瀹炴柦鐨勮兘鍔涜寖鍥淬€?
鍦ㄧず渚嬪钩鍙颁笂锛屽彧鏈夊墠 2GB 鍙兘鍙敤锛屼絾 Linux 涓轰簡閬靛惊褰撳墠瑙勮寖锛屾棤娉曟瀯寤?Region 骞舵妸 Endpoint 涓庝腑闂?Switch Decoder 鎸傛帴鍒板畠浠笂闈€?
鏈夊涓け璐ョ偣锛屽師鍥犲湪浜庝汉浠湡鏈?Root Decoder 鐨?HPA 澶у皬锛堢瓑浜庨厤缃畠鐨?CFMWS 澶у皬锛夊繀椤诲ぇ浜庢垨绛変簬鍖归厤鐨?Switch 鍜?Endpoint HDM Decoder銆?
涓轰簡鎴愬姛鏋勫缓骞舵寕鎺ワ紝Linux 蹇呴』鐢?Root Decoder 鐨?HPA 鑼冨洿澶у皬鏋勫缓涓€涓?Region锛岀劧鍚庢妸灞炰簬璇ュ眰娆＄粨鏋勭殑鎵€鏈変腑闂?Switch Decoder 鍜?Endpoint Decoder 鎸傛帴鍒拌 Region锛岃€屼笉璁哄畠浠悇鑷殑鑼冨洿澶у皬銆?
### Benefits of the Change


濡傛灉涓嶅仛姝ゆ敼鍔紝OSPM 灏嗘棤娉曟妸涓棿 Switch 鍜?Endpoint Decoder 涓庨厤缃簡涓嶇鍚?NIW * 256MB 绾︽潫鐨?CFMWS HPA 澶у皬鐨?Root Decoder 鍖归厤璧锋潵锛屼粠鑰屽鑷?memdev 瀹归噺涓㈠け銆?
璇ユ敼鍔ㄤ娇 OSPM 鑳藉鏋勫缓 Region 骞舵妸涓棿 Switch 鍜?Endpoint Decoder 鎸傛帴鍒板畠浠紝浠庤€屼娇鍐呭瓨璁惧鎬诲閲忎腑鍙鍧€鐨勯儴鍒嗗鐢ㄦ埛鍙敤銆?
### References


Compute Express Link Specification Revision 3.2, Version 1.0
<https://www.computeexpresslink.org/>

### Detailed Description of the Change


琛?9-22 涓?Window Size 瀛楁鐨勬弿杩伴渶瑕侀【鍙婂瓨鍦ㄤ綆鍐呭瓨绌烘礊锛圠ow Memory Holes锛夌殑骞冲彴锛岄偅閲?SPA 鑼冨洿鍙兘鏄?endpoints HPA 鐨勫瓙闆嗐€傚洜姝わ紝瀹冮渶瑕佹敼涓哄涓嬪唴瀹癸細

"璇ョ獥鍙ｆ墍浠ｈ〃鐨?HPA 杩炵画瀛楄妭鎬绘暟銆傝鍊煎簲涓?NIW * 256 MB 鐨勬暣鏁板€嶃€?
鍦ㄤ繚鐣?4 GB 浠ヤ笅鐗╃悊鍦板潃鐨勫钩鍙帮紙渚嬪 x86 涓婄敤浜?PCIe MMIO 鐨勪綆鍐呭瓨绌烘礊锛変笂锛孊ase HPA 鑼冨洿涓?0 鐨勬煇涓?CFMWS 瀹炰緥锛屽叾澶у皬鍙兘涓嶇鍚?NIW * 256 MB 绾︽潫銆?
娉ㄦ剰锛屽尮閰嶇殑涓棿 Switch Decoder 鍜?Endpoint Decoder 鐨?HPA 鑼冨洿澶у皬浠嶉』绗﹀悎涓婅堪瑙勫垯锛屼絾瓒呭嚭 CFMWS 绐楀彛澶у皬鐨勯偅閮ㄥ垎鍐呭瓨瀹归噺灏嗕笉鍙闂€?銆?