
## eBPF 鏍￠獙鍣?
eBPF 绋嬪簭鐨勫畨鍏ㄦ€у垎涓ゆ纭畾銆?
绗竴姝ヨ繘琛?DAG锛堟湁鍚戞棤鐜浘锛夋鏌ワ紝浠ョ姝㈠惊鐜拰鍏朵粬 CFG锛堟帶鍒舵祦鍥撅級鏍￠獙銆傜壒鍒?鏄畠浼氭娴嬪嚭鍚湁涓嶅彲杈炬寚浠ょ殑绋嬪簭銆傦紙灏界缁忓吀 BPF 鏍￠獙鍣ㄥ厑璁稿畠浠級

绗簩姝ヤ粠绗竴鏉℃寚浠ゅ紑濮嬶紝閬嶅巻鎵€鏈夊彲鑳借矾寰勩€傚畠妯℃嫙姣忔潯鎸囦护鐨勬墽琛屽苟瑙傚療瀵勫瓨鍣ㄤ笌
鏍堢殑鐘舵€佸彉鍖栥€?
鍦ㄧ▼搴忓紑濮嬫椂锛屽瘎瀛樺櫒 R1 鍖呭惈涓€涓寚鍚?context 鐨勬寚閽堬紝绫诲瀷涓?PTR_TO_CTX銆?濡傛灉鏍￠獙鍣ㄧ湅鍒颁竴鏉?R2=R1 鐨勬寚浠わ紝閭ｄ箞 R2 鐜板湪涔熷叿鏈夌被鍨?PTR_TO_CTX锛屽苟涓斿彲浠?鐢ㄥ湪琛ㄨ揪寮忕殑鍙充晶銆傚鏋?R1=PTR_TO_CTX 涓旀寚浠や负 R2=R1+R1锛岄偅涔?R2=SCALAR_VALUE锛?鍥犱负涓や釜鏈夋晥鎸囬拡鐩稿姞浼氫骇鐢熸棤鏁堟寚閽堛€傦紙鍦?secure"妯″紡涓嬶紝鏍￠獙鍣ㄥ皢鎷掔粷浠讳綍绫诲瀷
鐨勬寚閽堢畻鏈紝浠ョ‘淇濆唴鏍稿湴鍧€涓嶄細娉勬紡缁欓潪鐗规潈鐢ㄦ埛锛?
```
  bpf_mov R0 = R2
  bpf_exit
```

灏嗚鎷掔粷锛屽洜涓哄湪绋嬪簭寮€濮嬪 R2 鏄笉鍙鐨勩€?
鍦ㄥ唴鏍稿嚱鏁拌皟鐢ㄤ箣鍚庯紝R1-R5 琚噸缃负涓嶅彲璇伙紝鑰?R0 鍏锋湁璇ュ嚱鏁扮殑杩斿洖绫诲瀷銆?
鐢变簬 R6-R9 鏄璋冪敤鏂逛繚瀛橈紙callee saved锛夌殑锛屽畠浠殑鐘舵€佸湪璋冪敤涔嬮棿鏄繚鐣欑殑銆?
```
  bpf_mov R6 = 1
  bpf_call foo
  bpf_mov R0 = R6
  bpf_exit
```

鏄竴涓纭殑绋嬪簭銆傚鏋滆繖閲岀敤鐨勬槸 R1 鑰屼笉鏄?R6锛屽畠灏变細琚嫆缁濄€?
load/store 鎸囦护鍙厑璁镐娇鐢ㄥ叿鏈夋湁鏁堢被鍨嬬殑瀵勫瓨鍣紝杩欎簺绫诲瀷鏄?PTR_TO_CTX銆?PTR_TO_MAP銆丳TR_TO_STACK銆傚畠浠細缁忚繃杈圭晫鍜屽鍏舵鏌ャ€?
```
 bpf_mov R1 = 1
 bpf_mov R2 = 2
 bpf_xadd *(u32 *)(R1 + 3) += R2
 bpf_exit
```

灏嗚鎷掔粷锛屽洜涓哄湪鎵ц bpf_xadd 鎸囦护鏃?R1 涓嶅叿澶囨湁鏁堢殑鎸囬拡绫诲瀷銆?
鍦ㄥ紑濮嬫椂 R1 鐨勭被鍨嬩负 PTR_TO_CTX锛堟寚鍚戦€氱敤 `struct bpf_context` 鐨勬寚閽堬級銆備娇鐢?涓€涓洖璋冩潵鑷畾涔夋牎楠屽櫒锛屼互灏?eBPF 绋嬪簭瀵?ctx 缁撴瀯浣撳唴鐗瑰畾瀛楁锛堝叿鏈夋寚瀹氬ぇ灏忓拰
瀵归綈锛夌殑璁块棶闄愬埗涓轰粎鍏佽杩欎簺瀛楁銆?
```
  bpf_ld R0 = *(u32 *)(R6 + 8)
```

鎰忓浘浠庡湴鍧€ R6 + 8 鍔犺浇涓€涓瓧骞跺皢鍏跺瓨鍏?R0銆傚鏋?R6=PTR_TO_CTX锛岄€氳繃
is_valid_access() 鍥炶皟锛屾牎楠屽櫒灏嗙煡閬撳亸绉婚噺 8銆佸ぇ灏忎负 4 瀛楄妭鐨勫尯鍩熷彲浠ヨ璇诲彇璁块棶锛?鍚﹀垯鏍￠獙鍣ㄥ皢鎷掔粷璇ョ▼搴忋€傚鏋?R6=PTR_TO_STACK锛岄偅涔堣闂簲褰撴槸瀵归綈鐨勪笖浣嶄簬鏍堣竟鐣?[-MAX_BPF_STACK, 0) 涔嬪唴銆傚湪姝や緥涓亸绉婚噺涓?8锛屽洜姝ゅ畠灏嗘棤娉曢€氳繃鏍￠獙锛屽洜涓哄畠瓒婄晫浜嗐€?
鏍￠獙鍣ㄥ彧鍏佽 eBPF 绋嬪簭鍦ㄥ悜鏍堜腑鍐欏叆涔嬪悗浠庢爤涓鍙栨暟鎹€?
缁忓吀 BPF 鏍￠獙鍣ㄥ M[0-15] 鍐呭瓨妲藉仛绫讳技鐨勬鏌ャ€?
```
  bpf_ld R0 = *(u32 *)(R10 - 4)
  bpf_exit
```

鏄棤鏁堢殑绋嬪簭銆傚敖绠?R10 鏄纭殑鍙瀵勫瓨鍣ㄤ笖绫诲瀷涓?PTR_TO_STACK锛屽苟涓?R10 - 4
鍦ㄦ爤杈圭晫涔嬪唴锛屼絾閭ｉ噷浠庢湭琚啓鍏ヨ繃銆?
鎸囬拡瀵勫瓨鍣ㄧ殑婧㈠嚭/濉厖锛坰pill/fill锛変篃琚窡韪紝鍥犱负鍥涗釜锛圧6-R9锛夎璋冪敤鏂逛繚瀛樺瘎瀛樺櫒
瀵规煇浜涚▼搴忔潵璇村彲鑳戒笉澶熺敤銆?
鍏佽鐨勫嚱鏁拌皟鐢ㄩ€氳繃 bpf_verifier_ops->get_func_proto() 鑷畾涔夈€俥BPF 鏍￠獙鍣ㄥ皢妫€鏌?瀵勫瓨鍣ㄦ槸鍚﹀尮閰嶅弬鏁扮害鏉熴€傝皟鐢ㄤ箣鍚庯紝瀵勫瓨鍣?R0 灏嗚璁剧疆涓鸿鍑芥暟鐨勮繑鍥炵被鍨嬨€?
鍑芥暟璋冪敤鏄墿灞?eBPF 绋嬪簭鍔熻兘鐨勪富瑕佹満鍒躲€傚鎺ュ瓧杩囨护鍣ㄥ彲鑳藉厑璁哥▼搴忚皟鐢ㄤ竴缁勫嚱鏁帮紝鑰?璺熻釜杩囨护鍣ㄥ彲鑳藉厑璁镐竴缁勫畬鍏ㄤ笉鍚岀殑鍑芥暟銆?
濡傛灉涓€涓嚱鏁拌寮€鏀剧粰 eBPF 绋嬪簭浣跨敤锛岄偅涔堜粠瀹夊叏瑙掑害闇€瑕佷粩缁嗚€冭檻銆傛牎楠屽櫒灏嗕繚璇佽鍑芥暟
浠ユ湁鏁堢殑鍙傛暟琚皟鐢ㄣ€?
seccomp 涓庡鎺ュ瓧杩囨护鍣ㄥ缁忓吀 BPF 鏈変笉鍚岀殑瀹夊叏闄愬埗銆係eccomp 閫氳繃涓ら樁娈垫牎楠屽櫒瑙ｅ喅杩?涓棶棰橈細缁忓吀 BPF 鏍￠獙鍣ㄤ箣鍚庤窡闅?seccomp 鏍￠獙鍣ㄣ€傝€屽湪 eBPF 涓紝涓€涓彲閰嶇疆鐨勬牎楠屽櫒
琚墍鏈夌敤渚嬪叡浜€?
eBPF 鏍￠獙鍣ㄧ殑缁嗚妭鍙傝 kernel/bpf/verifier.c銆?
## 瀵勫瓨鍣ㄥ€艰窡韪?
涓轰簡纭畾 eBPF 绋嬪簭鐨勫畨鍏ㄦ€э紝鏍￠獙鍣ㄥ繀椤昏窡韪瘡涓瘎瀛樺櫒浠ュ強姣忎釜鏍堟Ы涓彲鑳藉嚭鐜扮殑鍊肩殑
鑼冨洿銆傝繖鏄€氳繃 `struct bpf_reg_state` 瀹屾垚鐨勶紝瀹冨畾涔変簬 include/linux/bpf_verifier.h锛?缁熶竴浜嗗鏍囬噺鍊煎拰鎸囬拡鍊肩殑璺熻釜銆傛瘡涓瘎瀛樺櫒鐘舵€佹湁涓€涓被鍨嬶紝瀹冭涔堟槸 NOT_INIT锛堝瘎瀛樺櫒
灏氭湭琚啓鍏ワ級锛岃涔堟槸 SCALAR_VALUE锛堟煇涓笉鍙敤浣滄寚閽堢殑鍊硷級锛岃涔堟槸涓€涓寚閽堢被鍨嬨€傛寚閽?鐨勭被鍨嬫弿杩颁簡鍏跺熀鍧€锛屽涓嬶細

    PTR_TO_CTX
			鎸囧悜 bpf_context 鐨勬寚閽堛€?    CONST_PTR_TO_MAP
			鎸囧悜 struct bpf_map 鐨勬寚閽堛€?Const"锛堝父閲忥級鏄洜涓虹姝㈠杩欎簺
			鎸囬拡杩涜绠楁湳杩愮畻銆?    PTR_TO_MAP_VALUE
			鎸囧悜瀛樺偍鍦?map 鍏冪礌涓殑鍊肩殑鎸囬拡銆?    PTR_TO_MAP_VALUE_OR_NULL
			瑕佷箞鏄寚鍚?map 鍊肩殑鎸囬拡锛岃涔堟槸 NULL锛沵ap 璁块棶锛堣 maps.rst锛?			杩斿洖姝ょ被鍨嬶紝褰撹妫€鏌?!= NULL 鏃跺畠鍙樹负 PTR_TO_MAP_VALUE銆傜姝?			瀵硅繖浜涙寚閽堣繘琛岀畻鏈繍绠椼€?    PTR_TO_STACK
			甯ф寚閽堬紙frame pointer锛夈€?    PTR_TO_PACKET
			skb->data銆?    PTR_TO_PACKET_END
			skb->data + headlen锛涚姝㈢畻鏈繍绠椼€?    PTR_TO_SOCKET
			鎸囧悜 struct bpf_sock_ops 鐨勬寚閽堬紝闅愬紡寮曠敤璁℃暟銆?    PTR_TO_SOCKET_OR_NULL
			瑕佷箞鏄寚鍚?socket 鐨勬寚閽堬紝瑕佷箞鏄?NULL锛泂ocket 鏌ユ壘杩斿洖姝ょ被鍨嬶紝
			褰撹妫€鏌?!= NULL 鏃跺畠鍙樹负 PTR_TO_SOCKET銆侾TR_TO_SOCKET 鏄紩鐢?			璁℃暟鐨勶紝鍥犳绋嬪簭蹇呴』鍦ㄧ▼搴忕粨鏉熷墠閫氳繃 socket 閲婃斁鍑芥暟閲婃斁璇ュ紩鐢ㄣ€?			绂佹瀵硅繖浜涙寚閽堣繘琛岀畻鏈繍绠椼€?
鐒惰€岋紝涓€涓寚閽堝彲鑳界浉瀵逛簬杩欎釜鍩哄潃鏈夊亸绉伙紙浣滀负鎸囬拡绠楁湳鐨勭粨鏋滐級锛岃繖閫氳繃涓ら儴鍒嗚窡韪細
'fixed offset'锛堝浐瀹氬亸绉伙級鍜?'variable offset'锛堝彲鍙樺亸绉伙級銆傚墠鑰呭湪灏嗕竴涓‘鍒囧凡鐭ョ殑鍊?锛堜緥濡備竴涓珛鍗虫暟鎿嶄綔鏁帮級鍔犲埌涓€涓寚閽堟椂浣跨敤锛屽悗鑰呯敤浜庝笉瀹屽叏纭畾宸茬煡鐨勫€笺€傚彲鍙樺亸绉?涔熺敤浜?SCALAR_VALUE锛屼互璺熻釜瀵勫瓨鍣ㄤ腑鍙兘鍑虹幇鐨勫€肩殑鑼冨洿銆?
鏍￠獙鍣ㄥ叧浜庡彲鍙樺亸绉荤殑鐭ヨ瘑鍖呮嫭锛?
- 浣滀负鏃犵鍙锋暟鐨勬渶灏忓€煎拰鏈€澶у€?- 浣滀负鏈夌鍙锋暟鐨勬渶灏忓€煎拰鏈€澶у€?
- 鍗曚釜姣旂壒鍊肩殑鐭ヨ瘑锛屽舰寮忎负 'tnum'锛氫竴涓?u64 鐨?'mask' 鍜屼竴涓?u64 鐨?'value'銆俶ask
  涓殑 1 琛ㄧず鍊兼湭鐭ョ殑姣旂壒锛泇alue 涓殑 1 琛ㄧず宸茬煡涓?1 鐨勬瘮鐗广€傚凡鐭ヤ负 0 鐨勬瘮鐗瑰湪 mask
  鍜?value 涓兘涓?0锛涙病鏈変换浣曟瘮鐗瑰簲璇ュ湪涓よ€呬腑閮戒负 1銆備緥濡傦紝濡傛灉涓€涓瓧鑺備粠鍐呭瓨璇诲叆
  涓€涓瘎瀛樺櫒锛岃瀵勫瓨鍣ㄧ殑楂?56 浣嶅凡鐭ヤ负 0锛岃€屼綆 8 浣嶆湭鐭モ€斺€旇繖琛ㄧず涓?tnum (0x0; 0xff)銆?  濡傛灉鎴戜滑鍐嶅皢鍏朵笌 0x40 鍋?OR 杩愮畻锛屽緱鍒?(0x40; 0xbf)锛屽鏋滄垜浠啀鍔?1 鍒欏緱鍒?(0x0;
  0x1ff)锛屽洜涓哄彲鑳芥湁杩涗綅銆?
闄や簡绠楁湳杩愮畻锛屽瘎瀛樺櫒鐘舵€佷篃鍙互琚潯浠跺垎鏀洿鏂般€備緥濡傦紝濡傛灉涓€涓?SCALAR_VALUE 琚瘮杈?> 8锛屽湪 'true'锛堢湡锛夊垎鏀腑瀹冨皢鏈変竴涓?umin_value锛堟棤绗﹀彿鏈€灏忓€硷級涓?9锛岃€屽湪 'false'
锛堝亣锛夊垎鏀腑瀹冨皢鏈変竴涓?umax_value 涓?8銆傛湁绗﹀彿姣旇緝锛堜娇鐢?BPF_JSGT 鎴?BPF_JSGE锛夊皢
鏀逛负鏇存柊鏈夌鍙风殑鏈€灏忓€?鏈€澶у€笺€傛潵鑷湁绗﹀彿鍜屾棤绗﹀彿杈圭晫鐨勪俊鎭彲浠ョ粍鍚堬紱渚嬪濡傛灉涓€涓?鍊煎厛琚祴璇?< 8 鐒跺悗琚祴璇?s> 4锛屾牎楠屽櫒灏嗘帹鏂嚭璇ュ€间篃 > 4 涓?s< 8锛屽洜涓鸿竟鐣岄樆姝㈣法瓒?绗﹀彿浣嶃€?
甯︽湁鍙彉鍋忕Щ閮ㄥ垎鐨?PTR_TO_PACKET 鏈変竴涓?'id'锛屽畠瀵规墍鏈夊叡浜悓涓€鍙彉鍋忕Щ鐨勬寚閽堟槸
閫氱敤鐨勩€傝繖瀵逛簬鍖呰寖鍥存鏌ュ緢閲嶈锛氬湪缁欏寘鎸囬拡瀵勫瓨鍣?A 鍔犱笂涓€涓彉閲忎箣鍚庯紝濡傛灉浣犲皢鍏跺鍒?鍒板彟涓€涓瘎瀛樺櫒 B锛岀劧鍚庣粰 A 鍔犱笂甯搁噺 4锛屼袱涓瘎瀛樺櫒灏嗗叡浜悓涓€涓?'id'锛屼絾 A 灏嗘湁涓€涓?鍥哄畾鍋忕Щ +4銆傜劧鍚庡鏋?A 缁忚繃杈圭晫妫€鏌ュ苟琚彂鐜板皬浜庝竴涓?PTR_TO_PACKET_END锛岄偅涔堝瘎瀛樺櫒
B 鐜板湪灏辩煡閬撴湁鑷冲皯 4 瀛楄妭鐨勫畨鍏ㄨ寖鍥淬€傚叧浜?PTR_TO_PACKET 鑼冨洿鐨勬洿澶氬唴瀹癸紝鍙傝涓嬮潰鐨?"Direct packet access"锛堢洿鎺ュ寘璁块棶锛夈€?
'id' 瀛楁涔熺敤浜?PTR_TO_MAP_VALUE_OR_NULL锛屽鎵€鏈変粠 map 鏌ユ壘杩斿洖鐨勬寚閽堝壇鏈€氱敤銆傝繖
鎰忓懗鐫€褰撲竴涓壇鏈妫€鏌ュ苟鍙戠幇闈?NULL 鏃讹紝鎵€鏈夊壇鏈兘鍙互鍙樹负 PTR_TO_MAP_VALUE銆傞櫎浜?鑼冨洿妫€鏌ワ紝琚窡韪殑淇℃伅涔熺敤浜庡己鍒舵寚閽堣闂殑瀵归綈銆備緥濡傦紝鍦ㄥぇ澶氭暟绯荤粺涓婂寘鎸囬拡鍦ㄤ竴涓?4 瀛楄妭瀵归綈涔嬪悗 2 瀛楄妭澶勩€傚鏋滀竴涓▼搴忓姞涓?14 瀛楄妭浠ヨ烦杩囦互澶綉澶撮儴锛岀劧鍚庤鍙?IHL 骞?鍔犱笂锛圛HL * 4锛夛紝寰楀埌鐨勬寚閽堝皢鏈変竴涓凡鐭ヤ负 4n+2锛堝鏌?n锛夌殑鍙彉鍋忕Щ锛屽洜姝ゅ姞涓婅繖 2 瀛楄妭
锛圢ET_IP_ALIGN锛夊氨寰楀埌涓€涓?4 瀛楄妭瀵归綈锛屽洜姝ら€氳繃璇ユ寚閽堣繘琛岀殑瀛楋紙word锛夊ぇ灏忚闂槸瀹夊叏鐨勩€?'id' 瀛楁涔熺敤浜?PTR_TO_SOCKET 鍜?PTR_TO_SOCKET_OR_NULL锛屽鎵€鏈変粠 socket 鏌ユ壘杩斿洖鐨?鎸囬拡鍓湰閫氱敤銆傚叾琛屼负涓?PTR_TO_MAP_VALUE_OR_NULL->PTR_TO_MAP_VALUE 鐨勫鐞嗙被浼硷紝浣嗗畠
涔熷鐞嗘寚閽堢殑寮曠敤璺熻釜銆侾TR_TO_SOCKET 闅愬紡鍦拌〃绀轰竴涓鐩稿簲 `struct sock` 鐨勫紩鐢ㄣ€備负浜?纭繚璇ュ紩鐢ㄤ笉琚硠婕忥紝蹇呴』瀵硅寮曠敤鍋?NULL 妫€鏌ワ紝骞朵笖鍦ㄩ潪 NULL 鐨勬儏鍐典笅锛屽皢鏈夋晥寮曠敤浼犻€掔粰
socket 閲婃斁鍑芥暟銆?
## 鐩存帴鍖呰闂?
鍦?cls_bpf 鍜?act_bpf 绋嬪簭涓紝鏍￠獙鍣ㄥ厑璁搁€氳繃 skb->data 鍜?skb->data_end 鎸囬拡鐩存帴
璁块棶鍖呮暟鎹€?
```
    1:  r4 = *(u32 *)(r1 +80)  /* 鍔犺浇 skb->data_end */
    2:  r3 = *(u32 *)(r1 +76)  /* 鍔犺浇 skb->data */
    3:  r5 = r3
    4:  r5 += 14
    5:  if r5 > r4 goto pc+16
    R1=ctx R3=pkt(id=0,off=0,r=14) R4=pkt_end R5=pkt(id=0,off=14,r=14) R10=fp
    6:  r0 = *(u16 *)(r3 +12) /* 璁块棶鍖呯殑绗?12 鍜?13 瀛楄妭 */
```

杩欎釜浠庡寘鐨?2 瀛楄妭鍔犺浇鏄畨鍏ㄧ殑锛屽洜涓虹▼搴忎綔鑰呭湪绗?5 鏉℃寚浠ゅ纭疄妫€鏌ヤ簡
`if (skb->data + 14 > skb->data_end) goto err`锛岃繖鎰忓懗鐫€鍦?fall-through 鎯呭喌涓嬶紝
瀵勫瓨鍣?R3锛堟寚鍚?skb->data锛夎嚦灏戞湁 14 涓彲鐩存帴璁块棶鐨勫瓧鑺傘€傛牎楠屽櫒灏嗗叾鏍囪涓?R3=pkt(id=0,off=0,r=14)銆俰d=0 琛ㄧず娌℃湁鍚戣瀵勫瓨鍣ㄦ坊鍔犻澶栫殑鍙橀噺銆俹ff=0 琛ㄧず娌℃湁娣诲姞
棰濆鐨勫父閲忋€俽=14 鏄畨鍏ㄨ闂殑鑼冨洿锛屾剰鍛崇潃瀛楄妭 [R3, R3 + 14) 鏄病闂鐨勩€傛敞鎰?R5 琚?鏍囪涓?R5=pkt(id=0,off=14,r=14)銆傚畠涔熸寚鍚戝寘鏁版嵁锛屼絾鏄悜瀵勫瓨鍣ㄥ姞浜嗗父閲?14锛屾墍浠ュ畠鐜板湪
鎸囧悜 `skb->data + 14`锛屽彲璁块棶鑼冨洿鏄?[R5, R5 + 14 - 14)锛屽嵆闆跺瓧鑺傘€?
```
    R0=inv1 R1=ctx R3=pkt(id=0,off=0,r=14) R4=pkt_end R5=pkt(id=0,off=14,r=14) R10=fp
    6:  r0 = *(u8 *)(r3 +7) /* 浠庡寘鍔犺浇绗?7 瀛楄妭 */
    7:  r4 = *(u8 *)(r3 +12)
    8:  r4 *= 14
    9:  r3 = *(u32 *)(r1 +76) /* 鍔犺浇 skb->data */
    10:  r3 += r4
    11:  r2 = r1
    12:  r2 <<= 48
    13:  r2 >>= 48
    14:  r3 += r2
    15:  r2 = r3
    16:  r2 += 8
    17:  r1 = *(u32 *)(r1 +80) /* 鍔犺浇 skb->data_end */
    18:  if r2 > r1 goto pc+2
    R0=inv(id=0,umax_value=255,var_off=(0x0; 0xff)) R1=pkt_end R2=pkt(id=2,off=8,r=8) R3=pkt(id=2,off=0,r=8) R4=inv(id=0,umax_value=3570,var_off=(0x0; 0xfffe)) R5=pkt(id=0,off=14,r=14) R10=fp
    19:  r1 = *(u8 *)(r3 +4)
```

瀵勫瓨鍣?R3 鐨勭姸鎬佹槸 R3=pkt(id=2,off=0,r=8)銆俰d=2 琛ㄧず鐪嬪埌浜嗕袱鏉?`r3 += rX` 鎸囦护锛?鍥犳 r3 鎸囧悜鍖呭唴鐨勬煇涓亸绉伙紝骞朵笖鐢变簬绋嬪簭浣滆€呭湪绗?18 鏉℃寚浠ゅ鍋氫簡 `if (r3 + 8 > r1)
goto err`锛屽畨鍏ㄨ寖鍥存槸 [R3, R3 + 8)銆傛牎楠屽櫒鍙厑璁稿鍖呭瘎瀛樺櫒杩涜 'add'/'sub'锛堝姞/鍑忥級
鎿嶄綔銆備换浣曞叾浠栨搷浣滈兘浼氬皢瀵勫瓨鍣ㄧ姸鎬佽缃负 'SCALAR_VALUE'锛屽畠灏嗕笉鍐嶅彲鐢ㄤ簬鐩存帴鍖呰闂€?
鎿嶄綔 `r3 += rX` 鍙兘婧㈠嚭骞跺彉寰楀皬浜庡師濮嬬殑 skb->data锛屽洜姝ゆ牎楠屽櫒蹇呴』闃绘杩欎竴鐐广€傛墍浠?褰撳畠鐪嬪埌 `r3 += rX` 鎸囦护涓?rX 鏄ぇ浜?16 浣嶇殑鍊兼椂锛屼换浣曢殢鍚庡 r3 閽堝 skb->data_end
鐨勮竟鐣屾鏌ラ兘涓嶄細缁欐垜浠?鑼冨洿"淇℃伅锛屽洜姝ゅ閫氳繃璇ユ寚閽堢殑璇诲彇灏濊瘯灏嗙粰鍑?invalid access
to packet"锛堝鍖呯殑鏃犳晥璁块棶锛夐敊璇€?
渚嬪鍦ㄧ 7 鏉℃寚浠?`r4 = **(u8 **)(r3 +12)` 涔嬪悗锛宺4 鐨勭姸鎬佹槸 R4=inv(id=0,
umax_value=255,var_off=(0x0; 0xff))锛岃繖鎰忓懗鐫€瀵勫瓨鍣ㄧ殑楂?56 浣嶄繚璇佷负闆讹紝鑰屽浣?8 浣?涓€鏃犳墍鐭ャ€傚湪绗?`r4 *= 14` 鏉℃寚浠や箣鍚庯紝鐘舵€佸彉涓?R4=inv(id=0,umax_value=3570,
var_off=(0x0; 0xfffe))锛屽洜涓哄皢涓€涓?8 浣嶅€间箻浠ュ父閲?14 浼氫繚鎸侀珮 52 浣嶄负闆讹紝涓旀渶浣庢湁鏁?浣嶄篃灏嗕负闆讹紝鍥犱负 14 鏄伓鏁般€傜被浼煎湴 `r2 >>= 48` 灏嗕娇寰?R2=inv(id=0,umax_value=65535,
var_off=(0x0; 0xffff))锛屽洜涓虹Щ浣嶄笉鏄鍙锋墿灞曘€傝繖涓€昏緫鍦?adjust_reg_min_max_vals()
鍑芥暟涓疄鐜帮紝瀹冨"鎸囬拡鍔犳爣閲?锛堟垨鍙嶄箣锛夎皟鐢?adjust_ptr_min_max_vals()锛屽涓や釜鏍囬噺涓婄殑
杩愮畻璋冪敤 adjust_scalar_min_max_vals()銆?
鏈€缁堢殑缁撴灉鏄?bpf 绋嬪簭浣滆€呭彲浠ョ洿鎺ヨ闂寘锛?
```
  void *data = (void *)(long)skb->data;
  void *data_end = (void *)(long)skb->data_end;
  struct eth_hdr *eth = data;
  struct iphdr *iph = data + sizeof(*eth);
  struct udphdr *udp = data + sizeof(*eth) + sizeof(*iph);

  if (data + sizeof(*eth) + sizeof(*iph) + sizeof(*udp) > data_end)
	  return 0;
  if (eth->h_proto != htons(ETH_P_IP))
	  return 0;
  if (iph->protocol != IPPROTO_UDP || iph->ihl != 5)
	  return 0;
  if (udp->dest == 53 || udp->source == 9)
	  ...;
```

涓?LD_ABS 鎸囦护鐩告瘮锛岃繖浣垮緱姝ょ被绋嬪簭鏇存槗浜庣紪鍐欙紝骞朵笖鏄捐憲鏇村揩銆?
## 鍓灊锛圥runing锛?
鏍￠獙鍣ㄥ疄闄呬笂骞朵笉閬嶅巻绋嬪簭涓墍鏈夊彲鑳界殑璺緞銆傚浜庢瘡涓€鏉¤鍒嗘瀽鐨勬柊鍒嗘敮锛屾牎楠屽櫒鏌ョ湅瀹冩鍓?鍦ㄨ繖鏉℃寚浠ゅ鏇剧粡澶勪簬鐨勬墍鏈夌姸鎬併€傚鏋滃叾涓换浣曚竴涓寘鍚綋鍓嶇姸鎬佷綔涓哄瓙闆嗭紝璇ュ垎鏀氨琚?"pruned"锛堝壀鏋濓級鈥斺€斾篃灏辨槸璇达紝鍏堝墠鐘舵€佽鎺ュ彈鐨勪簨瀹炴剰鍛崇潃褰撳墠鐘舵€佷篃浼氳鎺ュ彈銆備緥濡傦紝濡傛灉鍦?鍏堝墠鐘舵€佷腑 r1 鎸佹湁涓€涓寘鎸囬拡锛岃€屽湪褰撳墠鐘舵€佷腑 r1 鎸佹湁涓€涓寖鍥磋嚦灏戝悓鏍烽暱涓斿榻愯嚦灏戝悓鏍?涓ユ牸鐨勫寘鎸囬拡锛岄偅涔?r1 鏄畨鍏ㄧ殑銆傜被浼煎湴锛屽鏋?r2 涔嬪墠鏄?NOT_INIT锛岄偅涔堜粠閭ｄ竴鐐瑰嚭鍙戠殑
浠讳綍璺緞閮戒笉鍙兘浣跨敤杩囧畠锛屽洜姝?r2 涓殑浠讳綍鍊硷紙鍖呮嫭鍙︿竴涓?NOT_INIT锛夐兘鏄畨鍏ㄧ殑銆傝瀹炵幇
鍦?regsafe() 鍑芥暟涓€傚壀鏋濅笉浠呰€冭檻瀵勫瓨鍣紝涔熻€冭檻鏍堬紙浠ュ強瀹冨彲鑳芥寔鏈夌殑浠讳綍婧㈠嚭鐨勫瘎瀛樺櫒锛夈€?瀹冧滑蹇呴』鍏ㄩ儴瀹夊叏锛岃鍒嗘敮鎵嶄細琚壀鏋濄€傝繖鍦?states_equal() 涓疄鐜般€?
鍏充簬鐘舵€佸壀鏋濆疄鐜扮殑涓€浜涙妧鏈粏鑺傚彲浠ュ湪涓嬮潰鎵惧埌銆?
### 瀵勫瓨鍣ㄦ椿璺冩€ц窡韪?
涓轰簡浣跨姸鎬佸壀鏋濇湁鏁堬紝浼氬姣忎釜瀵勫瓨鍣ㄥ拰鏍堟Ы璺熻釜娲昏穬鎬э紙liveness锛夌姸鎬併€傚熀鏈€濇兂鏄窡韪?鍝簺瀵勫瓨鍣ㄥ拰鏍堟Ы鍦ㄧ▼搴忕殑鍚庣画鎵ц涓紙鐩村埌鍒拌揪绋嬪簭閫€鍑猴級瀹為檯琚娇鐢ㄣ€備粠鏈浣跨敤杩囩殑瀵勫瓨
鍣ㄥ拰鏍堟Ы鍙互浠庣紦瀛樼殑鐘舵€佷腑绉婚櫎锛屼粠鑰屼娇鏇村鐘舵€佺瓑浠蜂簬涓€涓紦瀛樼姸鎬侊細

```
  0: call bpf_get_prandom_u32()
  1: r1 = 0
  2: if r0 == 0 goto +1
  3: r0 = 1
  --- checkpoint ---
  4: r0 = r1
  5: exit
```

鍋囪鍦ㄦ寚浠?#4 澶勫垱寤轰簡涓€涓姸鎬佺紦瀛樻潯鐩紙姝ょ被鏉＄洰鍦ㄤ笅鏂囦腑涔熺О涓?checkpoints"锛堟鏌ョ偣锛夛級銆?鏍￠獙鍣ㄥ彲鑳藉甫鐫€浠ヤ笅涓ょ鍙兘鐨勫瘎瀛樺櫒鐘舵€佷箣涓€鍒拌揪璇ユ寚浠わ細

- r0 = 1, r1 = 0
- r0 = 0, r1 = 0

鐒惰€岋紝鍙湁瀵勫瓨鍣?`r1` 鐨勫€煎浜庢垚鍔熷畬鎴愭牎楠屾墠鏄噸瑕佺殑銆傛椿璺冩€ц窡韪畻娉曠殑鐩爣鏄彂鐜拌繖涓€
浜嬪疄锛屽苟寮勬竻杩欎袱绉嶇姸鎬佸疄闄呬笂鏄瓑浠风殑銆?
## 鐞嗚В eBPF 鏍￠獙鍣ㄦ秷鎭?
浠ヤ笅鏄嚑涓棤鏁?eBPF 绋嬪簭浠ュ強鍦ㄦ牎楠屽櫒鏃ュ織涓湅鍒扮殑閿欒淇℃伅鐨勭ず渚嬶細

```
  static struct bpf_insn prog[] = {
  BPF_EXIT_INSN(),
  BPF_EXIT_INSN(),
  };
```

```
  unreachable insn 1
```

```
  BPF_MOV64_REG(BPF_REG_0, BPF_REG_2),
  BPF_EXIT_INSN(),
```

```
  0: (bf) r0 = r2
  R2 !read_ok
```

```
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_1),
  BPF_EXIT_INSN(),
```

```
  0: (bf) r2 = r1
  1: (95) exit
  R0 !read_ok
```

```
    BPF_ST_MEM(BPF_DW, BPF_REG_10, 8, 0),
    BPF_EXIT_INSN(),
```

```
    0: (7a) *(u64 *)(r10 +8) = 0
    invalid stack off=8 size=8
```

```
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_EXIT_INSN(),
```

```
  0: (bf) r2 = r10
  1: (07) r2 += -8
  2: (b7) r1 = 0x0
  3: (85) call 1
  invalid indirect read from stack off -8+0 size 8
```

```
  BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_EXIT_INSN(),
```

```
  0: (7a) *(u64 *)(r10 -8) = 0
  1: (bf) r2 = r10
  2: (07) r2 += -8
  3: (b7) r1 = 0x0
  4: (85) call 1
  fd 0 is not pointing to valid bpf_map
```

鍦ㄨ闂箣鍓嶄笉妫€鏌?map_lookup_elem() 杩斿洖鍊肩殑绋嬪簭锛?
```
  BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_ST_MEM(BPF_DW, BPF_REG_0, 0, 0),
  BPF_EXIT_INSN(),
```

```
  0: (7a) *(u64 *)(r10 -8) = 0
  1: (bf) r2 = r10
  2: (07) r2 += -8
  3: (b7) r1 = 0x0
  4: (85) call 1
  5: (7a) *(u64 *)(r0 +0) = 0
  R0 invalid mem access 'map_value_or_null'
```

姝ｇ‘妫€鏌ヤ簡 map_lookup_elem() 杩斿洖鍊兼槸鍚︿负 NULL锛屼絾锛?
```
  BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0, 1),
  BPF_ST_MEM(BPF_DW, BPF_REG_0, 4, 0),
  BPF_EXIT_INSN(),
```

```
  0: (7a) *(u64 *)(r10 -8) = 0
  1: (bf) r2 = r10
  2: (07) r2 += -8
  3: (b7) r1 = 1
  4: (85) call 1
  5: (15) if r0 == 0x0 goto pc+1
   R0=map_ptr R10=fp
  6: (7a) *(u64 *)(r0 +4) = 0
  misaligned access off 4 size 8
```

姝ｇ‘妫€鏌ヤ簡 map_lookup_elem() 杩斿洖鍊兼槸鍚︿负 NULL锛屽苟鍦?'if' 鍒嗘敮鐨勪竴渚т互姝ｇ‘瀵归綈璁块棶
鍐呭瓨锛屼絾澶辫触鐨勭▼搴忥細

```
  BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_LD_MAP_FD(BPF_REG_1, 0),
  BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
  BPF_JMP_IMM(BPF_JEQ, BPF_REG_0, 0, 2),
  BPF_ST_MEM(BPF_DW, BPF_REG_0, 0, 0),
  BPF_EXIT_INSN(),
  BPF_ST_MEM(BPF_DW, BPF_REG_0, 0, 1),
  BPF_EXIT_INSN(),
```

```
  0: (7a) *(u64 *)(r10 -8) = 0
  1: (bf) r2 = r10
  2: (07) r2 += -8
  3: (b7) r1 = 1
  4: (85) call 1
  5: (15) if r0 == 0x0 goto pc+2
   R0=map_ptr R10=fp
  6: (7a) *(u64 *)(r0 +0) = 0
  7: (95) exit

  from 5 to 8: R0=imm0 R10=fp
  8: (7a) *(u64 *)(r0 +0) = 1
  R0 invalid mem access 'imm'
```

鎵ц socket 鏌ユ壘鐒跺悗灏嗘寚閽堣涓?NULL锛屽嵈娌℃湁锛?
```
  BPF_MOV64_IMM(BPF_REG_2, 0),
  BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_2, -8),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_MOV64_IMM(BPF_REG_3, 4),
  BPF_MOV64_IMM(BPF_REG_4, 0),
  BPF_MOV64_IMM(BPF_REG_5, 0),
  BPF_EMIT_CALL(BPF_FUNC_sk_lookup_tcp),
  BPF_MOV64_IMM(BPF_REG_0, 0),
  BPF_EXIT_INSN(),
```

```
  0: (b7) r2 = 0
  1: (63) *(u32 *)(r10 -8) = r2
  2: (bf) r2 = r10
  3: (07) r2 += -8
  4: (b7) r3 = 4
  5: (b7) r4 = 0
  6: (b7) r5 = 0
  7: (85) call bpf_sk_lookup_tcp#65
  8: (b7) r0 = 0
  9: (95) exit
  Unreleased reference id=1, alloc_insn=7
```

鎵ц socket 鏌ユ壘浣嗘湭瀵硅繑鍥炵殑鎸囬拡鍋?NULL 妫€鏌ョ殑绋嬪簭锛?
```
  BPF_MOV64_IMM(BPF_REG_2, 0),
  BPF_STX_MEM(BPF_W, BPF_REG_10, BPF_REG_2, -8),
  BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
  BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
  BPF_MOV64_IMM(BPF_REG_3, 4),
  BPF_MOV64_IMM(BPF_REG_4, 0),
  BPF_MOV64_IMM(BPF_REG_5, 0),
  BPF_EMIT_CALL(BPF_FUNC_sk_lookup_tcp),
  BPF_EXIT_INSN(),
```

```
  0: (b7) r2 = 0
  1: (63) *(u32 *)(r10 -8) = r2
  2: (bf) r2 = r10
  3: (07) r2 += -8
  4: (b7) r3 = 4
  5: (b7) r4 = 0
  6: (b7) r5 = 0
  7: (85) call bpf_sk_lookup_tcp#65
  8: (95) exit
  Unreleased reference id=1, alloc_insn=7
```
