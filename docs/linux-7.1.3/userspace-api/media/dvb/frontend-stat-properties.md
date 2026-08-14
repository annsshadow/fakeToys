######## 鍓嶇缁熻鎸囨爣

鍚勬暟鍊奸€氳繃 `dtv_property.stat` 杩斿洖銆傝嫢璇ュ睘鎬у彈鏀寔锛屽垯 `dtv_property.stat.len` 澶т簬闆躲€?

瀵逛簬澶у鏁颁紶杈撶郴缁燂紝鑻ユ敮鎸佽缁熻锛宍dtv_property.stat.len` 涓?1锛屼笖鍚勫弬鏁板彧杩斿洖涓€涓崟涓€鏁板€笺€?

浣嗛渶娉ㄦ剰锛岃濡?ISDB 绛夋柊鐨?OFDM 浼犺緭绯荤粺鍙互涓烘瘡涓€缁勮浇娉娇鐢ㄤ笉鍚岀殑璋冨埗绫诲瀷銆傚湪姝ょ被鏍囧噯涓嬶紝鏈€澶氬彲鎻愪緵 3 缁勭粺璁★紝骞朵笖 `dtv_property.stat.len` 浼氭洿鏂颁互鍙嶆槧鈥滃叏灞€鈥濇寚鏍囷紝鍐嶅姞涓婃瘡缁勮浇娉㈢殑涓€涓寚鏍囷紙鍦?ISDB 涓О涓衡€滃眰鈥濓級銆?

鍥犳锛屼负涓庡叾浠栦紶杈撶郴缁熶繚鎸佷竴鑷达紝`dtv_property.stat.dtv_stats <dtv_stats>` 鏁扮粍鐨勭涓€涓厓绱犳寚浠ｅ叏灞€鎸囨爣锛涙暟缁勭殑鍏朵綑鍏冪礌琛ㄧず鍚勪釜灞傦紝浠庡眰 A锛堢储寮?1锛夈€佸眰 B锛堢储寮?2锛変緷娆＄被鎺ㄣ€?

宸插～鍏呯殑鍏冪礌涓暟淇濆瓨鍦?`dtv_property.stat.len` 涓€?

`dtv_property.stat.dtv_stats` 鏁扮粍鐨勬瘡涓厓绱犵敱涓や釜閮ㄥ垎缁勬垚锛?

- `svalue` 鎴?`uvalue`锛屽叾涓?`svalue` 鐢ㄤ簬甯︾鍙风殑娴嬮噺鍊硷紙dB 搴﹂噺锛夛紝`uvalue` 鐢ㄤ簬鏃犵鍙峰€硷紙璁℃暟鍣ㄣ€佺浉瀵规瘮渚嬶級銆?

- `scale` 鈥斺€?鏁板€肩殑姣斾緥灏哄害銆傚叾鍙栧€煎彲涓猴細

  - `FE_SCALE_NOT_AVAILABLE` 鈥斺€?鍓嶇鏀寔璇ュ弬鏁帮紝浣嗘棤娉曢噰闆嗗埌瀹冿紙鍙兘鏄殏鏃舵€ф垨姘镐箙鎬х殑鎯呭喌锛夈€?

  - `FE_SCALE_DECIBEL` 鈥斺€?鍙傛暟涓哄甫绗﹀彿鍊硷紝浠?1/1000 dB 涓哄崟浣嶅害閲忋€?

  - `FE_SCALE_RELATIVE` 鈥斺€?鍙傛暟涓烘棤绗﹀彿鍊硷紝鍏朵腑 0 琛ㄧず 0%锛?5535 琛ㄧず 100%銆?

  - `FE_SCALE_COUNTER` 鈥斺€?鍙傛暟涓烘棤绗﹀彿鍊硷紝鐢ㄤ簬缁熻鏌愪簨浠跺彂鐢熺殑娆℃暟锛屼緥濡傝鐮併€佽鍧楁垨娴侀€濈殑鏃堕棿銆?


## DTV_STAT_SIGNAL_STRENGTH

琛ㄧず璋冭皭鍣ㄦ垨瑙ｈ皟鍣ㄦā鎷熼儴鍒嗙殑淇″彿寮哄害姘村钩銆?

姝ゆ寚鏍囧彲鑳界殑姣斾緥灏哄害鏈夛細

- `FE_SCALE_NOT_AVAILABLE` 鈥斺€?娴嬮噺澶辫触锛屾垨娴嬮噺灏氭湭瀹屾垚銆?

- `FE_SCALE_DECIBEL` 鈥斺€?淇″彿寮哄害浠?0.001 dBm 涓哄崟浣嶏紝鍔熺巼浠ユ鐡﹀害閲忋€傝鍊奸€氬父涓鸿礋鏁般€?

- `FE_SCALE_RELATIVE` 鈥斺€?鍓嶇鎻愪緵 0% 鍒?100% 鐨勫姛鐜囨祴閲忥紙瀹為檯涓?0 鍒?65535锛夈€?


## DTV_STAT_CNR

琛ㄧず涓昏浇娉㈢殑淇″櫔姣斻€?

姝ゆ寚鏍囧彲鑳界殑姣斾緥灏哄害鏈夛細

- `FE_SCALE_NOT_AVAILABLE` 鈥斺€?娴嬮噺澶辫触锛屾垨娴嬮噺灏氭湭瀹屾垚銆?

- `FE_SCALE_DECIBEL` 鈥斺€?淇″櫔姣斾互 0.001 dB 涓哄崟浣嶃€?

- `FE_SCALE_RELATIVE` 鈥斺€?鍓嶇鎻愪緵 0% 鍒?100% 鐨勪俊鍣瘮娴嬮噺锛堝疄闄呬负 0 鍒?65535锛夈€?


## DTV_STAT_PRE_ERROR_BIT_COUNT

搴﹂噺鍓嶅悜绾犻敊锛團EC锛変箣鍓嶃€佸唴缂栫爜鍧楋紙鍗?Viterbi銆丩DPC 鎴栧叾浠栧唴鐮佷箣鍓嶏級鐨勮鐮佹暟銆?

璇ュ害閲忓湪 `DTV_STAT_PRE_TOTAL_BIT_COUNT` 鎵€瑕嗙洊鐨勫悓涓€鏃堕棿闂撮殧鍐呰幏鍙栥€?

涓哄緱鍒?BER锛堣鐮佺巼锛夋祴閲忓€硷紝搴斿皢鍏堕櫎浠?`DTV_STAT_PRE_TOTAL_BIT_COUNT <DTV-STAT-PRE-TOTAL-BIT-COUNT>`銆?

闅忕潃鍓嶇鑾峰彇鏇村鐨勪綅璁℃暟娴嬮噺锛岃娴嬮噺鍊煎崟璋冮€掑銆傚綋璋冭皭鍒版煇涓閬?杞彂鍣ㄦ椂锛屽墠绔彲鑳戒細灏嗗叾閲嶇疆銆?

姝ゆ寚鏍囧彲鑳界殑姣斾緥灏哄害鏈夛細

- `FE_SCALE_NOT_AVAILABLE` 鈥斺€?娴嬮噺澶辫触锛屾垨娴嬮噺灏氭湭瀹屾垚銆?

- `FE_SCALE_COUNTER` 鈥斺€?鍐呯紪鐮佷箣鍓嶇粺璁″埌鐨勮鐮佷釜鏁般€?


## DTV_STAT_PRE_TOTAL_BIT_COUNT

搴﹂噺鍐呯紪鐮佸潡涔嬪墠銆佸湪鍚屼竴鍛ㄦ湡鍐呮帴鏀跺埌鐨勪綅鏁帮紝璇ュ懆鏈熶笌 `DTV_STAT_PRE_ERROR_BIT_COUNT <DTV-STAT-PRE-ERROR-BIT-COUNT>` 娴嬮噺鎵€閲囩敤鐨勫懆鏈熺浉鍚屻€?

闇€娉ㄦ剰锛岀敱浜庡墠绔彲鑳介渶瑕佹墜鍔ㄩ噸鍚祴閲忥紝浠庤€屽湪姣忎釜娴嬮噺闂撮殧涔嬮棿涓㈠け閮ㄥ垎鏁版嵁锛屽洜姝よ娴嬮噺鍊煎彲鑳藉皬浜庝紶杈撴祦鐨勬€讳綅鏁般€?

闅忕潃鍓嶇鑾峰彇鏇村鐨勪綅璁℃暟娴嬮噺锛岃娴嬮噺鍊煎崟璋冮€掑銆傚綋璋冭皭鍒版煇涓閬?杞彂鍣ㄦ椂锛屽墠绔彲鑳戒細灏嗗叾閲嶇疆銆?

姝ゆ寚鏍囧彲鑳界殑姣斾緥灏哄害鏈夛細

- `FE_SCALE_NOT_AVAILABLE` 鈥斺€?娴嬮噺澶辫触锛屾垨娴嬮噺灏氭湭瀹屾垚銆?

- `FE_SCALE_COUNTER` 鈥斺€?鍦ㄦ祴閲?`DTV_STAT_PRE_ERROR_BIT_COUNT <DTV-STAT-PRE-ERROR-BIT-COUNT>` 鏃剁粺璁″埌鐨勪綅鏁般€?


## DTV_STAT_POST_ERROR_BIT_COUNT

搴﹂噺鍓嶅悜绾犻敊锛團EC锛変箣鍚庛€佺敱鍐呯紪鐮佸潡锛堝嵆 Viterbi銆丩DPC 鎴栧叾浠栧唴鐮佷箣鍚庯級浜х敓鐨勮鐮佹暟銆?

璇ュ害閲忓湪 `DTV_STAT_POST_TOTAL_BIT_COUNT` 鎵€瑕嗙洊鐨勫悓涓€鏃堕棿闂撮殧鍐呰幏鍙栥€?

涓哄緱鍒?BER锛堣鐮佺巼锛夋祴閲忓€硷紝搴斿皢鍏堕櫎浠?`DTV_STAT_POST_TOTAL_BIT_COUNT <DTV-STAT-POST-TOTAL-BIT-COUNT>`銆?

闅忕潃鍓嶇鑾峰彇鏇村鐨勪綅璁℃暟娴嬮噺锛岃娴嬮噺鍊煎崟璋冮€掑銆傚綋璋冭皭鍒版煇涓閬?杞彂鍣ㄦ椂锛屽墠绔彲鑳戒細灏嗗叾閲嶇疆銆?

姝ゆ寚鏍囧彲鑳界殑姣斾緥灏哄害鏈夛細

- `FE_SCALE_NOT_AVAILABLE` 鈥斺€?娴嬮噺澶辫触锛屾垨娴嬮噺灏氭湭瀹屾垚銆?

- `FE_SCALE_COUNTER` 鈥斺€?鍐呯紪鐮佷箣鍚庣粺璁″埌鐨勮鐮佷釜鏁般€?


## DTV_STAT_POST_TOTAL_BIT_COUNT

搴﹂噺鍐呯紪鐮佷箣鍚庛€佸湪鍚屼竴鍛ㄦ湡鍐呮帴鏀跺埌鐨勪綅鏁帮紝璇ュ懆鏈熶笌 `DTV_STAT_POST_ERROR_BIT_COUNT <DTV-STAT-POST-ERROR-BIT-COUNT>` 娴嬮噺鎵€閲囩敤鐨勫懆鏈熺浉鍚屻€?

闇€娉ㄦ剰锛岀敱浜庡墠绔彲鑳介渶瑕佹墜鍔ㄩ噸鍚祴閲忥紝浠庤€屽湪姣忎釜娴嬮噺闂撮殧涔嬮棿涓㈠け閮ㄥ垎鏁版嵁锛屽洜姝よ娴嬮噺鍊煎彲鑳藉皬浜庝紶杈撴祦鐨勬€讳綅鏁般€?

闅忕潃鍓嶇鑾峰彇鏇村鐨勪綅璁℃暟娴嬮噺锛岃娴嬮噺鍊煎崟璋冮€掑銆傚綋璋冭皭鍒版煇涓閬?杞彂鍣ㄦ椂锛屽墠绔彲鑳戒細灏嗗叾閲嶇疆銆?

姝ゆ寚鏍囧彲鑳界殑姣斾緥灏哄害鏈夛細

- `FE_SCALE_NOT_AVAILABLE` 鈥斺€?娴嬮噺澶辫触锛屾垨娴嬮噺灏氭湭瀹屾垚銆?

- `FE_SCALE_COUNTER` 鈥斺€?鍦ㄦ祴閲?`DTV_STAT_POST_ERROR_BIT_COUNT <DTV-STAT-POST-ERROR-BIT-COUNT>` 鏃剁粺璁″埌鐨勪綅鏁般€?


## DTV_STAT_ERROR_BLOCK_COUNT

搴﹂噺澶栧墠鍚戠籂閿欑紪鐮侊紙鍗?Reed-Solomon 鎴栧叾浠栧鐮佷箣鍚庯級涔嬪悗鐨勮鍧楁暟銆?

闅忕潃鍓嶇鑾峰彇鏇村鐨勪綅璁℃暟娴嬮噺锛岃娴嬮噺鍊煎崟璋冮€掑銆傚綋璋冭皭鍒版煇涓閬?杞彂鍣ㄦ椂锛屽墠绔彲鑳戒細灏嗗叾閲嶇疆銆?

姝ゆ寚鏍囧彲鑳界殑姣斾緥灏哄害鏈夛細

- `FE_SCALE_NOT_AVAILABLE` 鈥斺€?娴嬮噺澶辫触锛屾垨娴嬮噺灏氭湭瀹屾垚銆?

- `FE_SCALE_COUNTER` 鈥斺€?澶栫紪鐮佷箣鍚庣粺璁″埌鐨勮鍧椾釜鏁般€?


## DTV-STAT_TOTAL_BLOCK_COUNT

搴﹂噺鍦ㄤ笌 `DTV_STAT_ERROR_BLOCK_COUNT <DTV-STAT-ERROR-BLOCK-COUNT>` 娴嬮噺鐩稿悓鐨勫懆鏈熷唴鎵€鎺ユ敹鍒扮殑鍧楁€绘暟銆?

鍙敤浜庤绠?PER 鎸囨爣锛屾柟娉曟槸灏?`DTV_STAT_ERROR_BLOCK_COUNT <DTV-STAT-ERROR-BLOCK-COUNT>` 闄や互 `DTV-STAT-TOTAL-BLOCK-COUNT`銆?

姝ゆ寚鏍囧彲鑳界殑姣斾緥灏哄害鏈夛細

- `FE_SCALE_NOT_AVAILABLE` 鈥斺€?娴嬮噺澶辫触锛屾垨娴嬮噺灏氭湭瀹屾垚銆?

- `FE_SCALE_COUNTER` 鈥斺€?鍦ㄦ祴閲?`DTV_STAT_ERROR_BLOCK_COUNT <DTV-STAT-ERROR-BLOCK-COUNT>` 鏃剁粺璁″埌鐨勫潡鏁般€?
