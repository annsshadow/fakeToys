
## Spectre 渚т俊閬?

Spectre 鏄竴绫讳晶淇￠亾鏀诲嚮锛屽畠鍒╃敤鐜颁唬 CPU 涓婄殑鍒嗘敮棰勬祴鍜屾帹娴嬫墽琛岋紙speculative
execution锛夋潵璇诲彇鍐呭瓨锛屽彲鑳界粫杩囪闂帶鍒躲€傛帹娴嬫墽琛屼晶淇￠亾鏀诲嚮涓嶄慨鏀瑰唴瀛橈紝鑰屾槸
璇曞浘鎺ㄦ柇鍐呭瓨涓殑鐗规潈鏁版嵁銆?
鏈枃妗ｆ兜鐩?Spectre 鍙樹綋 1 鍜?Spectre 鍙樹綋 2銆?
### 鍙楀奖鍝嶇殑澶勭悊鍣?

鎺ㄦ祴鎵ц渚т俊閬撴柟娉曞奖鍝嶄簡骞挎硾鑼冨洿鐨勭幇浠ｉ珮鎬ц兘澶勭悊鍣紝鍥犱负澶у鏁扮幇浠ｉ珮閫?澶勭悊鍣ㄩ兘浣跨敤鍒嗘敮棰勬祴鍜屾帹娴嬫墽琛屻€?
浠ヤ笅 CPU 鏄撳彈鏀诲嚮锛?
    - Intel Core銆丄tom銆丳entium 鍜?Xeon 澶勭悊鍣?
    - AMD Phenom銆丒PYC 鍜?Zen 澶勭悊鍣?
    - IBM POWER 鍜?zSeries 澶勭悊鍣?
    - 楂樼 ARM 澶勭悊鍣?
    - Apple CPU

    - 楂樼 MIPS CPU

    - 鍙兘杩樻湁澶у鏁板叾浠栭珮鎬ц兘 CPU銆傝鎯呰鑱旂郴鎮ㄧ殑 CPU 渚涘簲鍟嗐€?
鏌愪釜澶勭悊鍣ㄦ槸鍚﹀彈褰卞搷锛屽彲浠ヤ粠 sysfs 涓殑 Spectre 婕忔礊鏂囦欢涓鍑恒€傝鍙傞槄
spectre_sys_info銆?
### 鐩稿叧鐨?CVE


浠ヤ笅 CVE 鏉＄洰鎻忚堪浜?Spectre 鍙樹綋锛?
   =============   =======================  ==========================
   CVE-2017-5753   Bounds check bypass      Spectre variant 1
   CVE-2017-5715   Branch target injection  Spectre variant 2
   CVE-2019-1125   Spectre v1 swapgs        Spectre variant 1 (swapgs)
   =============   =======================  ==========================

### 闂


CPU 浣跨敤鎺ㄦ祴鎿嶄綔鏉ユ彁鍗囨€ц兘銆傝繖鍙兘浼氬湪澶勭悊鍣ㄧ殑缂撳瓨銆佺紦鍐插尯鍜屽垎鏀娴嬪櫒涓?鐣欎笅鍐呭瓨璁块棶鎴栬绠楃殑鐥曡抗銆傛伓鎰忚蒋浠跺彲鑳藉奖鍝嶆帹娴嬫墽琛岃矾寰勶紝鐒跺悗鍒╃敤鎺ㄦ祴鎵ц
鍦?CPU 缂撳瓨鍜岀紦鍐插尯涓暀涓嬬殑鍓綔鐢紝鏉ユ帹鏂湪鎺ㄦ祴鎵ц鏈熼棿琚Е鍙婄殑鐗规潈鏁版嵁銆?
Spectre 鍙樹綋 1 鏀诲嚮鍒╃敤鏉′欢鍒嗘敮鐨勬帹娴嬫墽琛岋紝鑰?Spectre 鍙樹綋 2 鏀诲嚮浣跨敤闂存帴
鍒嗘敮鐨勬帹娴嬫墽琛屾潵娉勯湶鐗规潈鍐呭瓨銆傚弬瑙?[^1^] <spec_ref1> [^5^] <spec_ref5> [^6^] <spec_ref6>
[^7^] <spec_ref7> [^10^] <spec_ref10> [^11^] <spec_ref11>銆?
### Spectre 鍙樹綋 1锛堣竟鐣屾鏌ョ粫杩囷紝Bounds Check Bypass锛?

杈圭晫妫€鏌ョ粫杩囨敾鍑?[^2^] <spec_ref2> 鍒╃敤鎺ㄦ祴鎵ц缁曡繃鐢ㄤ簬鍐呭瓨璁块棶杈圭晫妫€鏌?锛堜緥濡傦紝妫€鏌ユ暟缁勭殑绱㈠紩鏄惁瀵艰嚧鍐呭瓨鍦ㄦ湁鏁堣寖鍥村唴鐨勮闂級鐨勬潯浠跺垎鏀寚浠ゃ€傝繖浼?瀵艰嚧瀵规棤鏁堝唴瀛橈紙甯︽湁瓒婄晫绱㈠紩锛夌殑鍐呭瓨璁块棶锛岃繖浜涜闂湪楠岃瘉妫€鏌ヨВ鍐充箣鍓嶈
鎺ㄦ祴鍦版墽琛屻€傝繖鏍风殑鎺ㄦ祴鍐呭瓨璁块棶鍙兘鐣欎笅鍓綔鐢紝浠庤€屽垱寤哄皢淇℃伅娉勯湶缁欐敾鍑昏€呯殑
渚т俊閬撱€?
Spectre 鍙樹綋 1 鏀诲嚮鏈変竴浜涙墿灞曪紝鐢ㄤ簬閫氳繃缃戠粶璇诲彇鏁版嵁锛屽弬瑙?[^12^] <spec_ref12>銆?鐒惰€屾绫绘敾鍑诲洶闅俱€佸甫瀹戒綆銆佽剢寮憋紝琚涓轰綆椋庨櫓銆?
璇锋敞鎰忥紝灏界鍚嶄负鈥淏ounds Check Bypass锛堣竟鐣屾鏌ョ粫杩囷級鈥濓紝Spectre 鍙樹綋 1 骞堕潪
浠呭叧涔庣敤鎴锋帶鍒剁殑鏁扮粍杈圭晫妫€鏌ャ€傚畠鍙互褰卞搷浠讳綍鏉′欢妫€鏌ャ€傚唴鏍稿叆鍙ｄ唬鐮佺殑
涓柇銆佸紓甯稿拰 NMI 澶勭悊绋嬪簭閮芥湁鏉′欢鐨?swapgs 妫€鏌ャ€傚湪鍐呮牳浠ｇ爜鍙互鍦ㄦ帹娴嬫墽琛屼腑
浠ヤ竴涓敤鎴?GS 杩愯鐨勬儏鍐典笅锛岄偅浜涙鏌ュ湪 Spectre v1 鐨勮澧冧笅鍙兘鎴愰棶棰樸€?
### Spectre 鍙樹綋 2锛堝垎鏀洰鏍囨敞鍏ワ紝Branch Target Injection锛?

鍒嗘敮鐩爣娉ㄥ叆鏀诲嚮鍒╃敤闂存帴鍒嗘敮鐨勬帹娴嬫墽琛?[^3^] <spec_ref3>銆傚鐞嗗櫒鍐呴儴鐢ㄤ簬
鐚滄祴闂存帴鍒嗘敮鐩爣鐨勯棿鎺ュ垎鏀娴嬪櫒鍙兘鍙楀埌鏀诲嚮鑰呭奖鍝嶏紝瀵艰嚧 gadget 浠ｇ爜琚帹娴?鎵ц锛屼粠鑰屾毚闇插彈瀹宠€呰Е鍙婄殑鏁忔劅鏁版嵁銆傛帹娴嬫墽琛屾湡闂村湪 CPU 缂撳瓨涓暀涓嬬殑鍓綔鐢?鍙互琚祴閲忎互鎺ㄦ柇鏁版嵁鍊笺€?

鍦?Spectre 鍙樹綋 2 鏀诲嚮涓紝鏀诲嚮鑰呭彲浠ラ€氳繃姣掑寲锛坧oisoning锛夌敤浜庨娴嬮棿鎺ュ垎鏀?鍦板潃鐨?CPU 鐨勫垎鏀洰鏍囩紦鍐插尯锛圔TB锛夛紝鏉ュ皢鍙楀鑰呯殑鎺ㄦ祴闂存帴鍒嗘敮瀵煎悜 gadget 浠ｇ爜銆?杩欑姣掑寲鍙互閫氳繃闂存帴鍒嗘敮杩涘叆宸叉湁浠ｇ爜鏉ュ畬鎴愶紝闂存帴鍒嗘敮鐨勫湴鍧€鍋忕Щ鐢辨敾鍑昏€呮帶鍒躲€?鐢变簬鍦ㄥ彈褰卞搷纭欢涓婄殑鍒嗘敮棰勬祴涓嶈兘瀹屽叏娑堟涔夊垎鏀湴鍧€銆佽€屾槸浣跨敤鍋忕Щ杩涜棰勬祴锛?杩欏彲鑳藉鑷寸壒鏉冧唬鐮佺殑闂存帴鍒嗘敮璺宠浆鍒板叿鏈夌浉鍚屽亸绉荤殑 gadget 浠ｇ爜銆?
鏈€鏈夌敤鐨?gadget 鎺ュ彈涓€涓敾鍑昏€呮帶鍒剁殑杈撳叆鍙傛暟锛堜緥濡備竴涓瘎瀛樺櫒鍊硷級锛屼互渚垮彈鎺?鍦拌鍙栧唴瀛樸€傛病鏈夎緭鍏ュ弬鏁扮殑 gadget 涔熸湁鍙兘锛屼絾鏀诲嚮鑰呭鍏惰兘璇诲彇浠€涔堝唴瀛樺嚑涔?鏃犳硶鎺у埗锛屼粠鑰岄檷浣庝簡鏀诲嚮娉勯湶鏈夌敤鏁版嵁鐨勯闄┿€?
鍙樹綋 2 鐨勫彟涓€绉嶆敾鍑婚€斿緞鏄敾鍑昏€呮瘨鍖栬繑鍥炴爤缂撳啿鍖猴紙RSB锛塠^13^] <spec_ref13>锛?浠ュ鑷存帹娴嬬殑瀛愪緥绋嬭繑鍥炴寚浠ゆ墽琛岃烦杞埌 gadget銆傛敾鍑昏€呬笉骞宠　鐨勫瓙渚嬬▼璋冪敤鎸囦护
鍙兘鈥滄瘨鍖栤€濊繑鍥炴爤缂撳啿鍖轰腑鐨勬潯鐩紝杩欎簺鏉＄洰闅忓悗琚彈瀹宠€呯殑瀛愪緥绋嬭繑鍥炴寚浠ゆ秷璐广€?杩欑鏀诲嚮鍙互閫氳繃鍦ㄤ笂涓嬫枃鍒囨崲鎴栬櫄鎷熸満锛圴M锛夐€€鍑烘椂鍒锋柊杩斿洖鏍堢紦鍐插尯鏉ョ紦瑙ｃ€?
鍦ㄥ叿鏈夊悓姝ュ绾跨▼锛圫MT锛夌殑绯荤粺涓婏紝鏀诲嚮鍙兘鏉ヨ嚜鍏勫紵绾跨▼锛坰ibling thread锛夛紝
鍥犱负涓€绾х紦瀛樺拰鍒嗘敮鐩爣缂撳啿鍖猴紙BTB锛夊彲鑳藉湪 CPU 鏍稿唴鐨勭‖浠剁嚎绋嬩箣闂村叡浜€傝繍琛?鍦ㄥ厔寮熺嚎绋嬩笂鐨勬伓鎰忕▼搴忓彲鑳藉奖鍝嶅叾瀵圭瓑鏂圭殑 BTB锛屽皢鍏堕棿鎺ュ垎鏀帹娴嬪鍚?gadget
浠ｇ爜锛屽苟娴嬮噺鐣欏湪涓€绾х紦瀛樹腑鐨勬帹娴嬫墽琛屽壇浣滅敤锛屼互鎺ㄦ柇鍙楀鑰呯殑鏁版嵁銆?
鍙樹綋 2 鐨勫張涓€绉嶆敾鍑婚€斿緞鏄敾鍑昏€呮瘨鍖栧垎鏀巻鍙茬紦鍐插尯锛圔HB锛夛紝浠ユ帹娴嬪湴灏嗕竴涓?闂存帴鍒嗘敮瀵煎悜涓€涓壒瀹氱殑鍒嗘敮鐩爣缂撳啿鍖猴紙BTB锛夋潯鐩紝鍗充娇璇ユ潯鐩笌璇ラ棿鎺ュ垎鏀殑
婧愬湴鍧€骞朵笉鍏宠仈銆傚叿浣撴潵璇达紝鍗充娇鍦?Enhanced IBRS 瀛樺湪鐨勬儏鍐典笅锛孊HB 涔熷彲鑳借法
鐗规潈绾у埆鍏变韩銆?
姝ゅ墠鍞竴宸茬煡鐨勭湡瀹炰笘鐣?BHB 鏀诲嚮閫斿緞鏄€氳繃闈炵壒鏉?eBPF銆傝繘涓€姝ョ殑鐮旂┒鍙戠幇浜?涓嶉渶瑕侀潪鐗规潈 eBPF 鐨勬敾鍑汇€備负浜嗗 BHB 鏀诲嚮杩涜瀹屾暣缂撹В锛屽缓璁娇鐢?BHI_DIS_S
鎴栭噰鐢?BHB 娓呴櫎搴忓垪銆?
### 鏀诲嚮鍦烘櫙


浠ヤ笅鏄凡琚瑙佺殑鏀诲嚮鍦烘櫙鍒楄〃锛屼絾鍙兘鏈兜鐩栨墍鏈夊彲鑳界殑鏀诲嚮閫斿緞銆?
##### 1. 鐢ㄦ埛杩涚▼鏀诲嚮鍐呮牳


#### Spectre 鍙樹綋 1


   鏀诲嚮鑰呴€氳繃瀵勫瓨鍣ㄦ垨鍦ㄧ郴缁熻皟鐢紙syscall锛夋湡闂寸粡鐢卞唴瀛樹腑宸茬煡鍦板潃锛屽悜鍐呮牳
   浼犻€掍竴涓弬鏁般€傝繖鏍风殑鍙傛暟涔嬪悗鍙兘琚唴鏍哥敤浣滄暟缁勭殑绱㈠紩锛屾垨鐢ㄤ簬鎺ㄥ涓€涓?   鎸囧悜 Spectre 鍙樹綋 1 鏀诲嚮鐨勬寚閽堛€傝绱㈠紩鎴栨寚閽堟槸鏃犳晥鐨勶紝浣嗗湪琚噰鍙栫敤浜?   鎺ㄦ祴鎵ц鐨勪唬鐮佸垎鏀腑锛岃竟鐣屾鏌ヨ缁曡繃銆傝繖鍙兘瀵艰嚧鐗规潈鍐呭瓨琚闂苟娉勯湶銆?
   瀵逛簬宸茶瘑鍒嚭鏁版嵁鎸囬拡鍙兘鍙?Spectre 鏀诲嚮褰卞搷鐨勫唴鏍镐唬鐮侊紝浣跨敤鏂扮殑鈥渘ospec鈥?   璁块棶鍣ㄥ畯鏉ラ槻姝㈡暟鎹殑鎺ㄦ祴鍔犺浇銆?
#### Spectre 鍙樹綋 1锛坰wapgs锛?

   鏀诲嚮鑰呭彲浠ヨ缁冨垎鏀娴嬪櫒锛屼互鎺ㄦ祴鍦拌烦杩囦腑鏂垨寮傚父瀵瑰簲鐨?swapgs 璺緞銆?   濡傛灉浠栦滑鎶?GS 瀵勫瓨鍣ㄥ垵濮嬪寲涓轰竴涓敤鎴风┖闂村€硷紝鑰?swapgs 琚帹娴嬭烦杩囷紝閭ｄ箞鍦?   鎺ㄦ祴绐楀彛涓悗缁殑 GS 鐩稿叧 percpu 璁块棶灏嗕娇鐢ㄦ敾鍑昏€呮帶鍒剁殑 GS 鍊艰繘琛屻€傝繖鍙兘
   瀵艰嚧鐗规潈鍐呭瓨琚闂苟娉勯湶銆?
   渚嬪锛?
```

     if (coming from user space)
         swapgs
     mov %gs:<percpu_offset>, %reg
     mov (%reg), %reg1

   褰撴潵鑷敤鎴风┖闂存椂锛孋PU 鍙互鎺ㄦ祴鍦拌烦杩?swapgs锛岀劧鍚庡埄鐢ㄧ敤鎴?GS 鍊艰繘琛屾帹娴?   鐨?percpu 鍔犺浇銆傚洜姝ょ敤鎴峰彲浠ユ帹娴嬪湴寮哄埗璇诲彇浠讳綍鍐呮牳鍊笺€傚鏋滃瓨鍦ㄤ竴涓?gadget锛?   浣跨敤 percpu 鍊间綔涓哄彟涓€娆″姞杞?瀛樺偍涓殑鍦板潃锛岄偅涔堝唴鏍稿€肩殑鍐呭鍙兘閫氳繃 L1
   渚т俊閬撴敾鍑诲彉寰楀彲瑙併€?
   褰撴潵鑷唴鏍哥┖闂存椂瀛樺湪绫讳技鐨勬敾鍑汇€侰PU 鍙互鎺ㄦ祴鍦版墽琛?swapgs锛屽鑷寸敤鎴?GS
   琚敤浜庢帹娴嬬獥鍙ｇ殑鍏朵綑閮ㄥ垎銆?
```

#### Spectre 鍙樹綋 2


   涓€涓?Spectre 鍙樹綋 2 鏀诲嚮鑰呭彲浠ュ湪鍙戣捣鏀诲嚮涔嬪墠姣掑寲 <poison_btb> 鍒嗘敮鐩爣
   缂撳啿鍖猴紙BTB锛夈€傝繘鍏ュ唴鏍稿悗锛屽唴鏍稿彲鑳藉湪闂存帴璺宠浆涓婁娇鐢ㄨ姣掑寲鐨勫垎鏀洰鏍囩紦鍐?   鍖猴紝骞惰烦杞埌鎺ㄦ祴鎵ц涓殑 gadget 浠ｇ爜銆?
   濡傛灉鏀诲嚮鑰呰瘯鍥炬帶鍒舵帹娴嬫墽琛屾湡闂存硠闇茬殑鍐呭瓨鍦板潃锛屼粬杩橀渶瑕侀€氳繃瀵勫瓨鍣ㄦ垨鍐呭瓨涓?   宸茬煡鍦板潃鍚?gadget 浼犻€掍竴涓弬鏁般€傚湪 gadget 鎵ц涔嬪悗锛屼粬鍙互娴嬮噺鍓綔鐢ㄣ€?
   鍐呮牳鍙互閫氳繃瀵规墍鏈夐棿鎺ュ垎鏀娇鐢ㄨ繑鍥炶功搴婏紙return trampolines锛屼篃绉颁负
   鈥渞etpoline鈥濓級[^3^] <spec_ref3> [^9^] <spec_ref9> 鏉ラ槻姝㈡秷璐硅姣掑寲鐨勫垎鏀?   鐩爣缂撳啿鍖烘潯鐩€傝繑鍥炶功搴婃崟鑾锋帹娴嬫墽琛岃矾寰勶紝浠ラ槻姝㈠湪鎺ㄦ祴鎵ц鏈熼棿璺宠浆鍒?gadget
   浠ｇ爜銆傚叿鏈夌‖浠跺彲鐢ㄧ殑 Enhanced Indirect Branch Restricted Speculation
   锛圗nhanced IBRS锛屽寮哄瀷闂存帴鍒嗘敮闄愬埗鎺ㄦ祴锛夌殑 x86 CPU 搴斾娇鐢ㄨ鐗规€ф潵缂撹В
   Spectre 鍙樹綋 2锛岃€屼笉鏄?retpoline銆侲nhanced IBRS 姣?retpoline 鏇撮珮鏁堛€?
   鍥轰欢涓彲鑳藉惈鏈?gadget 浠ｇ爜锛屽彲鑳借鎭舵剰鐢ㄦ埛杩涚▼鍒╃敤 Spectre 鍙樹綋 2 鏀诲嚮
   鍔犱互鍒╃敤銆備负浜嗗湪 x86 涓婄紦瑙ｆ绫绘敾鍑伙紝鍦ㄨ皟鐢ㄤ换浣曞浐浠朵唬鐮佷箣鍓嶅紑鍚?Indirect
   Branch Restricted Speculation锛圛BRS锛岄棿鎺ュ垎鏀檺鍒舵帹娴嬶級鐗规€с€?
##### 2. 鐢ㄦ埛杩涚▼鏀诲嚮鍙︿竴涓敤鎴疯繘绋?

   鎭舵剰鐢ㄦ埛杩涚▼鍙互灏濊瘯鏀诲嚮鍙︿竴涓敤鎴疯繘绋嬶紝瑕佷箞缁忕敱鍚屼竴纭欢绾跨▼涓婄殑涓婁笅鏂?   鍒囨崲锛岃涔堟潵鑷湪鍚屾澶氱嚎绋嬶紙SMT锛夌郴缁熶笂鍏变韩涓€涓墿鐞嗗鐞嗗櫒鏍哥殑鍏勫紵瓒呯嚎绋?   锛坰ibling hyperthread锛夈€?
   Spectre 鍙樹綋 1 鏀诲嚮閫氬父闇€瑕佸湪杩涚▼涔嬮棿浼犻€掑弬鏁帮紝杩欓渶瑕佷竴绉嶆暟鎹紶閫掑叧绯伙紝
   渚嬪杩滅▼杩囩▼璋冪敤锛圧PC锛夈€傝繖浜涘弬鏁板湪 gadget 浠ｇ爜涓鐢ㄦ潵鎺ㄥ璁块棶琚敾鍑?   杩涚▼涓壒鏉冨唴瀛樼殑鏃犳晥鏁版嵁鎸囬拡銆?
   Spectre 鍙樹綋 2 鏀诲嚮鍙互鐢变竴涓伓鎰忚繘绋嬮€氳繃姣掑寲 <poison_btb> 鍒嗘敮鐩爣缂撳啿
   鍖哄彂璧枫€傝繖鍙互褰卞搷鍙楀鑰呰繘绋嬬殑闂存帴鍒嗘敮鐩爣锛岃鍙楀鑰呰繘绋嬭涔堢◢鍚庡湪鍚屼竴
   纭欢绾跨▼涓婅繍琛岋紝瑕佷箞鍦ㄥ叡浜悓涓€鐗╃悊鏍哥殑鍏勫紵纭欢绾跨▼涓婂苟鍙戣繍琛屻€?
   鐢ㄦ埛杩涚▼鍙互閫氳繃浣跨敤 prctl() 绯荤粺璋冪敤鏉ヤ负鑷繁绂佺敤闂存帴鍒嗘敮鎺ㄦ祴锛屼粠鑰屼繚鎶?   鑷韩鍏嶅彈 Spectre 鍙樹綋 2 鏀诲嚮銆傜鐞嗗憳涔熷彲浠ラ€氳繃绂佺敤璇ヨ繘绋嬬殑闂存帴鍒嗘敮鎺ㄦ祴锛?   灏嗗叾闅旂锛坈ordon off锛夛紝闃叉鍏舵薄鏌撳垎鏀洰鏍囩紦鍐插尯銆傝繖浼氬甫鏉ユ€ц兘浠ｄ环锛屽洜涓?   涓嶅啀浣跨敤闂存帴鍒嗘敮鎺ㄦ祴骞堕渶瑕佹竻闄ゅ垎鏀洰鏍囩紦鍐插尯銆傚湪 x86 涓婂惎鐢?SMT 鏃讹紝瀵逛簬
   闂存帴鍒嗘敮鎺ㄦ祴琚鐢ㄧ殑杩涚▼锛屼細寮€鍚崟绾跨▼闂存帴鍒嗘敮棰勬祴鍣紙STIBP锛塠^4^] <spec_ref4>
   浠ラ槻姝㈠厔寮熺嚎绋嬫帶鍒跺垎鏀洰鏍囩紦鍐插尯銆傛澶栵紝鍦ㄥ垏鎹㈠埌姝ょ被杩涚▼浠ュ強浠庢绫昏繘绋?   鍒囨崲鍑哄幓鏃讹紝浼氬彂鍑洪棿鎺ュ垎鏀娴嬪睆闅滐紙IBPB锛変互娓呴櫎鍒嗘敮鐩爣缂撳啿鍖恒€?
   鍦?x86 涓婏紝杩斿洖鏍堢紦鍐插尯鍦ㄤ笂涓嬫枃鍒囨崲鏃惰濉厖锛坰tuffed锛夈€傝繖闃叉浜嗚繑鍥炴爤
   缂撳啿鍖哄湪鍒囨崲鍒版洿娣辩殑璋冪敤鏍堟椂鍙戠敓涓嬫孩鏃惰鐢ㄤ簬鍒嗘敮棰勬祴銆傚墠涓€涓繘绋嬬暀鍦ㄨ繑鍥?   鏍堢紦鍐插尯涓殑浠讳綍琚瘨鍖栨潯鐩篃灏嗚娓呴櫎銆?
   鐢ㄦ埛绋嬪簭搴斾娇鐢ㄥ湴鍧€绌洪棿闅忔満鍖栨潵浣挎敾鍑绘洿鍥伴毦锛堣缃?   /proc/sys/kernel/randomize_va_space = 1 鎴?2锛夈€?
##### 3. 铏氭嫙鍖栧鎴锋満鏀诲嚮瀹夸富鏈?

   鏀诲嚮鏈哄埗绫讳技浜庣敤鎴疯繘绋嬫敾鍑诲唴鏍哥殑鏂瑰紡銆傚唴鏍哥粡鐢辫秴绾ц皟鐢紙hyper-call锛夋垨
   鍏朵粬铏氭嫙鍖栭€€鍑鸿矾寰勮繘鍏ャ€?
   瀵逛簬 Spectre 鍙樹綋 1 鏀诲嚮锛屾伓鎰忓鎴锋満鍙互缁忕敱瓒呯骇璋冪敤浼犻€掑弬鏁帮紙渚嬪鍦?   瀵勫瓨鍣ㄤ腑锛夛紝浠ュ湪杩涘叆鍐呮牳鍚庢帹瀵兼寚鍚戠壒鏉冨唴瀛樼殑鏃犳晥鎸囬拡杩涜鎺ㄦ祴銆傚浜庡凡璇嗗埆
   鍑烘绫诲唴鏍镐唬鐮佺殑鍦版柟锛屼娇鐢?nospec 璁块棶鍣ㄥ畯鏉ラ樆姝㈡帹娴嬪唴瀛樿闂€?
   瀵逛簬 Spectre 鍙樹綋 2 鏀诲嚮锛屾伓鎰忓鎴锋満鍙互 :ref:`poison <poison_btb>` 鍒嗘敮
   鐩爣缂撳啿鍖烘垨杩斿洖鏍堢紦鍐插尯锛屽鑷村唴鏍歌烦杞埌鎺ㄦ祴鎵ц璺緞涓殑 gadget 浠ｇ爜銆?
   涓轰簡缂撹В鍙樹綋 2锛屽涓绘満鍐呮牳鍙互瀵归棿鎺ュ垎鏀娇鐢ㄨ繑鍥炶功搴婏紝浠ョ粫杩囪姣掑寲鐨勫垎鏀?   鐩爣缂撳啿鍖猴紝骞跺湪 VM 閫€鍑烘椂鍒锋柊杩斿洖鏍堢紦鍐插尯銆傝繖鍙互闃叉鎭舵剰瀹㈡埛鏈哄奖鍝嶅涓绘満
   鍐呮牳涓殑闂存帴鍒嗘敮銆?
   涓轰簡淇濇姢瀹夸富鏈鸿繘绋嬪厤鍙楁伓鎰忓鎴锋満褰卞搷锛屽涓绘満杩涚▼鍙互閫氳繃 prctl() 绂佺敤鍏?   闂存帴鍒嗘敮鎺ㄦ祴銆傚湪鍒囨崲鍒版绫昏繘绋嬩箣鍓嶏紝浼氭竻闄ゅ垎鏀洰鏍囩紦鍐插尯銆?
##### 4. 铏氭嫙鍖栧鎴锋満鏀诲嚮鍏朵粬瀹㈡埛鏈?

   鎭舵剰瀹㈡埛鏈哄彲浠ユ敾鍑诲彟涓€涓鎴锋満锛屼互鑾峰彇璇ュ鎴锋満鍙闂殑鏁版嵁銆?
   濡傛灉鍙傛暟鍙互鍦ㄥ鎴锋満涔嬮棿浼犻€掞紝Spectre 鍙樹綋 1 鏀诲嚮鏄彲鑳界殑銆傝繖鍙互閫氳繃
   鍏变韩鍐呭瓨鎴栨秷鎭紶閫掔瓑鏈哄埗瀹屾垚銆傛绫诲弬鏁板彲鐢ㄤ簬鎺ㄥ鎸囧悜瀹㈡埛鏈轰腑鐗规潈鏁版嵁鐨?   鏁版嵁鎸囬拡銆傝鐗规潈鏁版嵁鍙兘琚彈瀹宠€呮帹娴嬭矾寰勪腑鐨?gadget 浠ｇ爜璁块棶銆?
   Spectre 鍙樹綋 2 鏀诲嚮鍙互鐢辨伓鎰忓鎴锋満閫氳繃姣掑寲 <poison_btb> 鍒嗘敮鐩爣缂撳啿鍖?   鎴栬繑鍥炴爤缂撳啿鍖哄彂璧枫€傝繖浜涜姣掑寲鐨勬潯鐩彲鐢ㄤ簬褰卞搷鍙楀鑰呭鎴锋満涓殑鎺ㄦ祴鎵ц璺緞銆?
   Linux 鍐呮牳閫氳繃鍦?VM 閫€鍑烘椂鍒锋柊杩斿洖鏍堢紦鍐插尯锛屼互鍙婂湪鍒囨崲鍒版柊瀹㈡埛鏈轰箣鍓嶆竻闄?   鍒嗘敮鐩爣缂撳啿鍖猴紝鏉ョ紦瑙ｅ鍚屼竴 CPU 纭欢绾跨▼涓婅繍琛岀殑鍏朵粬瀹㈡埛鏈虹殑鏀诲嚮銆?
   濡傛灉浣跨敤 SMT锛屾潵鑷厔寮熻秴绾跨▼涓笉鍙椾俊浠诲鎴锋満鐨?Spectre 鍙樹綋 2 鏀诲嚮鍙互鐢?   绠＄悊鍛橀€氳繃灏嗕笉瀹夊叏瀹㈡埛鏈虹殑闂存帴鍒嗘敮鎺ㄦ祴缁忕敱 prctl() 鍏抽棴鏉ョ紦瑙ｃ€傚鎴锋満涔?   鍙互閫氳繃鍦ㄨ嚜韬唴閮ㄥ紑鍚熀浜庡井鐮佺殑缂撹В锛堜緥濡?x86 涓婄殑 IBPB 鎴?STIBP锛夋潵淇濇姢
   鑷繁銆?

### Spectre 绯荤粺淇℃伅


Linux 鍐呮牳鎻愪緵涓€涓?sysfs 鎺ュ彛锛岀敤浜庢灇涓剧郴缁熼拡瀵?Spectre 鐨勫綋鍓嶇紦瑙ｇ姸鎬侊細
绯荤粺鏄惁鏄撳彈鏀诲嚮锛屼互鍙婂摢浜涚紦瑙ｆ帾鏂藉浜庢椿鍔ㄧ姸鎬併€?
鏄剧ず Spectre 鍙樹綋 1 缂撹В鐘舵€佺殑 sysfs 鏂囦欢鏄細

   /sys/devices/system/cpu/vulnerabilities/spectre_v1

璇ユ枃浠朵腑鍙兘鐨勫€间负锛?
```

     * - 'Not affected'
       - 澶勭悊鍣ㄤ笉鏄撳彈鏀诲嚮銆?     * - 'Vulnerable: __user pointer sanitization and usercopy barriers only; no swapgs barriers'
       - swapgs 淇濇姢琚鐢紱鍚﹀垯瀹冨湪鍐呮牳涓熀浜庡叿浣撴儏鍐碉紝閫氳繃鏄惧紡鐨勬寚閽堝噣鍖栧拰
         usercopy LFENCE 灞忛殰鎻愪緵淇濇姢銆?     * - 'Mitigation: usercopy/swapgs barriers and __user pointer sanitization'
       - 鍐呮牳涓熀浜庡叿浣撴儏鍐碉紝閫氳繃鏄惧紡鎸囬拡鍑€鍖栥€乽sercopy LFENCE 灞忛殰鍜?swapgs
         LFENCE 灞忛殰鎻愪緵淇濇姢銆?
```
鐒惰€岋紝杩欎簺淇濇姢鏄寜鍏蜂綋鎯呭喌瀹炴柦鐨勶紝骞朵笉鑳戒繚璇佽鐩?Spectre 鍙樹綋 1 鐨勬墍鏈夊彲鑳?鏀诲嚮閫斿緞銆?
spectre_v2 鍐呮牳鏂囦欢鎶ュ憡鍐呮牳鏄惁浣跨敤 retpoline 缂撹В缂栬瘧锛屾垨鑰?CPU 鏄惁鍏锋湁
纭欢缂撹В锛屼互鍙?CPU 鏄惁鏀寔棰濆鐨勩€佽繘绋嬬壒瀹氱殑缂撹В銆?
璇ユ枃浠惰繕鎶ュ憡鐢卞井鐮佸惎鐢ㄧ殑銆佺敤浜庣紦瑙ｇ敤鎴疯繘绋嬩箣闂存敾鍑荤殑 CPU 鐗规€э細

1. Indirect Branch Prediction Barrier锛圛BPB锛岄棿鎺ュ垎鏀娴嬪睆闅滐級浠ュ鍔犱笉鍚?   鐢ㄦ埛杩涚▼涔嬮棿鐨勯殧绂汇€?2. Single Thread Indirect Branch Predictors锛圫TIBP锛屽崟绾跨▼闂存帴鍒嗘敮棰勬祴鍣級浠?   澧炲姞杩愯鍦ㄥ悓涓€鏍镐笂鐨?CPU 绾跨▼涔嬮棿鐨勯殧绂汇€?
杩欎簺 CPU 鐗规€у湪浣跨敤鏃跺彲鑳藉奖鍝嶆€ц兘锛屽彲浠ユ寜杩涚▼鍩轰簬鍏蜂綋鎯呭喌鍚敤銆?
鏄剧ず Spectre 鍙樹綋 2 缂撹В鐘舵€佺殑 sysfs 鏂囦欢鏄細

   /sys/devices/system/cpu/vulnerabilities/spectre_v2

璇ユ枃浠朵腑鍙兘鐨勫€间负锛?
  - 鍐呮牳鐘舵€侊細

  ========================================  =================================
  'Not affected'                            The processor is not vulnerable
  'Mitigation: None'                        Vulnerable, no mitigation
  'Mitigation: Retpolines'                  Use Retpoline thunks
  'Mitigation: LFENCE'                      Use LFENCE instructions
  'Mitigation: Enhanced IBRS'               Hardware-focused mitigation
  'Mitigation: Enhanced IBRS + Retpolines'  Hardware-focused + Retpolines
  'Mitigation: Enhanced IBRS + LFENCE'      Hardware-focused + LFENCE
  ========================================  =================================

  - 鍥轰欢鐘舵€侊細鏄剧ず璋冪敤鍥轰欢鏃讹紙浠?x86锛夛紝鏄惁浣跨敤 Indirect Branch Restricted
    Speculation锛圛BRS锛夋潵闃茶寖 Spectre 鍙樹綋 2 鏀诲嚮銆?
  ========== =============================================================
  'IBRS_FW'  Protection against user program attacks when calling firmware
  ========== =============================================================

  - 闂存帴鍒嗘敮棰勬祴灞忛殰锛圛BPB锛夌姸鎬侊紝鐢ㄤ簬涓嶅悓鐢ㄦ埛杩涚▼涔嬮棿鐨勪繚鎶ゃ€傝鐗规€у彲浠ユ寜
    杩涚▼閫氳繃 prctl() 鎺у埗锛屾垨閫氳繃鍐呮牳鍛戒护琛岄€夐」鎺у埗銆傝繖鏄竴涓粎 x86 鐨勭壒鎬с€?    鏇村缁嗚妭瑙佷笅鏂囥€?
  ===================   ========================================================
  'IBPB: disabled'      IBPB unused
  'IBPB: always-on'     Use IBPB on all tasks
  'IBPB: conditional'   Use IBPB on SECCOMP or indirect branch restricted tasks
  ===================   ========================================================

  - 鍗曠嚎绋嬮棿鎺ュ垎鏀娴嬶紙STIBP锛夌姸鎬侊紝鐢ㄤ簬涓嶅悓瓒呯嚎绋嬩箣闂寸殑淇濇姢銆傝鐗规€у彲浠ユ寜
    杩涚▼閫氳繃 prctl 鎺у埗锛屾垨閫氳繃鍐呮牳鍛戒护琛岄€夐」鎺у埗銆傝繖鏄竴涓粎 x86 鐨勭壒鎬с€?    鏇村缁嗚妭瑙佷笅鏂囥€?
  ====================  ========================================================
  'STIBP: disabled'     STIBP unused
  'STIBP: forced'       Use STIBP on all tasks
  'STIBP: conditional'  Use STIBP on SECCOMP or indirect branch restricted tasks
  ====================  ========================================================

  - 杩斿洖鏍堢紦鍐插尯锛圧SB锛変繚鎶ょ姸鎬侊細

  =============   ===========================================
  'RSB filling'   Protection of RSB on context switch enabled
  =============   ===========================================

  - EIBRS 灞忛殰鍚庤繑鍥炴爤缂撳啿鍖猴紙PBRSB锛変繚鎶ょ姸鎬侊細

  ===========================  =======================================================
  'PBRSB-eIBRS: SW sequence'   CPU is affected and protection of RSB on VMEXIT enabled
  'PBRSB-eIBRS: Vulnerable'    CPU is vulnerable
  'PBRSB-eIBRS: Not affected'  CPU is not affected by PBRSB
  ===========================  =======================================================

  - 鍒嗘敮鍘嗗彶娉ㄥ叆锛圔HI锛変繚鎶ょ姸鎬侊細


 - - BHI: Not affected
   - System is not affected
 - - BHI: Retpoline
   - System is protected by retpoline
 - - BHI: BHI_DIS_S
   - System is protected by BHI_DIS_S
 - - BHI: SW loop, KVM SW loop
   - System is protected by software clearing sequence
 - - BHI: Vulnerable
   - System is vulnerable to BHI
 - - BHI: Vulnerable, KVM: SW loop
   - System is vulnerable; KVM is protected by software clearing sequence

瀹屾暣鐨勭紦瑙ｅ彲鑳介渶瑕佹潵鑷?CPU 渚涘簲鍟嗙殑寰爜鏇存柊銆傚綋蹇呰鐨勫井鐮佷笉鍙敤鏃讹紝鍐呮牳灏?鎶ュ憡婕忔礊銆?
### 寮€鍚拡瀵?Spectre 鍙樹綋 1 鍜?Spectre 鍙樹綋 2 鐨勭紦瑙?

##### 1. 鍐呮牳缂撹В


#### Spectre 鍙樹綋 1


   瀵逛簬 Spectre 鍙樹綋 1锛屾槗鍙楁敾鍑荤殑鍐呮牳浠ｇ爜锛堢敱浠ｇ爜瀹℃煡鎴栨壂鎻忓伐鍏风‘瀹氾級鍩轰簬
   鍏蜂綋鎯呭喌杩涜鏍囨敞锛屼互浣跨敤 nospec 璁块棶鍣ㄥ畯杩涜杈圭晫瑁佸壀 :ref:`[^2^]
   <spec_ref2>`锛屼互閬垮厤浠讳綍鍙敤鐨勬硠闇?gadget銆傜劧鑰岋紝瀹冨彲鑳芥棤娉曡鐩?Spectre
   鍙樹綋 1 鐨勬墍鏈夋敾鍑婚€斿緞銆?
   浠庣敤鎴峰鍒讹紙copy-from-user锛変唬鐮佹湁涓€涓?LFENCE 灞忛殰锛屼互闃叉 access_ok()
   妫€鏌ヨ閿欒鎺ㄦ祴銆傝灞忛殰鐢?barrier_nospec() 瀹忓畬鎴愩€?
   瀵逛簬 Spectre 鍙樹綋 1 鐨?swapgs 鍙樹綋锛屽湪闇€瑕佹椂锛屼細鍦ㄤ腑鏂€佸紓甯稿拰 NMI 鍏ュ彛
   娣诲姞 LFENCE 灞忛殰銆傝繖浜涘睆闅滅敱 FENCE_SWAPGS_KERNEL_ENTRY 鍜?   FENCE_SWAPGS_USER_ENTRY 瀹忓畬鎴愩€?
#### Spectre 鍙樹綋 2


   瀵逛簬 Spectre 鍙樹綋 2 缂撹В锛岀紪璇戝櫒灏嗗唴鏍镐腑鐨勯棿鎺ヨ皟鐢ㄦ垨璺宠浆杞崲涓虹瓑浠风殑
   杩斿洖韫﹀簥锛坮etpolines锛塠^3^] <spec_ref3> [^9^] <spec_ref9> 浠ヨ烦杞埌鐩爣
   鍦板潃銆傝繑鍥炶功搴婁笅鐨勬帹娴嬫墽琛岃矾寰勮鎹曡幏鍦ㄤ竴涓棤闄愬惊鐜腑锛屼互闃叉浠讳綍鎺ㄦ祴鎵ц
   璺宠浆鍒?gadget銆?
   瑕佸湪鏄撳彈鏀诲嚮鐨?CPU 涓婂紑鍚?retpoline 缂撹В锛屽唴鏍搁渶瑕佷娇鐢ㄦ敮鎸?   -mindirect-branch=thunk-extern -mindirect-branch-register 閫夐」鐨?gcc 缂栬瘧鍣?   缂栬瘧銆傚鏋滃唴鏍镐娇鐢?Clang 缂栬瘧鍣ㄧ紪璇戯紝缂栬瘧鍣ㄩ渶瑕佹敮鎸?-mretpoline-external-thunk
   閫夐」銆傞渶瑕佸紑鍚唴鏍搁厤缃?CONFIG_MITIGATION_RETPOLINE锛屽苟涓?CPU 闇€瑕佽繍琛屾渶鏂扮殑
   鏇存柊寰爜銆?
   鍦?Intel Skylake 鏃朵唬鐨勭郴缁熶笂锛岀紦瑙ｈ鐩栦簡澶у鏁颁絾骞堕潪鍏ㄩ儴鎯呭喌銆傛洿澶氱粏鑺?   鍙傝 [^3^] <spec_ref3>銆?
   鍦ㄥ叿鏈夐拡瀵?Spectre 鍙樹綋 2 鐨勭‖浠剁紦瑙ｏ紙渚嬪 x86 涓婄殑 IBRS 鎴?enhanced IBRS锛?   鐨?CPU 涓婏紝retpoline 鍦ㄨ繍琛屾椂琚嚜鍔ㄧ鐢ㄣ€?
   鏀寔 enhanced IBRS锛坋IBRS锛夌殑绯荤粺鍦ㄥ惎鍔ㄦ椂閫氳繃缃綅 IBRS 浣嶄竴娆℃€у紑鍚?IBRS
   淇濇姢锛屽畠浠嚜鍔ㄥ彈鍒伴拡瀵规煇浜?Spectre v2 鍙樹綋鏀诲嚮鐨勪繚鎶ゃ€侭HB 浠嶇劧鍙互褰卞搷闂存帴
   鍒嗘敮棰勬祴鍣ㄦ潯鐩殑閫夋嫨锛屽苟涓斿敖绠″湪鍚敤 eIBRS 鏃跺垎鏀娴嬪櫒鏉＄洰鍦ㄦā寮忎箣闂磋
   闅旂锛孊HB 鏈韩鍦ㄦā寮忎箣闂村苟鏈闅旂銆傛敮鎸?BHI_DIS_S 鐨勭郴缁熶細璁剧疆瀹冧互闃插
   BHI 鏀诲嚮銆?
   鍦?Intel 鐨?enhanced IBRS 绯荤粺涓婏紝杩欏寘鎷?SMT 绯荤粺锛圫TIBP锛変笂鐨勮法绾跨▼鍒嗘敮
   鐩爣娉ㄥ叆銆傛崲鍙ヨ瘽璇达紝Intel eIBRS 涔熷惎鐢ㄤ簡 STIBP銆?
   AMD Automatic IBRS 涓嶄繚鎶ょ敤鎴风┖闂达紝鑰?Legacy IBRS 绯荤粺鍦ㄨ繑鍥炵敤鎴风┖闂存椂娓呴櫎
   IBRS 浣嶏紝鍥犳涓よ€呴兘鏄惧紡鍦板惎鐢?STIBP銆?
   retpoline 缂撹В鍦ㄦ槗鍙楁敾鍑荤殑 CPU 涓婇粯璁ゅ紑鍚€傜鐞嗗憳鍙互閫氳繃鍐呮牳鍛戒护琛屽拰
   sysfs 鎺у埗鏂囦欢寮哄埗寮€鍚垨鍏抽棴瀹冦€傝鍙傞槄 spectre_mitigation_control_command_line銆?
   鍦?x86 涓婏紝鍦ㄨ皟鐢ㄤ换浣曞浐浠朵唬鐮佷箣鍓嶏紝榛樿寮€鍚棿鎺ュ垎鏀檺鍒舵帹娴嬶紝浠ラ槻姝㈠埄鐢?   鍥轰欢鐨?Spectre 鍙樹綋 2 鏀诲嚮銆?
   浣跨敤鍐呮牳鍦板潃绌洪棿闅忔満鍖栵紙鍐呮牳閰嶇疆涓殑 CONFIG_RANDOMIZE_BASE=y 鍜?   CONFIG_SLAB_FREELIST_RANDOM=y锛変娇閽堝鍐呮牳鐨勬敾鍑婚€氬父鏇村洶闅俱€?
##### 2. 鐢ㄦ埛绋嬪簭缂撹В


   鐢ㄦ埛绋嬪簭鍙互浣跨敤 LFENCE 鎴栤€滆竟鐣岃鍓紙bounds clipping锛夆€濇潵缂撹В Spectre
   鍙樹綋 1銆傛洿澶氱粏鑺傚弬瑙?[^2^] <spec_ref2>銆?
   瀵逛簬 Spectre 鍙樹綋 2 缂撹В锛屽崟涓敤鎴风▼搴忓彲浠ョ敤閽堝闂存帴鍒嗘敮鐨勮繑鍥炶功搴婄紪璇戙€?   杩欎繚鎶ゅ畠浠厤鍙楁伓鎰忚蒋浠剁暀鍦ㄥ垎鏀洰鏍囩紦鍐插尯涓殑琚瘨鍖栨潯鐩殑娑堣垂銆?
   鍦?legacy IBRS 绯荤粺涓婏紝鍦ㄨ繑鍥炵敤鎴风┖闂存椂锛岄殣寮?STIBP 琚鐢紝鍥犱负鍐呮牳娓呴櫎浜?   IBRS 浣嶃€傚湪杩欑鎯呭喌涓嬶紝鐢ㄦ埛绌洪棿绋嬪簭鍙互閫氳繃 prctl() 绂佺敤鍏堕棿鎺ュ垎鏀帹娴?   锛堝弬瑙?Documentation/userspace-api/spec_ctrl.rst <set_spec_ctrl>锛夈€傚湪 x86
   涓婏紝杩欏皢鍦ㄧ敤鎴风▼搴忚繍琛屾椂寮€鍚?STIBP 浠ラ槻澶囨潵鑷厔寮熺嚎绋嬬殑鏀诲嚮锛屽苟鍦ㄥ垏鎹㈠埌/
   浠庤绋嬪簭鐨勫垏鎹腑浣跨敤 IBPB 鍒锋柊鍒嗘敮鐩爣缂撳啿鍖恒€?
   闄愬埗鐢ㄦ埛绋嬪簭鐨勯棿鎺ュ垎鏀帹娴嬩篃灏嗛槻姝㈣绋嬪簭鍦?x86 涓婂彂璧峰彉浣?2 鏀诲嚮銆傜鐞嗗憳
   鍙互閫氳繃鍐呮牳鍛戒护琛屽拰 sysfs 鎺у埗鏂囦欢鏀瑰彉杩欑琛屼负銆傝鍙傞槄
   spectre_mitigation_control_command_line銆?
   绂佺敤鍏堕棿鎺ュ垎鏀帹娴嬬殑绋嬪簭浼氭湁鏇村寮€閿€骞惰繍琛屽緱鏇存參銆?
   鐢ㄦ埛绋嬪簭搴斾娇鐢ㄥ湴鍧€绌洪棿闅忔満鍖栵紙/proc/sys/kernel/randomize_va_space = 1 鎴?   2锛夋潵浣挎敾鍑绘洿鍥伴毦銆?
##### 3. 铏氭嫙鏈虹紦瑙?

   鍦ㄥ唴鏍稿唴閮紝鏉ヨ嚜鎭舵剰瀹㈡埛鏈虹殑 Spectre 鍙樹綋 1 鏀诲嚮鍦?VM 閫€鍑鸿矾寰勪笂鍩轰簬鍏蜂綋
   鎯呭喌琚紦瑙ｃ€傛槗鍙楁敾鍑荤殑浠ｇ爜浣跨敤 nospec 璁块棶鍣ㄥ畯杩涜鈥滆竟鐣岃鍓€濓紝浠ラ伩鍏嶄换浣?   鍙敤鐨勬硠闇?gadget銆傜劧鑰岋紝杩欏彲鑳芥棤娉曡鐩栨墍鏈夊彉浣?1 鏀诲嚮閫斿緞銆?
   閽堝鏉ヨ嚜鎭舵剰瀹㈡埛鏈哄埌鍐呮牳鐨?Spectre 鍙樹綋 2 鏀诲嚮锛孡inux 鍐呮牳浣跨敤 retpoline
   鎴?Enhanced IBRS 鏉ラ槻姝㈡秷璐规伓鎰忓鎴锋満鐣欏湪鍒嗘敮鐩爣缂撳啿鍖轰腑鐨勮姣掑寲鏉＄洰銆傚畠
   杩樹細鍦ㄦ瘡娆?VM 閫€鍑烘椂鍒锋柊杩斿洖鏍堢紦鍐插尯锛屼互闃叉杩斿洖鏍堢紦鍐插尯鐨勪笅婧紝浠庤€屼娇琚?   姣掑寲鐨勫垎鏀洰鏍囩紦鍐插尯鍙浣跨敤锛屾垨闃叉鏀诲嚮瀹㈡埛鏈哄湪杩斿洖鏍堢紦鍐插尯涓暀涓嬭姣掑寲
   鐨勬潯鐩€?
   涓轰簡缂撹В鍚屼竴 CPU 纭欢绾跨▼涓婄殑瀹㈡埛鏈哄埌瀹㈡埛鏈烘敾鍑伙紝鍦ㄥ垏鎹㈠埌 CPU 涓婄殑鏂板鎴锋満
   涔嬪墠锛岄€氳繃鍒锋柊鏉ュ噣鍖栧垎鏀洰鏍囩紦鍐插尯銆?
   涓婅堪缂撹В鍦ㄦ槗鍙楁敾鍑荤殑 CPU 涓婇粯璁ゅ紑鍚€?
   涓轰簡缂撹В SMT 浣跨敤鏃舵潵鑷厔寮熺嚎绋嬬殑瀹㈡埛鏈哄埌瀹㈡埛鏈烘敾鍑伙紝鍦ㄥ厔寮熺嚎绋嬩腑杩愯鐨?   涓嶅彈淇′换瀹㈡埛鏈哄彲浠ョ敱绠＄悊鍛樼粡鐢?prctl() 绂佺敤鍏堕棿鎺ュ垎鏀帹娴嬨€?
   鍐呮牳杩樺厑璁稿鎴锋満浣跨敤瀹冧滑閫夋嫨鐨勪换浣曞熀浜庡井鐮佺殑缂撹В锛堜緥濡?x86 涓婄殑 IBPB 鎴?   STIBP锛夋潵淇濇姢鑷繁銆?

### 鍐呮牳鍛戒护琛屼笂鐨勭紦瑙ｆ帶鍒?

涓€鑸€岃█锛屽唴鏍镐細涓哄綋鍓?CPU 閫夋嫨鍚堢悊鐨勯粯璁ょ紦瑙ｆ帾鏂姐€?
Spectre 榛樿缂撹В鍙互閫氳繃浠ヤ笅閫夐」鍦ㄥ唴鏍稿懡浠よ涓婄鐢ㄦ垨鏇存敼锛?
 - nospectre_v1
 - nospectre_v2
 - spectre_v2={option}
 - spectre_v2_user={option}
 - spectre_bhi={option}

鏈夊叧鍙敤閫夐」鐨勬洿澶氱粏鑺傦紝璇峰弬闃?Documentation/admin-guide/kernel-parameters.txt

### 缂撹В閫夋嫨鎸囧崡


##### 1. 鍙俊鐢ㄦ埛绌洪棿


   濡傛灉鎵€鏈夌敤鎴风┖闂村簲鐢ㄩ兘鏉ヨ嚜鍙俊鏉ユ簮锛屽苟涓斾笉鎵ц澶栭儴鎻愪緵鐨勪笉鍙椾俊浠讳唬鐮侊紝
   閭ｄ箞鍙互绂佺敤缂撹В鎺柦銆?
##### 2. 淇濇姢鏁忔劅绋嬪簭


   瀵逛簬甯︽湁绉樺瘑锛堜緥濡傚姞瀵嗗瘑閽ワ級鐨勫畨鍏ㄦ晱鎰熺▼搴忥紝鍙互鍦ㄧ▼搴忚繍琛屾椂閫氳繃绂佺敤鍏?   闂存帴鍒嗘敮鎺ㄦ祴鏉ュ疄鏂介拡瀵?Spectre 鍙樹綋 2 鐨勪繚鎶わ紙鍙傝
   Documentation/userspace-api/spec_ctrl.rst <set_spec_ctrl>锛夈€?
##### 3. 娌欑鍖栦笉鍙椾俊浠荤▼搴?

   鍙兘鎴愪负鏀诲嚮鏉ユ簮鐨勪笉鍙椾俊浠荤▼搴忓彲浠ラ€氳繃鍦ㄨ繍琛屾椂绂佺敤鍏堕棿鎺ュ垎鏀帹娴嬫潵闅旂
   锛堝弬瑙?Documentation/userspace-api/spec_ctrl.rst <set_spec_ctrl>锛夈€傝繖闃叉
   涓嶅彈淇′换绋嬪簭姹℃煋鍒嗘敮鐩爣缂撳啿鍖恒€傝繖绉嶈涓哄彲浠ラ€氳繃鍐呮牳鍛戒护琛屽拰 sysfs 鎺у埗
   鏂囦欢鏀瑰彉銆傝鍙傞槄 spectre_mitigation_control_command_line銆?
##### 3. 楂樺畨鍏ㄦā寮?

   鎵€鏈?Spectre 鍙樹綋 2 缂撹В閮藉彲浠ュ湪鍚姩鏃跺鎵€鏈夌▼搴忓己鍒跺紑鍚紙鍙傝
   spectre_mitigation_control_command_line 涓殑鈥渙n鈥濋€夐」锛夈€傝繖灏嗗鍔犲紑閿€锛屽洜涓?   鎵€鏈夌▼搴忕殑闂存帴鍒嗘敮鎺ㄦ祴閮藉皢鍙楀埌闄愬埗銆?
   鍦?x86 涓婏紝鍦ㄥ垏鎹㈠埌鏂扮▼搴忔椂锛屽垎鏀洰鏍囩紦鍐插尯灏嗛€氳繃 IBPB 鍒锋柊銆係TIBP 濮嬬粓
   淇濇寔寮€鍚紝浠ヤ繚鎶ょ▼搴忓厤鍙楁潵鑷厔寮熺嚎绋嬩笂杩愯鐨勭▼搴忕殑鍙樹綋 2 鏀诲嚮銆?
   鎴栬€咃紝STIBP 鍙互浠呯敤浜庤繍琛岄偅浜涢棿鎺ュ垎鏀帹娴嬭鏄惧紡绂佺敤鐨勭▼搴忥紝鑰?IBPB 浠?   鍦ㄥ垏鎹㈠埌鏂扮▼搴忔椂濮嬬粓浣跨敤锛屼互娓呴櫎鍒嗘敮鐩爣缂撳啿鍖猴紙鍙傝
   spectre_mitigation_control_command_line 涓殑鈥渋bpb鈥濋€夐」锛夈€傝繖涓€渋bpb鈥濋€夐」
   姣斺€渙n鈥濋€夐」鎬ц兘浠ｄ环鏇村皬锛屽悗鑰呬細璁?STIBP 濮嬬粓寮€鍚€?
### Spectre 鐩稿叧鍙傝€冭祫鏂?

Intel 鐧界毊涔︼細


[^1^] `Intel analysis of speculative execution side channels <https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/analysis-of-speculative-execution-side-channels-white-paper.pdf>`_.


[^2^] `Bounds check bypass <https://software.intel.com/security-software-guidance/software-guidance/bounds-check-bypass>`_.


[^3^] `Deep dive: Retpoline: A branch target injection mitigation <https://software.intel.com/security-software-guidance/insights/deep-dive-retpoline-branch-target-injection-mitigation>`_.


[^4^] `Deep Dive: Single Thread Indirect Branch Predictors <https://software.intel.com/security-software-guidance/insights/deep-dive-single-thread-indirect-branch-predictors>`_.

AMD 鐧界毊涔︼細


[^5^] `AMD64 technology indirect branch control extension <https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/white-papers/111006-architecture-guidelines-update-amd64-technology-indirect-branch-control-extension.pdf>`_.


[^6^] `Software techniques for managing speculation on AMD processors <https://developer.amd.com/wp-content/resources/Managing-Speculation-on-AMD-Processors.pdf>`_.

ARM 鐧界毊涔︼細


[^7^] `Cache speculation side-channels <https://developer.arm.com/support/arm-security-updates/speculative-processor-vulnerability/download-the-whitepaper>`_.


[^8^] `Cache speculation issues update <https://developer.arm.com/support/arm-security-updates/speculative-processor-vulnerability/latest-updates/cache-speculation-issues-update>`_.

Google 鐧界毊涔︼細


[^9^] `Retpoline: a software construct for preventing branch-target-injection <https://support.google.com/faqs/answer/7625886>`_.

MIPS 鐧界毊涔︼細


[^10^] `MIPS: response on speculative execution and side channel vulnerabilities <https://web.archive.org/web/20220512003005if_/https://www.mips.com/blog/mips-response-on-speculative-execution-and-side-channel-vulnerabilities/>`_.

瀛︽湳璁烘枃锛?

[^11^] `Spectre Attacks: Exploiting Speculative Execution <https://spectreattack.com/spectre.pdf>`_.


[^12^] `NetSpectre: Read Arbitrary Memory over Network <https://arxiv.org/abs/1807.10535>`_.


[^13^] `Spectre Returns! Speculation Attacks using the Return Stack Buffer <https://www.usenix.org/system/files/conference/woot18/woot18-paper-koruyeh.pdf>`_.
