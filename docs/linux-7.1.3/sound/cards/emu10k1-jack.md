## 浣跨敤 JACK 涓?emu10k1/emu10k2 瀹炵幇浣庡欢杩熴€佸澹伴亾闊抽


鏈枃妗ｆ槸涓€浠芥寚鍗楋紝浠嬬粛濡備綍灏嗗熀浜?emu10k1 鐨勮澶囦笌 JACK 閰嶅悎浣跨敤锛屼互鑾峰緱浣庡欢杩熴€佸澹伴亾褰曢煶鍔熻兘銆?鎴戣繎鏈熸墍鏈夎 Linux 鐢ㄦ埛浣跨敤鍏剁‖浠跺叏閮ㄨ兘鍔涚殑宸ヤ綔锛岄兘鍙楀埌 kX Project 鐨勫惎鍙戙€傛病鏈変粬浠殑鎴愭灉锛屾垜
姘歌繙鏃犳硶鍙戠幇杩欐纭欢鐨勭湡姝ｅ▉鍔涖€?
	http://www.kxproject.com
      - Lee Revell锛?005.03.30

鐩村埌鏈€杩戯紝Linux 涓婄殑 emu10k1 鐢ㄦ埛杩樻棤娉曚娇鐢ㄥ叾 Windows 椹卞姩涓€渒X ASIO鈥濈壒鎬ф墍鎻愪緵鐨勭浉鍚屼綆寤惰繜銆?澶氬０閬撳姛鑳姐€傝嚜 ALSA 1.0.9 璧凤紝杩欏凡鎴愪负杩囧幓锛?
瀵逛簬閭ｄ簺涓嶇啛鎮?kX ASIO 鐨勪汉锛屽畠鐢?16 涓噰闆嗛€氶亾涓?16 涓洖鏀鹃€氶亾缁勬垚銆備娇鐢?2.6.9 涔嬪悗鐨?Linux 鍐呮牳锛?浣庤嚦 64锛?.33 ms锛夌敋鑷?32锛?.66ms锛夊抚鐨勫欢杩熷簲璇ラ兘鑳借壇濂藉伐浣溿€?
閰嶇疆姣斿湪 Windows 涓婄◢寰鏉備竴浜涳紝鍥犱负浣犲繀椤婚€夋嫨姝ｇ‘鐨勮澶囦緵 JACK 浣跨敤銆傚疄闄呬笂锛屽浜?qjackctl 鐢ㄦ埛
鏉ヨ杩欑浉褰撲竴鐩簡鐒垛€斺€旈€夋嫨 Duplex锛岀劧鍚庝负閲囬泦涓庡洖鏀鹃€夋嫨澶氬０閬撹澶囷紝灏嗚緭鍏ヤ笌杈撳嚭閫氶亾璁句负 16锛岄噰鏍风巼
璁句负 48000Hz銆傚懡浠よ濡備笅锛?```

  /usr/local/bin/jackd -R -dalsa -r48000 -p64 -n2 -D -Chw:0,2 -Phw:0,3 -S

```
杩欏皢涓轰綘鎻愪緵 16 涓緭鍏ョ鍙ｄ笌 16 涓緭鍑虹鍙ｃ€?
16 涓緭鍑虹鍙ｆ槧灏勫埌 16 涓?FX 鎬荤嚎锛堝浜?Audigy 鍒欐槸鍓?16 涓紝鍏?64 涓級銆備粠 FX 鎬荤嚎鍒扮墿鐞嗚緭鍑虹殑鏄犲皠
鍦?sb-live-mixer.rst锛堟垨 audigy-mixer.rst锛変腑鎻忚堪銆?
16 涓緭鍏ョ鍙ｈ繛鎺ュ埌 16 涓墿鐞嗚緭鍏ャ€備笌鏅亶鐪嬫硶鐩稿弽锛屾墍鏈?emu10k1 鍗￠兘鏄澹伴亾鍗°€傝繖浜涜緭鍏ラ€氶亾涓摢浜?杩炴帴鏈夌墿鐞嗚緭鍏ワ紝鍙栧喅浜庡崱鐨勫瀷鍙枫€傚己鐑堝缓璁€氳繃璇曢敊鏉ョ‘瀹氾紱涓€浜涘瘜鏈夎繘鍙栧績鐨?kX 鐢ㄦ埛宸茬粡閫嗗悜宸ョ▼鍑鸿鍗＄殑
寮曡剼鍥撅紝骞跺彲鍦ㄧ綉涓婃壘鍒般€侻eterbridge 鍦ㄨ繖閲屽緢鏈夊府鍔╋紝kX 璁哄潧涓篃鍏呮枼鐫€鏈夌敤鐨勪俊鎭€?
姣忎釜杈撳叆绔彛瑕佷箞瀵瑰簲浜庝竴涓暟瀛楋紙SPDIF锛夎緭鍏ャ€佷竴涓ā鎷熻緭鍏ワ紝瑕佷箞浠€涔堜篃娌℃湁銆傚敮涓€鐨勪緥澶栨槸 SBLive! 5.1銆?鍦ㄨ繖浜涜澶囦笂锛岀浜屼釜涓庣涓変釜杈撳叆绔彛琚帴鍒?center/LFE 杈撳嚭銆備綘浠嶇劧浼氱湅鍒?16 涓噰闆嗛€氶亾锛屼絾鍙湁 14 涓?鍙敤浜庡綍闊宠緭鍏ャ€?
涓嬭〃鍊熺敤鑷?kxfxlib/da_asio51.cpp锛屾弿杩颁簡 JACK 绔彛鍒?FXBUS2锛堝杞ㄥ綍闊宠緭鍏ワ級涓?EXTOUT锛堢墿鐞嗚緭鍑猴級
閫氶亾鐨勬槧灏勩€?
10k1 5.1 SBLive 鍗′笂鐨?JACK锛堝強 ASIO锛夋槧灏勶細

==============  ========        ============
JACK		Epilog		FXBUS2(nr)
==============  ========        ============
capture_1	asio14		FXBUS2(0xe)
capture_2	asio15		FXBUS2(0xf)
capture_3	asio0		FXBUS2(0x0)	
~capture_4	Center		EXTOUT(0x11)	// 鐢?Center 鏄犲皠鑰屾潵
~capture_5	LFE		EXTOUT(0x12)	// 鐢?LFE 鏄犲皠鑰屾潵
capture_6	asio3		FXBUS2(0x3)
capture_7	asio4		FXBUS2(0x4)
capture_8	asio5		FXBUS2(0x5)
capture_9	asio6		FXBUS2(0x6)
capture_10	asio7		FXBUS2(0x7)
capture_11	asio8		FXBUS2(0x8)
capture_12	asio9		FXBUS2(0x9)
capture_13	asio10		FXBUS2(0xa)
capture_14	asio11		FXBUS2(0xb)
capture_15	asio12		FXBUS2(0xc)
capture_16	asio13		FXBUS2(0xd)
==============  ========        ============

寰呭姙锛氭弿杩?ld10k1/qlo10k1 涓?JACK 缁撳悎浣跨敤
