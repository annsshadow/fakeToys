


######## Digital TV property parameters


鏈夎嫢骞蹭笉鍚岀殑鏁板瓧鐢佃锛圖igital TV锛夊弬鏁板彲渚?`FE_SET_PROPERTY` 涓?
`FE_GET_PROPERTY` ioctl<FE_GET_PROPERTY> 浣跨敤銆傛湰鑺傚皢閫愪竴鎻忚堪瀹冧滑銆備絾璇锋敞鎰忥紝
璁剧疆鍓嶇锛坒rontend锛夋椂鍙渶鐢ㄥ埌鍏朵腑鐨勪竴涓瓙闆嗐€?



## DTV_UNDEFINED


鍐呴儴浣跨敤銆傚鍏舵墽琛?GET/SET 鎿嶄綔涓嶄細鏀瑰彉鎴栬繑鍥炰换浣曞唴瀹广€?



## DTV_TUNE


瑙ｆ瀽鏁版嵁缂撳瓨锛坈ache锛夛紝鏋勫缓涓€鏉′紶缁熺殑 frontend 璋冭皭璇锋眰锛坱unerequest锛夛紝浠ヤ究
鑳藉閫氳繃 `FE_SET_FRONTEND` ioctl 鐨勬牎楠屻€?



## DTV_CLEAR


鍦ㄦ澶勯噸缃笓灞炰簬璇?frontend 鐨勬暟鎹紦瀛橈紙cache锛夈€傝繖涓嶄細褰卞搷纭欢銆?



## DTV_FREQUENCY


鏁板瓧鐢佃杞彂鍣紙transponder锛?棰戦亾锛坈hannel锛夌殑棰戠巼銆?


  #. 瀵逛簬鍗槦浼犺緭绯荤粺锛岄鐜囧崟浣嶄负 kHz銆?

  #. 瀵逛簬鏈夌嚎鐢佃锛坈able锛夊拰鍦伴潰锛坱errestrial锛変紶杈撶郴缁燂紝棰戠巼鍗曚綅涓?Hz銆?

  #. 鍦ㄥぇ澶氭暟浼犺緭绯荤粺涓紝棰戠巼涓鸿浆鍙戝櫒/棰戦亾鐨勪腑蹇冮鐜囥€侷SDB-T 渚嬪锛屽叾涓昏浇娉㈢浉瀵?
     涓績鏈?1/7 鐨勫亸绉汇€?

  #. 瀵逛簬 ISDB-T锛岄閬撻€氬父甯︽湁绾?143kHz 鐨勫亸绉昏繘琛屼紶杈撱€備緥濡傦紝涓€涓湁鏁堢殑棰戠巼鍙互
     鏄?474,143 kHz銆傛杩涳紙stepping锛変笌棰戦亾甯﹀鐩稿叧锛岄€氬父涓?6MHz銆?

  #. 鍦?ISDB-Tsb 涓紝棰戦亾浠呯敱涓€娈垫垨涓夋缁勬垚锛岄鐜囨杩涘垎鍒负 429kHz銆?*429銆?



## DTV_MODULATION


涓烘敮鎸佸绉嶈皟鍒讹紙modulation锛夌殑浼犺緭绯荤粺鎸囧畾 frontend 鐨勮皟鍒剁被鍨嬨€?

璋冨埗锛坢odulation锛夊彲浠ユ槸鏋氫妇 `fe_modulation` 鎵€瀹氫箟鐨勭被鍨嬩箣涓€銆?

澶у鏁版暟瀛楃數瑙嗘爣鍑嗛兘鎻愪緵澶氫簬涓€绉嶅彲鑳界殑璋冨埗绫诲瀷銆?

涓嬭〃姹囨€讳簡褰撳墠瑙勮寖涓畾涔夌殑鍚勪紶杈撶郴缁熸墍鏀寔鐨勮皟鍒剁被鍨嬨€?

======================= =======================================================
Standard		Modulation types
======================= =======================================================
ATSC (version 1)	8-VSB and 16-VSB.
DMTB			4-QAM, 16-QAM, 32-QAM, 64-QAM and 4-QAM-NR.
DVB-C Annex A/C		16-QAM, 32-QAM, 64-QAM and 256-QAM.
DVB-C Annex B		64-QAM.
DVB-C2			QPSK, 16-QAM, 64-QAM, 256-QAM, 1024-QAM and 4096-QAM.
DVB-T			QPSK, 16-QAM and 64-QAM.
DVB-T2			QPSK, 16-QAM, 64-QAM and 256-QAM.
DVB-S			No need to set. It supports only QPSK.
DVB-S2			QPSK, 8-PSK, 16-APSK and 32-APSK.
DVB-S2X			8-APSK-L, 16-APSK-L, 32-APSK-L, 64-APSK and 64-APSK-L.
ISDB-T			QPSK, DQPSK, 16-QAM and 64-QAM.
ISDB-S			8-PSK, QPSK and BPSK.
======================= =======================================================


   鐢变簬 DVB-S2X 鏄 DVB-S2 鏍囧噯鐨勬墿灞曪紝浣跨敤浜嗙浉鍚岀殑浼犺緭绯荤粺鏋氫妇鍊硷紙SYS_DVBS2锛夈€?

   璇锋敞鎰忥紝涓婅堪鏌愪簺璋冨埗绫诲瀷褰撳墠鍙兘灏氭湭鍦ㄥ唴鏍革紙Kernel锛変腑瀹氫箟銆傚師鍥犲緢绠€鍗曪細灏氭病鏈?
   椹卞姩闇€瑕佽繖鏍风殑瀹氫箟銆?



## DTV_BANDWIDTH_HZ


棰戦亾鐨勫甫瀹斤紝鍗曚綅涓?Hz銆?

浠呭簲鍦ㄥ湴闈紶杈撶郴缁熶腑璁剧疆銆?

鍙兘鐨勫€硷細`1712000`銆乣5000000`銆乣6000000`銆乣7000000`銆?
`8000000`銆乣10000000`銆?

======================= =======================================================
Terrestrial Standard	Possible values for bandwidth
======================= =======================================================
ATSC (version 1)	No need to set. It is always 6MHz.
DMTB			No need to set. It is always 8MHz.
DVB-T			6MHz, 7MHz and 8MHz.
DVB-T2			1.172 MHz, 5MHz, 6MHz, 7MHz, 8MHz and 10MHz
ISDB-T			5MHz, 6MHz, 7MHz and 8MHz, although most places
			use 6MHz.
======================= =======================================================




  #. 瀵逛簬 ISDB-Tsb锛屽甫瀹戒細闅忔墍杩炴帴娈碉紙segment锛夌殑鏁伴噺鑰屽彉鍖栥€?

     瀹冨彲鐢卞叾浠栧弬鏁帮紙DTV_ISDBT_SB_SEGMENT_IDX銆丏TV_ISDBT_SB_SEGMENT_COUNT锛?
     杞绘槗鎺ㄥ寰楀嚭銆?

  #. 鍦ㄥ崼鏄熷拰鏈夌嚎鐢佃浼犺緭绯荤粺涓紝甯﹀鍙栧喅浜庣鍙风巼锛坰ymbol rate锛夈€傚唴鏍镐細闈欓粯蹇界暐浠讳綍
     DTV-BANDWIDTH-HZ 璁剧疆锛屽苟鐢ㄥ甫瀹戒及绠楀€艰鐩栧畠銆?

     璇ュ甫瀹戒及绠椾細鑰冭檻鐢?DTV-SYMBOL-RATE 璁剧疆鐨勭鍙风巼锛屼互鍙婃粴闄嶏紙rolloff锛夊洜瀛?
     锛堝浜?DVB-C 鍜?DVB-S 涓哄浐瀹氬€硷級銆?

     瀵逛簬 DVB-S2锛屾粴闄嶈繕搴旈€氳繃 DTV-ROLLOFF 璁剧疆銆?



## DTV_INVERSION


鎸囧畾 frontend 鏄惁搴旇繘琛岄璋卞弽杞紙spectral inversion锛夈€?

鍙帴鍙楃殑鍊肩敱 `fe_spectral_inversion` 瀹氫箟銆?



## DTV_DISEQC_MASTER


褰撳墠鏈疄鐜般€?



## DTV_SYMBOL_RATE


鐢ㄤ簬鏈夌嚎鐢佃鍜屽崼鏄熶紶杈撶郴缁熴€?

鏁板瓧鐢佃绗﹀彿鐜囷紙symbol rate锛夛紝鍗曚綅涓烘尝鐗癸紙bauds锛屽嵆绗﹀彿/绉掞級銆?



## DTV_INNER_FEC


鐢ㄤ簬鏈夌嚎鐢佃鍜屽崼鏄熶紶杈撶郴缁熴€?

鍙帴鍙楃殑鍊肩敱 `fe_code_rate` 瀹氫箟銆?



## DTV_VOLTAGE


鐢ㄤ簬鍗槦浼犺緭绯荤粺銆?

鐢靛帇閫氬父鐢ㄤ簬涓嶅叿澶?DiSEqC 鑳藉姏鐨?LNB锛屼互鍒囨崲鏋佸寲鏂瑰紡锛堟按骞?鍨傜洿锛夈€備娇鐢?DiSEqC
璁惧鏃讹紝璇ョ數鍘嬪繀椤婚殢 DiSEqC 鍛戒护涓€鑷村湴鍒囨崲锛屽 DiSEqC 瑙勮寖涓墍杩般€?

鍙帴鍙楃殑鍊肩敱 `fe_sec_voltage` 瀹氫箟銆?



## DTV_TONE


褰撳墠鏈娇鐢ㄣ€?



## DTV_PILOT


鐢ㄤ簬 DVB-S2銆?

璁剧疆 DVB-S2 鐨勫棰戯紙pilot锛夈€?

鍙帴鍙楃殑鍊肩敱 `fe_pilot` 瀹氫箟銆?



## DTV_ROLLOFF


鐢ㄤ簬 DVB-S2銆?

璁剧疆 DVB-S2 鐨勬粴闄嶏紙rolloff锛夈€?

鍙帴鍙楃殑鍊肩敱 `fe_rolloff` 瀹氫箟銆?



## DTV_DISEQC_SLAVE_REPLY


褰撳墠鏈疄鐜般€?



## DTV_FE_CAPABILITY_COUNT


褰撳墠鏈疄鐜般€?



## DTV_FE_CAPABILITY


褰撳墠鏈疄鐜般€?



## DTV_DELIVERY_SYSTEM


鎸囧畾浼犺緭绯荤粺锛坉elivery system锛夌殑绫诲瀷銆?

鍙帴鍙楃殑鍊肩敱 `fe_delivery_system` 瀹氫箟銆?



## DTV_ISDBT_PARTIAL_RECEPTION


浠呯敤浜?ISDB銆?

濡傛灉 `DTV_ISDBT_SOUND_BROADCASTING` 涓?'0'锛岃浣嶅瓧娈佃〃绀洪閬撴槸鍚﹀浜庨儴鍒嗘帴鏀?
锛坧artial reception锛夋ā寮忋€?

濡傛灉涓?'1'锛屽垯 `DTV_ISDBT_LAYERA_*` 鐨勫€艰鍒嗛厤缁欎腑蹇冩锛坈enter segment锛夛紝涓?
`DTV_ISDBT_LAYERA_SEGMENT_COUNT` 蹇呴』涓?'1'銆?

濡傛灉 `DTV_ISDBT_SOUND_BROADCASTING` 杩樹负 '1'锛屽垯 `DTV_ISDBT_PARTIAL_RECEPTION`
琛ㄧず璇?ISDB-Tsb 棰戦亾鏄敱涓€娈典竴灞傝繕鏄笁娈典袱灞傜粍鎴愩€?

鍙兘鐨勫€硷細0銆?銆?1锛圓UTO锛?



## DTV_ISDBT_SOUND_BROADCASTING


浠呯敤浜?ISDB銆?

璇ュ瓧娈佃〃绀哄叾浠?DTV_ISDBT_*-鍙傛暟鎵€鎸囩殑鏄竴涓?ISDB-T 棰戦亾杩樻槸涓€涓?ISDB-Tsb 棰戦亾銆?
锛堝彟瑙?`DTV_ISDBT_PARTIAL_RECEPTION`锛夈€?

鍙兘鐨勫€硷細0銆?銆?1锛圓UTO锛?



## DTV_ISDBT_SB_SUBCHANNEL_ID


浠呯敤浜?ISDB銆?

璇ュ瓧娈典粎褰?`DTV_ISDBT_SOUND_BROADCASTING` 涓?'1' 鏃堕€傜敤銆?

锛堜綔鑰呮敞锛氳繖鍙兘骞堕潪瀵?`SUBCHANNEL-ID` 鍏ㄩ儴缁嗚妭鐨勫噯纭弿杩帮紝浣嗗畠鏄垜瀵圭紪绋嬭澶?
鎵€闇€鎶€鏈儗鏅殑鐞嗚В锛?

涓€涓?ISDB-Tsb 棰戦亾锛? 娈垫垨 3 娈碉級鍙互鍗曠嫭骞挎挱锛屼篃鍙互鎴愮粍锛坰et锛夊湴涓庡叾浠栫浉杩炵殑
ISDB-Tsb 棰戦亾涓€璧峰箍鎾€傚湪杩欑粍棰戦亾涓紝姣忎釜棰戦亾閮藉彲浠ョ嫭绔嬫帴鏀躲€傜浉杩炵殑 ISDB-Tsb
娈电殑鏁伴噺鍙互鍙樺寲锛屼緥濡傚彇鍐充簬鍙敤鐨勯璋卞甫瀹姐€?

绀轰緥锛氬亣璁惧箍鎾簡 8 涓浉杩炵殑 ISDB-Tsb 娈点€傚箍鎾柟鏈夊绉嶆柟寮忓皢杩欎簺棰戦亾閫佷笂绌轰腑锛?
鍋囪涓€涓櫘閫氱殑 13 娈?ISDB-T 棰戣氨锛屼粬鍙互灏嗚繖 8 娈典粠浣嶇疆 1-8 瀵归綈鍒?5-13锛屾垨浠嬩簬
涓よ€呬箣闂寸殑浠讳綍浣嶇疆銆?

娈碉紙segment锛夌殑涓嬪眰鏄瓙淇￠亾锛坰ub-channel锛夛細姣忎釜娈电敱鑻ュ共鍏锋湁棰勫畾涔?ID 鐨勫瓙淇￠亾
缁勬垚銆傚瓙淇￠亾鐢ㄤ簬甯姪瑙ｈ皟鍣紙demodulator锛変笌棰戦亾鍚屾銆?

涓€涓?ISDB-T 棰戦亾鎬绘槸浠ユ墍鏈夊瓙淇￠亾涓轰腑蹇冨榻愩€傚涓婇潰绀轰緥鎵€杩帮紝鍦?ISDB-Tsb 涓氨涓嶅啀
濡傛绠€鍗曚簡銆?

`DTV_ISDBT_SB_SUBCHANNEL_ID` 鍙傛暟鐢ㄤ簬缁欏嚭寰呰В璋冪殑娈电殑瀛愪俊閬?ID銆?

鍙兘鐨勫€硷細0 .. 41銆?1锛圓UTO锛?



## DTV_ISDBT_SB_SEGMENT_IDX


浠呯敤浜?ISDB銆?

璇ュ瓧娈典粎褰?`DTV_ISDBT_SOUND_BROADCASTING` 涓?'1' 鏃堕€傜敤銆?

`DTV_ISDBT_SB_SEGMENT_IDX` 缁欏嚭寰呰В璋冩鐨勭储寮曪紝鐢ㄤ簬澶氫釜 ISDB-Tsb 棰戦亾浠ョ浉杩炴柟寮?
浼犺緭鐨勬儏鍐点€?

鍙兘鐨勫€硷細0 .. `DTV_ISDBT_SB_SEGMENT_COUNT` - 1

娉ㄦ剰锛氳鍊兼棤娉曠敱鑷姩棰戦亾鎼滅储纭畾銆?



## DTV_ISDBT_SB_SEGMENT_COUNT


浠呯敤浜?ISDB銆?

璇ュ瓧娈典粎褰?`DTV_ISDBT_SOUND_BROADCASTING` 涓?'1' 鏃堕€傜敤銆?

`DTV_ISDBT_SB_SEGMENT_COUNT` 缁欏嚭鐩歌繛鐨?ISDB-Tsb 棰戦亾鐨勬€绘暟銆?

鍙兘鐨勫€硷細1 .. 13

娉ㄦ剰锛氳鍊兼棤娉曠敱鑷姩棰戦亾鎼滅储纭畾銆?



## DTV-ISDBT-LAYER[A-C] parameters


浠呯敤浜?ISDB銆?

ISDB-T 棰戦亾鍙互閲囩敤鍒嗗眰锛坔ierarchical锛夌紪鐮併€備笌 DVB-T 涓嶅悓锛孖SDB-T 涓殑鍒嗗眰鍙互
鍚屾椂瑙ｇ爜銆傚洜姝や竴涓?ISDB-T 瑙ｈ皟鍣ㄦ嫢鏈?3 涓?Viterbi 鍜?3 涓?Reed-Solomon 瑙ｇ爜鍣ㄣ€?

ISDB-T 鏈?3 涓垎灞傦紝姣忎竴灞傚彲浠ヤ娇鐢ㄥ彲鐢ㄦ鐨勪竴閮ㄥ垎銆傛墍鏈夊眰鐨勬€绘鏁板湪 ISDB-T 涓?
蹇呴』涓?13銆?

鍏辨湁 3 缁勫弬鏁帮紝鍒嗗埆鐢ㄤ簬灞?A銆丅 鍜?C銆?



### DTV_ISDBT_LAYER_ENABLED


浠呯敤浜?ISDB銆?

ISDB-T 涓殑鍒嗗眰鎺ユ敹锛坔ierarchical reception锛夐€氳繃瑙ｇ爜杩囩▼涓惎鐢ㄦ垨绂佺敤鍚勫眰鏉ュ疄鐜般€?
灏?`DTV_ISDBT_LAYER_ENABLED` 鐨勬墍鏈変綅璁句负 '1' 浼氬己鍒惰В璋冩墍鏈夊眰锛堝閫傜敤锛夈€傝繖鏄?
榛樿琛屼负銆?

濡傛灉棰戦亾澶勪簬閮ㄥ垎鎺ユ敹妯″紡锛坄DTV_ISDBT_PARTIAL_RECEPTION` = 1锛夛紝涓績娈靛彲浠ョ嫭绔嬩簬
鍏朵粬 12 娈佃瑙ｇ爜銆傚湪璇ユā寮忎笅锛屽眰 A 鐨?`SEGMENT_COUNT` 蹇呴』涓?1銆?

鍦?ISDB-Tsb 涓粎浣跨敤灞?A锛屾牴鎹?`DTV_ISDBT_PARTIAL_RECEPTION`锛屽畠鍙互鏄?1 鎴?3銆?
`SEGMENT_COUNT` 蹇呴』鐩稿簲鍦板～鍐欍€?

浠呬娇鐢ㄥ墠 3 浣嶇殑鍊笺€傚叾浠栦綅灏嗚闈欓粯蹇界暐锛?

`DTV_ISDBT_LAYER_ENABLED` 浣?0锛氬惎鐢ㄥ眰 A

`DTV_ISDBT_LAYER_ENABLED` 浣?1锛氬惎鐢ㄥ眰 B

`DTV_ISDBT_LAYER_ENABLED` 浣?2锛氬惎鐢ㄥ眰 C

`DTV_ISDBT_LAYER_ENABLED` 浣?3-31锛氭湭浣跨敤



### DTV_ISDBT_LAYER[A-C]_FEC


浠呯敤浜?ISDB銆?

缁欏畾 ISDB 灞傛墍浣跨敤鐨勫墠鍚戠籂閿欙紙Forward Error Correction锛夋満鍒讹紝鐢?`fe_code_rate`
瀹氫箟銆?

鍙兘鐨勫€间负锛歚FEC_AUTO`銆乣FEC_1_2`銆乣FEC_2_3`銆乣FEC_3_4`銆?
`FEC_5_6`銆乣FEC_7_8`



### DTV_ISDBT_LAYER[A-C]_MODULATION


浠呯敤浜?ISDB銆?

缁欏畾 ISDB 灞傛墍浣跨敤鐨勮皟鍒讹紙modulation锛夛紝鐢?`fe_modulation` 瀹氫箟銆?

鍙兘鐨勫€间负锛歚QAM_AUTO`銆乣QPSK`銆乣QAM_16`銆乣QAM_64`銆乣DQPSK`


   #. 濡傛灉灞?C 涓?`DQPSK`锛屽垯灞?B 蹇呴』涓?`DQPSK`銆?

   #. 濡傛灉灞?B 涓?`DQPSK` 涓?`DTV_ISDBT_PARTIAL_RECEPTION` = 0锛屽垯灞傚繀椤讳负
      `DQPSK`銆?



### DTV_ISDBT_LAYER[A-C]_SEGMENT_COUNT


浠呯敤浜?ISDB銆?

鍙兘鐨勫€硷細0銆?銆?銆?銆?銆?銆?銆?銆?銆?銆?0銆?1銆?2銆?3銆?1锛圓UTO锛?

娉ㄦ剰锛歚DTV_ISDBT_SOUND_BROADCASTING`銆乣DTV_ISDBT_PARTIAL_RECEPTION` 涓?
`LAYER[A-C]_SEGMENT_COUNT` 鐨勭湡鍊艰〃锛坱ruth table锛?


    :header-rows:  1
    :stub-columns: 0


    - .. row 1

       - Partial Reception

       - Sound Broadcasting

       - Layer A width

       - Layer B width

       - Layer C width

       - total width

    - .. row 2

       - 0

       - 0

       - 1 .. 13

       - 1 .. 13

       - 1 .. 13

       - 13

    - .. row 3

       - 1

       - 0

       - 1

       - 1 .. 13

       - 1 .. 13

       - 13

    - .. row 4

       - 0

       - 1

       - 1

       - 0

       - 0

       - 1

    - .. row 5

       - 1

       - 1

       - 1

       - 2

       - 0

       - 13



### DTV_ISDBT_LAYER[A-C]_TIME_INTERLEAVING


浠呯敤浜?ISDB銆?

鏈夋晥鍊硷細0銆?銆?銆?銆?1锛圓UTO锛?

褰?DTV_ISDBT_SOUND_BROADCASTING 澶勪簬婵€娲荤姸鎬佹椂锛屽€?8 涔熸槸鏈夋晥鐨勩€?

娉ㄦ剰锛氬疄闄呯殑鏃跺煙浜ょ粐锛坱ime interleaving锛夐暱搴﹀彇鍐充簬妯″紡锛坒ft 澶у皬锛夈€傛澶勭殑鍊?
鎸囩殑鏄彲鍦?TMCC 缁撴瀯涓壘鍒扮殑鍐呭锛屽涓嬭〃鎵€绀恒€?


    :header-rows:  1
    :stub-columns: 0


    - .. row 1

       - `DTV_ISDBT_LAYER[A-C]_TIME_INTERLEAVING`

       - Mode 1 (2K FFT)

       - Mode 2 (4K FFT)

       - Mode 3 (8K FFT)

    - .. row 2

       - 0

       - 0

       - 0

       - 0

    - .. row 3

       - 1

       - 4

       - 2

       - 1

    - .. row 4

       - 2

       - 8

       - 4

       - 2

    - .. row 5

       - 4

       - 16

       - 8

       - 4



### DTV_ATSCMH_FIC_VER


浠呯敤浜?ATSC-MH銆?

FIC锛團ast Information Channel锛屽揩閫熶俊鎭俊閬擄級淇′护鏁版嵁鐨勭増鏈彿銆?

FIC 鐢ㄤ簬浼犻€掍俊鎭紝浠ヤ究鎺ユ敹绔揩閫熻幏鍙栨湇鍔°€?

鍙兘鐨勫€硷細0銆?銆?銆?銆?..銆?0銆?1



### DTV_ATSCMH_PARADE_ID


浠呯敤浜?ATSC-MH銆?

Parade 鏍囪瘑鍙凤紙parade identification number锛夈€?

涓€涓?parade 鏄嚦澶?8 涓?MH 缁勶紙group锛夌殑闆嗗悎锛屾壙杞戒竴涓垨涓や釜 ensemble銆?

鍙兘鐨勫€硷細0銆?銆?銆?銆?..銆?26銆?27



### DTV_ATSCMH_NOG


浠呯敤浜?ATSC-MH銆?

鎸囧畾 parade 涓瘡涓?MH 瀛愬抚锛坰ubframe锛夌殑 MH 缁勶紙group锛夋暟閲忋€?

鍙兘鐨勫€硷細1銆?銆?銆?銆?銆?銆?銆?



### DTV_ATSCMH_TNOG


浠呯敤浜?ATSC-MH銆?

MH 缁勭殑鎬绘暟锛屽寘鍚睘浜庝竴涓?MH 瀛愬抚涓墍鏈?MH parade 鐨勫叏閮?MH 缁勩€?

鍙兘鐨勫€硷細0銆?銆?銆?銆?..銆?0銆?1



### DTV_ATSCMH_SGN


浠呯敤浜?ATSC-MH銆?

璧峰缁勫彿锛坰tart group number锛夈€?

鍙兘鐨勫€硷細0銆?銆?銆?銆?..銆?4銆?5



### DTV_ATSCMH_PRC


浠呯敤浜?ATSC-MH銆?

Parade 閲嶅鍛ㄦ湡锛坧arade repetition cycle锛夈€?

鍙兘鐨勫€硷細1銆?銆?銆?銆?銆?銆?銆?



### DTV_ATSCMH_RS_FRAME_MODE


浠呯敤浜?ATSC-MH銆?

Reed Solomon锛圧S锛夊抚妯″紡锛坒rame mode锛夈€?

鍙帴鍙楃殑鍊肩敱 `atscmh_rs_frame_mode` 瀹氫箟銆?



### DTV_ATSCMH_RS_FRAME_ENSEMBLE


浠呯敤浜?ATSC-MH銆?

Reed Solomon锛圧S锛夊抚 ensemble銆?

鍙帴鍙楃殑鍊肩敱 `atscmh_rs_frame_ensemble` 瀹氫箟銆?



### DTV_ATSCMH_RS_CODE_MODE_PRI


浠呯敤浜?ATSC-MH銆?

Reed Solomon锛圧S锛夌紪鐮佹ā寮忥紙code mode锛屼富锛夈€?

鍙帴鍙楃殑鍊肩敱 `atscmh_rs_code_mode` 瀹氫箟銆?



### DTV_ATSCMH_RS_CODE_MODE_SEC


浠呯敤浜?ATSC-MH銆?

Reed Solomon锛圧S锛夌紪鐮佹ā寮忥紙code mode锛屾锛夈€?

鍙帴鍙楃殑鍊肩敱 `atscmh_rs_code_mode` 瀹氫箟銆?



### DTV_ATSCMH_SCCC_BLOCK_MODE


浠呯敤浜?ATSC-MH銆?

涓叉帴鍗风Н鐮佸潡妯″紡锛圫eries Concatenated Convolutional Code Block Mode锛夈€?

鍙帴鍙楃殑鍊肩敱 `atscmh_sccc_block_mode` 瀹氫箟銆?



### DTV_ATSCMH_SCCC_CODE_MODE_A


浠呯敤浜?ATSC-MH銆?

涓叉帴鍗风Н鐮佺巼锛圫eries Concatenated Convolutional Code Rate锛夈€?

鍙帴鍙楃殑鍊肩敱 `atscmh_sccc_code_mode` 瀹氫箟銆?



### DTV_ATSCMH_SCCC_CODE_MODE_B


浠呯敤浜?ATSC-MH銆?

涓叉帴鍗风Н鐮佺巼锛圫eries Concatenated Convolutional Code Rate锛夈€?

鍙兘鐨勫€间笌鏋氫妇 `atscmh_sccc_code_mode` 涓褰曠殑鍊肩浉鍚屻€?



### DTV_ATSCMH_SCCC_CODE_MODE_C


浠呯敤浜?ATSC-MH銆?

涓叉帴鍗风Н鐮佺巼锛圫eries Concatenated Convolutional Code Rate锛夈€?

鍙兘鐨勫€间笌鏋氫妇 `atscmh_sccc_code_mode` 涓褰曠殑鍊肩浉鍚屻€?



### DTV_ATSCMH_SCCC_CODE_MODE_D


浠呯敤浜?ATSC-MH銆?

涓叉帴鍗风Н鐮佺巼锛圫eries Concatenated Convolutional Code Rate锛夈€?

鍙兘鐨勫€间笌鏋氫妇 `atscmh_sccc_code_mode` 涓褰曠殑鍊肩浉鍚屻€?



## DTV_API_VERSION


杩斿洖鏁板瓧鐢佃 API 鐨勪富/娆＄増鏈彿銆?



## DTV_CODE_RATE_HP


鐢ㄤ簬鍦伴潰浼犺緭銆?

鍙帴鍙楃殑鍊肩敱 `fe_transmit_mode` 瀹氫箟銆?



## DTV_CODE_RATE_LP


鐢ㄤ簬鍦伴潰浼犺緭銆?

鍙帴鍙楃殑鍊肩敱 `fe_transmit_mode` 瀹氫箟銆?



## DTV_GUARD_INTERVAL


鍙帴鍙楃殑鍊肩敱 `fe_guard_interval` 瀹氫箟銆?


   #. 濡傛灉 `DTV_GUARD_INTERVAL` 璁剧疆涓?`GUARD_INTERVAL_AUTO`锛岀‖浠跺皢灏濊瘯鎵惧埌姝ｇ‘鐨?
      淇濇姢闂撮殧锛坓uard interval锛夛紙鑻ユ敮鎸侊級锛屽苟浣跨敤 TMCC 濉厖缂哄け鐨勫弬鏁般€?
   #. 闂撮殧 `GUARD_INTERVAL_1_64` 浠呯敤浜?DVB-C2銆?
   #. 闂撮殧 `GUARD_INTERVAL_1_128` 鍚屾椂鐢ㄤ簬 DVB-C2 鍜?DVB_T2銆?
   #. 闂撮殧 `GUARD_INTERVAL_19_128` 涓?`GUARD_INTERVAL_19_256` 浠呯敤浜?DVB-T2銆?
   #. 闂撮殧 `GUARD_INTERVAL_PN420`銆乣GUARD_INTERVAL_PN595` 涓?
      `GUARD_INTERVAL_PN945` 褰撳墠浠呯敤浜?DMTB銆傚湪璇ユ爣鍑嗕笅锛屼粎杩欎簺闂撮殧涓?
      `GUARD_INTERVAL_AUTO` 鏄湁鏁堢殑銆?


## DTV_TRANSMISSION_MODE



浠呯敤浜庡熀浜?OFDM 鐨勬爣鍑嗭紝渚嬪 DVB-T/T2銆両SDB-T銆丏TMB銆?

鎸囧畾璇ユ爣鍑嗘墍浣跨敤鐨?FFT 澶у皬锛堝搴斾簬杞芥尝鐨勮繎浼兼暟閲忥級銆?

鍙帴鍙楃殑鍊肩敱 `fe_transmit_mode` 瀹氫箟銆?


   #. ISDB-T 鏀寔涓夌杞芥尝/绗﹀彿澶у皬锛?K銆?K銆?K銆傚湪璇ユ爣鍑嗕腑绉颁负**妯″紡锛坢ode锛?*锛?
      骞朵粠 1 鍒?3 缂栧彿锛?

      ====	========	========================
      Mode	FFT size	Transmission mode
      ====	========	========================
      1		2K		`TRANSMISSION_MODE_2K`
      2		4K		`TRANSMISSION_MODE_4K`
      3		8K		`TRANSMISSION_MODE_8K`
      ====	========	========================

   #. 濡傛灉 `DTV_TRANSMISSION_MODE` 璁剧疆涓?`TRANSMISSION_MODE_AUTO`锛岀‖浠跺皢灏濊瘯鎵惧埌
      姝ｇ‘鐨?FFT 澶у皬锛堣嫢鏀寔锛夛紝骞朵娇鐢?TMCC 濉厖缂哄け鐨勫弬鏁般€?

   #. DVB-T 瑙勫畾 2K 鍜?8K 涓烘湁鏁堝ぇ灏忋€?

   #. DVB-T2 瑙勫畾 1K銆?K銆?K銆?K銆?6K 鍜?32K銆?

   #. DTMB 瑙勫畾 C1 鍜?C3780銆?



## DTV_HIERARCHY


浠呯敤浜?DVB-T 鍜?DVB-T2銆?

Frontend 鍒嗗眰锛坔ierarchy锛夈€?

鍙帴鍙楃殑鍊肩敱 `fe_hierarchy` 瀹氫箟銆?



## DTV_STREAM_ID


鐢ㄤ簬 DVB-C2銆丏VB-S2銆丏VB-T2 鍜?ISDB-S銆?

DVB-C2銆丏VB-S2銆丏VB-T2 鍜?ISDB-S 鏀寔鍦ㄥ崟涓€浼犺緭娴侊紙transport stream锛変笂浼犺緭澶氫釜
娴侊紙stream锛夈€傚綋纭欢鏀寔鏃讹紝璇ュ睘鎬т娇鏁板瓧鐢佃椹卞姩鑳藉澶勭悊瀛愭祦杩囨护锛坰ubstream
filtering锛夈€傞粯璁ゆ儏鍐典笅锛屽瓙娴佽繃婊ゆ槸绂佺敤鐨勩€?

瀵逛簬 DVB-C2銆丏VB-S2 鍜?DVB-T2锛屾湁鏁堢殑瀛愭祦 id 鑼冨洿涓?0 鍒?255銆?

瀵逛簬 ISDB锛屾湁鏁堢殑瀛愭祦 id 鑼冨洿涓?1 鍒?65535銆?

瑕佺鐢ㄥ畠锛屽簲浣跨敤鐗规畩瀹?NO_STREAM_ID_FILTER銆?

娉ㄦ剰锛氫换浣曡秴鍑?id 鑼冨洿鐨勫€间篃浼氱鐢ㄨ繃婊ゃ€?



## DTV_DVBT2_PLP_ID_LEGACY


宸插簾寮冿紝鐢?DTV_STREAM_ID 鍙栦唬銆?



## DTV_ENUM_DELSYS


涓€涓鏍囧噯锛坢ulti standard锛塮rontend 闇€瑕侀€氬憡鍏舵墍鎻愪緵鐨勪紶杈撶郴缁熴€傚簲鐢ㄧ▼搴忓湪浣跨敤
鍓嶇鐨勪换浣曞叾浠栨搷浣滀箣鍓嶏紝闇€瑕佹灇涓炬墍鎻愪緵鐨勪紶杈撶郴缁熴€傚湪寮曞叆璇ュ睘鎬т箣鍓嶏紝浣跨敤
FE_GET_INFO 鏉ョ‘瀹氬墠绔被鍨嬨€傚浜庢彁渚涘涓紶杈撶郴缁熺殑鍓嶇锛孎E_GET_INFO 甯姪涓嶅ぇ銆?
鎵撶畻浣跨敤澶氭爣鍑嗗墠绔殑搴旂敤绋嬪簭蹇呴』鏋氫妇涓庡叾鍏宠仈鐨勪紶杈撶郴缁燂紝鑰屼笉鏄皾璇曚娇鐢?
FE_GET_INFO銆傚浜庨仐鐣欏墠绔紝缁撴灉涓?FE_GET_INFO 鐩稿悓锛屼絾鏍煎紡鏇村叿缁撴瀯鍖栥€?

鍙帴鍙楃殑鍊肩敱 `fe_delivery_system` 瀹氫箟銆?



## DTV_INTERLEAVING


瑕佷娇鐢ㄧ殑鏃跺煙浜ょ粐锛坱ime interleaving锛夈€?

鍙帴鍙楃殑鍊肩敱 `fe_interleaving` 瀹氫箟銆?



## DTV_LNA


浣庡櫔澹版斁澶у櫒锛圠ow-noise amplifier锛夈€?

纭欢鍙兘鎻愪緵鍙帶鐨?LNA锛屽彲閫氳繃璇ュ弬鏁版墜鍔ㄨ缃€傞€氬父 LNA 鍙湪鍦伴潰璁惧涓瓨鍦紙濡傛灉
鏈夌殑璇濓級銆?

鍙兘鐨勫€硷細0銆?銆丩NA_AUTO

0锛孡NA 鍏抽棴

1锛孡NA 寮€鍚?

浣跨敤鐗规畩瀹?LNA_AUTO 璁剧疆 LNA 涓鸿嚜鍔ㄦā寮?



## DTV_SCRAMBLING_SEQUENCE_INDEX


鐢ㄤ簬 DVB-S2銆?

璇?18 浣嶅瓧娈碉紙瀛樺湪鏃讹級鎵胯浇 DVB-S2 鐗╃悊灞傚姞鎵板簭鍒楋紙scrambling sequence锛夌殑绱㈠紩锛?
濡?EN 302 307 绗?5.5.4 鑺傛墍瀹氫箟銆傛病鏈夋樉寮忕殑淇′护鏂规硶灏嗗姞鎵板簭鍒楃储寮曞彂閫佺粰鎺ユ敹绔€?
濡傛灉鍙敤鐨勮瘽锛屽彲浠ヤ娇鐢?S2 鍗槦浼犺緭绯荤粺鎻忚堪绗︽潵璇诲彇鍔犳壈搴忓垪绱㈠紩锛圗N 300 468
琛?41锛夈€?

榛樿浣跨敤 gold 鍔犳壈搴忓垪绱㈠紩 0銆?

鏈夋晥鐨勫姞鎵板簭鍒楃储寮曡寖鍥翠负 0 鍒?262142銆?
