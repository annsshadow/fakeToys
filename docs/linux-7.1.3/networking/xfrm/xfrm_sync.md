

## XFRM 鍚屾


璇ュ悓姝ヨˉ涓佸伐浣滃熀浜?Krisztian <hidden@balabit.hu> 鍙婂叾浠栦汉鎻愪緵鐨勫垵濮嬭ˉ涓侊紝
浠ュ強 Jamal <hadi@cyberus.ca> 鎻愪緵鐨勯澶栬ˉ涓併€?

鍚屾鐨勬渶缁堢洰鏍囨槸鑳藉鎻掑叆灞炴€у苟鐢熸垚浜嬩欢锛屼粠鑰屽彲浠ュ皢 SA 瀹夊叏鍦颁粠涓€鍙版満鍣?
杩佺Щ鍒板彟涓€鍙版満鍣ㄤ互瀹炵幇楂樺彲鐢紙HA锛夌洰鐨勩€?
鍏舵€濊矾鏄悓姝?SA锛岃繖鏍锋帴绠℃満鍣ㄥ湪鑳藉璁块棶璇?SA 鏃讹紝鍙互灏藉彲鑳界簿纭湴澶勭悊瀹冦€?

鎴戜滑宸茬粡鍏峰鐢熸垚 SA add/del/upd 浜嬩欢鐨勮兘鍔涖€?
杩欎簺琛ヤ竵澧炲姞浜嗗悓姝ヨ兘鍔涳紝骞舵彁渚涗簡绮剧‘鐨勭敓瀛樻湡瀛楄妭璁℃暟锛堜互纭繚 SA 姝ｇ‘琛板噺锛?
浠ュ強閲嶆斁璁℃暟鍣紝浠庤€屽湪鏁呴殰鍒囨崲鏃跺敖鍙兘鍑忓皯鎹熷け锛岄伩鍏嶉噸鏀炬敾鍑汇€?
杩欐牱锛屽浠芥満灏辫兘淇濇寔涓庝富鐢ㄦ垚鍛樺敖鍙兘鎺ヨ繎鐨勬渶鏂扮姸鎬併€?

鐢变簬涓婅堪鍚勯」浼氶殢鐫€ SA 鏀跺埌鐨勬瘡涓€涓暟鎹寘鑰屽彉鍖栵紝鍥犳鏈夊彲鑳戒骇鐢熷ぇ閲忎簨浠躲€?
鍑轰簬杩欎釜鍘熷洜锛屾垜浠繕鍔犲叆浜嗕竴绉嶇被浼?nagle 鐨勭畻娉曟潵闄愬埗浜嬩欢鏁伴噺銆傚嵆鎴戜滑浼?
璁剧疆闃堝€硷紝渚嬪鈥滃綋閲嶆斁搴忓垪鍙烽槇鍊艰揪鍒版垨宸茶繃鍘?10 绉掓椂閫氱煡鎴戔€濄€?
杩欎簺闃堝€煎彲閫氳繃 sysctl 杩涜绯荤粺绾ц缃紝涔熷彲浠ユ寜 SA 鏇存柊銆?

闇€瑕佸悓姝ョ殑椤瑰寘鎷細
- 鐢熷瓨鏈熷瓧鑺傝鏁板櫒
娉ㄦ剰锛氬鏋滀綘鍋囪鏁呴殰鍒囨崲鏈哄櫒鏄鍏堝凡鐭ョ殑锛屽垯鐢熷瓨鏈熸椂闂撮檺鍒跺苟涓嶉噸瑕侊紝鍥犱负
鏃堕棿鍊掕鏃剁殑琛板噺骞朵笉鏄敱鏁版嵁鍖呭埌杈鹃┍鍔ㄧ殑銆?
- 鍏ュ悜涓庡嚭鍚戠殑閲嶆斁搴忓垪鍙?

### 1) 娑堟伅缁撴瀯


nlmsghdr:aevent_id:optional-TLVs銆?

netlink 娑堟伅绫诲瀷鍖呮嫭锛?

XFRM_MSG_NEWAE 涓?XFRM_MSG_GETAE銆?

XFRM_MSG_GETAE 涓嶅甫 TLV銆?

XFRM_MSG_NEWAE 鑷冲皯浼氬寘鍚袱涓?TLV锛堝涓嬫枃杩涗竴姝ヨ璁猴級銆?

```

   struct xfrm_aevent_id {
	     struct xfrm_usersa_id           sa_id;
	     xfrm_address_t                  saddr;
	     __u32                           flags;
	     __u32                           reqid;
   };

```
鍞竴鐨?SA 鐢?xfrm_usersa_id銆乺eqid 涓?saddr 鐨勭粍鍚堟潵鏍囪瘑銆?

flags 鐢ㄤ簬鎸囩ず涓嶅悓鐨勫惈涔夈€傚彲鑳界殑

```

	XFRM_AE_RTHR=1, /* replay threshold*/
	XFRM_AE_RVAL=2, /* replay value */
	XFRM_AE_LVAL=4, /* lifetime value */
	XFRM_AE_ETHR=8, /* expiry timer threshold */
	XFRM_AE_CR=16, /* Event cause is replay update */
	XFRM_AE_CE=32, /* Event cause is timer expiry */
	XFRM_AE_CU=64, /* Event cause is policy update */

```
杩欎簺 flags 濡備綍浣跨敤鍙栧喅浜庢秷鎭殑鏂瑰悜锛坘ernel<->user锛変互鍙婅捣鍥狅紙閰嶇疆銆佹煡璇㈡垨浜嬩欢锛夈€?
涓嬫枃鍦ㄤ笉鍚岀殑娑堟伅涓細鍔犱互璇存槑銆?

pid 浼氬湪 netlink 涓閫傚綋璁剧疆浠ヨ瘑鍒柟鍚戯紙鍙戝線鍐呮牳鏃朵负 0锛屼粠鍐呮牳鍒扮敤鎴风┖闂存椂
pid = 鍒涘缓璇ヤ簨浠剁殑杩涚▼ ID锛?

绋嬪簭闇€瑕佽闃呭鎾粍 XFRMNLGRP_AEVENTS 鎵嶈兘鏀跺埌杩欎簺浜嬩欢鐨勯€氱煡銆?

### 2) TLV 鍙嶆槧涓嶅悓鐨勫弬鏁?


a) 瀛楄妭鍊硷紙XFRMA_LTIME_VAL锛?

   璇?TLV 鎼哄甫鑷笂娆′簨浠朵互鏉ュ瓧鑺傜敓瀛樻湡鐨勮繍琛?褰撳墠璁℃暟鍣ㄣ€?

b) 閲嶆斁鍊硷紙XFRMA_REPLAY_VAL锛?

   璇?TLV 鎼哄甫鑷笂娆′簨浠朵互鏉ラ噸鏀惧簭鍒楀彿鐨勮繍琛?褰撳墠璁℃暟鍣ㄣ€?

c) 閲嶆斁闃堝€硷紙XFRMA_REPLAY_THRESH锛?

   璇?TLV 鎼哄甫鍐呮牳鐢ㄤ簬鍦ㄩ噸鏀惧簭鍒楀彿瓒呭嚭鏃惰Е鍙戜簨浠剁殑闃堝€笺€?

d) 杩囨湡瀹氭椂鍣紙XFRMA_ETIMER_THRESH锛?

   杩欐槸涓€涓互姣涓哄崟浣嶇殑瀹氭椂鍣ㄥ€硷紝鐢ㄤ綔闄愬埗浜嬩欢閫熺巼鐨?nagle 鍊笺€?

### 3) 鍙傛暟鐨勯粯璁ら厤缃?


榛樿鎯呭喌涓嬭繖浜涗簨浠跺簲褰撴槸鍏抽棴鐨勶紝闄ら潪鑷冲皯鏈変竴涓洃鍚櫒娉ㄥ唽浠ョ洃鍚鎾粍
XFRMNLGRP_AEVENTS銆?

瀹夎 SA 鐨勭▼搴忛渶瑕佹寚瀹氳繖涓や釜闃堝€硷紝浣嗘槸锛屼负浜嗕笉鏀瑰彉璇稿 racoon 涔嬬被鐨?
鐜版湁搴旂敤绋嬪簭锛屾垜浠篃閽堝杩欎簺涓嶅悓鍙傛暟鎻愪緵浜嗛粯璁ら槇鍊硷紝浠ラ槻瀹冧滑鏈鎸囧畾銆?

涓や釜 sysctl/proc 椤逛负锛?

a) /proc/sys/net/core/sysctl_xfrm_aevent_etime

   鐢ㄤ簬浠?100ms 涓洪€掑鏃堕棿鍗曚綅鎻愪緵 XFRMA_ETIMER_THRESH 鐨勯粯璁ゅ€笺€傞粯璁ゆ槸 10锛? 绉掞級

b) /proc/sys/net/core/sysctl_xfrm_aevent_rseqth

   鐢ㄤ簬浠ラ€掑鐨勬暟鎹寘璁℃暟鎻愪緵 XFRMA_REPLAY_THRESH 鍙傛暟鐨勯粯璁ゅ€笺€傞粯璁ゆ槸涓や釜鏁版嵁鍖呫€?

### 4) 娑堟伅绫诲瀷


a) XFRM_MSG_GETAE 鐢辩敤鎴风┖闂?--> 鍐呮牳鍙戝嚭銆?
   XFRM_MSG_GETAE 涓嶆惡甯︿换浣?TLV銆?

   鍝嶅簲鏄竴涓?XFRM_MSG_NEWAE锛屽叾鏍煎紡鍙栧喅浜?XFRM_MSG_GETAE 鎵€鏌ヨ鐨勫唴瀹广€?

   鍝嶅簲濮嬬粓甯︽湁 XFRMA_LTIME_VAL 涓?XFRMA_REPLAY_VAL TLV銆?

     - 濡傛灉璁剧疆浜?XFRM_AE_RTHR 鏍囧織锛屽垯涔熶細鍙栧洖 XFRMA_REPLAY_THRESH
     - 濡傛灉璁剧疆浜?XFRM_AE_ETHR 鏍囧織锛屽垯涔熶細鍙栧洖 XFRMA_ETIMER_THRESH

b) XFRM_MSG_NEWAE 鏃㈠彲鐢辩敤鎴风┖闂村彂鍑轰互杩涜閰嶇疆锛?
   涔熷彲鐢卞唴鏍稿彂鍑轰互瀹ｅ憡浜嬩欢鎴栧搷搴?XFRM_MSG_GETAE銆?

   i) 鐢ㄦ埛 --> 鍐呮牳锛岀敤浜庨厤缃煇涓壒瀹氱殑 SA銆?

      鍙互閫氳繃浼犻€掔浉搴旂殑 TLV 鏉ユ洿鏂颁换鎰忓€兼垨闃堝€煎弬鏁般€?

      浼氬悜鐢ㄦ埛绌洪棿涓殑鍙戦€佹柟鍥為€佸搷搴旓紝鎸囩ず鎴愬姛鎴栧け璐ャ€?

      鍦ㄦ垚鍔熺殑鎯呭喌涓嬶紝杩樹細棰濆鍚戜换浣曠洃鍚櫒鍙戝嚭涓€涓甫鏈?XFRM_MSG_NEWAE 鐨勪簨浠讹紝濡?iii) 鎵€杩般€?

   ii) 鍐呮牳 -> 鐢ㄦ埛鏂瑰悜锛屼綔涓哄 XFRM_MSG_GETAE 鐨勫搷搴?

       鍝嶅簲濮嬬粓甯︽湁 XFRMA_LTIME_VAL 涓?XFRMA_REPLAY_VAL TLV銆?

       濡傛灉 XFRM_MSG_GETAE 娑堟伅涓樉寮忚姹傦紝鍒欎細鍖呭惈闃堝€?TLV銆?

   iii) 鍐呮牳 -> 鐢ㄦ埛锛岀敤浜庢姤鍛婁簨浠讹細濡傛灉鏈変汉浣跨敤 XFRM_MSG_NEWAE锛堝涓婃枃 #i 鎵€杩帮級
        涓烘煇涓?SA 璁剧疆浜嗕换鎰忓€兼垨闃堝€笺€傚湪杩欑鎯呭喌涓嬩細璁剧疆 XFRM_AE_CU 鏍囧織锛?
        浠ュ憡鐭ョ敤鎴疯鍙樻洿鏄敱涓€娆℃洿鏂板紩璧风殑銆?
        璇ユ秷鎭缁堝甫鏈?XFRMA_LTIME_VAL 涓?XFRMA_REPLAY_VAL TLV銆?

   iv) 鍐呮牳 -> 鐢ㄦ埛锛岀敤浜庡湪閲嶆斁闃堝€兼垨瓒呮椂瓒呭嚭鏃舵姤鍛婁簨浠躲€?

鍦ㄨ繖绉嶆儏鍐典笅浼氳缃?XFRM_AE_CR锛堥噸鏀捐秴鍑猴級鎴?XFRM_AE_CE锛堝彂鐢熻秴鏃讹級涔嬩竴锛?
浠ュ憡鐭ョ敤鎴峰彂鐢熶簡浠€涔堛€傛敞鎰忚繖涓や釜鏍囧織鏄簰鏂ョ殑銆?
璇ユ秷鎭缁堝甫鏈?XFRMA_LTIME_VAL 涓?XFRMA_REPLAY_VAL TLV銆?

### 5) 闃堝€艰缃殑渚嬪鎯呭喌


濡傛灉浣犳湁涓€涓?SA锛屽叾娴侀噺鏄獊鍙戝紡鍛戒腑锛屼互鑷充簬瀛樺湪涓€娈靛畾鏃跺櫒闃堝€煎凡杩囨湡浣嗘湭鐪嬪埌
浠讳綍鏁版嵁鍖呯殑鏃舵湡锛岄偅涔堜細鍑虹幇濡備笅鐨勫紓甯歌涓猴細
瀹氭椂鍣ㄨ繃鏈熷悗鐨勭涓€涓暟鎹寘鍒拌揪鏃朵細瑙﹀彂涓€涓秴鏃朵簨浠讹紱鍗虫垜浠笉浼氱瓑寰呰秴鏃?
鍛ㄦ湡鎴栨暟鎹寘闃堝€艰揪鍒般€傝繖鏍峰仛鏄嚭浜庣畝鍗曟€т笌鏁堢巼鐨勮€冭檻銆?

-JHS
